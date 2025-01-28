use crate::config::{
    LitNodeConfig, CFG_KEY_CHAIN_POLLING_INTERVAL_MS_DEFAULT,
    CFG_KEY_RESTORE_LOG_INTERVAL_MS_DEFAULT,
};
use crate::error::unexpected_err;
use crate::node_state::{NodeState, State, Transition};
use crate::peers::peer_state::models::{SimplePeer, SimplePeerExt};
use crate::peers::{peer_state::models::NetworkState, PeerState};
use crate::tss::common::restore::{report_progress, NodeRecoveryStatus, RestoreState};
use crate::tss::common::traits::epoch_manager::EpochManager;
use crate::tss::common::traits::fsm_worker_metadata::FSMWorkerMetadata;
use crate::tss::common::{curve_type::CurveType, dkg_type::DkgType};
use crate::version::get_version;
use ethers::types::U256;
use lit_blockchain::contracts::pubkey_router::RootKey;
use lit_blockchain::contracts::staking::Version;
use lit_core::config::ReloadableLitConfig;
use lit_core::error::{Result, Unexpected};
use lit_core::utils::binary::hex_to_bytes;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, Mutex, RwLock};

#[derive(Debug, Clone)]
pub struct DKGRootKey {
    pub curve_type: CurveType,
    pub pubkey: String,
}

// Main FSM worker thread.
#[allow(clippy::too_many_arguments)]
pub async fn node_fsm_worker(
    mut quit_rx: mpsc::Receiver<bool>,
    cfg: ReloadableLitConfig,
    peer_state: Arc<PeerState>,
    node_state: Arc<Mutex<NodeState>>,
    restore_state: Arc<RwLock<RestoreState>>,
    recovery_epoch_managers: Vec<Box<dyn EpochManager>>,
    standard_epoch_managers: Vec<Box<dyn EpochManager>>,
    fsm_worker_metadata: Arc<dyn FSMWorkerMetadata<LifecycleId = u64>>,
) {
    let mut root_keys: Vec<DKGRootKey> = Vec::new();
    let mut epoch_to_signal_ready = U256::from(0);
    let mut previous_included_epoch_number = U256::from(0); // Any initial value will work
    let mut previous_retries = U256::from(0);
    let interval_ms = cfg
        .load_full()
        .chain_polling_interval_ms()
        .unwrap_or(CFG_KEY_CHAIN_POLLING_INTERVAL_MS_DEFAULT) as u64;
    let mut interval = tokio::time::interval(Duration::from_millis(interval_ms));

    // We're currently in a state where the FSM is doing many operations and is often
    // taking longer than the interval tick. Setting the missed tick behavior will ensure
    // we maintain the interval between ticks whenever we're able to process certain loops
    // faster and have missed many prior ticks.
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

    // really more pertinent for nodes coming online with the network already up.
    if let Err(e) = peer_state.connect_to_validators_union().await {
        error!("Error in connect_to_validators_union: {}", e);
    }

    info!(
        "Starting: PeerState and NodeState polling (will try every {}s)",
        interval.period().as_secs()
    );

    // Restore the key store if the config is set so. If the config did not set it,
    // assume new network initialization and do not attempt to restore.
    if let Ok(true) = cfg.load().enter_restore_state() {
        report_progress(&cfg.load_full(), NodeRecoveryStatus::StartedInRestoreState).await;

        let restore_log_interval =
            cfg.load()
                .restore_log_interval()
                .unwrap_or(CFG_KEY_RESTORE_LOG_INTERVAL_MS_DEFAULT) as u64;
        let log_frequency_in_loop = restore_log_interval / interval_ms;
        let mut thick_counter = 0;
        info!("RestoreState: enter_restore_state is set in the config. Attempting to restore.");
        let staker_address = &peer_state.hex_staker_address();

        // Try to restore the key shares until all the key shares are restored.
        loop {
            // Try to restore the key shares under the read lock.
            let (newly_recovered_bls_keys, newly_recovered_ecdsa_keys) = {
                let state = restore_state.read().await;
                let epoch = peer_state.epoch().await;
                state.try_restore_key_shares(epoch, staker_address).await
            };

            // Mark these keys as recovered.
            // We are doing this separately because the process of recovering keys
            // takes long, so we do it with a read lock. We take the write lock
            // only to set their boolean `restored` flags as true.
            {
                let mut state = restore_state.write().await;
                state.mark_keys_restored(&newly_recovered_bls_keys, &newly_recovered_ecdsa_keys);
            }

            thick_counter = match thick_counter >= log_frequency_in_loop {
                true => {
                    let state = restore_state.read().await;
                    state.log();
                    0
                }
                false => thick_counter + 1,
            };

            // Check if the restoration is complete for this node.
            let all_keys_restored = {
                let state = restore_state.read().await;
                state.are_all_keys_restored()
            };
            if all_keys_restored {
                info!("RestoreState: All the root keys are restored");
                report_progress(&cfg.load_full(), NodeRecoveryStatus::AllKeysAreRestored).await;
                break;
            }

            // Check if the restoration is complete for the network.
            match peer_state.network_state().await {
                Ok(NetworkState::Restore) | Ok(NetworkState::Paused) => {}
                Ok(s) => {
                    info!("RestoreState: network state changed: {:?}", s);
                    report_progress(
                        &cfg.load_full(),
                        NodeRecoveryStatus::AbandonedRecoveryDueToNetworkState,
                    )
                    .await;
                    break;
                }
                Err(_) => {}
            };

            interval.tick().await;
        }

        // The loop is over which means the restoration is complete.
        // Clear the restore state and generate new blinders.
        {
            let mut state = restore_state.write().await;
            state.exit(&cfg.load_full());
            info!("RestoreState: Exiting Restore State");
        }

        // Wait until the network is active again
        loop {
            if let Ok(state) = peer_state.network_state().await {
                if state != NetworkState::Restore && state != NetworkState::Paused {
                    info!("RestoreState: Exiting recovery code, starting the fsm loop.");
                    break;
                }
            }
            interval.tick().await;
        }
    }

    // initialize the node state
    {
        node_state.lock().await.next(Transition::Init);
    }

    // set initial previous_retries
    // because the retry counter only increases in the staking contract
    // and is never reset to 0
    match peer_state.get_epoch().await {
        Ok(epoch) => {
            previous_retries = epoch.3;
        }
        Err(e) => {
            error!(
                "Error setting initial previous_retries with get_epoch: {}",
                e
            );
        }
    }

    // Main FSM Loop

    loop {
        // Check if we should quit, or continue with the next check of FSM.
        tokio::select! {
            _ = quit_rx.recv() => {
                info!("Stopped: PeerState and BlsState poller");
                return;
            }
            _ = interval.tick() => {
                // Pad the interval with a 2s sleep - if the loop took longer than the interval time, the next loop
                // will start immediately, which is not what we want. This is only a short-term fix, and we should
                // aim to make the loop lighter weight and faster.
                tokio::time::sleep(Duration::from_secs(2)).await;

                // Continue below.
            }
        }

        // Update worker metadata
        fsm_worker_metadata.update_lifecycle_id(None);

        let lifecycle_id = fsm_worker_metadata.get_lifecycle_id();
        trace!("Node State polling - tick: {}", lifecycle_id);

        let (epoch_number, network_state, retries) =
            match get_fsm_state(&peer_state, &node_state).await {
                Ok(fsm_state) => fsm_state,
                Err(e) => {
                    error!("Error in get_fsm_state: {}", e);
                    continue;
                }
            };
        // if we're paused, just do another loop.
        if network_state == NetworkState::Paused {
            info!("Network state is Paused.  Pausing FSM node state polling.");
            match peer_state.connect_to_validators_union().await {
                Ok(_) => {}
                Err(e) => {
                    error!("Error in connect_to_validators_union: {}", e);
                }
            }
            continue;
        }

        // If the network is Restoring, and we are not, this process quits.
        if network_state == NetworkState::Restore {
            std::process::exit(0);
        }

        // set epoch_to_signal_ready to the current epoch if uninitialized (aka == 0)
        if epoch_to_signal_ready == U256::from(0) {
            epoch_to_signal_ready = epoch_number;
        }

        // the flow is different if we're waiting to be part of the network, or an actual current member.
        match peer_state.part_of_validators_union().await {
            Err(e) => error!("Error in part_of_validators_union: {}", e),

            // functions/transitions to check if we're not part of the current validators
            Ok(false) => {
                let mut _node_state = node_state.lock().await;
                trace!(
                    "Not part_of_validators_union; node_state: {:?}",
                    _node_state.current_state()
                );

                match _node_state.current_state() {
                    State::Online => {
                        info!("Node is online, but not part of the current validators.  Waiting for next epoch to join.  Preloading Peer State Data.");
                        let _ = get_current_and_new_peer_addresses(peer_state.clone()).await;
                    }
                    State::Active | State::PendingActive | State::Locked => {
                        if let Err(e) = peer_state.validators_in_active_state().await {
                            error!("Error in handle_if_validators_in_active_state: {}", e);
                        }
                        _node_state.next(Transition::Leave)
                    }
                    State::Suspended => {
                        if epoch_number > previous_included_epoch_number + 1 {
                            _node_state.next(Transition::Rejoin);
                        }
                    }
                    // unhandled?
                    _ => {
                        trace!(
                            "Not part_of_validators_union; unhandled node_state: {:?}",
                            _node_state.current_state()
                        );
                        continue;
                    }
                }
            }

            // This is the main point for functions that deal with nodes that are part of the current active set.
            Ok(true) => {
                // Locked & next epoch? As before
                let mut _node_state = node_state.lock().await;
                trace!(
                    "Epoch (prior/current): {} / {} - node state: {:?} - epoch_to_signal_ready: {} - network state: {:?}",
                    previous_included_epoch_number,
                    epoch_number,
                    _node_state.current_state(),
                    epoch_to_signal_ready,
                    network_state
                );

                match _node_state.current_state() {
                    State::Suspended => {
                        // if we're in the node set, and we're in the suspended state, we should move to Online
                        // so that we can participate in the epoch transition
                        _node_state.next(Transition::Rejoin);
                    }
                    State::Active | State::Online => {
                        // The node should request to leave if it is running a soon-to-be / already incompatible node version.
                        let is_node_running_compatible_version =
                            check_version_compatibility(peer_state.clone());
                        match is_node_running_compatible_version.await {
                            Ok(false) => {
                                info!("Node is running an incompatible version. Requesting to leave the network.");
                                if let Err(e) = peer_state.request_to_leave().await {
                                    error!("Error in request_to_leave: {}", e);
                                }
                                continue;
                            }
                            Err(e) => {
                                error!("Error in is_compatible_version: {}", e);
                                continue;
                            }
                            _ => {}
                        }

                        if epoch_number > previous_included_epoch_number {
                            match peer_state.connect_to_validators_union().await {
                                Ok(_) => {}
                                Err(e) => {
                                    error!("Error in connect_to_validators_union: {}", e);
                                }
                            }
                            if network_state == NetworkState::NextValidatorSetLocked {
                                // Here we have detected a chain state update. Let's forcibly update the CDM before we transition.
                                // This is important because we need to make sure that the CDM is up to date before we start doing epoch change
                                // operations in the Locked state.
                                match peer_state
                                    .chain_data_config_manager
                                    .set_peer_and_epoch_data_from_chain()
                                    .await
                                {
                                    Ok(_) => {
                                        debug!("Updated chain data manager state");
                                    }
                                    Err(e) => {
                                        error!(
                                            "Failed to update chain data manager state: {:?}",
                                            e
                                        );
                                    }
                                }
                                if _node_state.current_state() == State::Active {
                                    _node_state.next(Transition::Incumbent);
                                    debug!("Moving from Active node to Locked!");
                                } else {
                                    _node_state.next(Transition::Selected);
                                    debug!("Moving from Online node to Locked!");
                                }
                            }
                        } else if retries > previous_retries {
                            // check if we should retry
                            // this happens if the node thinks it finished reshare, but the
                            // contract is forcing a retry because the node set changed after locking

                            // reset previous_included_epoch_number because it was incremented in anticipation of the epoch transitioning, but
                            // the epoch didn't transition, and instead we are retrying
                            if previous_included_epoch_number == epoch_number {
                                previous_included_epoch_number = epoch_number - U256::from(1);
                            }
                            previous_retries = retries;
                            debug!("Retrying: Staying in Active or Online state to retry");
                            continue;
                        }
                    }

                    State::PendingActive => {
                        //FIXME-> this is debugging work:
                        set_peer_state_debugging_data(peer_state.clone()).await;

                        let sigst = peer_state.clone();
                        // don't both signalling ready for next epoch - enough of threshold already did, and calling again will fail.
                        if epoch_number > epoch_to_signal_ready {
                            _node_state.next(Transition::Complete);
                            trace!(
                                "Node state (pending->complete) : {:?}",
                                _node_state.current_state()
                            );
                            continue;
                        }

                        let res = sigst
                            .rpc_signal_ready_for_next_epoch(epoch_to_signal_ready)
                            .await;
                        if res.is_err() {
                            error!("Failed to signal ready for next epoch! Error: {:?}.", res);
                            // if we failed to signal ready here, then it's likely because
                            // the transition failed, and we should retry
                            if retries > previous_retries {
                                // reset previous_included_epoch_number because it was incremented in anticipation of the epoch transitioning, but
                                // the epoch didn't transition, and instead we are retrying
                                if previous_included_epoch_number == epoch_number {
                                    previous_included_epoch_number = epoch_number - U256::from(1);
                                }
                                previous_retries = retries;
                                _node_state.next(Transition::Retry);
                                debug!("Retrying: Moving from PendingActive to Active");
                                continue;
                            }
                        } else {
                            _node_state.next(Transition::Complete);
                            trace!(
                                "Node state (pending->complete) : {:?}",
                                _node_state.current_state()
                            );
                        }
                    }

                    State::Locked => {
                        previous_included_epoch_number = epoch_number;
                        epoch_to_signal_ready = epoch_number;

                        // We don't do a backup in the first epoch as there's nothing to backup
                        let not_first_epoch = epoch_number > U256::from(1);

                        // Perform a DKG only when the backup party in the nextState has changed which happens when `registerNewBackupParty` is called
                        let backup_party_not_empty = match peer_state.backup_party_not_empty().await
                        {
                            Ok(backup_party_not_empty) => backup_party_not_empty,
                            Err(e) => {
                                error!("Failed to call getNextBackupPartyMembers w/ err {:?}", e);
                                continue;
                            }
                        };

                        let is_recovery_dkg_completed =
                            match peer_state.is_recovery_dkg_registered().await {
                                Ok(is_recovery_dkg_completed) => is_recovery_dkg_completed,
                                Err(e) => {
                                    error!("Failed to call isRecoveryDkgCompleted w/ err {:?}", e);
                                    continue;
                                }
                            };

                        let potentially_participate_in_recover_dkg =
                            not_first_epoch && backup_party_not_empty && !is_recovery_dkg_completed;

                        if potentially_participate_in_recover_dkg {
                            match peer_state.set_recovery_dkg_member().await {
                                Ok(_) => info!("called setMemberForDkg"),
                                Err(e) => {
                                    error!("Fail to set member for recovery DKG w/ err {:?}", e);
                                    continue;
                                }
                            };

                            // Check whether we're participating in the Recovery DKG
                            let do_recovery_dkg = match peer_state.is_node_mapped().await {
                                Ok(do_recovery_dkg) => do_recovery_dkg,
                                Err(e) => {
                                    error!("Failed to call for isNodeForDkg w/ err {:?}", e);
                                    continue;
                                }
                            };

                            if do_recovery_dkg {
                                info!("Doing Recovery DKG!");

                                let (passed_epoch_managers, recovery_keys_result) =
                                    perform_epoch_change(
                                        &cfg,
                                        &recovery_epoch_managers,
                                        fsm_worker_metadata.clone(),
                                        peer_state.clone(),
                                        DkgType::RecoveryParty,
                                        epoch_number,
                                        true,
                                    )
                                    .await;

                                if passed_epoch_managers < recovery_epoch_managers.len() {
                                    debug!(
                                        "Recovery: passed_epoch_managers = {}",
                                        passed_epoch_managers
                                    );
                                    continue;
                                }

                                // NOTE: We can't continue until it's registered on chain as other nodes won't know that the Recovery DKG is completed so they should keep on waiting
                                let recovery_keys_result = match recovery_keys_result {
                                    Some(recovery_keys) => recovery_keys,
                                    None => {
                                        debug!("recovery_keys_result == None");
                                        continue;
                                    }
                                };

                                // NOTE?: Assume ECDSA DKG pass when BLS passes just like Standard DKG
                                peer_state
                                    .register_recovery_keys(recovery_keys_result)
                                    .await;
                            }
                        }

                        let is_recovery_dkg_completed = match peer_state
                            .is_recovery_dkg_registered()
                            .await
                        {
                            Ok(is_recovery_dkg_completed) => is_recovery_dkg_completed,
                            Err(e) => {
                                error!("Can't proceed to Standard DKG, failed to call isRecoveryDkgCompleted w/ err {:?}", e);
                                continue;
                            }
                        };

                        // Don't perform Standard DKG until the Recovery DKG is successful
                        if not_first_epoch && backup_party_not_empty && !is_recovery_dkg_completed {
                            info!("Recovery DKG isn't completed. Checking again in the next FSM interation");
                            continue;
                        }

                        //FIXME: Match new peers to target peers.  This should be a built in check of some sort!
                        // let target_peers = peer_state.get_validators_in_next_epoch().await.unwrap();
                        let (passed_epoch_managers, root_keys_result) = perform_epoch_change(
                            &cfg,
                            &standard_epoch_managers,
                            fsm_worker_metadata.clone(),
                            peer_state.clone(),
                            DkgType::Standard,
                            epoch_number,
                            false,
                        )
                        .await;
                        if passed_epoch_managers == 0 {
                            debug!("Standard: passed_epoch_managers == 0");
                            continue;
                        }

                        let root_keys_result = match root_keys_result {
                            Some(root_keys) => root_keys,
                            None => {
                                debug!("root_keys_result == None");
                                continue;
                            }
                        };

                        root_keys.extend(root_keys_result); // NOTE?: Assume all ECDSA DKGs pass when BLS passes
                                                            // if we've gone through the nodes and appended all the keys (if required), we can signal ready.

                        if passed_epoch_managers == standard_epoch_managers.len() {
                            _node_state.next(Transition::Complete);
                            debug!(
                                "Node state (t:complete) : {:?}",
                                _node_state.current_state()
                            );
                        } else {
                            trace!("Not transitioning state to pending. Only {} of {} epoch managers passed.", passed_epoch_managers, standard_epoch_managers.len());
                        }
                    }

                    // unhandled?
                    _ => {
                        debug!(
                            "Part_of_validators_union; but unhandled node_state: {:?}",
                            _node_state.current_state()
                        );
                        continue;
                    }
                }

                //FIXME-> this is debugging work:
                set_peer_state_debugging_data(peer_state.clone()).await;

                // attempt to lock validators if we're in a state that can do so
                if network_state == NetworkState::Active || network_state == NetworkState::Unlocked
                {
                    peer_state.rpc_lock_validators_for_next_epoch().await;
                }

                // attempt to advance the epoch if we're in a state that can do so
                if network_state == NetworkState::ReadyForNextEpoch {
                    let r_epoch = peer_state.rpc_advance_epoch().await;
                }

                // if we've loaded things up and we have an epoch, vote for any keys.
                // FIXME -> probably want to put this in a seperate thread/task ???  Though it only happens once.
                check_root_key_voting(&mut root_keys, &cfg, &peer_state, epoch_number).await;
            }
        }
    }
}

async fn perform_epoch_change(
    cfg: &ReloadableLitConfig,
    epoch_managers: &Vec<Box<dyn EpochManager>>,
    fsm_worker_metadata: Arc<dyn FSMWorkerMetadata<LifecycleId = u64>>,
    peer_state: Arc<PeerState>,
    dkg_type: DkgType,
    epoch_number: U256,
    is_recovery_dkg: bool,
) -> (usize, Option<Vec<DKGRootKey>>) {
    async fn _perform_epoch_change(
        addr: &str,
        epoch_managers: &Vec<Box<dyn EpochManager>>,
        dkg_id: String,
        epoch_number: U256,
        current_peers: &Vec<SimplePeer>,
        new_peers: &Vec<SimplePeer>,
        dkg_type: DkgType,
    ) -> (usize, Option<Vec<DKGRootKey>>) {
        let mut passed_epoch_managers = 0;
        let mut root_keys: Vec<DKGRootKey> = Vec::new();

        for epoch_manager in epoch_managers {
            let curve_type = epoch_manager.curve_type();
            let epoch_dkg_id = format!("{}.{}.{}", dkg_id, curve_type, dkg_type);
            match epoch_manager
                .change_epoch(
                    epoch_dkg_id,
                    epoch_number.as_u64(),
                    addr,
                    current_peers,
                    new_peers,
                    dkg_type,
                )
                .await
            {
                Err(e) => {
                    error!("Error in epoch_manager.change_epoch: {}", e);
                    return (passed_epoch_managers, None);
                }

                Ok((result, pubkeys)) => {
                    if !result {
                        warn!(
                            "DKG for epoch change failed for {:?}.",
                            epoch_manager.curve_type()
                        );

                        // note that if any DKG fails for any key type, we abort ALL Dkgs and lose any keys we've processed - this is by design.
                        return (passed_epoch_managers, None);
                    }

                    passed_epoch_managers += 1;

                    let mut root_pubkeys = pubkeys
                        .iter()
                        .map(|k| DKGRootKey {
                            curve_type,
                            pubkey: k.to_string(),
                        })
                        .collect::<Vec<DKGRootKey>>();
                    root_keys.append(root_pubkeys.as_mut());

                    debug!(
                        "DKG for epoch change complete for {:?}.",
                        epoch_manager.curve_type()
                    );
                }
            }
        }

        (passed_epoch_managers, Some(root_keys))
    }

    struct EpochChangeResOrUpdateNeeded {
        pub epoch_change_res: Option<(usize, Option<Vec<DKGRootKey>>)>,
        pub update_req: Option<u64>,
    }

    // Derive the DKG ID.
    let fsm_worker_lifecycle_id = fsm_worker_metadata.get_lifecycle_id();
    let dkg_id = derive_dkg_id(epoch_number, fsm_worker_lifecycle_id);

    // We keep looping until we get a result from a completed epoch change operation.
    let mut latest_dkg_id = dkg_id.clone();
    let mut abort_and_restart_count = 0;

    // We currently set the limit of aborts and restarts to be a high number to avoid infinite loops. This should never happen,
    // in theory, but we want to be safe. This will be removed as soon as we have implemented an improved strategy to synchronize
    // the DKG ID across our distributed network of nodes.
    while abort_and_restart_count < 10 {
        info!(
            "Performing epoch change for dkg_id: {} (abort_and_restart_count: {})",
            latest_dkg_id, abort_and_restart_count
        );

        // make sure peers are up to date, across potential abort + restarts.
        let (current_peers, new_peers) = if is_recovery_dkg {
            let recovery_dkg_peers = match peer_state.get_recovery_dkg_peers().await {
                Ok(recovery_dkg_peers) => recovery_dkg_peers,
                Err(e) => {
                    error!("Error in getting Recovery DKG peers: {:?}", e);
                    return (0, None);
                }
            };

            (vec![], recovery_dkg_peers)
        } else {
            match get_current_and_new_peer_addresses(peer_state.clone()).await {
                Ok((current_peers, new_peers)) => (current_peers, new_peers),
                Err(e) => {
                    error!("Error in get_peer_addresses: {}", e);
                    return (0, None);
                }
            }
        };

        let epoch_change_res_or_update_needed = tokio::select! {
            // We stop polling the other future as soon as `yield_until_update` returns and
            // after we parse the lifecycle IDs.
            new_lifecycle_id = fsm_worker_metadata.yield_until_update() => {
                let existing_lifecycle_id = fsm_worker_metadata.get_lifecycle_id();
                info!("FSMWorkerMetadata is outdated, updating the lifecycle id from {} to {}, aborting the current epoch change and restarting with the new updated lifecycle id", existing_lifecycle_id, new_lifecycle_id);
                EpochChangeResOrUpdateNeeded {
                    epoch_change_res: None,
                    update_req: Some(new_lifecycle_id),
                }
            }
            res = _perform_epoch_change(&peer_state.addr, epoch_managers, latest_dkg_id.clone(), epoch_number, &current_peers, &new_peers, dkg_type) => {
                EpochChangeResOrUpdateNeeded {
                    epoch_change_res: Some(res),
                    update_req: None,
                }
            }
        };

        // If there is a result, we immediately return the result.
        if let Some(res) = epoch_change_res_or_update_needed.epoch_change_res {
            return res;
        }

        // If we are here, that means that we need to update the lifecycle ID and restart the epoch change.
        let new_lifecycle_id = match epoch_change_res_or_update_needed.update_req {
            Some(new_lifecycle_id) => new_lifecycle_id,
            None => {
                error!("epoch_change_res_or_update_needed.update_req is None");
                return (0, None);
            }
        };

        fsm_worker_metadata.update_lifecycle_id(Some(new_lifecycle_id));

        let existing_epoch_number = match parse_epoch_number_from_dkg_id(&dkg_id) {
            Ok(existing_epoch_number) => existing_epoch_number,
            Err(e) => {
                error!("Error in parse_epoch_number_from_dkg_id: {}", e);
                return (0, None);
            }
        };
        trace!("existing_epoch_number: {}", existing_epoch_number);

        let previous_dkg_id = latest_dkg_id.clone();
        latest_dkg_id = derive_dkg_id(existing_epoch_number, new_lifecycle_id);
        info!(
            "Previous dkg_id was {}, new dkg_id is {}",
            previous_dkg_id, latest_dkg_id
        );

        abort_and_restart_count += 1;
    }

    // If we are here, that means that we have aborted and restarted the epoch change too many times. Just return a failure.
    error!("Aborted and restarted the epoch change too many times. Aborting the epoch change.");
    (0, None)
}

pub async fn get_fsm_state(
    peer_state: &Arc<PeerState>,
    node_state: &Arc<Mutex<NodeState>>,
) -> Result<(U256, NetworkState, U256)> {
    let network_state = peer_state
        .network_state()
        .await
        .map_err(|e| unexpected_err(e, Some("Could not get network_state".into())))?;

    // FIXME: Check for errors.
    let epoch = peer_state
        .get_epoch()
        .await
        .map_err(|e| unexpected_err(e, Some("Could not get epoch".into())))?;
    let epoch_number = epoch.1;
    let retries = epoch.3;

    let block_number = peer_state
        .get_block_number()
        .await
        .map_err(|e| unexpected_err(e, Some("Could not get block_number".into())))?;

    let peers_in_epoch = peer_state.peers().await?;

    trace!(
        "Block: {} Epoch: {}, Network state: {:?}, Node state: {:?}, Retries: {:?}, Peers: {:?} ",
        block_number,
        epoch_number,
        network_state,
        node_state.lock().await.current_state(),
        retries,
        peers_in_epoch.debug_addresses(),
    );

    Ok((epoch_number, network_state, retries))
}

pub async fn check_root_key_voting(
    root_keys: &mut Vec<DKGRootKey>,
    cfg: &ReloadableLitConfig,
    peer_state: &Arc<PeerState>,
    epoch_number: U256,
) {
    if !root_keys.is_empty() && epoch_number >= U256::from(1) {
        match vote_for_root_pubkeys(cfg, root_keys.clone()).await {
            Ok(result) => {
                if !result {
                    info!("vote_for_root_pubkeys returned false");
                    return;
                }

                root_keys.clear();
            }
            Err(e) => {
                if let Ok(current_validators) = peer_state.validators_in_current_epoch().await {
                    warn!("Current Validators: {:?}", current_validators);
                } // debug message is in function call.
            }
        }
    }
}

pub async fn get_current_and_new_peer_addresses(
    peer_state: Arc<PeerState>,
) -> Result<(Vec<SimplePeer>, Vec<SimplePeer>)> {
    peer_state.connect_to_validators_union().await?;

    let current_peers = peer_state.peers().await?;

    let new_peers = peer_state.peers_in_next_epoch().await?.active_peers();

    info!(
        "Current/new peers for epoch change: ( {}/{} )  {} / {} ",
        &current_peers.len(),
        &new_peers.len(),
        &current_peers.debug_addresses(),
        &new_peers.debug_addresses(),
    );

    info!(
        "Validators for next epoch are locked: {}",
        peer_state.validators_for_next_epoch_locked().await?
    );

    Ok((current_peers, new_peers))
}

pub async fn vote_for_root_pubkeys(
    cfg: &ReloadableLitConfig,
    pubkeys: Vec<DKGRootKey>,
) -> Result<bool> {
    use crate::pkp::utils::vote_for_root_key;
    use ethers::core::types::Bytes;

    info!("incoming root pubkeys: {:?}", pubkeys);

    let mut root_keys: Vec<RootKey> = Vec::new();

    for dkg_root_key in pubkeys {
        let pk_bytes = hex_to_bytes(&dkg_root_key.pubkey)?;
        let rootkey = RootKey {
            pubkey: Bytes::from(pk_bytes),
            key_type: dkg_root_key.curve_type.into(),
        };
        root_keys.push(rootkey);
    }

    info!("Root Keys to vote for: {:?}", root_keys);
    vote_for_root_key(&cfg.load_full(), root_keys).await
}

// FIXME: This is debugging work.  Remove when done.
pub async fn set_peer_state_debugging_data(peer_state: Arc<PeerState>) {
    let next_validators = match peer_state.validators_in_next_epoch().await {
        Ok(v) => v,
        Err(e) => {
            error!("Error in get_validators_in_next_epoch: {}", e);
            return;
        }
    };
    // let mut val_res: Vec<(u32, bool)> = Vec::new();
    let mut m1 = "Validators signaled ready (".to_string();
    let mut m2 = "".to_string();

    for v in next_validators {
        m1.push_str(&format!("{},", v.port));
        let is_peer_ready_for_next_epoch = match peer_state.get_ready_signal(v.staker_address).await
        {
            Ok(r) => r,
            Err(e) => {
                error!("Error in get_ready_signal: {}", e);
                return;
            }
        };
        m2.push_str(&format!("{},", is_peer_ready_for_next_epoch,));
    }
    trace!("{}): {}", m1, m2);

    let current_validator_count_for_consensus =
        match peer_state.current_validator_count_for_consensus().await {
            Ok(v) => v,
            Err(e) => {
                error!("Error in current_validator_count_for_consensus: {}", e);
                return;
            }
        };

    let next_validator_count_for_consensus =
        match peer_state.next_validator_count_for_consensus().await {
            Ok(v) => v,
            Err(e) => {
                error!("Error in next_validator_count_for_consensus: {}", e);
                return;
            }
        };

    let count_of_validators_ready_for_next_epoch = match peer_state
        .get_count_of_validators_ready_for_next_epoch()
        .await
    {
        Ok(v) => v,
        Err(e) => {
            error!(
                "Error in get_count_of_validators_ready_for_next_epoch: {}",
                e
            );
            return;
        }
    };

    trace!(
        "Req'd consensus (current/next) : {:?}/{:?} - Validators ready for next epoch {:?}",
        current_validator_count_for_consensus,
        next_validator_count_for_consensus,
        count_of_validators_ready_for_next_epoch,
    );
}

pub fn derive_dkg_id(
    epoch_number: U256,
    fsm_worker_lifecycle_id: <dyn FSMWorkerMetadata<LifecycleId = u64> as FSMWorkerMetadata>::LifecycleId,
) -> String {
    format!("EPOCH_DKG_{}_{}", epoch_number, fsm_worker_lifecycle_id)
}

fn parse_epoch_number_from_dkg_id<T>(dkg_id: T) -> Result<U256>
where
    T: AsRef<str>,
{
    let dkg_id = dkg_id.as_ref();
    let epoch_number = dkg_id
        .split('_')
        .nth(2)
        .expect_or_err("Failed to parse epoch number")?;
    let epoch_number_u128 = epoch_number
        .parse::<u128>()
        .expect_or_err("Failed to parse epoch number as u128")?;
    Ok(U256::from(epoch_number_u128))
}

async fn check_version_compatibility(peer_state: Arc<PeerState>) -> Result<bool> {
    let min_valid_version = peer_state
        .chain_data_config_manager
        .get_min_version_requirement()
        .await
        .map_err(|e| unexpected_err(e, Some("Could not get min version requirement".into())))?;
    let max_valid_version = peer_state
        .chain_data_config_manager
        .get_max_version_requirement()
        .await
        .map_err(|e| unexpected_err(e, Some("Could not get max version requirement".into())))?;
    is_compatible_version(
        &get_version().to_string(),
        min_valid_version,
        max_valid_version,
    )
}

fn is_compatible_version(
    version: &str,
    min_valid_version: Version,
    max_valid_version: Version,
) -> Result<bool> {
    debug!(
        "Checking version compatibility: version: {}, min_valid_version: {:?}, max_valid_version: {:?}",
        version, min_valid_version, max_valid_version
    );

    // Parse version (eg. "0.2.14"), otherwise known as NODE_VERSION_UNMARKED!
    let version_parts = version.split('.').collect::<Vec<&str>>();
    if version_parts.len() != 3 {
        return Err(unexpected_err(
            format!("Invalid version: {}", version),
            None,
        ));
    }
    let curr_major = U256::from_dec_str(version_parts[0]).map_err(|e| unexpected_err(e, None))?;
    let curr_minor = U256::from_dec_str(version_parts[1]).map_err(|e| unexpected_err(e, None))?;
    let curr_patch = U256::from_dec_str(version_parts[2]).map_err(|e| unexpected_err(e, None))?;

    // If the min_valid_version is set to default values, that means the version is not set on-chain, so we should not check against
    // the minimum version requirement.
    if min_valid_version != Version::default()
        && (curr_major < min_valid_version.major
            || (curr_major == min_valid_version.major && curr_minor < min_valid_version.minor)
            || (curr_major == min_valid_version.major
                && curr_minor == min_valid_version.minor
                && curr_patch < min_valid_version.patch))
    {
        return Ok(false);
    }

    // If the max_valid_version is set to default values, that means the version is not set on-chain, so we should not check against
    // the maximum version requirement.
    if max_valid_version != Version::default()
        && (curr_major > max_valid_version.major
            || (curr_major == max_valid_version.major && curr_minor > max_valid_version.minor)
            || (curr_major == max_valid_version.major
                && curr_minor == max_valid_version.minor
                && curr_patch > max_valid_version.patch))
    {
        return Ok(false);
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::is_compatible_version;
    use crate::tasks::{fsm::parse_epoch_number_from_dkg_id, utils::parse_version};
    use crate::version::get_unmarked_version;
    use lit_blockchain::contracts::staking::Version;

    struct TestCase {
        node_version: String,
        min_valid_version: Version,
        max_valid_version: Version,
        expected_result: bool,
    }

    #[test]
    fn test_version_compatibility() {
        let test_cases = get_version_compability_test_cases();
        for (i, test_case) in test_cases.iter().enumerate() {
            let min_valid_version = test_case.min_valid_version.clone();
            let max_valid_version = test_case.max_valid_version.clone();
            let result = is_compatible_version(
                &test_case.node_version,
                min_valid_version,
                max_valid_version,
            )
            .expect("Failed to check version compatibility");
            assert_eq!(
                result,
                test_case.expected_result,
                "Test case {} failed",
                i + 1
            );
        }
    }

    fn get_version_compability_test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                node_version: get_unmarked_version().to_string(),
                min_valid_version: Version::default(),
                max_valid_version: Version::default(),
                expected_result: true,
            },
            // Test patch version
            TestCase {
                node_version: get_unmarked_version().to_string(),
                min_valid_version: parse_version("0.2.13").expect("Unable to parse version"),
                max_valid_version: Version::default(),
                expected_result: true,
            },
            TestCase {
                node_version: get_unmarked_version().to_string(),
                min_valid_version: parse_version("0.2.15").expect("Unable to parse version"),
                max_valid_version: Version::default(),
                expected_result: false,
            },
            TestCase {
                node_version: get_unmarked_version().to_string(),
                min_valid_version: Version::default(),
                max_valid_version: parse_version("0.2.14").expect("Unable to parse version"),
                expected_result: true,
            },
            TestCase {
                node_version: get_unmarked_version().to_string(),
                min_valid_version: Version::default(),
                max_valid_version: parse_version("0.2.13").expect("Unable to parse version"),
                expected_result: false,
            },
            TestCase {
                node_version: get_unmarked_version().to_string(),
                min_valid_version: parse_version("0.2.13").expect("Unable to parse version"),
                max_valid_version: parse_version("0.2.15").expect("Unable to parse version"),
                expected_result: true,
            },
            // Test minor version
            TestCase {
                node_version: get_unmarked_version().to_string(),
                min_valid_version: parse_version("0.1.14").expect("Unable to parse version"),
                max_valid_version: Version::default(),
                expected_result: true,
            },
            TestCase {
                node_version: get_unmarked_version().to_string(),
                min_valid_version: parse_version("0.3.14").expect("Unable to parse version"),
                max_valid_version: Version::default(),
                expected_result: false,
            },
            TestCase {
                node_version: get_unmarked_version().to_string(),
                min_valid_version: Version::default(),
                max_valid_version: parse_version("0.1.14").expect("Unable to parse version"),
                expected_result: false,
            },
            TestCase {
                node_version: get_unmarked_version().to_string(),
                min_valid_version: Version::default(),
                max_valid_version: parse_version("0.3.14").expect("Unable to parse version"),
                expected_result: true,
            },
            TestCase {
                node_version: get_unmarked_version().to_string(),
                min_valid_version: parse_version("0.1.14").expect("Unable to parse version"),
                max_valid_version: parse_version("0.3.14").expect("Unable to parse version"),
                expected_result: true,
            },
            // Test major version
            TestCase {
                node_version: "1.2.14".to_string(),
                min_valid_version: parse_version("0.2.14").expect("Unable to parse version"),
                max_valid_version: Version::default(),
                expected_result: true,
            },
            TestCase {
                node_version: "1.2.14".to_string(),
                min_valid_version: parse_version("2.2.14").expect("Unable to parse version"),
                max_valid_version: Version::default(),
                expected_result: false,
            },
            TestCase {
                node_version: "1.2.14".to_string(),
                min_valid_version: Version::default(),
                max_valid_version: parse_version("0.2.14").expect("Unable to parse version"),
                expected_result: false,
            },
            TestCase {
                node_version: "1.2.14".to_string(),
                min_valid_version: Version::default(),
                max_valid_version: parse_version("2.2.14").expect("Unable to parse version"),
                expected_result: true,
            },
            TestCase {
                node_version: "1.2.14".to_string(),
                min_valid_version: parse_version("0.2.14").expect("Unable to parse version"),
                max_valid_version: parse_version("2.2.14").expect("Unable to parse version"),
                expected_result: true,
            },
        ]
    }

    #[test]
    fn test_parse_epoch_number() {
        let epoch_number = parse_epoch_number_from_dkg_id("EPOCH_DKG_10_151")
            .expect("Failed to parse epoch number");
        assert_eq!(epoch_number.as_usize(), 10usize);
    }
}

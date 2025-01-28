use super::models::*;
use super::models::{BeaverMessage, RequestMapResponse, SimpleHash};
use crate::config::{LitNodeConfig, CFG_KEY_ECDSA_ROUND_TIMEOUT_MS_DEFAULT};
use crate::error::{self, unexpected_err};
use crate::p2p_comms::CommsManager;
use crate::peers::peer_state::models::{SimplePeer, SimplePeerExt};
#[cfg(feature = "rtmetrics")]
use crate::tasks::realtime_metrics::{MetricAction, MetricActionType, MetricsMessage::NewAction};
use crate::tasks::realtime_metrics::{MetricsMessage, RealTimeTripleCounter};
use crate::tss::common::storage::{
    delete_beaver_triple, read_beaver_triple_from_disk, write_beaver_triple_to_disk,
};
use elliptic_curve::sec1::ToEncodedPoint;
use flume::Sender;
use lit_core::config::ReloadableLitConfig;
use std::sync::Arc;
use std::time::Duration;
use tracing::instrument;

impl BeaverManager {
    pub async fn listen(
        &mut self,
        mut quit_rx: tokio::sync::broadcast::Receiver<bool>,
        config: ReloadableLitConfig,
    ) {
        info!("Listening for requests to create new triples...");
        // these are transitory requests for triples when this node is the leader.
        let mut request_map = ActiveTripleMap::new();
        // either load the triples from disk and/or generate new ones.
        let mut triple_list = self.load_from_disk(true).await;
        info!(
            "Loaded {} triples from disk. Leader for {}. ",
            self.current_generation_count,
            triple_list.total_shares_count(),
        );

        self.set_chain_defaults().await;

        let cfg = config.load_full();
        let tx_metrics = self.tss_state.tx_metrics_manager.clone();
        let timeout = cfg
            .ecdsa_round_timeout()
            .unwrap_or(CFG_KEY_ECDSA_ROUND_TIMEOUT_MS_DEFAULT) as u64;

        loop {
            let mut heartbeat = tokio::time::interval(Duration::from_millis(2 * timeout));
            heartbeat.tick().await; // First tick is immediate
            tokio::select! {
                biased;
                _ = quit_rx.recv() => {
                    info!("Shutting down: tasks::beaver_manager");
                    break;
                }
                beaver_msg = self.rx.recv_async() =>  {
                    let beaver_msg = match beaver_msg {
                        Ok(m) => m,
                        Err(e) => {
                            error!("Error receiving message: {}", e);
                            continue;
                        }
                    };
                    match beaver_msg {
                        BeaverMessage::RequestTriple(req, tx) => {
                            send_real_time_metrics(tx_metrics.clone(), triple_list.clone()).await;
                            if self.is_leader(req.clone()).await {
                                // if we're the leader, we need to generate a key and return it.
                                self.leader_node_triple_key_request(req, &mut request_map, &mut triple_list, tx).await;
                            } else {
                                // otherwise, we need to send the request to the leader node.
                                self.get_triple_key_from_remote_host(req, tx).await;
                            }
                        }
                        BeaverMessage::RemoteRequestForStorageKey(req, tx) => {
                            let request_key = TripleRequestKey::from(req.clone());
                            let request_map_response = self.get_cached_storage_key(&request_key, &mut request_map).await;
                            let key = match request_map_response.triple_map_item.triple_key {
                                0 => {
                                    // we need to check if the request is in the map, and if so, return the key.  if it was not in the map, then we need to get the next one from the list.
                                    // this is because a pregen could have completed between the time the first request was made and the second.  we could accidently return a pregen if we don't check this.
                                    if request_map_response.in_request_map {
                                        request_map_response.triple_map_item.to_leader_response()
                                    } else {
                                    // we're the leader, we need to generate a key and return it if it's not in the map
                                     // we could verify that we're the leader here, but a malicious attacker could generate a valid request that indicates we're the leader, so we'll just generate a key and return it.
                                        self.get_next_triple_storage_key_from_triple_list(&mut triple_list, &mut request_map, req.threshold, &req.peers, request_key).await
                                    }


                                }
                                key => request_map_response.triple_map_item.to_leader_response(),
                            };
                            if (tx.send_async(key).await).is_err() {
                                error!("Error returning triple key.");
                            }

                        }
                        // this is a follow up to the original RequestTriple function - because the leader may be a remote note, we send a message back the this channel once that async function has completed.
                        BeaverMessage::FullfillTripleRequest(request_hash, leader_response, peers, tx) => {
                            info!("Fullfilling triple request for hash: {}", request_hash);
                            self.check_to_start_regenerating_triples(request_hash, leader_response.remaining_triple_pairs);
                            if leader_response.triple_storage_key  == 0 {
                                self.generate_real_time_triple(request_hash, &peers, leader_response.remaining_triple_pairs, tx );
                                trace!("BT Cache Miss: {}", request_hash);
                            } else {
                                self.return_triple_from_disk(request_hash, &peers, leader_response, &request_map, tx).await;
                                trace!("BT Cache Hit: {}", request_hash);
                            }
                        }
                        BeaverMessage::Generate(txn_hash) => {
                            self.beaver_message_generate(&triple_list, txn_hash).await;
                        }
                        BeaverMessage::Store(triple_pair_box, request_hash) => {
                            send_real_time_metrics(tx_metrics.clone(), triple_list.clone()).await;
                            self.beaver_message_store(&mut triple_list, triple_pair_box, request_hash).await;
                        }
                        BeaverMessage::Clear => {
                            self.beaver_message_clear(&mut triple_list).await;
                        }
                        BeaverMessage::Count => {
                            self.beaver_message_count(&mut triple_list).await;
                        }
                        BeaverMessage::RemoveGenerationHash(txn_hash) => {
                            self.generating_txn_ids.retain(|&x| x != txn_hash);
                            warn!("Removed {}, remaining: {:?}", txn_hash, &self.generating_txn_ids);
                        }
                        BeaverMessage::ThresholdReceived(request, tx) => {
                            self.broadcast_selection(request, &mut request_map, &mut triple_list, tx).await;
                        }
                    }
                }
                _ = heartbeat.tick() => {
                    self.set_chain_defaults().await;
                    self.clean_request_map(&mut request_map, timeout).await;
                }
            }
        }
    }

    async fn set_chain_defaults(&mut self) {
        let generic_chain_config = self
            .tss_state
            .peer_state
            .chain_data_config_manager
            .generic_config
            .read()
            .await;
        self.min_triples = generic_chain_config.min_triple_count;
        self.max_triples = generic_chain_config.max_triple_count;
        self.max_triple_concurrency = generic_chain_config.max_triple_concurrency;
    }

    #[instrument(skip_all, fields(txn_prefix = req.txn_prefix))]
    async fn is_leader(&self, req: TripleRequest) -> bool {
        let request_key = TripleRequestKey::from(req);
        let request_key_hash = request_key.hash();
        let peers = match self.tss_state.peer_state.peers().await {
            Ok(p) => p.active_peers(),
            Err(e) => {
                error!("Error getting peers: {}", e);
                return false;
            }
        };
        peers.address_is_leader(request_key_hash, &self.tss_state.addr)
    }

    #[instrument(skip_all, fields(txn_prefix = req.txn_prefix))]
    async fn leader_node_triple_key_request(
        &mut self,
        req: TripleRequest,
        request_map: &mut ActiveTripleMap,
        triple_list: &mut TripleListByGroup,
        tx: Sender<error::Result<Option<BeaverTriplePair>>>,
    ) {
        let request_key = TripleRequestKey::from(req.clone());
        let request_hash = request_key.hash();
        info!(
            "Received request for triple key.  Hash value : {}.",
            request_hash
        );

        let txn_prefix = format!("GET_TRP_{}", request_hash);
        let round = "0";
        let state = self.signing_state.state.clone();
        let mut signing_peers = req.peers.clone();
        signing_peers.set_all_protocol_indices(0);

        info!(
            "Setting up leader for {} for {:?}",
            txn_prefix,
            signing_peers.debug_addresses()
        );

        let cm = match CommsManager::new(&state, 0, &txn_prefix, round).await {
            Ok(c) => c,
            Err(e) => {
                error!("Error setting up comms manager: {}", e);
                if tx
                    .send_async(Err(unexpected_err(
                        e,
                        Some("Error setting up comms manager.".to_string()),
                    )))
                    .await
                    .is_err()
                {
                    error!("Error returning triple error.");
                }
                return;
            }
        };

        let self_tx = self.tx.clone();

        tokio::spawn(async move {
            let r = cm.collect::<String>().await;

            info!(
                "All peers have acknowledged required triple key request {} . ",
                txn_prefix
            );

            if self_tx
                .send_async(BeaverMessage::ThresholdReceived(req, tx))
                .await
                .is_err()
            {
                error!("Error returning threshold received message.");
            }
        });
    }

    async fn broadcast_selection(
        &mut self,
        req: TripleRequest,
        request_map: &mut ActiveTripleMap,
        triple_list: &mut TripleListByGroup,
        tx: Sender<error::Result<Option<BeaverTriplePair>>>,
    ) {
        let request_key = TripleRequestKey::from(req.clone());
        let request_hash = request_key.hash();
        let txn_prefix = format!("GET_TRP_{}", request_hash);
        let round = "0";
        let state = self.signing_state.state.clone();
        let mut signing_peers = req.peers.clone();
        signing_peers.set_all_protocol_indices(0);

        let cm = match CommsManager::new(&state, 0, &txn_prefix, round).await {
            Ok(c) => c,
            Err(e) => {
                error!("Error setting up comms manager: {}", e);
                if tx
                    .send_async(Err(unexpected_err(
                        e,
                        Some("Error setting up comms manager.".to_string()),
                    )))
                    .await
                    .is_err()
                {
                    error!("Error returning triple error.");
                }
                return;
            }
        };

        let leader_response = self
            .get_storage_key_from_request_key(
                request_key.clone(),
                &req.peers,
                request_map,
                triple_list,
                false,
                req.threshold,
                tx.clone(),
            )
            .await;

        let self_tx = self.tx.clone();
        tokio::spawn(async move {
            info!(
                "Sending triple key to peers {:?} for triple key request {} . ",
                req.peers.debug_addresses(),
                txn_prefix
            );

            if let Err(e) = cm.broadcast(leader_response.clone()).await {
                error!("Error broadcasting triple key to peers: {}", e);
                if tx
                    .send_async(Err(unexpected_err(
                        e,
                        Some("Error sending triple key to peers.".to_string()),
                    )))
                    .await
                    .is_err()
                {
                    error!("Error returning triple error.");
                }
                return;
            };

            let beaver_msg = BeaverMessage::FullfillTripleRequest(
                request_hash,
                leader_response,
                req.peers.clone(),
                tx.clone(),
            );

            if self_tx.send_async(beaver_msg).await.is_err() {
                error!("Error returning triple key.");
            }
        });
    }

    #[doc = "Generates new triples"]
    #[instrument(skip_all)]
    async fn beaver_message_generate(&mut self, triple_list: &TripleListByGroup, txn_hash: u64) {
        let signing_state = self.signing_state.clone();
        // let n = 1;
        let tx = self.tx.clone();
        let txn_prefix = format!("pregen_{}", txn_hash);
        let start = std::time::Instant::now();
        let active_peers = match self.tss_state.peer_state.peers().await {
            Ok(p) => p.active_peers(),
            Err(e) => {
                error!("Error getting peers: {}", e);
                return;
            }
        };

        if !active_peers.contains_address(&self.tss_state.addr) {
            let _r = tx
                .send_async(BeaverMessage::RemoveGenerationHash(txn_hash))
                .await;
            warn!("Non active peer was asked to generate triples. Aborting request to generate; other peers will still succeed.");
            return;
        }

        let some_peers = Some(active_peers);

        tokio::spawn(async move {
            let tx_metrics = signing_state.state.tx_metrics_manager.clone();
            #[cfg(feature = "rtmetrics")]
            let _ = tx_metrics
                .send_async(NewAction(MetricAction {
                    type_id: MetricActionType::BeaverTriple,
                    txn_id: generate_hash(txn_prefix.clone()),
                    is_start: true,
                    is_success: true,
                }))
                .await;

            let triple_result = signing_state
                .generate_triple_pair(some_peers, txn_prefix.clone())
                .await;

            #[cfg(feature = "rtmetrics")]
            let _ = tx_metrics
                .send_async(NewAction(MetricAction {
                    type_id: MetricActionType::BeaverTriple,
                    txn_id: generate_hash(txn_prefix.to_string()),
                    is_start: false,
                    is_success: triple_result.is_ok(),
                }))
                .await;

            match triple_result {
                Ok(triple_pair) => {
                    debug!(
                        "Generated triple pair in {} ms.",
                        start.elapsed().as_millis()
                    );
                    if let Err(e) = tx
                        .send_async(BeaverMessage::Store(Box::new(triple_pair), txn_hash))
                        .await
                    {
                        error!("Error sending triple pair to store: {}", e);
                    }
                }
                Err(e) => {
                    let _r = tx
                        .send_async(BeaverMessage::RemoveGenerationHash(txn_hash))
                        .await;

                    error!(
                        "Error generating triple pair: {} in {} ms",
                        e,
                        start.elapsed().as_millis()
                    );
                }
            }
        });
    }

    #[doc = "Clears the triple list from internal map and disk"]
    #[instrument(skip_all)]
    async fn beaver_message_clear(&mut self, triple_list: &mut TripleListByGroup) {
        info!("Clearing and starting repopulation...");
        let old_map = triple_list.clone();
        triple_list.clear();
        let share_index = match self
            .tss_state
            .peer_state
            .node_index_in_current_epoch()
            .await
        {
            Ok(i) => i as u16,
            Err(e) => {
                error!("Error getting node index: {}", e);
                return;
            }
        };

        let staker_address = self.tss_state.peer_state.hex_staker_address();

        for key in old_map.triple_list_values() {
            let staker_address = staker_address.clone();
            tokio::spawn(async move {
                // delete the files
                if let Err(e) =
                    delete_beaver_triple(&key.to_string(), share_index, &staker_address).await
                {
                    error!("Error deleting triple file: {:?}", e);
                }
            });
        }

        info!("Cleared triple list; actual triples on disk are being deleted.");
    }

    #[doc = "Counts the triples for this node from it's internal map."]
    #[instrument(skip_all)]
    async fn beaver_message_count(&mut self, triple_list: &mut TripleListByGroup) {
        info!(
            "This node is the leader for {} triples.",
            triple_list.total_shares_count()
        );
    }

    #[doc = "Stores a triple pair to disk and adds it to the triple list.  Will request another triple to be generated if the list is not full."]
    #[instrument(skip_all)]
    async fn beaver_message_store(
        &mut self,
        triple_list: &mut TripleListByGroup,
        triple_pair_box: Box<BeaverTriplePair>,
        request_hash: u64,
    ) {
        let triple_pair = *triple_pair_box;
        let triple_storage_key = Self::triple_key_from_triple_pair(&triple_pair);

        info!("Storing triple pair... key : {}", triple_storage_key);
        let staker_address = self.tss_state.peer_state.hex_staker_address();

        if let Err(e) = write_beaver_triple_to_disk(
            &triple_storage_key.to_string(),
            0, // hardcode 0 for share index, since it doesn't matter
            &staker_address,
            &triple_pair,
        )
        .await
        {
            error!("Error writing triple to disk: {}", e);
        };

        self.current_generation_count += 1;

        // if we're to be the "leader" for this triple, add it to the list.
        let peers = match self.tss_state.peer_state.peers().await {
            Ok(p) => p.active_peers(),
            Err(e) => {
                error!("Error getting peers: {}", e);
                return;
            }
        };
        if peers.address_is_leader(triple_storage_key, &self.tss_state.addr) {
            triple_list.add_storage_key(triple_pair.peer_group_id, triple_storage_key);
        }
        let xor_filter_with_threshold = XorFilterWithThreshold {
            filter: triple_pair.xor_filter,
            threshold: triple_pair.pub0.threshold,
        };

        self.xor_filters
            .entry(triple_pair.peer_group_id)
            .or_insert(xor_filter_with_threshold);
        // end of adding triple to the list, if I'm the leader.

        // remove the range of triples being generated that is greater than the max, as sorted by u64 (hash value).
        self.generating_txn_ids.sort();
        if self.generating_txn_ids.len() > self.max_triple_concurrency as usize {
            let range_to_remove =
                (self.max_triple_concurrency as usize)..self.generating_txn_ids.len();
            self.generating_txn_ids.drain(range_to_remove);
        }

        // if we're no longer in the list, we're done!
        if !self.generating_txn_ids.contains(&request_hash) {
            return;
        }

        self.generating_txn_ids.retain(|&x| x != request_hash);

        // check if we should keep generating
        // also, make it possible to disable triple pregen by setting max triples to 0
        if self.current_generation_count < self.max_triples && self.max_triples != 0 {
            // this blocks other concurrent requests from coming in.
            // it's possible that two or three happen right at the start and eventually some get aborted.
            self.last_generated = std::time::Instant::now(); // not correct, but useful if all requests fail.
            info!(
                "Requesting more triples. Generation at {} of {}.",
                self.current_generation_count, self.max_triples
            );

            let next_request_hash = request_hash + 1;
            self.generating_txn_ids.push(next_request_hash);
            warn!(
                "Requesting more triples. Generation at {} of {}. \n Removed {} : current {:?}",
                self.current_generation_count,
                self.max_triples,
                request_hash,
                &self.generating_txn_ids
            );
            if let Err(e) = self
                .tx
                .send_async(BeaverMessage::Generate(next_request_hash))
                .await
            {
                error!("Error sending generate message: {}", e);
            };
        } else {
            info!(
                "Finished generating {} triples.",
                self.current_generation_count
            );
        }
    }

    #[doc = "Executes a triple generation process in a new thread and returns it directly to the calling function - used when a triple is needed in real time and the list is empty."]
    #[instrument(fields(txn_prefix = format!("rt_{}", triple_hash)))]
    fn generate_real_time_triple(
        &mut self,
        triple_hash: TripleRequestKeyHash,
        peers: &Vec<SimplePeer>,
        remaining_triples: u64,
        tx: Sender<error::Result<Option<BeaverTriplePair>>>,
    ) {
        let signing_state = self.signing_state.clone();
        let txn_prefix = format!("rt_{}", triple_hash);
        let some_peers = Some(peers.clone());
        let included = peers.contains_address(&self.tss_state.addr);
        let peers = peers.clone();
        let local_tx = self.tx.clone();
        trace!("BT Cache Miss: {}", triple_hash);

        tokio::spawn(async move {
            // if we're not part of this generation, just return a blank triple.
            if !included {
                trace!("Not included in this generation.  Returning blank triple.");
                if (tx.send_async(Ok(None)).await).is_err() {
                    error!("Error returning triple.");
                }
                return;
            }
            trace!(
                "Generating real time triple pair {} with peers {:?}.",
                txn_prefix,
                peers.debug_addresses()
            );

            let tx_metrics = signing_state.state.tx_metrics_manager.clone();
            #[cfg(feature = "rtmetrics")]
            let _ = tx_metrics
                .send_async(NewAction(MetricAction {
                    type_id: MetricActionType::BeaverTriple,
                    txn_id: generate_hash(txn_prefix.clone()),
                    is_start: true,
                    is_success: true,
                }))
                .await;

            let triple_result = signing_state
                .generate_triple_pair(some_peers, txn_prefix.clone())
                .await;

            #[cfg(feature = "rtmetrics")]
            let _ = tx_metrics
                .send_async(NewAction(MetricAction {
                    type_id: MetricActionType::BeaverTriple,
                    txn_id: generate_hash(txn_prefix.clone()),
                    is_start: false,
                    is_success: triple_result.is_ok(),
                }))
                .await;

            match triple_result {
                Ok(result) => {
                    info!("Generated triple pair in real time: {}", txn_prefix);
                    if let Err(e) = tx.send_async(Ok(Some(result))).await {
                        error!("Error returning triple: {}", e);
                    }
                }
                Err(e) => {
                    error!("Error retrieving triple: {}", e);
                    if let Err(e) = tx
                        .send_async(Err(unexpected_err("Could not retrieve triple.", None)))
                        .await
                    {
                        error!("Error returning triple err: {}", e);
                    }
                }
            };
        });
    }

    #[doc = "Checks to see if we need to start generating triples, and if so, sends a message to the channel to do so."]
    pub fn check_to_start_regenerating_triples(
        &mut self,
        triple_hash: u64,
        remaining_triples: u64,
    ) {
        // if the incoming remaining triples is above the low threshold mark, don't do anything.
        // also, make it possible to disable triple pregen by setting max triples to 0
        if remaining_triples > self.min_triples || self.max_triples == 0 {
            return;
        }

        // set the "counter" to whatever the leader says needs to be generated.
        self.current_generation_count = remaining_triples;
        self.generating_txn_ids.push(triple_hash);
        let local_tx = self.tx.clone();
        // this spawn probably doesn't add much - it's a pretty quick message to send.
        tokio::spawn(async move {
            trace!("Sending generate message for hash: {}", triple_hash);
            if let Err(e) = local_tx
                .send_async(BeaverMessage::Generate(triple_hash))
                .await
            {
                error!("Error sending generate message: {}", e);
            };
        });
    }

    // is this unique enough?
    pub fn triple_key_from_triple_pair(triple_pair: &BeaverTriplePair) -> u64 {
        generate_hash(triple_pair.pub0.big_a.to_encoded_point(false))
    }

    // error handling to return a message back to the request originator - ie cait-sith
    pub async fn return_error_to_requester(
        &self,
        tx: Sender<error::Result<Option<BeaverTriplePair>>>,
        error: String,
    ) {
        error!("Error getting triple: {}", error);
        if (tx.send_async(Err(unexpected_err(error, None))).await).is_err() {
            error!("Error returning triple through channel.");
        }
    }

    #[doc = "Gets a triple 'storage key'  (public key?) from the active request map, based on a key - returns 0, if none exists."]
    #[instrument(skip_all)]
    async fn get_cached_storage_key(
        &mut self,
        request_key: &TripleRequestKey,
        request_map: &mut ActiveTripleMap,
    ) -> RequestMapResponse {
        let request_hash = request_key.hash();
        info!("Check request map cache for hash: {} ", request_hash);
        // find the key either because someone has requested one with this hash already (ie, other node) or from the local storage.
        match request_map.contains_key(&request_hash) {
            true => {
                let triple_map_item = request_map.get(&request_hash);
                info!("Key in request map: {:?}.", triple_map_item);
                if let Some(map_item) = triple_map_item {
                    let key_response = map_item.triple_key;
                    info!(
                        "Found (hash :{}) key response: {}.",
                        request_hash, key_response,
                    );
                    // return the item in the map, and indicate that it was found
                    RequestMapResponse {
                        triple_map_item: map_item.clone(),
                        in_request_map: true,
                    }
                } else {
                    RequestMapResponse {
                        triple_map_item: TripleMapItem::zero(),
                        in_request_map: false,
                    }
                }
            }
            false => RequestMapResponse {
                triple_map_item: TripleMapItem::zero(),
                in_request_map: false,
            },
        }
    }

    #[instrument(skip_all)]
    #[allow(clippy::too_many_arguments)]
    async fn get_storage_key_from_request_key(
        &mut self,
        request_key: TripleRequestKey,
        request_peers: &Vec<SimplePeer>,
        request_map: &mut ActiveTripleMap,
        triple_list: &mut TripleListByGroup,
        is_local_request: bool,
        threshold: u16,
        tx: Sender<error::Result<Option<BeaverTriplePair>>>,
    ) -> TripleLeaderResponse {
        let request_hash = request_key.hash();
        info!("Getting next pair for hash: {} ", request_hash);
        info!("Request Map {:?}", request_map);
        // find the key either because someone has requested one with this hash already (ie, other node) or from the local storage.
        let request_map_response = self.get_cached_storage_key(&request_key, request_map).await;
        let leader_response = match request_map_response.triple_map_item.triple_key {
            0 => {
                // we need to check if the request is in the map, and if so, return the key.  if it was not in the map, then we need to get the next one from the list.
                // this is because a pregen could have completed between the time the first request was made and the second.  we could accidently return a pregen if we don't check this.
                if request_map_response.in_request_map {
                    request_map_response.triple_map_item.to_leader_response()
                } else {
                    // if we're the leader, we need to generate a key and return it.
                    // get the next key from our list
                    let leader_response = self
                        .get_next_triple_storage_key_from_triple_list(
                            triple_list,
                            request_map,
                            threshold,
                            request_peers,
                            request_key,
                        )
                        .await;
                    info!(
                        "Got local key for {}, from triple list: {}.  Triples remaining: {}",
                        request_hash,
                        leader_response.triple_storage_key,
                        leader_response.remaining_triple_pairs
                    );
                    leader_response
                }
            }
            key => TripleLeaderResponse {
                triple_storage_key: key,
                remaining_triple_pairs: request_map_response.triple_map_item.remaining_triple_pairs,
            },
        };

        if leader_response.triple_storage_key == 0 {
            warn!("Leader has no active Triples.");
        }

        leader_response
    }

    #[instrument(skip_all)]
    async fn return_triple_from_disk(
        &mut self,
        request_hash: TripleRequestKeyHash,
        peers: &Vec<SimplePeer>,
        leader_response: TripleLeaderResponse,
        request_map: &ActiveTripleMap,
        tx: Sender<error::Result<Option<BeaverTriplePair>>>,
    ) {
        let pubkey = leader_response.triple_storage_key.to_string();

        let start = std::time::Instant::now();
        let staker_address = self.tss_state.peer_state.hex_staker_address();

        let included = peers.contains_address(&self.tss_state.addr);
        tokio::spawn(async move {
            let triple = match read_beaver_triple_from_disk::<BeaverTriplePair>(
                &pubkey,
                0,
                &staker_address, //hardcode 0 for share index, since it doesn't matter,
            )
            .await
            {
                Ok(s) => s as BeaverTriplePair,
                Err(e) => {
                    if e.is_kind(lit_core::error::Kind::Io, true) {
                        // If there is an error reading the file, we assume that it is because the file doesn't exist.
                        // In that case, we first check whether this peer is part of the peer subset:
                        // - If yes, this is a serious error and we should produce an error log.
                        // - If not, this is an expected scenario and we should produce an info log and return Ok(None).
                        if included {
                            error!("Error reading triple from disk: {} and self is included in peers - this is an issue.", e);
                            match tx.send_async(Err(e)).await {
                                Ok(_) => {
                                    trace!("Returned error to triple requester");
                                }
                                Err(e) => {
                                    error!("Error returning error to triple requester: {}", e);
                                }
                            }
                            return;
                        } else {
                            debug!(
                                "Could not read triple from disk: {}, but self is not in peers, so this is not an issue.",
                                e
                            );
                            match tx.send_async(Ok(None)).await {
                                Ok(_) => {
                                    trace!("Returned response to triple requester");
                                }
                                Err(e) => {
                                    error!("Error returning response to triple requester: {}", e);
                                }
                            }
                            return;
                        }
                    } else {
                        error!("Error reading triple from disk: {}", e);
                        match tx.send_async(Err(e)).await {
                            Ok(_) => {
                                trace!("Returned error to triple requester");
                            }
                            Err(e) => {
                                error!("Error returning error to triple requester: {}", e);
                            }
                        }
                        return;
                    }
                }
            };

            // delete the file only if this is a local request.
            info!("Deleting local triple file: {}", pubkey);
            //hardcode 0 for share index, since it doesn't matter,
            if let Err(e) = delete_beaver_triple(&pubkey, 0, &staker_address).await {
                error!("Error deleting triple file: {:?}", e);
                match tx.send_async(Err(e)).await {
                    Ok(_) => {
                        trace!("Returned error to triple requester");
                    }
                    Err(e) => {
                        error!("Error returning error to triple requester: {}", e);
                    }
                }
                return;
            }

            match tx.send_async(Ok(Some(triple))).await {
                Ok(_) => {
                    info!("Successfully returned triple response.");
                }
                Err(e) => {
                    error!("Error returning triple: {}", e);
                }
            }
        });

        info!("Raw read & delete from disk in {:?}.", start.elapsed());
    }

    #[instrument(skip_all)]
    async fn get_next_triple_storage_key_from_triple_list(
        &mut self,
        triple_list: &mut TripleListByGroup,
        request_map: &mut ActiveTripleMap,
        threshold: u16,
        peers: &Vec<SimplePeer>,
        request_key: TripleRequestKey,
    ) -> TripleLeaderResponse {
        let request_hash = request_key.hash();
        info!("We are the leader node - getting key from local storage.");

        let peer_group_id =
            self.get_peer_group_id_from_xor_filter(triple_list, peers, threshold as usize);
        if peer_group_id == 0 {
            info!("No triples for peers: {:?}", peers.debug_addresses());
            // insert 0 into request map
            // so that subsequent requests don't hit a triple
            // that was generated between the first request and subsequent ones
            request_map.insert(request_hash, TripleMapItem::zero());
            return TripleLeaderResponse {
                triple_storage_key: 0,
                remaining_triple_pairs: 0,
            }; // if we don't have any triples for this group, return 0.
        }

        // technically this should always be something, even if it's empty, and we already did a len check on the quantity....
        let triple_list = match triple_list.get_mut(&peer_group_id) {
            Some(v) => v,
            None => {
                error!(
                    "Error getting triple list from hashmap for group id: {}",
                    peer_group_id
                );
                return TripleLeaderResponse {
                    triple_storage_key: 0,
                    remaining_triple_pairs: 0,
                };
            }
        };

        // get the next key from our list
        let triple_key = triple_list.pop_front();

        // add it to the hashmap & return, or return 0 if something failed.
        let triple_key = triple_key.unwrap_or(0);

        if triple_key == 0 {
            warn!("Leader has no active Triples.");
        };

        let request_hash = request_key.hash();
        info!(
            "Adding key to request map {:?} -> {:?}  ",
            request_hash, triple_key
        );

        // this may not be relevant for just storing in the map, but to match the leader request....
        let remaining_triple_pairs = match self.generating_txn_ids.is_empty() {
            true => {
                if (triple_list.len() as u64) < self.min_triples {
                    // this returns a fixed value since these results are sometimes returned in varying orders
                    // a fixed results ensures that the "last" value received by every node is the same, making
                    // the number of triples to be generated equivalent across all nodes
                    self.min_triples
                } else {
                    triple_list.len() as u64
                }
            }
            false => self.max_triples,
        };
        let item = TripleMapItem {
            triple_key,
            threshold,
            remaining_triple_pairs,
            created_at: chrono::Utc::now().timestamp_millis() as u64,
        };
        request_map.insert(request_hash, item.clone());

        item.to_leader_response()
    }

    // function to periodically check the request map and delete old entries
    #[instrument(skip_all)]
    pub async fn clean_request_map(&mut self, request_map: &mut ActiveTripleMap, timeout: u64) {
        let now = chrono::Utc::now().timestamp_millis() as u64;
        request_map.retain(|key, value| ((now - value.created_at) < timeout))
    }

    #[instrument(skip_all)]
    async fn get_triple_key_from_remote_host(
        &self,
        req: TripleRequest,
        tx: Sender<error::Result<Option<BeaverTriplePair>>>,
    ) {
        info!("Getting triple key from remote host.");

        let request_key = TripleRequestKey::from(req.clone());
        let request_key_hash = request_key.hash();

        let txn_prefix = format!("GET_TRP_{}", request_key_hash);
        let round = "0";
        let state = self.signing_state.state.clone();
        let mut signing_peers = req.peers.clone();
        signing_peers.set_all_protocol_indices(0);
        let mut all_peers = match state.peer_state.peers().await {
            Ok(p) => p.active_peers(),
            Err(e) => {
                error!("Error getting peers: {}", e);
                if tx.send_async(Err(unexpected_err(e, None))).await.is_err() {
                    error!("Error returning triple error.");
                }
                return;
            }
        };
        all_peers.set_all_protocol_indices(0);

        info!(
            "Getting triples using {} with signing peers: {:?}",
            txn_prefix,
            signing_peers.debug_addresses()
        );

        let cm = match CommsManager::new(&state, 0, &txn_prefix, round).await {
            Ok(c) => c,
            Err(e) => {
                error!("Error setting up comms manager: {}", e);
                if tx
                    .send_async(Err(unexpected_err(
                        e,
                        Some("Error setting up comms manager.".to_string()),
                    )))
                    .await
                    .is_err()
                {
                    error!("Error returning triple error.");
                }
                return;
            }
        };

        let all_peers_cloned = all_peers.clone();
        let leader_peer = match all_peers_cloned.leader_for_active_peers(request_key_hash) {
            Ok(peer) => peer,
            Err(e) => {
                if tx.send_async(Err(e)).await.is_err() {
                    error!("Error returning triple error.");
                }
                return;
            }
        }
        .clone();

        info!(
            "Requesting triples for {} from {}",
            txn_prefix, leader_peer.socket_address
        );

        let local_tx = self.tx.clone();

        tokio::spawn(async move {
            let r = match cm.send_direct(&leader_peer, req.clone()).await {
                Ok(r) => r,
                Err(e) => {
                    error!("Error sending request to leader: {}", e);
                    if tx
                        .send_async(Err(unexpected_err(
                            e,
                            Some("Error sending request to leader.".to_string()),
                        )))
                        .await
                        .is_err()
                    {
                        error!("Error returning triple error.");
                    }
                    return;
                }
            };
            let leader_vec = vec![leader_peer.clone()];

            let triple_leader_response =
                match cm.collect_from::<TripleLeaderResponse>(&leader_vec).await {
                    Ok(r) => r,
                    Err(e) => {
                        error!("Error getting leader response: {}", e);
                        if tx
                            .send_async(Err(unexpected_err(
                                e,
                                Some("Error getting leader response.".to_string()),
                            )))
                            .await
                            .is_err()
                        {
                            error!("Error returning triple error.");
                        }
                        return;
                    }
                };

            let triple_leader_response = match triple_leader_response.into_iter().next() {
                Some(r) => r.1,
                None => {
                    if tx
                        .send_async(Err(unexpected_err("Invalid leader response.", None)))
                        .await
                        .is_err()
                    {
                        error!("Error returning triple error.");
                    }
                    return;
                }
            };

            let beaver_msg = BeaverMessage::FullfillTripleRequest(
                request_key_hash,
                triple_leader_response,
                req.peers.clone(),
                tx,
            );

            // send this remote request back for processing.
            if local_tx.send_async(beaver_msg).await.is_err() {
                error!("Error returning triple key.");
            }
        });
    }
}

async fn send_real_time_metrics(
    tx: Arc<flume::Sender<MetricsMessage>>,
    triple_list: TripleListByGroup,
) {
    tokio::spawn(async move {
        let mut triple_counter: Vec<RealTimeTripleCounter> = Vec::new();
        for bucket in triple_list {
            let peer_group_id = bucket.0;
            let count = bucket.1.len() as u64;
            let rtpc = RealTimeTripleCounter {
                peer_group_id,
                count,
                peer_keys: Vec::new(),
            };
            triple_counter.push(rtpc);
        }

        let item = MetricsMessage::TripleStatus(triple_counter);
        let _r = tx.send_async(item).await;
    });
}

#[instrument(skip_all, fields(txn_prefix = req.txn_prefix))]
pub fn request_hash(req: TripleRequest) -> u64 {
    let request_key = TripleRequestKey::from(req.clone());
    request_key.hash()
}

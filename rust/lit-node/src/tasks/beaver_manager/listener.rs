use super::models::*;
use super::models::{BeaverMessage, SimpleHash};
use crate::config::{LitNodeConfig, CFG_KEY_ECDSA_ROUND_TIMEOUT_MS_DEFAULT};
use crate::error::{self, unexpected_err, Result};
use crate::peers::peer_state::models::{PeerSocketAddress, PeerSocketAddressVec};
use crate::tasks::realtime_metrics::{MetricsMessage, RealTimeTripleCounter};

#[cfg(feature = "rtmetrics")]
use crate::tasks::realtime_metrics::{MetricAction, MetricActionType, MetricsMessage::NewAction};

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
        config: ReloadableLitConfig,
        mut quit_rx: tokio::sync::mpsc::Receiver<bool>,
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
                            self.set_chain_defaults().await;
                            if self.is_leader(req.clone()).await {
                                // if we're the leader, we need to generate a key and return it.
                                self.leader_node_triple_key_request(req, &mut request_map, &mut triple_list, tx).await;
                            } else {
                                // otherwise, we need to send the request to the leader node.
                                self.get_triple_key_from_remote_host( req, tx).await;
                            }
                        }
                        BeaverMessage::RemoteRequestForStorageKey(req, tx) => {
                            let request_key = TripleRequestKey::from(req.clone());
                            let triple_map_item = self.get_cached_storage_key(&request_key, &mut request_map).await;
                            let key = match triple_map_item.triple_key {
                                0 => {
                                    // we're the leader, we need to generate a key and return it.
                                    // we could verify that we're the leader here, but a malicious attacker could generate a valid request that indicates we're the leader, so we'll just generate a key and return it.
                                    self.get_next_triple_storage_key_from_triple_list(&mut triple_list, &mut request_map, req.threshold, &req.peers, request_key).await
                                }
                                key => triple_map_item.to_leader_response(),
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
                        BeaverMessage::RemoveGenerationHash(txn_hash) => {
                            self.generating_txn_ids.retain(|&x| x != txn_hash);
                            warn!("Removed {}, remaining: {:?}", txn_hash, &self.generating_txn_ids);
                        }
                    }
                }
                _ = heartbeat.tick() => {
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
        let peers = match self.tss_state.peer_state.peer_socket_addresses().await {
            Ok(p) => p.active_addresses(),
            Err(e) => {
                error!("Error getting peers: {}", e);
                return false;
            }
        };
        addr_is_leader(request_key_hash, &peers, &self.tss_state.addr)
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

        let leader_response = match request_map.contains_key(&request_hash) {
            true => {
                info!("Found key in request map.");
                let key: Option<&mut TripleMapItem> = request_map.get_mut(&request_hash);
                if let Some(key) = key {
                    info!(
                        "Found (hash :{}) key response: {}.",
                        request_hash, &key.triple_key,
                    );
                    TripleLeaderResponse {
                        triple_storage_key: key.triple_key,
                        remaining_triple_pairs: key.remaining_triple_pairs,
                    }
                } else {
                    error!("Error returning triple key.");
                    TripleLeaderResponse {
                        triple_storage_key: 0,
                        remaining_triple_pairs: 0,
                    }
                }
            }
            false => {
                // otherwise get the next one... note the final "false" parameters which prevents the file from being deleted.
                info!("Key not found in request map.  Getting next key from list.");
                self.get_storage_key_from_request_key(
                    request_key.clone(),
                    &req.peers,
                    request_map,
                    triple_list,
                    false,
                    req.threshold,
                    tx.clone(),
                )
                .await
            }
        };

        let beaver_msg = BeaverMessage::FullfillTripleRequest(
            request_hash,
            leader_response,
            req.peers.clone(),
            tx.clone(),
        );
        if self.tx.send_async(beaver_msg).await.is_err() {
            error!("Error returning triple key.");
        }
    }

    #[doc = "Generates new triples"]
    #[instrument(skip_all)]
    async fn beaver_message_generate(&mut self, triple_list: &TripleListByGroup, txn_hash: u64) {
        let signing_state = self.signing_state.clone();
        // let n = 1;
        let tx = self.tx.clone();
        let txn_prefix = format!("pregen_{}", txn_hash);
        let start = std::time::Instant::now();
        let some_peers = match self.tss_state.peer_state.peer_socket_addresses().await {
            Ok(p) => Some(p.active_peers()),
            Err(e) => {
                error!("Error getting peers: {}", e);
                return;
            }
        };
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
                    // for triple_pair in triple_pairs {
                    if let Err(e) = tx
                        .send_async(BeaverMessage::Store(Box::new(triple_pair), txn_hash))
                        .await
                    {
                        error!("Error sending triple pair to store: {}", e);
                    }
                    // }
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

        for key in old_map.triple_list_values() {
            tokio::spawn(async move {
                // delete the files
                if let Err(e) = delete_beaver_triple(&key.to_string(), share_index).await {
                    error!("Error deleting triple file: {:?}", e);
                }
            });
        }
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
        if let Err(e) = write_beaver_triple_to_disk(
            &triple_storage_key.to_string(),
            0, // hardcode 0 for share index, since it doesn't matter
            &triple_pair,
        )
        .await
        {
            error!("Error writing triple to disk: {}", e);
        };

        self.current_generation_count += 1;

        // if we're to be the "leader" for this triple, add it to the list.
        let peers = match self.tss_state.peer_state.peer_socket_addresses().await {
            Ok(p) => p.active_addresses(),
            Err(e) => {
                error!("Error getting peers: {}", e);
                return;
            }
        };
        if addr_is_leader(triple_storage_key, &peers, &self.tss_state.addr) {
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

        if self.current_generation_count < self.max_triples {
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
        peers: &Vec<PeerSocketAddress>,
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
                peers.all_addresses()
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
        if remaining_triples > self.min_triples {
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
    ) -> TripleMapItem {
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
                    map_item.clone()
                } else {
                    TripleMapItem::zero()
                }
            }
            false => TripleMapItem::zero(),
        }
    }

    #[instrument(skip_all)]
    #[allow(clippy::too_many_arguments)]
    async fn get_storage_key_from_request_key(
        &mut self,
        request_key: TripleRequestKey,
        request_peers: &Vec<PeerSocketAddress>,
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
        let triple_map_item = self.get_cached_storage_key(&request_key, request_map).await;
        let leader_response = match triple_map_item.triple_key {
            0 => {
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
            key => TripleLeaderResponse {
                triple_storage_key: key,
                remaining_triple_pairs: triple_map_item.remaining_triple_pairs,
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
        peers: &Vec<PeerSocketAddress>,
        leader_response: TripleLeaderResponse,
        request_map: &ActiveTripleMap,
        tx: Sender<error::Result<Option<BeaverTriplePair>>>,
    ) {
        let pubkey = leader_response.triple_storage_key.to_string();

        let start = std::time::Instant::now();

        let included = peers.contains_address(&self.tss_state.addr);
        tokio::spawn(async move {
            let triple = match read_beaver_triple_from_disk::<BeaverTriplePair>(
                &pubkey, 0, //hardcode 0 for share index, since it doesn't matter,
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
            if let Err(e) = delete_beaver_triple(&pubkey, 0).await {
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

        // self.current_triples -= 1;
    }

    #[instrument(skip_all)]
    async fn get_next_triple_storage_key_from_triple_list(
        &mut self,
        triple_list: &mut TripleListByGroup,
        request_map: &mut ActiveTripleMap,
        threshold: u16,
        peers: &Vec<PeerSocketAddress>,
        request_key: TripleRequestKey,
    ) -> TripleLeaderResponse {
        let request_hash = request_key.hash();
        info!("We are the leader node - getting key from local storage.");

        let peer_group_id =
            self.get_peer_group_id_from_xor_filter(triple_list, peers, threshold as usize);
        if peer_group_id == 0 {
            info!("No triples for peers: {:?}", peers.all_addresses());
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
        &mut self,
        req: TripleRequest,
        tx: Sender<error::Result<Option<BeaverTriplePair>>>,
    ) {
        info!("Getting triple key from remote host.");
        let request_key = TripleRequestKey::from(req.clone());
        let request_key_hash = request_key.hash();
        let peers = match self.tss_state.peer_state.peer_socket_addresses().await {
            Ok(p) => p.active_addresses(),
            Err(e) => {
                error!("Error getting peers: {}", e);
                return;
            }
        };
        let addr = match leader_addr(request_key_hash, &peers) {
            Ok(addr) => addr,
            Err(e) => {
                let _ = tx.send_async(Err(e)).await;
                return;
            }
        };
        let lit_config = self.tss_state.lit_config.clone();
        let http_client = self.tss_state.http_client.clone();
        let req = req.clone();
        let url_path = "select_triple".to_string();
        let url = format!(
            "{}{}/{}",
            self.tss_state
                .lit_config
                .http_prefix_when_talking_to_other_nodes(),
            addr,
            url_path
        );

        let local_tx = self.tx.clone();
        tokio::spawn(async move {
            let request_builder = http_client.post(url.clone()).json(&req);
            // Disabled for now - but think we need this.
            // let enable_http_header_descriptors = lit_config.enable_http_header_descriptors();
            // match enable_http_header_descriptors {
            //     Err(e) => {
            //         error!(
            //             "Error getting enable_http_header_descriptors from lit_config: {}",
            //             e
            //         );
            //     }
            //     Ok(enable) => {
            //         if enable {
            //             let header_value = join_multiple_body_descriptor_parameters(
            //                 &get_body_descriptor_for_node_transmission_batch_entries(
            //                     &batch_entries,
            //                 ),
            //             );
            //             request_builder = request_builder
            //                 .header(headers::NODE_REQUEST_BODY_DESCRIPTOR_KEY, header_value);
            //         }
            //     }
            // }

            info!(
                "Sending request for {} to: {}",
                request_key_hash,
                url.clone()
            );
            let triple_leader_response: TripleLeaderResponse = match request_builder.send().await {
                Ok(res) => {
                    let res_body = res.text().await;
                    match res_body {
                        Ok(body) => {
                            let triple_leader_response =
                                serde_json::from_str::<TripleLeaderResponse>(&body);
                            match triple_leader_response {
                                Ok(triple_leader_response) => {
                                    info!(
                                        "Got key for hash {} from remote host: {:?}",
                                        request_key_hash, triple_leader_response
                                    );
                                    triple_leader_response
                                }
                                Err(e) => {
                                    error!("Error parsing key from body: {} >> {}", e, body);
                                    TripleLeaderResponse {
                                        triple_storage_key: 0,
                                        remaining_triple_pairs: 0,
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            error!("Error getting body from response: {}", e);
                            TripleLeaderResponse {
                                triple_storage_key: 0,
                                remaining_triple_pairs: 0,
                            }
                        }
                    }
                }
                Err(error) => {
                    let msg = format!(
                        "Problem getting TripleKey from ({}).  Error: {:?}",
                        url.clone(),
                        error
                    );
                    error!("{}", msg);
                    TripleLeaderResponse {
                        triple_storage_key: 0,
                        remaining_triple_pairs: 0,
                    }
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
    // use super::finder::staker_hashes_from_peer_group_id;
    tokio::spawn(async move {
        let mut triple_counter: Vec<RealTimeTripleCounter> = Vec::new();
        for bucket in triple_list {
            let peer_group_id = bucket.0;
            let count = bucket.1.len() as u64;
            let rtpc = RealTimeTripleCounter {
                peer_group_id,
                count,
                peer_keys: Vec::new(),
                // peer_keys: staker_hashes_from_peer_group_id(peer_group_id, &xor_filters, &peers).await;
            };
            triple_counter.push(rtpc);
        }

        let item = MetricsMessage::TripleStatus(triple_counter);
        let _r = tx.send_async(item).await;
    });
}

#[instrument(skip_all)]
pub fn addr_is_leader(request_key_hash: u64, peers: &Vec<String>, addr: &String) -> bool {
    let leader_address = match leader_addr(request_key_hash, peers) {
        Ok(addr) => addr,
        Err(e) => {
            error!("Error getting leader address: {}", e);
            return false;
        }
    };

    addr == leader_address
}

#[instrument(skip_all)]
pub fn leader_addr(request_key_hash: u64, peers: &Vec<String>) -> Result<&String> {
    let leader_hash = generate_hash(request_key_hash);
    let group_size = match peers.len() {
        0 => {
            return Err(unexpected_err("No peers found in leader_addr.", None));
        }
        size => size,
    };
    let leader_id = leader_hash % group_size as u64;
    Ok(&peers[leader_id as usize])
}

#[instrument(skip_all, fields(txn_prefix = req.txn_prefix))]
pub fn request_hash(req: TripleRequest) -> u64 {
    let request_key = TripleRequestKey::from(req.clone());
    request_key.hash()
}

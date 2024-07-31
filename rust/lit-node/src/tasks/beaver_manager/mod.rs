pub mod finder;
pub mod listener;
pub mod models;

use crate::tasks::beaver_manager::models::XorFilterWithThreshold;
use crate::tss::common::dkg_type::DkgType;
use crate::tss::common::tss_state::TssState;
use crate::{error::Result, tss::ecdsa_cait_sith::CsEcdsaState};
use reqwest::Client;
use std::collections::HashMap;
use std::sync::Arc;

use self::models::{BeaverManager, TripleRequest, TripleRequestKey};

impl TripleRequestKey {
    pub fn from(request: TripleRequest) -> Self {
        TripleRequestKey {
            message_bytes: request.message_bytes,
            public_key: request.public_key,
            request_id: request.request_id,
        }
    }
}

impl BeaverManager {
    pub fn new(
        rx: flume::Receiver<models::BeaverMessage>,
        tx: flume::Sender<models::BeaverMessage>,
        tss_state: Arc<TssState>,
        http_client: Client,
    ) -> Result<Self> {
        info!("Creating new beaver triple manager");

        let signing_state = CsEcdsaState {
            state: (*tss_state).clone(),
            dkg_type: DkgType::Standard,
        };

        let min_triples = 0;
        let max_triples = 10;
        let max_triple_concurrency = 2;
        let is_generating = false;
        let xor_filters: HashMap<models::PeerGroupId, XorFilterWithThreshold> = HashMap::new();

        Ok(BeaverManager {
            min_triples,
            max_triples,
            max_triple_concurrency,
            rx,
            tx,
            tss_state,
            http_client,
            signing_state,
            current_generation_count: 0,
            generating_txn_ids: Vec::new(),
            last_generated: std::time::Instant::now(),
            xor_filters,
        })
    }
}

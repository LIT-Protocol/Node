use cait_sith::{protocol::Participant, CSCurve};
use serde::{Deserialize, Serialize};

use crate::tss::common::web::models::NodeConfig;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EcdsaKeyShare<C: CSCurve> {
    pub private_share: C::Scalar,
    pub public_key: C::AffinePoint,
    pub index: u16,
    pub threshold: u16,
    pub total_shares: u16,
    pub txn_prefix: String,
}

pub struct ReshareConfigData {
    pub new_threshold: usize,
    pub old_threshold: usize,
    pub node_config: NodeConfig,
}

#[derive(Debug, Clone)]
pub struct ProtocolTransactionParams {
    pub txn_prefix: String,
    pub self_participant: Participant,
    pub participants: Vec<Participant>,
    pub all_participants: Vec<Participant>,
    pub node_config: NodeConfig,
    pub threshold: usize,
}

impl From<ProtocolTransactionParams>
    for (
        String,
        Participant,
        Vec<Participant>,
        Vec<Participant>,
        NodeConfig,
        usize,
    )
{
    fn from(
        e: ProtocolTransactionParams,
    ) -> (
        String,
        Participant,
        Vec<Participant>,
        Vec<Participant>,
        NodeConfig,
        usize,
    ) {
        let ProtocolTransactionParams {
            txn_prefix,
            self_participant,
            participants,
            all_participants,
            node_config,
            threshold,
        } = e;
        (
            txn_prefix,
            self_participant,
            participants,
            all_participants,
            node_config,
            threshold,
        )
    }
}

impl ProtocolTransactionParams {
    pub fn clone_with_extension(
        &mut self,
        txn_prefix_extension: String,
    ) -> ProtocolTransactionParams {
        let txn_prefix = format!("{}{}", self.txn_prefix, txn_prefix_extension);
        ProtocolTransactionParams {
            txn_prefix,
            self_participant: self.self_participant,
            participants: self.participants.clone(),
            all_participants: self.all_participants.clone(),
            node_config: self.node_config.clone(),
            threshold: self.threshold,
        }
    }

    pub fn from_config(
        config: (
            String,
            Participant,
            Vec<Participant>,
            Vec<Participant>,
            NodeConfig,
            usize,
        ),
    ) -> ProtocolTransactionParams {
        let (txn_prefix, self_participant, participants, all_participants, node_config, threshold) =
            config;
        ProtocolTransactionParams {
            txn_prefix,
            self_participant,
            participants,
            all_participants,
            node_config,
            threshold,
        }
    }
}

// priate_share and public_key are the keygen_ouput values from Cait-Sith.

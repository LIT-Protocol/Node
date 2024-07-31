use crate::{
    peers::peer_state::models::{SimplePeer, SimplePeerExt},
    tasks::beaver_manager::models::BeaverTriplePair,
};
use cait_sith::protocol::Participant;

pub struct ReshareConfigData {
    pub new_threshold: u16,
    pub old_threshold: u16,
}

#[derive(Debug, Clone)]
pub struct ProtocolTransactionParams {
    pub txn_prefix: String,
    pub self_participant: Participant,
    pub participants: Vec<Participant>,
    pub peers: Vec<SimplePeer>,
    pub bt_id: Option<Participant>,
    pub bt_participants: Option<Vec<Participant>>,
    pub threshold: u16,
}

impl From<ProtocolTransactionParams>
    for (String, Participant, Vec<Participant>, Vec<SimplePeer>, u16)
{
    fn from(
        e: ProtocolTransactionParams,
    ) -> (String, Participant, Vec<Participant>, Vec<SimplePeer>, u16) {
        let ProtocolTransactionParams {
            txn_prefix,
            self_participant,
            participants,
            peers,
            bt_id,
            bt_participants,
            threshold,
        } = e;
        (txn_prefix, self_participant, participants, peers, threshold)
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
            peers: self.peers.clone(),
            bt_id: self.bt_id,
            bt_participants: self.bt_participants.clone(),
            threshold: self.threshold,
        }
    }

    pub fn from_config(
        config: (String, Participant, Vec<Participant>, Vec<SimplePeer>, u16),
    ) -> ProtocolTransactionParams {
        let (txn_prefix, self_participant, participants, peers, threshold) = config;
        ProtocolTransactionParams {
            txn_prefix,
            self_participant,
            participants,
            peers,
            bt_id: None,
            bt_participants: None,
            threshold,
        }
    }

    pub fn update_triple_info(&mut self, triple_pair: &BeaverTriplePair) {
        self.bt_id = Some(Participant::from(triple_pair.share_index as u32));

        let mut subset: Vec<SimplePeer> = Vec::new();
        for participant in &self.participants {
            let protocol_index: u32 = (*participant).into();
            match self.peers.peer_at_share_index(protocol_index as u16) {
                Ok(peer) => subset.push(peer),
                _ => continue,
            };
        }

        let bt_participants = triple_pair
            .indices_from_peers(subset)
            .iter()
            .map(|p| Participant::from(*p))
            .collect::<Vec<_>>();
        self.bt_participants = Some(bt_participants);
    }
}

// priate_share and public_key are the keygen_ouput values from Cait-Sith.

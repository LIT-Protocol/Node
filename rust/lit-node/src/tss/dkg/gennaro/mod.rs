pub mod models;
mod protected;
use super::traits::{Dkg, RoundData};
use crate::{
    error::{
        conversion_err, lock_err, unexpected_err, unexpected_err_code, validation_err, Result, EC,
    },
    peers::peer_state::models::SimplePeer,
    utils::consensus::get_threshold_count,
};
use elliptic_curve::group::{Group, GroupEncoding};
pub use gennaro_dkg::Round;
use gennaro_dkg::*;
use lit_core::error::Unexpected;
pub use models::*;
use serde::{Deserialize, Serialize};
use soteria_rs::Protected;
use std::sync::{Arc, Mutex};
use std::{collections::BTreeMap, num::NonZeroUsize};

pub enum RoundInputs<G: Group + GroupEncoding + Default> {
    Round1,
    Round2(Box<Round2InputData<G>>),
    Round3(Box<Round3InputData>),
    Round4(Box<Round4InputData<G>>),
    Round5(Box<Round5InputData<G>>),
}

impl<G: Group + GroupEncoding + Default> From<Round2InputData<G>> for RoundInputs<G> {
    fn from(input: Round2InputData<G>) -> Self {
        RoundInputs::Round2(Box::new(input))
    }
}

impl<G: Group + GroupEncoding + Default> From<Round3InputData> for RoundInputs<G> {
    fn from(input: Round3InputData) -> Self {
        RoundInputs::Round3(Box::new(input))
    }
}

impl<G: Group + GroupEncoding + Default> From<Round4InputData<G>> for RoundInputs<G> {
    fn from(input: Round4InputData<G>) -> Self {
        RoundInputs::Round4(Box::new(input))
    }
}

impl<G: Group + GroupEncoding + Default> From<Round5InputData<G>> for RoundInputs<G> {
    fn from(input: Round5InputData<G>) -> Self {
        RoundInputs::Round5(Box::new(input))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RoundOutputs<G: Group + GroupEncoding + Default> {
    Round1(Box<Round1OutputData<G>>),
    Round2(Box<Round2OutputData>),
    Round3(Box<Round3OutputData<G>>),
    Round4(Box<Round4OutputData<G>>),
    Round5(Box<Round5OutputData>),
}

impl<G: Group + GroupEncoding + Default> From<Round1OutputData<G>> for RoundOutputs<G> {
    fn from(output: Round1OutputData<G>) -> Self {
        RoundOutputs::Round1(Box::new(output))
    }
}

impl<G: Group + GroupEncoding + Default> From<Round2OutputData> for RoundOutputs<G> {
    fn from(output: Round2OutputData) -> Self {
        RoundOutputs::Round2(Box::new(output))
    }
}

impl<G: Group + GroupEncoding + Default> From<Round3OutputData<G>> for RoundOutputs<G> {
    fn from(output: Round3OutputData<G>) -> Self {
        RoundOutputs::Round3(Box::new(output))
    }
}

impl<G: Group + GroupEncoding + Default> From<Round4OutputData<G>> for RoundOutputs<G> {
    fn from(output: Round4OutputData<G>) -> Self {
        RoundOutputs::Round4(Box::new(output))
    }
}

impl<G: Group + GroupEncoding + Default> From<Round5OutputData> for RoundOutputs<G> {
    fn from(output: Round5OutputData) -> Self {
        RoundOutputs::Round5(Box::new(output))
    }
}

impl<G: Group + GroupEncoding + Default> From<RoundOutputs<G>> for Round {
    fn from(value: RoundOutputs<G>) -> Self {
        match value {
            RoundOutputs::Round1(_) => Round::One,
            RoundOutputs::Round2(_) => Round::Two,
            RoundOutputs::Round3(_) => Round::Three,
            RoundOutputs::Round4(_) => Round::Four,
            RoundOutputs::Round5(_) => Round::Five,
        }
    }
}

#[cfg(test)]
impl<G: Group + GroupEncoding + Default> RoundOutputs<G> {
    /// Returns the round 1 broadcast data if is Round1
    pub fn round1_broadcast_data(&self) -> Option<Round1BroadcastData<G>> {
        match self {
            RoundOutputs::Round1(data) => Some(data.broadcast_data.clone()),
            _ => None,
        }
    }

    /// Returns the round 1 peer data if is Round1
    pub fn round1_peer_data(&self) -> Option<BTreeMap<usize, Round1P2PData>> {
        match self {
            RoundOutputs::Round1(data) => Some(data.peer_data.clone()),
            _ => None,
        }
    }

    /// Returns the round 2 broadcast data if is Round2
    pub fn round2_broadcast_data(&self) -> Option<Round2EchoBroadcastData> {
        match self {
            RoundOutputs::Round2(data) => Some(data.0.clone()),
            _ => None,
        }
    }

    /// Returns the round 3 broadcast data if is Round3
    pub fn round3_broadcast_data(&self) -> Option<Round3BroadcastData<G>> {
        match self {
            RoundOutputs::Round3(data) => Some(data.0.clone()),
            _ => None,
        }
    }

    /// Returns the round 4 broadcast data if is Round4
    pub fn round4_broadcast_data(&self) -> Option<Round4EchoBroadcastData<G>> {
        match self {
            RoundOutputs::Round4(data) => Some(data.0),
            _ => None,
        }
    }
}

impl<G: Group + GroupEncoding + Default, I: ParticipantImpl<G> + Default> Dkg
    for Participant<I, G>
{
    type RoundInputs = RoundInputs<G>;
    type RoundOutputs = RoundOutputs<G>;

    fn next_round(&mut self, input: &Self::RoundInputs) -> Result<Option<Self::RoundOutputs>> {
        match (self.get_round(), input) {
            (Round::One, RoundInputs::Round1) => {
                let (round1_broadcast_data, round1_p2p_data) = self.round1().map_err(|e| {
                    unexpected_err_code(
                        e,
                        EC::NodeDkgRoundFailed,
                        Some("Gennaro Round 1 Failed".into()),
                    )
                })?;
                Ok(Some(RoundOutputs::from(Round1OutputData {
                    broadcast_data: round1_broadcast_data,
                    peer_data: round1_p2p_data,
                })))
            }
            (Round::Two, RoundInputs::Round2(data)) => {
                let round1_broadcast_data = &data.broadcast_data;
                let protected_round1_p2p_data = &data.peer_data;
                let mut round1_p2p_data = BTreeMap::new();
                for (key, value) in protected_round1_p2p_data {
                    let mut binding = value
                        .lock()
                        .map_err(|e| lock_err(e.to_string(), Some("Locked P2P value".into())))?;
                    let unprotected_share = binding
                        .unprotect()
                        .ok_or(unexpected_err("Invalid secret unprotected", None))?;
                    let val = unprotected_share.serde::<Round1P2PData>().map_err(|e| {
                        conversion_err(
                            format!("Unable to deserialize Round1P2PData: {:?}", e),
                            None,
                        )
                    })?;
                    round1_p2p_data.insert(*key, val);
                }
                let round2_echo_broadcast_data = self
                    .round2(round1_broadcast_data.clone(), round1_p2p_data)
                    .map_err(|e| {
                        unexpected_err_code(
                            e,
                            EC::NodeDkgRoundFailed,
                            Some("Gennaro Round 2 Failed".into()),
                        )
                    })?;
                Ok(Some(RoundOutputs::from(Round2OutputData(
                    round2_echo_broadcast_data,
                ))))
            }
            (Round::Three, RoundInputs::Round3(round2_echo_broadcast_data)) => {
                let round3_broadcast_data =
                    self.round3(&round2_echo_broadcast_data.0).map_err(|e| {
                        unexpected_err_code(
                            e,
                            EC::NodeDkgRoundFailed,
                            Some("Gennaro Round 3 Failed".into()),
                        )
                    })?;
                Ok(Some(RoundOutputs::from(Round3OutputData(
                    round3_broadcast_data,
                ))))
            }
            (Round::Four, RoundInputs::Round4(round3_broadcast_data)) => {
                let round4_echo_broadcast_data =
                    self.round4(&round3_broadcast_data.0).map_err(|e| {
                        unexpected_err_code(
                            e,
                            EC::NodeDkgRoundFailed,
                            Some("Gennaro Round 4 Failed".into()),
                        )
                    })?;
                Ok(Some(RoundOutputs::from(Round4OutputData(
                    round4_echo_broadcast_data,
                ))))
            }
            (Round::Five, RoundInputs::Round5(round4_echo_broadcast_data)) => {
                self.round5(&round4_echo_broadcast_data.0).map_err(|e| {
                    unexpected_err_code(
                        e,
                        EC::NodeDkgRoundFailed,
                        Some("Gennaro Round 5 Failed".into()),
                    )
                })?;
                Ok(None)
            }
            (Round::Five, _) => {
                if self.completed() {
                    return Ok(None);
                }
                Err(unexpected_err("Invalid round input", None))
            }
            (_, _) => Err(unexpected_err("Invalid round input", None)),
        }
    }

    fn rounds_remaining(&self) -> Option<usize> {
        let max: usize = Round::Five.into();
        let cur: usize = self.get_round().into();
        Some(max - cur)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RoundResult<G: Group + GroupEncoding + Default> {
    #[serde(bound(serialize = "Round1BroadcastData<G>: Serialize"))]
    #[serde(bound(deserialize = "Round1BroadcastData<G>: Deserialize<'de>"))]
    Round1BroadcastAndP2PData(Box<(Round1BroadcastData<G>, Round1P2PData)>),
    Round2EchoBroadcastData(Box<Round2EchoBroadcastData>),
    #[serde(bound(serialize = "Round3BroadcastData<G>: Serialize"))]
    #[serde(bound(deserialize = "Round3BroadcastData<G>: Deserialize<'de>"))]
    Round3BroadcastData(Box<Round3BroadcastData<G>>),
    #[serde(bound(serialize = "Round4EchoBroadcastData<G>: Serialize"))]
    #[serde(bound(deserialize = "Round4EchoBroadcastData<G>: Deserialize<'de>"))]
    Round4EchoBroadcastData(Box<Round4EchoBroadcastData<G>>),
}

impl<G: Group + GroupEncoding + Default> RoundResult<G> {
    pub fn from_round_outputs(output: &RoundOutputs<G>, peer_index: usize) -> Option<Self> {
        match output {
            RoundOutputs::Round1(data) => {
                let peer_data = match data.peer_data.get(&peer_index) {
                    Some(peer_data) => peer_data,
                    None => {
                        warn!("Index out of range: {} for Round 1. This generally means that the DKG parameters (limit) are wrong", peer_index);
                        return None;
                    }
                };
                Some(Self::from((data.broadcast_data.clone(), peer_data.clone())))
            }
            RoundOutputs::Round2(data) => Some(Self::from(data.0.clone())),
            RoundOutputs::Round3(data) => Some(Self::from(data.0.clone())),
            RoundOutputs::Round4(data) => Some(Self::from(data.0)),
            _ => None,
        }
    }
}

impl<G: Group + GroupEncoding + Default> From<(Round1BroadcastData<G>, Round1P2PData)>
    for RoundResult<G>
{
    fn from(data: (Round1BroadcastData<G>, Round1P2PData)) -> Self {
        RoundResult::Round1BroadcastAndP2PData(Box::new(data))
    }
}

impl<G: Group + GroupEncoding + Default> From<Round2EchoBroadcastData> for RoundResult<G> {
    fn from(data: Round2EchoBroadcastData) -> Self {
        RoundResult::Round2EchoBroadcastData(Box::new(data))
    }
}

impl<G: Group + GroupEncoding + Default> From<Round3BroadcastData<G>> for RoundResult<G> {
    fn from(data: Round3BroadcastData<G>) -> Self {
        RoundResult::Round3BroadcastData(Box::new(data))
    }
}

impl<G: Group + GroupEncoding + Default> From<Round4EchoBroadcastData<G>> for RoundResult<G> {
    fn from(data: Round4EchoBroadcastData<G>) -> Self {
        RoundResult::Round4EchoBroadcastData(Box::new(data))
    }
}

#[derive(Clone, Debug)]
enum ParticipantType<G: Group + GroupEncoding + Default> {
    SecretParticipantType(SecretParticipant<G>),
    RefreshParticipantType(RefreshParticipant<G>),
}

trait ParticipantHelper {
    type Point: Group + GroupEncoding + Default;

    fn get_public_key(&self) -> Option<Self::Point>;
    fn get_secret_share(&self) -> Option<<Self::Point as Group>::Scalar>;
    fn get_round(&self) -> Option<Round>;
    fn execute_next_round_with_input(
        &mut self,
        input: Option<RoundInputs<Self::Point>>,
    ) -> Result<Option<RoundOutputs<Self::Point>>>;
}

impl<G: Group + GroupEncoding + Default> ParticipantHelper for ParticipantType<G> {
    type Point = G;

    fn get_public_key(&self) -> Option<G> {
        match self {
            ParticipantType::SecretParticipantType(secret_participant) => {
                secret_participant.get_public_key()
            }
            ParticipantType::RefreshParticipantType(refresh_participant) => {
                refresh_participant.get_public_key()
            }
        }
    }

    fn get_secret_share(&self) -> Option<G::Scalar> {
        match self {
            ParticipantType::SecretParticipantType(secret_participant) => {
                secret_participant.get_secret_share()
            }
            ParticipantType::RefreshParticipantType(refresh_participant) => {
                refresh_participant.get_secret_share()
            }
        }
    }

    fn get_round(&self) -> Option<Round> {
        match self {
            ParticipantType::SecretParticipantType(secret_participant) => {
                Some(secret_participant.get_round())
            }
            ParticipantType::RefreshParticipantType(refresh_participant) => {
                Some(refresh_participant.get_round())
            }
        }
    }

    fn execute_next_round_with_input(
        &mut self,
        input: Option<RoundInputs<G>>,
    ) -> Result<Option<RoundOutputs<G>>> {
        let input = input.ok_or(validation_err(
            format!("Empty input for round: {:?}", self.get_round()),
            None,
        ))?;

        match self {
            ParticipantType::SecretParticipantType(secret_participant) => {
                if secret_participant
                    .rounds_remaining()
                    .expect_or_err("Unable to get rounds remaining")?
                    > 0
                {
                    return secret_participant.next_round(&input);
                }
            }
            ParticipantType::RefreshParticipantType(refresh_participant) => {
                if refresh_participant
                    .rounds_remaining()
                    .expect_or_err("Unable to get rounds remaining")?
                    > 0
                {
                    return refresh_participant.next_round(&input);
                }
            }
        }

        Ok(None)
    }
}

#[derive(Debug)]
pub struct GennaroDkg<G: Group + GroupEncoding + Default> {
    round1_broadcast_data: BTreeMap<usize, Round1BroadcastData<G>>,
    round1_p2p_data: BTreeMap<usize, Arc<Mutex<Protected>>>,
    round2_echo_broadcast_data: BTreeMap<usize, Round2EchoBroadcastData>,
    round3_broadcast_data: BTreeMap<usize, Round3BroadcastData<G>>,
    round4_echo_broadcast_data: BTreeMap<usize, Round4EchoBroadcastData<G>>,
    participant: Option<ParticipantType<G>>,
}

impl<G: Group + GroupEncoding + Default> Default for GennaroDkg<G> {
    fn default() -> Self {
        Self {
            round1_broadcast_data: BTreeMap::new(),
            round1_p2p_data: BTreeMap::new(),
            round2_echo_broadcast_data: BTreeMap::new(),
            round3_broadcast_data: BTreeMap::new(),
            round4_echo_broadcast_data: BTreeMap::new(),
            participant: None, // As we can't initialize if the node isn't a peer yet
        }
    }
}

impl<G: Group + GroupEncoding + Default> GennaroDkg<G> {
    pub fn clear_dkg_data(&mut self) {
        self.round1_broadcast_data.clear();
        self.round1_p2p_data.clear();
        self.round2_echo_broadcast_data.clear();
        self.round3_broadcast_data.clear();
        self.round4_echo_broadcast_data.clear();
    }

    pub fn get_public_key(&self) -> Option<G> {
        if let Some(participant) = &self.participant {
            return participant.get_public_key();
        }
        None
    }

    pub fn get_secret_share(&self) -> Option<G::Scalar> {
        if let Some(participant) = &self.participant {
            return participant.get_secret_share();
        }
        None
    }

    pub fn get_round(&self) -> Option<Round> {
        if let Some(participant) = self.participant.as_ref() {
            return participant.get_round();
        }
        None
    }

    pub fn get_peer_data_length_and_round(&self) -> Option<(Round, usize)> {
        match self.get_round() {
            Some(Round::Two) => Some((Round::One, self.round1_broadcast_data.len())),
            Some(Round::Three) => Some((Round::Two, self.round2_echo_broadcast_data.len())),
            Some(Round::Four) => Some((Round::Three, self.round3_broadcast_data.len())),
            Some(Round::Five) => Some((Round::Four, self.round4_echo_broadcast_data.len())),
            _ => None,
        }
    }

    pub fn is_participant_initialized(&self) -> bool {
        self.participant.is_none()
    }

    // TODO?: Insert iff self.get_round() == round_result::Round
    pub fn add_peer_data(&mut self, peer_id: usize, round_result: RoundResult<G>) -> Result<()> {
        match round_result {
            RoundResult::Round1BroadcastAndP2PData(data) => {
                let (round1_broadcast_data, round1_p2p_data) = (data.0, data.1);
                self.round1_broadcast_data
                    .entry(peer_id)
                    .or_insert(round1_broadcast_data);
                self.round1_p2p_data
                    .entry(peer_id)
                    .or_insert(Arc::new(Mutex::new(
                        Protected::serde(&round1_p2p_data)
                            .map_err(|e| unexpected_err(format!("Serde Error: {:?}", e), None))?,
                    )));
            }
            RoundResult::Round2EchoBroadcastData(round2_echo_broadcast_data) => {
                self.round2_echo_broadcast_data
                    .entry(peer_id)
                    .or_insert(*round2_echo_broadcast_data);
            }
            RoundResult::Round3BroadcastData(round3_broadcast_data) => {
                self.round3_broadcast_data
                    .entry(peer_id)
                    .or_insert(*round3_broadcast_data);
            }
            RoundResult::Round4EchoBroadcastData(round4_echo_broadcast_data) => {
                self.round4_echo_broadcast_data
                    .entry(peer_id)
                    .or_insert(*round4_echo_broadcast_data);
            }
        }

        Ok(())
    }

    fn next_round_input(&self) -> Option<RoundInputs<G>> {
        if let Some(round) = self.get_round() {
            let input = match round {
                Round::One => RoundInputs::Round1,
                Round::Two => RoundInputs::from(Round2InputData {
                    broadcast_data: self.round1_broadcast_data.clone(),
                    peer_data: self.round1_p2p_data.clone(),
                }),
                Round::Three => {
                    RoundInputs::from(Round3InputData(self.round2_echo_broadcast_data.clone()))
                }
                Round::Four => {
                    RoundInputs::from(Round4InputData(self.round3_broadcast_data.clone()))
                }
                Round::Five => {
                    RoundInputs::from(Round5InputData(self.round4_echo_broadcast_data.clone()))
                }
            };
            return Some(input);
        }
        None
    }

    fn get_share_ids_and_my_index(
        current_peers: &[SimplePeer],
        next_peers: &[SimplePeer],
        my_address: &String,
    ) -> (usize, Vec<G::Scalar>) {
        let next_stakers = next_peers.iter().map(|p| p.key_hash).collect::<Vec<_>>();

        let survivors = current_peers
            .iter()
            .filter(|p| next_stakers.contains(&p.key_hash))
            .collect::<Vec<_>>();

        let share_ids = survivors
            .iter()
            .map(|p| {
                G::Scalar::from(match p.get_protocol_index() {
                    Ok(i) => i as u64,
                    Err(_) => {
                        error!(
                            "Unable to get protocol index for peer: {}",
                            p.socket_address
                        );
                        0
                    } // technically this is impossible, as the protocol exits early if the node is not a peer.
                })
            })
            .collect::<Vec<_>>();

        let my_share_index = match survivors
            .iter()
            .position(|p| p.socket_address == *my_address)
        {
            Some(i) => i,
            None => {
                error!(
                    "Unable to find my address in the list of survivors: {}",
                    my_address
                );
                0 // technically this is impossible, as the protocol exits early if the node is not a peer.
            }
        };
        (my_share_index, share_ids)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn init_participant(
        &mut self,
        mode: &Mode,
        peer_id: usize,
        next_peers: &Vec<SimplePeer>,
        current_peers: &Vec<SimplePeer>,
        my_address: &String,
        share: Option<<G as Group>::Scalar>,
    ) -> Result<()> {
        let limit = next_peers.len();
        let threshold = get_threshold_count(limit);
        let threshold_param = NonZeroUsize::new(threshold)
            .expect_or_err("Threshold is zero, next_peers might be empty")?;
        let limit_param =
            NonZeroUsize::new(limit).expect_or_err("Limit is zero, next_peers might be empty")?;
        let parameters = Parameters::<G>::new(threshold_param, limit_param);

        match mode {
            Mode::Initial => {
                self.participant = Some(ParticipantType::SecretParticipantType(
                    SecretParticipant::<G>::new(
                        NonZeroUsize::new(peer_id).expect_or_err("Unable to get NonZeroUsize")?,
                        parameters,
                    )
                    .map_err(|e| unexpected_err(e, None))?,
                ));
            }
            Mode::RefreshPeer => {
                if share.is_none() {
                    return Err(unexpected_err(
                        "Share wasn't indiciated for recovery process.",
                        None,
                    ));
                }

                self.participant = Some(ParticipantType::RefreshParticipantType(
                    RefreshParticipant::<G>::new(
                        NonZeroUsize::new(peer_id).expect_or_err("Unable to get NonZeroUsize")?,
                        parameters,
                    )
                    .map_err(|e| unexpected_err(e, None))?,
                ));
            }
            Mode::ExistingPeer => {
                let (my_share_index, share_ids) =
                    Self::get_share_ids_and_my_index(current_peers, next_peers, my_address);

                if share.is_none() {
                    return Err(unexpected_err(
                        "Share wasn't indiciated for recovery process.",
                        None,
                    ));
                }

                self.participant = Some(ParticipantType::SecretParticipantType(
                    SecretParticipant::<G>::with_secret(
                        NonZeroUsize::new(peer_id).expect_or_err("Unable to get NonZeroUsize")?,
                        parameters,
                        share.expect_or_err("Unable to get share")?,
                        &share_ids,
                        my_share_index,
                    )
                    .map_err(|e| unexpected_err(e, None))?,
                ));
            }
            Mode::NewPeer => {
                self.participant = Some(ParticipantType::RefreshParticipantType(
                    RefreshParticipant::<G>::new(
                        NonZeroUsize::new(peer_id).expect_or_err("Unable to get NonZeroUsize")?,
                        parameters,
                    )
                    .map_err(|e| unexpected_err(e, None))?,
                ));
            }
        }

        Ok(())
    }

    // Actual execution of the rounds of Gennaro happens here
    pub fn execute(&mut self) -> Result<Option<RoundOutputs<G>>> {
        if self.participant.is_none() {
            return Err(unexpected_err("Participant not Initialized", None));
        }

        let input = self.next_round_input();
        if input.is_none() {
            return Err(unexpected_err("Invalid RoundInput", None));
        }

        if let Some(participant) = self.participant.as_mut() {
            return participant.execute_next_round_with_input(input);
        }

        Ok(None)
    }
}

#[test]
fn trait_dkg_works() {
    use gennaro_dkg::*;
    use std::num::NonZeroUsize;

    let parameters = Parameters::<curve25519_dalek::RistrettoPoint>::new(
        NonZeroUsize::new(2).unwrap(),
        NonZeroUsize::new(3).unwrap(),
    );

    let mut secret_participant1 =
        SecretParticipant::new(NonZeroUsize::new(1).unwrap(), parameters).unwrap();
    let mut secret_participant2 =
        SecretParticipant::new(NonZeroUsize::new(2).unwrap(), parameters).unwrap();
    let mut secret_participant3 =
        SecretParticipant::new(NonZeroUsize::new(3).unwrap(), parameters).unwrap();

    assert_eq!(secret_participant1.rounds_remaining(), Some(4));
    assert_eq!(secret_participant2.rounds_remaining(), Some(4));
    assert_eq!(secret_participant3.rounds_remaining(), Some(4));

    let p1_r1_data_res = secret_participant1.next_round(&RoundInputs::Round1);
    let p2_r1_data_res = secret_participant2.next_round(&RoundInputs::Round1);
    let p3_r1_data_res = secret_participant3.next_round(&RoundInputs::Round1);
    assert!(p1_r1_data_res.is_ok());
    assert!(p2_r1_data_res.is_ok());
    assert!(p3_r1_data_res.is_ok());
    assert_eq!(secret_participant1.rounds_remaining(), Some(3));
    assert_eq!(secret_participant2.rounds_remaining(), Some(3));
    assert_eq!(secret_participant3.rounds_remaining(), Some(3));
    let p1_r1_data_res2 = secret_participant1.next_round(&RoundInputs::Round1);
    let p2_r1_data_res2 = secret_participant2.next_round(&RoundInputs::Round1);
    let p3_r1_data_res2 = secret_participant3.next_round(&RoundInputs::Round1);
    assert!(p1_r1_data_res2.is_err());
    assert!(p2_r1_data_res2.is_err());
    assert!(p3_r1_data_res2.is_err());
    let p1_r1_data = p1_r1_data_res.unwrap().unwrap();
    let p2_r1_data = p2_r1_data_res.unwrap().unwrap();
    let p3_r1_data = p3_r1_data_res.unwrap().unwrap();
    let p1_r2_data_res = secret_participant1.next_round(&RoundInputs::from(Round2InputData {
        broadcast_data: maplit::btreemap! {
            2 => p2_r1_data.round1_broadcast_data().unwrap(),
            3 => p3_r1_data.round1_broadcast_data().unwrap(),
        },
        peer_data: maplit::btreemap! {
            2 => Arc::new(Mutex::new(Protected::serde(&p2_r1_data.round1_peer_data().unwrap()[&1]).unwrap())),
            3 => Arc::new(Mutex::new(Protected::serde(&p3_r1_data.round1_peer_data().unwrap()[&1]).unwrap())),
        },
    }));
    let p2_r2_data_res = secret_participant2.next_round(&RoundInputs::from(Round2InputData {
        broadcast_data: maplit::btreemap! {
            1 => p1_r1_data.round1_broadcast_data().unwrap(),
            3 => p3_r1_data.round1_broadcast_data().unwrap(),
        },
        peer_data: maplit::btreemap! {
        1 => Arc::new(Mutex::new(Protected::serde(&p1_r1_data.round1_peer_data().unwrap()[&2]).unwrap())),
        3 => Arc::new(Mutex::new(Protected::serde(&p3_r1_data.round1_peer_data().unwrap()[&2]).unwrap())),
        },
    }));
    let p3_r2_data_res = secret_participant3.next_round(&RoundInputs::from(Round2InputData {
        broadcast_data: maplit::btreemap! {
            1 => p1_r1_data.round1_broadcast_data().unwrap(),
            2 => p2_r1_data.round1_broadcast_data().unwrap(),
        },
        peer_data: maplit::btreemap! {
            1 => Arc::new(Mutex::new(Protected::serde(&p1_r1_data.round1_peer_data().unwrap()[&3]).unwrap())),
            2 => Arc::new(Mutex::new(Protected::serde(&p2_r1_data.round1_peer_data().unwrap()[&3]).unwrap())),
        },
    }));
    assert!(p1_r2_data_res.is_ok());
    assert!(p2_r2_data_res.is_ok());
    assert!(p3_r2_data_res.is_ok());
    assert_eq!(secret_participant1.rounds_remaining(), Some(2));
    assert_eq!(secret_participant2.rounds_remaining(), Some(2));
    assert_eq!(secret_participant3.rounds_remaining(), Some(2));
    let p1_r2_data_opt = p1_r2_data_res.unwrap();
    let p2_r2_data_opt = p2_r2_data_res.unwrap();
    let p3_r2_data_opt = p3_r2_data_res.unwrap();
    assert!(p1_r2_data_opt.is_some());
    assert!(p2_r2_data_opt.is_some());
    assert!(p3_r2_data_opt.is_some());
    let p1_r2_data = p1_r2_data_opt.unwrap();
    let p2_r2_data = p2_r2_data_opt.unwrap();
    let p3_r2_data = p3_r2_data_opt.unwrap();
    let p1_r3_data_res =
        secret_participant1.next_round(&RoundInputs::from(Round3InputData(maplit::btreemap! {
        2 => p2_r2_data.round2_broadcast_data().unwrap(),
        3 => p3_r2_data.round2_broadcast_data().unwrap(),
        })));
    let p2_r3_data_res =
        secret_participant2.next_round(&RoundInputs::from(Round3InputData(maplit::btreemap! {
            1 => p1_r2_data.round2_broadcast_data().unwrap(),
            3 => p3_r2_data.round2_broadcast_data().unwrap(),
        })));
    let p3_r3_data_res =
        secret_participant3.next_round(&RoundInputs::from(Round3InputData(maplit::btreemap! {
            1 => p1_r2_data.round2_broadcast_data().unwrap(),
            2 => p2_r2_data.round2_broadcast_data().unwrap(),
        })));
    assert!(p1_r3_data_res.is_ok());
    assert!(p2_r3_data_res.is_ok());
    assert!(p3_r3_data_res.is_ok());
    assert_eq!(secret_participant1.rounds_remaining(), Some(1));
    assert_eq!(secret_participant2.rounds_remaining(), Some(1));
    assert_eq!(secret_participant3.rounds_remaining(), Some(1));
    let p1_r3_data_opt = p1_r3_data_res.unwrap();
    let p2_r3_data_opt = p2_r3_data_res.unwrap();
    let p3_r3_data_opt = p3_r3_data_res.unwrap();
    assert!(p1_r3_data_opt.is_some());
    assert!(p2_r3_data_opt.is_some());
    assert!(p3_r3_data_opt.is_some());
    let p1_r3_data = p1_r3_data_opt.unwrap();
    let p2_r3_data = p2_r3_data_opt.unwrap();
    let p3_r3_data = p3_r3_data_opt.unwrap();
    let p1_r4_data_res =
        secret_participant1.next_round(&RoundInputs::from(Round4InputData(maplit::btreemap! {
            2 => p2_r3_data.round3_broadcast_data().unwrap(),
            3 => p3_r3_data.round3_broadcast_data().unwrap(),
        })));
    let p2_r4_data_res =
        secret_participant2.next_round(&RoundInputs::from(Round4InputData(maplit::btreemap! {
            1 => p1_r3_data.round3_broadcast_data().unwrap(),
            3 => p3_r3_data.round3_broadcast_data().unwrap(),
        })));
    let p3_r4_data_res =
        secret_participant3.next_round(&RoundInputs::from(Round4InputData(maplit::btreemap! {
            1 => p1_r3_data.round3_broadcast_data().unwrap(),
            2 => p2_r3_data.round3_broadcast_data().unwrap(),
        })));
    assert!(p1_r4_data_res.is_ok());
    assert!(p2_r4_data_res.is_ok());
    assert!(p3_r4_data_res.is_ok());
    assert_eq!(secret_participant1.rounds_remaining(), Some(0));
    assert_eq!(secret_participant2.rounds_remaining(), Some(0));
    assert_eq!(secret_participant3.rounds_remaining(), Some(0));
    let p1_r4_data_opt = p1_r4_data_res.unwrap();
    let p2_r4_data_opt = p2_r4_data_res.unwrap();
    let p3_r4_data_opt = p3_r4_data_res.unwrap();
    assert!(p1_r4_data_opt.is_some());
    assert!(p2_r4_data_opt.is_some());
    assert!(p3_r4_data_opt.is_some());
    let p1_r4_data = p1_r4_data_opt.unwrap();
    let p2_r4_data = p2_r4_data_opt.unwrap();
    let p3_r4_data = p3_r4_data_opt.unwrap();
    let p1_r5_data_res =
        secret_participant1.next_round(&RoundInputs::from(Round5InputData(maplit::btreemap! {
            2 => p2_r4_data.round4_broadcast_data().unwrap(),
            3 => p3_r4_data.round4_broadcast_data().unwrap(),
        })));
    let p2_r5_data_res =
        secret_participant2.next_round(&RoundInputs::from(Round5InputData(maplit::btreemap! {
            1 => p1_r4_data.round4_broadcast_data().unwrap(),
            3 => p3_r4_data.round4_broadcast_data().unwrap(),
        })));
    let p3_r5_data_res =
        secret_participant3.next_round(&RoundInputs::from(Round5InputData(maplit::btreemap! {
            1 => p1_r4_data.round4_broadcast_data().unwrap(),
            2 => p2_r4_data.round4_broadcast_data().unwrap(),
        })));
    assert!(p1_r5_data_res.is_ok());
    assert!(p2_r5_data_res.is_ok());
    assert!(p3_r5_data_res.is_ok());
    let p1_r5_data_opt = p1_r5_data_res.unwrap();
    let p2_r5_data_opt = p2_r5_data_res.unwrap();
    let p3_r5_data_opt = p3_r5_data_res.unwrap();
    assert!(p1_r5_data_opt.is_none());
    assert!(p2_r5_data_opt.is_none());
    assert!(p3_r5_data_opt.is_none());
    let p1_r5_data_res2 =
        secret_participant1.next_round(&RoundInputs::from(Round5InputData(maplit::btreemap! {
            2 => p2_r4_data.round4_broadcast_data().unwrap(),
            3 => p3_r4_data.round4_broadcast_data().unwrap(),
        })));
    let p2_r5_data_res2 =
        secret_participant2.next_round(&RoundInputs::from(Round5InputData(maplit::btreemap! {
            1 => p1_r4_data.round4_broadcast_data().unwrap(),
            3 => p3_r4_data.round4_broadcast_data().unwrap(),
        })));
    let p3_r5_data_res2 =
        secret_participant3.next_round(&RoundInputs::from(Round5InputData(maplit::btreemap! {
            1 => p1_r4_data.round4_broadcast_data().unwrap(),
            2 => p2_r4_data.round4_broadcast_data().unwrap(),
        })));
    assert!(p1_r5_data_res2.is_ok());
    assert!(p2_r5_data_res2.is_ok());
    assert!(p3_r5_data_res2.is_ok());
    let p1_r5_data_opt2 = p1_r5_data_res2.unwrap();
    let p2_r5_data_opt2 = p2_r5_data_res2.unwrap();
    let p3_r5_data_opt2 = p3_r5_data_res2.unwrap();
    assert!(p1_r5_data_opt2.is_none());
    assert!(p2_r5_data_opt2.is_none());
    assert!(p3_r5_data_opt2.is_none());
}

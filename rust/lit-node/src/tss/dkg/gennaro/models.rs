use crate::tss::common::tss_state::TssState;
use crate::tss::common::{curve_type::CurveType, dkg_type::DkgType};

use super::*;
use elliptic_curve::group::{Group, GroupEncoding};
use gennaro_dkg::*;
use serde::{Deserialize, Serialize};
use soteria_rs::Protected;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct GennaroMpcDkg<G: Group + GroupEncoding + Default> {
    pub state: TssState,
    pub _phantom: std::marker::PhantomData<G>,
    pub dkg_type: DkgType,
    pub curve_type: CurveType,
}

impl<G: Group + GroupEncoding + Default> GennaroMpcDkg<G> {
    pub fn new(state: TssState, curve_type: CurveType) -> Self {
        Self {
            state,
            _phantom: std::marker::PhantomData::<G>,
            dkg_type: DkgType::Standard,
            curve_type,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Mode {
    Initial,
    NewPeer,      // For new Peers
    RefreshPeer,  // For existing Peers where the new and old peer sets are the same.
    ExistingPeer, // For Survivors in a reshare
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Round1OutputData<G: Group + GroupEncoding + Default> {
    #[serde(bound(serialize = "Round1BroadcastData<G>: Serialize"))]
    #[serde(bound(deserialize = "Round1BroadcastData<G>: Deserialize<'de>"))]
    pub broadcast_data: Round1BroadcastData<G>,
    pub peer_data: BTreeMap<usize, Round1P2PData>,
}

impl<G: Group + GroupEncoding + Default> RoundData for Round1OutputData<G> {
    type PeerData = BTreeMap<usize, Round1P2PData>;
    type BroadcastData = Round1BroadcastData<G>;

    fn peer_data(&self) -> Option<&Self::PeerData> {
        Some(&self.peer_data)
    }

    fn broadcast_data(&self) -> &Self::BroadcastData {
        &self.broadcast_data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Round2OutputData(pub Round2EchoBroadcastData);

impl RoundData for Round2OutputData {
    type PeerData = ();
    type BroadcastData = Round2EchoBroadcastData;

    fn peer_data(&self) -> Option<&Self::PeerData> {
        None
    }

    fn broadcast_data(&self) -> &Self::BroadcastData {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Round3OutputData<G: Group + GroupEncoding + Default>(
    #[serde(bound(serialize = "Round3BroadcastData<G>: Serialize"))]
    #[serde(bound(deserialize = "Round3BroadcastData<G>: Deserialize<'de>"))]
    pub Round3BroadcastData<G>,
);

impl<G: Group + GroupEncoding + Default> RoundData for Round3OutputData<G> {
    type PeerData = ();
    type BroadcastData = Round3BroadcastData<G>;

    fn peer_data(&self) -> Option<&Self::PeerData> {
        None
    }

    fn broadcast_data(&self) -> &Self::BroadcastData {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Round4OutputData<G: Group + GroupEncoding + Default>(
    #[serde(bound(serialize = "Round3BroadcastData<G>: Serialize"))]
    #[serde(bound(deserialize = "Round3BroadcastData<G>: Deserialize<'de>"))]
    pub Round4EchoBroadcastData<G>,
);

impl<G: Group + GroupEncoding + Default> RoundData for Round4OutputData<G> {
    type PeerData = ();
    type BroadcastData = Round4EchoBroadcastData<G>;

    fn peer_data(&self) -> Option<&Self::PeerData> {
        None
    }

    fn broadcast_data(&self) -> &Self::BroadcastData {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Round5OutputData(bool);

impl RoundData for Round5OutputData {
    type PeerData = ();
    type BroadcastData = bool;

    fn peer_data(&self) -> Option<&Self::PeerData> {
        None
    }

    fn broadcast_data(&self) -> &Self::BroadcastData {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Round2InputData<G: Group + GroupEncoding + Default> {
    #[serde(bound(serialize = "Round1BroadcastData<G>: Serialize"))]
    #[serde(bound(deserialize = "Round1BroadcastData<G>: Deserialize<'de>"))]
    pub broadcast_data: BTreeMap<usize, Round1BroadcastData<G>>,
    #[serde(with = "protected")]
    pub peer_data: BTreeMap<usize, Arc<Mutex<Protected>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Round3InputData(pub BTreeMap<usize, Round2EchoBroadcastData>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Round4InputData<G: Group + GroupEncoding + Default>(
    #[serde(bound(serialize = "Round3BroadcastData<G>: Serialize"))]
    #[serde(bound(deserialize = "Round3BroadcastData<G>: Deserialize<'de>"))]
    pub BTreeMap<usize, Round3BroadcastData<G>>,
);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Round5InputData<G: Group + GroupEncoding + Default>(
    #[serde(bound(serialize = "Round4EchoBroadcastData<G>: Serialize"))]
    #[serde(bound(deserialize = "Round4EchoBroadcastData<G>: Deserialize<'de>"))]
    pub BTreeMap<usize, Round4EchoBroadcastData<G>>,
);

#[test]
fn round_serialization_works() {
    use std::num::NonZeroUsize;
    let parameters = Parameters::<k256::ProjectivePoint>::new(
        NonZeroUsize::new(2).unwrap(),
        NonZeroUsize::new(2).unwrap(),
    );
    let mut participant1 =
        SecretParticipant::new(NonZeroUsize::new(1).unwrap(), parameters).unwrap();
    let mut participant2 =
        SecretParticipant::new(NonZeroUsize::new(2).unwrap(), parameters).unwrap();

    let (broadcast1, peer1) = participant1.round1().unwrap();
    let (broadcast2, peer2) = participant2.round1().unwrap();
    let output1 = Round1OutputData {
        broadcast_data: broadcast1.clone(),
        peer_data: peer1.clone(),
    };
    let res = serde_json::to_string(&output1);
    assert!(res.is_ok());
    let str1 = res.unwrap();
    let res = serde_json::from_str::<Round1OutputData<k256::ProjectivePoint>>(&str1);
    assert!(res.is_ok());

    let input2 = Round2InputData {
        broadcast_data: maplit::btreemap! {
            1 => broadcast1.clone(),
        },
        peer_data: maplit::btreemap! {
            1 => Arc::new(Mutex::new(Protected::serde(&peer1[&2]).unwrap())),
        },
    };
    let res = serde_json::to_string(&input2);
    assert!(res.is_ok());
    let str2 = res.unwrap();
    let res = serde_json::from_str::<Round2InputData<k256::ProjectivePoint>>(&str2);
    assert!(res.is_ok());

    let broadcast2_1 = participant1
        .round2(
            maplit::btreemap! {
                2 => broadcast2,
            },
            maplit::btreemap! {
                2 => peer2[&1].clone(),
            },
        )
        .unwrap();
    let broadcast2_2 = participant2
        .round2(
            maplit::btreemap! {
                1 => broadcast1,
            },
            maplit::btreemap! {
                1 => peer1[&2].clone(),
            },
        )
        .unwrap();
    let output2_1 = Round2OutputData(broadcast2_1.clone());
    let res = serde_json::to_string(&output2_1);
    assert!(res.is_ok());
    let res = serde_json::from_str::<Round2OutputData>(&res.unwrap());
    assert!(res.is_ok());
    let broadcast2 = maplit::btreemap! {
        1 => broadcast2_1.clone(),
        2 => broadcast2_2,
    };
    let broadcast3_1 = participant1.round3(&broadcast2).unwrap();
    let broadcast3_2 = participant2.round3(&broadcast2).unwrap();
    let output3_1 = Round3OutputData(broadcast3_1.clone());
    let res = serde_json::to_string(&output3_1);
    assert!(res.is_ok());
    let res = serde_json::from_str::<Round3OutputData<k256::ProjectivePoint>>(&res.unwrap());
    assert!(res.is_ok());

    let broadcast3 = maplit::btreemap! {
        1 => broadcast3_1,
        2 => broadcast3_2,
    };
    let broadcast4_1 = participant1.round4(&broadcast3).unwrap();
    let broadcast4_2 = participant2.round4(&broadcast3).unwrap();
    let output4_1 = Round4OutputData(broadcast4_1);
    let res = serde_json::to_string(&output4_1);
    assert!(res.is_ok());
    let res = serde_json::from_str::<Round4OutputData<k256::ProjectivePoint>>(&res.unwrap());
    assert!(res.is_ok());
}

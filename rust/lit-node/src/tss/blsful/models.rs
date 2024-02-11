use elliptic_curve::group::{Group, GroupEncoding};
use serde::{Deserialize, Serialize};

use super::super::dkg::gennaro::GennaroState;

#[derive(Debug)]
pub struct BlsState<G>(GennaroState<G>)
where
    G: Group + GroupEncoding + Default;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BlsKeyShare<G: Group + GroupEncoding + Default> {
    pub private_share: G::Scalar,
    pub public_key: G,
    pub index: u16,
    pub threshold: u16,
    pub total_shares: u16,
    pub txn_prefix: String,
}

use crate::tss::common::{dkg_type::DkgType, tss_state::TssState};
use elliptic_curve::group::{Group, GroupEncoding};

use std::marker::PhantomData;

#[derive(Debug)]
pub struct BlsState<G: Group + GroupEncoding + Default> {
    pub state: TssState,
    pub dkg_type: DkgType,
    pub _phantom: PhantomData<G>,
}

impl<G: Group + GroupEncoding + Default> BlsState<G> {
    pub fn new(state: TssState) -> Self {
        Self::new_with_dkg_type(state, DkgType::Standard)
    }

    pub fn new_with_dkg_type(state: TssState, dkg_type: DkgType) -> Self {
        BlsState {
            state,
            dkg_type,
            _phantom: PhantomData,
        }
    }
}

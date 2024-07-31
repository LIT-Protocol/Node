use crate::error::Result;

/// Provides an interface for a distributed key generation algorithm.
pub trait Dkg {
    /// The round input data
    type RoundInputs;
    /// The round output data
    type RoundOutputs;

    /// Receives an input and computes the next round of the distributed key generation.
    /// Returns Ok(Some(value)) for the next value computed by the dkg, or Ok(None) if there are no more remaining rounds.
    fn next_round(&mut self, input: &Self::RoundInputs) -> Result<Option<Self::RoundOutputs>>;
    /// Returns the number of rounds remaining in the distributed key generation if known.
    fn rounds_remaining(&self) -> Option<usize>;
}

/// Data input for or output from a round of the dkg.
pub trait RoundData {
    /// Peer to peer data type
    type PeerData;
    /// Broadcast data type
    type BroadcastData;

    /// Returns the peer to peer data if any
    fn peer_data(&self) -> Option<&Self::PeerData>;

    /// Returns the broadcast data
    fn broadcast_data(&self) -> &Self::BroadcastData;
}

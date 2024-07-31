use elliptic_curve::group::{Group, GroupEncoding};

#[derive(Clone, Debug)]
pub struct KeyHelper<G>
where
    G: Group + GroupEncoding + Default,
{
    pubkey: G, // need to use the Generic in the struct somehow, otherwise it won't allow us to use a generic.
}

impl<G> Default for KeyHelper<G>
where
    G: Group + GroupEncoding + Default,
{
    fn default() -> Self {
        Self {
            pubkey: G::default(),
        }
    }
}

impl<G> KeyHelper<G>
where
    G: Group + GroupEncoding + Default,
{
    pub fn pubkey(&self) -> G {
        self.pubkey
    }
}

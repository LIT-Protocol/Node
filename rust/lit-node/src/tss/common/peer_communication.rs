/// The purpose of this struct is to track which peers still have not sent a message to this node.
#[derive(Debug)]
pub struct PeerCommunicationChecker<T> {
    peers_not_communicated_with_self_yet: Vec<T>,
}

impl<T: PartialEq + Copy> PeerCommunicationChecker<T> {
    pub fn new(peers_expected_to_hear_from: &Vec<T>) -> Self {
        let mut peers_not_communicated_with_self_yet =
            Vec::with_capacity(peers_expected_to_hear_from.len());
        for p in peers_expected_to_hear_from {
            peers_not_communicated_with_self_yet.push(p.to_owned());
        }

        Self {
            peers_not_communicated_with_self_yet,
        }
    }

    pub fn mark_peer_as_communicated_with(&mut self, peer: &T) {
        self.peers_not_communicated_with_self_yet
            .retain(|p| p != peer);
    }

    pub fn peers_not_communicated_with_self_yet(&self) -> &Vec<T> {
        &self.peers_not_communicated_with_self_yet
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum State {
    Offline,
    Online,
    Locked,
    PendingActive,
    Active,
    Suspended,
    Failure,
}

#[derive(Debug)]
pub enum Transition {
    Init,
    Selected,
    Complete,
    Incumbent,
    Leave,
    Rejoin,
    Retry,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeState {
    state: State,
}

impl Default for NodeState {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeState {
    pub fn new() -> Self {
        Self {
            state: State::Offline,
        }
    }

    pub fn current_state(&self) -> State {
        self.state
    }

    pub fn next(&mut self, transition: Transition) {
        self.state = match (self.state, transition) {
            (State::Offline, Transition::Init) => State::Online,
            (State::Online, Transition::Selected) => State::Locked,
            (State::Locked, Transition::Complete) => State::PendingActive,
            (State::PendingActive, Transition::Complete) => State::Active,
            (State::PendingActive, Transition::Retry) => State::Active,
            (State::PendingActive, Transition::Leave) => State::Suspended,
            (State::Locked, Transition::Leave) => State::Suspended,
            (State::Active, Transition::Incumbent) => State::Locked,
            (State::Active, Transition::Leave) => State::Suspended,
            (State::Suspended, Transition::Rejoin) => State::Online,
            (_state, _transition) => State::Failure,
        };
    }
}

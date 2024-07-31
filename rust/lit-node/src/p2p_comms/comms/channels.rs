use crate::{
    error::Result,
    tss::common::models::{RoundCommand, RoundCommsChannel, RoundData, RoundRegistration},
};
use flume::Sender;
use std::sync::Arc;

pub async fn register_comms_channel(
    tx_round_sender: Arc<Sender<RoundData>>,
    txn_prefix: &str,
    round: &str,
) -> Result<RoundCommsChannel> {
    let (tx, rx) = flume::unbounded();
    let channels = RoundCommsChannel { tx, rx };

    let channel_id = format!("{}-{}", txn_prefix, round);
    let channels = channels.clone();

    let round_registration = RoundRegistration {
        id: channel_id,
        channels: Some(channels.clone()),
    };

    let round_data = RoundData {
        command: RoundCommand::AddChannel,
        round_registration: Some(round_registration),
        round_share_set: None,
    };

    let result = tx_round_sender.send_async(round_data).await;
    if result.is_err() {
        error!(
            "Failed to send comms channel registration for round {:?} with error: {:?}",
            round, result
        );
    }

    Ok(channels)
}

pub async fn deregister_comms_channel(
    tx_round_sender: Arc<Sender<RoundData>>,
    txn_prefix: &String,
    round: &str,
) {
    let channel_id = format!("{}-{}", txn_prefix, round);
    let round_registration = RoundRegistration {
        id: channel_id,
        channels: None,
    };

    let round_data = RoundData {
        command: RoundCommand::RemoveChannel,
        round_registration: Some(round_registration),
        round_share_set: None,
    };

    let result = tx_round_sender.send_async(round_data).await;
    if result.is_err() {
        error!(
            "Failed to send comms channel deregistration for round {:?} with error: {:?}",
            round, result
        );
    }
}

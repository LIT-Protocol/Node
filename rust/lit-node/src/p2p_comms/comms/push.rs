use crate::{
    error::{Result, Unexpected},
    peers::peer_state::models::SimplePeer,
    tss::common::models::{NodeTransmissionDetails, NodeTransmissionEntry},
};
use std::time::SystemTime;
use tracing::instrument;

#[instrument(skip_all, fields(txn_prefix))]
#[allow(clippy::too_many_arguments)]
pub async fn node_share_push_direct(
    txn_prefix: &str,
    tx_batch_sender: &flume::Sender<NodeTransmissionDetails>,
    source_peer: &SimplePeer,
    dest_peer: &SimplePeer,
    round: &str,
    data: Vec<u8>,
) -> Result<bool> {
    let tx_batch_sender = tx_batch_sender.clone();
    let dest_peer = dest_peer.clone();
    let source_peer = source_peer.clone();
    let round = round.to_string();
    let txn_prefix = txn_prefix.to_string();
    let src_index = source_peer.share_index;
    let dest_index = dest_peer.share_index;

    tokio::spawn(async move {
        let key = format_node_share_key(&txn_prefix, src_index, dest_index, &round);

        let l = tokio::time::Instant::now();
        let entry = NodeTransmissionEntry {
            key,
            src_index,
            dest_index,
            value: data,
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("Duration since UNIX_EPOCH should be valid")
                .as_micros(),
        };

        let transmission_details = NodeTransmissionDetails {
            dest_peer: dest_peer.clone(),
            node_transmission_entry: entry,
            round: round.to_string(),
        };

        let r = tx_batch_sender
            .send_async(transmission_details.clone())
            .await;
        if let Err(e) = r {
            error!("Error sending message in node_share_push_direct: {:?} \nFrom: {:?}:\nTo: {:?}\nEntry:{:?}", e, source_peer, dest_peer, transmission_details);
        }
    });

    Ok(true)
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ParsedTxnPrefix {
    pub epoch_number: u128,
    pub lifecycle_id: u64,
    pub key_type: String,
}

/// An example `txn_prefix` might be `EPOCH_DKG_1_2.BLS`
pub fn parse_epoch_change_operation_id<T>(operation_id: T) -> Result<ParsedTxnPrefix>
where
    T: AsRef<str>,
{
    let operation_id = operation_id.as_ref();
    let mut operation_id_parts = operation_id.split('.');
    let dkg_id = operation_id_parts
        .next()
        .expect_or_err("Invalid key - Missing operation type and id")?;
    let key_type = operation_id_parts
        .next()
        .expect_or_err("Invalid key - Missing key type")?;
    let dkg_id_parts = dkg_id.split('_');
    let epoch_number = dkg_id_parts
        .clone()
        .nth(2)
        .expect_or_err("Invalid key - Missing epoch number")?
        .parse::<u128>()
        .expect_or_err("Invalid key - Epoch number is not a number")?;
    let lifecycle_id = dkg_id_parts
        .clone()
        .nth(3)
        .expect_or_err("Invalid key - Missing lifecycle id")?
        .parse::<u64>()
        .expect_or_err("Invalid key - Lifecycle id is not a number")?;
    Ok(ParsedTxnPrefix {
        epoch_number,
        lifecycle_id,
        key_type: key_type.to_string(),
    })
}

pub fn is_operation_epoch_change<T>(operation_type_and_id: T) -> bool
where
    T: AsRef<str>,
{
    let operation_type_and_id = operation_type_and_id.as_ref();
    operation_type_and_id.starts_with("EPOCH_DKG")
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ParsedNodeShareKey {
    pub operation_type_and_id: String,
    pub current_index: u16,
    pub dest_index: u16,
    pub round: String,
}

pub fn format_node_share_key(
    operation_type_and_id: &str,
    current_index: u16,
    dest_index: u16,
    round: &str,
) -> String {
    format!(
        "{}--{}-{}-{}",
        operation_type_and_id, current_index, dest_index, round
    )
}

/// An example `key` might be `EPOCH_DKG_1_2.BLS--1-2-1` or `TRP0.known_value_full_lit_9489d2c30aa7b--0-1-CS`
pub fn parse_node_share_key<T>(key: &T) -> Result<ParsedNodeShareKey>
where
    T: AsRef<str>,
{
    let key_parts = key.as_ref().split("--");
    let operation_type_and_id = key_parts
        .clone()
        .next()
        .expect_or_err("Invalid key - Missing operation type and id")?;
    let mut round_parts = key_parts
        .clone()
        .nth(1)
        .expect_or_err("Invalid key - Missing round parts")?
        .split('-');
    let current_index = round_parts
        .next()
        .expect_or_err("Invalid key - Missing current index")?
        .parse::<u16>()
        .expect_or_err("Invalid key - Current index is not a number")?;
    let dest_index = round_parts
        .next()
        .expect_or_err("Invalid key - Missing dest index")?
        .parse::<u16>()
        .expect_or_err("Invalid key - Dest index is not a number")?;
    let round = round_parts
        .next()
        .expect_or_err("Invalid key - Missing round number")?;

    Ok(ParsedNodeShareKey {
        operation_type_and_id: operation_type_and_id.to_string(),
        current_index,
        dest_index,
        round: round.to_string(),
    })
}

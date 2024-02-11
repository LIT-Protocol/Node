use crate::tss::common::models::NodeTransmissionBatchEntries;

pub fn get_body_descriptor_for_node_transmission_batch_entries(
    message: &NodeTransmissionBatchEntries,
) -> Vec<String> {
    message.entries.iter().map(|e| e.key.clone()).collect()
}

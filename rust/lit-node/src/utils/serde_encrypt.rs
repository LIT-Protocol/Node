use lit_core::error::Unexpected;
use serde::{de::DeserializeOwned, Serialize};
use serde_encrypt::{
    traits::SerdeEncryptPublicKey, EncryptedMessage, ReceiverCombinedKey, SenderCombinedKey,
};
use serde_encrypt_core::key::key_pair::public_key::{ReceiverPublicKey, SenderPublicKey};
use tracing::instrument;
use xor_name::XorName;

use crate::{
    error::{unexpected_err_code, Result, EC},
    peers::PeerState,
};

#[instrument(skip_all)]
fn get_receiver_pubkey(peer_state: &PeerState, peer_addr: &str) -> Result<ReceiverPublicKey> {
    let peer_state_data = peer_state.union_data.load_full();

    let peer_item = peer_state_data
        .get_peer_by_addr(peer_addr)
        .expect_or_err_code(
            EC::NodePeerNotFound,
            format!("Could not find peer with addr {}", peer_addr),
        )?;
    let rpk: crypto_box::PublicKey = peer_item.receiver_public_key.into();
    Ok(ReceiverPublicKey::from(rpk))
}

#[instrument(skip_all)]
pub fn encrypt_and_serialize(
    peer_state: &PeerState,
    peer_addr: &str,
    data: &(impl SerdeEncryptPublicKey + Serialize),
) -> Result<Vec<u8>> {
    // First get the receiver public key.
    let receiver_pubkey = get_receiver_pubkey(peer_state, peer_addr)?;

    // Then, construct the combined key.
    let combined_key = SenderCombinedKey::new(
        peer_state.comskeys.my_sender_private_key(),
        &receiver_pubkey,
    );

    // Then, encrypt the data.
    let encrypted_data = data.encrypt(&combined_key).map_err(|e| {
        unexpected_err_code(
            e,
            EC::NodeEncryptionError,
            Some("Unable to encrypt entry".into()),
        )
    })?;

    // Finally, serialize the encrypted data.
    Ok(encrypted_data.serialize())
}

#[instrument(skip_all)]
fn get_sender_pubkey(peer_state: &PeerState, sender_id: XorName) -> Result<SenderPublicKey> {
    let peer_state_data = peer_state.union_data.load_full();

    let peer_item = peer_state_data
        .get_peer_by_peer_id(&sender_id)
        .expect_or_err_code(
            EC::NodePeerNotFound,
            format!(
                "Could not find peer with id {}.  Peers: {:?}",
                sender_id, &peer_state_data
            ),
        )?;
    let spk: crypto_box::PublicKey = peer_item.sender_public_key.into();
    Ok(SenderPublicKey::from(spk))
}

#[instrument(skip_all)]
pub fn deserialize_and_decrypt<T: SerdeEncryptPublicKey + DeserializeOwned>(
    peer_state: &PeerState,
    sender_id: XorName,
    data: &[u8],
) -> Result<T> {
    // First get the sender public key.
    let sender_pubkey = get_sender_pubkey(peer_state, sender_id)?;

    // Then, construct the combined key.
    let combined_key = ReceiverCombinedKey::new(
        &sender_pubkey,
        peer_state.comskeys.my_receiver_private_key(),
    );

    // Then, deserialize the data.
    let encrypted_data = EncryptedMessage::deserialize(data.to_vec()).map_err(|e| {
        unexpected_err_code(
            e,
            EC::NodeEncryptionError,
            Some("Unable to deserialize entry".into()),
        )
    })?;

    // Finally, decrypt the data.
    let decrypted_data = T::decrypt_owned(&encrypted_data, &combined_key).map_err(|e| {
        unexpected_err_code(
            e,
            EC::NodeEncryptionError,
            Some("Unable to decrypt entry".into()),
        )
    })?;

    // Finally, return the decrypted data.
    Ok(decrypted_data)
}

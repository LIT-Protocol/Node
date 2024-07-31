use crate::config::LitNodeConfig;
use crate::error::Result;
use crate::utils::encoding;
use crypto_box::SecretKey;
use lit_core::config::LitConfig;
use lit_core::error::Unexpected;
use rand::RngCore; // for fill bytes
use serde_encrypt::ReceiverKeyPairCore;
use serde_encrypt::{
    key::key_pair::{ReceiverKeyPair, SenderKeyPair},
    SenderKeyPairCore,
};
use serde_encrypt_core::key::key_pair::public_key::{ReceiverPublicKey, SenderPublicKey};
use serde_encrypt_core::key::key_pair::{
    private_key::ReceiverPrivateKey, private_key::SenderPrivateKey,
};

#[derive(Debug, Clone)]
// Keys for internode communication.
pub struct ComsKeys {
    se_s_pair: SenderKeyPair,
    se_r_pair: ReceiverKeyPair,
}

impl Default for ComsKeys {
    fn default() -> Self {
        Self::new()
    }
}

impl ComsKeys {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut pk = [0u8; 32];
        let mut sk = [0u8; 32];
        let mut seed = [0u8; 32];
        rng.fill_bytes(&mut seed);
        sodalite::box_keypair_seed(&mut pk, &mut sk, &seed);

        Self {
            se_s_pair: SenderKeyPair::generate(),
            se_r_pair: ReceiverKeyPair::generate(),
        }
    }

    pub fn new_from_keypairs(
        sender_private_key: SenderPrivateKey,
        sender_public_key: SenderPublicKey,
        receiver_private_key: ReceiverPrivateKey,
        receiver_public_key: ReceiverPublicKey,
    ) -> Self {
        Self {
            se_s_pair: SenderKeyPair::new(sender_private_key, sender_public_key),
            se_r_pair: ReceiverKeyPair::new(receiver_private_key, receiver_public_key),
        }
    }

    pub fn from_node_config(cfg: &LitConfig) -> Result<Self> {
        let sender_privkey = cfg.coms_keys_sender_privkey()?;
        let receiver_privkey = cfg.coms_keys_receiver_privkey()?;

        let sender_privkey = encoding::hex_to_bytes(sender_privkey)
            .expect_or_err("Failed to hex encode LIT_NODE_COMS_KEYS_SENDER_PRIVKEY config var")?;
        let receiver_privkey = encoding::hex_to_bytes(receiver_privkey)
            .expect_or_err("Failed to hex encode LIT_NODE_COMS_KEYS_RECEIVER_PRIVKEY config var")?;

        let sender_privkey_array: [u8; 32] = sender_privkey
            .as_slice()
            .try_into()
            .expect_or_err("LIT_NODE_COMS_KEYS_SENDER_PRIVKEY slice with incorrect length")?;
        let se_s_secret = SecretKey::from(sender_privkey_array);
        let se_s_public = se_s_secret.public_key();

        let receiver_privkey_array: [u8; 32] = receiver_privkey
            .as_slice()
            .try_into()
            .expect_or_err("LIT_NODE_COMS_KEYS_RECEIVER_PRIVKEY slice with incorrect length")?;
        let se_r_secret = SecretKey::from(receiver_privkey_array);
        let se_r_public = se_r_secret.public_key();

        Ok(Self::new_from_keypairs(
            SenderPrivateKey::from(se_s_secret),
            SenderPublicKey::from(se_s_public),
            ReceiverPrivateKey::from(se_r_secret),
            ReceiverPublicKey::from(se_r_public),
        ))
    }

    pub fn parse_secret_key(secret_key: &str) -> Result<SecretKey> {
        let secret_key = encoding::hex_to_bytes(secret_key)
            .expect_or_err("Failed to hex encode LIT_NODE_COMS_KEYS_SENDER_PRIVKEY config var")?;

        let secret_key_array: [u8; 32] = secret_key
            .as_slice()
            .try_into()
            .expect_or_err("LIT_NODE_COMS_KEYS_SENDER_PRIVKEY slice with incorrect length")?;
        let secret_key = SecretKey::from(secret_key_array);

        Ok(secret_key)
    }

    pub fn receiver_public_key(&self) -> [u8; 32] {
        *self.se_r_pair.public_key().as_ref().as_bytes()
    }
    pub fn sender_public_key(&self) -> [u8; 32] {
        *self.se_s_pair.public_key().as_ref().as_bytes()
    }
    pub fn my_sender_private_key(&self) -> &SenderPrivateKey {
        self.se_s_pair.private_key()
    }
    pub fn my_receiver_private_key(&self) -> &ReceiverPrivateKey {
        self.se_r_pair.private_key()
    }
}

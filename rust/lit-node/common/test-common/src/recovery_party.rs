use blsful::inner_types::{Bls12381G1, G1Projective};
use bulletproofs::BulletproofCurveArithmetic as BCA;
use ethers::types::{Address, H160};
use k256::ecdsa::{RecoveryId, Signature, SigningKey, VerifyingKey};
use sha3::{digest::Digest, Keccak256};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use ethers::middleware::SignerMiddleware;
use ethers::prelude::{Http, LocalWallet, Signer};
use ethers::providers::Provider;
use ethers::signers::Wallet;
use lit_blockchain::contracts::{
    backup_recovery::BackupRecovery,
    staking::{AddressMapping, Staking, Validator},
};
use lit_node::auth::auth_material::JsonAuthSig;
use lit_node::models::DownloadedShareData;
use lit_node::utils::encoding::CompressedPointBytes;
use reqwest::Url;
use std::sync::Arc;
use tracing::info;

const LIT_NODE_DOWNLOAD_SHARE_ENDPOINT: &str = "/web/recovery/get_dec_share";

const PUBLIC_KEYS: [&'static str; 10] = [
    "f39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
    "70997970C51812dc3A010C7d01b50e0d17dc79C8",
    "3C44CdDdB6a900fa2b585dd299e03d12FA4293BC",
    "90F79bf6EB2c4f870365E785982E1f101E93b906",
    "15d34AAf54267DB7D7c367839AAf71A00a2C6A65",
    "9965507D1a55bcC2695C58ba16FB37d819B0A4dc",
    "976EA74026E726554dB657fA54763abd0C3a0aa9",
    "14dC79964da2C08b23698B3D3cc7Ca32193d9955",
    "23618e81E3f5cdF7f54C3d65f7FBc0aBf5B21E8f",
    "a0Ee7A142d267C1f36714E4a8F75612F20a79720",
];

const PRIVATE_KEYS: [&'static str; 10] = [
    "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
    "59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d",
    "5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a",
    "7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6",
    "47e179ec197488593b187f80a00eb0da91f1b9d0b13f8733639f19c30a34926a",
    "8b3a350cf5c34c9194ca85829a2df0ec3153be0318b5e2d3348e872092edffba",
    "92db14e403b83dfe3df233f83dfa3a0d7096f21ca9b0d6d6b8d88b2b4ec1564e",
    "4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356",
    "dbda1821b80551c9d65939329250298aa3472ba22feea921c0cf5d620ea67b97",
    "2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6",
];

pub trait EthereumAddress {
    fn to_eth_address(&self) -> String;
}

pub trait EthereumSignature {
    fn sign_eth(&self, pre_hash: &[u8]) -> (Signature, RecoveryId);
}

pub trait SiweSignature {
    fn sign_siwe(&self, pre_hash: &[u8]) -> (Signature, RecoveryId);
}

impl EthereumAddress for VerifyingKey {
    fn to_eth_address(&self) -> String {
        let pub_key_pt = self.to_encoded_point(false);
        let digest = keccak256(&pub_key_pt.as_bytes()[1..]);
        let last_20 = <[u8; 20]>::try_from(&digest[12..]).unwrap();
        let address = fmt_address(&last_20);
        let mut buffer = String::new();
        buffer.push('0');
        buffer.push('x');
        buffer.push_str(&String::from_utf8(address.to_vec()).unwrap());
        buffer
    }
}

impl EthereumAddress for SigningKey {
    fn to_eth_address(&self) -> String {
        let public_key = self.verifying_key();
        public_key.to_eth_address()
    }
}

impl EthereumSignature for SigningKey {
    fn sign_eth(&self, message: &[u8]) -> (Signature, RecoveryId) {
        let digest = keccak256(message);
        self.sign_prehash_recoverable(&digest).unwrap()
    }
}

impl SiweSignature for SigningKey {
    fn sign_siwe(&self, pre_hash: &[u8]) -> (Signature, RecoveryId) {
        const PREFIX: &str = "\x19Ethereum Signed Message:\n";
        let mut hasher = Keccak256::default();
        let len = pre_hash.len();
        let len_str = len.to_string();
        hasher.update(PREFIX.as_bytes());
        hasher.update(len_str.as_bytes());
        hasher.update(pre_hash);
        let digest = hasher.finalize();
        self.sign_prehash_recoverable(&digest).unwrap()
    }
}

fn fmt_address(bytes: &[u8; 20]) -> [u8; 40] {
    let mut buffer = [0u8; 40];
    hex::encode_to_slice(bytes, &mut buffer).unwrap();

    let checksum = keccak256(&buffer);

    for i in 0..buffer.len() {
        let byte = checksum[i / 2];
        let nibble = 0xf & if i & 1 == 0 { byte >> 4 } else { byte };
        if nibble >= 8 {
            buffer[i] = buffer[i].to_ascii_uppercase();
        }
    }
    buffer
}

fn keccak256(bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::default();
    hasher.update(bytes);
    hasher.finalize().into()
}

pub fn get_recovery_member_signing_key() -> SigningKey {
    let bytes = hex::decode(PRIVATE_KEYS[0]).unwrap();
    SigningKey::from_slice(&bytes).unwrap()
}

fn to_address(pub_key: &str) -> Address {
    let bytes = hex::decode(pub_key).unwrap();
    Address::from_slice(&bytes)
}

pub fn get_wallet_addresses(count: usize) -> Vec<Address> {
    PUBLIC_KEYS[0..count - 1]
        .iter()
        .map(|k| to_address(k))
        .collect()
}

fn get_auth_sig(signing_key: &SigningKey, signed_message: String) -> JsonAuthSig {
    let address = signing_key.to_eth_address();
    let (signature, recovery_id) = signing_key.sign_siwe(signed_message.as_bytes());
    let mut buffer = [0u8; 65];
    buffer[..64].copy_from_slice(&signature.to_bytes());
    buffer[64] = recovery_id.to_byte();

    JsonAuthSig::new(
        hex::encode(buffer),
        "web3.eth.personal.sign".to_string(),
        signed_message,
        address,
        None,
    )
}

pub fn get_contracts_for_recovery_member(
    backup_recovery_address: H160,
    staking_address: H160,
    rpc_url: &str,
    chain_id: u64,
) -> (
    BackupRecovery<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    Staking<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
) {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .use_rustls_tls()
        .build()
        .unwrap();
    let url = Url::parse(rpc_url).unwrap();
    let provider = Provider::new(Http::new_with_client(url, client));
    let secret_key = get_recovery_member_signing_key();
    let wallet = LocalWallet::from(secret_key).with_chain_id(chain_id);
    let sm = Arc::new(SignerMiddleware::new(provider.clone(), wallet));
    let backup_recovery_contract = BackupRecovery::new(backup_recovery_address, sm.clone());
    let staking_contract = Staking::new(staking_address, sm.clone());
    (backup_recovery_contract, staking_contract)
}

pub async fn get_validator_struct_for_recovery_share(
    backup_recovery_contract: BackupRecovery<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    staking_contract: Staking<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
) -> Validator {
    let node_address = backup_recovery_contract
        .get_node_for_backup_member()
        .call()
        .await
        .unwrap();

    let staker_mappings: Vec<AddressMapping> = staking_contract
        .get_node_staker_address_mappings(vec![node_address])
        .call()
        .await
        .unwrap();
    let node_staker_address = staker_mappings[0].staker_address;

    get_validator_struct_from_staker_address(staking_contract, node_staker_address).await
}

pub async fn get_validator_struct_from_staker_address(
    staking_contract: Staking<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    staker_address: H160,
) -> Validator {
    info!("Getting comm address of the staker: {}", staker_address);
    let val_struct: Vec<Validator> = staking_contract
        .get_validators_structs(vec![staker_address])
        .call()
        .await
        .unwrap();

    if val_struct.is_empty() {
        tracing::error!("Could not find validator with given address");
    }

    val_struct[0].clone()
}

pub async fn download_share(validator: &Validator) -> Vec<DownloadedShareData> {
    let url = format!(
        "http://{}:{}{}",
        std::net::Ipv4Addr::from(validator.ip),
        validator.port,
        LIT_NODE_DOWNLOAD_SHARE_ENDPOINT
    );
    info!("Downloading share data from: {}", url);
    let key = get_recovery_member_signing_key();
    let auth_sig = get_auth_sig(
        &key,
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string(),
    );
    let mut json_map = serde_json::Map::new();
    let auth_sig_val = serde_json::to_value(auth_sig).unwrap();
    json_map.insert("authSig".to_string(), auth_sig_val);
    info!("sending request for share download");

    let client = reqwest::ClientBuilder::new()
        .tls_sni(false)
        .build()
        .unwrap();
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&json_map).unwrap())
        .send()
        .await
        .unwrap();
    let response_bytes = response.bytes().await.unwrap();
    let share_data: Vec<DownloadedShareData> = serde_json::from_slice(&response_bytes).unwrap();
    info!("got share data{:?}", share_data);
    share_data
}

pub fn check_share_data(mut share_data: Vec<DownloadedShareData>) {
    assert_eq!(share_data.len(), 2);

    let share1 = share_data.pop().unwrap();
    let share2 = share_data.pop().unwrap();
    let (bls_share, ecdsa_share) = match (share1.curve.as_str(), share2.curve.as_str()) {
        ("BLS12381G1", "Secp256k1") => (share1, share2),
        ("Secp256k1", "BLS12381G1") => (share2, share1),
        (x, y) => panic!("Expected BLS12831G1 and Secp256k1, found {} and {}", x, y),
    };

    // Parse BLS public key
    let sized_array: [u8; 48] = hex::decode(&bls_share.encryption_key)
        .unwrap()
        .as_slice()
        .try_into()
        .unwrap();
    G1Projective::from_compressed(&sized_array).unwrap();

    // Parse BLS private key
    let _ = <Bls12381G1 as BCA>::Scalar::from_be_hex(&bls_share.decryption_key_share);

    // Parse ECDSA public key
    k256::ProjectivePoint::from_compressed(&hex::decode(&ecdsa_share.encryption_key).unwrap())
        .unwrap();
    // Parse ECDSA private key
    let scalar_primitive = elliptic_curve::scalar::ScalarPrimitive::from_slice(
        &hex::decode(&ecdsa_share.decryption_key_share).unwrap(),
    )
    .unwrap();
    let _ = k256::Scalar::from(&scalar_primitive);
}

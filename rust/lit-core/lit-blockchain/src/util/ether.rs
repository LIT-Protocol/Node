use ethers::prelude::Address;
use ethers::types::TransactionReceipt;
use ethers::utils::keccak256;
use serde_json::{Map, Value};

use crate::error::{conversion_err, Result};

/// This method is provided primarily for errors/logging.
pub fn transaction_receipt_to_serde(txn_rec: &TransactionReceipt) -> Value {
    let mut map: Map<String, Value> = Map::new();
    map.insert("transaction_hash".into(), Value::String(format!("{}", txn_rec.transaction_hash)));
    if let Some(status) = txn_rec.status {
        map.insert("status".into(), Value::String(format!("{status}")));
    }

    Value::Object(map)
}

/// Logic borrowed from ethers utils::secret_key_to_address
pub fn public_key_to_address(public_key: &[u8]) -> Result<Address> {
    if public_key[0] != 0x04 {
        return Err(conversion_err(
            "public_key_to_address given public key without 0x04 prefix", None,
        ));
    }

    let hash = keccak256(&public_key[1..]);

    Ok(Address::from_slice(&hash[12..]))
}

use gennaro_dkg::Round1P2PData;
use serde::{de, ser, Deserialize, Deserializer, Serialize, Serializer};
use soteria_rs::Protected;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

pub fn serialize<S: Serializer>(
    input: &BTreeMap<usize, Arc<Mutex<Protected>>>,
    s: S,
) -> Result<S::Ok, S::Error> {
    let mut placeholder = BTreeMap::new();
    for (key, value) in input {
        let mut p = value
            .lock()
            .map_err(|_e| ser::Error::custom("Unable to acquire lock".to_string()))?;
        let u = p
            .unprotect()
            .ok_or_else(|| ser::Error::custom("memory tampered"))?;
        let val = u
            .serde::<Round1P2PData>()
            .map_err(|e| ser::Error::custom(e.to_string()))?;
        placeholder.insert(*key, val);
    }

    placeholder.serialize(s)
}

pub fn deserialize<'de, D: Deserializer<'de>>(
    d: D,
) -> Result<BTreeMap<usize, Arc<Mutex<Protected>>>, D::Error> {
    let input = BTreeMap::<usize, Round1P2PData>::deserialize(d)?;
    let mut placeholder = BTreeMap::new();
    for (key, value) in &input {
        let val = Arc::new(Mutex::new(
            Protected::serde(value).map_err(|_e| de::Error::custom("Serde Error".to_string()))?,
        ));
        placeholder.insert(*key, val);
    }
    Ok(placeholder)
}

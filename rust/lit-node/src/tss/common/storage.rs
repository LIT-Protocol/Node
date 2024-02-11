use super::key_type::KeyType;
use super::traits::keyresolver::KeyResolver;
use crate::config::{
    backup_key_path, beaver_triple_path, bls_key_path, ecdsa_key_path, segmented_paths,
};
use crate::error::{io_err, io_err_code, unexpected_err, unexpected_err_code, Result, EC};
use async_std::path::{Path, PathBuf};
use glob::glob;
use lit_core::error::Unexpected;
use std::fs;
use std::io::{Error, ErrorKind};
use tracing::instrument;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StorageType {
    KeyShare(KeyType),
    BeaverTriple,
    Backup(KeyType),
}

impl StorageType {
    fn get_root_dir(&self) -> Result<PathBuf> {
        match self {
            StorageType::KeyShare(KeyType::BLS) => Ok(bls_key_path()),
            StorageType::KeyShare(KeyType::EcdsaCaitSith) => Ok(ecdsa_key_path()),
            StorageType::BeaverTriple => Ok(beaver_triple_path()),
            StorageType::Backup(_) => Ok(backup_key_path()),
        }
    }

    fn file_name_prefix(&self) -> &'static str {
        match self {
            StorageType::KeyShare(_) => "Key",
            StorageType::BeaverTriple => "BeaverTriple",
            StorageType::Backup(_) => "Backup",
        }
    }
}

impl From<StorageType> for KeyType {
    fn from(storage_type: StorageType) -> KeyType {
        match storage_type {
            StorageType::KeyShare(key_type) => key_type,
            StorageType::BeaverTriple => KeyType::EcdsaCaitSith,
            StorageType::Backup(key_type) => key_type,
        }
    }
}

/**************** KEY SHARE ****************/

#[doc = "Reads a local key share from disk"]
#[instrument(name = "read_key_share_from_disk")]
pub async fn read_key_share_from_disk<T>(pubkey: &str, share_index: u16, epoch: u64) -> Result<T>
where
    T: serde::de::DeserializeOwned + KeyResolver,
{
    let key_type = T::resolve_key_from_trait();
    let storage_type = StorageType::KeyShare(key_type);
    let path = get_full_path(storage_type, pubkey, share_index, None, Some(epoch)).await?;
    do_read_from_disk(&path).await
}

#[doc = "Writes a local key share to disk"]
#[instrument(name = "write_key_share_to_disk", skip_all)]
pub async fn write_key_share_to_disk<T>(
    pubkey: &str,
    share_index: u16,
    epoch: u64,
    local_key: &T,
) -> Result<bool>
where
    T: serde::Serialize + std::marker::Sync + KeyResolver,
{
    let key_type = local_key.resolve_key();
    let storage_type = StorageType::KeyShare(key_type);
    let path = get_full_path(storage_type, pubkey, share_index, None, Some(epoch)).await?;
    do_write_to_disk(&path, local_key).await
}

#[allow(dead_code)] // FIXME: this code will be used in the near future; remove the allow(dead_code) when it is used
#[doc = "Checks if a local key share exists on disk"]
pub(crate) async fn key_share_exists(
    pubkey: &str,
    share_index: u16,
    epoch: u64,
) -> Result<Option<KeyType>> {
    for key_type in KeyType::into_iter() {
        let storage_type = StorageType::KeyShare(key_type);
        let path = get_full_path(storage_type, pubkey, share_index, None, Some(epoch)).await?;
        if path.exists().await {
            return Ok(Some(key_type));
        }
    }
    Ok(None)
}

#[doc = "Checks if a local key share exists on disk"]
#[instrument]
pub(crate) async fn any_key_share_exists(pubkey: &str) -> Result<Option<(KeyType, u16)>> {
    let pubkey = match pubkey.starts_with("0x") {
        true => &pubkey[2..],
        false => pubkey,
    };

    for key_type in KeyType::into_iter() {
        let file_names = fetch_file_names(StorageType::KeyShare(key_type), pubkey, "*").await?;
        if let Some(file_name) = file_names.first() {
            let (_, _, share_index) = file_name_parts(file_name)?;
            info!("Found key share: {} - {:?}", file_name, key_type);
            return Ok(Some((key_type, share_index)));
        }
    }

    debug!("No key share found for {}", pubkey);
    Ok(None)
}

#[allow(dead_code)]
#[doc = "Delete a key share from disk."]
#[instrument(name = "delete_keyshare")]
pub(crate) async fn delete_keyshare(
    key_type: KeyType,
    pubkey: &str,
    share_index: u16,
    epoch: u64,
) -> Result<bool> {
    let storage_type = StorageType::KeyShare(key_type);
    let path = get_full_path(storage_type, pubkey, share_index, None, Some(epoch)).await?;
    delete_from_disk(path)
}

/**************** BEAVER TRIPLE PAIR ****************/

#[doc = "Reads a beaver triple pair from disk"]
#[instrument(name = "read_beaver_triple_from_disk")]
pub async fn read_beaver_triple_from_disk<T>(pubkey: &str, share_index: u16) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let path = get_full_path(StorageType::BeaverTriple, pubkey, share_index, None, None).await?;
    do_read_from_disk(&path).await
}

#[doc = "Reads a beaver triple pair from disk"]
#[instrument(name = "read_beaver_triple_from_disk_direct")]
pub async fn read_beaver_triple_from_disk_direct<T>(filename: &str) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let path = PathBuf::from(filename);
    do_read_from_disk(&path).await
}

#[doc = "Writes a beaver triple pair to disk"]
#[instrument(name = "write_beaver_triple_to_disk", skip_all)]
pub async fn write_beaver_triple_to_disk<T>(
    pubkey: &str,
    share_index: u16,
    local_key: &T,
) -> Result<bool>
where
    T: serde::de::DeserializeOwned + serde::Serialize + std::marker::Sync,
{
    let path = get_full_path(StorageType::BeaverTriple, pubkey, share_index, None, None).await?;
    do_write_to_disk(&path, local_key).await
}

#[doc = "Delete a beaver triple pair from disk."]
#[instrument(name = "delete_beaver_triple")]
pub(crate) async fn delete_beaver_triple(pubkey: &str, share_index: u16) -> Result<bool> {
    let path = get_full_path(StorageType::BeaverTriple, pubkey, share_index, None, None).await?;
    delete_from_disk(path)
}

/**************** BACKUP KEYS ****************/

#[doc = "Writes a key share to be backed up to disk"]
#[instrument(name = "write_backup_to_disk", skip_all, ret)]
pub async fn write_backup_to_disk<T>(
    pubkey: &str,
    share_index: u16,
    local_key: &T,
    peers: &[String],
) -> Result<bool>
where
    T: serde::de::DeserializeOwned + serde::Serialize + std::marker::Sync + KeyResolver,
{
    let key_type = local_key.resolve_key();
    let storage_type = StorageType::Backup(key_type);
    let path = get_full_path(
        storage_type,
        pubkey,
        share_index,
        Some(peers_hash(peers)),
        None,
    )
    .await?;
    // Backup is different than the other types, in that older data should not be overwriten
    // by the newer. Luckily, epochs are an hour away from each other, so the concurrency is
    // not an issue in this case.
    if !path.exists().await {
        do_write_to_disk(&path, local_key).await?;
    }
    Ok(true)
}

#[doc = "Reads backup files with the matching key type and pubkey from disk"]
#[instrument(name = "read_backup_from_disk")]
pub async fn read_backup_from_disk<T>(pubkey: &str, node_set_hash: &str) -> Result<Vec<(String, T)>>
where
    T: serde::de::DeserializeOwned + serde::Serialize + std::marker::Sync + KeyResolver,
{
    let key_type = T::resolve_key_from_trait();
    let storage_type = StorageType::Backup(key_type);
    let file_names = fetch_file_names(storage_type, pubkey, node_set_hash).await?;
    let root_dir = storage_type.get_root_dir()?;

    let mut shares = Vec::with_capacity(file_names.len());
    for file_name in file_names.into_iter() {
        let mut path = root_dir.clone();
        path.push(file_name.clone());
        let share = do_read_from_disk(&path).await?;
        shares.push((file_name, share));
    }

    Ok(shares)
}

#[doc = "Reads all the backup files with the matching key type from disk"]
pub async fn read_all_backup_from_disk<T>(
    node_set_hash: &Option<String>,
) -> Result<Vec<(String, T)>>
where
    T: serde::de::DeserializeOwned + serde::Serialize + std::marker::Sync + KeyResolver,
{
    let hash = match node_set_hash {
        Some(hash) => hash,
        None => "*", // Match anything
    };
    read_backup_from_disk::<T>("*", hash).await
}

#[doc = "Removes the backup of a key share from disk"]
#[instrument(name = "delete_backup_from_disk", ret)]
pub async fn delete_backup_from_disk<T>(
    pubkey: &str,
    share_index: u16,
    peers: &[String],
) -> Result<bool>
where
    T: serde::de::DeserializeOwned + serde::Serialize + std::marker::Sync + KeyResolver,
{
    let key_type = T::resolve_key_from_trait();
    let storage_type = StorageType::Backup(key_type);
    let path = get_full_path(
        storage_type,
        pubkey,
        share_index,
        Some(peers_hash(peers)),
        None,
    )
    .await?;
    delete_from_disk(path)
}

#[doc = "Deletes backup files with the matching key type and pubkey from disk"]
#[instrument(name = "delete_backups_from_disk")]
pub async fn delete_backups_from_disk<T>(pubkey: &str) -> Result<()>
where
    T: serde::de::DeserializeOwned + serde::Serialize + std::marker::Sync + KeyResolver,
{
    let key_type = T::resolve_key_from_trait();
    let storage_type = StorageType::Backup(key_type);
    let file_names = fetch_file_names(storage_type, pubkey, "*").await?;
    let root_dir = storage_type.get_root_dir()?;

    for file_name in file_names.iter() {
        let mut path = root_dir.clone();
        path.push(file_name);
        delete_from_disk(path)?;
    }

    Ok(())
}

/**************** RESTORATION DATA ****************/

#[doc = "Reads encrypted keys from disk"]
pub async fn read_encrypted_keys_from_disk<T>(path: &PathBuf) -> Result<Vec<(String, T)>>
where
    T: serde::de::DeserializeOwned + serde::Serialize + std::marker::Sync + KeyResolver,
{
    let key_type = T::resolve_key_from_trait();
    let storage_type = StorageType::Backup(key_type);
    let file_names = fetch_file_names_in_path(storage_type, "*", "*", path.clone()).await?;

    let mut shares = Vec::with_capacity(file_names.len());
    for file_name in file_names.into_iter() {
        let mut path = path.clone();
        path.push(file_name.clone());
        let share: T = do_read_from_disk(&path).await?;
        let public_key_in_file_name = fetch_public_key_from_file_name(&file_name)?;
        shares.push((public_key_in_file_name, share));
    }

    Ok(shares)
}

#[doc = "Writes given data to disk"]
pub async fn write_to_disk<T>(mut path: PathBuf, file_name: &str, data: &T) -> Result<()>
where
    T: serde::Serialize + std::marker::Sync,
{
    path.push(file_name);
    do_write_to_disk(&path, data).await.map(|b| ())
}

#[doc = "Reads requested type from disk"]
pub async fn read_from_disk<T>(mut path: PathBuf, file_name: &str) -> Result<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    path.push(file_name);
    do_read_from_disk(&path).await
}

/**************** INTERNAL FUNCTIONS ****************/

#[doc = "Reads local share from disk"]
#[instrument(name = "do_read_from_disk")]
pub async fn do_read_from_disk<T>(path: &PathBuf) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let file = fs::File::open(path).map_err(|e| {
        io_err_code(
            e,
            EC::NodeSystemFault,
            Some(format!(
                "Could not open file for reading: {:?}",
                path.clone()
            )),
        )
    })?;

    let local_key: T = ciborium::from_reader(file).map_err(|e| {
        unexpected_err_code(
            e,
            EC::NodeSystemFault,
            Some(format!("Could not deserialize file: {:?}", path)),
        )
    })?;

    Ok(local_key)
}

#[doc = "Writes a local share to disk"]
#[instrument(name = "do_write_to_disk", skip(local_key))]
async fn do_write_to_disk<T>(path: &PathBuf, local_key: &T) -> Result<bool>
where
    T: serde::Serialize + std::marker::Sync,
{
    debug!("Writing to disk: {:?}", path);

    // CBOR
    let file = fs::File::create(path).map_err(|e| {
        io_err_code(
            e,
            EC::NodeSystemFault,
            Some(format!(
                "Could not open key file for writing: {:?}",
                path.clone()
            )),
        )
    })?;

    ciborium::into_writer(local_key, file).map_err(|e| {
        io_err_code(
            e,
            EC::NodeSystemFault,
            Some(format!("Could not write key file: {:?}", path)),
        )
    })?;

    Ok(true)
}

#[doc = "Delete data file from disk."]
#[instrument(name = "delete_from_disk", ret)]
fn delete_from_disk(path: PathBuf) -> Result<bool> {
    fs::remove_file(path.clone()).map_err(|e| {
        unexpected_err_code(
            e,
            EC::NodeSystemFault,
            Some(format!("Could not delete file: {:?}", path)),
        )
    })?;
    Ok(true)
}

async fn get_full_path(
    storage_type: StorageType,
    pubkey: &str,
    share_index: u16,
    node_set_hash: Option<u64>,
    epoch: Option<u64>,
) -> Result<PathBuf> {
    let mut path = get_directory(storage_type, pubkey).await?;
    let file_name = get_file_name(storage_type, pubkey, share_index, node_set_hash, epoch);
    path.push(file_name);
    Ok(path)
}

async fn get_directory(storage_type: StorageType, pubkey: &str) -> Result<PathBuf> {
    let root_dir = storage_type.get_root_dir()?;
    let path = match storage_type {
        StorageType::Backup(_) => root_dir,
        _ => segmented_paths(root_dir, pubkey, 3, true)?,
    };
    create_storage_dir(path.as_path()).await?;
    Ok(path)
}

fn get_file_name(
    storage_type: StorageType,
    pubkey: &str,
    share_index: u16,
    node_set_hash: Option<u64>,
    epoch: Option<u64>,
) -> String {
    let hash = match node_set_hash {
        Some(hash) => hash.to_string(),
        None => String::from("H"),
    };

    let hash = match epoch {
        Some(epoch) => format!("{}-{}", hash, epoch),
        None => hash,
    };

    let prefix = storage_type.file_name_prefix();
    let key_type = KeyType::from(storage_type) as u8;
    format!(
        "{}-H-{}-{}-{}-{}.cbor",
        prefix, key_type, pubkey, share_index, hash
    )
}

fn fetch_public_key_from_file_name(file_name: &str) -> Result<String> {
    let mut split = file_name.split('-');
    match split.nth(3) {
        Some(pub_key) => Ok(String::from(pub_key)),
        None => {
            let err_msg = format!("{} is not a valid key file name", file_name);
            Err(unexpected_err(Error::new(ErrorKind::Other, err_msg), None))
        }
    }
}

fn file_name_parts(file_name: &str) -> Result<(KeyType, String, u16)> {
    let parts: Vec<&str> = file_name.split('-').collect();

    let key_type_from_file = parts[2].parse::<u16>().map_err(|e| io_err(e, None))?;
    let key_type: KeyType = KeyType::try_from(key_type_from_file as u8)?;
    let pubkey = String::from(parts[3]);
    let share_index = parts[4].parse::<u16>().map_err(|e| io_err(e, None))?;
    Ok((key_type, pubkey, share_index))
}

async fn create_storage_dir(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    if path.exists().await {
        return Ok(());
    }

    if let Err(e) = fs::create_dir_all(path).map_err(|e| io_err(e, None)) {
        // Might happen during concurrent calls, we'll check below.
        if !path.exists().await {
            return Err(e);
        }
    }

    Ok(())
}

#[doc = "Returns file names if such data exists on disk"]
async fn fetch_file_names(
    storage_type: StorageType,
    pubkey: &str,
    node_set_hash: &str,
) -> Result<Vec<String>> {
    let path = get_directory(storage_type, pubkey).await?;
    fetch_file_names_in_path(storage_type, pubkey, node_set_hash, path).await
}

#[doc = "Returns file names from the directory if such data exists on disk"]
async fn fetch_file_names_in_path(
    storage_type: StorageType,
    pubkey: &str,
    node_set_hash: &str,
    mut path: PathBuf,
) -> Result<Vec<String>> {
    let key_type = KeyType::from(storage_type) as u8;
    path.push(format!(
        "{}-H-{}-{}-*-{}.cbor",
        storage_type.file_name_prefix(),
        key_type,
        pubkey,
        node_set_hash
    ));

    let pattern = path
        .to_str()
        .expect_or_err("Could not convert path to string")?;
    info!("Checking for data existence: {}", pattern);

    match glob(pattern) {
        Err(e) => {
            debug!("Error reading glob pattern: {} - {:?}", pattern, e);
            Err(io_err(e, None))
        }
        Ok(entries) => Ok(entries
            .flatten()
            .filter_map(|entry| match entry.as_path().file_name() {
                Some(name) => name.to_str().map(String::from),
                None => None,
            })
            .collect()),
    }
}

// Hashes the peers set, ensuring that a reorder does not change the hash value.
pub(crate) fn peers_hash(peers: &[String]) -> u64 {
    use std::hash::{Hash, Hasher};
    peers.iter().fold(0, |acc, p| {
        // The `finish` method does not reset the hash. Therefore, we need
        // a fresh hasher for the computation of the hash of each peer.
        // Otherwise, the order would affect the result.
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        p.hash(&mut hasher);
        let hash = hasher.finish();
        // XOR is commutative, as required here.
        acc ^ hash
    })
}

#[cfg(test)]
mod test {
    use crate::tests::key_shares::{TEST_BLS_KEY_SHARE, TEST_BLS_KEY_SHARE_2};
    use crate::tss::blsful::models::BlsKeyShare;
    use crate::tss::common::storage::{
        delete_backups_from_disk, peers_hash, read_backup_from_disk, write_backup_to_disk,
    };
    use blsful::inner_types::G1Projective;

    #[test]
    fn test_peers_hash() {
        let peers1: Vec<_> = ["1", "2", "3"].iter().map(|s| s.to_string()).collect();
        let peers2: Vec<_> = ["1", "3", "2"].iter().map(|s| s.to_string()).collect();
        let peers3: Vec<_> = ["1", "2", "4"].iter().map(|s| s.to_string()).collect();
        let peers4: Vec<_> = ["1", "2", "3", "4"].iter().map(|s| s.to_string()).collect();

        let hash1 = peers_hash(&peers1);
        let hash2 = peers_hash(&peers2);
        let hash3 = peers_hash(&peers3);
        let hash4 = peers_hash(&peers4);
        let hash1_2 = peers_hash(&peers1);

        assert_eq!(hash1, hash2);
        assert_eq!(hash1, hash1_2);
        assert_ne!(hash1, hash3);
        assert_ne!(hash1, hash4);
    }

    #[test]
    fn test_writing_backups_to_disk() {
        type BlsShare = BlsKeyShare<G1Projective>;
        let rt = tokio::runtime::Runtime::new().expect("Could not create tokio runtime.");
        let key_1: BlsShare = serde_json::from_str(TEST_BLS_KEY_SHARE).unwrap();
        let key_2: BlsShare = serde_json::from_str(TEST_BLS_KEY_SHARE_2).unwrap();

        let peers_a: Vec<String> = ["1", "2", "3"].iter().map(|s| s.to_string()).collect();
        let peers_b: Vec<String> = ["A", "B", "C"].iter().map(|s| s.to_string()).collect();

        let pubkey = "pubkey";

        // 0. Remove from the disk the remainings of previous runs.
        let _ = rt.block_on(delete_backups_from_disk::<BlsShare>(pubkey));

        // 1. Write one key and check if it could be read back.
        rt.block_on(write_backup_to_disk(pubkey, 0, &key_1, &peers_a))
            .unwrap();
        let shares = rt
            .block_on(read_backup_from_disk::<BlsShare>(pubkey, "*"))
            .unwrap();
        let shares: Vec<_> = shares.into_iter().map(|(file_name, share)| share).collect();
        assert!(shares.contains(&key_1));

        // 2. Try to write another key with the same pubkey and node_set.
        // Assert that the key is not updated.
        rt.block_on(write_backup_to_disk(pubkey, 0, &key_2, &peers_a))
            .unwrap();
        let shares = rt
            .block_on(read_backup_from_disk::<BlsShare>(pubkey, "*"))
            .unwrap();
        let shares: Vec<_> = shares.into_iter().map(|(file_name, share)| share).collect();
        assert!(shares.contains(&key_1));
        assert!(!shares.contains(&key_2));

        // 3. Now write this second key with the same pubkey but a different node_set.
        // Assert that both keys still exist.
        rt.block_on(write_backup_to_disk(pubkey, 0, &key_2, &peers_b))
            .unwrap();
        let hash_a = peers_hash(&peers_a).to_string();
        let hash_b = peers_hash(&peers_b).to_string();
        let shares_a = rt
            .block_on(read_backup_from_disk::<BlsShare>(pubkey, &hash_a))
            .unwrap();
        let shares_b = rt
            .block_on(read_backup_from_disk::<BlsShare>(pubkey, &hash_b))
            .unwrap();
        let shares_all = rt
            .block_on(read_backup_from_disk::<BlsShare>(pubkey, "*"))
            .unwrap();
        let shares_a: Vec<_> = shares_a.into_iter().map(|(_, share)| share).collect();
        let shares_b: Vec<_> = shares_b.into_iter().map(|(_, share)| share).collect();
        let shares_all: Vec<_> = shares_all.into_iter().map(|(_, share)| share).collect();
        assert!(shares_all.contains(&key_1));
        assert!(shares_all.contains(&key_2));
        assert!(shares_a.contains(&key_1));
        assert!(!shares_a.contains(&key_2));
        assert!(!shares_b.contains(&key_1));
        assert!(shares_b.contains(&key_2));
    }

    #[test]
    fn test_fetch_public_key_from_file_name() {
        use crate::tss::common::storage::{
            fetch_public_key_from_file_name, get_file_name, KeyType, StorageType,
        };
        let pub_key = "0123456789abcdef";
        let file_name = get_file_name(StorageType::Backup(KeyType::BLS), pub_key, 2, None, None);
        let extracted_pub_key = fetch_public_key_from_file_name(&file_name).unwrap();
        assert_eq!(&extracted_pub_key, pub_key);
    }
}

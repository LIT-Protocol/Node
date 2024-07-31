use std::env;
use std::time::Duration;

use async_std::fs;
use async_std::fs::{File, OpenOptions};
use async_std::future::timeout;
use async_std::io::{ReadExt, WriteExt};
use async_std::path::{Path, PathBuf};
use async_std::stream::StreamExt;
use bytes::{Bytes, BytesMut};
use fs4::async_std::AsyncFileExt;
use ipfs_api_backend_hyper::IpfsApi;
use ipfs_hasher::IpfsHasher;
use ipfs_unixfs::file::adder::FileAdder;
use log::{as_error, as_serde, trace, warn};
use serde_json::Value;

use crate::config::LitConfig;
use crate::env::{ENV_CACHE_PATH_DEFAULT, ENV_CACHE_PATH_KEY};
use crate::error::unexpected::Unexpected;
use crate::error::{conversion_err, io_err, ipfs_err, lock_err, unexpected_err, Error, Result};
use crate::utils::cache::cache_create_path;

const IPFS_CAT_CHUNK_TIMEOUT_MS: u64 = 30000;

/// This method fetches and caches content locally from IPFS. It also verifies the content
/// against the CID to avoid MITM attacks.
pub async fn ipfs_cat_cached_content<S>(cfg: &LitConfig, cid: S) -> Result<(PathBuf, Bytes)>
where
    S: AsRef<str>,
{
    match ipfs_cat_cached(cfg, cid.as_ref(), true).await {
        Ok((path, Some(bytes))) => Ok((path, bytes)),
        Ok((_, None)) => Err(unexpected_err(
            "no content returned from 'ipfs_cat_cached' with want_content: true",
            None,
        )),
        Err(e) => Err(e),
    }
}

/// This method fetches and caches content locally from IPFS. It also verifies the content
/// against the CID to avoid MITM attacks.
pub async fn ipfs_cat_cached(
    cfg: &LitConfig, cid: &str, want_content: bool,
) -> Result<(PathBuf, Option<Bytes>)> {
    let cache_file = PathBuf::from(format!(
        "{}/{}",
        env::var(ENV_CACHE_PATH_KEY).unwrap_or(ENV_CACHE_PATH_DEFAULT.to_string()),
        cid
    ));

    if cache_file.exists().await {
        trace!("ipfs_cat_cached: cache HIT for CID: {cid}");

        if want_content {
            let content = locked_read_file(cache_file.as_path()).await?;

            if let Err(e) = verify_ipfs_cid(cid, &content) {
                _handle_corrupted_file(e, cache_file.as_path()).await?;
                return Box::pin(ipfs_cat_cached(cfg, cid, want_content)).await; // Recursion - start over
            }

            return Ok((cache_file, Some(Bytes::from(content))));
        } else {
            if let Err(e) = verify_ipfs_cid_file(cid, cache_file.as_path()).await {
                _handle_corrupted_file(e, cache_file.as_path()).await?;
                return Box::pin(ipfs_cat_cached(cfg, cid, want_content)).await; // Recursion - start over
            }

            return Ok((cache_file, None));
        }
    } else {
        cache_create_path(cache_file.parent().unwrap()).await?;
    }

    trace!("ipfs_cat_cached: cache MISS for CID: {cid}");

    if want_content {
        let mut content = BytesMut::new();
        ipfs_cat(cfg, cid, Some(cache_file.as_path()), Some(&mut content)).await?;

        Ok((cache_file, Some(content.into())))
    } else {
        ipfs_cat(cfg, cid, Some(cache_file.as_path()), None).await?;

        Ok((cache_file, None))
    }
}

async fn _handle_corrupted_file<P>(e: Error, file: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let filename = file.as_ref().to_str().expect_or_err("File could not to_str()")?;

    warn!(
        error = as_error!(e),
        file = as_serde!(filename);
        "Cached IPFS file corrupted, deleting and re-fetching"
    );

    fs::remove_file(file.as_ref()).await.map_err(|e| {
        io_err(e, Some("failed to remove corrupted IPFS file".into()))
            .add_field("file", Value::from(filename))
    })?;

    Ok(())
}

/// This method fetches and saves content locally from IPFS. It also verifies the content
/// against the CID to avoid MITM attacks.
pub async fn ipfs_cat<S>(
    cfg: &LitConfig, cid: S, path: Option<&Path>, content: Option<&mut BytesMut>,
) -> Result<()>
where
    S: AsRef<str> + Send + Sync,
{
    trace!("ipfs_cat: fetching CID: {}", cid.as_ref());

    let mut file: Option<File> = None;
    if let Some(path) = path {
        let f = OpenOptions::new()
            .write(true)
            .create(true)
            .append(false)
            .open(path)
            .await
            .map_err(|e| io_err(e, None))?;

        f.lock_exclusive().map_err(|e| lock_err(e, None))?;

        let _ = file.insert(f);
    }

    match inner_ipfs_cat(cfg, cid.as_ref(), file.as_ref(), content).await {
        Ok(r) => Ok(r),
        Err(e) => {
            if let Some(file) = file {
                file.unlock().map_err(|e| lock_err(e, None))?;
            }
            if let Some(path) = path {
                if path.exists().await {
                    // Ignore remove error.
                    let _ = fs::remove_file(&path).await;
                }
            }

            Err(e)
        }
    }
}

async fn inner_ipfs_cat<S>(
    cfg: &LitConfig, cid: S, mut file: Option<&File>, mut content: Option<&mut BytesMut>,
) -> Result<()>
where
    S: AsRef<str> + Send + Sync,
{
    let cid = cid.as_ref();
    let ipfs_client = cfg.ipfs_client();

    let mut adder = FileAdder::default();
    let chunk_size = adder.size_hint();
    let timeout_dur = Duration::from_millis(IPFS_CAT_CHUNK_TIMEOUT_MS);

    let mut stream = ipfs_client.cat(cid);
    loop {
        match timeout(timeout_dur, stream.next()).await {
            Ok(Some(res)) => match res {
                Ok(chunk) => {
                    let chunk_len = chunk.len();
                    if chunk_len > chunk_size {
                        return Err(ipfs_err(
                            format!(
                                "chunk size larger than expected! ({} > {})",
                                chunk_len, chunk_size
                            ),
                            None,
                        ));
                    }

                    if let Some(file) = file.as_mut() {
                        timeout(timeout_dur, file.write_all(&chunk[..]))
                            .await
                            .map_err(|e| {
                                io_err(
                                    e,
                                    Some(format!(
                                "file write timed-out after {}ms during IPFS cat for CID: {}",
                                timeout_dur.as_millis(),
                                cid
                            )),
                                )
                            })?
                            .map_err(|e| io_err(e, None))?;
                    }

                    if let Some(content) = content.as_mut() {
                        content.extend_from_slice(&chunk[..]);
                    }

                    let (_blocks, pushed) = adder.push(&chunk[..]);
                    if pushed != chunk_len {
                        return Err(ipfs_err(
                            format!("chunk not fully drained! ({} vs {})", chunk_len, pushed),
                            None,
                        ));
                    }
                }
                Err(e) => return Err(ipfs_err(e, None)),
            },
            Ok(None) => break,
            Err(e) => {
                return Err(ipfs_err(
                    e,
                    Some(format!(
                        "IPFS cat timed-out after {}ms during request for CID: {}",
                        timeout_dur.as_millis(),
                        cid
                    )),
                ))
            }
        }
    }

    if let Some(file) = file.as_mut() {
        file.flush().await.map_err(|e| io_err(e, None))?;
    }

    if let Some((calculated_cid, _data)) = adder.finish().last() {
        let calculated_cid = calculated_cid.to_string();
        if !calculated_cid.eq(cid) {
            return Err(crate::error::ipfs_err(
                format!("IPFS CID miss-match ({} vs {})", cid, &calculated_cid),
                None,
            ));
        }
    } else {
        return Err(crate::error::ipfs_err("IPFS CID calculation missing", None));
    }

    Ok(())
}

/// Verify that the CID matches the content (prevent MITM attacks).
pub fn verify_ipfs_cid<S, B>(expected_cid: S, content: B) -> Result<()>
where
    S: AsRef<str> + Send + Sync,
    B: AsRef<[u8]>,
{
    let expected_cid = expected_cid.as_ref();

    let cid = ipfs_cid_of_content(content.as_ref())?;
    if !cid.as_str().eq(expected_cid) {
        return Err(crate::error::ipfs_err(
            format!("IPFS CID miss-match ({} vs {})", expected_cid, cid),
            None,
        ));
    }

    Ok(())
}

/// Calculate the CID of some given bytes.
pub fn ipfs_cid_of_content<B>(content: B) -> Result<String>
where
    B: AsRef<[u8]>,
{
    let ipfs_hasher = IpfsHasher::default();

    Ok(ipfs_hasher.compute(content.as_ref()))
}

/// Verify that the CID matches the content (prevent MITM attacks).
pub async fn verify_ipfs_cid_file<S, P>(expected_cid: S, path: P) -> Result<()>
where
    S: AsRef<str> + Send + Sync,
    P: AsRef<Path>,
{
    let expected_cid = expected_cid.as_ref();

    let cid = ipfs_cid_of_file(path.as_ref()).await?;
    if !cid.as_str().eq(expected_cid) {
        return Err(crate::error::ipfs_err(
            format!("IPFS CID miss-match ({} vs {})", expected_cid, cid),
            None,
        ));
    }

    Ok(())
}

/// Calculate the IPFS CID of a file
pub async fn ipfs_cid_of_file<P>(path: P) -> Result<String>
where
    P: AsRef<Path>,
{
    let file = File::open(path).await.map_err(|e| io_err(e, None))?;

    file.lock_shared().map_err(|e| lock_err(e, None))?;

    let res = inner_ipfs_cid_of_file(&file).await;
    let _ = file.unlock();
    res
}

async fn inner_ipfs_cid_of_file(mut file: &File) -> Result<String> {
    let mut adder = FileAdder::default();

    let chunk_size = adder.size_hint();
    let mut chunk = Vec::with_capacity(chunk_size);
    loop {
        let n = file
            .by_ref()
            .take(chunk_size as u64)
            .read_to_end(&mut chunk)
            .await
            .map_err(|e| io_err(e, None))?;
        if n == 0 {
            break;
        }

        let (_blocks, pushed) = adder.push(chunk.as_ref());
        if pushed != n {
            return Err(unexpected_err("expected chunk to be drained!", None));
        }

        chunk.clear();

        if n < chunk_size {
            break;
        }
    }

    if let Some((cid, _data)) = adder.finish().last() {
        Ok(cid.to_string())
    } else {
        Err(crate::error::ipfs_err("IPFS CID calculation missing", None))
    }
}

pub fn bytes_to_ipfs_cid<B>(bytes: B) -> Result<String>
where
    B: AsRef<[u8]>,
{
    // base58 encode
    let encoded = bs58::encode(bytes.as_ref()).into_string();

    Ok(encoded)
}

pub fn ipfs_cid_to_bytes<S>(ipfs_id: S) -> Result<Bytes>
where
    S: AsRef<str> + Send + Sync,
{
    // base58 decode
    let decoded = bs58::decode(ipfs_id.as_ref())
        .into_vec()
        .map_err(|e| conversion_err(e, Some("failed to decode ipfs as bs58".into())))?;

    Ok(Bytes::from(decoded))
}

// Utils

async fn locked_read_file<P>(path: P) -> Result<Vec<u8>>
where
    P: AsRef<Path>,
{
    let mut file =
        OpenOptions::new().read(true).open(path.as_ref()).await.map_err(|e| io_err(e, None))?;

    file.lock_shared().map_err(|e| lock_err(e, None))?;

    let mut content: Vec<u8> = Vec::new();

    if let Err(e) = file.read_to_end(&mut content).await.map_err(|e| crate::error::io_err(e, None))
    {
        let _ = file.unlock();
        return Err(e);
    }
    file.read_to_end(&mut content).await.map_err(|e| crate::error::io_err(e, None))?;

    file.unlock().map_err(|e| lock_err(e, None))?;

    Ok(content)
}

#[cfg(test)]
mod tests {
    use crate::utils::ipfs::ipfs_cid_to_bytes;

    #[test]
    fn ipfs_cid_to_bytes_test() {
        let bytes = ipfs_cid_to_bytes("QmVhvbXG4u69s3UUAmNL8RnjndBYcxGPQmHh8aP5uwr5Wz")
            .expect("failed to call ipfs_cid_to_bytes");

        assert_eq!(bytes.len(), 34);
    }
}

use std::{fs, io::Write};

use futures_util::StreamExt;
use serde::Deserialize;
use tracing::{info, trace};
use utils::get_target_branch_commit_hash;

const BRANCH_TARGETS: [&str; 4] = ["develop", "habanero", "manzano", "release-cayenne-*"];

/// This script downloads the latest builds for:
/// - Latest commit to `develop` branch
/// - Latest commit to `habanero` branch
/// - Latest commit to `manzano` branch
/// - Latest commit to the latest Cayenne release branch
///
/// Run as:
/// ```bash
/// GH_PAT=<YOUR_GITHUB_PAT> RUST_LOG=debug cargo run
/// ```
///
/// You will need to have these target branches checked out at least once previously on your local machine.
#[tokio::main]
async fn main() {
    env_logger::init();

    for branch in BRANCH_TARGETS.iter() {
        info!("Getting latest commit hash for branch: {}", branch);

        // Get the latest commit hash for the branch.
        let commit_hash = match get_target_branch_commit_hash(branch) {
            Ok(commit_hash) => commit_hash,
            Err(e) => {
                panic!(
                    "Failed to get latest commit hash for branch: {}: {}",
                    branch, e
                );
            }
        };

        // Fetch the Github artifact against the REST API.
        let req = reqwest::Client::new()
            .get(&format!(
                "https://api.github.com/repos/lit-protocol/lit-assets/actions/artifacts?name=lit_node_{}",
                commit_hash
            ))
            .header("Accept", "application/vnd.github.v4+json")
            .header("Authorization", format!("Bearer {}", std::env::var("GH_PAT").unwrap()))
            .header("X-Github-Api-Version", "2022-11-28")
            .header("User-Agent", "lit-protocol-downloader")
            .send()
            .await
            .expect("Failed to fetch Github artifact");
        // Deserialize the response into an artifact list.
        let artifact_list: ArtifactList = req
            .json()
            .await
            .expect("Failed to deserialize artifact list");
        info!("Artifact list: {:?}", artifact_list);

        // Get the first (latest, and should be the only one anyway!) artifact from the list.
        let artifact = match artifact_list.artifacts.first() {
            Some(artifact) => artifact,
            None => {
                panic!("No artifacts found for branch: {}", branch);
            }
        };

        // Check if the artifact already exists in the /target/debug directory.
        let file_path = format!("../../target/debug/{}", artifact.name);
        if fs::metadata(&file_path).is_ok() {
            info!(
                "Artifact already exists, skipping download: {}",
                artifact.name
            );
            continue;
        }

        // Download the artifact into the /target/debug directory.
        info!("Downloading artifact: {}", artifact.name);
        let mut file = std::fs::File::create(&file_path).expect("Failed to create file");
        let mut response_stream = reqwest::Client::new()
            .get(&artifact.archive_download_url)
            .header("Accept", "application/vnd.github+json")
            .header(
                "Authorization",
                format!("Bearer {}", std::env::var("GH_PAT").unwrap()),
            )
            .header("X-Github-Api-Version", "2022-11-28")
            .header("User-Agent", "lit-protocol-downloader")
            .send()
            .await
            .expect("Failed to download artifact")
            .bytes_stream();

        while let Some(chunk_result) = response_stream.next().await {
            let chunk = chunk_result.expect("Failed to get chunk");
            trace!("Chunk: {:?}", chunk);
            file.write_all(&chunk).expect("Failed to write to file");
        }

        // Once the download has completed, make sure we run chmod +x on the file.
        std::process::Command::new("chmod")
            .arg("+x")
            .arg(&file_path)
            .output()
            .expect("Failed to run chmod on file");
    }

    info!("All artifacts downloaded successfully");
}

#[derive(Debug, Deserialize)]
struct ArtifactList {
    total_count: u32,
    artifacts: Vec<Artifact>,
}

#[derive(Debug, Deserialize)]
struct Artifact {
    id: u32,
    node_id: String,
    name: String,
    size_in_bytes: u32,
    url: String,
    archive_download_url: String,
    expired: bool,
    created_at: String,
    updated_at: String,
}

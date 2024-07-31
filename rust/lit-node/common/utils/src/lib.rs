use std::process::Command;

use anyhow::Result;
use tracing::info;

pub fn get_target_branch(target_branch: &str) -> Result<String> {
    let target_branch = if target_branch.contains('*') {
        let git_branch_command = Command::new("git")
            .args(&["branch", "-al", target_branch])
            .output()
            .map_err(|e| anyhow::anyhow!("Failed to list branches: {}", e))?;
        if !git_branch_command.status.success() {
            return Err(anyhow::anyhow!(
                "Failed to list branches: {}",
                String::from_utf8_lossy(&git_branch_command.stderr)
            ));
        }

        let branches = String::from_utf8_lossy(&git_branch_command.stdout);
        let branches: Vec<&str> = branches.split('\n').collect();
        let branches: Vec<&str> = branches
            .iter()
            .map(|b| b.trim())
            .filter(|b| !b.is_empty())
            .collect();
        if branches.is_empty() {
            return Err(anyhow::anyhow!("No release branches found"));
        }

        // Get the latest release branch. This is how the ordering works:
        // 1. Since the format is in `release-*-YYYY-MM-DD`, strip to get the YYYY-MM-DD part.
        // 2. Then sort by the date using chrono.
        let latest_branch = branches
            .iter()
            // Filter out branches that don't have a parseable date from the last 10 characters.
            .filter(|b| {
                let date = &b[b.len() - 10..];
                chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d").is_ok()
            })
            .max_by(|a, b| {
                // Get the date part by getting the last 10 characters.
                let a_date = &a[a.len() - 10..];
                let b_date = &b[b.len() - 10..];

                // Compare the dates.
                let a_date = chrono::NaiveDate::parse_from_str(a_date, "%Y-%m-%d").unwrap();
                let b_date = chrono::NaiveDate::parse_from_str(b_date, "%Y-%m-%d").unwrap();
                a_date.cmp(&b_date)
            })
            .unwrap();
        latest_branch.to_string()
    } else {
        target_branch.to_string()
    };

    // Strip the 'remotes/' prefix if it exists.
    let target_branch = target_branch.trim_start_matches("remotes/");

    // Strip the 'origin/' prefix if it exists.
    let target_branch = target_branch.trim_start_matches("origin/");

    Ok(target_branch.to_string())
}

pub fn get_target_branch_commit_hash(target_branch: &str) -> Result<String> {
    // Perform a git fetch to get the latest git state.
    let git_fetch_command = Command::new("git")
        .args(&["fetch"])
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to fetch latest git state: {}", e))?;
    if !git_fetch_command.status.success() {
        return Err(anyhow::anyhow!(
            "Failed to fetch latest git state: {}",
            String::from_utf8_lossy(&git_fetch_command.stderr)
        ));
    }

    // Check whether the target branch is a release branch, in which case we would need to use a regex.
    // The release branches are in the format `release-cayenne-2024-01-01`. We should find the one with
    // the most recent date.
    let target_branch_to_get_commit_hash = get_target_branch(target_branch)?;
    info!(
        "Target branch to get commit hash: {}",
        target_branch_to_get_commit_hash
    );

    // Get the commit hash for the target branch using git ls-remote
    let git_ls_remote_command = Command::new("git")
        .args(&["ls-remote", "origin", &target_branch_to_get_commit_hash])
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to get commit hash: {}", e))?;
    if !git_ls_remote_command.status.success() {
        return Err(anyhow::anyhow!(
            "Failed to get commit hash: {}",
            String::from_utf8_lossy(&git_ls_remote_command.stderr)
        ));
    }
    let commit_hash_to_checkout = String::from_utf8_lossy(&git_ls_remote_command.stdout)
        .split_whitespace()
        .next()
        .unwrap()
        .to_string();
    info!(
        "Commit hash for branch {}: {}",
        target_branch_to_get_commit_hash, commit_hash_to_checkout
    );

    Ok(commit_hash_to_checkout)
}

use clap::Args;
use lit_cli_core::cmd::CliGlobalOpts;
use lit_core::config::LitConfig;
use std::error::Error;
use std::path::Path;
use std::process::Command;

// Path to the LIT assets repository
// TBD: Fetch it from LitConfig?
const ASSETS_REPO_PATH: &str = "/opt/assets/lit-assets";
const OS_REPO_PATH: &str = "/opt/assets/lit-os";

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct Version {}

/// Repository information containing git commit hash and branch
#[derive(Debug)]
struct GitInfo {
    commit: String,
    branch: String,
}

/// Retrieves the current git commit hash and branch name of the project
/// Returns a formatted string with either the short hash, branch name or an error message
fn get_git_info(repo_path: &str) -> Result<GitInfo, Box<dyn Error>> {
    if !Path::new(repo_path).exists() {
        return Err(format!("Error: Repository path '{}' not found.", repo_path).into());
    }

    // Get the git commit hash
    let hash_output = Command::new("git")
        .current_dir(repo_path)
        .args(["rev-parse", "--short", "HEAD"])
        .output()?;

    if !hash_output.status.success() {
        return Err(format!(
            "Git command failed: {}",
            String::from_utf8_lossy(&hash_output.stderr)
        )
        .into());
    }

    let commit = String::from_utf8(hash_output.stdout)
        .map(|hash| hash.trim().to_string())
        .map_err(|e| -> Box<dyn Error> { format!("Invalid UTF-8 output: {}", e).into() })?;

    // Get the git branch name
    let branch_output = Command::new("git")
        .current_dir(repo_path)
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()?;

    if !branch_output.status.success() {
        return Err(format!(
            "Git command failed: {}",
            String::from_utf8_lossy(&branch_output.stderr)
        )
        .into());
    }

    let branch = String::from_utf8(branch_output.stdout)
        .map(|branch| branch.trim().to_string())
        .map_err(|e| -> Box<dyn Error> { format!("Invalid UTF-8 output: {}", e).into() })?;

    Ok(GitInfo { commit, branch })
}

pub(crate) fn handle_cmd_os_version(_cfg: LitConfig, _opts: CliGlobalOpts, _arg: Version) -> bool {
    // Get assets repo info
    let assets_info = match get_git_info(ASSETS_REPO_PATH) {
        Ok(info) => format!("commit_hash {} ({})", info.commit, info.branch),
        Err(e) => {
            println!("Error getting assets repo info: {}", e);
            return false;
        }
    };

    // Get os repo info
    let os_info = match get_git_info(OS_REPO_PATH) {
        Ok(info) => format!("commit_hash {} ({})", info.commit, info.branch),
        Err(e) => {
            println!("Error getting os repo info: {}", e);
            return false;
        }
    };

    println!("lit-assets: {}\nlit-os: {}", assets_info, os_info);
    true
}

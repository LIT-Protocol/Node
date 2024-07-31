use utils::get_target_branch_commit_hash;

fn main() {
    // Get the target branch from the command line arguments.
    let target_branch = std::env::args().nth(1).expect("No target branch provided");

    println!(
        "{}",
        get_target_branch_commit_hash(&target_branch)
            .expect("Failed to get target branch commit hash")
    );
}

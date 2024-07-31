use utils::get_target_branch_commit_hash;

fn main() {
    env_logger::init();

    // Get the name of the target branch from the 1st command line argument.
    let target_branch = std::env::args().nth(1).expect("No target branch provided");

    let target_branch_commit_hash = get_target_branch_commit_hash(&target_branch)
        .expect("Failed to get target branch commit hash");

    // Print the target branch commit hash.
    println!("{}", target_branch_commit_hash);
}

# This script is assumed to be run at root of rust/lit-node.

TARGET_BRANCHES=("*release-habanero-*" "*release-manzano-*" "*release-cayenne-*")

# First get the current git branch.
current_branch=$(git rev-parse --abbrev-ref HEAD)

for branch in "${TARGET_BRANCHES[@]}"; do
    # For each target branch, cd into common/utils and run `cargo run <target_branch>`.
    echo "Getting commit hash for branch: $branch"
    commit_hash=$(cd common/utils && cargo run --quiet -- $branch)

    target_file_path="./target/debug/lit_node_${commit_hash}"

    # If the target file exists, log and move on.
    if [ -f $target_file_path ]; then
        echo "Target file already exists for branch: $branch"
        continue
    fi
    echo "Target file does not exist for branch: $branch"

    # If the target file does not exist, git checkout that commit hash.
    echo "Checking out commit hash: $commit_hash"
    git checkout $commit_hash || exit 1

    # Then, run cargo build.
    echo "Building binary for branch: $branch"
    cargo build --features lit-actions,testing

    # Then, rename the binary ./target/debug/lit_node to ./target/debug/lit_node_<commit_hash>
    echo "Renaming binary"
    mv ./target/debug/lit_node $target_file_path

    # Git checkout the original branch and move on to the next target branch.
    echo "Checking out original branch: $current_branch"
    git checkout $current_branch
done

# Finally, git checkout the original branch.
echo "Checking out original branch: $current_branch"
git checkout $current_branch

# Done.
echo "Done."

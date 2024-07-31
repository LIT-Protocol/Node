# Instructions

## Getting The Latest Workflow Run

Run:

```bash
RUST_LOG=debug GH_PAT=<YOUR_GITHUB_PAT> cargo run --bin get_latest_workflow_run rust/lit-node-build-commit-hash develop
```
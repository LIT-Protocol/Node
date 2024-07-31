Note: some of the expect script don't fully exit, so you may need to kill them manually and run the next one manually.

1. To destroy the current prov and create a new one, run `./scripts/remote_dev/lit_os/destroy_then_create_prov.sh`

2. To create a new node release, run `./scripts/remote_dev/lit_os/create_node_release.sh`

3. To install the node release on all the machines, run `./scripts/remote_dev/lit_os/create_all_nodes.sh`

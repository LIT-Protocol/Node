Run these scripts from the lit-assets/rust/lit-node folder.  Your current local copy of the code will be copied over and compiled, so you can make changes locally and then run them remotely, without committing or pushing.

To bootstrap the nodes (this only needs to be done once), and install rust etc, run: ./scripts/remote_dev/bootstrap.sh

To copy your current local build, and rebuild it remotely, and then run it, run: ./scripts/remote_dev/build.sh

To view the logs of a given remote node, use ./scripts/remote_dev/follow_logs.sh.  You may pass a single optional parameter to this, which is the node index to follow (default 0).

To download files from the remote nodes,

```bash
sh ./scripts/remote_dev/download_file.sh <SSH_USERNAME> <REMOTE_FILE_PATH> <LOCAL_FILE_PATH>
```

The node IP addresses to connect to, along with other variables, are in ./scripts/remote_dev/vars.sh.

Message Chris with any questions 🔥
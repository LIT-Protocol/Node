# Lit Protocol Replica Node

To run a replica node for the mainnet, just run `docker-compose -f docker-compose-lit-protocol.yml up --detach`.

To run a replica node for the testnet, just run `docker-compose -f docker-compose-lit-test.yml up --detach`.

The data transport layer service is an indexer that indexes L2 transactions on the Lit testnet node. The replica exposes a port at 8549 which can handle any number of read requests.

We've inserted our mainnet Matic RPC URL (from our Alchemy account) in this demo, but feel free to replace and use your own Polygon RPC URL.

In addition, any transactions (eth_sendRawTransaction requests) will be forwarded to the sequencer http url. Beware of stale nonces though, as the replica will take some time to catch up and process the request.
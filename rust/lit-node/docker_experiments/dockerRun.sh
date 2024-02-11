#!/bin/bash

# this would run the container by itself
# sudo docker run -p 7470:7470 --env "VIRTUAL_HOST=node3.litgateway.com"  --env "LETSENCRYPT_HOST=node3.litgateway.com" --env "VIRTUAL_PORT=7470" --env "LETSENCRYPT_EMAIL=chris@litprotocol.com" lit_node

# runs all the services including nginx reverse proxy
sudo docker-compose up
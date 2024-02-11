#!/bin/bash

sudo systemctl stop lit-node@{0..2}



#lit_node_rust % kill -9 $(lsof -ti:7470,7471,7472,7473,7474,7475,7476,7477,7478,7479);
#!/bin/bash
if [ ! -f "./config.toml" ]; then
  cp config.toml.sample config.toml
fi

CMD="cargo run"
eval $CMD
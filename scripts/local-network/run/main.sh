#!/bin/bash

set -e

FAUCET="subject wagon soup symbol theme during two toilet open demise protect assist field alone monitor private throw weekend stool train travel vessel aisle noise"
VAL_HOME=./backends/cosmos/node-configs/gen-val

(./scripts/codegen/main.sh)
(cd ./frontends/rust/client; wasm-pack build --target web --dev --out-dir ../server/dst;)
(trap 'kill 0' SIGINT; (go run ./scripts/reverse-proxy/main.go) & (./backends/cosmos/scripts/network/genesis-validator/run/main.sh $VAL_HOME) & (cd ./backends/cosmos/cmd/faucet; cargo run "$FAUCET" "http://localhost:9090") & (go run ./frontends/rust/server/main.go))
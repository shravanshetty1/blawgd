#!/bin/bash

MNEMONIC="voice salt fortune fork draw endless figure layer need begin trouble use cream will alpha cheese glad cook monkey used rigid better describe demise"
VAL_HOME=./backends/cosmos/node-configs/gen-val

(./scripts/codegen/main.sh)
(cd ./frontends/rust/client; wasm-pack build --target web --out-dir ../server/dst;)
(trap 'kill 0' SIGINT; (./backends/cosmos/scripts/network/genesis-validator/run/main.sh $VAL_HOME) & (cd ./backends/cosmos/cmd/faucet; cargo run "$MNEMONIC" "http://localhost:9090") & (go run ./frontends/rust/server/main.go))
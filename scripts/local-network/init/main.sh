#!/bin/bash

set -e

MNEMONIC="voice salt fortune fork draw endless figure layer need begin trouble use cream will alpha cheese glad cook monkey used rigid better describe demise"
FAUCET="subject wagon soup symbol theme during two toilet open demise protect assist field alone monitor private throw weekend stool train travel vessel aisle noise"
VAL_HOME=./backends/cosmos/node-configs/gen-val
go run ./backends/cosmos/scripts/network/genesis-validator/init/main.go $VAL_HOME "$MNEMONIC" "$FAUCET"
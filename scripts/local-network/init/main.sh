#!/bin/bash

MNEMONIC="voice salt fortune fork draw endless figure layer need begin trouble use cream will alpha cheese glad cook monkey used rigid better describe demise"
VAL_HOME=./backends/cosmos/node-configs/gen-val
go run ./backends/cosmos/scripts/network/genesis-validator/init/main.go $VAL_HOME "$MNEMONIC"
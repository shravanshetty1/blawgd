#!/bin/bash


rm -R ~/.samachar
go install ./...

echo "voice salt fortune fork draw endless figure layer need begin trouble use cream will alpha cheese glad cook monkey used rigid better describe demise" | samachard keys add alice --keyring-backend test --recover
samachard init val1 --chain-id samachar -o
samachard add-genesis-account cosmos13kx7tkt2kfg4cpsmu9hrfhynlmem4vfl5vl54r 100000000000stake,1000tok
samachard gentx alice 100000000stake --chain-id samachar --keyring-backend test
samachard collect-gentxs
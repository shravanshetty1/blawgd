#!/bin/bash


rm -R ~/.samachar
echo "1"
go install ./cmd/samachard/samachard.go
echo "1"
echo "voice salt fortune fork draw endless figure layer need begin trouble use cream will alpha cheese glad cook monkey used rigid better describe demise" | samachard keys add alice --keyring-backend test --recover
echo "1"
samachard init val1 --chain-id samachar -o
echo "1"
samachard add-genesis-account cosmos13kx7tkt2kfg4cpsmu9hrfhynlmem4vfl5vl54r 100000000000stake
echo "1"
samachard gentx alice 100000000stake --chain-id samachar --keyring-backend test
echo "1"
samachard collect-gentxs
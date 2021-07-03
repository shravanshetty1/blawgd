#!/bin/bash

go install cmd/samachard/main.go

rm -R ~/.samachar
rm -R ~/.samachard
rm -R ~/.samacharcli

samachard keys add alice --keyring-backend test
samachard keys add bob --keyring-backend test
ALICE_ADDRESS=$(samachard keys show alice -a --keyring-backend test)
BOB_ADDRESS=$(samachard keys show bob -a --keyring-backend test)
echo $ALICE_ADDRESS
echo $BOB_ADDRESS

samachard init val1 --chain-id samachar -o
samachard add-genesis-account $ALICE_ADDRESS 100000000000stake,1000tok
samachard add-genesis-account $BOB_ADDRESS 1000tok
samachard gentx alice 100000000stake --chain-id samachar --keyring-backend test
samachard collect-gentxs
samachard start --minimum-gas-prices 0stake
#!/bin/bash

#rm -R ~/.blawgd
rm -R $1
go install ./backends/cosmos/cmd/blawgdd/blawgdd.go
# initialize current system to be a node
blawgdd init genesis-validator --chain-id blawgd -o
# add an account to the keyring
echo "$2" | blawgdd keys add alice --keyring-backend test --recover
GEN_ADDR=$(blawgdd keys show alice -a --keyring-backend test)
echo "$3" | blawgdd keys add bob --keyring-backend test --recover
FAUCET_ADDR=$(blawgdd keys show bob -a --keyring-backend test)

# give the account the networks entire supply of tokens, the account delegates some of its tokens to the current node to convert it to a validator
blawgdd add-genesis-account $GEN_ADDR 1000000000000000stake
blawgdd add-genesis-account $FAUCET_ADDR 100000stake
blawgdd gentx alice 100000000stake --chain-id blawgd --keyring-backend test

# genesis file for the network has been created
blawgdd collect-gentxs

# home flag was not working when i did this
mkdir -p $1
mv ~/.blawgd/* $1/

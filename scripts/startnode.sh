#!/bin/bash

go install ./cmd/samachard/samachard.go
(trap 'kill 0' SIGINT; samachard start --minimum-gas-prices 0stake & (cd ./faucet; cargo run))
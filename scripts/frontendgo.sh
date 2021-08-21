#!/bin/bash

GOOS=js GOARCH=wasm go build -o ./frontend-go/dst/main.wasm ./frontend-go/cmd/wasm/main.go
go install ./...
(trap 'kill 0' SIGINT; samachard start --minimum-gas-prices 0stake & (cd ./faucet; cargo run) & go run ./frontend-go/cmd/frontend-server/main.go)
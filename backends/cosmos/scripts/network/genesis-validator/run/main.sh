#!/bin/bash

go install ./backends/cosmos/cmd/blawgdd/blawgdd.go
blawgdd start --minimum-gas-prices 0stake --home $1
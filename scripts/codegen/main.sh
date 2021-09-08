#!/bin/bash

./scripts/codegen/backend-cosmos/main.sh
cd ./scripts/codegen/frontend-rust; cargo run;
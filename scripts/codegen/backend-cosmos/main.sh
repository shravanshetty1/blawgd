#!/bin/bash

protoc --gocosmos_out=plugins=interfacetype+grpc,Mgoogle/protobuf/any.proto=github.com/cosmos/cosmos-sdk/codec/types:. \
    ./backends/cosmos/api/grpc/blawgd.proto
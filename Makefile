PACKAGES=$(shell go list ./... | grep -v '/simulation')

VERSION := $(shell echo $(shell git describe --tags) | sed 's/^v//')
COMMIT := $(shell git log -1 --format='%H')

ldflags = -X github.com/cosmos/cosmos-sdk/version.Name=samachar \
	-X github.com/cosmos/cosmos-sdk/version.ServerName=samachard \
	-X github.com/cosmos/cosmos-sdk/version.Version=$(VERSION) \
	-X github.com/cosmos/cosmos-sdk/version.Commit=$(COMMIT)

BUILD_FLAGS := -ldflags '$(ldflags)'

all: install

install: go.sum
	@echo "--> Installing samachard"
	@go install -mod=readonly $(BUILD_FLAGS) ./cmd/samachard

go.sum: go.mod
	@echo "--> Ensure dependencies have not been modified"
	GO111MODULE=on go mod verify

test:
	@go test -mod=readonly $(PACKAGES)

frontend:
	GOOS=js GOARCH=wasm go build -o ./client/assets/main.wasm ./client/cmd/wasm
	go run ./client/cmd/server

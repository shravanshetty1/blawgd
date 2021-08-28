# Blawgd

## Test environment setup

You will need the following dependencies -
* Latest version of golang - (for blockchain)
* Latest version of rust - (for frontend)
* Latest version of protoc - (for grpc setup)

To initialize a node - (Warning! this will reset current chain)
```
go run ./scripts/initnode/main.go
```

To start a node -
```
make local
```
.PHONY: frontend

local:
	cd frontend/codegen; cargo run;
	cd frontend/wasm; wasm-pack build --target web --out-dir ../dist;
	./scripts/startnode.sh

go:
	GOOS=js GOARCH=wasm go build -o ./frontend-go/dst/main.wasm ./frontend-go/cmd/wasm/main.go
	go run ./frontend-go/cmd/frontend-server/main.go

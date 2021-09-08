.PHONY: frontend

local:
	cd frontend/codegen; cargo run;
	cd frontend/wasm; wasm-pack build --target web --out-dir ../dist;
	./scripts/startnode.sh


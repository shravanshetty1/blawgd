.PHONY: frontend

local:
	cd frontend/wasm; wasm-pack build --target web --out-dir ../dist;
	pwd
	./scripts/freshnode.sh

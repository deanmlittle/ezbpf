SRC = ./crates/ezbpf-wasm
OUT = ../../wasm

.PHONY: web bundler node wasm

web:
	wasm-pack build --release --out-dir ${OUT}/web --target web ${SRC} && rm ./wasm/web/.gitignore

bundler:
	wasm-pack build --release --out-dir ${OUT}/bundler --target bundler ${SRC} && rm ./wasm/bundler/.gitignore

node:
	wasm-pack build --release --out-dir ${OUT}/node --target nodejs ${SRC} && rm ./wasm/node/.gitignore

wasm: web bundler node
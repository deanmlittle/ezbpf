SRC = ./crates/ezbpf-wasm
OUT = ../../wasm

.PHONY: web bundler node wasm

web:
	wasm-pack build --release --out-dir ${OUT}/web --target web ${SRC}

bundler:
	wasm-pack build --release --out-dir ${OUT}/bundler --target bundler ${SRC}

node:
	wasm-pack build --release --out-dir ${OUT}/node --target nodejs ${SRC}

wasm: web bundler node
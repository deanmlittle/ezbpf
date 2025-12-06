# DEPRECATED: 
This project is now deprecated in favor of [sbpf](https://github.com/blueshift-gg/sbpf) – a more advanced, actively maintained alternative.

## ezBPF

A simple sBPF (Solana eBPF) disassembler. There are 3 main packages:

1. `ezbpf-core` - the core disassembler code with all sbpf instructions, serialization, deserialization
2. `ezbpf-cli` - a CLI for printing out the disassembled code of a .so file
3. `ezbpf-wasm` - a WIP wasm version of ezbpf core for browser-based disassembly

### Installation

To install ezBPF simply run this command

```sh
cargo install --git https://github.com/deanmlittle/ezbpf
```

### How to Use

```
Usage: ezbpf [OPTIONS] <FILENAME>

Arguments:
  <FILENAME>  Filename of IDL file

Options:
  -a, --asm      Print asm instead of json
  -h, --help     Print help
  -V, --version  Print version
```

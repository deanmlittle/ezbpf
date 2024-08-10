# ezBPF
A simple sBPF (Solana eBPF) disassembler. There are 3 main packages:

1. ezbpf-core - the core disassembler code with all sbpf instructions, serialization, deserialization
2. ezbpf-cli - a CLI for printing out the disassembled code of a .so file
3. ezbpf-wasm - a WIP wasm version of ezbpf core for browser-based disassembly

### Installation

To install ezBPF simply run this command

```cargo install --git https://github.com/deanmlittle/ezbpf```

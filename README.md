# wasmfibo

calc fibonacci

## Build

```sh
cargo build --target=wasm32-wasi --release
```

## Run

```sh
wasmer target/wasm32-wasi/release/wasmfibo.wasm 100
```

#!/bin/bash

cargo build --release --target wasm32-unknown-unknown
wasm-build --target wasm32-unknown-unknown ./target wasm_hello_world
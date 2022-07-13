#!/bin/bash
near delete test.crimsoft.testnet crimsoft.testnet
near create-account test.crimsoft.testnet --masterAccount crimsoft.testnet
near deploy test.crimsoft.testnet --wasmFile=./res/rust_counter.wasm
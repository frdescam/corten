#! /bin/bash

cargo run --manifest-path ./deploy_corten/Cargo.toml
cargo build-sbf --manifest-path ../code/program/Cargo.toml
solana program deploy ../target/deploy/program.so
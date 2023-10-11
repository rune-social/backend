#!/bin/bash
cargo +nightly fmt --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test
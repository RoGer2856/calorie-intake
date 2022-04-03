#!/bin/bash

cd $(dirname $0)

RUST_LOG=debug cargo run --bin diet-server -- \
    --listener-address [::1]:3001 \
    -s test_resources/test_secrets.toml

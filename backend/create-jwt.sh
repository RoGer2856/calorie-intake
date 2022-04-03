#!/bin/bash

cd $(dirname $0)

cargo run --bin diet-jwt-manager -- \
    create -s test_resources/test_secrets.toml \
    -r regular_user \
    -u jane \
    -c 2100

cargo run --bin diet-jwt-manager -- \
    create -s test_resources/test_secrets.toml \
    -r regular_user \
    -u john \
    -c 2100

cargo run --bin diet-jwt-manager -- \
    create -s test_resources/test_secrets.toml \
    -r admin \
    -u admin \
    -c 2100

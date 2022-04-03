#!/bin/bash

cd $(dirname $0)

cargo run --bin diet-jwt-manager -- \
    create -s test_resources/test_secrets.toml \
    -r regular_user \
    -u jane

cargo run --bin diet-jwt-manager -- \
    create -s test_resources/test_secrets.toml \
    -r regular_user \
    -u john

cargo run --bin diet-jwt-manager -- \
    create -s test_resources/test_secrets.toml \
    -r admin \
    -u admin

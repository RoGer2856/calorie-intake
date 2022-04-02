# Backend

This project is using cargo as its build system. It can be best installed with rustup. Please follow the instructions on its web page [https://rustup.rs/](https://rustup.rs/).

The project contains a server (**diet-server**) and a jwt manager (**diet-jwt-manager**).

## Testing

The project contains automatic tests. Please run `cargo test --all` to perform all tests.

## Running **diet-server**

For debug mode, run `cargo run --bin diet-server -- -h`

For release mode, run `cargo run --bin diet-server --release -- -h`

Examples
* Set log level to debug and listen on [::1]:3001: `RUST_LOG=debug cargo run --bin diet-server -- --listener-address [::1]:3001`

## Running **diet-jwt-manager**

For debug mode, run `cargo run --bin diet-jwt-manager -- -h`

For release mode, run `cargo run --bin diet-jwt-manager --release -- -h`

Examples:
* Creating a new regular user jwt for diet-jwt-manager: `cargo run --bin diet-jwt-manager -- create -s test_resources/test_secrets.toml -r regular_user -u john`
* Creating a new admin user jwt for diet-jwt-manager: `cargo run --bin diet-jwt-manager -- create -s test_resources/test_secrets.toml -r admin -u john`

## Building

The project can be built in debug and release mode.

For debug mode, run `cargo build`

For release mode, run `cargo build --release`
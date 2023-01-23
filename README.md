# Axum Postgres dynamic parameters example

An example for trying to get constructing query parameters dynamically to work with Axum.

Based on the Axum [tokio-postgres example](https://github.com/tokio-rs/axum/blob/main/examples/tokio-postgres/src/main.rs).

The solution for dynamic query params is taken from this [rust-postgres issue](https://github.com/sfackler/rust-postgres/issues/712).

__It works!__
# Axum Postgres dynamic parameters example

An example for trying to get constructing query parameters dynamically to work with Axum.

Based on the Axum [tokio-postgres example](https://github.com/tokio-rs/axum/blob/main/examples/tokio-postgres/src/main.rs).

The solution for dynamic query params is taken from this [rust-postgres issue](https://github.com/sfackler/rust-postgres/issues/712).

This is a bit tricky because the Postgres query params type is [`[&(dyn ToSql + Sync)]`](https://docs.rs/postgres/latest/postgres/struct.Client.html#method.execute), but it also has to be `Sync`.

__It works!__
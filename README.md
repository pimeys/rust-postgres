# Rust-Postgres

PostgreSQL support for Rust. This version adds a new config option for
pgbouncer, allowing prepared statement use on pgbouncer's transaction mode. If
you don't need this, you're much better off with the [original
crate](https://crates.io/crates/postgres).

## postgres [![Latest Version](https://img.shields.io/crates/v/prisma-postgres.svg)](https://crates.io/crates/prisma-postgres)

[Documentation](https://docs.rs/prisma-postgres)

A native, synchronous PostgreSQL client.

## tokio-postgres [![Latest Version](https://img.shields.io/crates/v/tokio-postgres.svg)](https://crates.io/crates/prisma-tokio-postgres)

[Documentation](https://docs.rs/prisma-tokio-postgres)

A native, asynchronous PostgreSQL client.

## postgres-types [![Latest Version](https://img.shields.io/crates/v/postgres-types.svg)](https://crates.io/crates/prisma-postgres-types)

[Documentation](https://docs.rs/prisma-postgres-types)

Conversions between Rust and Postgres types.

## postgres-native-tls [![Latest Version](https://img.shields.io/crates/v/postgres-native-tls.svg)](https://crates.io/crates/prisma-postgres-native-tls)

[Documentation](https://docs.rs/prisma-postgres-native-tls)

TLS support for postgres and tokio-postgres via native-tls.

# Running test suite

The test suite requires postgres to be running in the correct configuration. The easiest way to do this is with docker:

1. Install `docker` and `docker-compose`.
   1. On ubuntu: `sudo apt install docker.io docker-compose`.
1. Make sure your user has permissions for docker.
   1. On ubuntu: ``sudo usermod -aG docker $USER``
1. Change to top-level directory of `rust-postgres` repo.
1. Run `docker-compose up -d`.
1. Run `cargo test`.
1. Run `docker-compose stop`.

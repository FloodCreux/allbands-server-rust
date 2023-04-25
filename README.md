# All Bands Server

<br />

## Technologies

- [Rust](https://doc.rust-lang.org/book/)
- [Tokio](https://docs.rs/tokio/latest/tokio/)
- [Actix](https://actix.rs/docs)
- [Docker](https://docs.docker.com/)
- [Redis](https://redis.io/docs/)

## Overview

This is a first effort to build an API server in Rust

### Domain

Contains the `struct`s and `impl`s required to interact with the persistence layer

### Routes

Contains the presentation layer logic, in this case all of the API endpoint logic

### Telemetry

Contains the logic necessary for logging across the application

### Configuration

Contains the `struct`s for configuring and running the application

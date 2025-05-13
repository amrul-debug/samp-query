# SAMP Query

 Implementation of the SA-MP Query Mechanism in Rust.

## Overview

This project provides a complete implementation of the SA-MP Query protocol, allowing you to query SA-MP servers for information such as player lists, server rules, and detailed player information. It is designed to be a modern, reliable replacement for the original SAMP API, which is no longer maintained.

## Features

- **Complete Protocol Implementation**: Full support for all query types (information, rules, clients, detailed player data)
- **Robust Error Handling**: Comprehensive error types and proper error propagation
- **Performance Optimized**: Efficient network communication with appropriate timeouts and retries
- **Thread-Safe Design**: Proper concurrency controls for multi-threaded applications
- **Multiple Interfaces**: Library, REST API, and CLI components
- **Comprehensive Documentation**: Detailed API documentation and implementation guides

## Components

### Core Library

The core library implements the SA-MP Query protocol and provides a clean API for querying servers.

```rust
use samp_query::{Client, QueryType};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let addr: SocketAddr = "127.0.0.1:7777".parse()?;
    let client = Client::connect(addr).await?;

    // Query server information
    let info = client.query_info().await?;
    println!("Server: {}", info.hostname);
    println!("Players: {}/{}", info.players, info.max_players);
    println!("Gamemode: {}", info.gamemode);

    Ok(())
}
```

### REST API

The REST API service provides HTTP endpoints for querying servers.

```
GET /api/v1/servers/{ip}:{port}/info
GET /api/v1/servers/{ip}:{port}/rules
GET /api/v1/servers/{ip}:{port}/players
GET /api/v1/servers/{ip}:{port}/players/detailed
GET /api/v1/servers/{ip}:{port}/ping
POST /api/v1/servers/{ip}:{port}/rcon
```

### CLI Tool

The CLI tool provides a command-line interface for querying servers.

```
samp-query info 127.0.0.1:7777
samp-query rules 127.0.0.1:7777
samp-query players 127.0.0.1:7777
samp-query players-detailed 127.0.0.1:7777
samp-query ping 127.0.0.1:7777
samp-query rcon 127.0.0.1:7777 "password" "command"
```

## Installation

### From crates.io

```
cargo install samp-query
```

### From source

```
git clone https://github.com/amrul-debug/samp-query.git
cd samp-query
cargo build --release
```

## Documentation

- [API Documentation](https://docs.rs/samp-query)
- [Implementation Guide](GUIDE.md)
- [Changelog](CHANGELOG.md)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

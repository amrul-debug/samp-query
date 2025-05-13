# SAMP Query Implementation Guide

This document provides a comprehensive explanation of the SAMP Query implementation, including architecture decisions, protocol details, and usage patterns for all components of the project.

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Protocol Implementation](#protocol-implementation)
3. [Module Relationships](#module-relationships)
4. [Core Library Usage](#core-library-usage)
5. [CLI Tool Usage](#cli-tool-usage)
6. [REST API Usage](#rest-api-usage)
7. [Extension Points](#extension-points)
8. [Performance Considerations](#performance-considerations)
9. [Troubleshooting](#troubleshooting)

## Architecture Overview

The SAMP Query project is structured as a Rust workspace with three main components:

1. **Core Library (`samp-query`)**: Implements the SA-MP Query protocol and provides a high-level API for querying servers.
2. **CLI Tool (`samp-query-cli`)**: Provides a command-line interface for querying servers.
3. **REST API Service (`samp-query-api`)**: Provides a web service for querying servers via HTTP.

Each component follows a layered architecture:

### Core Library Layers

1. **Protocol Layer**: Defines the protocol constants, packet structures, and query types.
2. **Network Layer**: Handles UDP communication, timeouts, and retries.
3. **Client Layer**: Provides a high-level API for querying servers.

### CLI Tool Layers

1. **Command Layer**: Defines the command-line interface and parses arguments.
2. **Service Layer**: Handles the business logic for each command.
3. **Presentation Layer**: Formats and displays the results to the user.

### REST API Layers

1. **Routing Layer**: Defines the API endpoints and routes requests.
2. **Handler Layer**: Processes requests and invokes the core library.
3. **Response Layer**: Formats and returns the results as JSON.

This separation of concerns allows for clean abstractions and makes the codebase easier to maintain and extend. The modular design also enables users to choose the interface that best suits their needs.

## Protocol Implementation

The SA-MP Query protocol is a UDP-based protocol for querying SA-MP servers. It supports several query types, each identified by a single-character opcode:

- `i` - Information: Basic server information (hostname, player count, etc.)
- `r` - Rules: Server rules/variables
- `c` - Client List: List of players with names and scores
- `d` - Detailed Player Info: Detailed information about players
- `p` - Ping: Used to measure server latency
- `x` - RCON: Execute RCON commands on the server

### Packet Structure

Each query packet has the following structure:

1. SAMP signature (`SAMP`) - 4 bytes
2. Server IP address - 4 bytes
3. Server port - 2 bytes
4. Query opcode - 1 byte
5. Additional data (for RCON and ping queries) - variable length

Response packets have a similar structure, with the actual response data following the header.

### Implementation Details

The protocol implementation is contained in the `protocol.rs` and `packet.rs` modules. The `protocol.rs` module defines the query types and protocol constants, while the `packet.rs` module handles packet creation, validation, and parsing.

The packet parsing logic is carefully implemented to handle different data types and string encodings used by the protocol. For example, strings in the protocol can be null-terminated, length-prefixed (8-bit, 16-bit, or 32-bit length), or fixed-length.

## Module Relationships

### Core Library Modules

The core library is organized into several modules, each with a specific responsibility:

- `lib.rs`: Main entry point, re-exports public API
- `protocol.rs`: Protocol constants and query types
- `packet.rs`: Packet creation, validation, and parsing
- `client.rs`: High-level client API
- `error.rs`: Error types and handling
- `types.rs`: Data structures for query responses
- `utils.rs`: Utility functions

The relationships between these modules are as follows:

- `client.rs` depends on `packet.rs`, `protocol.rs`, `error.rs`, and `types.rs`
- `packet.rs` depends on `protocol.rs` and `error.rs`
- `types.rs` is independent of other modules
- `error.rs` is independent of other modules
- `utils.rs` is independent of other modules

### CLI Tool Modules

The CLI tool is organized into the following modules:

- `main.rs`: Entry point, command-line argument parsing
- `commands.rs`: Implementation of each command
- `output.rs`: Formatting and display of results

### REST API Modules

The REST API service is organized into the following modules:

- `main.rs`: Entry point, server setup, and routing
- `error.rs`: API-specific error handling

This organization minimizes dependencies between modules and makes the codebase easier to understand and maintain. The modular design also allows for easy extension and customization.

## Core Library Usage

### Basic Usage

The most common usage pattern is to create a client, connect to a server, and query for information:

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

### Advanced Usage

For more advanced use cases, you can customize the client configuration:

```rust
use samp_query::{Client, ClientConfig, QueryType};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a custom configuration
    let config = ClientConfig {
        timeout_ms: 2000,    // 2 seconds timeout
        max_retries: 5,      // 5 retries
    };

    // Create a client with custom configuration
    let addr: SocketAddr = "127.0.0.1:7777".parse()?;
    let client = Client::connect_with_config(addr, config).await?;

    // Query server information
    let info = client.query_info().await?;
    println!("Server: {}", info.hostname);

    Ok(())
}
```

### Error Handling

The library provides comprehensive error handling through the `Error` enum:

```rust
use samp_query::{Client, Error, QueryType};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Create a client
    let addr: SocketAddr = "127.0.0.1:7777".parse().unwrap();
    let client = match Client::connect(addr).await {
        Ok(client) => client,
        Err(Error::Connect(e)) => {
            eprintln!("Failed to connect to server: {}", e);
            return;
        }
        Err(Error::Timeout) => {
            eprintln!("Connection timed out");
            return;
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    // Query server information
    match client.query_info().await {
        Ok(info) => {
            println!("Server: {}", info.hostname);
        }
        Err(e) => {
            eprintln!("Error querying server: {}", e);
        }
    }
}
```

## CLI Tool Usage

The CLI tool provides a command-line interface for querying SA-MP servers. It supports all the query types provided by the core library.

### Command Structure

The CLI tool uses the following command structure:

```
samp-query-cli [OPTIONS] <COMMAND>
```

#### Global Options

- `-t, --timeout <TIMEOUT>`: Timeout in milliseconds (default: 1000)
- `-r, --retries <RETRIES>`: Number of retries (default: 3)
- `-h, --help`: Print help information
- `-V, --version`: Print version information

#### Available Commands

- `info`: Query server information
- `rules`: Query server rules
- `players`: Query player list
- `players-detailed`: Query detailed player information
- `ping`: Query server ping
- `rcon`: Execute RCON command
- `help`: Print help information for a specific command

### Examples

#### Query Server Information

```bash
samp-query-cli info 127.0.0.1:7777
```

Output:
```
Server Information
Hostname: LastPride Roleplay || Reunion
Players: 1/100
Gamemode: LP:RP v1.0.5
Language: English
Password: No
```

#### Query Server Rules

```bash
samp-query-cli rules 127.0.0.1:7777
```

Output:
```
Server Rules
+-----------+-------------+
| Rule      | Value       |
+-----------+-------------+
| lagcomp   | On          |
+-----------+-------------+
| version   | 0.3.DL-R1   |
+-----------+-------------+
| worldtime | 12:40       |
+-----------+-------------+
| mapname   | San Andreas |
+-----------+-------------+
| artwork   | No          |
+-----------+-------------+
| weather   | 10          |
+-----------+-------------+
| weburl    | coming-soon |
+-----------+-------------+
```

#### Query Player List

```bash
samp-query-cli players 127.0.0.1:7777
```

Output:
```
Players (1)
+--------------+-------+
| Name         | Score |
+--------------+-------+
| Theo_Douglas | 5     |
+--------------+-------+
```

#### Query Detailed Player Information

```bash
samp-query-cli players-detailed 127.0.0.1:7777
```

Output:
```
Players (1)
+----+--------------+-------+------+
| ID | Name         | Score | Ping |
+----+--------------+-------+------+
| 0  | Theo_Douglas | 5     | 26   |
+----+--------------+-------+------+
```

#### Query Server Ping

```bash
samp-query-cli ping 127.0.0.1:7777
```

Output:
```
Ping: 3 ms
```

#### Execute RCON Command

```bash
samp-query-cli rcon 127.0.0.1:7777 "password" "players"
```

Note: This command requires a valid RCON password.

## REST API Usage

The REST API service provides a web interface for querying SA-MP servers. It exposes several endpoints that correspond to the different query types.

### API Endpoints

- `GET /`: API root, returns a simple message
- `GET /api/v1/servers/:address/info`: Query server information
- `GET /api/v1/servers/:address/rules`: Query server rules
- `GET /api/v1/servers/:address/players`: Query player list
- `GET /api/v1/servers/:address/players/detailed`: Query detailed player information
- `GET /api/v1/servers/:address/ping`: Query server ping

### Examples

#### Query Server Information

```
GET http://localhost:3001/api/v1/servers/127.0.0.1:7777/info
```

Response:
```json
{
  "password": false,
  "players": 1,
  "max_players": 100,
  "hostname": "LastPride Roleplay || Reunion",
  "gamemode": "LP:RP v1.0.5",
  "language": "English"
}
```

#### Query Server Rules

```
GET http://localhost:3001/api/v1/servers/127.0.0.1:7777/rules
```

Response:
```json
{
  "rules": {
    "weburl": "coming-soon",
    "worldtime": "12:35",
    "mapname": "San Andreas",
    "lagcomp": "On",
    "version": "0.3.DL-R1",
    "artwork": "No",
    "weather": "10"
  }
}
```

#### Query Player List

```
GET http://localhost:3001/api/v1/servers/127.0.0.1:7777/players
```

Response:
```json
{
  "players": [
    {
      "name": "Theo_Douglas",
      "score": 5
    }
  ]
}
```

#### Query Detailed Player Information

```
GET http://localhost:3001/api/v1/servers/127.0.0.1:7777/players/detailed
```

Response:
```json
{
  "players": [
    {
      "id": 0,
      "name": "Theo_Douglas",
      "score": 5,
      "ping": 32
    }
  ]
}
```

#### Query Server Ping

```
GET http://localhost:3001/api/v1/servers/127.0.0.1:7777/ping
```

Response:
```json
{
  "ping_ms": 0
}
```

### Error Handling

The API returns appropriate HTTP status codes and error messages in case of failures:

- `400 Bad Request`: Invalid server address
- `404 Not Found`: Server not found or unreachable
- `500 Internal Server Error`: Server error
- `504 Gateway Timeout`: Server did not respond in time
```

## Extension Points

The library is designed to be extensible in several ways:

### Custom Query Types

You can extend the `QueryType` enum to add support for custom query types:

```rust
use samp_query::protocol::QueryType;

// Add a custom query type
enum CustomQueryType {
    Standard(QueryType),
    Custom(u8),
}

impl CustomQueryType {
    fn opcode(&self) -> u8 {
        match self {
            CustomQueryType::Standard(query_type) => query_type.opcode(),
            CustomQueryType::Custom(opcode) => *opcode,
        }
    }
}
```

### Custom Response Parsers

You can implement custom response parsers for specific query types:

```rust
use samp_query::packet::Packet;
use samp_query::error::Result;
use std::io::Cursor;

fn parse_custom_response(packet: &Packet) -> Result<CustomResponse> {
    let data = packet.parse_response(QueryType::Information)?;
    let mut cursor = Cursor::new(&data);

    // Parse the custom response
    // ...

    Ok(CustomResponse { /* ... */ })
}
```

## Performance Considerations

The library is designed with performance in mind:

- **Minimal Allocations**: The packet handling code minimizes allocations by using pre-allocated buffers.
- **Efficient Parsing**: The packet parsing code is optimized for efficiency.
- **Timeout Handling**: The client handles timeouts gracefully and supports retries.
- **Concurrency**: The client is designed to be thread-safe and can be used in concurrent applications.

For high-performance applications, consider the following:

- Use a connection pool to reuse clients
- Adjust the timeout and retry settings based on your network conditions
- Use the async API to handle multiple queries concurrently

## Troubleshooting

### Common Issues

#### Connection Timeouts

If you're experiencing connection timeouts, try the following:

1. Increase the timeout value:
   ```rust
   let config = ClientConfig {
       timeout_ms: 5000,    // 5 seconds timeout
       max_retries: 3,
   };
   ```

2. Verify that the server is running and accessible:
   ```bash
   ping 127.0.0.1
   ```

3. Check if the server is using a non-standard port:
   ```bash
   samp-query-cli info 127.0.0.1:7777
   ```

#### Invalid Responses

If you're receiving invalid response errors, try the following:

1. Verify that the server is a valid SA-MP server:
   ```bash
   samp-query-cli info 127.0.0.1:7777
   ```

2. Check if the server is using a modified version of the SA-MP server software.

3. Increase the number of retries:
   ```rust
   let config = ClientConfig {
       timeout_ms: 1000,
       max_retries: 5,    // 5 retries
   };
   ```

#### API Port Conflicts

If the API service fails to start due to port conflicts, try the following:

1. Change the port in the `main.rs` file:
   ```rust
   let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
   ```

2. Check if another process is using the same port:
   ```bash
   netstat -ano | findstr :3000
   ```

3. Kill the process using the port:
   ```bash
   taskkill /PID <PID> /F
   ```

### Debugging

For more detailed debugging, you can enable logging:

```rust
// Initialize tracing
tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();
```

This will provide more detailed information about the queries being sent and the responses being received.

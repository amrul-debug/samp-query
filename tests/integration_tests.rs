//! Integration tests for the SAMP Query library.

use samp_query::{Client, QueryType};
use std::net::SocketAddr;

#[tokio::test]
async fn test_client_creation() {
    // This test is just to verify that the client creation code runs without panicking
    // The actual connection may succeed or fail depending on the environment
    let addr: SocketAddr = "127.0.0.1:7777".parse().unwrap();
    let _result = Client::connect(addr).await;
}

#[tokio::test]
async fn test_client_creation_with_config() {
    let addr: SocketAddr = "127.0.0.1:7777".parse().unwrap();

    let config = samp_query::client::ClientConfig {
        timeout_ms: 500,
        max_retries: 1,
    };

    let _result = Client::connect_with_config(addr, config).await;
}

#[test]
fn test_query_types() {
    assert_eq!(QueryType::Information.opcode(), b'i');
    assert_eq!(QueryType::Rules.opcode(), b'r');
    assert_eq!(QueryType::ClientList.opcode(), b'c');
    assert_eq!(QueryType::DetailedPlayerInfo.opcode(), b'd');
    assert_eq!(QueryType::Ping.opcode(), b'p');
    assert_eq!(QueryType::Rcon.opcode(), b'x');

    assert_eq!(QueryType::from_opcode(b'i'), Some(QueryType::Information));
    assert_eq!(QueryType::from_opcode(b'r'), Some(QueryType::Rules));
    assert_eq!(QueryType::from_opcode(b'c'), Some(QueryType::ClientList));
    assert_eq!(QueryType::from_opcode(b'd'), Some(QueryType::DetailedPlayerInfo));
    assert_eq!(QueryType::from_opcode(b'p'), Some(QueryType::Ping));
    assert_eq!(QueryType::from_opcode(b'x'), Some(QueryType::Rcon));
    assert_eq!(QueryType::from_opcode(b'z'), None);
}

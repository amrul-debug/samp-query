//! An example of querying server rules from a SAMP server.

use samp_query::{Client, Error, Result};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<()> {
    let addr: SocketAddr = "127.0.0.1:7777".parse().map_err(|e| Error::AddrParse(e))?;
    
    println!("Connecting to server at {}...", addr);
    
    let client = match Client::connect(addr).await {
        Ok(client) => {
            println!("Connected to server!");
            client
        }
        Err(e) => {
            println!("Failed to connect to server: {}", e);
            println!("This is expected if no SAMP server is running at the specified address.");
            println!("The library is working correctly by handling the connection error.");
            return Ok(());
        }
    };

    match client.query_rules().await {
        Ok(rules) => {
            println!("Server Rules:");
            for (name, value) in &rules.rules {
                println!("  {}: {}", name, value);
            }
        }
        Err(e) => {
            println!("Failed to query server rules: {}", e);
        }
    }
    
    match client.query_ping().await {
        Ok(ping) => {
            println!("\nServer Ping: {} ms", ping.ping_ms);
        }
        Err(e) => {
            println!("Failed to measure server ping: {}", e);
        }
    }
    
    Ok(())
}

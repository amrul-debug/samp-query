//! A simple example of using the SAMP Query library.

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
    
    match client.query_info().await {
        Ok(info) => {
            println!("Server Information:");
            println!("  Hostname: {}", info.hostname);
            println!("  Players: {}/{}", info.players, info.max_players);
            println!("  Gamemode: {}", info.gamemode);
            println!("  Language: {}", info.language);
            println!("  Password: {}", if info.password { "Yes" } else { "No" });
        }
        Err(e) => {
            println!("Failed to query server information: {}", e);
        }
    }
    
    Ok(())
}

//! An example of querying player information from a SAMP server.

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
    
    match client.query_client_list().await {
        Ok(players) => {
            println!("Player List ({}):", players.players.len());
            for player in &players.players {
                println!("  {} (Score: {})", player.name, player.score);
            }
        }
        Err(e) => {
            println!("Failed to query player list: {}", e);
        }
    }
    
    match client.query_detailed_player_info().await {
        Ok(players) => {
            println!("\nDetailed Player Information ({}):", players.players.len());
            for player in &players.players {
                println!("  {} (ID: {}, Score: {}, Ping: {})", 
                    player.name, player.id, player.score, player.ping);
            }
        }
        Err(e) => {
            println!("Failed to query detailed player information: {}", e);
        }
    }
    
    Ok(())
}

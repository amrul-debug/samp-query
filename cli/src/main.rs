//! Command-line interface for the SAMP Query library.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::Colorize;
use samp_query::{Client, ClientConfig};
use std::net::SocketAddr;

mod output;
use output::{format_detailed_player_list, format_player_list, format_rules, format_server_info};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    ///subcommand
    #[command(subcommand)]
    command: Commands,

    ///timeout in milliseconds
    #[arg(short, long, default_value = "1000")]
    timeout: u64,

    ///number of retries
    #[arg(short, long, default_value = "3")]
    retries: usize,
}

#[derive(Subcommand)]
enum Commands {
    ///query server information
    Info {
        ///server address (IP:PORT)
        address: String,
    },
    ///query server rules
    Rules {
        ///server address (IP:PORT)
        address: String,
    },
    ///query player list
    Players {
        ///server address (IP:PORT)
        address: String,
    },
    ///qery detailed player information
    PlayersDetailed {
        ///server address (IP:PORT)
        address: String,
    },
    ///query server ping
    Ping {
        ///server address (IP:PORT)
        address: String,
    },
    ///execute RCON command
    Rcon {
        ///server address (IP:PORT)
        address: String,
        ///RCON password
        password: String,
        ///RCON command
        command: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let cli = Cli::parse();

    let config = ClientConfig {
        timeout_ms: cli.timeout,
        max_retries: cli.retries,
    };

    match cli.command {
        Commands::Info { address } => {
            let addr: SocketAddr = address.parse().context("Invalid server address")?;
            let client = Client::connect_with_config(addr, config)
                .await
                .context("Failed to connect to server")?;

            let info = client.query_info().await.context("Failed to query server info")?;
            println!("{}", format_server_info(&info));
        }
        Commands::Rules { address } => {
            let addr: SocketAddr = address.parse().context("Invalid server address")?;
            let client = Client::connect_with_config(addr, config)
                .await
                .context("Failed to connect to server")?;

            let rules = client.query_rules().await.context("Failed to query server rules")?;
            println!("{}", format_rules(&rules));
        }
        Commands::Players { address } => {
            let addr: SocketAddr = address.parse().context("Invalid server address")?;
            let client = Client::connect_with_config(addr, config)
                .await
                .context("Failed to connect to server")?;

            let players = client
                .query_client_list()
                .await
                .context("Failed to query player list")?;
            println!("{}", format_player_list(&players));
        }
        Commands::PlayersDetailed { address } => {
            let addr: SocketAddr = address.parse().context("Invalid server address")?;
            let client = Client::connect_with_config(addr, config)
                .await
                .context("Failed to connect to server")?;

            let players = client
                .query_detailed_player_info()
                .await
                .context("Failed to query detailed player info")?;
            println!("{}", format_detailed_player_list(&players));
        }
        Commands::Ping { address } => {
            let addr: SocketAddr = address.parse().context("Invalid server address")?;
            let client = Client::connect_with_config(addr, config)
                .await
                .context("Failed to connect to server")?;

            let ping = client.query_ping().await.context("Failed to query server ping")?;
            println!("{}", format!("Ping: {} ms", ping.ping_ms).green());
        }
        Commands::Rcon {
            address,
            password,
            command,
        } => {
            let addr: SocketAddr = address.parse().context("Invalid server address")?;
            let client = Client::connect_with_config(addr, config)
                .await
                .context("Failed to connect to server")?;

            let response = client
                .rcon_command(&password, &command)
                .await
                .context("Failed to execute RCON command")?;
            println!("{}", response.message);
        }
    }

    Ok(())
}

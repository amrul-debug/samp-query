//! Data types for the SAMP Query protocol.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    /// Whether the server has a password.
    pub password: bool,
    /// Current number of players on the server.
    pub players: u16,
    /// Maximum number of players the server can hold.
    pub max_players: u16,
    /// Server hostname.
    pub hostname: String,
    /// Current gamemode.
    pub gamemode: String,
    /// Server language.
    pub language: String,
}

impl fmt::Display for ServerInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Server: {}", self.hostname)?;
        writeln!(f, "Players: {}/{}", self.players, self.max_players)?;
        writeln!(f, "Gamemode: {}", self.gamemode)?;
        writeln!(f, "Language: {}", self.language)?;
        writeln!(f, "Password: {}", if self.password { "Yes" } else { "No" })?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerRules {
    pub rules: HashMap<String, String>,
}

impl fmt::Display for ServerRules {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Server Rules:")?;
        for (name, value) in &self.rules {
            writeln!(f, "  {}: {}", name, value)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    /// Players nickname.
    pub name: String,
    /// Players score.
    pub score: i32,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (Score: {})", self.name, self.score)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedPlayer {
    /// Players ID.
    pub id: u8,
    /// Players nickname.
    pub name: String,
    /// Players score.
    pub score: i32,
    /// Players ping.
    pub ping: u32,
}

impl fmt::Display for DetailedPlayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (ID: {}, Score: {}, Ping: {})",
            self.name, self.id, self.score, self.ping
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerList {
    /// List of players.
    pub players: Vec<Player>,
}

impl fmt::Display for PlayerList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Players ({}):", self.players.len())?;
        for player in &self.players {
            writeln!(f, "  {}", player)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedPlayerList {
    /// List of detailed players.
    pub players: Vec<DetailedPlayer>,
}

impl fmt::Display for DetailedPlayerList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Players ({}):", self.players.len())?;
        for player in &self.players {
            writeln!(f, "  {}", player)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RconResponse {
    /// Response message.
    pub message: String,
}

impl fmt::Display for RconResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingInfo {
    /// Ping time in milliseconds.
    pub ping_ms: u64,
}

impl fmt::Display for PingInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ping: {} ms", self.ping_ms)
    }
}

//! Protocol implementation for the SAMP Query mechanism.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QueryType {
    /// Information query (opcode 'i').
    /// Returns basic server information like hostname, player count, etc.
    Information,
    
    /// Rules query (opcode 'r').
    /// Returns server rules/variables.
    Rules,
    
    /// Client list query (opcode 'c').
    /// Returns a list of players with their names and scores.
    ClientList,
    
    /// Detailed player information query (opcode 'd').
    /// Returns detailed information about players including IDs and pings.
    DetailedPlayerInfo,
    
    /// Ping query (opcode 'p').
    /// Used to measure the server's ping.
    Ping,
    
    /// RCON command query (opcode 'x').
    /// Executes an RCON command on the server.
    Rcon,
}

impl QueryType {
    pub fn opcode(&self) -> u8 {
        match self {
            QueryType::Information => b'i',
            QueryType::Rules => b'r',
            QueryType::ClientList => b'c',
            QueryType::DetailedPlayerInfo => b'd',
            QueryType::Ping => b'p',
            QueryType::Rcon => b'x',
        }
    }
    
    pub fn opcode_char(&self) -> char {
        match self {
            QueryType::Information => 'i',
            QueryType::Rules => 'r',
            QueryType::ClientList => 'c',
            QueryType::DetailedPlayerInfo => 'd',
            QueryType::Ping => 'p',
            QueryType::Rcon => 'x',
        }
    }
    
    pub fn from_opcode(opcode: u8) -> Option<Self> {
        match opcode {
            b'i' => Some(QueryType::Information),
            b'r' => Some(QueryType::Rules),
            b'c' => Some(QueryType::ClientList),
            b'd' => Some(QueryType::DetailedPlayerInfo),
            b'p' => Some(QueryType::Ping),
            b'x' => Some(QueryType::Rcon),
            _ => None,
        }
    }
}

impl fmt::Display for QueryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QueryType::Information => write!(f, "Information"),
            QueryType::Rules => write!(f, "Rules"),
            QueryType::ClientList => write!(f, "Client List"),
            QueryType::DetailedPlayerInfo => write!(f, "Detailed Player Info"),
            QueryType::Ping => write!(f, "Ping"),
            QueryType::Rcon => write!(f, "RCON"),
        }
    }
}

pub mod constants {
    /// The SAMP packet signature.
    pub const SAMP_SIGNATURE: &[u8] = b"SAMP";
    
    /// The size of the packet header.
    pub const HEADER_SIZE: usize = 11;
    
    /// The maximum size of a packet.
    pub const MAX_PACKET_SIZE: usize = 2048;
    
    /// The default timeout for queries in milliseconds.
    pub const DEFAULT_TIMEOUT_MS: u64 = 1000;
    
    /// The maximum number of retries for a query.
    pub const MAX_RETRIES: usize = 3;
}

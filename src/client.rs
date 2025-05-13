//! Client implementation for the SAMP Query protocol.

use crate::error::{Error, Result};
use crate::packet::{utils as packet_utils, Packet};
use crate::protocol::{constants, QueryType};
use crate::types::*;
use bytes::Buf;
use std::collections::HashMap;
use std::io::Cursor;
use std::net::SocketAddr;
use std::time::{Duration, Instant};
use tokio::net::UdpSocket;
use tokio::time::timeout;

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub timeout_ms: u64,
    pub max_retries: usize,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            timeout_ms: constants::DEFAULT_TIMEOUT_MS,
            max_retries: constants::MAX_RETRIES,
        }
    }
}

#[derive(Debug)]
pub struct Client {
    server_addr: SocketAddr,
    socket: UdpSocket,
    config: ClientConfig,
}

impl Client {
    pub async fn connect(server_addr: SocketAddr) -> Result<Self> {
        Self::connect_with_config(server_addr, ClientConfig::default()).await
    }

    pub async fn connect_with_config(
        server_addr: SocketAddr,
        config: ClientConfig,
    ) -> Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0").await.map_err(Error::Bind)?;

        socket
            .connect(server_addr)
            .await
            .map_err(Error::Connect)?;

        Ok(Self {
            server_addr,
            socket,
            config,
        })
    }

    async fn send_query(&self, packet: &Packet) -> Result<Vec<u8>> {
        let mut retries = 0;
        let timeout_duration = Duration::from_millis(self.config.timeout_ms);

        while retries < self.config.max_retries {
            self.socket
                .send(packet.as_bytes())
                .await
                .map_err(Error::Send)?;

            let mut buf = vec![0u8; constants::MAX_PACKET_SIZE];
            match timeout(timeout_duration, self.socket.recv(&mut buf)).await {
                Ok(Ok(size)) => {
                    buf.truncate(size);
                    return Ok(buf);
                }
                Ok(Err(e)) => return Err(Error::Receive(e)),
                Err(_) => {
                    retries += 1;
                    continue;
                }
            }
        }

        Err(Error::Timeout)
    }

    pub async fn query_info(&self) -> Result<ServerInfo> {
        let packet = Packet::create_query(self.server_addr, QueryType::Information)?;
        let response = self.send_query(&packet).await?;
        let response_packet = Packet::from_bytes(&response);
        let data = response_packet.parse_response(QueryType::Information)?;

        let mut cursor = Cursor::new(&data);

        let password = cursor.get_u8() != 0;
        let players = cursor.get_u16_le();
        let max_players = cursor.get_u16_le();

        let hostname = packet_utils::read_length_prefixed_string_32(&mut cursor)?;
        let gamemode = packet_utils::read_length_prefixed_string_32(&mut cursor)?;
        let language = packet_utils::read_length_prefixed_string_32(&mut cursor)?;

        Ok(ServerInfo {
            password,
            players,
            max_players,
            hostname,
            gamemode,
            language,
        })
    }

    pub async fn query_rules(&self) -> Result<ServerRules> {
        let packet = Packet::create_query(self.server_addr, QueryType::Rules)?;
        let response = self.send_query(&packet).await?;
        let response_packet = Packet::from_bytes(&response);
        let data = response_packet.parse_response(QueryType::Rules)?;

        let mut cursor = Cursor::new(&data);

        let rule_count = cursor.get_u16_le() as usize;
        let mut rules = HashMap::with_capacity(rule_count);

        for _ in 0..rule_count {
            let name = packet_utils::read_length_prefixed_string(&mut cursor)?;
            let value = packet_utils::read_length_prefixed_string(&mut cursor)?;
            rules.insert(name, value);
        }

        Ok(ServerRules { rules })
    }

    pub async fn query_client_list(&self) -> Result<PlayerList> {
        let packet = Packet::create_query(self.server_addr, QueryType::ClientList)?;
        let response = self.send_query(&packet).await?;
        let response_packet = Packet::from_bytes(&response);
        let data = response_packet.parse_response(QueryType::ClientList)?;

        let mut cursor = Cursor::new(&data);

        let player_count = cursor.get_u16_le() as usize;
        let mut players = Vec::with_capacity(player_count);

        for _ in 0..player_count {
            let name = packet_utils::read_length_prefixed_string(&mut cursor)?;
            let score = cursor.get_i32_le();

            players.push(Player { name, score });
        }

        Ok(PlayerList { players })
    }

    pub async fn query_detailed_player_info(&self) -> Result<DetailedPlayerList> {
        let packet = Packet::create_query(self.server_addr, QueryType::DetailedPlayerInfo)?;
        let response = self.send_query(&packet).await?;
        let response_packet = Packet::from_bytes(&response);
        let data = response_packet.parse_response(QueryType::DetailedPlayerInfo)?;

        let mut cursor = Cursor::new(&data);

        let player_count = cursor.get_u16_le() as usize;
        let mut players = Vec::with_capacity(player_count);

        for _ in 0..player_count {
            let id = cursor.get_u8();
            let name = packet_utils::read_length_prefixed_string(&mut cursor)?;
            let score = cursor.get_i32_le();
            let ping = cursor.get_u32_le();

            players.push(DetailedPlayer {
                id,
                name,
                score,
                ping,
            });
        }

        Ok(DetailedPlayerList { players })
    }

    pub async fn query_ping(&self) -> Result<PingInfo> {
        let (packet, random_bytes) = Packet::create_ping_query(self.server_addr)?;

        let start = Instant::now();
        let response = self.send_query(&packet).await?;
        let elapsed = start.elapsed();

        let response_packet = Packet::from_bytes(&response);
        let data = response_packet.parse_response(QueryType::Ping)?;

        if data.len() < 4 || &data[0..4] != &random_bytes {
            return Err(Error::InvalidResponse(
                "Invalid ping response".to_string(),
            ));
        }

        Ok(PingInfo {
            ping_ms: elapsed.as_millis() as u64,
        })
    }

    pub async fn rcon_command(&self, password: &str, command: &str) -> Result<RconResponse> {
        let packet = Packet::create_rcon_query(self.server_addr, password, command)?;
        let response = self.send_query(&packet).await?;
        let response_packet = Packet::from_bytes(&response);
        let data = response_packet.parse_response(QueryType::Rcon)?;

        if data.is_empty() {
            return Err(Error::RconAuthFailed);
        }

        let message = String::from_utf8(data).map_err(Error::from)?;

        Ok(RconResponse { message })
    }

    pub async fn query(&self, query_type: QueryType) -> Result<Box<dyn std::any::Any>> {
        match query_type {
            QueryType::Information => {
                let info = self.query_info().await?;
                Ok(Box::new(info))
            }
            QueryType::Rules => {
                let rules = self.query_rules().await?;
                Ok(Box::new(rules))
            }
            QueryType::ClientList => {
                let clients = self.query_client_list().await?;
                Ok(Box::new(clients))
            }
            QueryType::DetailedPlayerInfo => {
                let players = self.query_detailed_player_info().await?;
                Ok(Box::new(players))
            }
            QueryType::Ping => {
                let ping = self.query_ping().await?;
                Ok(Box::new(ping))
            }
            QueryType::Rcon => {
                return Err(Error::InvalidQueryType(
                    "RCON queries require a password and command".to_string(),
                ));
            }
        }
    }
}

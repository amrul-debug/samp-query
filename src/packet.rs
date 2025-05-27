//! Packet handling for the SAMP Query protocol.

use crate::error::{Error, Result};
use crate::protocol::{constants, QueryType};
use bytes::{BufMut, BytesMut};
use rand::Rng;
use std::net::{IpAddr, SocketAddr};

#[derive(Debug)]
pub struct Packet {
    /// raw packet data.
    data: BytesMut,
}

impl Packet {
    pub fn new() -> Self {
        Self {
            data: BytesMut::with_capacity(constants::MAX_PACKET_SIZE),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: BytesMut::with_capacity(capacity),
        }
    }

    pub fn from_bytes(data: &[u8]) -> Self {
        let mut packet = Self::with_capacity(data.len());
        packet.data.extend_from_slice(data);
        packet
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn create_query(server_addr: SocketAddr, query_type: QueryType) -> Result<Self> {
        let mut packet = Self::new();

        packet.data.extend_from_slice(constants::SAMP_SIGNATURE);

        match server_addr.ip() {
            IpAddr::V4(ipv4) => {
                for octet in ipv4.octets().iter() {
                    packet.data.put_u8(*octet);
                }
            }
            IpAddr::V6(_) => {
                return Err(Error::InvalidResponse(
                    "IPv6 addresses are not supported".to_string(),
                ));
            }
        }

        packet.data.put_u8((server_addr.port() & 0xFF) as u8);
        packet.data.put_u8(((server_addr.port() >> 8) & 0xFF) as u8);

        packet.data.put_u8(query_type.opcode());

        Ok(packet)
    }

    pub fn create_rcon_query(
        server_addr: SocketAddr,
        password: &str,
        command: &str,
    ) -> Result<Self> {
        if password.len() > 255 {
            return Err(Error::InvalidResponse(
                "RCON password too long (max 255 characters)".to_string(),
            ));
        }
        
        if command.len() > 1024 {
            return Err(Error::InvalidResponse(
                "RCON command too long (max 1024 characters)".to_string(),
            ));
        }
        
        let mut packet = Self::create_query(server_addr, QueryType::Rcon)?;

        packet.data.put_u16_le(password.len() as u16);
        packet.data.extend_from_slice(password.as_bytes());

        packet.data.put_u16_le(command.len() as u16);
        packet.data.extend_from_slice(command.as_bytes());

        Ok(packet)
    }

    pub fn create_ping_query(server_addr: SocketAddr) -> Result<(Self, [u8; 4])> {
        let mut packet = Self::create_query(server_addr, QueryType::Ping)?;

        let mut rng = rand::thread_rng();
        let random_bytes: [u8; 4] = rng.gen();

        packet.data.extend_from_slice(&random_bytes);

        Ok((packet, random_bytes))
    }

    pub fn validate_response(&self) -> Result<()> {
        if self.data.len() < constants::HEADER_SIZE {
            return Err(Error::InvalidResponse(
                "Response packet is too short".to_string(),
            ));
        }

        if &self.data[0..4] != constants::SAMP_SIGNATURE {
            return Err(Error::InvalidResponse(
                "Invalid SAMP signature in response".to_string(),
            ));
        }

        Ok(())
    }

    pub fn parse_response(&self, _query_type: QueryType) -> Result<Vec<u8>> {
        self.validate_response()?;

        Ok(self.data[constants::HEADER_SIZE..].to_vec())
    }
}

pub mod utils {
    use super::*;
    use bytes::Buf;
    use std::io::{Cursor, Read};

    pub fn read_string<B: AsRef<[u8]>>(cursor: &mut Cursor<B>) -> Result<String> {
        let mut bytes = Vec::new();
        let mut byte = [0u8; 1];

        while cursor.read_exact(&mut byte).is_ok() {
            if byte[0] == 0 {
                break;
            }
            bytes.push(byte[0]);
        }

        String::from_utf8(bytes).map_err(Error::from)
    }

    pub fn read_length_prefixed_string<B: AsRef<[u8]>>(cursor: &mut Cursor<B>) -> Result<String>
    where
        Cursor<B>: Buf,
    {
        let length = cursor.get_u8() as usize;
        
        if length > constants::MAX_PACKET_SIZE {
            return Err(Error::InvalidResponse(
                "String length exceeds maximum packet size".to_string(),
            ));
        }
        
        let mut bytes = vec![0u8; length];
        cursor.read_exact(&mut bytes)?;

        String::from_utf8(bytes).map_err(Error::from)
    }

    pub fn read_length_prefixed_string_16<B: AsRef<[u8]>>(cursor: &mut Cursor<B>) -> Result<String>
    where
        Cursor<B>: Buf,
    {
        let length = cursor.get_u16_le() as usize;
        
        if length > constants::MAX_PACKET_SIZE {
            return Err(Error::InvalidResponse(
                "String length exceeds maximum packet size".to_string(),
            ));
        }
        
        let mut bytes = vec![0u8; length];
        cursor.read_exact(&mut bytes)?;

        String::from_utf8(bytes).map_err(Error::from)
    }

    pub fn read_length_prefixed_string_32<B: AsRef<[u8]>>(cursor: &mut Cursor<B>) -> Result<String>
    where
        Cursor<B>: Buf,
    {
        let length = cursor.get_u32_le() as usize;
        
        if length > constants::MAX_PACKET_SIZE {
            return Err(Error::InvalidResponse(
                "String length exceeds maximum packet size".to_string(),
            ));
        }
        
        let mut bytes = vec![0u8; length];
        cursor.read_exact(&mut bytes)?;

        String::from_utf8(bytes).map_err(Error::from)
    }
}

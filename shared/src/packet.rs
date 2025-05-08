use std::io;
use tokio::net::TcpStream;
use async_trait::async_trait;
use bincode::config::Configuration;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use uuid::Uuid;

/// Packets sent from the Client, to the Server. (Serverbound)
#[derive(Deserialize, Serialize)]
pub enum ServerboundPacket {
    // TODO: Add RSA handshake.
    Login {
        username: String,
    },
    Message {
        token: Uuid,
        message: String,
    },
    Heartbeat {
        token: String,
    },
    Disconnect {
        token: String,
    },
}

/// Packets sent from the Server, to the Client. (Clientbound)
#[derive(Serialize, Deserialize)]
pub enum ClientboundPacket {
    // TODO: Add RSA Handshake response.
    Token {
        token: Uuid,
    },
    Message {
        msg: String,
    },
    HearbeatReq,
    Kicked {
        reason: String,
    },
}

#[derive(Deserialize, Serialize)]
pub enum Packet {
    Clientbound(ClientboundPacket),
    Serverbound(ServerboundPacket),
}

pub enum PacketRecieveError {
    InvalidLength,
    PacketParseError,
}

#[async_trait]
pub trait PacketHandler {
    async fn send_packet(&self, packet: Packet);
    async fn recieve_packet(&self) -> Result<Packet, PacketRecieveError>;
}

impl PacketHandler for TcpStream {
    async fn send_packet(&mut self, packet: Packet,) -> io::Result<()> {
        // TODO: Do error handling instead of unwrapping
        let bytes = bincode::serde::encode_to_vec(packet, Configuration::default()).unwrap();
        let len = bytes.len() as u32;

        self.write_all(&len.to_be_bytes()).await?;
        self.write_all(&bytes).await?;

        Ok(())
    }

    // TODO: Fix edge cases for reading and length
    async fn recieve_packet(&mut self) -> io::Result<Packet> {
        let mut len_buf = [0u8; 4]; // u32, since 4 * 4 * 4 * 4 = 32
        self.read(&mut len_buf).await?;
        let len = u32::from_be_bytes(len_buf);

        let mut packet_buf = vec![0u8; len as usize];
        self.read(&mut packet_buf).await?;

        // TODO: Do error handling instead of unwrapping
        let packet = bincode::serde::decode_from_slice::<Packet, Configuration>(&packet_buf, Configuration::default()).unwrap();

        Ok(packet.0)
    }
}
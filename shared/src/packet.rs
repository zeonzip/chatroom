// todos are getting fixed later, just want to make the server to work (generally) first.
// won't say no to PR's fixing todos tho!

use std::io;
use tokio::net::TcpStream;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// Packets sent from the Client, to the Server. (Serverbound)
#[derive(Deserialize, Serialize)]
pub enum ServerboundPacket {
    // TODO: Add RSA handshake.
    Login {
        username: String,
    },
    // no username bundled for more customization on the server.
    Message {
        token: String,
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
        token: String,
    },
    Message {
        msg: String,
    },
    HeartbeatReq,
    Kicked {
        reason: String,
    },
}

#[derive(Deserialize, Serialize)]
pub enum Packet {
    Clientbound(ClientboundPacket),
    Serverbound(ServerboundPacket),
}

#[async_trait]
pub trait PacketHandler {
    async fn send_packet(&mut self, packet: Packet) -> io::Result<()>;
    async fn receive_packet(&mut self) -> io::Result<Packet>;
}

#[async_trait]
impl PacketHandler for TcpStream {
    async fn send_packet(&mut self, packet: Packet) -> io::Result<()> {
        // TODO: Do error handling instead of unwrapping
        let bytes = bincode::serialize(&packet).unwrap();
        let len = bytes.len() as u32;

        self.write_all(&len.to_be_bytes()).await?;
        self.write_all(&bytes).await?;

        Ok(())
    }

    // TODO: Fix edge cases for reading and length
    async fn receive_packet(&mut self) -> io::Result<Packet> {
        let mut len_buf = [0u8; 4]; // u32, since 4 * 4 * 4 * 4 = 32
        self.read_exact(&mut len_buf).await?;
        let len = u32::from_be_bytes(len_buf);

        let mut packet_buf = vec![0u8; len as usize];
        self.read_exact(&mut packet_buf).await?;

        // TODO: Do error handling instead of unwrapping
        let packet: Packet = bincode::deserialize(&packet_buf).unwrap();

        Ok(packet)
    }
}
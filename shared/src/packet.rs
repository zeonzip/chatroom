use serde::{Deserialize, Serialize};

/// Packets sent from the Client, to the Server. (Serverbound)
#[derive(Deserialize, Serialize)]
pub enum ServerboundPacket {
    // TODO: Add RSA handshake.
    Login {
        username: String,
    },
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
enum ClientboundPacket {
    // TODO: Add RSA Handshake response.
    Token {
        token: String,
    },
    Message {
        msg: String,
    },
    HearbeatReq,
    Kicked {
        reason: String,
    },
}

enum Packet {
    Clientbound(ClientboundPacket),
    Serverbound(ServerboundPacket),
}

enum PacketRecieveError {
    InvalidLength,
    PacketParseError,
}

pub trait PacketHandler {
    async fn send_packet(&self, packet: Packet);
    async fn recieve_packet(&self) -> Result<Packet, PacketRecieveError>;
}
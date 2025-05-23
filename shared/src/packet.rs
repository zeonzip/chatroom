/// Packets sent from the Client, to the Server. (Serverbound)
pub enum ServerboundPacket {
    // TODO: Add RSA handshake.
    Login {
        username: String,
    },

}

/// Packets sent from the Server, to the Client. (Clientbound)
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
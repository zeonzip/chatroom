use tokio::net::{TcpListener, TcpStream};
use uuid::Uuid;
use shared::packet::{ClientboundPacket, Packet, PacketHandler, PacketRecieveError, ServerboundPacket};

struct User {
    username: String,
    token: Uuid,
    stream: TcpStream,
    // RSA
}

impl User {
    pub fn connect(packet: ServerboundPacket::Login, mut stream: TcpStream) -> Result<User, ()> {
        let token = Uuid::new_v4();
        let username = packet.username;
        stream.send_packet(Packet::Clientbound(ClientboundPacket::Token { token }))
    }

    pub fn kick(&mut self, reason: &str) {
        self.stream.send_packet(Packet::Clientbound(ClientboundPacket::Kicked { reason: reason.to_string() }))
    }
}

struct Message {
    author: User,
    content: String,
}

struct ChatroomServer {
    listener: TcpListener,
    users: Vec<User>,
}

impl ChatroomServer {

}
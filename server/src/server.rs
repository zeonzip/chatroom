use std::collections::HashMap;
use std::io;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::select;
use tokio::sync::Mutex;
use shared::packet::{ClientboundPacket, Packet, PacketHandler, ServerboundPacket};
use crate::errors::ServerNetworkError;
use crate::user::shareduser::{CrossUserCommand, Feedback, SharedUser};
use crate::user::user::User;
use crate::user::UserId;

pub struct ChatroomServer {
    listener: TcpListener,
    users: HashMap<UserId, SharedUser>,
}

impl ChatroomServer {
    pub async fn new(listener: TcpListener) -> Self {
        Self { listener,  users: HashMap::new() }
    }
    pub async fn run(server: Arc<Mutex<Self>> /* couldnt name this self */) -> io::Result<()> {
        if let (stream, _) = server.lock().await.listener.accept().await? {
            tokio::spawn(Self::handle_client(server.clone(), stream));
        }

        Ok(())
    }

    pub async fn handle_packet(packet: ServerboundPacket, stream: Arc<Mutex<User>>) -> io::Result<()> {
        match packet {
            ServerboundPacket::Message { .. } => {

            }
            ServerboundPacket::Heartbeat { .. } => {

            }
            ServerboundPacket::Disconnect { .. } => {

            }
            _ => {
                return Err(ServerNetworkError::InvalidPacketFromClient.into())
            }
        }

        Ok(())
    }

    pub async fn handle_client(server: Arc<Mutex<Self>>, mut stream: TcpStream) -> io::Result<()> {
        let packet = stream.receive_packet().await?;
        let user;
        let mut dp;

        match packet {
            Packet::Serverbound(p) => {
                match p {
                    ServerboundPacket::Login { username } => {
                        let id = (server.lock().await.users.len() + 1) as UserId;
                        user = Arc::new(Mutex::new(User::connect(
                            username.clone(),
                            id,
                            stream,
                        ).await?));
                        let (dpt, su) = SharedUser::new(username, id);
                        server.lock().await.users.insert(id, su);
                        dp = dpt;
                    }
                    _ => return Err(ServerNetworkError::InvalidPacketFromClient.into())
                }
            },
            Packet::Clientbound(_) => {
                stream.send_packet(Packet::Clientbound(ClientboundPacket::Kicked {
                    reason: String::from("Sent clientbound packet to server")
                })).await?;
                return Err(ServerNetworkError::SentClientboundToServer.into())
            }
        }

        loop {
            select! {
                biased;

                result = user.lock().await.receive_packet().await? => {
                    match user.lock().await.receive_packet().await? {
                        Packet::Serverbound(packet) => Self::handle_packet(packet, user.clone()).await?,
                        _ => return Err(ServerNetworkError::SentClientboundToServer.into()),
                    }
                }

                command = dp.recv().await {
                    match command {
                        CrossUserCommand::Packet(p) => {
                            // TODO: handle errors from sending
                            dp.send(CrossUserCommand::Feedback(Feedback::PacketSendResult(user.lock().await.send_packet(p).await))).await;
                        },
                        _ => {},
                    }
                }
            }
        }
    }
}
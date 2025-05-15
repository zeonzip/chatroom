// code for the shared user visible to all threads in the server.
// shared users should be able to take commands from other threads through a MPSC channel, and then the task channel for the private user should handle the commands.
use tokio::sync::mpsc;
use tokio::io;

use shared::packet::Packet;
use crate::consts::CHANNEL_BFR_SIZE;
use crate::utils::channel;
use crate::utils::channel::DuplexPeer;
use super::UserId;

pub enum CrossUserCommand {
    Packet(Packet),
    Feedback(Feedback),
    N,
}

pub enum Feedback {
    PacketSendResult(io::Result<()>),
    InvalidCommandData,
}

pub struct SharedUser {
    username: String,
    id: UserId,
    dp: DuplexPeer<CrossUserCommand>,
}

impl SharedUser {
    pub fn new(username: String, id: UserId) -> (DuplexPeer<CrossUserCommand>, SharedUser){
        let (mut dp1, mut dp2) = channel::channel(CHANNEL_BFR_SIZE);

        (dp1, SharedUser {
            username,
            id,
            dp: dp2,
        })
    }
}
use crate::{packet::Packet, session::SessionLink};
use bytes::Bytes;

pub mod association_lists;
pub mod authentication;
pub mod user_sessions;
pub mod util;

pub async fn keep_alive(_: &SessionLink, packet: &Packet) -> Packet {
    let mut frame = packet.frame;
    frame.msg_type = crate::packet::MessageType::PingReply;

    Packet::new(frame, Bytes::new(), Bytes::new())
}

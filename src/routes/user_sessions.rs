use crate::{models::user_sessions::ValidateSessionKey, packet::Packet, session::SessionLink};

pub async fn update_network_info(session: &SessionLink, packet: &Packet) -> Packet {
    // TODO: don't use unwrap
    let user = session.data.get_user().unwrap();

    let notification = Packet::notification(30722, 1, ValidateSessionKey { user });
    session.tx.notify(notification);

    // FIXME: weird handling
    Packet::reply_empty(packet)
}

use crate::{models::user_sessions::ValidateSessionKey, packet::Packet, session::SessionLink};

pub async fn update_network_info(session: &SessionLink, packet: &Packet) -> Packet {
    let user = match session.data.get_user() {
        Some(user) => user,
        None => {
            println!("update_network_info: no user in session");
            return Packet::error(packet, 1);
        }
    };

    let notification = Packet::notification(30722, 1, ValidateSessionKey { user });
    session.tx.notify(notification);

    // FIXME: weird handling
    Packet::reply_empty(packet)
}

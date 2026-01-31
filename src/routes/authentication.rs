use std::sync::Arc;

use crate::{
    models::{
        authentication::{AuthRequest, AuthResponse, Entitlement, ListEntitlementsResponse},
        user_sessions::UpdateHardwareFlags,
    },
    packet::Packet,
    session::{SessionLink, User},
};

pub async fn login(session: &SessionLink, packet: &Packet) -> Packet {
    // TODO: don't unwrap
    let req: AuthRequest = Packet::deserialize(packet).unwrap();
    println!("received login request for {}", req.token);

    let user = User {
        user_id: 2407107883,
        persona_id: 1011786733,
        username: "ploxxxxxxy".to_string(),
    };

    session.data.set_user(Arc::new(user));

    // TODO: dont unwrap
    let user = session.data.get_user().unwrap();
    let notification = Packet::notification(30722, 8, UpdateHardwareFlags { user: user.clone() });

    session.tx.notify(notification);

    Packet::reply(packet, AuthResponse { user })
}

static ENTITLEMENTS: &[Entitlement] = &[Entitlement::pc(
    345193323,
    "308903",
    2,
    "Origin.OFR.50.0001000",
    "ONLINE_ACCESS",
)];

pub async fn list_entitlments(_: &SessionLink, packet: &Packet) -> Packet {
    Packet::reply(packet, ListEntitlementsResponse { list: ENTITLEMENTS })
}

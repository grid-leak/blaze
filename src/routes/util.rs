use std::time::{Duration, SystemTime, UNIX_EPOCH};

use tdf::TdfMap;

use crate::{
    config,
    models::{
        user_sessions::{UpdateExtendedDataAttribute, UserSessionExtendedData},
        util::{
            ClientConfigRequest, ClientConfigResponse, PingResponse, PostAuthResponse,
            PreAuthResponse,
        },
    },
    packet::Packet,
    session::SessionLink,
};

pub async fn pre_auth(_: &SessionLink, packet: &Packet) -> Packet {
    Packet::reply(packet, PreAuthResponse)
}

pub async fn ping(_: &SessionLink, packet: &Packet) -> Packet {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_secs();

    Packet::reply(packet, PingResponse { time })
}

pub async fn fetch_client_config(_: &SessionLink, packet: &Packet) -> Packet {
    // TODO: don't unwrap
    let req: ClientConfigRequest = Packet::deserialize(packet).unwrap();

    println!("received client config request for {}", req.id);

    let config: TdfMap<&'static str, &'static str> = match req.id.as_str() {
        "IdentityParams" => [
            ("display", "console2/welcome"),
            ("redirect_uri", "http://127.0.0.1/success"),
        ]
        .into_iter()
        .collect(),
        "PamplonaEndpoints" => config::Settings::global()
            .endpoints
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect(),
        _ => {
            println!("unknown client config request {}", req.id);
            TdfMap::new()
        }
    };

    Packet::reply(packet, ClientConfigResponse { config })
}

pub async fn post_auth(session: &SessionLink, packet: &Packet) -> Packet {
    // TODO: don't unwrap
    let user = session.data.get_user().unwrap();

    let notification =
        Packet::notification(30722, 5, UpdateExtendedDataAttribute { user: user.clone() });
    session.tx.notify(notification);

    let notification =
        Packet::notification(30722, 2, UserSessionExtendedData { user: user.clone() });
    session.tx.notify(notification);

    Packet::reply(packet, PostAuthResponse { user })
}

pub async fn set_client_state(_: &SessionLink, packet: &Packet) -> Packet {
    Packet::reply_empty(packet)
}

pub async fn set_client_metrics(_: &SessionLink, packet: &Packet) -> Packet {
    Packet::reply_empty(packet)
}

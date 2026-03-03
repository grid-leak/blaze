use std::{
    sync::LazyLock,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use tdf::TdfMap;

use crate::{
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

static PAMPLONA_ENDPOINTS: LazyLock<TdfMap<String, String>> = LazyLock::new(|| {
    let gateway_api = std::env::var("GATEWAY_API_URL").expect("GATEWAY_API_URL must be set");
    let gateway_upload =
        std::env::var("GATEWAY_UPLOAD_URL").expect("GATEWAY_UPLOAD_URL must be set");
    let engagement_api =
        std::env::var("ENGAGEMENT_API_URL").expect("ENGAGEMENT_API_URL must be set");

    [
        ("bugSentryDisableCrashDumpCollection".into(), "true".into()),
        ("bugSentryDisableGpuHangReports".into(), "true".into()),
        ("engagementManagerApiEndpointUrlBase".into(), engagement_api),
        (
            "engagementManagerClientId".into(),
            "mirrorsedgecatalyst".into(),
        ),
        ("gatewayApiEndpointUrl".into(), gateway_api),
        (
            "gatewayClientId".into(),
            "pamplona-backend-as-user-pc".into(),
        ),
        ("gatewayUploadEndpointUrl".into(), gateway_upload),
        (
            "messageManagerFetchMessagesIntervalTime".into(),
            "300.0".into(),
        ),
        (
            "messageManagerTransientMessagesToFollowers".into(),
            "false".into(),
        ),
        ("npsWebUrlBase".into(), "https://nps.pulse.ea.com".into()),
        ("presenceUpdatePositionInterval".into(), "10.0".into()),
        ("telemetryProjectId".into(), "308903".into()),
    ]
    .into_iter()
    .collect()
});

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

    let config: TdfMap<String, String> = match req.id.as_str() {
        "IdentityParams" => [
            ("display".into(), "console2/welcome".into()),
            ("redirect_uri".into(), "http://127.0.0.1/success".into()),
        ]
        .into_iter()
        .collect(),
        "PamplonaEndpoints" => PAMPLONA_ENDPOINTS.clone(),
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

use std::sync::Arc;

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    db,
    entities::accounts::{self, Column},
    models::{
        authentication::{AuthRequest, AuthResponse, Entitlement, ListEntitlementsResponse},
        user_sessions::UpdateHardwareFlags,
    },
    oauth,
    packet::Packet,
    session::{SessionLink, User},
};

pub async fn login(session: &SessionLink, packet: &Packet) -> Packet {
    let req: AuthRequest = match Packet::deserialize(packet) {
        Ok(req) => req,
        Err(e) => {
            println!("failed to deserialize AuthRequest: {e}");
            return Packet::error(packet, 1);
        }
    };

    let discord_user = match oauth::fetch_discord_user(&req.token).await {
        Ok(u) => u,
        Err(e) => {
            println!("discord auth failed: {e}");
            return Packet::error(packet, 1);
        }
    };

    let account = match accounts::Entity::find()
        .filter(Column::Provider.eq("discord"))
        .filter(Column::ProviderUserId.eq(&discord_user.id))
        .one(db::db())
        .await
    {
        Ok(Some(a)) => a,
        Ok(None) => {
            println!(
                "no account found for discord user {} ({})",
                discord_user.username, discord_user.id
            );
            return Packet::error(packet, 1);
        }
        Err(e) => {
            println!("database error during login: {e}");
            return Packet::error(packet, 1);
        }
    };

    println!(
        "authenticated discord user {}, persona_id {}",
        discord_user.username, account.persona_id
    );

    let user = Arc::new(User {
        user_id: account.persona_id as u32,
        persona_id: account.persona_id as u32,
        username: account.provider_username.clone(),
    });

    session.data.set_user(user.clone());

    let notification = Packet::notification(30722, 8, UpdateHardwareFlags { user: user.clone() });

    session.tx.notify(notification);

    Packet::reply(packet, AuthResponse::ok(user))
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

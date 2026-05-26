use std::sync::Arc;

use entities::{accounts, users};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    db,
    models::{
        authentication::{AuthRequest, AuthResponse, Entitlement, ListEntitlementsResponse},
        user_sessions::UpdateHardwareFlags,
    },
    oauth::{self},
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
        .filter(accounts::Column::Provider.eq("discord"))
        .filter(accounts::Column::ProviderUserId.eq(&discord_user.id))
        .one(db::db())
        .await
    {
        Ok(a) => a,
        Err(e) => {
            println!("database error during login lookup: {e}");
            return Packet::error(packet, 1);
        }
    };

    let (persona_id, username) = match account {
        Some(existing) => {
            let persona_id = existing.persona_id;
            if existing.provider_username != discord_user.username {
                let mut active: accounts::ActiveModel = existing.into();
                active.provider_username = ActiveValue::Set(discord_user.username.clone());
                if let Err(e) = active.update(db::db()).await {
                    println!("failed to update provider username in DB: {e}");
                    return Packet::error(packet, 1);
                }
            }

            let local_user = match users::Entity::find_by_id(persona_id).one(db::db()).await {
                Ok(Some(u)) => u,
                Ok(None) => {
                    println!("user record not found for persona_id {persona_id}");
                    return Packet::error(packet, 1);
                }
                Err(e) => {
                    println!("database error during user lookup: {e}");
                    return Packet::error(packet, 1);
                }
            };
            (persona_id, local_user.name)
        }
        None => {
            let new_user = users::ActiveModel {
                persona_id: ActiveValue::NotSet,
                name: ActiveValue::Set(discord_user.username.clone()),
                stats: ActiveValue::Set(serde_json::json!({})),
                division_name: ActiveValue::Set("Copper".to_string()),
                division_rank: ActiveValue::Set(5),
                ghost_data: ActiveValue::Set(serde_json::json!({
                    "variation": 244578012,
                    "timestamp": chrono::Utc::now().timestamp(),
                })),
                tag_data: ActiveValue::Set(serde_json::json!({
                    "bg":     { "tag": "2556762952" },
                    "detail": { "tag": "1514008114" },
                    "frame":  { "tag": "3049936381" },
                })),
            };

            let inserted_user = match new_user.insert(db::db()).await {
                Ok(u) => u,
                Err(e) => {
                    println!("failed to insert new user in DB: {e}");
                    return Packet::error(packet, 1);
                }
            };
            let persona_id = inserted_user.persona_id;

            let new_account = accounts::ActiveModel {
                id: ActiveValue::NotSet,
                persona_id: ActiveValue::Set(persona_id),
                provider: ActiveValue::Set("discord".to_string()),
                provider_user_id: ActiveValue::Set(discord_user.id.clone()),
                provider_username: ActiveValue::Set(discord_user.username.clone()),
            };
            if let Err(e) = new_account.insert(db::db()).await {
                println!("failed to insert new account in DB: {e}");
                return Packet::error(packet, 1);
            }

            (persona_id, inserted_user.name)
        }
    };

    println!(
        "authenticated discord user {}, local user {}, persona_id {}",
        discord_user.username, username, persona_id
    );

    let user = Arc::new(User {
        user_id: persona_id as u32,
        persona_id: persona_id as u32,
        username,
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

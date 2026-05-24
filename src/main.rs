use crate::{
    router::{Router, build_router},
    session::Session,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use uuid::Uuid;

mod db;
mod models;
mod oauth;
mod packet;
mod router;
mod routes;
mod session;
mod socket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let port: u16 = std::env::var("PORT")
        .expect("PORT must be set")
        .parse()
        .expect("PORT must be a valid u16");

    db::init(&database_url)
        .await
        .expect("Failed to connect to database");

    let router = build_router! {
        0, 0 => routes::keep_alive,
        1, 10 => routes::authentication::login,
        1, 29 => routes::authentication::list_entitlments,
        9, 1 => routes::util::fetch_client_config,
        9, 2 => routes::util::ping,
        9, 7 => routes::util::pre_auth,
        9, 8 => routes::util::post_auth,
        9, 22 => routes::util::set_client_metrics,
        9, 28 => routes::util::set_client_state,
        25, 6 => routes::association_lists::get_lists,
        30722, 20 => routes::user_sessions::update_network_info,
    };

    let router = Arc::new(router);

    let listener = TcpListener::bind(("0.0.0.0", port)).await?;

    loop {
        let (stream, _) = listener.accept().await?;
        Session::start(Uuid::new_v4(), stream, router.clone());
    }
}

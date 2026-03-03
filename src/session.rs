use crate::{
    router::Router,
    socket::{BlazeRx, BlazeTx, spawn_socket},
};
use parking_lot::RwLock;
use std::{
    sync::{Arc, Weak},
    time::Duration,
};
use tokio::{
    io::{AsyncRead, AsyncWrite},
    spawn,
};
use uuid::Uuid;

pub type SessionLink = Arc<Session>;
pub type WeakSessionLink = Weak<Session>;

pub struct Session {
    pub id: Uuid,
    pub tx: BlazeTx,
    pub data: SessionData,
}

impl Session {
    pub fn start<S>(id: Uuid, stream: S, router: Arc<Router>) -> WeakSessionLink
    where
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        let (socket_handle, rx, tx) = spawn_socket(stream);

        spawn(async move {
            if let Err(e) = socket_handle.await {
                println!("socket task panic: {e:?}");
            } else {
                println!("session socket closed");
            }
        });

        println!("session started {id}");

        let session = Arc::new(Self {
            id,
            tx,
            data: SessionData::new(),
        });

        let weak_session = Arc::downgrade(&session);

        spawn(run_session(rx, session, router));
        weak_session
    }
}

pub struct SessionData {
    user: RwLock<Option<Arc<User>>>,
}

#[derive(Debug)]
pub struct User {
    pub user_id: u32,
    pub username: String,
    pub persona_id: u32,
}

impl SessionData {
    pub fn new() -> Self {
        Self {
            user: RwLock::new(None),
        }
    }

    pub fn get_user(&self) -> Option<Arc<User>> {
        self.user.read().clone()
    }

    pub fn set_user(&self, user: Arc<User>) {
        *self.user.write() = Some(user);
    }
}

const KEEP_ALIVE_TIMEOUT: u64 = 60;

async fn run_session(mut rx: BlazeRx, session: Arc<Session>, router: Arc<Router>) {
    let timeout = Duration::from_secs(KEEP_ALIVE_TIMEOUT);

    loop {
        match tokio::time::timeout(timeout, rx.recv()).await {
            Ok(Some(packet)) => {
                let response = router.handle(&session, &packet).await;
                session.tx.notify(response);
            }
            Ok(None) => break, // Channel closed
            Err(_) => {
                println!("session {} timed out", session.id);
                break;
            }
        }
    }
}

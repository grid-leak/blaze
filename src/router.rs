use crate::{packet::Packet, session::SessionLink};
use futures::future::BoxFuture;
use std::collections::HashMap;

const fn component_key(component: u16, command: u16) -> u32 {
    ((component as u32) << 16) + command as u32
}

pub type HandlerFn = for<'a> fn(&'a SessionLink, &'a Packet) -> BoxFuture<'a, Packet>;

pub struct Router {
    routes: HashMap<u32, HandlerFn>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn route(mut self, component: u16, command: u16, handler: HandlerFn) -> Self {
        let key = component_key(component, command);
        self.routes.insert(key, handler);
        self
    }

    pub async fn handle(&self, session: &SessionLink, packet: &Packet) -> Packet {
        let key = component_key(packet.frame.component, packet.frame.command);

        println!(
            "[RECV] component={} command={} type={:?} num={}",
            packet.frame.component,
            packet.frame.command,
            packet.frame.msg_type,
            packet.frame.msg_num
        );

        println!("{:#?}\n", packet);

        match self.routes.get(&key) {
            Some(handler) => handler(session, packet).await,
            None => {
                println!(
                    "no handler for component {} command {}",
                    packet.frame.component, packet.frame.command
                );
                Packet::reply_empty(packet)
            }
        }
    }
}

macro_rules! build_router {
    ( $( $component:expr, $command:expr => $handler:path ),* $(,)? ) => {
        Router::new()
            $(
                .route($component, $command, |s, p| Box::pin($handler(s, p)))
            )*
    };
}

pub(crate) use build_router;

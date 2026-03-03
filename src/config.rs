use std::{collections::HashMap, sync::OnceLock};

use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub port: u16,
    pub keep_alive_timeout: u32,
    pub database_url: String,
    pub endpoints: HashMap<String, String>,
}

pub static CONFIG: OnceLock<Settings> = OnceLock::new();

impl Settings {
    pub fn init() {
        let s = Config::builder()
            .add_source(File::with_name("config"))
            .build()
            .expect("Failed to build the config");

        let settings: Settings = s
            .try_deserialize()
            .expect("Failed to deserialize the config");

        CONFIG.set(settings).expect("Failed to set the config");
    }

    pub fn global() -> &'static Settings {
        CONFIG.get().expect("Config not initialized")
    }
}

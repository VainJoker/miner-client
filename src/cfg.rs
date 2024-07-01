use std::{fmt::Debug, fs, sync::OnceLock};

use config::Config;
use serde::{Deserialize, Serialize};

// Create a static lock for the configuration, ensuring
// that it's only initialized once across the entire application.
static CFG: OnceLock<AppConfig> = OnceLock::new();

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub mqtt: MqttConfig,
    pub sign: SignConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SignConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MqttConfig {
    pub host: String,
    pub port: u16,
    pub topics: Vec<TopicConfig>,
    pub keepalive: u64,
    pub client_id: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TopicConfig {
    pub topics: String,
    pub qos: u8,
}

/// Initializes the application's configuration from the provided file.
/// Expected to be run on startup of the application.
pub fn init(cfg_file: &String) {
    // Attempt to extract the canonical, absolute path of the configuration
    // file. Panic if this operation fails, as the configuration is critical
    // for execution.
    let path = fs::canonicalize(cfg_file).unwrap_or_else(|e| {
        panic!("💥 Failed to load configuration file: {e} - {cfg_file}");
    });

    // Attempt to build the configuration from the file.
    // Panic if any errors occur during loading or validation.
    let cfg = Config::builder()
        .add_source(config::File::with_name(path.to_str().unwrap_or_else(
            || {
                panic!("💥 Failed to build configuration: {cfg_file}");
            },
        )))
        .build()
        .unwrap_or_else(|e| {
            panic!("💥 Failed to build configuration: {e}");
        });

    let pay: AppConfig = cfg.try_deserialize().unwrap_or_else(|e| {
        panic!("💥 Failed to deserialize configuration: {e}");
    });
    // Attempt to lock the configuration for the first time.
    // Ignore the result because we'd panic if locking fails.
    let _ = CFG.set(pay);
    tracing::info!("🚀 Configuration loading is successful!");
}

/// Accesses the application's configuration, once initialized.
/// Panics if called before `init`.
pub fn config() -> &'static AppConfig {
    CFG.get().unwrap_or_else(|| {
        panic!("💥 Configuration accessed before initialization");
    })
}

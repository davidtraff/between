use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct NetworkConfig {
    pub ws_address: String,
    pub proxy_listen_address: String,
    pub proxy_connect_address: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub network: NetworkConfig,
}

impl AppConfig {
    pub fn load() -> Result<Self, config::ConfigError> {
        let settings = config::Config::builder()
            .add_source(config::File::with_name("between.toml"))
            .build()?;

        settings.try_deserialize()
    }
}

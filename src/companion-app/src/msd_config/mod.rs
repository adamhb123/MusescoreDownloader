use std::{collections::HashMap, fmt::Display, fs::File, io::Write, path::Path};
use config::Config;
use serde::{Deserialize, Serialize};

const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Serialize, Deserialize)]
pub struct MSDConfig {
    require_admin: bool,
    service_name: &'static str,
    address: &'static str,
    port: u16,
}
impl MSDConfig {
    pub fn as_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self)
    }
}
impl Default for MSDConfig {
    fn default() -> Self {
        Self {
            require_admin: true,
            service_name: "msd-companion",
            address: "127.0.0.1",
            port: 42134,
        }
    }
}
impl Display for MSDConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_json_string().unwrap().as_str())
    }
}
pub fn config_exists() -> bool {
    Path::new(CONFIG_FILE_NAME).exists()
}
pub fn generate_default_config() -> std::io::Result<MSDConfig> {
    let config = MSDConfig::default();
    let mut config_file = File::create(CONFIG_FILE_NAME)?;
    config_file.write_all(format!("{}", config).as_bytes())?;
    Ok(config)
}
pub fn get_config() -> HashMap<String, String> {
    Config::builder().add_source(config::File::with_name(CONFIG_FILE_NAME)).build().unwrap().try_deserialize().unwrap()
}
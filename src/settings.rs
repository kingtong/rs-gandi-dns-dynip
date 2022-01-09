
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;
use serde_json;

#[derive(Debug, Default, Deserialize)]
pub struct Settings {
    pub api_key: String,
    pub domain: String,
    pub record: String,
    #[serde(default)]
    pub ip: Option<String>,
}

impl Settings {
    pub fn load(settings_path_str: &str) -> Result<Settings> {
        let settings_path = Path::new(settings_path_str);
        let settings = if settings_path.exists() {
            let file = File::open(settings_path)?;
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).context("Could not parse config")?
        } else {
            Settings::default()
        };

        Ok(settings)
    }
}

use super::application_config::ApplicationConfig;
use anyhow::{Ok, Result};
use lazy_static::lazy_static;
use std::fs::File;
use std::io::prelude::*;

lazy_static! {
    static ref CONFIG: String = read_config().unwrap();
    pub static ref APPCONFIG: ApplicationConfig = load_config().unwrap();
}

fn read_config() -> Result<String> {
    let file_path = "application.yml";
    let mut file = File::open(file_path)?;
    let mut str_val = String::new();
    file.read_to_string(&mut str_val).unwrap();
    Ok(str_val)
}

pub fn load_config() -> Result<ApplicationConfig> {
    let config: ApplicationConfig = serde_yaml::from_str(&CONFIG.to_owned()).unwrap();
    Ok(config)
}

use std::{fs, io::Read, path};

use base64::{
    Engine as _, alphabet,
    engine::{self, GeneralPurpose, general_purpose},
};

pub struct Config {
    pub port: u16,
    pub auth: Option<(String, String)>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            port: 8080,
            auth: None,
        }
    }

    pub fn from_args() -> Self {
        let mut config = Config::new();

        if let Some(port) = std::env::args().nth(1) {
            config.port = port.parse().unwrap_or(8080);
        }

        if let Some(auth) = std::env::args().nth(2) {
            let parts: Vec<&str> = auth.split(':').collect();
            if parts.len() == 2 {
                config.auth = Some((parts[0].trim().to_string(), parts[1].trim().to_string()));
            }
        }

        config
    }

    pub fn from_file(path: &str) -> Self {
        let mut config = Config::new();

        if let Ok(mut file) = std::fs::File::open(path) {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            for line in contents.lines() {
                let parts: Vec<&str> = line.split('=').collect();
                if parts.len() == 2 {
                    let key = parts[0].trim();
                    let value = parts[1]
                        .trim()
                        .to_string()
                        .strip_prefix('"')
                        .unwrap_or("")
                        .strip_suffix('"')
                        .unwrap_or("")
                        .to_string();
                    match key {
                        "port" => config.port = parts[1].trim().parse().unwrap_or(8080),
                        "user" => {
                            let password = if let Some(auth) = config.auth {
                                auth.1.clone()
                            } else {
                                "".to_string()
                            };

                            config.auth = Some((value, password));
                        }
                        "password" => {
                            let user = if let Some(auth) = config.auth {
                                auth.0.clone()
                            } else {
                                "".to_string()
                            };
                            config.auth = Some((user, value))
                        }
                        _ => {}
                    }
                }
            }
        }

        config
    }

    pub fn auth_basic(&self) -> Option<String> {
        let auth = self.auth.clone()?;

        Some(format!(
            "Basic {}",
            general_purpose::STANDARD.encode(format!("{}:{}", auth.0, auth.1))
        ))
    }
}

pub fn config_file_exists(path: &str) -> bool {
    std::fs::metadata(path).is_ok()
}

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};

#[derive(Serialize, Deserialize)]
pub struct CDNConfig {
    pub client_id: String,
    pub client_secret: String,
    pub uri: String,
    pub redirect_uri: String,
    pub port: u16,
    pub users: Vec<UserSettings>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserSettings {
    pub user_id: String,
    pub user_key: String,
}

impl CDNConfig {
    pub fn new() -> std::io::Result<Self> {
        let file = File::open("config.json")?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)?;
        Ok(config)
    }

    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    pub fn client_secret(&self) -> &str {
        &self.client_secret
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }

    pub fn redirect_uri(&self) -> &str {
        &self.redirect_uri
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn users(&self) -> &[UserSettings] {
        &self.users
    }
}

pub fn setup_config() -> std::io::Result<()> {
    let file = File::open("config.json");
    match file {
        Ok(_) => Ok(()),
        Err(_) => {
            let default_config = CDNConfig {
                client_id: String::from("replace with client id"),
                client_secret: String::from("replace with client secret"),
                uri: String::from("http://localhost:6969"),
                redirect_uri: String::from("http://localhost:6969/callback"),
                port: 6969,
                users: Vec::from([UserSettings {
                    user_id: String::from("youruserid"),
                    user_key: String::from("super_secret_key"),
                }]),
            };
            let file = File::create("config.json")?;
            let writer = BufWriter::new(file);
            serde_json::to_writer(writer, &default_config)?;
            Ok(())
        }
    }
}

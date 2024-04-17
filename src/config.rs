use std::io::ErrorKind;

use serde::{Deserialize, Serialize};
use tokio::fs;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    #[serde(default = "default_host")]
    pub host: String,

    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_cors_allowed_origins")]
    pub cors_allowed_origins: Vec<String>,

    #[serde(default = "default_database_file")]
    pub database_file: String,

    #[serde(default = "default_static_file_root")]
    pub static_file_root: String,

    #[serde(default = "default_static_file_index")]
    pub static_file_index: String,

    #[serde(default = "default_enable_api_request_logging")]
    pub enable_api_request_logging: bool,
}

impl Config {
    pub async fn load(filename: &str) -> Self {
        let contents = match fs::read_to_string(filename).await {
            Ok(value) => value,
            Err(err) => {
                if err.kind() == ErrorKind::NotFound {
                    String::from("{}")
                } else {
                    panic!(
                        "Error occurred reading config file \"{}\": {:?}",
                        filename, err
                    )
                }
            }
        };

        serde_json::from_str(&contents).expect("Error occurred parsing config file")
    }
}

fn default_host() -> String {
    String::from("127.0.0.1")
}
fn default_port() -> u16 {
    8080
}
fn default_cors_allowed_origins() -> Vec<String> {
    Vec::from([
        // String::from("http://localhost:5173"),
        // String::from("http://127.0.0.1:5173"),
    ])
}
fn default_database_file() -> String {
    String::from("./database.sqlite3")
}
fn default_static_file_root() -> String {
    String::from("./client/dist/")
}
fn default_static_file_index() -> String {
    String::from("index.html")
}
fn default_enable_api_request_logging() -> bool {
    true
}
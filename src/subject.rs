use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum TokenSubject {
    #[serde(rename = "discord")]
    Discord(String),
    #[serde(rename = "server")]
    Server(String),
    #[serde(rename = "client")]
    Client(String),
}

use serde::{Deserialize, Serialize};

use super::DiscordToken;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum TokenClaims {
    #[serde(rename = "discord")]
    Discord(DiscordToken),
    #[serde(rename = "server")]
    Server(TokenClaimsServer),
    #[serde(rename = "custom")]
    Custom(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct TokenClaimsServer {
    guild: String,
    perms: Vec<String>,
}

impl TokenClaimsServer {
    pub fn new(guild: &str, perms: Vec<String>) -> Self {
        Self {
            guild: guild.to_string(),
            perms,
        }
    }

    pub fn guild(&self) -> &str {
        &self.guild
    }

    pub const fn perms(&self) -> &Vec<String> {
        &self.perms
    }
}

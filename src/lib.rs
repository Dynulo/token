pub const JWT_HEADER: &str = "SSOHeader";
pub const JWT_PAYLOAD: &str = "SSOPayload";
pub const JWT_SIGNATURE: &str = "SSOSignature";

mod claims;
mod discord;
mod subject;

pub use claims::{TokenClaims, TokenClaimsServer};
pub use discord::DiscordToken;
pub use subject::TokenSubject;

use chrono::serde::ts_seconds;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    /// JWT ID
    /// a uuid generated for each token
    pub jti: String,
    /// Subject
    /// The user of the token
    pub sub: TokenSubject,
    /// Claims
    /// The claims of the token
    pub claims: TokenClaims,
    /// Issued At
    /// The time the token was issued
    #[serde(with = "ts_seconds")]
    pub iat: DateTime<Utc>,
    /// Expires At
    /// The time the token expires
    #[serde(with = "ts_seconds")]
    pub exp: DateTime<Utc>,
    /// Issuer
    /// The issuer of the token
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub iss: String,
}

impl Token {
    pub fn new(
        subject: TokenSubject,
        claims: TokenClaims,
        duration: Option<Duration>,
        issuer: Option<String>,
    ) -> Self {
        Self {
            jti: Uuid::new_v4().to_string(),
            sub: subject,
            claims,
            iat: chrono::Utc::now(),
            exp: (chrono::Utc::now() + duration.unwrap_or_else(|| Duration::days(5))),
            iss: issuer.unwrap_or_default(),
        }
    }

    #[must_use]
    pub fn auth(&self) -> Option<String> {
        if let TokenClaims::Discord(id) = &self.claims {
            Some(format!("Bearer {}", id.expose()))
        } else {
            None
        }
    }

    pub fn as_issuer(&self) -> String {
        match &self.sub {
            TokenSubject::Discord(id) => format!("discord:{}", id),
            TokenSubject::Server(id) => format!("server:{}", id),
            TokenSubject::Client(id) => format!("client:{}", id),
        }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiscordToken(String);
impl DiscordToken {
    pub const fn new(token: String) -> Self {
        Self(token)
    }

    pub fn expose(&self) -> &str {
        &self.0
    }

    pub fn auth_header(&self) -> String {
        format!("Bearer {}", self.expose())
    }
}
impl std::fmt::Debug for DiscordToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[DiscordToken]")
    }
}

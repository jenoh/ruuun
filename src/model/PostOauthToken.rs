use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostOauthToken {
    pub client_id: String,
    pub client_secret: String,
    pub code: String,
    pub grant_type: String,
}

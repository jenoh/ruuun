use super::Athlete::Athlete;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseOauthToken {
    pub token_type: String,
    pub expires_at: i64,
    pub expires_in: i64,
    pub refresh_token: String,
    pub access_token: String,
    pub athlete: Athlete,
}

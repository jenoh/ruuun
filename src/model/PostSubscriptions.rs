use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostSubscriptions {
    pub client_id: String,
    pub client_secret: String,
    pub callback_url: String,
    pub verify_token: String,
}

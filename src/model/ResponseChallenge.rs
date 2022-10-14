use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseChallenge {
    #[serde(rename(serialize = "hub.challenge"))]
    pub challenge: String,
}

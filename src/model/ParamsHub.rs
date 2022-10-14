use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ParamsHub {
    #[serde(alias = "hub.mode")]
    pub mode: String,
    #[serde(alias = "hub.challenge")]
    pub challenge: String,
    #[serde(alias = "hub.verify_token")]
    pub verify_token: String,
}

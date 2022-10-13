use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Params {
    pub code: String,
    pub state: String,
    pub scope: String,
}

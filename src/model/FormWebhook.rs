use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FormWebhook {
    pub aspect_type: String,
    pub event_time: u32,
    pub object_id: usize,
    pub object_type: String,
    pub owner_id: u32,
    pub subscription_id: u32,
}

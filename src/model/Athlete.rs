use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Athlete {
    pub id: i64,
    pub username: String,
    pub resource_state: i64,
    pub firstname: String,
    pub lastname: String,
    pub bio: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub sex: String,
    pub premium: bool,
    pub summit: bool,
    pub created_at: String,
    pub updated_at: String,
    pub badge_type_id: i64,
    pub weight: f64,
    pub profile_medium: String,
    pub profile: String,
    pub friend: Option<String>,
    pub follower: Option<String>,
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Athlete {
    id: i32,
    username: String,
    firstname: String,
    lastname: String,
    bio: String,
    city: String,
    state: String,
    country: String,
    profile_medium: String,
    profile: String,
}

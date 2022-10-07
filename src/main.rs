use log::{debug, error, info, log_enabled, Level};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;

// https://www.strava.com/oauth/authorize?client_id=73985&response_type=code&redirect_uri=http://d741-92-88-241-146.ngrok.io&approval_prompt=force&scope=activity:read_all

#[derive(Debug, Serialize, Deserialize)]
struct PostOauthToken {
    client_id: String,
    client_secret: String,
    code: String,
    grant_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseOauthToken {
    pub token_type: String,
    pub expires_at: i64,
    pub expires_in: i64,
    pub refresh_token: String,
    pub access_token: String,
    pub athlete: Athlete,
}

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Get env vars
    let strava_code: String = env::var("STRAVA_CODE").unwrap();
    let strava_client_id: String = env::var("STRAVA_CLIENT_ID").unwrap();
    let strava_client_secret: String = env::var("STRAVA_CLIENT_SECRET").unwrap();

    info!(
        "Get env vars: {}, {}, {}",
        stringify!(strava_code),
        stringify!(strava_client_id),
        stringify!(strava_client_secret)
    );

    let post_oauth_token = PostOauthToken {
        client_id: strava_client_id,
        client_secret: strava_client_secret,
        code: strava_code,
        grant_type: "authorization_code".to_string(),
    };

    let res = reqwest::Client::new()
        .post("https://www.strava.com/oauth/token")
        .json(&post_oauth_token)
        .send()
        .await?;

    let js = res.json::<ResponseOauthToken>().await?;

    info!("Get oauth of: {}", js.athlete.username);
    // println!("{:?}", js);
    Ok(())
}

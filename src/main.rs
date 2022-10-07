use log::{debug, error, info, log_enabled, Level};
use serde_json::Value;
use std::env;

mod model;

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

    let post_oauth_token = model::PostOauthToken::PostOauthToken {
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

    let js = res
        .json::<model::ResponseOauthToken::ResponseOauthToken>()
        .await?;

    info!("Get oauth of: {}", js.athlete.username);
    // println!("{:?}", js);
    Ok(())
}

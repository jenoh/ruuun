use crate::model;
use log::info;

pub async fn oauth_token(
    strava_client_id: String,
    strava_client_secret: String,
    strava_code: String,
) -> Result<model::ResponseOauthToken::ResponseOauthToken, Box<dyn std::error::Error>> {
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

    let athlete = res
        .json::<model::ResponseOauthToken::ResponseOauthToken>()
        .await?;

    info!("Get oauth of: {}", athlete.athlete.username);

    Ok(athlete)
}

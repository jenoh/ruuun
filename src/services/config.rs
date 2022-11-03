use std::env;

pub fn get_strava_client_id() -> String {
    return env::var("STRAVA_CLIENT_ID").unwrap();
}

pub fn get_strava_client_secret() -> String {
    return env::var("STRAVA_CLIENT_SECRET").unwrap();
}

pub fn get_url_webhook() -> String {
    return env::var("URL_WEBHOOK").unwrap();
}

pub fn get_verify_token() -> String {
    return env::var("VERIFY_TOKEN").unwrap();
}

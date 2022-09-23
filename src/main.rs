use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let token = std::env::var("STRAVA_ACCESS_TOKEN");

    let res: model::Athlete = client
        .get("https://www.strava.com/api/v3/athlete")
        .header(
            reqwest::header::AUTHORIZATION,
            "Bearer ".to_owned() + &token.unwrap().to_owned(),
        )
        .send()
        .await?
        .json()
        .await?;
    println!("{:#?}", res);

    Ok(())
}

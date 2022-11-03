use crate::model;

pub async fn put_activity(id: String, token: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("{:?}", token);
    let update = model::UpdatableActivity::UpdatableActivity {
        commute: None,
        trainer: None,
        hide_from_home: None,
        description: Some("Sample".to_string()),
        name: Some("Sample".to_string()),
        gear_id: None,
        sport_type: None,
    };
    reqwest::Client::new()
        .put(format!("https://www.strava.com/api/v3/activities/{}", id))
        .header("Authorization", token.clone())
        .json(&update)
        .send()
        .await?;
    Ok(())
}

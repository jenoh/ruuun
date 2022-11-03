use crate::model;
use log::info;
pub async fn get_activity(
    id: String,
    token: String,
) -> Result<model::ResponseActivity::ResponseActivity, Box<dyn std::error::Error>> {
    let res = reqwest::Client::new()
        .get(format!("https://www.strava.com/api/v3/activities/{}", id))
        .header("Authorization", token.clone())
        .send()
        .await?;
    let activity = res
        .json::<model::ResponseActivity::ResponseActivity>()
        .await?;

    info!("Get informations on activity {}", id);
    Ok(activity)
}
pub async fn put_activity(
    id: String,
    token: String,
    distance: f64,
    // mooving_time: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    let distance_km: f64 = distance / 1000.00;
    // let speed: f64 = (mooving_time / 60) as f64 / distance_km;
    let update = model::UpdatableActivity::UpdatableActivity {
        commute: None,
        trainer: None,
        hide_from_home: None,
        description: Some(
            "This activity is modified by my script here: https://github.com/jenoh/ruuun"
                .to_string(),
        ),
        name: Some(format!("Run de {:.2}km", distance_km)),
        gear_id: None,
        sport_type: None,
    };
    reqwest::Client::new()
        .put(format!("https://www.strava.com/api/v3/activities/{}", id))
        .header("Authorization", token.clone())
        .json(&update)
        .send()
        .await?;

    info!("Activity {} is now modified ", id);
    Ok(())
}

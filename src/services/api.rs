use crate::model;

pub async fn get_activity(id: String, token: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("{:?}", token);

    let mut response_activity = reqwest::get("https://www.strava.com/api/v3/activities/7955598559")
        .await?
        .text()
        .await?;
    println!("{:?}", response_activity);
    Ok(())
}

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseActivity {
    pub resource_state: Option<i64>,
    pub athlete: Athlete,
    pub name: String,
    pub distance: f64,
    pub moving_time: i64,
    pub elapsed_time: i64,
    pub total_elevation_gain: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub sport_type: String,
    pub workout_type: Option<String>,
    pub id: i64,
    pub start_date: String,
    pub start_date_local: String,
    pub timezone: String,
    pub utc_offset: f64,
    pub location_city: Option<String>,
    pub location_state: Option<String>,
    pub location_country: String,
    pub achievement_count: i64,
    pub kudos_count: i64,
    pub comment_count: i64,
    pub athlete_count: i64,
    pub photo_count: i64,
    pub map: Map,
    pub trainer: bool,
    pub commute: bool,
    pub manual: bool,
    pub private: bool,
    pub visibility: String,
    pub flagged: bool,
    pub gear_id: Option<String>,
    pub start_latlng: Vec<String>,
    pub end_latlng: Vec<String>,
    pub average_speed: f64,
    pub max_speed: i64,
    pub has_heartrate: bool,
    pub heartrate_opt_out: bool,
    pub display_hide_heartrate_option: bool,
    pub upload_id: Option<String>,
    pub external_id: Option<String>,
    pub from_accepted_tag: bool,
    pub pr_count: i64,
    pub total_photo_count: i64,
    pub has_kudoed: bool,
    pub description: String,
    pub calories: f64,
    pub perceived_exertion: Option<String>,
    pub prefer_perceived_exertion: Option<String>,
    pub segment_efforts: Vec<String>,
    pub best_efforts: Vec<String>,
    pub photos: Photos,
    pub stats_visibility: Vec<StatsVisibility>,
    pub hide_from_home: Option<bool>,
    pub embed_token: String,
    pub similar_activities: SimilarActivities,
    pub available_zones: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Athlete {
    pub id: i64,
    pub resource_state: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Map {
    pub id: String,
    pub polyline: String,
    pub resource_state: i64,
    pub summary_polyline: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Photos {
    pub primary: Option<String>,
    pub count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatsVisibility {
    #[serde(rename = "type")]
    pub type_field: String,
    pub visibility: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimilarActivities {
    pub effort_count: i64,
    pub average_speed: i64,
    pub min_average_speed: i64,
    pub mid_average_speed: i64,
    pub max_average_speed: i64,
    pub pr_rank: Option<String>,
    pub frequency_milestone: Option<String>,
    pub trend: Trend,
    pub resource_state: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trend {
    pub speeds: Vec<String>,
    pub current_activity_index: Option<String>,
    pub min_speed: i64,
    pub mid_speed: i64,
    pub max_speed: i64,
    pub direction: i64,
}

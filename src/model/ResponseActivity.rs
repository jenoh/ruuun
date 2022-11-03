use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseActivity {
    pub resource_state: i64,
    pub external_id: String,
    pub upload_id: i64,
    pub athlete: Athlete,
    pub name: String,
    pub distance: i64,
    pub moving_time: i64,
    pub elapsed_time: i64,
    pub total_elevation_gain: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub sport_type: String,
    pub start_date: String,
    pub start_date_local: String,
    pub timezone: String,
    pub utc_offset: i64,
    pub start_latlng: Vec<f64>,
    pub end_latlng: Vec<f64>,
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
    pub flagged: bool,
    pub gear_id: String,
    pub from_accepted_tag: bool,
    pub average_speed: f64,
    pub max_speed: f64,
    pub average_cadence: f64,
    pub average_temp: i64,
    pub average_watts: f64,
    pub weighted_average_watts: i64,
    pub kilojoules: f64,
    pub device_watts: bool,
    pub has_heartrate: bool,
    pub max_watts: i64,
    pub elev_high: f64,
    pub elev_low: f64,
    pub pr_count: i64,
    pub total_photo_count: i64,
    pub has_kudoed: bool,
    pub workout_type: i64,
    pub suffer_score: Value,
    pub description: String,
    pub calories: f64,
    pub segment_efforts: Vec<SegmentEffort>,
    pub splits_metric: Vec<SplitsMetric>,
    pub laps: Vec<Lap>,
    pub gear: Gear,
    pub partner_brand_tag: Value,
    pub photos: Photos,
    pub highlighted_kudosers: Vec<HighlightedKudoser>,
    pub hide_from_home: bool,
    pub device_name: String,
    pub embed_token: String,
    pub segment_leaderboard_opt_out: bool,
    pub leaderboard_opt_out: bool,
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
pub struct SegmentEffort {
    pub id: i64,
    pub resource_state: i64,
    pub name: String,
    pub activity: Activity,
    pub athlete: Athlete2,
    pub elapsed_time: i64,
    pub moving_time: i64,
    pub start_date: String,
    pub start_date_local: String,
    pub distance: f64,
    pub start_index: i64,
    pub end_index: i64,
    pub average_cadence: f64,
    pub device_watts: bool,
    pub average_watts: f64,
    pub segment: Segment,
    pub kom_rank: Value,
    pub pr_rank: Value,
    pub achievements: Vec<Value>,
    pub hidden: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Activity {
    pub id: i64,
    pub resource_state: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Athlete2 {
    pub id: i64,
    pub resource_state: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Segment {
    pub id: i64,
    pub resource_state: i64,
    pub name: String,
    pub activity_type: String,
    pub distance: f64,
    pub average_grade: f64,
    pub maximum_grade: f64,
    pub elevation_high: f64,
    pub elevation_low: f64,
    pub start_latlng: Vec<f64>,
    pub end_latlng: Vec<f64>,
    pub climb_category: i64,
    pub city: String,
    pub state: String,
    pub country: String,
    pub private: bool,
    pub hazardous: bool,
    pub starred: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SplitsMetric {
    pub distance: f64,
    pub elapsed_time: i64,
    pub elevation_difference: f64,
    pub moving_time: i64,
    pub split: i64,
    pub average_speed: f64,
    pub pace_zone: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lap {
    pub id: i64,
    pub resource_state: i64,
    pub name: String,
    pub activity: Activity2,
    pub athlete: Athlete3,
    pub elapsed_time: i64,
    pub moving_time: i64,
    pub start_date: String,
    pub start_date_local: String,
    pub distance: f64,
    pub start_index: i64,
    pub end_index: i64,
    pub total_elevation_gain: i64,
    pub average_speed: f64,
    pub max_speed: f64,
    pub average_cadence: f64,
    pub device_watts: bool,
    pub average_watts: f64,
    pub lap_index: i64,
    pub split: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Activity2 {
    pub id: i64,
    pub resource_state: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Athlete3 {
    pub id: i64,
    pub resource_state: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gear {
    pub id: String,
    pub primary: bool,
    pub name: String,
    pub resource_state: i64,
    pub distance: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Photos {
    pub primary: Primary,
    pub use_primary_photo: bool,
    pub count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Primary {
    pub id: Value,
    pub unique_id: String,
    pub urls: Urls,
    pub source: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Urls {
    #[serde(rename = "100")]
    pub n100: String,
    #[serde(rename = "600")]
    pub n600: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HighlightedKudoser {
    pub destination_url: String,
    pub display_name: String,
    pub avatar_url: String,
    pub show_name: bool,
}

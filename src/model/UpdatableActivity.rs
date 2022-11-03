// use super::Athlete::Athlete;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdatableActivity {
    pub commute: Option<bool>,
    pub trainer: Option<bool>,
    pub hide_from_home: Option<bool>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub gear_id: Option<String>,
    pub sport_type: Option<String>,
}

// commute
// boolean	Whether this activity is a commute
// trainer
// boolean	Whether this activity was recorded on a training machine
// hide_from_home
// boolean	Whether this activity is muted
// description
// string	The description of the activity
// name
// string	The name of the activity
// type
// ActivityType	Deprecated. Prefer to use sport_type. In a request where both type and sport_type are present, this field will be ignored
// sport_type
// SportType	An instance of SportType.
// gear_id
// string

use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HourlyForecast {
    pub date_time: DateTime<Utc>,
    pub has_precipitation: bool,
    pub precipitation_probability: u32,
    pub icon_phrase: String,
}

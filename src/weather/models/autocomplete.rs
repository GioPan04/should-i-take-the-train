use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Autocomplete {
    pub key: String,
    pub localized_name: String,
    pub country: Area,
    pub administrative_area: Area,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Area {
    #[serde(rename = "ID")]
    pub id: String,
    pub localized_name: String,
}

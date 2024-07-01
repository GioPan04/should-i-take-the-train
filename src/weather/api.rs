use super::models::{Autocomplete, HourlyForecast};
use anyhow::Result;
use reqwest::{Client, Url};
use url;

const BASE_URL: &str = "https://dataservice.accuweather.com";

pub struct WeatherApi {
    api_key: String,
    language: Option<String>,
    client: Client,
}

impl WeatherApi {
    pub fn new(key: &str, lang: Option<String>) -> Self {
        WeatherApi {
            api_key: String::from(key),
            language: lang,
            client: Client::new(),
        }
    }

    fn build_url(&self, path: &str) -> Result<Url, url::ParseError> {
        let url;

        if let Some(lang) = &self.language {
            url = format!("{BASE_URL}/{path}?apikey={}&language={lang}", self.api_key);
        } else {
            url = format!("{BASE_URL}/{path}?apikey={}", self.api_key);
        }

        Url::parse(url.as_str())
    }

    pub async fn autocomplete(&self, q: &str) -> Result<Vec<Autocomplete>> {
        let mut url = self.build_url("locations/v1/cities/autocomplete")?;
        url.query_pairs_mut().extend_pairs(&[("q", q)]);

        let res = self.client.get(url).send().await?.json().await?;
        Ok(res)
    }

    pub async fn hourly12(&self, city_id: &str) -> Result<Vec<HourlyForecast>> {
        let url = self.build_url(format!("forecasts/v1/hourly/12hour/{city_id}").as_str())?;
        let res = self.client.get(url).send().await?.json().await?;

        Ok(res)
    }
}

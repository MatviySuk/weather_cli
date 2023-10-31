use crate::weather;
use serde_derive::Deserialize;

use async_trait::async_trait;
use reqwest::{self, Client};
use url::Url;

use super::Provider;

#[derive(Deserialize, Debug)]
struct WeatherData {
    location: Location,
    current: Current,
    forecast: Forecast,
}

#[derive(Deserialize, Debug)]
struct Location {
    name: String,
    region: String,
    country: String,
    lat: f64,
    lon: f64,
    tz_id: String,
    localtime_epoch: i64,
    localtime: String,
}

#[derive(Deserialize, Debug)]
struct Current {
    last_updated_epoch: i64,
    last_updated: String,
    temp_c: f64,
    temp_f: f64,
    is_day: i64,
    condition: Condition,
    wind_mph: f64,
    wind_kph: f64,
    wind_degree: i64,
    wind_dir: String,
    pressure_mb: f64,
    pressure_in: f64,
    precip_mm: f64,
    precip_in: f64,
    humidity: f64,
    cloud: f64,
    feelslike_c: f64,
    feelslike_f: f64,
    vis_km: f64,
    vis_miles: f64,
    uv: f64,
    gust_mph: f64,
    gust_kph: f64,
}

#[derive(Deserialize, Debug)]
struct Condition {
    text: String,
    icon: String,
    code: i64,
}

#[derive(Deserialize, Debug)]
struct Forecast {
    forecastday: Vec<ForecastDay>,
}

#[derive(Deserialize, Debug)]
struct ForecastDay {
    date: String,
    date_epoch: i64,
    day: Day,
    astro: Astro,
    hour: Vec<Hour>,
}

#[derive(Deserialize, Debug)]
struct Day {
    maxtemp_c: f64,
    maxtemp_f: f64,
    mintemp_c: f64,
    mintemp_f: f64,
    avgtemp_c: f64,
    avgtemp_f: f64,
    maxwind_mph: f64,
    maxwind_kph: f64,
    totalprecip_mm: f64,
    totalprecip_in: f64,
    avgvis_km: f64,
    avgvis_miles: f64,
    avghumidity: f64,
    daily_will_it_rain: i64,
    daily_chance_of_rain: i64,
    daily_will_it_snow: i64,
    daily_chance_of_snow: i64,
    condition: Condition,
    uv: f64,
}

#[derive(Deserialize, Debug)]
struct Astro {
    sunrise: String,
    sunset: String,
    moonrise: String,
    moonset: String,
    moon_phase: String,
    moon_illumination: i32,
}

#[derive(Deserialize, Debug)]
struct Hour {
    time_epoch: i64,
    time: String,
    temp_c: f64,
    temp_f: f64,
    is_day: i64,
    condition: Condition,
    wind_mph: f64,
    wind_kph: f64,
    wind_degree: i64,
    wind_dir: String,
    pressure_mb: f64,
    pressure_in: f64,
    precip_mm: f64,
    precip_in: f64,
    humidity: f64,
    cloud: f64,
    feelslike_c: f64,
    feelslike_f: f64,
    windchill_c: f64,
    windchill_f: f64,
    heatindex_c: f64,
    heatindex_f: f64,
    dewpoint_c: f64,
    dewpoint_f: f64,
    will_it_rain: i64,
    chance_of_rain: i64,
    will_it_snow: i64,
    chance_of_snow: i64,
    vis_km: f64,
    vis_miles: f64,
    gust_mph: f64,
    gust_kph: f64,
    uv: f64,
}

pub struct WeatherApi {
    client: Client,
    base_url: Url,
    key: String,
}

impl WeatherApi {
    pub fn new(key: String) -> Self {
        let base_url = Url::parse("https://api.weatherapi.com").unwrap();
        let client = reqwest::Client::builder().build().unwrap();

        WeatherApi {
            client,
            base_url,
            key,
        }
    }
}

#[async_trait]
impl Provider for WeatherApi {
    async fn get_forecast(&self, coord: weather::Coordinates, time: weather::ForecastTime) {
        let mut url = self.base_url.to_owned();
        url.set_path("/v1/forecast.json");

        let query = [
            ("q", format!("{},{}", coord.lat, coord.lon)), 
            ("key", self.key.to_string()),
            ("days", "5".to_string()),
            ("alerts", "yes".to_string()),
        ];
        
        let weather_data = self.client
            .get(url)
            .query(&query)
            .send()
            .await
            .unwrap()
            .json::<WeatherData>()
            .await
            .unwrap();

       println!("WeatherApi data: {:?}", weather_data);
    }
}

use crate::weather;

use serde_derive::Deserialize;
use async_trait::async_trait;
use reqwest::{self, Client};
use url::Url;

use super::Provider;

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    dt: u32,
    sunrise: Option<u32>,
    sunset: Option<u32>,
    temp: f32,
    feels_like: f32,
    pressure: f32,
    humidity: f32,
    dew_point: f32,
    clouds: f32,
    uvi: f32,
    visibility: f32,
    wind_speed: f32,
    wind_gust: Option<f32>,
    wind_deg: f32,
    rain: Option<Rain>,
    snow: Option<Snow>,
    weather: Vec<WeatherCondition>,
}

#[derive(Deserialize, Debug)]
struct Rain {
    #[serde(rename = "1h")]
    mm_h: f32,
}

#[derive(Deserialize, Debug)]
struct Snow {
    #[serde(rename = "1h")]
    mm_h: f32,
}

#[derive(Deserialize, Debug)]
struct WeatherCondition {
    id: u32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Deserialize, Debug)]
struct HourlyForecast {
    dt: u32,
    temp: f32,
    feels_like: f32,
    pressure: f32,
    humidity: f32,
    dew_point: f32,
    uvi: f32,
    clouds: f32,
    visibility: f32,
    wind_speed: f32,
    wind_gust: Option<f32>,
    wind_deg: f32,
    pop: f32,
    rain: Option<Rain>,
    snow: Option<Snow>,
    weather: Vec<WeatherCondition>,
}

#[derive(Deserialize, Debug)]
struct DailyForecast {
    dt: u32,
    sunrise: Option<u32>,
    sunset: Option<u32>,
    moonrise: Option<u32>,
    moonset: Option<u32>,
    moon_phase: f32,
    summary: String,
    temp: DailyTemperature,
    feels_like: DailyFeelsLike,
    pressure: f32,
    humidity: f32,
    dew_point: f32,
    wind_speed: f32,
    wind_gust: Option<f32>,
    wind_deg: f32,
    clouds: f32,
    uvi: f32,
    pop: f32,
    rain: Option<f32>,
    snow: Option<f32>,
    weather: Vec<WeatherCondition>,
}

#[derive(Deserialize, Debug)]
struct DailyTemperature {
    morn: f32,
    day: f32,
    eve: f32,
    night: f32,
    min: f32,
    max: f32,
}

#[derive(Deserialize, Debug)]
struct DailyFeelsLike {
    morn: f32,
    day: f32,
    eve: f32,
    night: f32,
}

#[derive(Deserialize, Debug)]
struct WeatherData {
    lat: f32,
    lon: f32,
    timezone: String,
    timezone_offset: u32,
    current: CurrentWeather,
    hourly: Vec<HourlyForecast>,
    daily: Vec<DailyForecast>,
}

pub struct OpenWeather<'a> {
    client: Client,
    base_url: Url,
    app_id: &'a str,
}

impl<'a> OpenWeather<'a> {
    pub fn new(app_id: &'a str) -> Self {
        let base_url = Url::parse("https://api.openweathermap.org").unwrap();
        let client = reqwest::Client::builder().build().unwrap();

        OpenWeather {
            client,
            base_url,
            app_id,
        }
    }
}

#[async_trait]
impl<'a> Provider for OpenWeather<'a> {
    async fn get_forecast(&self, coord: weather::Coordinates, time: weather::ForecastTime) {
        let mut url = self.base_url.to_owned();
        url.set_path("/data/3.0/onecall");

        let query = [
            ("lat", coord.lat.to_string()),
            ("lon", coord.lon.to_string()),
            ("appid", self.app_id.to_owned()),
            ("exclude", "minutely".to_string()),
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

       println!("OpenWeather data: {:?}", weather_data); 
    }
}

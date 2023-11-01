use crate::weather::{self};
use serde_derive::Deserialize;

use async_trait::async_trait;
use reqwest::{self, Client};
use url::Url;

use super::Provider;

#[derive(Deserialize, Debug)]
pub struct WeatherData {
    location: Location,
    current: Current,
    forecast: Forecast,
}

impl WeatherData {
    pub fn parse_to_current(self, unit: weather::UnitType) -> weather::CurrentWeather {
        let (sunset, sunrise) = self.forecast.forecastday.first().map_or((None, None), |d| {
            (
                Some(d.astro.sunrise.to_owned()),
                Some(d.astro.sunset.to_owned()),
            )
        });

        weather::CurrentWeather {
            temp: match unit {
                weather::UnitType::Metric => self.current.temp_c,
                weather::UnitType::Imperial => self.current.temp_f,
            },
            feels_like: match unit {
                weather::UnitType::Metric => self.current.feelslike_c,
                weather::UnitType::Imperial => self.current.feelslike_f,
            },
            visibility: match unit {
                weather::UnitType::Metric => self.current.vis_km,
                weather::UnitType::Imperial => self.current.vis_miles,
            },
            clouds: self.current.cloud,
            humidity: self.current.humidity,
            pressure: self.current.pressure_mb,
            wind_speed: match unit {
                weather::UnitType::Metric => self.current.vis_km / 3.6,
                weather::UnitType::Imperial => self.current.vis_miles,
            },
            wind_deg: self.current.wind_degree as f32,
            uvi: self.current.uv,
            sunrise,
            sunset,
            condition: self.current.condition.text,
            precip: Some(match unit {
                weather::UnitType::Metric => self.current.precip_mm,
                weather::UnitType::Imperial => self.current.precip_in,
            }),
            unit,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Location {
    name: String,
    region: String,
    country: String,
    lat: f32,
    lon: f32,
    tz_id: String,
    localtime_epoch: u32,
    localtime: String,
}

#[derive(Deserialize, Debug)]
pub struct Current {
    last_updated_epoch: u32,
    last_updated: String,
    temp_c: f32,
    temp_f: f32,
    is_day: u32,
    condition: Condition,
    wind_mph: f32,
    wind_kph: f32,
    wind_degree: u32,
    wind_dir: String,
    pressure_mb: f32,
    pressure_in: f32,
    precip_mm: f32,
    precip_in: f32,
    humidity: f32,
    cloud: f32,
    feelslike_c: f32,
    feelslike_f: f32,
    vis_km: f32,
    vis_miles: f32,
    uv: f32,
    gust_mph: f32,
    gust_kph: f32,
}

#[derive(Deserialize, Debug)]
pub struct Condition {
    text: String,
    icon: String,
    code: u32,
}

#[derive(Deserialize, Debug)]
pub struct Forecast {
    forecastday: Vec<ForecastDay>,
}

#[derive(Deserialize, Debug)]
pub struct ForecastDay {
    date: String,
    date_epoch: u32,
    day: Day,
    astro: Astro,
    hour: Vec<Hour>,
}

#[derive(Deserialize, Debug)]
pub struct Day {
    maxtemp_c: f32,
    maxtemp_f: f32,
    mintemp_c: f32,
    mintemp_f: f32,
    avgtemp_c: f32,
    avgtemp_f: f32,
    maxwind_mph: f32,
    maxwind_kph: f32,
    totalprecip_mm: f32,
    totalprecip_in: f32,
    avgvis_km: f32,
    avgvis_miles: f32,
    avghumidity: f32,
    daily_will_it_rain: u32,
    daily_chance_of_rain: u32,
    daily_will_it_snow: u32,
    daily_chance_of_snow: u32,
    condition: Condition,
    uv: f32,
}

#[derive(Deserialize, Debug)]
pub struct Astro {
    sunrise: String,
    sunset: String,
    moonrise: String,
    moonset: String,
    moon_phase: String,
    moon_illumination: u32,
}

#[derive(Deserialize, Debug)]
pub struct Hour {
    time_epoch: u32,
    time: String,
    temp_c: f32,
    temp_f: f32,
    is_day: u32,
    condition: Condition,
    wind_mph: f32,
    wind_kph: f32,
    wind_degree: u32,
    wind_dir: String,
    pressure_mb: f32,
    pressure_in: f32,
    precip_mm: f32,
    precip_in: f32,
    humidity: f32,
    cloud: f32,
    feelslike_c: f32,
    feelslike_f: f32,
    windchill_c: f32,
    windchill_f: f32,
    heatindex_c: f32,
    heatindex_f: f32,
    dewpoint_c: f32,
    dewpoint_f: f32,
    will_it_rain: u32,
    chance_of_rain: u32,
    will_it_snow: u32,
    chance_of_snow: u32,
    vis_km: f32,
    vis_miles: f32,
    gust_mph: f32,
    gust_kph: f32,
    uv: f32,
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
    async fn get_forecast(
        &self,
        coord: weather::Coordinates,
        time: weather::ForecastTime,
        unit: weather::UnitType,
    ) -> weather::Weather {
        let mut url = self.base_url.to_owned();
        url.set_path("/v1/forecast.json");

        let query = [
            ("q", format!("{},{}", coord.lat, coord.lon)),
            ("key", self.key.to_string()),
            ("days", parse_forecast_time(&time).to_string()),
            ("alerts", "yes".to_string()),
        ];

        let weather_data = self
            .client
            .get(url)
            .query(&query)
            .send()
            .await
            .unwrap()
            .json::<WeatherData>()
            .await
            .unwrap();

        match time {
            weather::ForecastTime::Now => {
                weather::Weather::Current(weather_data.parse_to_current(unit))
            }
            weather::ForecastTime::Hours24
            | weather::ForecastTime::Days3
            | weather::ForecastTime::Days5 => weather::Weather::Daily(vec![]),
        }
    }
}

fn parse_forecast_time(time: &weather::ForecastTime) -> usize {
    match time {
        weather::ForecastTime::Now | weather::ForecastTime::Hours24 => 1,
        weather::ForecastTime::Days3 => 3,
        weather::ForecastTime::Days5 => 5,
    }
}

use crate::weather;

use async_trait::async_trait;
use reqwest::{self, Client};
use serde_derive::Deserialize;
use url::Url;

use chrono::{self, DateTime, FixedOffset};

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

impl WeatherData {
    pub fn parse_to_current(self, unit: weather::UnitType) -> weather::CurrentWeather {
        let offset = FixedOffset::east_opt(self.timezone_offset as i32).unwrap();
        let mut sunrise: Option<String> = None;
        let mut sunset: Option<String> = None;

        if let Some(sr) = self.current.sunrise {
            let sunrise_dt = DateTime::from_timestamp(sr as i64, 0).unwrap().with_timezone(&offset);
            sunrise = Some(format!("{}", sunrise_dt.format("%H:%M")));
        }

        if let Some(ss) = self.current.sunset {
            let sunset_dt = DateTime::from_timestamp(ss as i64, 0).unwrap().with_timezone(&offset);
            sunset = Some(format!("{}", sunset_dt.format("%H:%M")));
        }
        
        let precip = self.current.rain.map(|r| r.mm_h);
        let condition = self
            .current
            .weather
            .first()
            .map_or("No data".to_string(), |w| w.description.clone());

        weather::CurrentWeather {
            temp: self.current.temp,
            feels_like: self.current.feels_like,
            visibility: self.current.visibility / 1000.0,
            clouds: self.current.clouds,
            humidity: self.current.humidity,
            pressure: self.current.pressure,
            wind_speed: self.current.wind_speed,
            wind_deg: self.current.wind_deg,
            uvi: self.current.uvi,
            sunrise,
            sunset,
            condition,
            precip,
            unit,
        }
    }
}

pub struct OpenWeather {
    client: Client,
    base_url: Url,
    app_id: String,
}

impl OpenWeather {
    pub fn new(app_id: String) -> Self {
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
impl Provider for OpenWeather {
    async fn get_forecast(
        &self,
        coord: weather::Coordinates,
        time: weather::ForecastTime,
        unit: weather::UnitType,
    ) -> weather::Weather {
        let mut url = self.base_url.to_owned();
        url.set_path("/data/3.0/onecall");

        let query = [
            ("lat", coord.lat.to_string()),
            ("lon", coord.lon.to_string()),
            ("appid", self.app_id.to_owned()),
            ("exclude", "minutely".to_string()),
            ("units", unit.to_string().to_lowercase()),
        ];

        let mut weather_data = self
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
                weather::Weather::Current(
                    weather_data.parse_to_current(unit)
                )
            }
            weather::ForecastTime::Hours24
            | weather::ForecastTime::Days3
            | weather::ForecastTime::Days5 => weather::Weather::Daily(vec![]),
        }
    }
}
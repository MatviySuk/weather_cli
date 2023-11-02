use crate::weather;

use async_trait::async_trait;
use reqwest::{self, Client};
use serde_derive::Deserialize;
use url::Url;

use chrono::{self, DateTime, FixedOffset};

use super::Provider;

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    dt: i64,
    sunrise: Option<i64>,
    sunset: Option<i64>,
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
    dt: i64,
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
    dt: i64,
    sunrise: Option<i64>,
    sunset: Option<i64>,
    moonrise: Option<i64>,
    moonset: Option<i64>,
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

        let precip = self.current.rain.map(|r| r.mm_h);
        let condition = self
            .current
            .weather
            .into_iter()
            .next()
            .map_or("No data".to_string(), |w| w.description);

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
            sunrise: datetime_to_str(self.current.sunrise, &offset, "%H:%M"),
            sunset: datetime_to_str(self.current.sunset, &offset, "%H:%M"),
            condition,
            precip,
            unit,
        }
    }

    pub fn parse_to_today(self, unit: weather::UnitType) -> Vec<weather::HourWeather> {
        let offset = FixedOffset::east_opt(self.timezone_offset as i32).unwrap();

        self.hourly
            .into_iter()
            .take(24)
            .map(|h| {
                let precip = h.rain.map(|r| r.mm_h);
                let condition = h
                    .weather
                    .into_iter()
                    .next()
                    .map_or("No data".to_string(), |w| w.description);

                weather::HourWeather {
                    time: datetime_to_str(Some(h.dt), &offset, "%Y-%m-%d %H:%M").unwrap(),
                    temp: h.temp,
                    feels_like: h.feels_like,
                    visibility: h.visibility / 1000.0,
                    clouds: h.clouds,
                    humidity: h.humidity,
                    pressure: h.pressure,
                    wind_speed: h.wind_speed,
                    wind_deg: h.wind_deg,
                    uvi: h.uvi,
                    condition,
                    precip,
                    unit: unit.clone(),
                }
            })
            .collect::<Vec<weather::HourWeather>>()
    }

    pub fn parse_to_days(
        self,
        n_days: usize,
        unit: weather::UnitType,
    ) -> Vec<weather::DailyWeather> {
        let offset = FixedOffset::east_opt(self.timezone_offset as i32).unwrap();

        self.daily
            .into_iter()
            .take(n_days)
            .map(|d| {
                let condition = d
                    .weather
                    .into_iter()
                    .next()
                    .map_or("No data".to_string(), |w| w.description);

                weather::DailyWeather {
                    date: datetime_to_str(Some(d.dt), &offset, "%Y-%m-%d").unwrap(),
                    min_temp: d.temp.min,
                    max_temp: d.temp.max,
                    avg_temp: None,
                    visibility: None,
                    humidity: d.humidity,
                    pressure: Some(d.pressure),
                    wind_speed: d.wind_speed,
                    uvi: d.uvi,
                    condition,
                    precip: d.rain,
                    clouds: Some(d.clouds),
                    sunrise: datetime_to_str(d.sunrise, &offset, "%H:%M"),
                    sunset: datetime_to_str(d.sunset, &offset, "%H:%M"),
                    moonrise: datetime_to_str(d.moonrise, &offset, "%H:%M"),
                    moonset: datetime_to_str(d.moonset, &offset, "%H:%M"),
                    moon_phase: Some(format!("{:.2}", d.moon_phase)),
                    unit: unit.clone(),
                }
            })
            .collect::<Vec<weather::DailyWeather>>()
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
            weather::ForecastTime::Hours24 => {
                weather::Weather::Today(weather_data.parse_to_today(unit))
            }
            weather::ForecastTime::Days3 => {
                weather::Weather::Daily(weather_data.parse_to_days(3, unit))
            }
            weather::ForecastTime::Days5 => {
                weather::Weather::Daily(weather_data.parse_to_days(5, unit))
            }
        }
    }
}

fn datetime_to_str<'a>(dt: Option<i64>, offset: &FixedOffset, fmt: &'a str) -> Option<String> {
    dt.map_or(None, |dt| {
        Some(
            DateTime::from_timestamp(dt, 0)
                .unwrap_or_default()
                .with_timezone(offset)
                .format(fmt)
                .to_string(),
        )
    })
}

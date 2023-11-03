use crate::{errors::AppError, weather, Result};

use async_trait::async_trait;
use reqwest::{self, Client};
use serde_derive::Deserialize;
use url::Url;

use chrono::{self, DateTime, FixedOffset};

use super::Provider;

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    sunrise: Option<i64>,
    sunset: Option<i64>,
    temp: f32,
    feels_like: f32,
    pressure: f32,
    humidity: f32,
    clouds: f32,
    uvi: f32,
    visibility: f32,
    wind_speed: f32,
    wind_deg: f32,
    rain: Option<Rain>,
    weather: Vec<WeatherCondition>,
}

#[derive(Deserialize, Debug)]
struct Rain {
    #[serde(rename = "1h")]
    mm_h: f32,
}

#[derive(Deserialize, Debug)]
struct WeatherCondition {
    description: String,
}

#[derive(Deserialize, Debug)]
struct HourlyForecast {
    dt: i64,
    temp: f32,
    feels_like: f32,
    pressure: f32,
    humidity: f32,
    uvi: f32,
    clouds: f32,
    visibility: f32,
    wind_speed: f32,
    wind_deg: f32,
    rain: Option<Rain>,
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
    temp: DailyTemperature,
    pressure: f32,
    humidity: f32,
    wind_speed: f32,
    clouds: f32,
    uvi: f32,
    rain: Option<f32>,
    weather: Vec<WeatherCondition>,
}

#[derive(Deserialize, Debug)]
struct DailyTemperature {
    min: f32,
    max: f32,
}

#[derive(Deserialize, Debug)]
struct WeatherData {
    timezone_offset: u32,
    current: CurrentWeather,
    hourly: Vec<HourlyForecast>,
    daily: Vec<DailyForecast>,
}

impl WeatherData {
    pub fn parse_to_current(self, unit: weather::UnitType) -> Result<weather::CurrentWeather> {
        let offset = FixedOffset::east_opt(self.timezone_offset as i32);

        let precip = self.current.rain.map(|r| r.mm_h);
        let condition = self
            .current
            .weather
            .into_iter()
            .next()
            .map_or("No data".to_string(), |w| w.description);

        Ok(weather::CurrentWeather {
            temp: self.current.temp,
            feels_like: self.current.feels_like,
            visibility: self.current.visibility / 1000.0,
            clouds: self.current.clouds,
            humidity: self.current.humidity,
            pressure: self.current.pressure,
            wind_speed: self.current.wind_speed,
            wind_deg: self.current.wind_deg,
            uvi: self.current.uvi,
            sunrise: Some(datetime_to_str(
                self.current.sunrise,
                offset.as_ref(),
                "%H:%M",
            )?),
            sunset: Some(datetime_to_str(
                self.current.sunset,
                offset.as_ref(),
                "%H:%M",
            )?),
            condition,
            precip,
            unit,
        })
    }

    pub fn parse_to_today(self, unit: weather::UnitType) -> Result<Vec<weather::HourWeather>> {
        let offset = FixedOffset::east_opt(self.timezone_offset as i32);

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

                Ok(weather::HourWeather {
                    time: datetime_to_str(Some(h.dt), offset.as_ref(), "%Y-%m-%d %H:%M")?,
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
                })
            })
            .collect::<Result<Vec<weather::HourWeather>>>()
    }

    pub fn parse_to_days(
        self,
        n_days: usize,
        unit: weather::UnitType,
    ) -> Result<Vec<weather::DailyWeather>> {
        let offset = FixedOffset::east_opt(self.timezone_offset as i32);

        self.daily
            .into_iter()
            .take(n_days)
            .map(|d| {
                let condition = d
                    .weather
                    .into_iter()
                    .next()
                    .map_or("No data".to_string(), |w| w.description);

                Ok(weather::DailyWeather {
                    date: datetime_to_str(Some(d.dt), offset.as_ref(), "%Y-%m-%d")?,
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
                    sunrise: Some(datetime_to_str(d.sunrise, offset.as_ref(), "%H:%M")?),
                    sunset: Some(datetime_to_str(d.sunset, offset.as_ref(), "%H:%M")?),
                    moonrise: Some(datetime_to_str(d.moonrise, offset.as_ref(), "%H:%M")?),
                    moonset: Some(datetime_to_str(d.moonset, offset.as_ref(), "%H:%M")?),
                    moon_phase: Some(format!("{:.2}", d.moon_phase)),
                    unit: unit.clone(),
                })
            })
            .collect::<Result<Vec<weather::DailyWeather>>>()
    }
}

pub struct OpenWeather {
    client: Client,
    base_url: Url,
    app_id: String,
}

impl OpenWeather {
    pub fn new(app_id: String) -> Result<Self> {
        let base_url = Url::parse("https://api.openweathermap.org")?;
        let client = reqwest::Client::builder().build()?;

        Ok(OpenWeather {
            client,
            base_url,
            app_id,
        })
    }
}

#[async_trait]
impl Provider for OpenWeather {
    async fn get_forecast(
        &self,
        coord: weather::Coordinates,
        time: weather::ForecastTime,
        unit: weather::UnitType,
    ) -> Result<weather::Weather> {
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
            .await?
            .json::<WeatherData>()
            .await?;

        Ok(match time {
            weather::ForecastTime::Now => {
                weather::Weather::Current(weather_data.parse_to_current(unit)?)
            }
            weather::ForecastTime::Hours24 => {
                weather::Weather::Today(weather_data.parse_to_today(unit)?)
            }
            weather::ForecastTime::Days3 => {
                weather::Weather::Daily(weather_data.parse_to_days(3, unit)?)
            }
            weather::ForecastTime::Days5 => {
                weather::Weather::Daily(weather_data.parse_to_days(5, unit)?)
            }
        })
    }
}

fn datetime_to_str(dt: Option<i64>, offset: Option<&FixedOffset>, fmt: &str) -> Result<String> {
    let datetime = dt.ok_or(AppError::TimeParse(
        "Failed to get UTC timestamp".to_string(),
    ))?;
    let offset = offset.ok_or(AppError::TimeParse(
        "Failed to parse provided UTC offset".to_string(),
    ))?;

    Ok(DateTime::from_timestamp(datetime, 0)
        .ok_or(AppError::TimeParse(format!(
            "Failed to parse provided timestamp: {} to DateTime.",
            datetime
        )))?
        .with_timezone(offset)
        .format(fmt)
        .to_string())
}

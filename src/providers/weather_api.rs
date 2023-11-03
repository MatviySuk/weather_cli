use crate::{weather, Result};
use serde_derive::Deserialize;

use async_trait::async_trait;
use reqwest::{self, Client};
use url::Url;

use chrono;

use super::Provider;

#[derive(Deserialize, Debug)]
pub struct WeatherData {
    current: Current,
    forecast: Forecast,
}

impl WeatherData {
    pub fn parse_to_current(self, unit: weather::UnitType) -> weather::CurrentWeather {
        let (sunset, sunrise) = self
            .forecast
            .forecastday
            .into_iter()
            .next()
            .map_or((None, None), |d| {
                (Some(d.astro.sunrise), Some(d.astro.sunset))
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
                weather::UnitType::Metric => self.current.wind_kph / 3.6,
                weather::UnitType::Imperial => self.current.wind_mph,
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

    pub fn parse_to_today(self, unit: weather::UnitType) -> Vec<weather::HourWeather> {
        let current_utc = chrono::Utc::now().timestamp();

        self.forecast
            .forecastday
            .into_iter()
            .flat_map(|d| d.hour)
            // Get current hour and next 23
            .filter(|h| h.time_epoch - current_utc > -3600)
            .take(24)
            .map(|h| weather::HourWeather {
                time: h.time.clone(),
                temp: match unit {
                    weather::UnitType::Metric => h.temp_c,
                    weather::UnitType::Imperial => h.temp_f,
                },
                feels_like: match unit {
                    weather::UnitType::Metric => h.feelslike_c,
                    weather::UnitType::Imperial => h.feelslike_f,
                },
                visibility: match unit {
                    weather::UnitType::Metric => h.vis_km,
                    weather::UnitType::Imperial => h.vis_miles,
                },
                clouds: h.cloud,
                humidity: h.humidity,
                pressure: h.pressure_mb,
                wind_speed: match unit {
                    weather::UnitType::Metric => h.wind_kph / 3.6,
                    weather::UnitType::Imperial => h.wind_mph,
                },
                wind_deg: h.wind_degree as f32,
                uvi: h.uv,
                condition: h.condition.text,
                precip: Some(match unit {
                    weather::UnitType::Metric => h.precip_mm,
                    weather::UnitType::Imperial => h.precip_in,
                }),
                unit: unit.to_owned(),
            })
            .collect::<Vec<weather::HourWeather>>()
    }

    pub fn parse_to_days(
        self,
        n_days: usize,
        unit: weather::UnitType,
    ) -> Vec<weather::DailyWeather> {
        self.forecast
            .forecastday
            .into_iter()
            .take(n_days)
            .map(|d| weather::DailyWeather {
                date: d.date,
                min_temp: match unit {
                    weather::UnitType::Metric => d.day.mintemp_c,
                    weather::UnitType::Imperial => d.day.mintemp_f,
                },
                max_temp: match unit {
                    weather::UnitType::Metric => d.day.maxtemp_c,
                    weather::UnitType::Imperial => d.day.maxtemp_f,
                },
                avg_temp: None,
                visibility: None,
                humidity: d.day.avghumidity,
                pressure: None,
                wind_speed: match unit {
                    weather::UnitType::Metric => d.day.maxwind_kph / 3.6,
                    weather::UnitType::Imperial => d.day.maxwind_mph,
                },
                uvi: d.day.uv,
                condition: d.day.condition.text,
                precip: Some(match unit {
                    weather::UnitType::Metric => d.day.totalprecip_mm,
                    weather::UnitType::Imperial => d.day.totalprecip_in,
                }),
                clouds: None,
                sunrise: Some(d.astro.sunrise),
                sunset: Some(d.astro.sunset),
                moonrise: Some(d.astro.moonrise),
                moonset: Some(d.astro.moonset),
                moon_phase: Some(d.astro.moon_phase),
                unit: unit.clone(),
            })
            .collect::<Vec<weather::DailyWeather>>()
    }
}

#[derive(Deserialize, Debug)]
pub struct Current {
    temp_c: f32,
    temp_f: f32,
    condition: Condition,
    wind_mph: f32,
    wind_kph: f32,
    wind_degree: u32,
    pressure_mb: f32,
    precip_mm: f32,
    precip_in: f32,
    humidity: f32,
    cloud: f32,
    feelslike_c: f32,
    feelslike_f: f32,
    vis_km: f32,
    vis_miles: f32,
    uv: f32,
}

#[derive(Deserialize, Debug)]
pub struct Condition {
    text: String,
}

#[derive(Deserialize, Debug)]
pub struct Forecast {
    forecastday: Vec<ForecastDay>,
}

#[derive(Deserialize, Debug)]
pub struct ForecastDay {
    date: String,
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
    maxwind_mph: f32,
    maxwind_kph: f32,
    totalprecip_mm: f32,
    totalprecip_in: f32,
    avghumidity: f32,
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
}

#[derive(Deserialize, Debug)]
pub struct Hour {
    time_epoch: i64,
    time: String,
    temp_c: f32,
    temp_f: f32,
    condition: Condition,
    wind_mph: f32,
    wind_kph: f32,
    wind_degree: u32,
    pressure_mb: f32,
    precip_mm: f32,
    precip_in: f32,
    humidity: f32,
    cloud: f32,
    feelslike_c: f32,
    feelslike_f: f32,
    vis_km: f32,
    vis_miles: f32,
    uv: f32,
}

pub struct WeatherApi {
    client: Client,
    base_url: Url,
    key: String,
}

impl WeatherApi {
    pub fn new(key: String) -> Result<Self> {
        let base_url = Url::parse("https://api.weatherapi.com")?;
        let client = reqwest::Client::builder().build()?;

        Ok(WeatherApi {
            client,
            base_url,
            key,
        })
    }
}

#[async_trait]
impl Provider for WeatherApi {
    async fn get_forecast(
        &self,
        coord: weather::Coordinates,
        time: weather::ForecastTime,
        unit: weather::UnitType,
    ) -> Result<weather::Weather> {
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
            .await?
            .json::<WeatherData>()
            .await?;

        Ok(match time {
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
        })
    }
}

fn parse_forecast_time(time: &weather::ForecastTime) -> usize {
    match time {
        weather::ForecastTime::Now => 1,
        weather::ForecastTime::Hours24 => 2,
        weather::ForecastTime::Days3 => 3,
        weather::ForecastTime::Days5 => 5,
    }
}

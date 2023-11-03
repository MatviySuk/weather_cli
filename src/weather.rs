use core::fmt;
use std::hash::Hash;

use crate::Result;
use clap::{Args, Subcommand, ValueEnum};
use serde_derive::{Deserialize, Serialize};

pub enum Weather {
    Current(CurrentWeather),
    Today(Vec<HourWeather>),
    Daily(Vec<DailyWeather>),
}
#[derive(Debug)]
pub struct DailyWeather {
    pub date: String,
    pub min_temp: f32,
    pub max_temp: f32,
    pub avg_temp: Option<f32>,
    pub visibility: Option<f32>,
    pub humidity: f32,
    pub pressure: Option<f32>,
    pub wind_speed: f32,
    pub uvi: f32,
    pub clouds: Option<f32>,
    pub condition: String,
    pub precip: Option<f32>,
    pub sunrise: Option<String>,
    pub sunset: Option<String>,
    pub moonrise: Option<String>,
    pub moonset: Option<String>,
    pub moon_phase: Option<String>,
    pub unit: UnitType,
}

impl fmt::Display for DailyWeather {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (temp_unit, speed_unit, vis_unit) = match self.unit {
            UnitType::Metric => ("°C", "m/s", "km"),
            UnitType::Imperial => ("°F", "mph", "mi"),
        };

        writeln!(f, "Daily Weather for {}:", self.date)?;
        writeln!(f, "Min Temp: {:.2}{}", self.min_temp, temp_unit)?;
        writeln!(f, "Max Temp: {:.2}{}", self.max_temp, temp_unit)?;

        if let Some(avg_temp) = self.avg_temp {
            writeln!(f, "Avg Temp: {:.2}{}", avg_temp, temp_unit)?;
        }

        if let Some(visibility) = self.visibility {
            writeln!(f, "Visibility: {:.2} {}", visibility, vis_unit)?;
        }

        writeln!(f, "Humidity: {:.2}%", self.humidity)?;

        if let Some(pressure) = self.pressure {
            writeln!(f, "Pressure: {:.2} hPa", pressure)?;
        }

        writeln!(f, "Wind Speed: {:.2} {}", self.wind_speed, speed_unit)?;
        writeln!(f, "UV Index: {:.2}", self.uvi)?;

        if let Some(clouds) = self.clouds {
            writeln!(f, "Clouds: {:.2}%", clouds)?;
        }

        writeln!(f, "Condition: {}", self.condition)?;

        if let Some(precip) = self.precip {
            writeln!(f, "Precipitation: {:.2} mm", precip)?;
        }

        if let Some(sunrise) = &self.sunrise {
            writeln!(f, "Sunrise: {}", sunrise)?;
        }

        if let Some(sunset) = &self.sunset {
            writeln!(f, "Sunset: {}", sunset)?;
        }

        if let Some(moonrise) = &self.moonrise {
            writeln!(f, "Moonrise: {}", moonrise)?;
        }

        if let Some(moonset) = &self.moonset {
            writeln!(f, "Moonset: {}", moonset)?;
        }

        if let Some(moon_phase) = self.moon_phase.as_deref() {
            writeln!(f, "Moon Phase: {}", moon_phase)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct HourWeather {
    pub time: String,
    pub temp: f32,
    pub feels_like: f32,
    pub visibility: f32,
    pub clouds: f32,
    pub humidity: f32,
    pub pressure: f32,
    pub wind_speed: f32,
    pub wind_deg: f32,
    pub uvi: f32,
    pub condition: String,
    pub precip: Option<f32>,
    pub unit: UnitType,
}

impl fmt::Display for HourWeather {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (temp_unit, speed_unit, vis_unit) = match self.unit {
            UnitType::Metric => ("°C", "m/s", "km"),
            UnitType::Imperial => ("°F", "mph", "mi"),
        };

        writeln!(f, "Time: {}", self.time)?;
        writeln!(f, "Temperature: {:.2} {}", self.temp, temp_unit)?;
        writeln!(f, "Feels Like: {:.2} {}", self.feels_like, temp_unit)?;
        writeln!(f, "Visibility: {:.2} {}", self.visibility, vis_unit)?;
        writeln!(f, "Clouds: {:.2}%", self.clouds)?;
        writeln!(f, "Humidity: {:.2}%", self.humidity)?;
        writeln!(f, "Pressure: {:.2} hPa", self.pressure)?;
        writeln!(f, "Wind Speed: {:.2} {}", self.wind_speed, speed_unit)?;
        writeln!(f, "Wind Direction: {}", cardinal_dir_from(self.wind_deg))?;
        writeln!(f, "UV Index: {:.2}", self.uvi)?;
        writeln!(f, "Condition: {}", self.condition)?;

        if let Some(precip) = self.precip {
            let precip_unit = match self.unit {
                UnitType::Metric => "mm",
                UnitType::Imperial => "inches",
            };
            writeln!(f, "Precipitation: {:.2} {}", precip, precip_unit)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct CurrentWeather {
    pub temp: f32,
    pub feels_like: f32,
    pub visibility: f32,
    pub clouds: f32,
    pub humidity: f32,
    pub pressure: f32,
    pub wind_speed: f32,
    pub wind_deg: f32,
    pub uvi: f32,
    pub sunrise: Option<String>,
    pub sunset: Option<String>,
    pub condition: String,
    pub precip: Option<f32>,
    pub unit: UnitType,
}

impl fmt::Display for CurrentWeather {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (temp_unit, speed_unit, vis_unit) = match self.unit {
            UnitType::Metric => ("°C", "m/s", "km"),
            UnitType::Imperial => ("°F", "mph", "mi"),
        };

        writeln!(f, "Current Weather:")?;
        writeln!(f, "Temperature: {:.2} {}", self.temp, temp_unit)?;
        writeln!(f, "Feels Like: {:.2} {}", self.feels_like, temp_unit)?;
        writeln!(f, "Visibility: {:.2} {}", self.visibility, vis_unit)?;
        writeln!(f, "Clouds: {:.2}%", self.clouds)?;
        writeln!(f, "Humidity: {:.2}%", self.humidity)?;
        writeln!(f, "Pressure: {:.2} hPa", self.pressure)?;
        writeln!(f, "Wind Speed: {:.2} {}", self.wind_speed, speed_unit)?;
        writeln!(f, "Wind Direction: {}", cardinal_dir_from(self.wind_deg))?;
        writeln!(f, "UV Index: {:.2}", self.uvi)?;

        if let Some(sunrise) = &self.sunrise {
            writeln!(f, "Sunrise: {}", sunrise)?;
        }

        if let Some(sunset) = &self.sunset {
            writeln!(f, "Sunset: {}", sunset)?;
        }

        writeln!(f, "Condition: {}", self.condition)?;

        if let Some(precip) = self.precip {
            let precip_unit = match self.unit {
                UnitType::Metric => "mm",
                UnitType::Imperial => "inches",
            };
            writeln!(f, "Precipitation: {:.2} {}", precip, precip_unit)?;
        }

        Ok(())
    }
}

#[derive(Deserialize, Serialize, Subcommand, Clone, Debug)]
pub enum Provider {
    OpenWeather(ProviderCredentials),
    WeatherApi(ProviderCredentials),
}

impl fmt::Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Provider::OpenWeather(_) => write!(f, "Open Weather"),
            Provider::WeatherApi(_) => write!(f, "Weather API"),
        }
    }
}

#[derive(Deserialize, Serialize, Args, Clone, Debug)]
pub struct ProviderCredentials {
    #[arg(short, long)]
    pub key: String,
}

#[derive(Subcommand, Clone, Debug)]
pub enum PlacesAction {
    /// Get all the saved places
    GetAll,

    /// Save the new place or update the location of existed place by tag
    Set(Place),

    /// Remove the place if it is present
    Remove(PlaceTag),
}

#[derive(Deserialize, Serialize, Args, Clone, Debug)]
pub struct Place {
    /// Tag or name of the place
    #[command(flatten)]
    pub tag: PlaceTag,

    /// Geodetic coordinate
    #[command(flatten)]
    pub coordinates: Coordinates,
}

impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Place: {}", self.tag.tag)?;
        writeln!(
            f,
            "Coordinates: (lat: {}, lon: {})",
            self.coordinates.lat, self.coordinates.lon
        )
    }
}

impl Hash for Place {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tag.hash(state);
    }
}

impl PartialEq for Place {
    fn eq(&self, other: &Self) -> bool {
        self.tag == other.tag
    }
}

impl Eq for Place {}

#[derive(Deserialize, Serialize, PartialEq, Hash, Eq, Args, Clone, Debug)]
pub struct PlaceTag {
    /// Tag or name of the place
    #[arg(short, long)]
    pub tag: String,
}

#[derive(Deserialize, Serialize, Args, Clone, Debug)]
pub struct Coordinates {
    /// Geodetic latitude of the location.
    /// Latitude must be between -90 and 90 degrees including
    #[arg(long = "lat")]
    pub lat: f32,

    /// Geodetic longitude of the location.
    /// Longitude must be between -180 and 180 degrees including
    #[arg(long = "lon")]
    pub lon: f32,
}

impl Coordinates {
    pub fn validate(&self) -> Result<()> {
        if !(-90.0f32..=90.0).contains(&self.lat) {
            return Err(crate::errors::AppError::Coordinates(
                crate::errors::CoordinatesError::Latitude(self.lat),
            ));
        }

        if !(-180.0f32..=180.0).contains(&self.lon) {
            return Err(crate::errors::AppError::Coordinates(
                crate::errors::CoordinatesError::Longitude(self.lon),
            ));
        }

        Ok(())
    }
}

#[derive(Subcommand, Clone, Debug)]
pub enum Location {
    /// Tag of the place saved to frequently used
    Place(PlaceTag),

    /// Geodetic coordinate
    Coordinates(Coordinates),
}

#[derive(ValueEnum, Clone, Debug)]
pub enum ForecastTime {
    Now,
    Hours24,
    Days3,
    Days5,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum UnitType {
    Metric,
    Imperial,
}

impl fmt::Display for UnitType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Metric => write!(f, "Metric"),
            Self::Imperial => write!(f, "Imperial"),
        }
    }
}

fn cardinal_dir_from(degree: f32) -> String {
    match degree as u32 % 360 {
        0..=22 => "North",
        23..=67 => "Northeast",
        68..=112 => "East",
        113..=157 => "Southeast",
        158..=202 => "South",
        203..=247 => "Southwest",
        248..=292 => "West",
        293..=337 => "Northwest",
        338..=360 => "North",
        _ => unreachable!("Impossible degree to cardinal direction value!"),
    }
    .to_string()
}

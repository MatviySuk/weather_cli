use core::fmt;
use std::hash::Hash;

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
        
        write!(f, "Daily Weather for {}:\n", self.date)?;
        write!(f, "Min Temp: {:.2}{}\n", self.min_temp, temp_unit)?;
        write!(f, "Max Temp: {:.2}{}\n", self.max_temp, temp_unit)?;
        
        if let Some(avg_temp) = self.avg_temp {
            write!(f, "Avg Temp: {:.2}{}\n", avg_temp, temp_unit)?;
        }
        
        if let Some(visibility) = self.visibility {
            write!(f, "Visibility: {:.2} {}\n", visibility, vis_unit)?;
        }
        
        write!(f, "Humidity: {:.2}%\n", self.humidity)?;
        
        if let Some(pressure) = self.pressure {
            write!(f, "Pressure: {:.2} hPa\n", pressure)?;
        }
        
        write!(f, "Wind Speed: {:.2} {}\n", self.wind_speed, speed_unit)?;
        write!(f, "UV Index: {:.2}\n", self.uvi)?;
        
        if let Some(clouds) = self.clouds {
            write!(f, "Clouds: {:.2}%\n", clouds)?;
        }
        
        write!(f, "Condition: {}\n", self.condition)?;
        
        if let Some(precip) = self.precip {
            write!(f, "Precipitation: {:.2} mm\n", precip)?;
        }
        
        if let Some(sunrise) = &self.sunrise {
            write!(f, "Sunrise: {}\n", sunrise)?;
        }
        
        if let Some(sunset) = &self.sunset {
            write!(f, "Sunset: {}\n", sunset)?;
        }
        
        if let Some(moonrise) = &self.moonrise {
            write!(f, "Moonrise: {}\n", moonrise)?;
        }
        
        if let Some(moonset) = &self.moonset {
            write!(f, "Moonset: {}\n", moonset)?;
        }
        
        if let Some(moon_phase) = self.moon_phase.as_deref() {
            write!(f, "Moon Phase: {}\n", moon_phase)?;
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

        write!(f, "Time: {}\n", self.time)?;
        write!(f, "Temperature: {:.2} {}\n", self.temp, temp_unit)?;
        write!(f, "Feels Like: {:.2} {}\n", self.feels_like, temp_unit)?;
        write!(f, "Visibility: {:.2} {}\n", self.visibility, vis_unit)?;
        write!(f, "Clouds: {:.2}%\n", self.clouds)?;
        write!(f, "Humidity: {:.2}%\n", self.humidity)?;
        write!(f, "Pressure: {:.2} hPa\n", self.pressure)?;
        write!(f, "Wind Speed: {:.2} {}\n", self.wind_speed, speed_unit)?;
        write!(f, "Wind Direction: {}\n", cardinal_dir_from(self.wind_deg))?;
        write!(f, "UV Index: {:.2}\n", self.uvi)?;
        write!(f, "Condition: {}\n", self.condition)?;

        if let Some(precip) = self.precip {
            let precip_unit = match self.unit {
                UnitType::Metric => "mm",
                UnitType::Imperial => "inches",
            };
            write!(f, "Precipitation: {:.2} {}\n", precip, precip_unit)?;
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

        write!(f, "Current Weather:\n")?;
        write!(f, "Temperature: {:.2} {}\n", self.temp, temp_unit)?;
        write!(f, "Feels Like: {:.2} {}\n", self.feels_like, temp_unit)?;
        write!(f, "Visibility: {:.2} {}\n", self.visibility, vis_unit)?;
        write!(f, "Clouds: {:.2}%\n", self.clouds)?;
        write!(f, "Humidity: {:.2}%\n", self.humidity)?;
        write!(f, "Pressure: {:.2} hPa\n", self.pressure)?;
        write!(f, "Wind Speed: {:.2} {}\n", self.wind_speed, speed_unit)?;
        write!(f, "Wind Direction: {}\n", cardinal_dir_from(self.wind_deg))?;
        write!(f, "UV Index: {:.2}\n", self.uvi)?;

        if let Some(sunrise) = &self.sunrise {
            write!(f, "Sunrise: {}\n", sunrise)?;
        }

        if let Some(sunset) = &self.sunset {
            write!(f, "Sunset: {}\n", sunset)?;
        }

        write!(f, "Condition: {}\n", self.condition)?;

        if let Some(precip) = self.precip {
            let precip_unit = match self.unit {
                UnitType::Metric => "mm",
                UnitType::Imperial => "inches",
            };
            write!(f, "Precipitation: {:.2} {}\n", precip, precip_unit)?;
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
        write!(f, "Place: {}\n", self.tag.tag)?;
        write!(
            f,
            "Coordinates: (lat: {}, lon: {})\n",
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
    /// Value must be between -90 and 90 degrees including
    #[arg(long = "lat")]
    pub lat: f32,

    /// Geodetic longitude of the location.
    /// Value must be between -180 and 180 degrees including
    #[arg(long = "lon")]
    pub lon: f32,
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

use std::hash::Hash;

use clap::{Subcommand, Args, ValueEnum};
use serde_derive::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Subcommand, Clone, Debug)]
pub enum Provider {
    OpenWeather(ProviderCredentials),
    WeatherApi(ProviderCredentials),
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
    Remove(Place),
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

impl Eq for Place { }

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
    Today,
    Tomorrow,
    Days5,
}
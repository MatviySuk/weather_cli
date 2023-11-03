use std::collections::HashSet;

use crate::{
    weather::{Place, PlaceTag, Provider},
    Result,
};
use confy::{load, store};
use serde_derive::{Deserialize, Serialize};

const APP_NAME: &str = "weather";
const CONFIG_NAME: &str = "weather_config";

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct WeatherConfig {
    pub provider: Option<Provider>,
    pub places: HashSet<Place>,
}

impl WeatherConfig {
    pub fn get() -> Result<Self> {
        Ok(load(APP_NAME, CONFIG_NAME)?)
    }

    pub fn save(&self) -> Result<()> {
        Ok(store(APP_NAME, CONFIG_NAME, self)?)
    }

    pub fn place_by_tag(&self, tag: &PlaceTag) -> Option<Place> {
        self.places.iter().find(|p| p.tag == *tag).cloned()
    }
}

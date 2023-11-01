use std::collections::HashSet;

use crate::weather::{Place, PlaceTag, Provider};
use confy::{load, store};
use serde_derive::{Deserialize, Serialize};

const APP_NAME: &'static str = "weather";
const CONFIG_NAME: &'static str = "weather_config";

#[derive(Deserialize, Serialize, Debug)]
pub struct WeatherConfig {
    pub provider: Option<Provider>,
    pub places: HashSet<Place>,
}

impl Default for WeatherConfig {
    fn default() -> Self {
        WeatherConfig {
            provider: None,
            places: HashSet::new(),
        }
    }
}

impl WeatherConfig {
    pub fn get() -> Self {
        load(APP_NAME, CONFIG_NAME).unwrap()
    }

    pub fn save(&self) {
        store(APP_NAME, CONFIG_NAME, self).unwrap();
    }

    pub fn place_by_tag(&self, tag: &PlaceTag) -> Option<Place> {
        self.places
            .iter()
            .find(|p| p.tag == *tag)
            .cloned()
    }
}

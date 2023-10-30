use confy::{load, store};
use serde_derive::{Deserialize, Serialize};
use crate::weather::{Provider, ProviderCredentials};

const APP_NAME: &'static str = "weather";
const CONFIG_NAME: &'static str = "weather_config";

#[derive(Deserialize, Serialize, Debug)]
pub struct WeatherConfig {
    pub provider: Option<Provider>,
    pub places: Vec<String>,
}

impl Default for WeatherConfig {
    fn default() -> Self {
        WeatherConfig { provider: None, places: vec![] }
    }
}

impl WeatherConfig {
    pub fn get() -> Self {
        load(APP_NAME, CONFIG_NAME).unwrap()
    }
    
    pub fn save(&self) {
        store(APP_NAME, CONFIG_NAME, self).unwrap();
    }
}
use crate::weather::{Coordinates, ForecastTime};
use async_trait::async_trait;

pub mod open_weather;
pub mod weather_api;

#[async_trait]
pub trait Provider {
    async fn get_forecast(&self, coord: Coordinates, time: ForecastTime);
}

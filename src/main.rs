use crate::{
    cli::Cli,
    providers::{open_weather, weather_api, Provider},
    weather::{Coordinates, ForecastTime},
};

use clap::Parser;

pub mod cli;
pub mod providers;
pub mod weather;

#[tokio::main]
async fn main() {
    Cli::parse();
    
    let api: Vec<Box<dyn Provider>> = vec![
        Box::new(open_weather::OpenWeather::new(
            "2801cbb268e05f04d6dcfa32639f3123",
        )),
        Box::new(weather_api::WeatherApi::new(
            "c1f2a19fcbd049b790c125000233010",
        )),
    ];

    for provider in api {
        provider
            .get_forecast(
                Coordinates {
                    lat: 49.84,
                    lon: 24.03,
                },
                ForecastTime::Days5,
            )
            .await;
    }
}

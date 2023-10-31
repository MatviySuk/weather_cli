use clap::{Args, Parser, Subcommand};

use crate::{
    config::WeatherConfig,
    providers::{self, open_weather, weather_api},
    weather::*,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    operation: Operation,
}

impl Cli {
    pub async fn process(self) {
        match self.operation {
            Operation::Configure { provider } => configure_provider(provider),
            Operation::Places { action } => manage_places(action),
            Operation::Forecast(args) => get_forecast(args).await,
        }
    }
}

#[derive(Subcommand, Debug)]
enum Operation {
    /// Configure provider and credentials
    Configure {
        #[command(subcommand)]
        provider: Provider,
    },

    /// Manage frequently used locations
    Places {
        #[command(subcommand)]
        action: PlacesAction,
    },

    /// Get a weather forecast for the specific location
    Forecast(ForecastArgs),
}

#[derive(Args, Clone, Debug)]
struct ForecastArgs {
    /// Location to obtain weather information for
    #[command(subcommand)]
    location: Location,

    /// Time range to get the weather forecast for
    #[arg(value_enum)]
    time: ForecastTime,
}

fn configure_provider(prv: Provider) {
    let mut config = WeatherConfig::get();
    config.provider = Some(prv);
    config.save();
}

fn manage_places(act: PlacesAction) {
    let mut config = WeatherConfig::get();

    let places = match act {
        PlacesAction::GetAll => config.places,
        PlacesAction::Set(place) => {
            config.places.replace(place);
            config.save();

            config.places
        }
        PlacesAction::Remove(place) => {
            config.places.remove(&place);
            config.save();

            config.places
        }
    };
}

async fn get_forecast(args: ForecastArgs) {
    let config = WeatherConfig::get();

    if let Some(prv_type) = config.provider {
        let provider: Box<dyn providers::Provider> = match prv_type {
            Provider::OpenWeather(creds) => {
                Box::new(open_weather::OpenWeather::new(creds.key.to_owned()))
            }
            Provider::WeatherApi(creds) => {
                Box::new(weather_api::WeatherApi::new(creds.key.to_owned()))
            }
        };

        let coords = match args.location {
            Location::Coordinates(coords) => Some(coords),
            Location::Place(place) => config
                .places
                .iter()
                .find(|p| p.tag == place)
                .map(|p| p.coordinates.to_owned()),
        };

        if let Some(coords) = coords {
            provider.get_forecast(coords, args.time).await;
        }
    }
}

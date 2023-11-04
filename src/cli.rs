use clap::{Args, Parser, Subcommand};

use crate::{
    config::WeatherConfig,
    errors::AppError,
    providers::{self, open_weather, weather_api},
    weather::*,
    Result,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    operation: Operation,
}

impl Cli {
    pub async fn process(self) -> Result<()> {
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
    #[arg(value_enum, default_value_t = ForecastTime::Now)]
    time: ForecastTime,

    /// Units type
    #[arg(value_enum, default_value_t = UnitType::Metric)]
    unit: UnitType,
}

fn configure_provider(prv: Provider) -> Result<()> {
    let mut config = WeatherConfig::get()?;
    config.provider = Some(prv.clone());
    config.save()?;

    println!("Provider {} successfully configured!", prv);
    Ok(())
}

fn manage_places(act: PlacesAction) -> Result<()> {
    let mut config = WeatherConfig::get()?;

    let places = match act {
        PlacesAction::GetAll => config.places,
        PlacesAction::Set(place) => {
            place.coordinates.validate()?;
            config.places.replace(place);
            config.save()?;

            config.places
        }
        PlacesAction::Remove(tag) => {
            if let Some(remove_place) = config.place_by_tag(&tag) {
                config.places.remove(&remove_place);
                config.save()?;
            }

            config.places
        }
    };

    println!("Places: ");
    for place in places {
        println!("{}", place);
    }

    Ok(())
}

async fn get_forecast(args: ForecastArgs) -> Result<()> {
    let config = WeatherConfig::get()?;
    let prv_type = config.provider.as_ref().ok_or(AppError::EmptyProvider)?;

    let provider: Box<dyn providers::Provider> = match prv_type {
        Provider::OpenWeather(creds) => {
            Box::new(open_weather::OpenWeather::new(creds.key.to_owned())?)
        }
        Provider::WeatherApi(creds) => {
            Box::new(weather_api::WeatherApi::new(creds.key.to_owned())?)
        }
    };

    let coords = match args.location {
        Location::Coordinates(coords) => Some(coords),
        Location::Place(tag) => config.place_by_tag(&tag).map(|p| p.coordinates),
    };

    if let Some(coords) = coords {
        let weather = provider.get_forecast(coords, args.time, args.unit).await?;
        println!("Weather provider: {}", prv_type);
        match weather {
            Weather::Current(current) => {
                println!("{}", current);
            }
            Weather::Today(hours) => {
                for hour in hours {
                    println!("{}\n", hour);
                }
            }
            Weather::Daily(days) => {
                for day in days {
                    println!("{}\n", day);
                }
            }
        }
    }

    Ok(())
}

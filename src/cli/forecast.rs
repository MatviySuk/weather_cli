use crate::{
    config::WeatherConfig,
    errors::AppError,
    providers::{self, open_weather, weather_api},
    weather::*,
    Result,
};
use clap::Args;

#[derive(Args, Clone, Debug)]
pub struct ForecastArgs {
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

pub async fn get_forecast(args: ForecastArgs) -> Result<()> {
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

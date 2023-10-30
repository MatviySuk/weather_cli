use clap::{Args, Parser, Subcommand};

use crate::weather::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    operation: Operation,
}

impl Cli {
    pub fn process(self) {
        match self.operation {
            Operation::Configure { provider } => configure_provider(provider),
            Operation::Places { action } => manage_places(action),
            Operation::Forecast(args) => get_forecast(args),
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
    match prv {
        _ => {}
    }
}

fn manage_places(act: PlacesAction) {
    match act {
        PlacesAction::GetAll => {}
        PlacesAction::Set(place) => {}
        PlacesAction::Remove(place) => {}
    }
}

fn get_forecast(args: ForecastArgs) {}

mod configure;
mod forecast;
mod global;
mod places;

use configure::configure_provider;
use forecast::{get_forecast, ForecastArgs};
use places::manage_places;

pub use global::Cli;

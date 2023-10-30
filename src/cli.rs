use clap::{Args, Parser, Subcommand, ValueEnum};

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

// Operation -------------------------------------------------------------------

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

// Configure -------------------------------------------------------------------

#[derive(Subcommand, Clone, Debug)]
enum Provider {
    A(ProviderCredentials),
    B(ProviderCredentials),
}

#[derive(Args, Clone, Debug)]
struct ProviderCredentials {
    #[arg(short, long)]
    credentials: String,
}

// Places ----------------------------------------------------------------------

#[derive(Subcommand, Clone, Debug)]
enum PlacesAction {
    /// Get all the saved places
    GetAll,

    /// Save the new place or update the location of existed place by tag
    Set(Place),

    /// Remove the place if it is present
    Remove(Place),
}

#[derive(Args, Clone, Debug)]
struct Place {
    /// Tag or name of the place
    #[command(flatten)]
    tag: PlaceTag,

    /// Geodetic coordinate
    #[command(flatten)]
    coordinates: Coordinates,
}

#[derive(Args, Clone, Debug)]
struct PlaceTag {
    /// Tag or name of the place
    #[arg(short, long)]
    tag: String,
}

#[derive(Args, Clone, Debug)]
struct Coordinates {
    /// Value must be between -90 and 90 degrees including
    #[arg(long = "lat")]
    latitude: f32,

    /// Geodetic longitude of the location.
    /// Value must be between -180 and 180 degrees including
    #[arg(long = "long")]
    longitude: f32,
}

// Forecast --------------------------------------------------------------------

#[derive(Subcommand, Clone, Debug)]
enum Location {
    /// Tag of the place saved to frequently used
    Place(PlaceTag),

    /// Geodetic coordinate
    Coordinates(Coordinates),
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

#[derive(ValueEnum, Clone, Debug)]
enum ForecastTime {
    Now,
    Today,
    Tomorrow,
    Days5,
}

// Functions -------------------------------------------------------------------

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

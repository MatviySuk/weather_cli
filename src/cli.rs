use clap::{Parser, Subcommand, Args,};

#[derive(Parser, Debug,)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    operation: Operation,
}

impl Cli {
    pub fn process(self) {
        match self.operation {
            Operation::Configure { provider } => {},
            Operation::Places { action } => {},
            Operation::Forecast => {},
        }
    }
}

// Operation -------------------------------------------------------------------

#[derive(Subcommand, Debug,)]
enum Operation {
    /// Configure provider and credentials
    Configure { 
        #[command(subcommand)] provider: Provider, 
    },
    
    /// Save frequently used locations
    Places {
        #[command(subcommand)] action: PlacesAction,
    },

    /// Get a weather forecast for the specific location
    Forecast,
}

// Configure -------------------------------------------------------------------

#[derive(Subcommand, Clone, Debug,)]
enum Provider {
    A(ProviderCredentials), 
    B(ProviderCredentials),
}

#[derive(Args, Clone, Debug,)]
struct ProviderCredentials {
    #[arg(short, long)]
    credentials: String,
}

// Places ----------------------------------------------------------------------

#[derive(Subcommand, Clone, Debug,)]
enum PlacesAction {
    /// Get all the saved places
    GetAll,

    /// Save the new place or update the location of existed place by tag
    Set(Place),

    /// Remove the place if it is present
    Remove(Place),
}

#[derive(Args, Clone, Debug,)]
struct Place {
    /// Tag or name of the place
    #[arg(short, long)]
    tag: String,

    /// Geodetic latitude of the location.
    /// Value must be between -90 and 90 degrees including 
    #[arg(long = "lat")]
    latitude: f32,

    /// Geodetic longitude of the location. 
    /// Value must be between -180 and 180 degrees including 
    #[arg(long = "long")]
    longitude: f32,
}

// Forecast --------------------------------------------------------------------
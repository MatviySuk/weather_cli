use crate::cli::Cli;

use clap::Parser;

pub mod cli;
pub mod config;
pub mod errors;
pub mod providers;
pub mod weather;

pub type Result<T> = std::result::Result<T, errors::AppError>;

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(e) = Cli::parse().process().await {
        println!("{e}");
    }

    Ok(())
}

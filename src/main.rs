use crate::cli::Cli;

use clap::Parser;

pub mod cli;
pub mod config;
pub mod providers;
pub mod weather;

#[tokio::main]
async fn main() {
    Cli::parse().process().await;
}

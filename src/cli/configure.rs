use crate::{config::WeatherConfig, weather::*, Result};

pub fn configure_provider(prv: Provider) -> Result<()> {
    let mut config = WeatherConfig::get()?;
    config.provider = Some(prv.clone());
    config.save()?;

    println!("Provider {} successfully configured!", prv);
    Ok(())
}

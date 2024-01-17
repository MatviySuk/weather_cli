use crate::{config::WeatherConfig, weather::*, Result};

pub fn manage_places(act: PlacesAction) -> Result<()> {
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

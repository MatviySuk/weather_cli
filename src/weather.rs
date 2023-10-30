
#[derive(Debug)]
pub struct Place {
    tag: PlaceTag,
    coordinates: Coordinates,
}

#[derive(Debug)]
pub struct PlaceTag {
    tag: String,
}

#[derive(Debug)]
pub struct Coordinates {
    pub lat: f32,
    pub lon: f32,
}

#[derive(Debug)]
pub enum Location {
    Place(PlaceTag),
    Coordinates(Coordinates),
}

#[derive(Debug)]
pub enum ForecastTime {
    Now,
    Today,
    Tomorrow,
    Days5,
}
#[allow(snake_case)]
use serde::Deserialize;

#[derive(Debug)]
pub struct Property {
    pub property_code: String,
    pub price_eur: f64,
    pub size_sqm: f64,
    pub floor: Option<u32>,
    pub address: String,
    pub province: String,
    pub municipality: String,
    pub district: String,
    pub neighborhood: String,
    pub latitude: f64,
    pub longitude: f64,
    pub has_lift: bool,
    pub price_per_sqm: f64,
    pub rooms: u32,
    pub bathrooms: u32,
    pub swimming_pool: bool,
    pub garden: bool,
    pub garage: bool,
    pub url: String,
}

impl Property {
    pub(crate) fn to_feature_vector(&self) -> Vec<f64> {
        vec![
            self.size_sqm,
            self.floor.unwrap_or(0) as f64,
            self.latitude,
            self.longitude,
            if self.has_lift { 1.0 } else { 0.0 },
            self.price_per_sqm,
            self.rooms as f64,
            self.bathrooms as f64,
            if self.swimming_pool { 1.0 } else { 0.0 },
            if self.garden { 1.0 } else { 0.0 },
            if self.garage { 1.0 } else { 0.0 },
        ]
    }
}


#[derive(Debug, Deserialize)]
pub struct Suggestion {
    pub name: String,
    pub locationId: Option<String>,
    pub total: u32,
}

#[derive(Debug, Deserialize)]
pub struct SuggestionsResponse {
    pub locations: Vec<Suggestion>,
}

#[derive(Debug)]
pub struct Location {
    pub locationId: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct HomeListing {
    pub propertyCode: String,
    pub price: f64,
    pub size: Option<f64>,
    pub floor: Option<String>,
    pub address: Option<String>,
    pub province: Option<String>,
    pub municipality: Option<String>,
    pub district: Option<String>,
    pub neighborhood: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub hasLift: Option<bool>,
    pub priceByArea: Option<f64>,
    pub rooms: Option<f64>,
    pub bathrooms: Option<f64>,
    pub swimmingPool: Option<bool>,
    pub garden: Option<bool>,
    pub garage: Option<bool>,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListingsResponse {
    pub elementList: Vec<HomeListing>,
    pub totalPages: u32,
}

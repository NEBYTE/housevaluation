use crate::core::types::{SuggestionsResponse, ListingsResponse, HomeListing, Location};

use std::error::Error;
use std::collections::{HashMap, HashSet};
use reqwest::blocking::Client;
use std::fs::{File, OpenOptions};
use std::path::Path;
use csv::{ReaderBuilder, Writer};

pub struct IdealistaScraper {
    client: Client,
    idealista_base_api_url: String,
    idealista_api_key: String,
    cached_location_ids: HashSet<String>,
}

impl IdealistaScraper {
    pub fn new() -> Self {
        let idealista_base_api_url = std::env::var("IDEALISTA_BASE_API_URL")
            .expect("Missing IDEALISTA_BASE_API_URL env var");
        let idealista_api_key = std::env::var("IDEALISTA_API_KEY")
            .expect("Missing IDEALISTA_API_KEY env var");

        let mut cached_location_ids = HashSet::new();
        if let Ok(file) = File::open("data/idealista_homes_spain.csv") {
            let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
            for result in rdr.records() {
                if let Ok(record) = result {
                    if let Some(location_id) = record.get(10) {
                        cached_location_ids.insert(location_id.to_string());
                    }
                }
            }
        }

        Self {
            client: Client::new(),
            idealista_base_api_url,
            idealista_api_key,
            cached_location_ids,
        }
    }

    pub fn fetch_location_ids(&mut self, cities: &[&str]) -> HashMap<String, String> {
        let mut location_map = HashMap::new();

        for city_name in cities {
            if self.cached_location_ids.contains(&city_name.to_string()) {
                println!("Skipping city {}: Already fetched", city_name);
                continue;
            }

            let url = format!(
                "{}/getsuggestions?prefix={}&location=es&propertyType=homes&operation=sale",
                &self.idealista_base_api_url, city_name
            );

            let response = self.client
                .get(&url)
                .header("x-rapidapi-host", "idealista7.p.rapidapi.com")
                .header("x-rapidapi-key", &self.idealista_api_key)
                .send();

            if let Ok(res) = response {
                let response_text = res.text().unwrap_or_else(|_| "Failed to read response body".to_string());
                if let Ok(data) = serde_json::from_str::<SuggestionsResponse>(&response_text) {
                    if let Some(best_match) = data.locations.iter().max_by_key(|loc| loc.total) {
                        if let Some(location_id) = &best_match.locationId {
                            location_map.insert(city_name.to_string(), location_id.clone());
                            self.cached_location_ids.insert(city_name.to_string());
                        }
                    }
                }
            }
        }

        location_map
    }

    pub fn scrape_listings(&self, location_id: &str, city_name: &str) -> Vec<HomeListing> {
        let url = format!(
            "{}/listhomes?order=relevance&operation=sale&propertyType=homes&locationId={}&locationName={}&numPage=1&maxItems=40&location=es&locale=es",
            self.idealista_base_api_url, location_id, city_name
        );

        let response = self.client
            .get(&url)
            .header("x-rapidapi-host", "idealista7.p.rapidapi.com")
            .header("x-rapidapi-key", &self.idealista_api_key)
            .send();

        if let Ok(res) = response {
            let response_text = res.text().unwrap_or_else(|_| "Failed to read response body".to_string());
            if let Ok(data) = serde_json::from_str::<ListingsResponse>(&response_text) {
                return data.elementList;
            }
        }

        vec![]
    }

    pub fn scrape_all_homes_spain(&mut self) -> Result<(), Box<dyn Error>> {
        let cities = [
            "Madrid", "Barcelona", "Seville", "Valencia", "Málaga", "Zaragoza", "A Coruña",
            "Gijón", "San Sebastián", "Pamplona", "Santander", "Burgos", "León", "Valladolid",
            "Salamanca", "Bilbao", "Vitoria-Gasteiz", "Alicante", "Castellón de la Plana", "Tarragona",
        ];

        let location_ids = self.fetch_location_ids(&cities);
        let locations: Vec<Location> = cities
            .iter()
            .filter_map(|&city| {
                location_ids.get(city).map(|id| {
                    Location {
                        locationId: id.clone(),
                        name: city.to_string(),
                    }
                })
            })
            .collect();

        let csv_file_path = "data/idealista_homes_spain.csv";
        let file_exists = Path::new(csv_file_path).exists();

        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(csv_file_path)?;

        let mut writer = Writer::from_writer(file);

        if !file_exists {
            writer.write_record(&[
                "Property Code", "Price (€)", "Size (m²)", "Floor", "Address", "Province",
                "Municipality", "District", "Neighborhood", "Latitude", "Longitude", "Has Lift",
                "Price by Area", "Rooms", "Bathrooms", "Swimming Pool", "Garden", "Garage", "URL",
            ])?;
        }

        for location in locations {
            let listings = self.scrape_listings(&location.locationId, &location.name);
            for home in &listings {
                writer.write_record(&[
                    home.propertyCode.clone(),
                    home.price.to_string(),
                    home.size.map_or("N/A".to_string(), |s| s.to_string()),
                    home.floor.clone().unwrap_or_else(|| "N/A".to_string()),
                    home.address.clone().unwrap_or_else(|| "N/A".to_string()),
                    home.province.clone().unwrap_or_else(|| "N/A".to_string()),
                    home.municipality.clone().unwrap_or_else(|| "N/A".to_string()),
                    home.district.clone().unwrap_or_else(|| "N/A".to_string()),
                    home.neighborhood.clone().unwrap_or_else(|| "N/A".to_string()),
                    home.latitude.map_or("N/A".to_string(), |lat| lat.to_string()),
                    home.longitude.map_or("N/A".to_string(), |lon| lon.to_string()),
                    home.hasLift.map_or("N/A".to_string(), |lift| lift.to_string()),
                    home.priceByArea.map_or("N/A".to_string(), |pba| pba.to_string()),
                    home.rooms.map_or("N/A".to_string(), |r| r.to_string()),
                    home.bathrooms.map_or("N/A".to_string(), |b| b.to_string()),
                    home.swimmingPool.map_or("N/A".to_string(), |sp| sp.to_string()),
                    home.garden.map_or("N/A".to_string(), |g| g.to_string()),
                    home.garage.map_or("N/A".to_string(), |gr| gr.to_string()),
                    home.url.clone().unwrap_or_else(|| "N/A".to_string()),
                ])?;
            }
        }

        Ok(())
    }
}

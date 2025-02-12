pub mod core;

use std::collections::HashMap;
use std::error::Error;
use std::fs;

use dialoguer::{Select, Input, Confirm};

use core::types::Property;
use core::scrapers::idealista::IdealistaScraper;
use core::cervo::Cervo;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    loop {
        let options = &["Scrape Data", "Predict Property Price", "Train Model (Make sure to have new data)", "Exit"];
        let selection = Select::new()
            .with_prompt("What do you want to do?")
            .items(options)
            .default(0)
            .interact()?;

        match selection {
            0 => {
                let mut scraper = IdealistaScraper::new();
                println!("Starting scraping process... (this may take a while, please be patient)");
                scraper.scrape_all_homes_spain()?;
                println!("Scraping completed, output saved to data/idealista_homes_spain.csv.");
            }
            1 => {
                let property = build_property_from_user_input();
                let cervo = Cervo::new("data/idealista_homes_spain.csv")?;
                let predicted_price = cervo.predict_price(&property);

                println!("💰 Predicted price: €{:.2}", predicted_price);
            }
            2 => {
                let k_folds = std::env::var("K_FOLDS")
                    .ok()
                    .and_then(|v| v.parse::<usize>().ok())
                    .unwrap_or(10);

                let new_k_folds = Input::new()
                    .with_prompt("Enter the number of folds (leave empty to use .env)")
                    .allow_empty(true)
                    .interact_text()
                    .ok()
                    .and_then(|input: String| input.parse().ok())
                    .unwrap_or(k_folds);

                println!("Using {} folds.", new_k_folds);

                update_env("K_FOLDS", &new_k_folds.to_string());

                Cervo::train_and_save_model("data/idealista_homes_spain.csv")?;
            }
            _ => {
                println!("👋 Exiting.");
                break Ok(());
            }
        }
    }
}

fn build_property_from_user_input() -> Property {
    let size_sqm: f64 = Input::new()
        .with_prompt("Enter the size (m²) of the property")
        .interact_text()
        .unwrap();

    let floor: Option<u32> = Input::new()
        .with_prompt("Enter the floor number (leave empty if not applicable)")
        .allow_empty(true)
        .interact_text()
        .ok()
        .and_then(|input: String| input.parse().ok());

    let latitude: f64 = Input::new()
        .with_prompt("Enter the latitude")
        .interact_text()
        .unwrap();

    let longitude: f64 = Input::new()
        .with_prompt("Enter the longitude")
        .interact_text()
        .unwrap();

    let has_lift = Confirm::new()
        .with_prompt("Does the property have a lift?")
        .interact()
        .unwrap();

    let price_per_sqm: Option<f64> = Input::new()
        .with_prompt("Enter price per m² (if not known, leave empty)")
        .allow_empty(true)
        .interact_text()
        .ok()
        .and_then(|input: String| input.parse().ok());

    let rooms: u32 = Input::new()
        .with_prompt("Number of bedrooms")
        .interact_text()
        .unwrap();

    let bathrooms: u32 = Input::new()
        .with_prompt("Number of bathrooms")
        .interact_text()
        .unwrap();

    let swimming_pool = Confirm::new()
        .with_prompt("Does the property have a swimming pool?")
        .interact()
        .unwrap();

    let garden = Confirm::new()
        .with_prompt("Does the property have a garden?")
        .interact()
        .unwrap();

    let garage = Confirm::new()
        .with_prompt("Does the property have a garage?")
        .interact()
        .unwrap();

    Property {
        property_code: "".to_string(),
        price_eur: 0.0,
        size_sqm,
        floor,
        address: "".to_string(),
        province: "".to_string(),
        municipality: "".to_string(),
        district: "".to_string(),
        neighborhood: "".to_string(),
        latitude,
        longitude,
        has_lift,
        price_per_sqm: price_per_sqm.unwrap_or(0.0),
        rooms,
        bathrooms,
        swimming_pool,
        garden,
        garage,
        url: "".to_string(),
    }
}

fn update_env(key: &str, value: &str) {
    let env_path = ".env";
    let mut env_vars: HashMap<String, String> = HashMap::new();

    if let Ok(contents) = fs::read_to_string(env_path) {
        for line in contents.lines() {
            if let Some((k, v)) = line.split_once('=') {
                env_vars.insert(k.trim().to_string(), v.trim().to_string());
            }
        }
    }

    env_vars.insert(key.to_string(), value.to_string());

    let new_contents = env_vars
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<String>>()
        .join("\n");

    if let Err(e) = fs::write(env_path, new_contents) {
        println!("Failed to update .env file: {}", e);
    }
}

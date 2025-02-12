use linfa::prelude::*;

use csv::Reader;
use linfa::Dataset;
use linfa_elasticnet::ElasticNet;
use ndarray::{arr0, Array0, Array1, Array2, ArrayView1};

use serde::{Deserialize, Serialize};

use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

use crate::core::types::Property;

const MODEL_FILE: &str = "output/cervo_model.bin";

#[derive(Serialize, Deserialize)]
pub struct Cervo {
    model: ElasticNet<f64>,
}

impl Cervo {
    pub fn new(filename: &str) -> Result<Self, Box<dyn Error>> {
        if let Ok(model) = Self::load_model() {
            println!("✅ Loaded existing trained model.");
            return Ok(Self { model });
        }

        println!("⚠️ No saved model found. Training a new one...");
        let mut dataset = Self::load_data(filename)?;
        let model = Self::train_model(&mut dataset)?;
        Self::save_model(&model)?;

        Ok(Self { model })
    }

    fn load_data(filename: &str) -> Result<Dataset<f64, f64, ndarray::Ix1>, Box<dyn Error>> {
        let mut rdr = Reader::from_path(filename)?;
        let mut x_data = Vec::new();
        let mut y_data = Vec::new();

        for result in rdr.records() {
            let record = result?;
            let target: f64 = record[1].parse()?;
            let size: f64 = record[2].parse()?;
            let floor: f64 = record[3].parse().unwrap_or(0.0);
            let latitude: f64 = record[9].parse()?;
            let longitude: f64 = record[10].parse()?;
            let has_lift: f64 = if record[11].trim() == "true" { 1.0 } else { 0.0 };
            let price_by_area: f64 = record[12].parse()?;
            let rooms: f64 = record[13].parse()?;
            let bathrooms: f64 = record[14].parse()?;
            let swimming_pool: f64 = if record[15].trim() == "true" { 1.0 } else { 0.0 };
            let garden: f64 = if record[16].trim() == "true" { 1.0 } else { 0.0 };
            let garage: f64 = if record[17].trim() == "true" { 1.0 } else { 0.0 };

            x_data.push(vec![
                size, floor, latitude, longitude, has_lift, price_by_area, rooms, bathrooms,
                swimming_pool, garden, garage
            ]);
            y_data.push(target);
        }

        let num_samples = x_data.len();
        let num_features = x_data[0].len();

        let x_array = Array2::from_shape_vec((num_samples, num_features), x_data.concat())?;
        let y_array = Array2::from_shape_vec((num_samples, 1), y_data)?;

        let dataset = Dataset::new(x_array, y_array).into_single_target();
        Ok(dataset)
    }

    fn train_model(dataset: &mut Dataset<f64, f64, ndarray::Ix1>) -> Result<ElasticNet<f64>, Box<dyn Error>> {
        let k_folds = 5;

        let mut best_model = None;
        let mut best_score = f64::MAX;
        let mut best_penalty = None;
        let mut best_l1_ratio = None;

        let eval = |predicted: &Array1<f64>, expected: &ArrayView1<f64>| -> Result<Array0<f64>, linfa::Error> {
            predicted.r2(expected).map(arr0)
        };


        for &penalty in &[0.01, 0.05, 0.1, 0.5, 1.0] {
            for &l1_ratio in &[0.0, 0.25, 0.5, 0.75, 1.0] {
                let model_params = ElasticNet::params()
                    .penalty(penalty)
                    .l1_ratio(l1_ratio);

                let results = dataset.cross_validate(k_folds, &[model_params.clone()], &eval)?;

                let mean_mse = results.mean().unwrap();

                if mean_mse < best_score {
                    best_score = mean_mse;
                    best_model = Some(model_params.fit(dataset)?);
                    best_penalty = Some(penalty);
                    best_l1_ratio = Some(l1_ratio);
                }
            }
        }

        if let Some(model) = best_model {
            println!("Best model found with penalty {} and l1_ratio {}", best_penalty.unwrap(), best_l1_ratio.unwrap());
            Ok(model)
        } else {
            Err("No suitable model found".into())
        }

    }

    pub fn predict_price(&self, property: &Property) -> f64 {
        let features = property.to_feature_vector();

        let input_array = Array2::from_shape_vec((1, features.len()), features).unwrap();
        let prediction = self.model.predict(&input_array);
        prediction[0]
    }

    pub fn train_and_save_model(filename: &str) -> Result<(), Box<dyn Error>> {
        println!("Training a new model... This may take some time.");
        let mut dataset = Self::load_data(filename)?;
        let model = Self::train_model(&mut dataset)?;
        Self::save_model(&model)?;
        println!("Model training complete. Saved to {}", MODEL_FILE);
        Ok(())
    }

    fn save_model(model: &ElasticNet<f64>) -> Result<(), Box<dyn Error>> {
        let serialized = serde_json::to_string(model)?;
        let mut file = File::create(MODEL_FILE)?;
        file.write_all(serialized.as_bytes())?;
        println!("Saved model to {}", MODEL_FILE);
        Ok(())
    }

    fn load_model() -> Result<ElasticNet<f64>, Box<dyn Error>> {
        let mut file = File::open(MODEL_FILE)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).expect("Couldn't read to string buffer.");

        let model: ElasticNet<f64> = serde_json::from_str(&buffer)?;
        println!("Loaded model from {}", MODEL_FILE);
        Ok(model)
    }
}

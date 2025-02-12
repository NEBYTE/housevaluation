<img src="https://l7mozmkiwy.ufs.sh/f/HKemhjN71TyOBDSBjRYWE0OaYPF9Vq4jUDItmN6JuXrkiTAe" alt="Crypto Trading Bot">

# House Valuation AI (v0.1.0-pre.alpha.1)

[![Maintainer](https://img.shields.io/badge/maintainer-NEBYTE-blue)](https://github.com/carlos-crypto)  
[![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)  
[![License](https://img.shields.io/badge/License-GNU_AGPLv3-blue)](https://choosealicense.com/licenses/agpl-3.0/)

> [!NOTE]  
> This is a **machine learning-powered real estate valuation tool** built in Rust. It uses **Elastic Net Regression** to predict property prices based on **historical data** from real estate listings. **As of v0.1.0-pre.alpha.1, Spain + Idealista API is the only country available for scraping.**

This AI model processes **location-based property features** (size, floor, amenities, etc.) to estimate real estate values. It includes **scraping, training, and prediction capabilities**.

---

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
    - [Scrape Data](#scrape-data)
    - [Train the Model](#train-the-model)
    - [Predict Property Prices](#predict-property-prices)
- [Technical Overview](#technical-overview)
- [Dependencies](#dependencies)
- [License](#license)

---

## Features

> [!WARNING]  
> **This model is experimental.** It is not a financial advisory tool and should not be used for critical investment decisions.

- **Property Price Estimation** - Uses **Elastic Net Regression** to predict real estate prices.
- **Automated Data Scraping** - Fetches property data from Idealista.
- **Machine Learning Training** - Builds a model using historical property listings.
- **Feature Extraction** - Uses **location, size, rooms, bathrooms, and more** as predictive factors.
- **Evaluation Metrics** - Computes **R² score** to assess model performance.

---
## In action

```shell
What do you want to do?: Predict Property Price
Enter the size (m²) of the property: 196
Enter the floor number (leave empty if not applicable): 6
Enter the latitude: 32
Enter the longitude: 0.43
Does the property have a lift? yes
Enter price per m² (if not known, leave empty): 3500
Number of bedrooms: 4
Number of bathrooms: 3
Does the property have a swimming pool? yes
Does the property have a garden? yes
Does the property have a garage? yes
Loaded model from output/cervo_model.bin
✅ Loaded existing trained model.
💰 Predicted price: €1122271.88
What do you want to do?:
> Scrape Data
  Predict Property Price
  Train Model
  Exit

```

---

## Installation

### Prerequisites

- **Rust** (latest stable)
- **Cargo** package manager
- **CSV dataset** (automatically generated from scraper)

### Clone the Repository

```sh
git clone https://github.com/NEBYTE/HouseValuation.git
cd HouseValuation
```

### Build the Project

```sh
cargo build --release
```

---

## Usage

### Scrape Data

```sh
cargo run --release
```

This will **scrape Idealista real estate listings** and save them to `data/idealista_homes_spain.csv`.

---

### Train the Model

```sh
cargo run --release
```

The model will process the dataset and generate a **trained Elastic Net Regression model** saved to `output/cervo_model.bin`.

---

### Predict Property Prices

To estimate the price of a custom property:

```sh
cargo run --release -- predict
```

It will prompt the user for property details (size, rooms, location, etc.) and return a **predicted price**.

---

## Technical Overview

### **Machine Learning Model**
- **Algorithm**: Elastic Net Regression
- **Input Features**:
    - **Geolocation** (latitude, longitude)
    - **Size** (square meters)
    - **Floor level**
    - **Number of rooms & bathrooms**
    - **Amenities** (pool, garden, garage, lift)
- **Evaluation Metric**: **R² Score**

### **Dataset Handling**
- **CSV Format**
- **Auto-generated via scraper**
- **Stores property attributes & prices**

### **Risk & Error Handling**
- **Missing values handling**
- **Data normalization**
- **Cross-validation (K=5)**

---

## Dependencies

```toml
[dependencies]
csv = "1.2"
ndarray = "0.15"
linfa = "0.7.0"
linfa-elasticnet = { version = "0.7.0", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
dotenv = "0.15.0"
dialoguer = "0.11.0"
```

---

## License

Distributed under the [GNU AGPLv3](https://choosealicense.com/licenses/agpl-3.0/) license.
use std::{fs::File, io::Write};

use serde::{Deserialize, Serialize};

use rand::Rng;

pub fn check_save() {
    if File::open("save.json").is_err() {
        println!("Save file not found, creating a new one...");
        let new_save_file = File::create("save.json");
        if new_save_file.is_err() {
            panic!("Failed to create a new save file!");
        } else {
            let mut rng = rand::thread_rng();

            new_save_file
                .unwrap()
                .write_all(
                    serde_json::to_string(&Data {
                        stocks: vec![
                            Stock {
                                name: "AAPL".to_string(),
                                price: rng.gen_range(100.0..500.0),
                            },
                            Stock {
                                name: "AMZN".to_string(),
                                price: rng.gen_range(100.0..500.0),
                            },
                            Stock {
                                name: "GOOG".to_string(),
                                price: rng.gen_range(100.0..500.0),
                            },
                            Stock {
                                name: "MSFT".to_string(),
                                price: rng.gen_range(200.0..600.0),
                            },
                            Stock {
                                name: "TSLA".to_string(),
                                price: rng.gen_range(100.0..400.0),
                            },
                        ],
                        balance: 10000.0,
                    })
                    .unwrap()
                    .as_bytes(),
                )
                .unwrap();

            println!("New save file created!");
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    stocks: Vec<Stock>,
    balance: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Stock {
    name: String,
    price: f64,
}

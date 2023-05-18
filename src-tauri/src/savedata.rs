use std::{fs::File, io::Write};

use rand::Rng;

use crate::types::data::{Data, Stock};

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
                                name: "Apple".to_string(),
                                ticker: "AAPL".to_string(),
                                price: rng.gen_range(100.0..500.0),
                                owned: 0,
                            },
                            Stock {
                                name: "Amazon".to_string(),
                                ticker: "AMZN".to_string(),
                                price: rng.gen_range(100.0..500.0),
                                owned: 0,
                            },
                            Stock {
                                name: "Google".to_string(),
                                ticker: "GOOG".to_string(),
                                price: rng.gen_range(100.0..500.0),
                                owned: 0,
                            },
                            Stock {
                                name: "Microsoft".to_string(),
                                ticker: "MSFT".to_string(),
                                price: rng.gen_range(200.0..600.0),
                                owned: 0,
                            },
                            Stock {
                                name: "Tesla".to_string(),
                                ticker: "TSLA".to_string(),
                                price: rng.gen_range(100.0..400.0),
                                owned: 0,
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

pub fn load_save() -> Data {
    let save_file = File::open("save.json");
    if save_file.is_err() {
        panic!("Failed to open save file!");
    } else {
        let save_file = save_file.unwrap();
        let save_data: Data = serde_json::from_reader(save_file).unwrap();
        return save_data;
    }
}

pub fn save_data(data: Data) {
    let save_file = File::create("save.json");
    if save_file.is_err() {
        panic!("Failed to create save file!");
    } else {
        save_file
            .unwrap()
            .write_all(serde_json::to_string(&data).unwrap().as_bytes())
            .unwrap();
    }
}

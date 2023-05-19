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
                            Stock::new("Apple", "AAPL", rng.gen_range(100.0..500.0), 0),
                            Stock::new("Amazon", "AMZN", rng.gen_range(100.0..500.0), 0),
                            Stock::new("Google", "GOOG", rng.gen_range(100.0..500.0), 0),
                            Stock::new("Microsoft", "MSFT", rng.gen_range(200.0..600.0), 0),
                            Stock::new("Tesla", "TSLA", rng.gen_range(100.0..400.0), 0),
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

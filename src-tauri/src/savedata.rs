use std::{fs::File, io::Write};

use serde::{Deserialize, Serialize};

pub fn check_save() {
    let save_file = File::open("save.json");
    if save_file.is_err() {
        println!("Save file not found, creating a new one...");
        let new_save_file = File::create("save.json");
        if new_save_file.is_err() {
            println!("Failed to create a new save file!");
        } else {
            new_save_file.unwrap().write_all(serde_json::to_string(&Data {
                stocks: Vec::new(),
                balance: 10000.0,
            })
            .unwrap().as_bytes()).unwrap();

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
    amount: i32,
    total: f64,
}

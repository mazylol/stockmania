use serde::Deserialize;

use std::fs::File;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SaveData {
    
}

pub fn check_save() {
    let save_file = File::open("save.json");
    if save_file.is_err() {
        println!("Save file not found, creating a new one...");
        let new_save_file = File::create("save.json");
        if new_save_file.is_err() {
            println!("Failed to create a new save file!");
        }
    }
}
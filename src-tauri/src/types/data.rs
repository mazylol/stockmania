use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub stocks: Vec<Stock>,
    pub balance: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stock {
    pub name: String,
    pub ticker: String,
    pub price: f64,
    pub owned: i32,
}

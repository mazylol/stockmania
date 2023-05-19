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

impl Stock {
    pub fn new(name: &str, ticker: &str, price: f64, owned: i32) -> Stock {
        let name: String = name.to_string();
        let ticker: String = ticker.to_string();

        return Stock {
            name,
            ticker,
            price,
            owned,
        };
    }
}

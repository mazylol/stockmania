use rand::Rng;

use crate::savedata;

#[tauri::command]
pub fn buy_stock(ticker: String) {
    let mut data = savedata::load_save();

    let stock = data.stocks.iter_mut().find(|s| s.ticker == ticker).unwrap();

    if data.balance >= stock.price {
        data.balance -= stock.price;
        stock.owned += 1;
    }

    savedata::save_data(data);
}

#[tauri::command]
pub fn sell_stock(ticker: String) {
    let mut data = savedata::load_save();

    let stock = data.stocks.iter_mut().find(|s| s.ticker == ticker).unwrap();

    if stock.owned > 0 {
        data.balance += stock.price;
        stock.owned -= 1;
    }

    savedata::save_data(data);
}

#[tauri::command]
pub fn price_changes() {
    let mut rng = rand::thread_rng();
    let decision: i32 = rng.gen_range(0..100);

    let mut data = savedata::load_save();

    let stocks = data.stocks.iter_mut();

    // Good news
    if decision < 76 {
        for stock in stocks {
            let change = rng.gen_range(0.0..10.0);
            stock.price += change;
        }
    // Bad news
    } else {
        for stock in stocks {
            let change = rng.gen_range(-10.0..0.0);
            stock.price += change;
        }
    }

    savedata::save_data(data);
}

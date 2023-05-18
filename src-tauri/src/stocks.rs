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

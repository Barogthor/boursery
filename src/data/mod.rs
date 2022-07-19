use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct StockData {
    open: f32,
    high: f32,
    low: f32,
    close: f32,
    volume: f32,
    date: String,
    symbol: String,
}

#[derive(Deserialize, Debug)]
pub struct StockDataResponse {
    pub data: Vec<StockData>,
}
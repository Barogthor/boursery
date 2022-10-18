#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::io::ErrorKind;
use std::ops::Add;
use std::sync::Arc;

use chrono::{Date, Datelike, DateTime, TimeZone, Utc};
use reqwest::{Error, Url};
use serde::Deserialize;

use gui::{AppGUI, eframe};

use crate::data::{StockData, StockDataResponse};

// hide console window on Windows in release
mod data;

#[derive(Deserialize, Debug)]
struct User {
    id: u32,
    #[serde(rename = "userId")]
    user_id: u32,
    title: String,
    body: String,
}

fn open_cache_for_date(date: &Date<Utc>, symbol: &str) -> Result<Vec<StockData>, String> {
    let path = format!("cache/{}_{}.json", symbol, date.format("%Y-%m"));
    println!("{}", path);
    let file = File::open(path).map_err(|err| err.to_string());
    file.and_then(|f| serde_json::from_reader(f).map(|datas: StockDataResponse| datas.data).map_err(|err| err.to_string()))
}


async fn doSmth() -> Result<(), Error> {
// let request_url = format!("https://jsonplaceholder.typicode.com/posts");
    // println!("{}", request_url);
    // let response = reqwest::get(&request_url).await?;
    //
    // let users: Vec<User> = response.json().await?;
    // println!("{:#?}", users);
    let time = Utc::now().date().with_month(6).unwrap();
    let symbol = "SPY";
    let datas = open_cache_for_date(&time, symbol).unwrap();
    println!("{:#?}", datas);
    println!("{}", time.format("%F"));
    println!("{}", get_start_nmonth_ago(&time, 5).format("%F"));
    Ok(())
}

async fn fetch(api_key: String, symbol: String, to_date: Date<Utc>) -> Result<(), Error> {
    let from_date = get_start_nmonth_ago(&to_date, 5);
    Ok(())
}

/// num_month -> [0;12]
fn get_start_nmonth_ago(date: &Date<Utc>, num_months: u32) -> Date<Utc> {
    assert!(num_months > 0 && num_months <= 12);
    let year = if date.month() <= num_months { date.year() - 1 } else { date.year() };
    let month = if date.month() <= num_months { 12 + date.month() - num_months } else { date.month() - num_months };
    // println!("{}-{}-{}", year, month, 1);
    Utc.ymd(year, month, 1)
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    // doSmth().await
    let portfolio_repo = persistence::portfolio::InMemoryPortfolioRepository::new();
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        Box::new(AppGUI::new(Arc::new(portfolio_repo))),
        options,
    );
}


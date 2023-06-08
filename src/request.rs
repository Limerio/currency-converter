use std::collections::HashMap;

use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Exchanges {
    pub amount: f64,
    pub base: String,
    pub date: String,
    pub rates: HashMap<String, f64>,
}

pub async fn get_exchange_rate(from: String, to: String) -> Result<Exchanges, reqwest::Error> {
    let url = {
        if to.is_empty() && !from.is_empty() {
            format!("https://api.frankfurter.app/latest?from={}", from)
        } else if !to.is_empty() && from.is_empty() {
            format!("https://api.frankfurter.app/latest?to={}", to)
        } else {
            format!("https://api.frankfurter.app/latest?from={}&to={}", from, to)
        }
    };

    reqwest::get(url).await.unwrap().json::<Exchanges>().await
}

pub async fn get_exchange_rate_all() -> Result<Exchanges, reqwest::Error> {
    reqwest::get("https://api.frankfurter.app/latest")
        .await
        .unwrap()
        .json::<Exchanges>()
        .await
}

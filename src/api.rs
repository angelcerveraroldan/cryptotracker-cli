use std::collections::HashMap;
use std::fmt::{Debug, format};
use std::iter::repeat_with;
use async_trait::async_trait;
use serde::{de, Deserialize, Serialize};
use serde::de::DeserializeOwned;
use crate::utils;

pub mod apis_models;

pub struct Api {
    name: String,
    base_path: String,
    intervals: HashMap<String, String>,
    get_symbol_path: Option<String>,
    get_candle_path: Option<String>,
}

impl Api {
    pub fn from(
        name: String,
        base_path: String,
        intervals: HashMap<String, String>,
        get_symbol_path: Option<String>,
        get_candle_path: Option<String>,
    ) -> Self {
        Api {
            name,
            base_path,
            intervals,
            get_symbol_path,
            get_candle_path,
        }
    }

    pub fn dydx() -> Self {
        Api {
            name: String::from("dydx"),
            base_path: String::from("https://api.dydx.exchange"),
            intervals: HashMap::from([
                ("1m".to_string(), "1MIN".to_string()),
                ("1h".to_string(), "1HOUR".to_string()),
                ("1d".to_string(), "1DAY".to_string()),
            ]),
            get_symbol_path: Some(String::from("/v3/markets")),
            get_candle_path: Some(String::from("/v3/candles/{symbol}")),
        }
    }

    pub fn mexc() -> Self {
        Api {
            name: String::from("mexc"),
            base_path: String::from("https://www.mexc.com"),
            intervals: HashMap::from([
                ("1m".to_string(), "1MIN".to_string()),
                ("1h".to_string(), "1HOUR".to_string()),
                ("1d".to_string(), "1DAY".to_string()),
            ]),
            get_symbol_path: Some(String::from("")),
            get_candle_path: Some(String::from("/open/api/v2/market/kline")),
        }
    }
}

#[derive(Debug)]
pub struct Candle {
    pub timestamp: u32,
    pub open: f32,
    pub close: f32,
    pub high: f32,
    pub low: f32,
    pub volume: Option<f32>,
}

pub trait SymbolsReply {
    fn get_symbols_arr(&self) -> Vec<String>;
}

pub trait CandleReply {
    fn get_candles_arr(&self) -> Vec<Candle>;
}

#[async_trait]
pub trait ApiMethods {
    /// Get vector with candles
    async fn get_candles<S: CandleReply + DeserializeOwned>(&self, interval: String, params: HashMap<String, String>) -> Vec<Candle>;
    /// Get vector containing the symbols (strings) from api
    async fn get_symbols<S: SymbolsReply + DeserializeOwned>(&self) -> Vec<String>;
}

#[async_trait]
impl ApiMethods for Api {
    async fn get_candles<C: CandleReply + DeserializeOwned>(&self, interval: String, params: HashMap<String, String>) -> Vec<Candle> {
        let url = format!(
            "{}{}",
            self.base_path,
            utils::fix_url(params, self.get_candle_path.clone().unwrap())
        );

        reqwest::get(url)
            .await.expect("Could not get candles")
            .json::<C>()
            .await.expect("Could not parse candles")
            .get_candles_arr()
    }

    async fn get_symbols<S: SymbolsReply + DeserializeOwned>(&self) -> Vec<String> {
        reqwest::get(
            format!("{}{}",
                    self.base_path,
                    self.get_symbol_path
                        .clone()
                        .expect("No path to symbols found")
            ))
            .await.expect("Could not get symbols")
            .json::<S>()
            .await.expect("Could not parse symbols")
            .get_symbols_arr()
    }
}
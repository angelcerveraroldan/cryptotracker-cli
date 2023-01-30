use std::collections::HashMap;
use std::fmt::Debug;
use std::iter::repeat_with;
use async_trait::async_trait;
use serde::{de, Deserialize, Serialize};

pub mod mexc;
pub mod dydx;

pub struct Api {
    name: String,
    base: String,
    intervals: HashMap<String, String>,
    get_symbol_path: Option<String>,
    get_candle_path: Option<String>,
}

impl Api {
    pub fn from(
        name: String,
        base: String,
        intervals: HashMap<String, String>,
        get_symbol_path: Option<String>,
        get_candle_path: Option<String>,
    ) -> Self {
        Api {
            name,
            base,
            intervals,
            get_symbol_path,
            get_candle_path,
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

trait SymbolsReply {
    fn get_symbols_arr(&self) -> Vec<String>;
}

#[async_trait]
pub trait ApiMethods {
    // TODO: Get Candles

    /// Get vector containing the symbols (strings) from api
    async fn get_symbols<S: SymbolsReply + de::DeserializeOwned>(&self) -> Vec<String>;
}

#[async_trait]
impl ApiMethods for Api {
    async fn get_symbols<S: SymbolsReply + de::DeserializeOwned>(&self) -> Vec<String> {
        reqwest::get(
            format!("{}{}",
                    self.base,
                    self.get_symbol_path
                        .as_ref()
                        .expect("No path to symbols found")
            ))
            .await.expect("Could not get symbols")
            .json::<S>()
            .await.expect("Could not parse symbols")
            .get_symbols_arr()
    }
}
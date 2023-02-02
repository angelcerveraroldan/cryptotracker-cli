use std::collections::HashMap;
use serde::{Deserialize, Deserializer};
use crate::api::{Candle, CandleReply, SymbolsReply};

#[derive(Deserialize, Debug)]
pub struct DyDxSymbolsReply {
    markets: HashMap<String, DyDxSymbolReply>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DyDxSymbolReply {
    market: String,
    base_asset: String,
    quote_asset: String,
}

impl SymbolsReply for DyDxSymbolsReply {
    fn get_symbols_arr(&self) -> Vec<String> {
        let mut symbols = Vec::new();

        for (_, v) in &self.markets {
            symbols.push(v.market.clone())
        }

        symbols
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DyDxCandleReply {
    started_at: String,
    resolution: String,
    low: String,
    high: String,
    open: String,
    close: String,
    usd_volume: String,
}

#[derive(Deserialize, Debug)]
pub struct DyDxCandlesReply {
    candles: Vec<DyDxCandleReply>,
}

impl CandleReply for DyDxCandlesReply {
    fn get_candles_arr(&self) -> Vec<Candle> {
        let mut candles: Vec<Candle> = Vec::new();

        for c in self.candles.clone().into_iter() {
            candles.push(Candle {
                timestamp: 0,
                open: c.open.parse::<f32>().unwrap(),
                close: c.close.parse::<f32>().unwrap(),
                high: c.high.parse::<f32>().unwrap(),
                low: c.low.parse::<f32>().unwrap(),
                volume: c.usd_volume.parse::<f32>().ok(),
            })
        }

        candles
    }
}
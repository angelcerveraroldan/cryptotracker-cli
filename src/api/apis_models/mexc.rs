use crate::api::{Api, Candle, CandleReply, SymbolsReply};
use serde::{Deserialize, Deserializer};

#[derive(Deserialize, Debug)]
struct MexcSymbolReply {
    symbol: String,
    symbol_partition: String,
}

#[derive(Deserialize, Debug)]
pub struct MexcSymbolsReply {
    code: u8,
    data: Vec<MexcSymbolReply>,
}

impl SymbolsReply for MexcSymbolsReply {
    fn get_symbols_arr(&self) -> Vec<String> {
        let mut symbols: Vec<String> = Vec::new();

        for i in &self.data {
            symbols.push(i.symbol.clone())
        }

        symbols
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct MexcCandleReply {
    timestamp: u32,
    open: String,
    close: String,
    high: String,
    low: String,
    volume: String,
    amount: String,
}

#[derive(Debug, Deserialize)]
pub struct MexcCandlesReply {
    code: u8,
    data: Vec<MexcCandleReply>,
}

impl CandleReply for MexcCandlesReply {
    fn get_candles_arr(&self) -> Vec<Candle> {
        let mut candles: Vec<Candle> = Vec::new();

        for c in self.data.clone().into_iter() {
            candles.push(Candle {
                timestamp: c.timestamp,
                open: c.open.parse::<f32>().unwrap(),
                close: c.close.parse::<f32>().unwrap(),
                high: c.high.parse::<f32>().unwrap(),
                low: c.low.parse::<f32>().unwrap(),
                volume: c.volume.parse::<f32>().ok(),
            })
        }

        candles
    }
}
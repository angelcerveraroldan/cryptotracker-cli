use crate::api::{Api, Candle, CandlesReply, SymbolsReply};
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

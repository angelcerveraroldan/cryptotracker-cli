use std::collections::HashMap;
use serde::{Deserialize, Deserializer};
use crate::api::SymbolsReply;

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
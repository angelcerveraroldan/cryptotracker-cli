use std::any::Any;
use crate::api::{Api, Candle};
use serde::{de, Deserialize, Deserializer, Serialize};


#[derive(Deserialize, Debug)]
pub struct LiquidSymbolReply {
    quoted_currency: String,
    base_currency: String,
    currency_pair_code: String,
}

impl LiquidSymbolReply {
    pub async fn fill_from_api(api: &Api) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        let get_symbols_url: String = format!(
            "{}{}",
            &api.base,
            &api.get_symbol_path.as_ref().expect("no path found")
        );


        Ok(reqwest::get(get_symbols_url).await?
            .json::<Vec<LiquidSymbolReply>>()
            .await?)
    }

    pub async fn get_symbols_arr() -> Vec<String> {
        let mexc_api: Api = Api {
            name: String::from("mexc"),
            base: String::from("https://api.liquid.com"),
            get_symbol_path: Some(String::from("/products")),
            get_candle_path: Some(String::from("")),
        };

        Self::fill_from_api(&mexc_api)
            .await
            .expect("Could not get symbols from liquid")
            .into_iter()
            .map(|x| x.currency_pair_code)
            .collect()
    }
}

// Candles seem to not be working in the API
pub struct LiquidCandleReply {}

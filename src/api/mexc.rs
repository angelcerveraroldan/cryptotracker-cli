use crate::api::{Api, Candle};
use serde::{de, Deserialize, Deserializer, Serialize};

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

#[derive(Debug, Default, Deserialize)]
pub struct MexcCandleReply {
    pub timestamp: u32,
    pub open: String,
    pub close: String,
    pub high: String,
    pub low: String,
    pub volume: String,
    pub amount: String,
}

impl MexcCandleReply {
    fn to_candle(&self) -> Candle {
        Candle {
            timestamp: self.timestamp,
            open: self.open.parse::<f32>().expect("Could not parse sting to int"),
            close: self.close.parse::<f32>().expect("Could not parse sting to int"),
            high: self.high.parse::<f32>().expect("Could not parse sting to int"),
            low: self.low.parse::<f32>().expect("Could not parse sting to int"),
            volume: Some(self.volume.parse::<f32>().expect("Could not parse sting to int")),
        }
    }
}

impl MexcSymbolsReply {
    // Get the symbols for the Api
    async fn fill_from_api(api: &Api) -> Result<Self, Box<dyn std::error::Error>> {
        let get_symbols_url: String = format!(
            "{}{}",
            &api.base,
            &api.get_symbol_path.as_ref().expect("no path found")
        );

        Ok(reqwest::get(get_symbols_url)
            .await?
            .json::<MexcSymbolsReply>()
            .await?)
    }

    pub async fn get_symbols_arr() -> Vec<String> {
        let mexc_api: Api = Api {
            name: String::from("mexc"),
            base: String::from("https://www.mexc.com"),
            get_symbol_path: Some(String::from("/open/api/v2/market/symbols")),
            get_candle_path: Some(String::from("/open/api/v2/market/kline?symbol=BTC_USDT&interval=1m")),
        };

        Self::fill_from_api(&mexc_api)
            .await
            .expect("Could not get symbols from mexc")
            .data.into_iter()
            .map(|x| x.symbol)
            .collect()
    }
}

#[derive(Deserialize, Debug)]
pub struct MexcCandlesReply {
    code: u8,
    data: Vec<MexcCandleReply>,
}

impl MexcCandlesReply {
    pub async fn fill_from_api(api: &Api) -> Result<Self, Box<dyn std::error::Error>> {
        let symbols = MexcSymbolsReply::get_symbols_arr().await;

        let get_symbols_url: String = format!(
            "{}{}",
            &api.base,
            &api.get_candle_path.as_ref().expect("no path found")
        );

        Ok(reqwest::get(get_symbols_url)
            .await?
            .json::<MexcCandlesReply>()
            .await?)
    }

    pub async fn get_from_api() -> Vec<Candle> {
        let mexc_api: Api = Api {
            name: String::from("mexc"),
            base: String::from("https://www.mexc.com"),
            get_symbol_path: Some(String::from("/open/api/v2/market/symbols")),
            get_candle_path: Some(String::from("/open/api/v2/market/kline?symbol=BTC_USDT&interval=1m")),
        };

        Self::fill_from_api(&mexc_api)
            .await
            .expect("Could not get symbols from mexc")
            .data
            .into_iter()
            .map(|x| x.to_candle())
            .collect()
    }
}

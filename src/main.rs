use std::collections::HashMap;
use crate::api::{Api, ApiMethods};
use crate::api::mexc::{MexcSymbolsReply};
use crate::api::dydx::{DyDxSymbolsReply};

mod api;

#[tokio::main]
async fn main() {
    let dydx_api: Api = Api::from(
        String::from("mexc"),
        String::from("https://api.dydx.exchange"),
        HashMap::new(),
        Some(String::from("/v3/markets")),
        Some(String::from("/open/api/v2/market/kline?symbol=BTC_USDT&interval=1m")),
    );

    let x = dydx_api.get_symbols::<DyDxSymbolsReply>().await;

    println!("{:?}", x);
}

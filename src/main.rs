use std::collections::HashMap;
use crate::api::{Api, ApiMethods};
use crate::api::apis_models::mexc::{MexcSymbolsReply};
use crate::api::apis_models::dydx::{DyDxSymbolsReply};

mod api;
mod utils;

#[tokio::main]
async fn main() {
    let dydx_api: Api = Api::from(
        String::from("dydx"),
        String::from("https://api.dydx.exchange"),
        HashMap::from([
            ("1m".to_string(), "1MIN".to_string()),
            ("1h".to_string(), "1HOUR".to_string()),
            ("1d".to_string(), "1DAY".to_string()),
        ]),
        // Make the path string have parameters which will have to be
        Some(String::from("/v3/markets")),
        Some(String::from("v3/candles/{symbol}")),
    );

    let x = dydx_api.get_symbols::<DyDxSymbolsReply>().await;

    println!("{:?}", x);
}

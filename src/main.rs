use std::collections::HashMap;
use url::form_urlencoded::parse;
use crate::api::{Api, ApiMethods};
use crate::api::apis_models::mexc::{MexcCandlesReply, MexcSymbolsReply};
use crate::api::apis_models::dydx::{DyDxCandleReply, DyDxCandlesReply, DyDxSymbolsReply};

mod api;
mod utils;

#[tokio::main]
async fn main() {
    // let dydx_api: Api = Api::dydx();
    //
    // let x = dydx_api.get_symbols::<DyDxSymbolsReply>().await;
    //
    // let params = HashMap::from([
    //     ("symbol".to_string(), "CELO-USD".to_string()),
    // ]);
    //
    // let y = dydx_api.get_candles::<DyDxCandlesReply>("x".to_string(), params).await;
    //
    // println!("{:?}", y)

    let mexc: Api = Api::mexc();

    let params = HashMap::from([
        ("interval".to_string(), "1,".to_string()),
        ("symbol".to_string(), "PAXG_USDP".to_string())
    ]);

    let c = mexc.get_candles::<MexcCandlesReply>("x".to_string(), params).await;

    println!("{:?}", c)
}

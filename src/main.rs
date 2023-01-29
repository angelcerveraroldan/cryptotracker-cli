use crate::api::Api;
use crate::api::liquid::LiquidSymbolReply;
use crate::api::mexc::{MexcCandlesReply, MexcSymbolsReply};

mod api;

#[tokio::main]
async fn main() {
    let symbols = MexcSymbolsReply::get_symbols_arr().await;

    println!("{:?}", MexcCandlesReply::get_from_api().await)
}

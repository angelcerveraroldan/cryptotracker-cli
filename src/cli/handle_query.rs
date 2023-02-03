use crate::cli::get_query::{get_query, Query};
use crate::cli::args::Exchange;
use crate::api::*;
use crate::api::apis_models::dydx::DyDxSymbolsReply;

pub async fn handle_query() {
    let query = get_query();

    println!("we got here");

    match query {
        Query::Get(getQuery) => {}
        Query::List(exchange) => handle_list(exchange).await
    }
}

async fn handle_list(exchange: Exchange) {
    let api = exchange.get_api();

    let symbols = api.get_symbols::<DyDxSymbolsReply>().await;

    println!("{:?}", symbols);
}

use crate::cli::args::{Args, Exchange};
use clap::Parser;

pub struct GetQuery {
    exchange: Exchange,
    symbol: String,
}

pub enum Query {
    Get(GetQuery),
    List(Exchange),
}

pub fn get_query() -> Query {
    let args = Args::parse();

    let exchange = match args.exchange.to_lowercase().as_str() {
        "dydx" => Exchange::DyDx,
        "mexc" => Exchange::Mexc,
        _ => panic!("Exchange not found")
    };

    match args.command.to_lowercase().as_str() {
        "get" => Query::Get(GetQuery {
            exchange,
            symbol: args.market.expect("No market has been specified, so candles cannot be obtained"),
        }),
        "list" => Query::List(exchange),
        _ => panic!("<COMMAND> should be either 'list' or 'get'")
    }
}
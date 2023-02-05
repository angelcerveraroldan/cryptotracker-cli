use std::process::exit;
use crate::cli::args::{Args, Exchange};
use clap::Parser;

pub struct GetQuery {
    pub exchange: Exchange,
    pub symbol: String,
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
        _ => {
            println!("<EXCHANGE> should be either 'dydx' or 'mexc'");
            exit(1);
        }
    };

    match args.command.to_lowercase().as_str() {
        "get" => Query::Get(GetQuery {
            exchange,
            symbol: args.market.expect("No market has been specified, so candles cannot be obtained"),
        }),
        "list" => Query::List(exchange),
        _ => {
            println!("<COMMAND> should be either 'list' or 'get'");
            exit(1);
        }
    }
}
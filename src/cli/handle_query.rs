use std::collections::HashMap;
use crate::cli::get_query::{get_query, GetQuery, Query};
use crate::cli::args::Exchange;
use crate::api::*;
use crate::api::apis_models::dydx::{DyDxCandlesReply, DyDxSymbolsReply};
use crate::api::apis_models::mexc::{MexcCandlesReply, MexcSymbolsReply};

pub async fn handle_query() {
    let query = get_query();

    match query {
        Query::Get(get_query) => handle_get(get_query).await,
        Query::List(exchange) => handle_list(exchange).await
    }
}

/*
TODO: Change the generics

The following code
```rust
let symbols = match exchange {
    Exchange::DyDx => api.get_symbols::<DyDxSymbolsReply>().await,
    Exchange::Mexc => api.get_symbols::<MexcSymbolsReply>().await,
};
```

needs to be changet to this
```rust
let symbols = match exchange {
    Exchange::DyDx => api.get_symbols().await,
    Exchange::Mexc => api.get_symbols().await,
};
```

The struct should be a part of the API, and so it wont need to be passed into the function
 */

async fn handle_list(exchange: Exchange) {
    let api = exchange.get_api();

    let symbols = match exchange {
        Exchange::DyDx => api.get_symbols::<DyDxSymbolsReply>().await,
        Exchange::Mexc => api.get_symbols::<MexcSymbolsReply>().await,
    };

    println!("{:?}", symbols);
}

async fn handle_get(get_query: GetQuery) {
    let api = get_query.exchange.get_api();
    let symbol = get_query.symbol;

    let interval = "1m".to_string();
    let params = HashMap::from([("symbol".to_string(), symbol)]);

    let candles = match get_query.exchange {
        Exchange::DyDx => api.get_candles::<DyDxCandlesReply>(interval, params).await,
        Exchange::Mexc => api.get_candles::<MexcCandlesReply>(interval, params).await,
    };

    println!("{:?}", candles)
}
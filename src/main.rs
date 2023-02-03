extern crate core;

use std::collections::HashMap;
use clap::Parser;
use url::form_urlencoded::parse;
use crate::api::{Api, ApiMethods};
use crate::api::apis_models::mexc::{MexcCandlesReply, MexcSymbolsReply};
use crate::api::apis_models::dydx::{DyDxCandleReply, DyDxCandlesReply, DyDxSymbolsReply};
use crate::cli::handle_query::handle_query;

mod api;
mod utils;
mod cli;

#[tokio::main]
async fn main() {
    handle_query().await;
}

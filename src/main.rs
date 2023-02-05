extern crate core;

use crate::cli::handle_query::handle_query;

mod api;
mod utils;
mod cli;

#[tokio::main]
async fn main() {
    handle_query().await;
}

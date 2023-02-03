use clap::Parser;
use crate::api::Api;

/// Sample requests
/// - rccli get dydx BTC_USDT
/// - rccli get dydx BTC_USDT from YYYY-MM-DD
/// - rccli list dydx
#[derive(Parser)]
pub struct Args {
    /// Command to be run, one of the following:
    /// 1. get
    /// 2. list
    pub(crate) command: String,
    /// Exchange, such as dydx or mexc
    #[arg(short, long)]
    pub(crate) exchange: String,
    /// Market, such as BTC_USDT
    /// To you can list the available markets for any given exchange
    #[arg(short, long)]
    pub(crate) market: Option<String>,
    /// Candlestics starting date
    #[arg(short, long)]
    pub(crate) starting_date: Option<String>,
}

pub enum Exchange {
    DyDx,
    Mexc,
}

impl Exchange {
    pub fn get_api(&self) -> Api {
        match self {
            Exchange::DyDx => Api::dydx(),
            Exchange::Mexc => Api::mexc(),
        }
    }
}
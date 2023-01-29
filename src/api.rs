pub mod mexc;
pub mod liquid;

pub struct Api {
    name: String,
    base: String,
    get_symbol_path: Option<String>,
    get_candle_path: Option<String>,
}

#[derive(Debug)]
pub struct Candle {
    pub timestamp: u32,
    pub open: f32,
    pub close: f32,
    pub high: f32,
    pub low: f32,
    pub volume: Option<f32>,
}

impl Api {
    pub fn from(
        name: String,
        base: String,
        get_symbol_path: Option<String>,
        get_candle_path: Option<String>,
    ) -> Self {
        Api {
            name,
            base,
            get_symbol_path,
            get_candle_path,
        }
    }
}
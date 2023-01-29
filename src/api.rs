pub mod mexc;

pub struct Api {
    name: String,
    base: String,
    get_symbol_path: Option<String>,
    get_candle_path: Option<String>,
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
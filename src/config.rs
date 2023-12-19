use std::collections::HashMap;

pub struct AppConfig {
    pub active_ticker: String,
    pub available_tickers: HashMap<String, String>,
}

pub fn init_config() -> AppConfig {
    let mut app_config = AppConfig {
        active_ticker: "btcusdt".to_string(),
        available_tickers: HashMap::new(),
    };

    app_config.available_tickers.insert("btcusdt".to_string(), "wss://stream.binance.com:9443/ws/btcusdt@trade".to_string());
    app_config.available_tickers.insert("ethusdt".to_string(), "wss://stream.binance.com:9443/ws/ethusdt@trade".to_string());

    app_config
}

use tokio::sync::mpsc;
use tokio::runtime::Runtime;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;
use futures::StreamExt;

use std::collections::HashMap;

struct AppConfig {
    active_ticker: String,
    available_tickers: HashMap<String, String>,
}


async fn process_websocket(config: AppConfig, tx: tokio::sync::mpsc::Sender<String>) {
    let url = &config.available_tickers[&config.active_ticker];
    let url = Url::parse(url).expect("Failed to parse URL");

    let (mut socket, _) = connect_async(url)
        .await
        .expect("Failed to connect");

    println!("Connected to the WebSocket stream.");

    while let Some(message) = socket.next().await {
        match message {
            Ok(msg) => {
                if let Message::Text(text) = msg {
                    tx.send(text).await.unwrap();
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
}

fn main() {
    let mut app_config = AppConfig {
        active_ticker: "btcusdt".to_string(),
        available_tickers: HashMap::new(),
    };

    app_config.available_tickers.insert("btcusdt".to_string(), "wss://stream.binance.com:9443/ws/btcusdt@trade".to_string());
    app_config.available_tickers.insert("ethusdt".to_string(), "wss://stream.binance.com:9443/ws/ethusdt@trade".to_string());

    // Create a multi-threaded Tokio runtime
    let rt = Runtime::new().unwrap();
    
    rt.block_on(async {
        // Create an asynchronous channel
        let (tx, mut rx) = mpsc::channel(32); // Adjust the channel size as needed

        // Spawn the WebSocket processing in a separate task
        tokio::spawn(async move {
            process_websocket(app_config, tx).await;
        });

        // Handle received messages in the main async context
        while let Some(message) = rx.recv().await {
            println!("Received: {}", message);
            // Process the message here
        }
    });
}

use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;
use futures::StreamExt;
use crate::config::AppConfig; // Make sure to import AppConfig

pub async fn process_websocket(config: AppConfig, tx: mpsc::Sender<String>) {
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

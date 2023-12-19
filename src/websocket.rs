use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, WebSocketStream};
use url::Url;
use futures::StreamExt;
use crate::config::AppConfig;

pub async fn connect_to_websocket(url: &str) -> WebSocketStream<impl tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin> {
    let url = Url::parse(url).expect("Failed to parse URL");
    connect_async(url).await.expect("Failed to connect").0
}

pub async fn process_websocket(config: AppConfig, tx: mpsc::Sender<String>) {
    let mut socket = connect_to_websocket(&config.available_tickers[&config.active_ticker]).await;

    while let Some(message) = socket.next().await {
        match message {
            Ok(msg) => {
                if let Message::Text(text) = msg {
                    tx.send(text).await.expect("Failed to send message");
                }
            },
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

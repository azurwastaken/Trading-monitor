mod config;      // Import the config module
mod websocket;   // Import the websocket module

use tokio::sync::mpsc;
use tokio::runtime::Runtime;
use config::init_config;
use websocket::process_websocket; 


fn main() {
    let app_config = init_config();

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

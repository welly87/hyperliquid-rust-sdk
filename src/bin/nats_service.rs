use async_nats::connect;

use hyperliquid_rust_sdk::messages::OrderRequest;
use log::{error, info, LevelFilter};
use futures_util::stream::StreamExt;
use std::env;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .init();

    // Get NATS URL from environment or use default
    let nats_url = env::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".to_string());
    let subject = env::var("NATS_SUBJECT").unwrap_or_else(|_| "hyperliquid.orders".to_string());
    
    // Get HyperLiquid API URL from environment or use default
    let api_url = env::var("HYPERLIQUID_API_URL")
        .unwrap_or_else(|_| "https://api.hyperliquid.xyz".to_string());

    info!("Connecting to NATS server at {}", nats_url);
    let nc = connect(&nats_url).await?;
    info!("Connected to NATS server");

    info!("Subscribing to subject: {}", subject);
    let mut sub = nc.subscribe(subject.clone()).await?;
    info!("Subscribed to {}", subject);

    info!("NATS service started. Waiting for messages...");

    // Process incoming messages
    while let Some(msg) = sub.next().await {
        match process_message(&msg, &api_url).await {
            Ok(_) => info!("Successfully processed message: {:?}", msg),
            Err(e) => error!("Error processing message: {}", e),
        }
    }

    Ok(())
}

async fn process_message(
    msg: &async_nats::Message,
    api_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Extract and log message headers if present
    let headers = msg.headers.as_ref();
    let message_type = headers
        .and_then(|h| h.get("message_type").cloned())
        .map(|hv| hv.to_string())
        .unwrap_or_else(|| "order".to_string());
    
    let correlation_id = headers
        .and_then(|h| h.get("correlation_id").cloned());

    info!("Received message with type: {}, correlation_id: {:?}", 
          message_type, correlation_id);

    let payload = String::from_utf8(msg.payload.to_vec())?;
    info!("Message payload: {}", payload);

    let order_req: OrderRequest = serde_json::from_str(&payload)?;
    info!("Processing order request: {:?}", order_req);

    if let Some(limit_px) = &order_req.limit_price {
        info!("Processing limit order: {:?} at {}", order_req, limit_px);
        // TODO: client.limit_order(...).await?;
    } else {
        info!("Processing market order: {:?}", order_req);
        // TODO: client.market_order(...).await?;
    }
    Ok(())
}

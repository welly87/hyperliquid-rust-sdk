use async_nats::connect;
use ethers::signers::LocalWallet;
use futures_util::stream::StreamExt;
use hyperliquid_rust_sdk::messages::{ExchangeMessage, Message};
use hyperliquid_rust_sdk::{
    messages::{
        ApproveAgentRequest, ApproveBuilderFeeRequest, CancelOrderRequest, ClassTransferRequest,
        MessageHeader, MessageType, OrderRequest, SetReferrerRequest, TransferRequest,
        UpdateIsolatedMarginRequest, UpdateLeverageRequest, VaultTransferRequest, WithdrawRequest,
    },
    BaseUrl, ClientCancelRequest, ClientCancelRequestCloid, ClientLimit, ClientOrder,
    ClientOrderRequest, ExchangeClient, MarketOrderParams,
};
use log::{error, info, LevelFilter};
use std::env;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .init();

    let nats_url = env::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".to_string());
    let subject = env::var("NATS_SUBJECT").unwrap_or_else(|_| "hyperliquid.orders".to_string());

    let priv_key = env::var("PRIVATE_KEY").unwrap_or_else(|_| {
        "e908f86dbb4d55ac876378565aafeabc187f6690f046459397b17d9b9a19688e".to_string()
    });
    let wallet: LocalWallet = priv_key.parse()?;

    let base = match env::var("BASE_URL")
        .unwrap_or_else(|_| "mainnet".to_string())
        .to_lowercase()
        .as_str()
    {
        "testnet" => BaseUrl::Testnet,
        "localhost" => BaseUrl::Localhost,
        _ => BaseUrl::Mainnet,
    };

    let client = ExchangeClient::new(None, wallet, Some(base), None, None).await?;

    info!("Connecting to NATS server at {}", nats_url);
    let nc = connect(&nats_url).await?;
    info!("Connected to NATS server");

    let mut sub = nc.subscribe(subject.clone()).await?;
    info!("Subscribed to {}", subject);
    info!("NATS service started. Waiting for messages...");

    while let Some(msg) = sub.next().await {
        if let Err(e) = process_message(&msg, &client).await {
            error!("Error processing message: {}", e);
        }
    }
    Ok(())
}

async fn process_message(
    msg: &async_nats::Message,
    client: &ExchangeClient,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = &msg.payload;
    if data.len() < 4 {
        return Err("message too short".into());
    }
    let header_len = u32::from_be_bytes([data[0], data[1], data[2], data[3]]) as usize;
    let header: MessageHeader = rmp_serde::from_slice(&data[4..4 + header_len])?;

    match header.msg_type {
        MessageType::Order => {
            let req = <OrderRequest as ExchangeMessage>::from_msgpack(data)?;
            handle_order(req, client).await?
        }
        MessageType::CancelOrder => {
            let req = <CancelOrderRequest as ExchangeMessage>::from_msgpack(data)?;
            handle_cancel(req, client).await?
        }
        MessageType::ModifyOrder => {
            log::warn!("modify order message handling not implemented");
        }
        MessageType::UpdateLeverage => {
            let req = <UpdateLeverageRequest as ExchangeMessage>::from_msgpack(data)?;
            client
                .update_leverage(req.leverage, &req.asset, req.is_cross, None)
                .await?;
        }
        MessageType::Transfer => {
            let req = <TransferRequest as ExchangeMessage>::from_msgpack(data)?;
            if req.asset.to_uppercase() == "USDC" {
                client
                    .usdc_transfer(&req.amount, &req.destination, None)
                    .await?;
            } else {
                // treat as spot transfer with token in asset field
                client
                    .spot_transfer(&req.amount, &req.destination, &req.asset, None)
                    .await?;
            }
        }
        MessageType::Withdraw => {
            let req = <WithdrawRequest as ExchangeMessage>::from_msgpack(data)?;
            client
                .withdraw_from_bridge(&req.amount, &req.destination, None)
                .await?;
        }
        MessageType::ClassTransfer => {
            // Try class transfer first
            if let Ok(req) = <ClassTransferRequest as ExchangeMessage>::from_msgpack(data) {
                client.class_transfer(req.amount, req.to_perp, None).await?;
            } else if let Ok(req) = <VaultTransferRequest as ExchangeMessage>::from_msgpack(data) {
                let addr = req.vault_address.as_deref().and_then(|a| a.parse().ok());
                client
                    .vault_transfer(req.is_deposit, req.usd, addr, None)
                    .await?;
            } else {
                log::warn!("Unknown class transfer message");
            }
        }
        MessageType::UpdateIsolatedMargin => {
            let req = <UpdateIsolatedMarginRequest as ExchangeMessage>::from_msgpack(data)?;
            client
                .update_isolated_margin(req.amount, &req.asset, None)
                .await?;
        }
        MessageType::ApproveAgent => {
            let _req = <ApproveAgentRequest as ExchangeMessage>::from_msgpack(data)?;
            let (_key, _res) = client.approve_agent(None).await?;
            info!("Approved agent: {}", _key);
        }
        MessageType::SetReferrer => {
            let req = <SetReferrerRequest as ExchangeMessage>::from_msgpack(data)?;
            client.set_referrer(req.code, None).await?;
        }
        MessageType::ApproveBuilderFee => {
            let req = <ApproveBuilderFeeRequest as ExchangeMessage>::from_msgpack(data)?;
            client
                .approve_builder_fee(req.builder, req.max_fee_rate, None)
                .await?;
        }
    }
    Ok(())
}

async fn handle_order(
    req: OrderRequest,
    client: &ExchangeClient,
) -> Result<(), Box<dyn std::error::Error>> {
    let sz = req.size.parse::<f64>()?;
    let cloid = match &req.cloid {
        Some(c) => Some(Uuid::parse_str(c)?),
        None => None,
    };

    if let Some(px) = req.limit_price {
        let px = px.parse::<f64>()?;
        let order = ClientOrderRequest {
            asset: req.asset,
            is_buy: req.is_buy,
            reduce_only: req.reduce_only,
            limit_px: px,
            sz,
            cloid,
            order_type: ClientOrder::Limit(ClientLimit {
                tif: req.time_in_force,
            }),
        };
        client.order(order, None).await?;
    } else {
        let params = MarketOrderParams {
            asset: &req.asset,
            is_buy: req.is_buy,
            sz,
            px: None,
            slippage: None,
            cloid,
            wallet: None,
        };
        client.market_open(params).await?;
    }
    Ok(())
}

async fn handle_cancel(
    req: CancelOrderRequest,
    client: &ExchangeClient,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(oid) = req.order_id {
        let cancel = ClientCancelRequest {
            asset: req.asset,
            oid,
        };
        client.cancel(cancel, None).await?;
    } else if let Some(cloid) = req.cloid {
        let cancel = ClientCancelRequestCloid {
            asset: req.asset,
            cloid: Uuid::parse_str(&cloid)?,
        };
        client.cancel_by_cloid(cancel, None).await?;
    }
    Ok(())
}

//! Order-related message types

use serde::{Deserialize, Serialize};

use crate::messages::ExchangeMessage;

use super::MessageType;

/// Request to place a new order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderRequest {
    /// The asset to trade (e.g., "BTC")
    pub asset: String,

    /// Whether this is a buy order (true) or sell order (false)
    pub is_buy: bool,

    /// Size of the order in the asset's base unit
    pub size: String,

    /// Limit price (required for limit orders)
    pub limit_price: Option<String>,

    /// Client order ID (optional)
    pub cloid: Option<String>,

    /// Whether this is a reduce-only order
    pub reduce_only: bool,

    /// Time in force (e.g., "Gtc", "Ioc", "Fok")
    pub time_in_force: String,
}

impl OrderRequest {
    /// Create a new market order request
    pub fn market(asset: &str, is_buy: bool, size: &str) -> Self {
        Self {
            asset: asset.to_string(),
            is_buy,
            size: size.to_string(),
            limit_price: None,
            cloid: None,
            reduce_only: false,
            time_in_force: "Ioc".to_string(),
        }
    }

    /// Create a new limit order request
    pub fn limit(asset: &str, is_buy: bool, size: &str, price: &str) -> Self {
        Self {
            asset: asset.to_string(),
            is_buy,
            size: size.to_string(),
            limit_price: Some(price.to_string()),
            cloid: None,
            reduce_only: false,
            time_in_force: "Gtc".to_string(),
        }
    }

    /// Set a client order ID
    pub fn with_cloid(mut self, cloid: &str) -> Self {
        self.cloid = Some(cloid.to_string());
        self
    }

    /// Set reduce-only flag
    pub fn with_reduce_only(mut self, reduce_only: bool) -> Self {
        self.reduce_only = reduce_only;
        self
    }

    /// Set time in force
    pub fn with_time_in_force(mut self, tif: &str) -> Self {
        self.time_in_force = tif.to_string();
        self
    }
}

impl ExchangeMessage for OrderRequest {
    fn message_type_str(&self) -> &'static str {
        if self.limit_price.is_some() {
            "limit_order"
        } else {
            "market_order"
        }
    }

    fn message_type() -> MessageType {
        MessageType::Order
    }
}

/// Request to cancel an order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelOrderRequest {
    /// The asset of the order to cancel
    pub asset: String,

    /// The order ID to cancel (either this or cloid must be provided)
    pub order_id: Option<u64>,

    /// The client order ID to cancel (either this or order_id must be provided)
    pub cloid: Option<String>,
}

impl CancelOrderRequest {
    /// Create a new cancel request by order ID
    pub fn by_order_id(asset: &str, order_id: u64) -> Self {
        Self {
            asset: asset.to_string(),
            order_id: Some(order_id),
            cloid: None,
        }
    }

    /// Create a new cancel request by client order ID
    pub fn by_cloid(asset: &str, cloid: &str) -> Self {
        Self {
            asset: asset.to_string(),
            order_id: None,
            cloid: Some(cloid.to_string()),
        }
    }
}

impl ExchangeMessage for CancelOrderRequest {
    fn message_type_str(&self) -> &'static str {
        "cancel_order"
    }

    fn message_type() -> MessageType {
        MessageType::CancelOrder
    }
}

/// Request to modify an existing order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyOrderRequest {
    /// The order ID to modify (either this or cloid must be provided)
    pub order_id: Option<u64>,

    /// The client order ID to modify (either this or order_id must be provided)
    pub cloid: Option<String>,

    /// New size for the order (optional)
    pub new_size: Option<String>,

    /// New price for the order (optional)
    pub new_price: Option<String>,
}

impl ModifyOrderRequest {
    /// Create a new modify request by order ID
    pub fn by_order_id(order_id: u64) -> Self {
        Self {
            order_id: Some(order_id),
            cloid: None,
            new_size: None,
            new_price: None,
        }
    }

    /// Create a new modify request by client order ID
    pub fn by_cloid(cloid: &str) -> Self {
        Self {
            order_id: None,
            cloid: Some(cloid.to_string()),
            new_size: None,
            new_price: None,
        }
    }

    /// Set the new size
    pub(crate) fn with_size(mut self, size: &str) -> Self {
        self.new_size = Some(size.to_string());
        self
    }

    /// Set the new price
    pub(crate) fn with_price(mut self, price: &str) -> Self {
        self.new_price = Some(price.to_string());
        self
    }
}

impl ExchangeMessage for ModifyOrderRequest {
    fn message_type_str(&self) -> &'static str {
        "modify_order"
    }

    fn message_type() -> MessageType {
        MessageType::ModifyOrder
    }
}

/// Request to update leverage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateLeverageRequest {
    /// The asset to update leverage for
    pub asset: String,

    /// The new leverage value
    pub leverage: u32,

    /// Whether to use cross margin (true) or isolated margin (false)
    pub is_cross: bool,
}

impl UpdateLeverageRequest {
    /// Create a new update leverage request
    pub fn new(asset: &str, leverage: u32, is_cross: bool) -> Self {
        Self {
            asset: asset.to_string(),
            leverage,
            is_cross,
        }
    }
}

impl ExchangeMessage for UpdateLeverageRequest {
    fn message_type_str(&self) -> &'static str {
        "update_leverage"
    }

    fn message_type() -> MessageType {
        MessageType::UpdateLeverage
    }
}

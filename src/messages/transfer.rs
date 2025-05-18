//! Transfer-related message types

use serde::{Deserialize, Serialize};

use crate::messages::ExchangeMessage;

use super::MessageType;

/// Request to transfer funds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRequest {
    /// The asset to transfer (e.g., "USDC")
    pub asset: String,
    
    /// The amount to transfer
    pub amount: String,
    
    /// The destination address
    pub destination: String,
}

impl TransferRequest {
    /// Create a new transfer request
    pub fn new(asset: &str, amount: &str, destination: &str) -> Self {
        Self {
            asset: asset.to_string(),
            amount: amount.to_string(),
            destination: destination.to_string(),
        }
    }
}

impl ExchangeMessage for TransferRequest {
    fn message_type_str(&self) -> &'static str {
        "transfer"
    }
    
    fn message_type() -> MessageType {
        // This will be overridden by the impl_message! macro
        unreachable!()
    }
}

/// Request to withdraw from the bridge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawRequest {
    /// The asset to withdraw (e.g., "USDC")
    pub asset: String,
    
    /// The amount to withdraw
    pub amount: String,
    
    /// The destination address
    pub destination: String,
}

impl WithdrawRequest {
    /// Create a new withdraw request
    pub fn new(asset: &str, amount: &str, destination: &str) -> Self {
        Self {
            asset: asset.to_string(),
            amount: amount.to_string(),
            destination: destination.to_string(),
        }
    }
}

impl ExchangeMessage for WithdrawRequest {
    fn message_type_str(&self) -> &'static str {
        "withdraw"
    }
    
    fn message_type() -> MessageType {
        // This will be overridden by the impl_message! macro
        unreachable!()
    }
}

/// Request to transfer between spot and perp accounts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassTransferRequest {
    /// The amount to transfer (in USD)
    pub amount: f64,
    
    /// Whether to transfer to perp (true) or to spot (false)
    pub to_perp: bool,
}

impl ClassTransferRequest {
    /// Create a new class transfer request
    pub fn new(amount: f64, to_perp: bool) -> Self {
        Self {
            amount,
            to_perp,
        }
    }
}

impl ExchangeMessage for ClassTransferRequest {
    fn message_type_str(&self) -> &'static str {
        "class_transfer"
    }
    
    fn message_type() -> MessageType {
        // This will be overridden by the impl_message! macro
        unreachable!()
    }
}

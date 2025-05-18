//! Account-related message types

use serde::{Deserialize, Serialize};

use crate::messages::ExchangeMessage;

use super::MessageType;

/// Request to update isolated margin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateIsolatedMarginRequest {
    /// The asset to update margin for
    pub asset: String,
    
    /// The amount to add (positive) or remove (negative)
    pub amount: f64,
}

impl UpdateIsolatedMarginRequest {
    /// Create a new update isolated margin request
    pub fn new(asset: &str, amount: f64) -> Self {
        Self {
            asset: asset.to_string(),
            amount,
        }
    }
}

impl ExchangeMessage for UpdateIsolatedMarginRequest {
    fn message_type_str(&self) -> &'static str {
        "update_isolated_margin"
    }
    
    fn message_type() -> MessageType {
        // This will be overridden by the impl_message! macro
        unreachable!()
    }
}

/// Request to approve an agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApproveAgentRequest {
    /// The agent address to approve
    pub agent_address: String,
    
    /// Optional agent name
    pub agent_name: Option<String>,
}

impl ApproveAgentRequest {
    /// Create a new approve agent request
    pub fn new(agent_address: &str) -> Self {
        Self {
            agent_address: agent_address.to_string(),
            agent_name: None,
        }
    }
    
    /// Set the agent name
    pub fn with_agent_name(mut self, name: &str) -> Self {
        self.agent_name = Some(name.to_string());
        self
    }
}

impl ExchangeMessage for ApproveAgentRequest {
    fn message_type_str(&self) -> &'static str {
        "approve_agent"
    }
    
    fn message_type() -> MessageType {
        // This will be overridden by the impl_message! macro
        unreachable!()
    }
}

/// Request to set a referrer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetReferrerRequest {
    /// The referrer code
    pub code: String,
}

impl SetReferrerRequest {
    /// Create a new set referrer request
    pub fn new(code: &str) -> Self {
        Self {
            code: code.to_string(),
        }
    }
}

impl ExchangeMessage for SetReferrerRequest {
    fn message_type_str(&self) -> &'static str {
        "set_referrer"
    }
    
    fn message_type() -> MessageType {
        // This will be overridden by the impl_message! macro
        unreachable!()
    }
}

/// Request to approve builder fee
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApproveBuilderFeeRequest {
    /// The builder address
    pub builder: String,
    
    /// The maximum fee rate (as a string to handle decimal precision)
    pub max_fee_rate: String,
}

impl ApproveBuilderFeeRequest {
    /// Create a new approve builder fee request
    pub fn new(builder: &str, max_fee_rate: &str) -> Self {
        Self {
            builder: builder.to_string(),
            max_fee_rate: max_fee_rate.to_string(),
        }
    }
}

impl ExchangeMessage for ApproveBuilderFeeRequest {
    fn message_type_str(&self) -> &'static str {
        "approve_builder_fee"
    }
    
    fn message_type() -> MessageType {
        // This will be overridden by the impl_message! macro
        unreachable!()
    }
}

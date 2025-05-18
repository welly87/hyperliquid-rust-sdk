//! Message types for the HyperLiquid Rust SDK
//! 
//! This module contains all message types that map to the public API of `ExchangeClient`.

mod types;
mod header;
mod order;
mod transfer;
mod account;

pub use types::*;
pub use header::MessageHeader;
pub use order::*;
pub use transfer::*;
pub use account::*;

use serde::{Deserialize, Serialize};


/// Trait for all exchange messages that can be serialized/deserialized
pub trait ExchangeMessage: Serialize + for<'de> Deserialize<'de> + std::fmt::Debug + Send + Sync + 'static {
    /// Returns the message type as a string (for backward compatibility)
    fn message_type_str(&self) -> &'static str;
    
    /// Get the message type
    fn message_type() -> MessageType
    where
        Self: Sized;
    
    /// Serialize the message to a byte vector with header
    fn to_msgpack(&self) -> Result<Vec<u8>, MessageError> {
        // Create the message header
        let header = MessageHeader::new(Self::message_type());
        
        // Serialize the header and body
        let header_bytes = rmp_serde::to_vec_named(&header)?;
        let body_bytes = rmp_serde::to_vec_named(self)?;
        
        // Combine header length (4 bytes), header, and body
        let mut result = Vec::with_capacity(4 + header_bytes.len() + body_bytes.len());
        result.extend_from_slice(&(header_bytes.len() as u32).to_be_bytes());
        result.extend(header_bytes);
        result.extend(body_bytes);
        
        Ok(result)
    }
    
    /// Deserialize a message from a byte slice
    fn from_msgpack(data: &[u8]) -> Result<Self, MessageError>
    where
        Self: Sized,
    {
        if data.len() < 4 {
            return Err(MessageError::InvalidFormat("Message too short".to_string()));
        }
        
        // Read header length (first 4 bytes, big endian)
        let header_len = u32::from_be_bytes([data[0], data[1], data[2], data[3]]) as usize;
        
        if data.len() < 4 + header_len {
            return Err(MessageError::InvalidFormat("Incomplete header".to_string()));
        }
        
        // Deserialize the message body
        rmp_serde::from_slice(&data[4 + header_len..])
            .map_err(MessageError::Decode)
    }
}


// Implement Message trait for all ExchangeMessage types
macro_rules! impl_message {
    ($t:ty, $msg_type:expr) => {
        impl crate::messages::types::Message for $t {
            fn message_type() -> crate::messages::types::MessageType {
                $msg_type
            }
            
            fn from_msgpack(data: &[u8]) -> Result<Self, crate::messages::types::MessageError>
            where
                Self: Sized,
            {
                <Self as crate::messages::types::Message>::default_from_msgpack(data)
            }
        }
    };
}

// Implement Message for order messages
impl_message!(OrderRequest, crate::messages::types::MessageType::Order);
impl_message!(CancelOrderRequest, crate::messages::types::MessageType::CancelOrder);
impl_message!(ModifyOrderRequest, crate::messages::types::MessageType::ModifyOrder);
impl_message!(UpdateLeverageRequest, crate::messages::types::MessageType::UpdateLeverage);

// Implement Message for transfer messages
impl_message!(TransferRequest, crate::messages::types::MessageType::Transfer);

impl_message!(WithdrawRequest, MessageType::Withdraw);
impl_message!(ClassTransferRequest, MessageType::ClassTransfer);

// Implement Message for account messages
impl_message!(UpdateIsolatedMarginRequest, MessageType::UpdateIsolatedMargin);
impl_message!(ApproveAgentRequest, MessageType::ApproveAgent);
impl_message!(SetReferrerRequest, MessageType::SetReferrer);
impl_message!(ApproveBuilderFeeRequest, MessageType::ApproveBuilderFee);

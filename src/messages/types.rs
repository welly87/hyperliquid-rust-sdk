//! Message types and serialization utilities

use chrono::Utc;
use rmp_serde::{decode, encode};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

/// Message type identifiers
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    // Order messages (0x0-0x0F)
    Order = 0x01,
    CancelOrder = 0x02,
    ModifyOrder = 0x03,
    UpdateLeverage = 0x04,
    
    // Transfer messages (0x10-0x1F)
    Transfer = 0x10,
    Withdraw = 0x11,
    ClassTransfer = 0x12,
    
    // Account messages (0x20-0x2F)
    UpdateIsolatedMargin = 0x20,
    ApproveAgent = 0x21,
    SetReferrer = 0x22,
    ApproveBuilderFee = 0x23,
}

impl TryFrom<u8> for MessageType {
    type Error = MessageError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(MessageType::Order),
            0x02 => Ok(MessageType::CancelOrder),
            0x03 => Ok(MessageType::ModifyOrder),
            0x04 => Ok(MessageType::UpdateLeverage),
            0x10 => Ok(MessageType::Transfer),
            0x11 => Ok(MessageType::Withdraw),
            0x12 => Ok(MessageType::ClassTransfer),
            0x20 => Ok(MessageType::UpdateIsolatedMargin),
            0x21 => Ok(MessageType::ApproveAgent),
            0x22 => Ok(MessageType::SetReferrer),
            0x23 => Ok(MessageType::ApproveBuilderFee),
            _ => Err(MessageError::InvalidMessageType(value)),
        }
    }
}

impl From<MessageType> for u8 {
    fn from(msg_type: MessageType) -> u8 {
        msg_type as u8
    }
}

impl std::fmt::Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageType::Order => write!(f, "Order"),
            MessageType::CancelOrder => write!(f, "CancelOrder"),
            MessageType::ModifyOrder => write!(f, "ModifyOrder"),
            MessageType::UpdateLeverage => write!(f, "UpdateLeverage"),
            MessageType::Transfer => write!(f, "Transfer"),
            MessageType::Withdraw => write!(f, "Withdraw"),
            MessageType::ClassTransfer => write!(f, "ClassTransfer"),
            MessageType::UpdateIsolatedMargin => write!(f, "UpdateIsolatedMargin"),
            MessageType::ApproveAgent => write!(f, "ApproveAgent"),
            MessageType::SetReferrer => write!(f, "SetReferrer"),
            MessageType::ApproveBuilderFee => write!(f, "ApproveBuilderFee"),
        }
    }
}

/// Message header that will be prepended to all messages
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageHeader {
    /// Type of the message
    pub msg_type: MessageType,
    /// Unique message ID (UUID v4 as bytes)
    pub msg_id: [u8; 16],
    /// Optional correlation ID for request/response matching
    pub correlation_id: Option<[u8; 16]>,
    /// Timestamp in milliseconds since epoch
    pub timestamp: u64,
    /// Expiration time in milliseconds since epoch
    pub expires_at: u64,
}

impl MessageHeader {
    /// Create a new message header
    pub(crate) fn new(msg_type: MessageType) -> Self {
        let uuid = Uuid::new_v4();
        let now = Utc::now().timestamp_millis() as u64;
        
        Self {
            msg_type,
            msg_id: *uuid.as_bytes(),
            correlation_id: None,
            timestamp: now,
            expires_at: now + 60_000, // 1 minute expiration by default
        }
    }

    /// Set the correlation ID
    pub(crate) fn with_correlation_id(mut self, correlation_id: [u8; 16]) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }
    
    /// Set the expiration time in seconds from now
    pub(crate) fn with_expiration_secs(mut self, secs: u64) -> Self {
        self.expires_at = self.timestamp + (secs * 1000);
        self
    }
    
    /// Check if the message has expired
    pub(crate) fn is_expired(&self) -> bool {
        let now = Utc::now().timestamp_millis() as u64;
        self.expires_at > 0 && now > self.expires_at
    }
}

/// Trait for all message types that can be serialized/deserialized
pub trait Message: Serialize + for<'de> Deserialize<'de> + std::fmt::Debug + Send + Sync + 'static {
    /// Get the message type
    fn message_type() -> MessageType;

    /// Serialize the message to a byte vector
    fn to_msgpack(&self) -> Result<Vec<u8>, MessageError> {
        self.default_to_msgpack()
    }
    
    /// Default implementation of to_msgpack
    fn default_to_msgpack(&self) -> Result<Vec<u8>, MessageError> {
        // Serialize the header
        let header = MessageHeader::new(Self::message_type());
        let header_bytes = rmp_serde::to_vec_named(&header)?;

        // Serialize the message body
        let body_bytes = rmp_serde::to_vec_named(self)?;

        // Combine header and body
        let mut result = Vec::with_capacity(header_bytes.len() + body_bytes.len() + 4);
        
        // Write header length (4 bytes, big endian)
        let header_len = header_bytes.len() as u32;
        result.extend(&header_len.to_be_bytes());
        
        // Write header and body
        result.extend(header_bytes);
        result.extend(body_bytes);
        
        Ok(result)
    }
    
    /// Deserialize a message from a byte slice
    fn from_msgpack(data: &[u8]) -> Result<Self, MessageError>
    where
        Self: Sized
    {
        Self::default_from_msgpack(data)
    }
    
    /// Default implementation of from_msgpack
    fn default_from_msgpack(data: &[u8]) -> Result<Self, MessageError>
    where
        Self: Sized + serde::de::DeserializeOwned,
    {
        if data.len() < 4 {
            return Err(MessageError::InvalidFormat("Message too short".to_string()));
        }
        
        // First 4 bytes are the header length
        let header_len = u32::from_be_bytes([data[0], data[1], data[2], data[3]]) as usize;
        
        if data.len() < 4 + header_len {
            return Err(MessageError::InvalidFormat("Invalid header length".to_string()));
        }
        
        // Deserialize header
        let header: MessageHeader = rmp_serde::from_slice(&data[4..4 + header_len])?;
        
        // Validate header
        let expected_type = Self::message_type();
        if header.msg_type != expected_type {
            return Err(MessageError::MismatchedType {
                expected: expected_type,
                actual: header.msg_type,
            });
        }
        
        if header.is_expired() {
            return Err(MessageError::Expired);
        }

        // Deserialize body
        let msg = rmp_serde::from_slice(&data[4 + header_len..])?;
        Ok(msg)
    }

    /// Validate the message header
    fn validate(header: &MessageHeader) -> Result<(), MessageError> {
        let expected_type = Self::message_type();
        if header.msg_type != expected_type {
            return Err(MessageError::MismatchedType {
                expected: expected_type,
                actual: header.msg_type,
            });
        }
        
        if header.is_expired() {
            return Err(MessageError::Expired);
        }
        
        Ok(())
    }
}


/// Error type for message operations
#[derive(Error, Debug)]
pub enum MessageError {
    #[error("Invalid message type: {0}")]
    InvalidMessageType(u8),
    
    #[error("MessagePack encode error: {0}")]
    Encode(#[from] encode::Error),
    
    #[error("MessagePack decode error: {0}")]
    Decode(#[from] decode::Error),
    
    #[error("Invalid message format: {0}")]
    InvalidFormat(String),
    
    #[error("Message expired")]
    Expired,
    
    #[error("Invalid message type: expected {expected:?}, got {actual:?}")]
    MismatchedType {
        expected: MessageType,
        actual: MessageType,
    },
    
    #[error("Invalid message: {0}")]
    Validation(String),
}

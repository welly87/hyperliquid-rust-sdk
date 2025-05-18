use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::messages::types::MessageType;

/// Standard message header for all exchange messages
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageHeader {
    /// Message type identifier
    pub msg_type: MessageType,
    
    /// Unique message ID (UUID v4 as bytes)
    pub msg_id: [u8; 16],
    
    /// Optional correlation ID for request/response tracking (UUID v4 as bytes)
    pub correlation_id: Option<[u8; 16]>,
    
    /// Timestamp in milliseconds since epoch
    pub timestamp: u64,
    
    /// Expiration timestamp in milliseconds since epoch (0 for no expiration)
    pub expires_at: u64,
}

impl MessageHeader {
    /// Create a new message header with the given message type
    pub fn new(msg_type: MessageType) -> Self {
        Self {
            msg_type,
            msg_id: Uuid::new_v4().into_bytes(),
            correlation_id: None,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            expires_at: 0, // No expiration by default
        }
    }
    
    /// Set a correlation ID
    pub fn with_correlation_id(mut self, correlation_id: [u8; 16]) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }
    
    /// Set an expiration time in seconds from now
    pub fn with_expiration_secs(mut self, seconds: u64) -> Self {
        self.expires_at = self.timestamp + (seconds * 1000);
        self
    }
    
    /// Check if the message has expired
    pub fn is_expired(&self) -> bool {
        if self.expires_at == 0 {
            return false; // No expiration
        }
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        now > self.expires_at
    }
}

impl Default for MessageHeader {
    fn default() -> Self {
        Self {
            msg_type: MessageType::Order,
            msg_id: [0; 16],
            correlation_id: None,
            timestamp: 0,
            expires_at: 0,
        }
    }
}

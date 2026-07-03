// messaging/types.rs
use iroh::NodeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessagePayload {
    Text {
        content: String,
    },
    File {
        hash: [u8; 32],
        name: String,
        size: u64,
    },
    Reaction {
        emoji: String,
        target_msg: [u8; 32],
    },
    Presence {
        status: PresenceStatus,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: [u8; 32], // Blake3 hash of (author + timestamp + content)
    pub author: NodeId,
    pub timestamp: u64,         // Unix millis
    pub conversation: [u8; 32], // Topic/room hash
    pub payload: MessagePayload,
    pub signature: [u8; 64], // Ed25519 signature
}

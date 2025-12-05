//! Message Bus for inter-loop communication

use crate::LoopId;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::broadcast;
use tracing::{debug, trace};

/// Maximum messages in channel before oldest are dropped
const CHANNEL_CAPACITY: usize = 1000;

/// Type of message being sent
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageType {
    /// Data message containing processing results
    Data,
    /// Control message for loop coordination
    Control,
    /// Feedback message for learning
    Feedback,
    /// Status update
    Status,
    /// Error notification
    Error,
}

/// A message sent between loops
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Unique message ID
    pub id: String,

    /// Source loop
    pub source: LoopId,

    /// Target loops (empty for broadcast)
    pub targets: Vec<LoopId>,

    /// Message type
    pub msg_type: MessageType,

    /// Message payload
    pub payload: serde_json::Value,

    /// Timestamp
    pub timestamp: SystemTime,
}

impl Message {
    /// Create a new data message
    pub fn data(source: LoopId, payload: serde_json::Value) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            source,
            targets: vec![],
            msg_type: MessageType::Data,
            payload,
            timestamp: SystemTime::now(),
        }
    }

    /// Create a new control message
    pub fn control(source: LoopId, targets: Vec<LoopId>, payload: serde_json::Value) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            source,
            targets,
            msg_type: MessageType::Control,
            payload,
            timestamp: SystemTime::now(),
        }
    }

    /// Create a feedback message for learning
    pub fn feedback(source: LoopId, target: LoopId, payload: serde_json::Value) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            source,
            targets: vec![target],
            msg_type: MessageType::Feedback,
            payload,
            timestamp: SystemTime::now(),
        }
    }

    /// Check if this message is for a specific loop
    pub fn is_for(&self, loop_id: LoopId) -> bool {
        self.targets.is_empty() || self.targets.contains(&loop_id)
    }
}

/// Message bus for cross-loop communication
pub struct MessageBus {
    /// Global broadcast channel
    broadcast_tx: broadcast::Sender<Message>,

    /// Per-loop subscription channels
    loop_channels: Arc<DashMap<LoopId, broadcast::Sender<Message>>>,

    /// Message history for debugging
    history: Arc<DashMap<String, Message>>,

    /// Maximum history size
    max_history: usize,
}

impl MessageBus {
    /// Create a new message bus
    pub fn new() -> Self {
        let (broadcast_tx, _) = broadcast::channel(CHANNEL_CAPACITY);

        Self {
            broadcast_tx,
            loop_channels: Arc::new(DashMap::new()),
            history: Arc::new(DashMap::new()),
            max_history: 10000,
        }
    }

    /// Publish a message to the bus
    pub async fn publish(&self, message: Message) {
        trace!("Publishing message {} from {:?}", message.id, message.source);

        // Store in history
        if self.history.len() < self.max_history {
            self.history.insert(message.id.clone(), message.clone());
        }

        // Broadcast globally
        let _ = self.broadcast_tx.send(message.clone());

        // Send to specific loops if targeted
        if !message.targets.is_empty() {
            for target in &message.targets {
                if let Some(channel) = self.loop_channels.get(target) {
                    let _ = channel.send(message.clone());
                }
            }
        }
    }

    /// Subscribe to all messages
    pub fn subscribe_all(&self) -> broadcast::Receiver<Message> {
        self.broadcast_tx.subscribe()
    }

    /// Subscribe to messages for a specific loop
    pub fn subscribe_loop(&self, loop_id: LoopId) -> broadcast::Receiver<Message> {
        self.loop_channels
            .entry(loop_id)
            .or_insert_with(|| {
                let (tx, _) = broadcast::channel(CHANNEL_CAPACITY);
                debug!("Created subscription channel for {:?}", loop_id);
                tx
            })
            .subscribe()
    }

    /// Send a message to specific loops
    pub async fn send_to(&self, targets: Vec<LoopId>, message: Message) {
        let mut targeted_msg = message;
        targeted_msg.targets = targets;
        self.publish(targeted_msg).await;
    }

    /// Get message from history
    pub fn get_message(&self, id: &str) -> Option<Message> {
        self.history.get(id).map(|entry| entry.value().clone())
    }

    /// Get recent messages from a specific loop
    pub fn get_recent_from(&self, source: LoopId, limit: usize) -> Vec<Message> {
        self.history
            .iter()
            .filter(|entry| entry.value().source == source)
            .take(limit)
            .map(|entry| entry.value().clone())
            .collect()
    }

    /// Clear message history
    pub fn clear_history(&self) {
        self.history.clear();
    }

    /// Get current history size
    pub fn history_size(&self) -> usize {
        self.history.len()
    }
}

impl Default for MessageBus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_message_creation() {
        let msg = Message::data(
            LoopId::Quantum,
            serde_json::json!({"test": "data"}),
        );

        assert_eq!(msg.source, LoopId::Quantum);
        assert_eq!(msg.msg_type, MessageType::Data);
        assert!(msg.targets.is_empty());
    }

    #[tokio::test]
    async fn test_message_bus_publish() {
        let bus = MessageBus::new();
        let mut rx = bus.subscribe_all();

        let msg = Message::data(
            LoopId::Neural,
            serde_json::json!({"value": 42}),
        );

        bus.publish(msg.clone()).await;

        let received = rx.recv().await.unwrap();
        assert_eq!(received.source, LoopId::Neural);
        assert_eq!(received.id, msg.id);
    }

    #[tokio::test]
    async fn test_targeted_messages() {
        let bus = MessageBus::new();
        let mut rx = bus.subscribe_loop(LoopId::Cognitive);

        let msg = Message::control(
            LoopId::Learning,
            vec![LoopId::Cognitive],
            serde_json::json!({"command": "update"}),
        );

        bus.publish(msg.clone()).await;

        let received = rx.recv().await.unwrap();
        assert!(received.is_for(LoopId::Cognitive));
    }
}

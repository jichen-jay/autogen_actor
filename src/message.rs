use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ToolCall {
    pub name: String,
    pub arguments: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum Content {
    Text(String),
    ToolCall(ToolCall),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct CompletionUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct LlamaResponseMessage {
    pub content: Content,
    pub role: Role,
    pub usage: CompletionUsage,
}

// Custom message type for cluster communication
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ClusterMessage {
    LlamaResponse(LlamaResponseMessage),
    Command(String),
    Acknowledgement(String),
}

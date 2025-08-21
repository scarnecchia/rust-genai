use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResponse {
	pub call_id: String,
	// For now, just a string (would probably be serialized JSON)
	pub content: String,
	/// Whether this tool response represents an error
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_error: Option<bool>,
}

/// Constructor
impl ToolResponse {
	pub fn new(tool_call_id: impl Into<String>, content: impl Into<String>) -> Self {
		Self {
			call_id: tool_call_id.into(),
			content: content.into(),
			is_error: None,
		}
	}
}

/// Getters
#[allow(unused)]
impl ToolResponse {
	fn tool_call_id(&self) -> &str {
		&self.call_id
	}

	fn content(&self) -> &str {
		&self.content
	}
}

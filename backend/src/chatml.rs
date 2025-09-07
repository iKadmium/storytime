use serde::{Deserialize, Serialize};

/// Represents a single ChatML message with role and content
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMLMessage {
    pub role: String,
    pub content: String,
}

impl ChatMLMessage {
    /// Create a new ChatML message
    pub fn new(role: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            role: role.into(),
            content: content.into(),
        }
    }

    /// Create a system message
    pub fn system(content: impl Into<String>) -> Self {
        Self::new("system", content)
    }

    /// Create a user message
    pub fn user(content: impl Into<String>) -> Self {
        Self::new("user", content)
    }

    /// Create an assistant message
    pub fn assistant(content: impl Into<String>) -> Self {
        Self::new("assistant", content)
    }

    /// Convert this message to ChatML format string
    pub fn to_chatml(&self) -> String {
        format!("<|im_start|>{}\n{}<|im_end|>", self.role, self.content)
    }
}

/// Represents a collection of ChatML messages that can be converted to/from prompt strings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMLPrompt {
    pub messages: Vec<ChatMLMessage>,
}

impl ChatMLPrompt {
    /// Create a new empty ChatML prompt
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    /// Create a new prompt with initial messages
    pub fn with_messages(messages: Vec<ChatMLMessage>) -> Self {
        Self { messages }
    }

    /// Add a message to the prompt
    pub fn add_message(&mut self, message: ChatMLMessage) -> &mut Self {
        self.messages.push(message);
        self
    }

    /// Add a system message
    pub fn add_system(&mut self, content: impl Into<String>) -> &mut Self {
        self.add_message(ChatMLMessage::system(content))
    }

    /// Add a user message
    pub fn add_user(&mut self, content: impl Into<String>) -> &mut Self {
        self.add_message(ChatMLMessage::user(content))
    }

    /// Add an assistant message
    pub fn add_assistant(&mut self, content: impl Into<String>) -> &mut Self {
        self.add_message(ChatMLMessage::assistant(content))
    }

    /// Convert the entire prompt to ChatML format string
    pub fn to_chatml(&self) -> String {
        self.messages
            .iter()
            .map(|msg| msg.to_chatml())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Convert to ChatML format with an incomplete assistant message (for generation)
    pub fn to_chatml_for_generation(&self) -> String {
        let mut result = self.to_chatml();
        if !result.is_empty() {
            result.push('\n');
        }
        result.push_str("<|im_start|>assistant\n");
        result
    }

    /// Parse a ChatML format string into a ChatMLPrompt
    pub fn from_chatml(input: &str) -> Result<Self, ChatMLParseError> {
        let mut messages = Vec::new();
        let mut current_pos = 0;

        while current_pos < input.len() {
            // Skip whitespace and newlines
            while current_pos < input.len()
                && input.chars().nth(current_pos).unwrap().is_whitespace()
            {
                current_pos += 1;
            }

            if current_pos >= input.len() {
                break;
            }

            // Look for start marker
            if let Some(start_pos) = input[current_pos..].find("<|im_start|>") {
                let absolute_start = current_pos + start_pos + "<|im_start|>".len();

                // Find the role (everything up to first newline)
                if let Some(newline_pos) = input[absolute_start..].find('\n') {
                    let role_end = absolute_start + newline_pos;
                    let role = input[absolute_start..role_end].trim().to_string();

                    // Find the end marker
                    let content_start = role_end + 1;
                    if let Some(end_pos) = input[content_start..].find("<|im_end|>") {
                        let content_end = content_start + end_pos;
                        let content = input[content_start..content_end].to_string();

                        messages.push(ChatMLMessage::new(role, content));
                        current_pos = content_end + "<|im_end|>".len();
                    } else {
                        // No end marker found, treat rest as content
                        let content = input[content_start..].to_string();
                        messages.push(ChatMLMessage::new(role, content));
                        break;
                    }
                } else {
                    return Err(ChatMLParseError::InvalidFormat(
                        "No newline found after role in ChatML message".to_string(),
                    ));
                }
            } else {
                // No more start markers found
                break;
            }
        }

        Ok(Self::with_messages(messages))
    }

    /// Get the last assistant message content, if any
    pub fn last_assistant_message(&self) -> Option<&str> {
        self.messages
            .iter()
            .rev()
            .find(|msg| msg.role == "assistant")
            .map(|msg| msg.content.as_str())
    }

    /// Get all messages with a specific role
    pub fn messages_with_role(&self, role: &str) -> Vec<&ChatMLMessage> {
        self.messages
            .iter()
            .filter(|msg| msg.role == role)
            .collect()
    }
}

impl Default for ChatMLPrompt {
    fn default() -> Self {
        Self::new()
    }
}

/// Errors that can occur when parsing ChatML
#[derive(Debug, thiserror::Error)]
pub enum ChatMLParseError {
    #[error("Invalid ChatML format: {0}")]
    InvalidFormat(String),

    #[error("Missing required markers")]
    MissingMarkers,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chatml_message_creation() {
        let system_msg = ChatMLMessage::system("You are a helpful assistant.");
        assert_eq!(system_msg.role, "system");
        assert_eq!(system_msg.content, "You are a helpful assistant.");

        let user_msg = ChatMLMessage::user("Hello!");
        assert_eq!(user_msg.role, "user");
        assert_eq!(user_msg.content, "Hello!");

        let assistant_msg = ChatMLMessage::assistant("Hi there!");
        assert_eq!(assistant_msg.role, "assistant");
        assert_eq!(assistant_msg.content, "Hi there!");
    }

    #[test]
    fn test_chatml_message_to_string() {
        let msg = ChatMLMessage::system("You are helpful.");
        let expected = "<|im_start|>system\nYou are helpful.<|im_end|>";
        assert_eq!(msg.to_chatml(), expected);
    }

    #[test]
    fn test_chatml_prompt_building() {
        let mut prompt = ChatMLPrompt::new();
        prompt
            .add_system("You are a helpful assistant.")
            .add_user("What is the capital of France?")
            .add_assistant("The capital of France is Paris.");

        assert_eq!(prompt.messages.len(), 3);
        assert_eq!(prompt.messages[0].role, "system");
        assert_eq!(prompt.messages[1].role, "user");
        assert_eq!(prompt.messages[2].role, "assistant");
    }

    #[test]
    fn test_chatml_prompt_to_string() {
        let mut prompt = ChatMLPrompt::new();
        prompt.add_system("You are helpful.").add_user("Hello!");

        let expected =
            "<|im_start|>system\nYou are helpful.<|im_end|>\n<|im_start|>user\nHello!<|im_end|>";
        assert_eq!(prompt.to_chatml(), expected);
    }

    #[test]
    fn test_chatml_prompt_for_generation() {
        let mut prompt = ChatMLPrompt::new();
        prompt.add_system("You are helpful.").add_user("Hello!");

        let expected = "<|im_start|>system\nYou are helpful.<|im_end|>\n<|im_start|>user\nHello!<|im_end|>\n<|im_start|>assistant\n";
        assert_eq!(prompt.to_chatml_for_generation(), expected);
    }

    #[test]
    fn test_chatml_parsing() {
        let input =
            "<|im_start|>system\nYou are helpful.<|im_end|>\n<|im_start|>user\nHello!<|im_end|>";
        let prompt = ChatMLPrompt::from_chatml(input).unwrap();

        assert_eq!(prompt.messages.len(), 2);
        assert_eq!(prompt.messages[0].role, "system");
        assert_eq!(prompt.messages[0].content, "You are helpful.");
        assert_eq!(prompt.messages[1].role, "user");
        assert_eq!(prompt.messages[1].content, "Hello!");
    }

    #[test]
    fn test_chatml_parsing_incomplete_assistant() {
        let input = "<|im_start|>system\nYou are helpful.<|im_end|>\n<|im_start|>user\nHello!<|im_end|>\n<|im_start|>assistant\nHi there!";
        let prompt = ChatMLPrompt::from_chatml(input).unwrap();

        assert_eq!(prompt.messages.len(), 3);
        assert_eq!(prompt.messages[2].role, "assistant");
        assert_eq!(prompt.messages[2].content, "Hi there!");
    }

    #[test]
    fn test_last_assistant_message() {
        let mut prompt = ChatMLPrompt::new();
        prompt
            .add_system("You are helpful.")
            .add_user("Hello!")
            .add_assistant("Hi there!")
            .add_user("How are you?")
            .add_assistant("I'm doing well!");

        assert_eq!(prompt.last_assistant_message(), Some("I'm doing well!"));
    }

    #[test]
    fn test_messages_with_role() {
        let mut prompt = ChatMLPrompt::new();
        prompt
            .add_system("You are helpful.")
            .add_user("Hello!")
            .add_user("How are you?")
            .add_assistant("I'm doing well!");

        let user_messages = prompt.messages_with_role("user");
        assert_eq!(user_messages.len(), 2);
        assert_eq!(user_messages[0].content, "Hello!");
        assert_eq!(user_messages[1].content, "How are you?");
    }

    #[test]
    fn test_round_trip_parsing() {
        let mut original = ChatMLPrompt::new();
        original
            .add_system("You are a helpful assistant.")
            .add_user("What is 2+2?")
            .add_assistant("2+2 equals 4.");

        let chatml_string = original.to_chatml();
        let parsed = ChatMLPrompt::from_chatml(&chatml_string).unwrap();

        assert_eq!(original, parsed);
    }
}

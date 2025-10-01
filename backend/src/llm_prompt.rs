use crate::chatml::ChatMLPrompt;
use crate::models::{Character, Chat, Job, Prompt};

/// Builds a ChatML prompt for a single setup item
/// Used for iterative LLM calls where each setup item results in a separate call
/// Returns a ChatMLPrompt structure that can be converted to string or used for parsing responses
pub fn build_setup_item_chatml_prompt(
    character: &Character,
    prompt: &Prompt,
    setup_item: &str,
    previous_responses: &[ChatMLPrompt],
    chat_history: &Chat,
) -> ChatMLPrompt {
    let mut chatml_prompt = ChatMLPrompt::new();

    // System message with character information and context
    let system_message = format!(
        "You are {}, described as: {}\n\
         Personality: {}\n\
         Background: {}\n\n\
         Context: {}",
        character.name,
        character.description,
        character.personality,
        character.background,
        prompt.context
    );

    chatml_prompt.add_system(system_message);

    // Add previous chat context if available
    for message in &chat_history.messages {
        for text in &message.text {
            chatml_prompt.add_user(text);
            chatml_prompt.add_assistant(format!("*{} responds*", character.name));
        }
    }

    // Add previous setup responses to build conversation context
    for (i, prev_response) in previous_responses.iter().enumerate() {
        if i < prompt.setup.len() - 1 {
            chatml_prompt.add_user(&prompt.setup[i]);
            // Get the last assistant response from the previous ChatML prompt
            if let Some(last_assistant_msg) = prev_response.last_assistant_message() {
                chatml_prompt.add_assistant(last_assistant_msg);
            }
        }
    }

    // Add the current setup item as user input
    chatml_prompt.add_user(setup_item);

    chatml_prompt
}

/// Builds a prompt for a single setup item using ChatML format (string version)
/// Used for iterative LLM calls where each setup item results in a separate call
pub fn build_setup_item_prompt(
    character: &Character,
    prompt: &Prompt,
    setup_item: &str,
    previous_responses: &[String],
    chat_history: Option<&Chat>,
) -> String {
    let mut chatml_prompt = ChatMLPrompt::new();

    // System message with character information and context
    let system_message = format!(
        "You are {}, described as: {}\n\
         Personality: {}\n\
         Background: {}\n\n\
         Context: {}",
        character.name,
        character.description,
        character.personality,
        character.background,
        prompt.context
    );

    chatml_prompt.add_system(system_message);

    // Add previous chat context if available
    if let Some(chat) = chat_history {
        for message in &chat.messages {
            for text in &message.text {
                chatml_prompt.add_user(text);
                chatml_prompt.add_assistant(format!("*{} responds*", character.name));
            }
        }
    }

    // Add previous setup responses to build conversation context
    for (i, response) in previous_responses.iter().enumerate() {
        if i < prompt.setup.len() - 1 {
            chatml_prompt.add_user(&prompt.setup[i]);
            chatml_prompt.add_assistant(response);
        }
    }

    // Add the current setup item as user input
    chatml_prompt.add_user(setup_item);

    // Convert to ChatML format for generation
    chatml_prompt.to_chatml_for_generation()
}

/// Builds a complete prompt for a job using ChatML format (legacy method)
/// Combines character information, prompt setup, and previous chat context
pub fn build_job_prompt(
    character: &Character,
    prompt: &Prompt,
    chat_history: Option<&Chat>,
) -> String {
    let mut chatml_prompt = ChatMLPrompt::new();

    // System message with character information and context
    let system_message = format!(
        "You are {}, described as: {}\n\
         Personality: {}\n\
         Background: {}\n\n\
         Context: {}",
        character.name,
        character.description,
        character.personality,
        character.background,
        prompt.context
    );

    chatml_prompt.add_system(system_message);

    // Add previous chat context if available
    if let Some(chat) = chat_history {
        for message in &chat.messages {
            // Add user messages (assuming they're from the "user" role)
            for text in &message.text {
                chatml_prompt.add_user(text);
                // For now, we'll add a simple assistant acknowledgment
                // In a real system, you'd want to track the actual conversation flow
                chatml_prompt.add_assistant(format!("*{} responds*", character.name));
            }
        }
    }

    // Add prompt setup items as a conversation flow
    for (i, setup_item) in prompt.setup.iter().enumerate() {
        if i == 0 {
            // First setup item goes as user input
            chatml_prompt.add_user(setup_item);
        } else if i == prompt.setup.len() - 1 {
            // Last setup item also goes as user input (this is what we want the assistant to respond to)
            chatml_prompt.add_user(setup_item);
        } else {
            // Middle setup items create a conversation flow
            chatml_prompt.add_user(setup_item);
            chatml_prompt.add_assistant(format!("*{} responds appropriately*", character.name));
        }
    }

    // Convert to ChatML format for generation (includes incomplete assistant start)
    chatml_prompt.to_chatml_for_generation()
}

/// Builds a simple prompt for direct LLM interaction (fallback method)
pub fn build_simple_prompt(_job: &Job, character: &Character, prompt: &Prompt) -> String {
    let mut prompt_text = String::new();

    // Character introduction
    prompt_text.push_str(&format!(
        "You are {}. {}\n\
         Personality: {}\n\
         Background: {}\n\n",
        character.name, character.description, character.personality, character.background
    ));

    // Context
    prompt_text.push_str(&format!("Context: {}\n\n", prompt.context));

    // Setup items as conversation
    if !prompt.setup.is_empty() {
        prompt_text.push_str("Conversation:\n");
        for setup_item in &prompt.setup {
            prompt_text.push_str(&format!("Human: {setup_item}\n"));
        }
        prompt_text.push_str(&format!("{}: ", character.name));
    }

    prompt_text
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;
    use crate::models::Voice;

    fn create_test_character() -> Character {
        Character {
            name: "Test Knight".to_string(),
            description: "A brave test character".to_string(),
            personality: "Noble and courageous".to_string(),
            background: "A knight of the test realm".to_string(),
            voice: Some(Voice {
                temperature: 0.7,
                exaggeration: 0.5,
                cfg_weight: 1.0,
                speed_factor: 1.0,
                voice_name: "test.wav".to_string(),
            }),
        }
    }

    fn create_test_prompt() -> Prompt {
        Prompt {
            title: "Test Prompt".to_string(),
            description: "A test prompt".to_string(),
            context: "This is a test conversation".to_string(),
            setup: vec![
                "Hello there!".to_string(),
                "How are you today?".to_string(),
                "Tell me a joke".to_string(),
            ],
            create_audio: false,
            create_images: false,
        }
    }

    fn create_test_job() -> Job {
        Job {
            id: Some(Uuid::parse_str("12345678-9012-3456-7890-123456789012").unwrap()),
            characters: vec!["Test Knight".to_string()],
            prompts: vec!["Test Prompt".to_string()],
            cadence: "daily".to_string(),
            prompt_override: None,
        }
    }

    #[test]
    fn test_build_job_prompt() {
        let character = create_test_character();
        let prompt = create_test_prompt();

        let result = build_job_prompt(&character, &prompt, None);

        // Check that it contains ChatML format
        assert!(result.contains("<|im_start|>system"));
        assert!(result.contains("<|im_end|>"));
        assert!(result.contains(&character.name));
        assert!(result.contains(&character.description));
        assert!(result.contains(&prompt.context));
    }

    #[test]
    fn test_build_simple_prompt() {
        let job = create_test_job();
        let character = create_test_character();
        let prompt = create_test_prompt();

        let result = build_simple_prompt(&job, &character, &prompt);

        assert!(result.contains(&character.name));
        assert!(result.contains(&character.description));
        assert!(result.contains(&prompt.context));
        assert!(result.contains("Conversation:"));
    }

    #[test]
    fn test_build_setup_item_prompt() {
        let character = create_test_character();
        let prompt = create_test_prompt();
        let setup_item = "Tell me a joke";
        let previous_responses = vec![
            "Hello!".to_string(),
            "I'm doing well, thank you!".to_string(),
        ];

        let result =
            build_setup_item_prompt(&character, &prompt, setup_item, &previous_responses, None);

        // Check that it contains ChatML format
        assert!(result.contains("<|im_start|>system"));
        assert!(result.contains("<|im_end|>"));
        assert!(result.contains(&character.name));
        assert!(result.contains(&character.description));
        assert!(result.contains(&prompt.context));
        assert!(result.contains(setup_item));
        // Should contain previous conversation
        assert!(result.contains("Hello there!")); // First setup item
        assert!(result.contains("Hello!")); // First response
    }
}

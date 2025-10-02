use regex::Regex;

/// Generate a URL-safe slug from a string
/// Converts to lowercase, replaces spaces and special characters with hyphens,
/// removes consecutive hyphens, and trims hyphens from ends
pub fn to_slug(input: &str) -> String {
    let re = Regex::new(r"[^a-zA-Z0-9\s-]").unwrap();
    let cleaned = re.replace_all(input, "");

    let re_spaces = Regex::new(r"[\s_]+").unwrap();
    let with_hyphens = re_spaces.replace_all(&cleaned, "-");

    let re_multiple_hyphens = Regex::new(r"-+").unwrap();
    let single_hyphens = re_multiple_hyphens.replace_all(&with_hyphens, "-");

    single_hyphens.trim_matches('-').to_lowercase()
}

/// Generate a slug for a character based on their name
pub fn character_slug(name: &str) -> String {
    to_slug(name)
}

/// Generate a slug for a prompt based on its title
pub fn prompt_slug(title: &str) -> String {
    to_slug(title)
}

/// Generate a new UUID for a job
pub fn generate_job_id() -> uuid::Uuid {
    uuid::Uuid::new_v4()
}

/// Generate a slug for a job based on character and prompt (legacy function for backward compatibility)
/// Format: {character_slug}-{prompt_slug}
pub fn job_slug(character: &str, prompt: &str) -> String {
    let character_slug = to_slug(character);
    let prompt_slug = to_slug(prompt);
    format!("{character_slug}-{prompt_slug}")
}

/// Generate a file path for a character
pub fn character_file_path(name: &str) -> String {
    format!("./data/characters/{}.json", character_slug(name))
}

/// Generate a file path for a prompt
pub fn prompt_file_path(title: &str) -> String {
    format!("./data/prompts/{}.json", prompt_slug(title))
}

/// Generate a file path for a job using UUID
pub fn job_file_path_from_id(id: &uuid::Uuid) -> String {
    format!("./data/jobs/{}.json", id)
}

/// Generate a file path for a job (legacy function for backward compatibility)
pub fn job_file_path(character: &str, prompt: &str) -> String {
    format!("./data/jobs/{}.json", job_slug(character, prompt))
}

/// Generate a file path for a job from its slug (legacy function for backward compatibility)
pub fn job_file_path_from_slug(slug: &str) -> String {
    format!("./data/jobs/{slug}.json")
}

/// Generate a file path for a chat based on character name
pub fn chat_file_path_from_character(character: &str) -> String {
    format!("./data/chats/{}.json", character_slug(character))
}

/// Generate a file path for an audio file
pub fn audio_file_path(character: &str, filename: &str) -> String {
    format!("./data/audio/{}/{}", character_slug(character), filename)
}

/// Generate a file path for an image file
pub fn image_file_path(character: &str, filename: &str) -> String {
    format!("./data/images/{}/{}", character_slug(character), filename)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_slug() {
        assert_eq!(to_slug("Brave Knight"), "brave-knight");
        assert_eq!(to_slug("Send a Joke Text"), "send-a-joke-text");
        assert_eq!(to_slug("The Jokester!!!"), "the-jokester");
        assert_eq!(to_slug("Multiple   Spaces"), "multiple-spaces");
        assert_eq!(to_slug("Special@#$Characters"), "specialcharacters");
        assert_eq!(to_slug("--Start-and-End--"), "start-and-end");
        assert_eq!(to_slug(""), "");
    }

    #[test]
    fn test_job_slug() {
        assert_eq!(
            job_slug("Brave Knight", "Send a Joke Text"),
            "brave-knight-send-a-joke-text"
        );
        assert_eq!(
            job_slug("The Jokester", "Daily Update"),
            "the-jokester-daily-update"
        );
    }

    #[test]
    fn test_file_paths() {
        assert_eq!(
            character_file_path("Brave Knight"),
            "./data/characters/brave-knight.json"
        );
        assert_eq!(
            prompt_file_path("Send a Joke Text"),
            "./data/prompts/send-a-joke-text.json"
        );
        assert_eq!(
            job_file_path("Brave Knight", "Send a Joke Text"),
            "./data/jobs/brave-knight-send-a-joke-text.json"
        );
    }
}

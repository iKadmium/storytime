use crate::models::Settings;
use std::fs;
use std::path::Path;

/// Ensures that the data directory structure and settings.json file exist.
/// Creates them with default values if they don't exist.
fn ensure_data_directory_and_settings() -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = Path::new("./data");
    let characters_dir = data_dir.join("characters");
    let prompts_dir = data_dir.join("prompts");
    let jobs_dir = data_dir.join("jobs");
    let chats_dir = data_dir.join("chats");
    let audio_dir = data_dir.join("audio");
    let settings_path = data_dir.join("settings.json");

    // Create data directory structure if it doesn't exist
    if !data_dir.exists() {
        tracing::info!("Creating data directory at ./data");
        fs::create_dir_all(data_dir)?;
    }

    // Create all required subdirectories
    for (dir_path, dir_name) in [
        (&characters_dir, "characters"),
        (&prompts_dir, "prompts"),
        (&jobs_dir, "jobs"),
        (&chats_dir, "chats"),
        (&audio_dir, "audio"),
    ] {
        if !dir_path.exists() {
            tracing::info!("Creating {} directory at ./data/{}", dir_name, dir_name);
            fs::create_dir_all(dir_path)?;
        }
    }

    // Create settings.json if it doesn't exist
    if !settings_path.exists() {
        tracing::info!("Creating default settings.json file");
        let default_settings = r#"{
    "ttsApi": "https://www.example.com/tts",
    "llmApi": "https://www.example.com/api/v1/generate"
}"#;
        fs::write(&settings_path, default_settings)?;
        tracing::info!("Settings file created at ./data/settings.json");
        tracing::warn!(
            "Please update the settings in ./data/settings.json with your actual API endpoints"
        );
    }

    Ok(())
}

pub fn load_settings() -> Result<Settings, Box<dyn std::error::Error>> {
    // Ensure data directory and settings file exist
    ensure_data_directory_and_settings()?;

    let settings_path = Path::new("./data/settings.json");

    let contents = fs::read_to_string(settings_path)?;
    let settings: Settings = serde_json::from_str(&contents)?;

    Ok(settings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::write;
    use tempfile::tempdir;

    #[test]
    fn test_load_settings() {
        let dir = tempdir().unwrap();
        let data_dir = dir.path().join("data");
        fs::create_dir(&data_dir).unwrap();
        let settings_file = data_dir.join("settings.json");

        let test_settings = r#"
        {
            "ttsApi": "https://test.tts.com",
            "llmApi": "https://test.llm.com"
        }
        "#;

        write(&settings_file, test_settings).unwrap();

        // Change to temp directory for test
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(&dir).unwrap();

        let result = load_settings();

        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        let settings = result.unwrap();
        assert_eq!(settings.tts_api, "https://test.tts.com");
        assert_eq!(settings.llm_api, "https://test.llm.com");
    }
}

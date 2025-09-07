use crate::models::Settings;
use std::fs;
use std::path::Path;

pub fn load_settings() -> Result<Settings, Box<dyn std::error::Error>> {
    let settings_path = Path::new("./data/settings.json");

    if !settings_path.exists() {
        return Err("Settings file not found at ./data/settings.json".into());
    }

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

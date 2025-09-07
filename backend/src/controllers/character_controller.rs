use axum::{Json as JsonExtract, extract::Path, http::StatusCode, response::Json};
use tokio::fs;

use crate::models::{ApiResponse, Character, CreateCharacterRequest, UpdateCharacterRequest};
use crate::utils::{character_file_path, character_slug};

/// Get all characters
pub async fn get_characters()
-> Result<Json<ApiResponse<Vec<Character>>>, (StatusCode, Json<ApiResponse<()>>)> {
    match load_all_characters().await {
        Ok(characters) => Ok(Json(ApiResponse {
            success: true,
            data: Some(characters),
            message: "Characters retrieved successfully".to_string(),
        })),
        Err(e) => {
            tracing::error!("Failed to load characters: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: format!("Failed to load characters: {e}"),
                }),
            ))
        }
    }
}

/// Get a specific character by slug
pub async fn get_character(
    Path(slug): Path<String>,
) -> Result<Json<ApiResponse<Character>>, (StatusCode, Json<ApiResponse<()>>)> {
    match load_character_by_slug(&slug).await {
        Ok(character) => Ok(Json(ApiResponse {
            success: true,
            data: Some(character),
            message: "Character retrieved successfully".to_string(),
        })),
        Err(e) => {
            if e.to_string().contains("No such file or directory")
                || e.to_string().contains("Character not found")
            {
                Err((
                    StatusCode::NOT_FOUND,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("Character with slug '{slug}' not found"),
                    }),
                ))
            } else {
                tracing::error!("Failed to load character with slug '{slug}': {e}");
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("Failed to load character: {e}"),
                    }),
                ))
            }
        }
    }
}

/// Create a new character
pub async fn create_character(
    JsonExtract(request): JsonExtract<CreateCharacterRequest>,
) -> Result<Json<ApiResponse<Character>>, (StatusCode, Json<ApiResponse<()>>)> {
    // Check if character already exists
    if load_character(&request.name).await.is_ok() {
        return Err((
            StatusCode::CONFLICT,
            Json(ApiResponse {
                success: false,
                data: None,
                message: format!("Character '{}' already exists", request.name),
            }),
        ));
    }

    let character = Character {
        name: request.name.clone(),
        description: request.description,
        personality: request.personality,
        background: request.background,
        voice: request.voice,
    };

    match save_character(&character).await {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            data: Some(character),
            message: "Character created successfully".to_string(),
        })),
        Err(e) => {
            tracing::error!("Failed to create character '{}': {}", request.name, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: format!("Failed to create character: {e}"),
                }),
            ))
        }
    }
}

/// Update an existing character
pub async fn update_character(
    Path(slug): Path<String>,
    JsonExtract(request): JsonExtract<UpdateCharacterRequest>,
) -> Result<Json<ApiResponse<Character>>, (StatusCode, Json<ApiResponse<()>>)> {
    let mut character = match load_character_by_slug(&slug).await {
        Ok(character) => character,
        Err(e) => {
            if e.to_string().contains("No such file or directory")
                || e.to_string().contains("Character not found")
            {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("Character with slug '{slug}' not found"),
                    }),
                ));
            } else {
                tracing::error!("Failed to load character with slug '{slug}': {e}");
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("Failed to load character: {e}"),
                    }),
                ));
            }
        }
    };

    // Update only provided fields
    if let Some(description) = request.description {
        character.description = description;
    }
    if let Some(personality) = request.personality {
        character.personality = personality;
    }
    if let Some(background) = request.background {
        character.background = background;
    }
    if let Some(voice) = request.voice {
        character.voice = Some(voice);
    }

    match save_character(&character).await {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            data: Some(character),
            message: "Character updated successfully".to_string(),
        })),
        Err(e) => {
            tracing::error!("Failed to update character with slug '{slug}': {e}");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: format!("Failed to update character: {e}"),
                }),
            ))
        }
    }
}

/// Delete a character by slug
pub async fn delete_character(
    Path(slug): Path<String>,
) -> Result<Json<ApiResponse<()>>, (StatusCode, Json<ApiResponse<()>>)> {
    // First, find the character to get its name for deletion
    let character = match load_character_by_slug(&slug).await {
        Ok(character) => character,
        Err(e) => {
            if e.to_string().contains("Character not found") {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("Character with slug '{slug}' not found"),
                    }),
                ));
            } else {
                tracing::error!("Failed to find character with slug '{slug}': {e}");
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("Failed to find character: {e}"),
                    }),
                ));
            }
        }
    };

    match delete_character_file(&character.name).await {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            data: None,
            message: format!("Character '{}' deleted successfully", character.name),
        })),
        Err(e) => {
            tracing::error!("Failed to delete character '{}': {e}", character.name);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: format!("Failed to delete character: {e}"),
                }),
            ))
        }
    }
}

// File I/O utility functions

/// Load all characters from the data/characters directory
async fn load_all_characters() -> Result<Vec<Character>, Box<dyn std::error::Error + Send + Sync>> {
    let mut characters = Vec::new();
    let dir_path = "./data/characters";

    let mut entries = fs::read_dir(dir_path).await?;
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = fs::read_to_string(&path).await?;
            let character: Character = serde_json::from_str(&content)?;
            characters.push(character);
        }
    }

    // Sort characters by name for consistent ordering
    characters.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(characters)
}

/// Load a specific character by name
async fn load_character(name: &str) -> Result<Character, Box<dyn std::error::Error + Send + Sync>> {
    let file_path = character_file_path(name);
    let content = fs::read_to_string(&file_path).await?;
    let character: Character = serde_json::from_str(&content)?;
    Ok(character)
}

/// Load a specific character by slug
/// This searches all characters to find one whose name converts to the given slug
async fn load_character_by_slug(
    slug: &str,
) -> Result<Character, Box<dyn std::error::Error + Send + Sync>> {
    let characters = load_all_characters().await?;

    for character in characters {
        if character_slug(&character.name) == slug {
            return Ok(character);
        }
    }

    Err(format!("Character not found with slug: {slug}").into())
}

/// Save a character to a JSON file
async fn save_character(
    character: &Character,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let file_path = character_file_path(&character.name);
    let json_content = serde_json::to_string_pretty(character)?;
    fs::write(&file_path, json_content).await?;
    Ok(())
}

/// Delete a character file
async fn delete_character_file(name: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let file_path = character_file_path(name);
    fs::remove_file(&file_path).await?;
    Ok(())
}

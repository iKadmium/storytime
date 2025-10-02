use axum::{Json as JsonExtract, extract::Path, http::StatusCode, response::Json};
use chrono::Utc;
use std::path::PathBuf;
use tokio::fs;

use crate::models::{
    AddMessageRequest, ApiResponse, Chat, CreateChatRequest, Message, UpdateChatRequest,
    UpdateMessageRequest,
};
use crate::utils::{audio_file_path, image_file_path};

/// Get all chats
pub async fn get_chats()
-> Result<Json<ApiResponse<Vec<String>>>, (StatusCode, Json<ApiResponse<()>>)> {
    match load_all_chat_names().await {
        Ok(chat_names) => Ok(Json(ApiResponse {
            success: true,
            data: Some(chat_names),
            message: "Chat names retrieved successfully".to_string(),
        })),
        Err(e) => {
            tracing::error!("Failed to load chat names: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: format!("Failed to load chat names: {e}"),
                }),
            ))
        }
    }
}

/// Get a specific chat by character name
pub async fn get_chat(
    Path(character): Path<String>,
) -> Result<Json<ApiResponse<Chat>>, (StatusCode, Json<ApiResponse<()>>)> {
    match load_chat(&character).await {
        Ok(chat) => Ok(Json(ApiResponse {
            success: true,
            data: Some(chat),
            message: "Chat retrieved successfully".to_string(),
        })),
        Err(e) => {
            if e.to_string().contains("No such file or directory") {
                Err((
                    StatusCode::NOT_FOUND,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("Chat for character '{character}' not found"),
                    }),
                ))
            } else {
                tracing::error!("Failed to load chat for character '{character}': {e}");
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("Failed to load chat: {e}"),
                    }),
                ))
            }
        }
    }
}

/// Create a new chat
pub async fn create_chat(
    JsonExtract(payload): JsonExtract<CreateChatRequest>,
) -> Result<Json<ApiResponse<Chat>>, (StatusCode, Json<ApiResponse<()>>)> {
    // Check if chat already exists
    if (load_chat(&payload.character).await).is_ok() {
        return Err((
            StatusCode::CONFLICT,
            Json(ApiResponse {
                success: false,
                data: None,
                message: format!("Chat for character '{}' already exists", payload.character),
            }),
        ));
    }

    let chat = Chat {
        character: payload.character.clone(),
        messages: vec![],
    };

    match save_chat(&payload.character, &chat).await {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            data: Some(chat),
            message: "Chat created successfully".to_string(),
        })),
        Err(e) => {
            tracing::error!(
                "Failed to create chat for character '{}': {}",
                payload.character,
                e
            );
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: format!("Failed to create chat: {e}"),
                }),
            ))
        }
    }
}

/// Update a chat (change character name)
pub async fn update_chat(
    Path(character): Path<String>,
    JsonExtract(payload): JsonExtract<UpdateChatRequest>,
) -> Result<Json<ApiResponse<Chat>>, (StatusCode, Json<ApiResponse<()>>)> {
    let mut chat = match load_chat(&character).await {
        Ok(chat) => chat,
        Err(e) => {
            if e.to_string().contains("No such file or directory") {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("Chat for character '{character}' not found"),
                    }),
                ));
            } else {
                tracing::error!("Failed to load chat for character '{character}': {e}");
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("Failed to load chat: {e}"),
                    }),
                ));
            }
        }
    };

    let old_character = character.clone();

    // Update character if provided
    if let Some(new_character) = payload.character {
        chat.character = new_character.clone();

        // If character name changed, delete old file and save with new name
        if old_character != new_character {
            // Check if new character chat already exists
            if (load_chat(&new_character).await).is_ok() {
                return Err((
                    StatusCode::CONFLICT,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("Chat for character '{new_character}' already exists"),
                    }),
                ));
            }

            // Save with new character name
            match save_chat(&new_character, &chat).await {
                Ok(_) => {
                    // Delete old file
                    if let Err(e) = delete_chat(&old_character).await {
                        tracing::error!(
                            "Failed to delete old chat file for '{}': {}",
                            old_character,
                            e
                        );
                        // Continue anyway since the new file was saved successfully
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to save updated chat: {}", e);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ApiResponse {
                            success: false,
                            data: None,
                            message: format!("Failed to update chat: {e}"),
                        }),
                    ));
                }
            }
        } else {
            // Same character name, just update in place
            match save_chat(&old_character, &chat).await {
                Ok(_) => {}
                Err(e) => {
                    tracing::error!("Failed to save updated chat: {}", e);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ApiResponse {
                            success: false,
                            data: None,
                            message: format!("Failed to update chat: {e}"),
                        }),
                    ));
                }
            }
        }
    }

    Ok(Json(ApiResponse {
        success: true,
        data: Some(chat),
        message: "Chat updated successfully".to_string(),
    }))
}

/// Delete a chat
pub async fn delete_chat_endpoint(
    Path(character): Path<String>,
) -> Result<Json<ApiResponse<()>>, (StatusCode, Json<ApiResponse<()>>)> {
    match delete_chat(&character).await {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            data: None,
            message: "Chat deleted successfully".to_string(),
        })),
        Err(e) => {
            if e.to_string().contains("No such file or directory") {
                Err((
                    StatusCode::NOT_FOUND,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("Chat for character '{character}' not found"),
                    }),
                ))
            } else {
                tracing::error!("Failed to delete chat for character '{character}': {e}");
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("Failed to delete chat: {e}"),
                    }),
                ))
            }
        }
    }
}

/// Add a message to a chat
pub async fn add_message(
    Path(character): Path<String>,
    JsonExtract(payload): JsonExtract<AddMessageRequest>,
) -> Result<Json<ApiResponse<Chat>>, (StatusCode, Json<ApiResponse<()>>)> {
    let mut chat = match load_chat(&character).await {
        Ok(chat) => chat,
        Err(e) => {
            if e.to_string().contains("No such file or directory") {
                // Chat doesn't exist, create a new one
                Chat {
                    character: character.clone(),
                    messages: vec![],
                }
            } else {
                tracing::error!("Failed to load chat for character '{character}': {e}");
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("Failed to load chat: {e}"),
                    }),
                ));
            }
        }
    };

    let message = Message {
        text: payload.text,
        audio: payload.audio,
        images: payload.images,
        read: payload.read,
        timestamp: payload.timestamp.unwrap_or_else(Utc::now),
    };

    chat.messages.push(message);

    match save_chat(&character, &chat).await {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            data: Some(chat),
            message: "Message added successfully".to_string(),
        })),
        Err(e) => {
            tracing::error!("Failed to save chat after adding message: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: format!("Failed to add message: {e}"),
                }),
            ))
        }
    }
}

/// Update a specific message in a chat
pub async fn update_message(
    Path((character, message_index)): Path<(String, usize)>,
    JsonExtract(payload): JsonExtract<UpdateMessageRequest>,
) -> Result<Json<ApiResponse<Chat>>, (StatusCode, Json<ApiResponse<()>>)> {
    let mut chat = match load_chat(&character).await {
        Ok(chat) => chat,
        Err(e) => {
            if e.to_string().contains("No such file or directory") {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("Chat for character '{character}' not found"),
                    }),
                ));
            } else {
                tracing::error!("Failed to load chat for character '{character}': {e}");
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("Failed to load chat: {e}"),
                    }),
                ));
            }
        }
    };

    if message_index >= chat.messages.len() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                data: None,
                message: format!("Message at index {message_index} not found"),
            }),
        ));
    }

    let message = &mut chat.messages[message_index];

    // Update message fields if provided
    if let Some(text) = payload.text {
        message.text = text;
    }
    if let Some(audio) = payload.audio {
        message.audio = audio;
    }
    if let Some(images) = payload.images {
        message.images = images;
    }
    if let Some(read) = payload.read {
        message.read = read;
    }
    if let Some(timestamp) = payload.timestamp {
        message.timestamp = timestamp;
    }

    match save_chat(&character, &chat).await {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            data: Some(chat),
            message: "Message updated successfully".to_string(),
        })),
        Err(e) => {
            tracing::error!("Failed to save chat after updating message: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: format!("Failed to update message: {e}"),
                }),
            ))
        }
    }
}

/// Delete a specific message from a chat
pub async fn delete_message(
    Path((character, message_index)): Path<(String, usize)>,
) -> Result<Json<ApiResponse<Chat>>, (StatusCode, Json<ApiResponse<()>>)> {
    let mut chat = match load_chat(&character).await {
        Ok(chat) => chat,
        Err(e) => {
            if e.to_string().contains("No such file or directory") {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("Chat for character '{character}' not found"),
                    }),
                ));
            } else {
                tracing::error!("Failed to load chat for character '{character}': {e}");
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("Failed to load chat: {e}"),
                    }),
                ));
            }
        }
    };

    if message_index >= chat.messages.len() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                data: None,
                message: format!("Message at index {message_index} not found"),
            }),
        ));
    }

    // Get the message before deletion to clean up associated files
    let message_to_delete = &chat.messages[message_index];

    // Clean up associated audio files
    if let Err(e) = delete_message_files(&character, message_to_delete).await {
        tracing::warn!(
            "Failed to delete some associated files for message {}: {}",
            message_index,
            e
        );
        // Don't fail the entire operation if file cleanup fails
    }

    chat.messages.remove(message_index);

    match save_chat(&character, &chat).await {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            data: Some(chat),
            message: "Message deleted successfully".to_string(),
        })),
        Err(e) => {
            tracing::error!("Failed to save chat after deleting message: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: format!("Failed to delete message: {e}"),
                }),
            ))
        }
    }
}

/// Mark a specific message as read
pub async fn mark_message_as_read(
    Path((character, message_index)): Path<(String, usize)>,
) -> Result<Json<ApiResponse<Chat>>, (StatusCode, Json<ApiResponse<()>>)> {
    let mut chat = match load_chat(&character).await {
        Ok(chat) => chat,
        Err(e) => {
            if e.to_string().contains("No such file or directory") {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("Chat for character '{character}' not found"),
                    }),
                ));
            } else {
                tracing::error!("Failed to load chat for character '{character}': {e}");
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("Failed to load chat: {e}"),
                    }),
                ));
            }
        }
    };

    if message_index >= chat.messages.len() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                data: None,
                message: format!("Message at index {message_index} not found"),
            }),
        ));
    }

    // Mark the message as read
    chat.messages[message_index].read = true;

    match save_chat(&character, &chat).await {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            data: Some(chat),
            message: "Message marked as read successfully".to_string(),
        })),
        Err(e) => {
            tracing::error!("Failed to save chat after marking message as read: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: format!("Failed to mark message as read: {e}"),
                }),
            ))
        }
    }
}

/// Mark all messages in a chat as read
pub async fn mark_all_messages_as_read(
    Path(character): Path<String>,
) -> Result<Json<ApiResponse<Chat>>, (StatusCode, Json<ApiResponse<()>>)> {
    let mut chat = match load_chat(&character).await {
        Ok(chat) => chat,
        Err(e) => {
            if e.to_string().contains("No such file or directory") {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("Chat for character '{character}' not found"),
                    }),
                ));
            } else {
                tracing::error!("Failed to load chat for character '{character}': {e}");
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("Failed to load chat: {e}"),
                    }),
                ));
            }
        }
    };

    // Mark all messages as read
    for message in &mut chat.messages {
        message.read = true;
    }

    match save_chat(&character, &chat).await {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            data: Some(chat),
            message: "All messages marked as read successfully".to_string(),
        })),
        Err(e) => {
            tracing::error!(
                "Failed to save chat after marking all messages as read: {}",
                e
            );
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: format!("Failed to mark all messages as read: {e}"),
                }),
            ))
        }
    }
}

// Helper functions

async fn load_all_chat_names() -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let chats_dir = PathBuf::from("data/chats");

    if !chats_dir.exists() {
        fs::create_dir_all(&chats_dir).await?;
        return Ok(vec![]);
    }

    let mut entries = fs::read_dir(chats_dir).await?;
    let mut chat_names = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        if let Some(file_name) = entry.file_name().to_str()
            && file_name.ends_with(".json")
        {
            let character_name = file_name.trim_end_matches(".json").to_string();
            chat_names.push(character_name);
        }
    }

    chat_names.sort();
    Ok(chat_names)
}

async fn load_chat(character: &str) -> Result<Chat, Box<dyn std::error::Error + Send + Sync>> {
    let file_path = PathBuf::from("data/chats").join(format!("{character}.json"));
    let contents = fs::read_to_string(file_path).await?;
    let chat: Chat = serde_json::from_str(&contents)?;
    Ok(chat)
}

async fn save_chat(
    character: &str,
    chat: &Chat,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let chats_dir = PathBuf::from("data/chats");

    if !chats_dir.exists() {
        fs::create_dir_all(&chats_dir).await?;
    }

    let file_path = chats_dir.join(format!("{character}.json"));
    let contents = serde_json::to_string_pretty(chat)?;
    fs::write(file_path, contents).await?;
    Ok(())
}

async fn delete_chat(character: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let file_path = PathBuf::from("data/chats").join(format!("{character}.json"));
    fs::remove_file(file_path).await?;
    Ok(())
}

/// Delete associated files (audio and images) for a message
async fn delete_message_files(
    character: &str,
    message: &Message,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Delete audio files
    for audio_file in &message.audio {
        let file_path = audio_file_path(character, audio_file);
        if let Err(e) = fs::remove_file(&file_path).await {
            tracing::warn!("Failed to delete audio file '{}': {}", file_path, e);
            // Continue with other files instead of failing entirely
        }
    }

    // Delete image files
    for image_file in &message.images {
        let file_path = image_file_path(character, image_file);
        if let Err(e) = fs::remove_file(&file_path).await {
            tracing::warn!("Failed to delete image file '{}': {}", file_path, e);
            // Continue with other files instead of failing entirely
        }
    }

    Ok(())
}

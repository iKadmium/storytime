use axum::{Json as JsonExtract, extract::Path, http::StatusCode, response::Json};
use tokio::fs;

use crate::models::{ApiResponse, CreatePromptRequest, Prompt, UpdatePromptRequest};
use crate::utils::{prompt_slug, prompt_file_path};

/// Get all prompts
pub async fn get_prompts()
-> Result<Json<ApiResponse<Vec<Prompt>>>, (StatusCode, Json<ApiResponse<()>>)> {
    match load_all_prompts().await {
        Ok(prompts) => Ok(Json(ApiResponse {
            success: true,
            data: Some(prompts),
            message: "Prompts retrieved successfully".to_string(),
        })),
        Err(e) => {
            tracing::error!("Failed to load prompts: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: format!("Failed to load prompts: {e}"),
                }),
            ))
        }
    }
}

/// Get a specific prompt by slug
pub async fn get_prompt(
    Path(slug): Path<String>,
) -> Result<Json<ApiResponse<Prompt>>, (StatusCode, Json<ApiResponse<()>>)> {
    match load_prompt_by_slug(&slug).await {
        Ok(prompt) => Ok(Json(ApiResponse {
            success: true,
            data: Some(prompt),
            message: "Prompt retrieved successfully".to_string(),
        })),
        Err(e) => {
            if e.to_string().contains("No such file or directory") || e.to_string().contains("Prompt not found") {
                Err((
                    StatusCode::NOT_FOUND,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("Prompt with slug '{slug}' not found"),
                    }),
                ))
            } else {
                tracing::error!("Failed to load prompt with slug '{slug}': {e}");
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("Failed to load prompt: {e}"),
                    }),
                ))
            }
        }
    }
}

/// Create a new prompt
pub async fn create_prompt(
    JsonExtract(request): JsonExtract<CreatePromptRequest>,
) -> Result<Json<ApiResponse<Prompt>>, (StatusCode, Json<ApiResponse<()>>)> {
    let prompt = Prompt {
        title: request.title.clone(),
        description: request.description,
        context: request.context,
        setup: request.setup,
        create_audio: request.create_audio,
        create_images: request.create_images,
    };

    match save_prompt(&prompt).await {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            data: Some(prompt),
            message: "Prompt created successfully".to_string(),
        })),
        Err(e) => {
            tracing::error!("Failed to create prompt '{}': {}", request.title, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: format!("Failed to create prompt: {e}"),
                }),
            ))
        }
    }
}

/// Update an existing prompt
pub async fn update_prompt(
    Path(slug): Path<String>,
    JsonExtract(request): JsonExtract<UpdatePromptRequest>,
) -> Result<Json<ApiResponse<Prompt>>, (StatusCode, Json<ApiResponse<()>>)> {
    // Load the existing prompt
    let mut prompt = match load_prompt_by_slug(&slug).await {
        Ok(prompt) => prompt,
        Err(e) => {
            if e.to_string().contains("No such file or directory") || e.to_string().contains("Prompt not found") {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("Prompt with slug '{slug}' not found"),
                    }),
                ));
            } else {
                tracing::error!("Failed to load prompt with slug '{slug}': {e}");
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("Failed to load prompt: {e}"),
                    }),
                ));
            }
        }
    };

    // Update fields if provided
    if let Some(description) = request.description {
        prompt.description = description;
    }
    if let Some(context) = request.context {
        prompt.context = context;
    }
    if let Some(setup) = request.setup {
        prompt.setup = setup;
    }
    if let Some(create_audio) = request.create_audio {
        prompt.create_audio = create_audio;
    }
    if let Some(create_images) = request.create_images {
        prompt.create_images = create_images;
    }

    // Save the updated prompt
    match save_prompt(&prompt).await {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            data: Some(prompt),
            message: "Prompt updated successfully".to_string(),
        })),
        Err(e) => {
            tracing::error!("Failed to update prompt with slug '{slug}': {e}");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: format!("Failed to update prompt: {e}"),
                }),
            ))
        }
    }
}

/// Delete a prompt
pub async fn delete_prompt(
    Path(slug): Path<String>,
) -> Result<Json<ApiResponse<()>>, (StatusCode, Json<ApiResponse<()>>)> {
    // First, find the prompt to get its title for deletion
    let prompt = match load_prompt_by_slug(&slug).await {
        Ok(prompt) => prompt,
        Err(e) => {
            if e.to_string().contains("Prompt not found") {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("Prompt with slug '{slug}' not found"),
                    }),
                ));
            } else {
                tracing::error!("Failed to find prompt with slug '{slug}': {e}");
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("Failed to find prompt: {e}"),
                    }),
                ));
            }
        }
    };

    match delete_prompt_file(&prompt.title).await {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            data: None,
            message: format!("Prompt '{}' deleted successfully", prompt.title),
        })),
        Err(e) => {
            tracing::error!("Failed to delete prompt '{}': {e}", prompt.title);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: format!("Failed to delete prompt: {e}"),
                }),
            ))
        }
    }
}

// Helper functions

/// Load all prompts from the data/prompts directory
async fn load_all_prompts() -> Result<Vec<Prompt>, Box<dyn std::error::Error + Send + Sync>> {
    let mut prompts = Vec::new();
    let mut dir = fs::read_dir("data/prompts").await?;

    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();

        // Skip non-JSON files
        if let Some(extension) = path.extension() {
            if extension != "json" {
                continue;
            }
        } else {
            continue;
        }

        let content = fs::read_to_string(&path).await?;
        let prompt: Prompt = serde_json::from_str(&content)?;
        prompts.push(prompt);
    }

    // Sort prompts by title for consistent ordering
    prompts.sort_by(|a, b| a.title.cmp(&b.title));
    Ok(prompts)
}


/// Load a specific prompt by slug
/// This searches all prompts to find one whose title converts to the given slug
async fn load_prompt_by_slug(slug: &str) -> Result<Prompt, Box<dyn std::error::Error + Send + Sync>> {
    let prompts = load_all_prompts().await?;
    
    for prompt in prompts {
        if prompt_slug(&prompt.title) == slug {
            return Ok(prompt);
        }
    }
    
    Err(format!("Prompt not found with slug: {slug}").into())
}

/// Save a prompt to the data/prompts directory
async fn save_prompt(prompt: &Prompt) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let path = prompt_file_path(&prompt.title);
    let content = serde_json::to_string_pretty(prompt)?;
    fs::write(path, content).await?;
    Ok(())
}

/// Delete a prompt file
async fn delete_prompt_file(title: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let path = prompt_file_path(title);
    fs::remove_file(path).await?;
    Ok(())
}





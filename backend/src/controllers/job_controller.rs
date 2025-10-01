use axum::{
    extract::{Json as JsonExtract, Path, State},
    http::StatusCode,
    response::Json,
};
use base64::Engine;
use std::sync::Arc;
use tokio::fs;
use uuid::Uuid;

use crate::chatml::ChatMLPrompt;
use crate::llm_prompt::build_setup_item_chatml_prompt;
use crate::models::{
    ApiResponse, Character, Chat, CreateJobRequest, Job, Message, Prompt, RunJobRequest, Settings,
    TestCharacterRequest, TestPromptRequest, UpdateJobRequest, Voice,
};
use crate::utils::{
    character_file_path, chat_file_path_from_character, job_file_path, job_file_path_from_slug,
    job_slug, prompt_file_path,
};
use crate::{
    ai_services::{call_llm, call_llm_chatml, call_tts},
    utils::to_slug,
};

/// Get all jobs
pub async fn get_jobs() -> Result<Json<ApiResponse<Vec<Job>>>, (StatusCode, Json<ApiResponse<()>>)>
{
    match load_all_jobs().await {
        Ok(jobs) => Ok(Json(ApiResponse {
            success: true,
            data: Some(jobs),
            message: "Jobs retrieved successfully".to_string(),
        })),
        Err(e) => {
            tracing::error!("Failed to load jobs: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: format!("Failed to load jobs: {e}"),
                }),
            ))
        }
    }
}

/// Get a specific job by slug
pub async fn get_job(
    Path(slug): Path<String>,
) -> Result<Json<ApiResponse<Job>>, (StatusCode, Json<ApiResponse<()>>)> {
    match load_job_by_slug(&slug).await {
        Ok(job) => Ok(Json(ApiResponse {
            success: true,
            data: Some(job),
            message: "Job retrieved successfully".to_string(),
        })),
        Err(e) => {
            if e.to_string().contains("No such file or directory") {
                Err((
                    StatusCode::NOT_FOUND,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("Job with slug '{slug}' not found"),
                    }),
                ))
            } else {
                tracing::error!("Failed to load job with slug '{slug}': {e}");
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("Failed to load job: {e}"),
                    }),
                ))
            }
        }
    }
}

/// Create a new job
pub async fn create_job(
    JsonExtract(request): JsonExtract<CreateJobRequest>,
) -> Result<Json<ApiResponse<Job>>, (StatusCode, Json<ApiResponse<()>>)> {
    let job = Job {
        characters: request.characters,
        prompts: request.prompts,
        cadence: request.cadence,
        prompt_override: request.prompt_override,
    };

    match save_job(&job).await {
        Ok(slug) => Ok(Json(ApiResponse {
            success: true,
            data: Some(job),
            message: format!("Job created successfully with slug '{slug}'"),
        })),
        Err(e) => {
            tracing::error!("Failed to create job: {e}");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: format!("Failed to create job: {e}"),
                }),
            ))
        }
    }
}

/// Update an existing job
pub async fn update_job(
    Path(slug): Path<String>,
    JsonExtract(request): JsonExtract<UpdateJobRequest>,
) -> Result<Json<ApiResponse<Job>>, (StatusCode, Json<ApiResponse<()>>)> {
    match load_job_by_slug(&slug).await {
        Ok(_existing_job) => {
            // PUT replaces the entire resource
            let job = Job {
                characters: request.characters,
                prompts: request.prompts,
                cadence: request.cadence,
                prompt_override: request.prompt_override,
            };

            match save_job(&job).await {
                Ok(new_slug) => Ok(Json(ApiResponse {
                    success: true,
                    data: Some(job),
                    message: format!("Job updated successfully with slug '{new_slug}'"),
                })),
                Err(e) => {
                    tracing::error!("Failed to update job with slug '{slug}': {e}");
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ApiResponse {
                            success: false,
                            data: None,
                            message: format!("Failed to update job: {e}"),
                        }),
                    ))
                }
            }
        }
        Err(e) => {
            if e.to_string().contains("No such file or directory") {
                Err((
                    StatusCode::NOT_FOUND,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("Job with slug '{slug}' not found"),
                    }),
                ))
            } else {
                tracing::error!("Failed to load job with slug '{slug}': {e}");
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("Failed to update job: {e}"),
                    }),
                ))
            }
        }
    }
}

/// Delete a job by slug
pub async fn delete_job(
    Path(slug): Path<String>,
) -> Result<Json<ApiResponse<()>>, (StatusCode, Json<ApiResponse<()>>)> {
    // First, find the job to get its character and prompt for deletion
    let job = match load_job_by_slug(&slug).await {
        Ok(job) => job,
        Err(e) => {
            if e.to_string().contains("No such file or directory") {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("Job with slug '{slug}' not found"),
                    }),
                ));
            } else {
                tracing::error!("Failed to find job with slug '{slug}': {e}");
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("Failed to find job: {e}"),
                    }),
                ));
            }
        }
    };

    match delete_job_by_slug(&slug).await {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            data: None,
            message: format!(
                "Job '{}' deleted successfully",
                job_slug(
                    job.characters.first().unwrap_or(&"default".to_string()),
                    job.prompts.first().unwrap_or(&"default".to_string())
                )
            ),
        })),
        Err(e) => {
            tracing::error!("Failed to delete job with slug '{slug}': {e}");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: format!("Failed to delete job: {e}"),
                }),
            ))
        }
    }
}

/// Run a job by its slug
pub async fn run_job_by_slug(
    State(settings): State<Arc<Settings>>,
    Path(slug): Path<String>,
) -> Result<Json<ApiResponse<Message>>, (StatusCode, Json<ApiResponse<()>>)> {
    match load_job_by_slug(&slug).await {
        Ok(job) => run_job_internal(job, settings, true).await,
        Err(e) => {
            if e.to_string().contains("No such file or directory") {
                Err((
                    StatusCode::NOT_FOUND,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("Job with slug '{slug}' not found"),
                    }),
                ))
            } else {
                tracing::error!("Failed to load job by slug {}: {}", slug, e);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("Failed to load job: {e}"),
                    }),
                ))
            }
        }
    }
}

/// Run a job via a run job request
pub async fn run_job(
    State(settings): State<Arc<Settings>>,
    JsonExtract(request): JsonExtract<RunJobRequest>,
) -> Result<Json<ApiResponse<Message>>, (StatusCode, Json<ApiResponse<()>>)> {
    run_job_internal(request.job, settings, request.save_to_chat_history).await
}

/// Internal job execution implementation
async fn run_job_internal(
    job: Job,
    settings: Arc<Settings>,
    save_to_chat_history: bool,
) -> Result<Json<ApiResponse<Message>>, (StatusCode, Json<ApiResponse<()>>)> {
    use rand::{SeedableRng, rngs::StdRng, seq::SliceRandom};

    // Validate that we have at least one character and prompt
    if job.characters.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                data: None,
                message: "Job must have at least one character".to_string(),
            }),
        ));
    }

    if job.prompts.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                data: None,
                message: "Job must have at least one prompt".to_string(),
            }),
        ));
    }

    // Randomly select a character and prompt using a Send-safe RNG
    let mut rng = StdRng::from_entropy();
    let selected_character_name = job.characters.choose(&mut rng).unwrap();
    let selected_prompt_name = job.prompts.choose(&mut rng).unwrap();

    tracing::info!(
        "Randomly selected character '{}' and prompt '{}' for job execution",
        selected_character_name,
        selected_prompt_name
    );

    // Load the selected character
    let character = match load_character(selected_character_name).await {
        Ok(character) => character,
        Err(e) => {
            tracing::error!(
                "Failed to load character '{}': {}",
                selected_character_name,
                e
            );
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: format!(
                        "Failed to load character '{}': {}",
                        selected_character_name, e
                    ),
                }),
            ));
        }
    };

    // Load the selected prompt
    let prompt = match load_prompt(selected_prompt_name).await {
        Ok(prompt) => prompt,
        Err(e) => {
            tracing::error!("Failed to load prompt '{}': {}", selected_prompt_name, e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: format!("Failed to load prompt '{}': {}", selected_prompt_name, e),
                }),
            ));
        }
    };

    run_job_with_character_and_prompt(job, character, prompt, settings, save_to_chat_history).await
}

// File I/O utility functions

/// Load all jobs from the data/jobs directory
async fn load_all_jobs() -> Result<Vec<Job>, Box<dyn std::error::Error + Send + Sync>> {
    let mut jobs = Vec::new();
    let dir_path = "./data/jobs";

    let mut entries = fs::read_dir(dir_path).await?;
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = fs::read_to_string(&path).await?;
            let job: Job = serde_json::from_str(&content)?;
            jobs.push(job);
        }
    }

    // Sort jobs by first character name and then by first prompt for consistent ordering
    jobs.sort_by(|a, b| {
        let a_first_char = a.characters.first().map(|s| s.as_str()).unwrap_or("");
        let b_first_char = b.characters.first().map(|s| s.as_str()).unwrap_or("");
        let a_first_prompt = a.prompts.first().map(|s| s.as_str()).unwrap_or("");
        let b_first_prompt = b.prompts.first().map(|s| s.as_str()).unwrap_or("");

        a_first_char
            .cmp(b_first_char)
            .then_with(|| a_first_prompt.cmp(b_first_prompt))
    });
    Ok(jobs)
}

/// Load a specific job by slug
async fn load_job_by_slug(slug: &str) -> Result<Job, Box<dyn std::error::Error + Send + Sync>> {
    let file_path = job_file_path_from_slug(slug);
    let content = fs::read_to_string(&file_path).await?;
    let job: Job = serde_json::from_str(&content)?;
    Ok(job)
}

/// Save a job to a JSON file
/// Generates a slug based on first character and first prompt
async fn save_job(job: &Job) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let default_character = "default".to_string();
    let default_prompt = "default".to_string();
    let first_character = job.characters.first().unwrap_or(&default_character);
    let first_prompt = job.prompts.first().unwrap_or(&default_prompt);
    let slug = job_slug(first_character, first_prompt);
    let file_path = job_file_path(first_character, first_prompt);
    let json_content = serde_json::to_string_pretty(job)?;
    fs::write(&file_path, json_content).await?;
    Ok(slug)
}

/// Delete a job file by slug
async fn delete_job_by_slug(slug: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let file_path = job_file_path_from_slug(slug);
    fs::remove_file(&file_path).await?;
    Ok(())
}

/// Load a character by name
async fn load_character(name: &str) -> Result<Character, Box<dyn std::error::Error + Send + Sync>> {
    let file_path = character_file_path(name);
    let content = fs::read_to_string(&file_path).await?;
    let character: Character = serde_json::from_str(&content)?;
    Ok(character)
}

/// Load a prompt by title
async fn load_prompt(title: &str) -> Result<Prompt, Box<dyn std::error::Error + Send + Sync>> {
    let file_path = prompt_file_path(title);
    let content = fs::read_to_string(&file_path).await?;
    let prompt: Prompt = serde_json::from_str(&content)?;
    Ok(prompt)
}

/// Load a chat by character name
async fn load_chat(character: &str) -> Result<Chat, Box<dyn std::error::Error + Send + Sync>> {
    let file_path = chat_file_path_from_character(character);
    let content = fs::read_to_string(&file_path).await?;
    let chat: Chat = serde_json::from_str(&content)?;
    Ok(chat)
}

/// Save a message to chat history
async fn save_message_to_chat(
    character: &str,
    message: &Message,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let file_path = chat_file_path_from_character(character);

    // Load existing chat or create new one
    let mut chat = match load_chat(character).await {
        Ok(chat) => chat,
        Err(_) => Chat {
            character: character.to_string(),
            messages: Vec::new(),
        },
    };

    // Add the new message
    chat.messages.push(message.clone());

    // Save back to file
    let json_content = serde_json::to_string_pretty(&chat)?;
    fs::write(&file_path, json_content).await?;

    Ok(())
}

/// Internal job execution implementation using provided prompt
async fn run_job_internal_with_prompt(
    job: Job,
    prompt: Prompt,
    settings: Arc<Settings>,
    save_to_chat_history: bool,
) -> Result<Json<ApiResponse<Message>>, (StatusCode, Json<ApiResponse<()>>)> {
    // Get the first character name (for testing, we expect only one)
    let character_name = job.characters.first().ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                data: None,
                message: "Job must have at least one character".to_string(),
            }),
        )
    })?;

    // Load the character
    let character = match load_character(character_name).await {
        Ok(character) => character,
        Err(e) => {
            tracing::error!("Failed to load character '{}': {}", character_name, e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: format!("Failed to load character '{}': {}", character_name, e),
                }),
            ));
        }
    };

    run_job_with_character_and_prompt(job, character, prompt, settings, save_to_chat_history).await
}

/// Internal job execution implementation using provided character
async fn run_job_internal_with_character(
    job: Job,
    character: Character,
    prompt: Prompt,
    settings: Arc<Settings>,
    save_to_chat_history: bool,
) -> Result<Json<ApiResponse<Message>>, (StatusCode, Json<ApiResponse<()>>)> {
    run_job_with_character_and_prompt(job, character, prompt, settings, save_to_chat_history).await
}

/// Unified AI workflow execution - the ONLY function that should call AI services directly
async fn execute_ai_workflow(
    job: &Job,
    character: &Character,
    prompt: &Prompt,
    settings: &Arc<Settings>,
    chat_history: &Chat,
) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    // Generate LLM responses based on prompt configuration
    if let Some(override_prompt) = &job.prompt_override {
        // Use simple prompt override approach
        call_llm(settings, override_prompt).await
    } else {
        // Use iterative ChatML approach for complex prompts
        let mut accumulated_chatml_responses: Vec<ChatMLPrompt> = Vec::new();
        let mut final_responses = Vec::new();

        for (i, setup_item) in prompt.setup.iter().enumerate() {
            let chatml_prompt = build_setup_item_chatml_prompt(
                character,
                prompt,
                setup_item,
                &accumulated_chatml_responses,
                chat_history,
            );

            let response_prompt = call_llm_chatml(settings, &chatml_prompt).await?;

            tracing::info!(
                "LLM ChatML response for setup item {}: {:#?}",
                i + 1,
                response_prompt
            );

            // Extract the assistant's response from the ChatML prompt
            if let Some(assistant_response) = response_prompt.last_assistant_message() {
                assistant_response
                    .to_string()
                    .replace("<br>", "\n")
                    .split_terminator(['\n', '\r'])
                    .for_each(|line| {
                        if !line.trim().is_empty() {
                            final_responses.push(line.trim().to_string());
                        }
                    });
            }

            // Add the response prompt to accumulated responses for next iteration
            accumulated_chatml_responses.push(response_prompt);
        }

        Ok(final_responses)
    }
}

/// Execute AI services (LLM and TTS) for a given job configuration
/// This is the single function that calls AI services - all other functions should use this
async fn execute_ai_services(
    job: &Job,
    character: &Character,
    prompt: &Prompt,
    settings: Arc<Settings>,
    save_to_chat_history: bool,
) -> Result<Message, (StatusCode, Json<ApiResponse<()>>)> {
    // Try to load existing chat history (optional)
    let chat_history = Chat {
        character: character.name.clone(),
        messages: Vec::new(),
    };

    // Execute the unified AI workflow
    let llm_responses =
        match execute_ai_workflow(job, character, prompt, &settings, &chat_history).await {
            Ok(responses) => responses,
            Err(e) => {
                tracing::error!("AI workflow execution failed: {}", e);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("AI workflow execution failed: {e}"),
                    }),
                ));
            }
        };

    // Generate TTS audio if requested
    let mut audio: Vec<String> = vec![];
    if prompt.create_audio {
        let default_voice = Voice::default();
        let voice = character.voice.as_ref().unwrap_or(&default_voice);
        match call_tts(&settings, &llm_responses.join("\n"), voice).await {
            Ok(tts_audio) => {
                if save_to_chat_history {
                    // Save the audio to a file in data/audio/{character}/{id}.mp3
                    let id = Uuid::new_v4();
                    let character_slug = to_slug(&character.name);
                    let chat_audio_path = format!("./data/audio/{character_slug}/{id}.mp3");

                    // Ensure the directory exists
                    let audio_dir = format!("./data/audio/{character_slug}");
                    if let Err(e) = fs::create_dir_all(&audio_dir).await {
                        tracing::warn!("Failed to create audio directory '{}': {}", audio_dir, e);
                    }

                    if let Err(e) = fs::write(&chat_audio_path, &tts_audio).await {
                        tracing::warn!("Failed to save TTS audio to '{}': {}", chat_audio_path, e);
                    } else {
                        tracing::info!("TTS audio saved to '{}'", chat_audio_path);
                    }
                    audio.push(id.to_string());
                } else {
                    // Return base64 encoded audio for test endpoints
                    let base64_audio = base64::engine::general_purpose::STANDARD.encode(&tts_audio);
                    audio.push(base64_audio);
                }
            }
            Err(e) => {
                tracing::error!("TTS call failed: {}", e);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: format!("TTS call failed: {e}"),
                    }),
                ));
            }
        }
    }

    // Create and return the message
    Ok(Message {
        text: llm_responses,
        audio,
        images: vec![], // TODO: Add image generation support if needed
        read: false,    // New messages are unread by default
    })
}

/// Core job execution logic using provided character and prompt
async fn run_job_with_character_and_prompt(
    job: Job,
    character: Character,
    prompt: Prompt,
    settings: Arc<Settings>,
    save_to_chat_history: bool,
) -> Result<Json<ApiResponse<Message>>, (StatusCode, Json<ApiResponse<()>>)> {
    // Execute AI services (LLM + TTS)
    let message =
        execute_ai_services(&job, &character, &prompt, settings, save_to_chat_history).await?;

    // Optionally save the message to chat history
    if save_to_chat_history && let Err(e) = save_message_to_chat(&character.name, &message).await {
        tracing::warn!("Failed to save message to chat history: {}", e);
        // Don't fail the entire operation if we can't save to chat history
    }

    Ok(Json(ApiResponse {
        success: true,
        data: Some(message),
        message: "Job executed successfully".to_string(),
    }))
}

/// Test a prompt with a specific character
pub async fn test_prompt_with_character(
    State(settings): State<Arc<Settings>>,
    JsonExtract(request): JsonExtract<TestPromptRequest>,
) -> Result<Json<ApiResponse<Message>>, (StatusCode, Json<ApiResponse<()>>)> {
    // Create a temporary job with the provided prompt and character
    let job = Job {
        characters: vec![request.character_name],
        prompts: vec![request.prompt.title.clone()],
        cadence: "once".to_string(),
        prompt_override: None,
    };

    // Create a modified run_job_internal call that uses the provided prompt directly
    run_job_internal_with_prompt(job, request.prompt, settings, request.save_to_chat_history).await
}

/// Test a character with a specific prompt
pub async fn test_character_with_prompt(
    State(settings): State<Arc<Settings>>,
    JsonExtract(request): JsonExtract<TestCharacterRequest>,
) -> Result<Json<ApiResponse<Message>>, (StatusCode, Json<ApiResponse<()>>)> {
    // Load the prompt by name
    let prompt = match load_prompt(&request.prompt_name).await {
        Ok(prompt) => prompt,
        Err(e) => {
            tracing::error!("Failed to load prompt '{}': {}", request.prompt_name, e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: format!("Failed to load prompt '{}': {}", request.prompt_name, e),
                }),
            ));
        }
    };

    // Create a temporary job with the provided character and prompt
    let job = Job {
        characters: vec![request.character.name.clone()],
        prompts: vec![request.prompt_name],
        cadence: "once".to_string(),
        prompt_override: None,
    };

    // Create a modified run_job_internal call that uses the provided character directly
    run_job_internal_with_character(
        job,
        request.character,
        prompt,
        settings,
        request.save_to_chat_history,
    )
    .await
}

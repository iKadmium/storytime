use axum::{
    Router,
    routing::{get, post, put},
};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

use crate::controllers::{
    audio_controller::serve_audio,
    character_controller::{
        create_character, delete_character, get_character, get_characters, update_character,
    },
    chat_controller::{
        add_message, create_chat, delete_chat_endpoint, delete_message, get_chat, get_chats,
        mark_all_messages_as_read, mark_message_as_read, update_chat, update_message,
    },
    health_controller::{health_check, hello, hello_name},
    job_controller::{
        create_job, delete_job, get_job, get_jobs, run_job, run_job_by_slug,
        test_character_with_prompt, test_prompt_with_character, update_job,
    },
    prompt_controller::{create_prompt, delete_prompt, get_prompt, get_prompts, update_prompt},
};
use crate::models::Settings;

pub fn create_app(settings: Arc<Settings>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/hello", get(hello))
        .route("/api/hello/{name}", get(hello_name))
        // Audio file serving
        .route("/audio/{character}/{filename}", get(serve_audio))
        // Character CRUD routes
        .route(
            "/api/characters",
            get(get_characters).post(create_character),
        )
        .route(
            "/api/characters/{slug}",
            get(get_character)
                .put(update_character)
                .delete(delete_character),
        )
        // Prompt CRUD routes
        .route("/api/prompts", get(get_prompts).post(create_prompt))
        .route(
            "/api/prompts/{slug}",
            get(get_prompt).put(update_prompt).delete(delete_prompt),
        )
        // Chat CRUD routes
        .route("/api/chats", get(get_chats).post(create_chat))
        .route(
            "/api/chats/{character}",
            get(get_chat).put(update_chat).delete(delete_chat_endpoint),
        )
        // Message CRUD routes within chats
        .route("/api/chats/{character}/messages", post(add_message))
        .route(
            "/api/chats/{character}/messages/{message_index}",
            put(update_message).delete(delete_message),
        )
        // Mark messages as read routes
        .route(
            "/api/chats/{character}/read-all",
            put(mark_all_messages_as_read),
        )
        .route(
            "/api/chats/{character}/messages/{message_index}/read",
            put(mark_message_as_read),
        )
        // Job CRUD routes
        .route("/api/jobs", get(get_jobs).post(create_job))
        .route(
            "/api/jobs/{slug}",
            get(get_job).put(update_job).delete(delete_job),
        )
        // Job execution routes
        .route("/api/jobs/{slug}/run", post(run_job_by_slug))
        .route("/api/jobs/run", post(run_job))
        // Test routes
        .route("/api/test/prompt", post(test_prompt_with_character))
        .route("/api/test/character", post(test_character_with_prompt))
        // Static file serving from assets directory - serve at root for SPA compatibility
        .fallback_service(
            ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html")),
        )
        .with_state(settings)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive()),
        )
}

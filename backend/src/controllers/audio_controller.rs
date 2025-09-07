use axum::{
    extract::Path,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use tokio::fs;

/// Serve audio files from the data/audio directory
pub async fn serve_audio(Path((character, filename)): Path<(String, String)>) -> Response {
    // Construct the file path
    let file_path = format!("./data/audio/{character}/{filename}");

    // Validate that the filename ends with .mp3 for security
    if !filename.ends_with(".mp3") {
        return (StatusCode::BAD_REQUEST, "Invalid file type").into_response();
    }

    // Read the file
    match fs::read(&file_path).await {
        Ok(file_contents) => {
            // Return the audio file with appropriate headers
            (
                StatusCode::OK,
                [
                    (header::CONTENT_TYPE, "audio/mpeg"),
                    (header::CACHE_CONTROL, "public, max-age=3600"), // Cache for 1 hour
                ],
                file_contents,
            )
                .into_response()
        }
        Err(_) => {
            // File not found or other error
            (StatusCode::NOT_FOUND, "Audio file not found").into_response()
        }
    }
}

use crate::chatml::ChatMLPrompt;
use crate::models::{KoboldCppGenerate, KoboldCppResponse, Settings, TtsRequest, Voice};

/// Calls the LLM API with a ChatML prompt and returns the response parsed as a ChatMLPrompt
pub(crate) async fn call_llm_chatml(
    settings: &Settings,
    chatml_prompt: &ChatMLPrompt,
) -> Result<ChatMLPrompt, Box<dyn std::error::Error + Send + Sync>> {
    let prompt_string = chatml_prompt.to_chatml_for_generation();

    let kobold_request = KoboldCppGenerate {
        max_context_length: 8192,
        max_length: 1000,
        prompt: prompt_string.clone(),
        quiet: false,
        rep_pen: 1.0,
        rep_pen_range: 1024,
        rep_pen_slope: 0.7,
        temperature: 0.9,
        tfs: 1.0,
        top_a: 0.0,
        top_k: 100,
        top_p: 0.9,
        typical: 1.0,
    };

    // Debug trace the request being sent
    tracing::debug!("Sending LLM request to: {}", settings.llm_api);
    tracing::trace!("Kobold request parameters: {:#?}", kobold_request);

    let client = reqwest::Client::new();
    let timer = std::time::Instant::now();
    let response = client
        .post(&settings.llm_api)
        .json(&kobold_request)
        .send()
        .await?;

    let duration = timer.elapsed();

    if !response.status().is_success() {
        return Err(format!(
            "LLM API returned error: {}, duration: {:?}",
            response.status(),
            duration
        )
        .into());
    } else {
        tracing::debug!(
            "LLM API returned success: {}, duration: {:?}",
            response.status(),
            duration
        );
    }

    let kobold_response: KoboldCppResponse = response.json().await?;

    // Debug trace the raw response
    tracing::trace!("Raw Kobold response: {:#?}", kobold_response);

    if let Some(result) = kobold_response.results.first() {
        // Create a complete ChatML string with the original prompt + assistant response
        let complete_chatml = format!("{}{}", prompt_string, result.text.trim());

        // Parse the complete response as ChatML
        match ChatMLPrompt::from_chatml(&complete_chatml) {
            Ok(parsed_prompt) => {
                tracing::trace!("Parsed ChatML response: {:#?}", parsed_prompt);
                Ok(parsed_prompt)
            }
            Err(e) => {
                tracing::warn!(
                    "Failed to parse response as ChatML, falling back to simple assistant message: {}",
                    e
                );
                // Fallback: create a simple response with just the assistant message
                let mut fallback_prompt = chatml_prompt.clone();
                fallback_prompt.add_assistant(&result.text);
                Ok(fallback_prompt)
            }
        }
    } else {
        Err("No results returned from LLM".into())
    }
}

/// Calls the LLM API with the given prompt and returns the processed result as lines
pub(crate) async fn call_llm(
    settings: &Settings,
    prompt: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let kobold_request = KoboldCppGenerate {
        max_context_length: 8192,
        max_length: 200,
        prompt: prompt.to_string(),
        quiet: false,
        rep_pen: 1.0,
        rep_pen_range: 1024,
        rep_pen_slope: 0.7,
        temperature: 0.9,
        tfs: 1.0,
        top_a: 0.0,
        top_k: 100,
        top_p: 0.9,
        typical: 1.0,
    };

    // Debug trace the request being sent
    tracing::debug!("Sending LLM request to: {}", settings.llm_api);
    tracing::debug!("Prompt being sent: {}", prompt);
    tracing::debug!("Kobold request parameters: {:#?}", kobold_request);

    let client = reqwest::Client::new();
    let timer = std::time::Instant::now();
    let response = client
        .post(&settings.llm_api)
        .json(&kobold_request)
        .send()
        .await?;

    let duration = timer.elapsed();

    if !response.status().is_success() {
        return Err(format!(
            "LLM API returned error: {}, duration: {:?}",
            response.status(),
            duration
        )
        .into());
    } else {
        tracing::debug!(
            "LLM API returned success: {}, duration: {:?}",
            response.status(),
            duration
        );
    }

    let kobold_response: KoboldCppResponse = response.json().await?;

    // Debug trace the raw response
    tracing::info!("Raw Kobold response: {:#?}", kobold_response);

    if let Some(result) = kobold_response.results.first() {
        tracing::debug!("Raw result text: {}", result.text);

        let marked_content = extract_marked_content(&result.text, "<|im_start|>", "<|im_end|>");
        tracing::debug!("Extracted marked content: {:#?}", marked_content);

        if !marked_content.is_empty() {
            Ok(marked_content)
        } else {
            tracing::debug!("No marked content found, returning raw text");
            Ok(vec![result.text.clone()])
        }
    } else {
        Err("No results returned from LLM".into())
    }
}

/// Extracts all occurrences of text between start_marker and end_marker
/// Returns a vector of all valid marked content found
fn extract_marked_content(text: &str, start_marker: &str, end_marker: &str) -> Vec<String> {
    let mut results = Vec::new();
    let mut current_pos = 0;

    while let Some(start_pos) = text[current_pos..].find(start_marker) {
        let absolute_start = current_pos + start_pos + start_marker.len();

        if let Some(end_pos) = text[absolute_start..].find(end_marker) {
            let absolute_end = absolute_start + end_pos;
            results.push(text[absolute_start..absolute_end].to_string());

            // Move past this end marker to look for the next pair
            current_pos = absolute_end + end_marker.len();
        } else {
            // No matching end marker found, move past this start marker
            current_pos = current_pos + start_pos + start_marker.len();
        }
    }

    results
}

/// Calls the TTS API with the given text and returns the audio data
pub(crate) async fn call_tts(
    settings: &Settings,
    text: &str,
    voice: &Voice,
) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    let tts_request = TtsRequest {
        text: text.to_string(),
        temperature: voice.temperature,
        exaggeration: voice.exaggeration,
        cfg_weight: voice.cfg_weight,
        speed_factor: voice.speed_factor,
        seed: 42,
        language: "en".to_string(),
        voice_mode: "clone".to_string(),
        split_text: true,
        chunk_size: 240,
        output_format: "mp3".to_string(),
        reference_audio_filename: voice.voice_name.clone(),
    };

    let client = reqwest::Client::new();
    tracing::debug!("Sending TTS request to: {}", settings.tts_api);
    tracing::trace!("TTS request parameters: {:#?}", tts_request);
    let timer = std::time::Instant::now();
    let response = client
        .post(&settings.tts_api)
        .json(&tts_request)
        .send()
        .await?;

    let duration = timer.elapsed();
    if !response.status().is_success() {
        return Err(format!(
            "TTS API returned error: {}, duration: {:?}",
            response.status(),
            duration
        )
        .into());
    } else {
        tracing::debug!(
            "TTS API returned success: {}, duration: {:?}",
            response.status(),
            duration
        );
    }

    let audio_bytes = response.bytes().await?;
    Ok(audio_bytes.to_vec())
}

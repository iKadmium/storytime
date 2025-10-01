use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct GenerateRequest {
    pub character: String,
    pub prompt: String,
}

#[derive(Serialize, Deserialize)]
pub struct GenerateLlmRequest {
    pub character: String,
    pub prompt: String,
}

#[derive(Serialize, Deserialize)]
pub struct GenerateTtsRequest {
    pub character: String,
    pub prompt: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GenerateResponse {
    pub result: Vec<String>,
    pub audio_data: Option<String>, // Base64 encoded audio data
}

#[derive(Serialize, Deserialize)]
pub struct GenerateLlmResponse {
    pub result: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GenerateTtsResponse {
    pub result: String, // Base64 encoded audio data
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    #[serde(rename = "ttsApi")]
    pub tts_api: String,
    #[serde(rename = "llmApi")]
    pub llm_api: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KoboldCppGenerate {
    pub max_context_length: u32,
    pub max_length: u32,
    pub prompt: String,
    pub quiet: bool,
    pub rep_pen: f64,
    pub rep_pen_range: u32,
    pub rep_pen_slope: f64,
    pub temperature: f64,
    pub tfs: f64,
    pub top_a: f64,
    pub top_k: u32,
    pub top_p: f64,
    pub typical: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KoboldCppResult {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KoboldCppResponse {
    pub results: Vec<KoboldCppResult>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Voice {
    pub temperature: f64,
    pub exaggeration: f64,
    #[serde(rename = "cfgWeight")]
    pub cfg_weight: f64,
    #[serde(rename = "speedFactor")]
    pub speed_factor: f64,
    #[serde(rename = "voiceName")]
    pub voice_name: String,
}

impl Default for Voice {
    fn default() -> Self {
        Self {
            temperature: 0.8,
            exaggeration: 0.4,
            cfg_weight: 0.5,
            speed_factor: 1.0,
            voice_name: "default.wav".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    pub name: String,
    pub description: String,
    pub personality: String,
    pub background: String,
    #[serde(default)]
    pub voice: Option<Voice>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Prompt {
    pub title: String,
    pub description: String,
    pub context: String,
    pub setup: Vec<String>,
    #[serde(default)]
    pub create_audio: bool,
    #[serde(default)]
    pub create_images: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TtsRequest {
    pub text: String,
    pub temperature: f64,
    pub exaggeration: f64,
    pub cfg_weight: f64,
    pub speed_factor: f64,
    pub seed: i32,
    pub language: String,
    pub voice_mode: String,
    pub split_text: bool,
    pub chunk_size: i32,
    pub output_format: String,
    pub reference_audio_filename: String,
}

// Character CRUD request/response models
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCharacterRequest {
    pub name: String,
    pub description: String,
    pub personality: String,
    pub background: String,
    pub voice: Option<Voice>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCharacterRequest {
    pub description: Option<String>,
    pub personality: Option<String>,
    pub background: Option<String>,
    pub voice: Option<Voice>,
}

// Prompt CRUD request/response models
#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePromptRequest {
    pub title: String,
    pub description: String,
    pub context: String,
    pub setup: Vec<String>,
    #[serde(default)]
    pub create_audio: bool,
    #[serde(default)]
    pub create_images: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePromptRequest {
    pub description: Option<String>,
    pub context: Option<String>,
    pub setup: Option<Vec<String>>,
    pub create_audio: Option<bool>,
    pub create_images: Option<bool>,
}

// Chat and Message models
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub text: Vec<String>,
    #[serde(default)]
    pub audio: Vec<String>,
    #[serde(default)]
    pub images: Vec<String>,
    #[serde(default)]
    pub read: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chat {
    pub character: String,
    pub messages: Vec<Message>,
}

// Chat CRUD request/response models
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateChatRequest {
    pub character: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateChatRequest {
    pub character: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddMessageRequest {
    pub text: Vec<String>,
    #[serde(default)]
    pub audio: Vec<String>,
    #[serde(default)]
    pub images: Vec<String>,
    #[serde(default)]
    pub read: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateMessageRequest {
    pub text: Option<Vec<String>>,
    pub audio: Option<Vec<String>>,
    pub images: Option<Vec<String>>,
    pub read: Option<bool>,
}

// Job models
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Job {
    pub id: Option<uuid::Uuid>,
    pub characters: Vec<String>,
    pub prompts: Vec<String>,
    pub cadence: String,
    #[serde(rename = "prompt-override")]
    pub prompt_override: Option<String>,
}

// Job CRUD request/response models
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateJobRequest {
    pub characters: Vec<String>,
    pub prompts: Vec<String>,
    pub cadence: String,
    #[serde(rename = "prompt-override")]
    pub prompt_override: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateJobRequest {
    pub id: Option<uuid::Uuid>,
    pub characters: Vec<String>,
    pub prompts: Vec<String>,
    pub cadence: String,
    #[serde(rename = "prompt-override")]
    pub prompt_override: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RunJobRequest {
    pub job: Job,
    #[serde(default = "default_true")]
    pub save_to_chat_history: bool,
}

fn default_true() -> bool {
    true
}

// Test request models
#[derive(Serialize, Deserialize, Debug)]
pub struct TestPromptRequest {
    pub prompt: Prompt,
    pub character_name: String,
    #[serde(default = "default_false")]
    pub save_to_chat_history: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestCharacterRequest {
    pub character: Character,
    pub prompt_name: String,
    #[serde(default = "default_false")]
    pub save_to_chat_history: bool,
}

fn default_false() -> bool {
    false
}

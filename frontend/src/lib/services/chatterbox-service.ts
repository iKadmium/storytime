/**
 * Chatterbox TTS Service
 * 
 * A comprehensive TypeScript service for interacting with the Chatterbox TTS server.
 * 
 * Features:
 * - Custom TTS generation with advanced parameters
 * - OpenAI compatible API endpoint 
 * - Voice management (predefined and cloned voices)
 * - File upload for reference audio and predefined voices
 * - Configuration management
 * - Audio blob utilities
 * 
 * Example usage:
 * ```typescript
 * import { generateCustomTTS, getPredefinedVoices } from './chatterbox-service';
 * 
 * // Generate TTS with a predefined voice
 * const audioBlob = await generateCustomTTS({
 *   text: "Hello, world!",
 *   voice_mode: "predefined",
 *   predefined_voice_id: "default_sample.wav"
 * });
 * 
 * // Get available voices
 * const voices = await getPredefinedVoices();
 * ```
 */

// =============================================================================
// Type Definitions
// =============================================================================

/**
 * Voice mode options for TTS generation
 */
export type VoiceMode = 'predefined' | 'clone';

/**
 * Supported audio output formats
 */
export type AudioFormat = 'wav' | 'opus' | 'mp3';

/**
 * Request model for the custom /tts endpoint
 */
export interface CustomTTSRequest {
    /** Text to be synthesized */
    text: string;
    /** Voice mode: 'predefined' for a built-in voice, 'clone' for voice cloning using a reference audio */
    voice_mode?: VoiceMode;
    /** Filename of the predefined voice to use (e.g., 'default_sample.wav'). Required if voice_mode is 'predefined' */
    predefined_voice_id?: string | null;
    /** Filename of a user-uploaded reference audio for voice cloning. Required if voice_mode is 'clone' */
    reference_audio_filename?: string | null;
    /** Desired audio output format */
    output_format?: AudioFormat | null;
    /** Whether to automatically split long text into chunks for processing */
    split_text?: boolean | null;
    /** Approximate target character length for text chunks when splitting is enabled (50-500) */
    chunk_size?: number | null;
    /** Overrides default temperature if provided */
    temperature?: number | null;
    /** Overrides default exaggeration if provided */
    exaggeration?: number | null;
    /** Overrides default CFG weight if provided */
    cfg_weight?: number | null;
    /** Overrides default seed if provided */
    seed?: number | null;
    /** Overrides default speed factor if provided */
    speed_factor?: number | null;
    /** Overrides default language if provided */
    language?: string | null;
}

/**
 * Request model for the OpenAI compatible /v1/audio/speech endpoint
 */
export interface OpenAISpeechRequest {
    /** Model to use for speech generation */
    model: string;
    /** Text to synthesize */
    input: string;
    /** Voice to use for synthesis */
    voice: string;
    /** Audio response format */
    response_format?: AudioFormat;
    /** Speed of speech (default: 1.0) */
    speed?: number;
    /** Seed for reproducible generation */
    seed?: number | null;
}

/**
 * Standard error response model for API errors
 */
export interface ErrorResponse {
    /** A human-readable explanation of the error */
    detail: string;
}

/**
 * Response model for status updates, e.g., after saving settings
 */
export interface UpdateStatusResponse {
    /** A message describing the result of the operation */
    message: string;
    /** Indicates if a server restart is recommended or required for changes to take full effect */
    restart_needed?: boolean | null;
}

/**
 * Predefined voice information
 */
export interface PredefinedVoice {
    /** Display name for the voice */
    display_name?: string;
    /** Filename of the voice file */
    filename?: string;
    [key: string]: string | undefined;
}

/**
 * Validation error details
 */
export interface ValidationError {
    /** Location of the error */
    loc: (string | number)[];
    /** Error message */
    msg: string;
    /** Error type */
    type: string;
}

/**
 * HTTP validation error response
 */
export interface HTTPValidationError {
    /** Array of validation errors */
    detail: ValidationError[];
}

/**
 * UI initial data response (contains configuration, file lists, and presets)
 */
export interface UIInitialData {
    /** Configuration data */
    config?: Record<string, unknown>;
    /** Available reference files */
    reference_files?: string[];
    /** Available predefined voices */
    predefined_voices?: PredefinedVoice[];
    /** Additional presets or settings */
    presets?: Record<string, unknown>;
    [key: string]: unknown;
}

/**
 * File upload response
 */
export interface FileUploadResponse {
    /** Success status */
    success?: boolean;
    /** Response message */
    message?: string;
    /** Uploaded file details */
    files?: string[];
    [key: string]: unknown;
}

/**
 * Settings configuration object
 */
export interface SettingsConfig {
    /** TTS model settings */
    model?: {
        temperature?: number;
        exaggeration?: number;
        cfg_weight?: number;
        seed?: number;
        speed_factor?: number;
        language?: string;
    };
    /** Audio processing settings */
    audio?: {
        output_format?: AudioFormat;
        split_text?: boolean;
        chunk_size?: number;
    };
    /** Server settings */
    server?: {
        port?: number;
        host?: string;
    };
    [key: string]: unknown;
}

// =============================================================================
// Configuration
// =============================================================================

/** Base URL for the Chatterbox TTS server */
const TTS_BASE_URL = 'http://192.168.68.65:8004';

// =============================================================================
// Service Functions
// =============================================================================

/**
 * Generate speech using the custom TTS endpoint with advanced parameters
 * Returns audio as a Blob that can be played or downloaded
 */
export async function generateCustomTTS(request: CustomTTSRequest): Promise<Blob> {
    const response = await fetch(`${TTS_BASE_URL}/tts`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(request)
    });

    if (!response.ok) {
        let errorMessage = `TTS generation failed: ${response.status} ${response.statusText}`;

        try {
            const errorData: ErrorResponse = await response.json();
            errorMessage = errorData.detail || errorMessage;
        } catch {
            // If we can't parse the error response, use the default message
        }

        throw new Error(errorMessage);
    }

    // Return the audio blob
    return await response.blob();
}

/**
 * Generate speech using the OpenAI compatible endpoint
 * Returns audio as a Blob that can be played or downloaded
 */
export async function generateOpenAISpeech(request: OpenAISpeechRequest): Promise<Blob> {
    const response = await fetch(`${TTS_BASE_URL}/v1/audio/speech`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(request)
    });

    if (!response.ok) {
        let errorMessage = `OpenAI TTS generation failed: ${response.status} ${response.statusText}`;

        try {
            const errorData: ErrorResponse = await response.json();
            errorMessage = errorData.detail || errorMessage;
        } catch {
            // If we can't parse the error response, use the default message
        }

        throw new Error(errorMessage);
    }

    // Return the audio blob
    return await response.blob();
}

// =============================================================================
// UI Helper Functions
// =============================================================================

/**
 * Get all necessary initial data for the UI to render,
 * including configuration, file lists, and presets
 */
export async function getUIInitialData(): Promise<UIInitialData> {
    const response = await fetch(`${TTS_BASE_URL}/api/ui/initial-data`, {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json'
        }
    });

    if (!response.ok) {
        throw new Error(`Failed to get initial data: ${response.status} ${response.statusText}`);
    }

    return await response.json();
}

/**
 * Get a list of valid reference audio filenames (.wav, .mp3)
 */
export async function getReferenceFiles(): Promise<string[]> {
    const response = await fetch(`${TTS_BASE_URL}/get_reference_files`, {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json'
        }
    });

    if (!response.ok) {
        throw new Error(`Failed to get reference files: ${response.status} ${response.statusText}`);
    }

    return await response.json();
}

/**
 * Get a list of predefined voices with display names and filenames
 */
export async function getPredefinedVoices(): Promise<PredefinedVoice[]> {
    const response = await fetch(`${TTS_BASE_URL}/get_predefined_voices`, {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json'
        }
    });

    if (!response.ok) {
        throw new Error(`Failed to get predefined voices: ${response.status} ${response.statusText}`);
    }

    return await response.json();
}

// =============================================================================
// Configuration Management Functions
// =============================================================================

/**
 * Save partial configuration updates to the config.yaml file
 * Merges the update with the current configuration
 */
export async function saveSettings(settings: SettingsConfig): Promise<UpdateStatusResponse> {
    const response = await fetch(`${TTS_BASE_URL}/save_settings`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(settings)
    });

    if (!response.ok) {
        throw new Error(`Failed to save settings: ${response.status} ${response.statusText}`);
    }

    return await response.json();
}

/**
 * Reset the configuration in config.yaml back to hardcoded defaults
 */
export async function resetSettings(): Promise<UpdateStatusResponse> {
    const response = await fetch(`${TTS_BASE_URL}/reset_settings`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        }
    });

    if (!response.ok) {
        throw new Error(`Failed to reset settings: ${response.status} ${response.statusText}`);
    }

    return await response.json();
}

/**
 * Trigger a server restart
 */
export async function restartServer(): Promise<UpdateStatusResponse> {
    const response = await fetch(`${TTS_BASE_URL}/restart_server`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        }
    });

    if (!response.ok) {
        throw new Error(`Failed to restart server: ${response.status} ${response.statusText}`);
    }

    return await response.json();
}

// =============================================================================
// File Management Functions  
// =============================================================================

/**
 * Upload reference audio files (.wav, .mp3) for voice cloning
 * Validates files and saves them to the configured reference audio path
 */
export async function uploadReferenceAudio(files: File[]): Promise<FileUploadResponse> {
    const formData = new FormData();

    files.forEach((file) => {
        formData.append('files', file);
    });

    const response = await fetch(`${TTS_BASE_URL}/upload_reference`, {
        method: 'POST',
        body: formData
    });

    if (!response.ok) {
        let errorMessage = `Failed to upload reference audio: ${response.status} ${response.statusText}`;

        try {
            const errorData: ErrorResponse = await response.json();
            errorMessage = errorData.detail || errorMessage;
        } catch {
            // If we can't parse the error response, use the default message
        }

        throw new Error(errorMessage);
    }

    return await response.json();
}

/**
 * Upload predefined voice files (.wav, .mp3)
 * Validates files and saves them to the configured predefined voices path
 */
export async function uploadPredefinedVoice(files: File[]): Promise<FileUploadResponse> {
    const formData = new FormData();

    files.forEach((file) => {
        formData.append('files', file);
    });

    const response = await fetch(`${TTS_BASE_URL}/upload_predefined_voice`, {
        method: 'POST',
        body: formData
    });

    if (!response.ok) {
        let errorMessage = `Failed to upload predefined voice: ${response.status} ${response.statusText}`;

        try {
            const errorData: ErrorResponse = await response.json();
            errorMessage = errorData.detail || errorMessage;
        } catch {
            // If we can't parse the error response, use the default message
        }

        throw new Error(errorMessage);
    }

    return await response.json();
}

// =============================================================================
// Utility Functions
// =============================================================================

/**
 * Create an audio URL from a blob that can be used in HTML audio elements
 */
export function createAudioURL(audioBlob: Blob): string {
    return URL.createObjectURL(audioBlob);
}

/**
 * Clean up an audio URL created with createAudioURL
 */
export function revokeAudioURL(url: string): void {
    URL.revokeObjectURL(url);
}

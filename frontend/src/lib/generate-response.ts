export interface GenerateResponse {
    success: boolean;
    message: string;
    data: {
        result: string[];
        audio_data?: string; // Base64 encoded mp3 audio from TTS
    }
}

export interface GenerateLlmResponse {
    success: boolean;
    message: string;
    data: {
        result: string[]
    }
}

export interface GenerateTtsResponse {
    success: boolean;
    message: string;
    data: {
        audio_data: string // Base64 encoded mp3 audio
    }
}
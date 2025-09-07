/**
 * Audio utilities for handling base64 mp3 data
 */

/**
 * Convert base64 mp3 data to an audio blob
 * @param base64Data - Base64 encoded mp3 audio data
 * @returns Blob containing the mp3 audio data
 */
export function base64ToAudioBlob(base64Data: string): Blob {
    // Remove data URL prefix if present (e.g., "data:audio/mp3;base64,")
    const base64Audio = base64Data.replace(/^data:audio\/[^;]+;base64,/, '');

    // Convert base64 to binary
    const binaryString = atob(base64Audio);
    const bytes = new Uint8Array(binaryString.length);

    for (let i = 0; i < binaryString.length; i++) {
        bytes[i] = binaryString.charCodeAt(i);
    }

    // Create blob with mp3 MIME type
    return new Blob([bytes], { type: 'audio/mp3' });
}

/**
 * Convert base64 mp3 data to an object URL for use in audio elements
 * @param base64Data - Base64 encoded mp3 audio data
 * @returns Object URL string
 */
export function base64ToAudioURL(base64Data: string): string {
    const blob = base64ToAudioBlob(base64Data);
    return URL.createObjectURL(blob);
}

/**
 * Clean up an audio URL created with base64ToAudioURL
 * @param url - Object URL to revoke
 */
export function revokeAudioURL(url: string): void {
    URL.revokeObjectURL(url);
}

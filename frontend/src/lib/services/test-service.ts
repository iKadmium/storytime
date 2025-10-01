import type { TestPromptRequest, TestCharacterRequest, TestResponse } from '../models/test.js';
import type { Character } from '../models/character.js';
import type { Prompt } from '../models/prompt.js';
import type { Message } from '../models/chat.js';

const API_BASE = '';

/**
 * Test a prompt with a specific character
 */
export async function testPromptWithCharacter(prompt: Prompt, characterName: string, saveToHistory: boolean = false): Promise<Message> {
	const request: TestPromptRequest = {
		prompt,
		character_name: characterName,
		save_to_chat_history: saveToHistory
	};

	const response = await fetch(`${API_BASE}/api/test/prompt`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(request)
	});

	if (!response.ok) {
		throw new Error(`Failed to test prompt: ${response.statusText}`);
	}

	const data: TestResponse = await response.json();

	if (!data.success || !data.data) {
		throw new Error(data.message || 'Failed to test prompt');
	}

	return data.data;
}

/**
 * Test a character with a specific prompt
 */
export async function testCharacterWithPrompt(character: Character, promptName: string, saveToHistory: boolean = false): Promise<Message> {
	const request: TestCharacterRequest = {
		character,
		prompt_name: promptName,
		save_to_chat_history: saveToHistory
	};

	const response = await fetch(`${API_BASE}/api/test/character`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(request)
	});

	if (!response.ok) {
		throw new Error(`Failed to test character: ${response.statusText}`);
	}

	const data: TestResponse = await response.json();

	if (!data.success || !data.data) {
		throw new Error(data.message || 'Failed to test character');
	}

	return data.data;
}

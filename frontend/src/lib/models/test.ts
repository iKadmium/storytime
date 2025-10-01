import type { Character } from './character';
import type { Prompt } from './prompt';
import type { Message } from './chat';

export interface TestPromptRequest {
	prompt: Prompt;
	character_name: string;
	save_to_chat_history?: boolean;
}

export interface TestCharacterRequest {
	character: Character;
	prompt_name: string;
	save_to_chat_history?: boolean;
}

export interface ApiResponse<T> {
	success: boolean;
	data?: T;
	message: string;
}

export type TestResponse = ApiResponse<Message>;

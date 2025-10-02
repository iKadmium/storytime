/**
 * Chat Service
 *
 * Service for handling chat-related API calls to the backend
 */

import type { Chat, CreateChatRequest, UpdateChatRequest, AddMessageRequest, ChatListItem } from '$lib/models/chat';
import { base64ToAudioURL } from '$lib/utils/audio-utils';
import { toSlug } from '$lib/utils/slug';

const API_BASE_URL = '/api';

export interface ApiResponse<T> {
	success: boolean;
	data?: T;
	message: string;
}

/**
 * Get all chat names
 */
export async function getAllChats(): Promise<string[]> {
	const response = await fetch(`${API_BASE_URL}/chats`);
	if (!response.ok) {
		throw new Error(`Failed to fetch chats: ${response.statusText}`);
	}

	const result: ApiResponse<string[]> = await response.json();
	if (!result.success || !result.data) {
		throw new Error(result.message || 'Failed to fetch chats');
	}

	return result.data;
}

/**
 * Get a specific chat by character name
 */
export async function getChat(character: string, fetch: typeof window.fetch): Promise<Chat> {
	const response = await fetch(`${API_BASE_URL}/chats/${encodeURIComponent(character)}`);
	if (!response.ok) {
		throw new Error(`Failed to fetch chat for ${character}: ${response.statusText}`);
	}

	const result: ApiResponse<Chat> = await response.json();
	if (!result.success || !result.data) {
		throw new Error(result.message || `Failed to fetch chat for ${character}`);
	}

	return result.data;
}

/**
 * Create a new chat
 */
export async function createChat(request: CreateChatRequest): Promise<Chat> {
	const response = await fetch(`${API_BASE_URL}/chats`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(request)
	});

	if (!response.ok) {
		throw new Error(`Failed to create chat: ${response.statusText}`);
	}

	const result: ApiResponse<Chat> = await response.json();
	if (!result.success || !result.data) {
		throw new Error(result.message || 'Failed to create chat');
	}

	return result.data;
}

/**
 * Update an existing chat
 */
export async function updateChat(character: string, request: UpdateChatRequest): Promise<Chat> {
	const response = await fetch(`${API_BASE_URL}/chats/${encodeURIComponent(character)}`, {
		method: 'PUT',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(request)
	});

	if (!response.ok) {
		throw new Error(`Failed to update chat: ${response.statusText}`);
	}

	const result: ApiResponse<Chat> = await response.json();
	if (!result.success || !result.data) {
		throw new Error(result.message || 'Failed to update chat');
	}

	return result.data;
}

/**
 * Delete a chat
 */
export async function deleteChat(character: string): Promise<void> {
	const response = await fetch(`${API_BASE_URL}/chats/${encodeURIComponent(character)}`, {
		method: 'DELETE'
	});

	if (!response.ok) {
		throw new Error(`Failed to delete chat: ${response.statusText}`);
	}

	const result: ApiResponse<void> = await response.json();
	if (!result.success) {
		throw new Error(result.message || 'Failed to delete chat');
	}
}

/**
 * Add a message to a chat
 */
export async function addMessage(character: string, request: AddMessageRequest): Promise<Chat> {
	const response = await fetch(`${API_BASE_URL}/chats/${encodeURIComponent(character)}/messages`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(request)
	});

	if (!response.ok) {
		throw new Error(`Failed to add message: ${response.statusText}`);
	}

	const result: ApiResponse<Chat> = await response.json();
	if (!result.success || !result.data) {
		throw new Error(result.message || 'Failed to add message');
	}

	return result.data;
}

/**
 * Get chat list items with metadata for display in chat list
 */
export async function getChatListItems(): Promise<ChatListItem[]> {
	const chatNames = await getAllChats();
	const chatListItems: ChatListItem[] = [];

	// Fetch each chat to get last message info
	for (const name of chatNames) {
		try {
			const chat = await getChat(name, window.fetch);
			const lastMessage = chat.messages.length > 0 ? chat.messages[chat.messages.length - 1].text.join(' ') : '';
			const unreadCount = chat.messages.filter((message) => !message.read).length;

			chatListItems.push({
				character: chat.character,
				lastMessage: lastMessage || 'No messages yet',
				messageCount: chat.messages.length,
				unreadCount: unreadCount
				// Note: We don't have timestamp data from backend, so we skip lastActivity for now
			});
		} catch (error) {
			console.error(`Failed to fetch chat for ${name}:`, error);
			// Still add the chat to the list even if we can't get details
			chatListItems.push({
				character: name,
				lastMessage: 'Error loading messages',
				messageCount: 0,
				unreadCount: 0
			});
		}
	}

	return chatListItems.sort((a, b) => a.character.localeCompare(b.character));
}

/**
 * Check if audio data is base64 encoded (from test endpoints) or a file path (from chat history)
 * @param audioData - The audio string to check
 * @returns true if it's base64 data, false if it's a file path
 */
export function isBase64Audio(audioData: string): boolean {
	// Check for data URL format
	if (audioData.startsWith('data:audio/')) {
		return true;
	}

	// Check for raw base64 (must be substantial length and only base64 characters)
	if (audioData.length > 100 && audioData.match(/^[A-Za-z0-9+/]+=*$/)) {
		return true;
	}

	// Otherwise assume it's a file path
	return false;
}

/**
 * Convert base64 audio to object URL for test endpoints
 * For chat history, file paths should be used directly
 * @param base64Audio - Base64 encoded mp3 audio
 * @returns Object URL for the audio blob
 */
export function convertTestAudioToUrl(base64Audio: string): string {
	return base64ToAudioURL(base64Audio);
}

/**
 * Mark a specific message as read
 */
export async function markMessageAsRead(character: string, messageIndex: number): Promise<Chat> {
	const slug = toSlug(character);
	const response = await fetch(`${API_BASE_URL}/chats/${encodeURIComponent(slug)}/messages/${messageIndex}/read`, {
		method: 'PUT',
		headers: {
			'Content-Type': 'application/json'
		}
	});

	if (!response.ok) {
		throw new Error(`Failed to mark message as read: ${response.statusText}`);
	}

	const result: ApiResponse<Chat> = await response.json();
	if (!result.success || !result.data) {
		throw new Error(result.message || 'Failed to mark message as read');
	}

	return result.data;
}

/**
 * Mark all messages in a chat as read
 */
export async function markAllMessagesAsRead(character: string): Promise<Chat> {
	const slug = toSlug(character);
	const response = await fetch(`${API_BASE_URL}/chats/${encodeURIComponent(slug)}/read-all`, {
		method: 'PUT',
		headers: {
			'Content-Type': 'application/json'
		}
	});

	if (!response.ok) {
		throw new Error(`Failed to mark all messages as read: ${response.statusText}`);
	}

	const result: ApiResponse<Chat> = await response.json();
	if (!result.success || !result.data) {
		throw new Error(result.message || 'Failed to mark all messages as read');
	}

	return result.data;
}

/**
 * Delete a specific message from a chat
 */
export async function deleteMessage(character: string, messageIndex: number): Promise<Chat> {
	const slug = toSlug(character);
	const response = await fetch(`${API_BASE_URL}/chats/${encodeURIComponent(slug)}/messages/${messageIndex}`, {
		method: 'DELETE',
		headers: {
			'Content-Type': 'application/json'
		}
	});

	if (!response.ok) {
		throw new Error(`Failed to delete message: ${response.statusText}`);
	}

	const result: ApiResponse<Chat> = await response.json();
	if (!result.success || !result.data) {
		throw new Error(result.message || 'Failed to delete message');
	}

	return result.data;
}

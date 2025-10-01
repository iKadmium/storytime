/**
 * Chat-related TypeScript interfaces and types
 * Matches the Rust models from the backend
 */

export interface Message {
	text: string[];
	audio: string[];
	images: string[];
	read?: boolean;
}

export interface Chat {
	character: string;
	messages: Message[];
}

export interface CreateChatRequest {
	character: string;
}

export interface UpdateChatRequest {
	character?: string;
}

export interface AddMessageRequest {
	text?: string[];
	audio?: string[];
	images?: string[];
	read?: boolean;
}

export interface UpdateMessageRequest {
	text?: string[];
	audio?: string[];
	images?: string[];
	read?: boolean;
}

// Helper type for chat list display
export interface ChatListItem {
	character: string;
	lastMessage?: string;
	messageCount: number;
	unreadCount?: number;
	lastActivity?: Date;
}

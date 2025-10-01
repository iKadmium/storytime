export interface Prompt {
	title: string;
	description: string;
	context: string;
	setup: string[];
	create_audio: boolean;
	create_images: boolean;
}

export interface CreatePromptRequest {
	title: string;
	description: string;
	context: string;
	setup: string[];
	create_audio: boolean;
	create_images: boolean;
}

export interface UpdatePromptRequest {
	description?: string;
	context?: string;
	setup?: string[];
	create_audio?: boolean;
	create_images?: boolean;
}

export interface ApiResponse<T> {
	success: boolean;
	data?: T;
	message: string;
}

export type PromptListResponse = ApiResponse<Prompt[]>;
export type PromptResponse = ApiResponse<Prompt>;
export type PromptDeleteResponse = ApiResponse<null>;

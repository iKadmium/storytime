export interface Job {
	characters: string[];
	prompts: string[];
	cadence: string;
	'prompt-override': string | null;
}

export interface CreateJobRequest {
	characters: string[];
	prompts: string[];
	cadence: string;
	'prompt-override': string | null;
}

export interface UpdateJobRequest {
	characters: string[];
	prompts: string[];
	cadence: string;
	'prompt-override': string | null;
}

export interface RunJobRequest {
	job: Job;
	save_to_chat_history: boolean;
}

export interface ApiResponse<T> {
	success: boolean;
	data?: T;
	message: string;
}

export type JobListResponse = ApiResponse<Job[]>;
export type JobResponse = ApiResponse<Job>;
export type JobDeleteResponse = ApiResponse<null>;

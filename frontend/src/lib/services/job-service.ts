import type { Job, CreateJobRequest, UpdateJobRequest, RunJobRequest, JobListResponse, JobResponse, JobDeleteResponse } from '../models/job.js';
import type { Message } from '../models/chat.js';
import { jobSlug } from '../utils/slug.js';

const API_BASE = '';

/**
 * Fetch all jobs
 */
export async function fetchJobs(): Promise<Job[]> {
	const response = await fetch(`${API_BASE}/api/jobs`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json'
		}
	});

	if (!response.ok) {
		throw new Error(`Failed to fetch jobs: ${response.statusText}`);
	}

	const result: JobListResponse = await response.json();

	if (!result.success) {
		throw new Error(result.message || 'Failed to fetch jobs');
	}

	return result.data || [];
}

/**
 * Fetch a specific job by character and prompt
 */
export async function fetchJob(character: string, prompt: string): Promise<Job> {
	const slug = jobSlug(character, prompt);
	const response = await fetch(`${API_BASE}/api/jobs/${slug}`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json'
		}
	});

	if (!response.ok) {
		if (response.status === 404) {
			throw new Error(`Job for '${character}' and '${prompt}' not found`);
		}
		throw new Error(`Failed to fetch job: ${response.statusText}`);
	}

	const result: JobResponse = await response.json();

	if (!result.success) {
		throw new Error(result.message || 'Failed to fetch job');
	}

	if (!result.data) {
		throw new Error('No job data returned');
	}

	return result.data;
}

/**
 * Create a new job
 */
export async function createJob(job: CreateJobRequest): Promise<Job> {
	const response = await fetch(`${API_BASE}/api/jobs`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(job)
	});

	if (!response.ok) {
		throw new Error(`Failed to create job: ${response.statusText}`);
	}

	const result: JobResponse = await response.json();

	if (!result.success) {
		throw new Error(result.message || 'Failed to create job');
	}

	if (!result.data) {
		throw new Error('No job data returned');
	}

	return result.data;
}

/**
 * Update an existing job
 */
export async function updateJob(character: string, prompt: string, updates: UpdateJobRequest): Promise<Job> {
	const slug = jobSlug(character, prompt);
	const response = await fetch(`${API_BASE}/api/jobs/${slug}`, {
		method: 'PUT',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(updates)
	});

	if (!response.ok) {
		if (response.status === 404) {
			throw new Error(`Job for '${character}' and '${prompt}' not found`);
		}
		throw new Error(`Failed to update job: ${response.statusText}`);
	}

	const result: JobResponse = await response.json();

	if (!result.success) {
		throw new Error(result.message || 'Failed to update job');
	}

	if (!result.data) {
		throw new Error('No job data returned');
	}

	return result.data;
}

/**
 * Delete a job
 */
export async function deleteJob(character: string, prompt: string): Promise<void> {
	const slug = jobSlug(character, prompt);
	const response = await fetch(`${API_BASE}/api/jobs/${slug}`, {
		method: 'DELETE',
		headers: {
			'Content-Type': 'application/json'
		}
	});

	if (!response.ok) {
		if (response.status === 404) {
			throw new Error(`Job for '${character}' and '${prompt}' not found`);
		}
		throw new Error(`Failed to delete job: ${response.statusText}`);
	}

	const result: JobDeleteResponse = await response.json();

	if (!result.success) {
		throw new Error(result.message || 'Failed to delete job');
	}
}

/**
 * Execute a job by slug
 */
export async function executeJobBySlug(character: string, prompt: string): Promise<Message> {
	const slug = jobSlug(character, prompt);
	const response = await fetch(`${API_BASE}/api/jobs/${slug}/run`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		}
	});

	if (!response.ok) {
		if (response.status === 404) {
			throw new Error(`Job for '${character}' and '${prompt}' not found`);
		}
		throw new Error(`Failed to execute job: ${response.statusText}`);
	}

	const result = await response.json();

	if (!result.success) {
		throw new Error(result.message || 'Failed to execute job');
	}

	if (!result.data) {
		throw new Error('No message data returned');
	}

	return result.data;
}

/**
 * Execute a job by passing the job object
 */
export async function executeJob(job: Job, save_to_chat_history: boolean = true): Promise<Message> {
	const runJobRequest: RunJobRequest = {
		job,
		save_to_chat_history
	};

	const response = await fetch(`${API_BASE}/api/jobs/run`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(runJobRequest)
	});

	if (!response.ok) {
		throw new Error(`Failed to execute job: ${response.statusText}`);
	}

	const result = await response.json();

	if (!result.success) {
		throw new Error(result.message || 'Failed to execute job');
	}

	if (!result.data) {
		throw new Error('No message data returned');
	}

	return result.data;
}

/**
 * Generate a filename based on character and prompt
 */
export function generateJobFilename(character: string, prompt: string): string {
	const sanitizedCharacter = character.toLowerCase().replace(/\s+/g, '-');
	const sanitizedPrompt = prompt.toLowerCase().replace(/\s+/g, '-');
	return `${sanitizedCharacter}-${sanitizedPrompt}`;
}

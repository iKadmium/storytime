import type {
    Prompt,
    CreatePromptRequest,
    UpdatePromptRequest,
    PromptListResponse,
    PromptResponse,
    PromptDeleteResponse
} from '../models/prompt.js';
import { promptSlug } from '../utils/slug.js';

const API_BASE = '';

/**
 * Fetch all prompts
 */
export async function fetchPrompts(): Promise<Prompt[]> {
    const response = await fetch(`${API_BASE}/api/prompts`, {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json'
        }
    });

    if (!response.ok) {
        throw new Error(`Failed to fetch prompts: ${response.statusText}`);
    }

    const data: PromptListResponse = await response.json();

    if (!data.success) {
        throw new Error(data.message || 'Failed to fetch prompts');
    }

    return data.data || [];
}

/**
 * Fetch a single prompt by title
 */
export async function fetchPrompt(title: string): Promise<Prompt> {
    const slug = promptSlug(title);
    const response = await fetch(`${API_BASE}/api/prompts/${slug}`, {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json'
        }
    });

    if (!response.ok) {
        if (response.status === 404) {
            throw new Error(`Prompt '${title}' not found`);
        }
        throw new Error(`Failed to fetch prompt: ${response.statusText}`);
    }

    const data: PromptResponse = await response.json();

    if (!data.success || !data.data) {
        throw new Error(data.message || 'Failed to fetch prompt');
    }

    return data.data;
}

/**
 * Create a new prompt
 */
export async function createPrompt(prompt: CreatePromptRequest): Promise<Prompt> {
    const response = await fetch(`${API_BASE}/api/prompts`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(prompt)
    });

    if (!response.ok) {
        if (response.status === 409) {
            throw new Error(`Prompt '${prompt.title}' already exists`);
        }
        throw new Error(`Failed to create prompt: ${response.statusText}`);
    }

    const data: PromptResponse = await response.json();

    if (!data.success || !data.data) {
        throw new Error(data.message || 'Failed to create prompt');
    }

    return data.data;
}

/**
 * Update an existing prompt
 */
export async function updatePrompt(title: string, updates: UpdatePromptRequest): Promise<Prompt> {
    const slug = promptSlug(title);
    const response = await fetch(`${API_BASE}/api/prompts/${slug}`, {
        method: 'PUT',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(updates)
    });

    if (!response.ok) {
        if (response.status === 404) {
            throw new Error(`Prompt '${title}' not found`);
        }
        throw new Error(`Failed to update prompt: ${response.statusText}`);
    }

    const data: PromptResponse = await response.json();

    if (!data.success || !data.data) {
        throw new Error(data.message || 'Failed to update prompt');
    }

    return data.data;
}

/**
 * Delete a prompt
 */
export async function deletePrompt(title: string): Promise<void> {
    const slug = promptSlug(title);
    const response = await fetch(`${API_BASE}/api/prompts/${slug}`, {
        method: 'DELETE',
        headers: {
            'Content-Type': 'application/json'
        }
    });

    if (!response.ok) {
        if (response.status === 404) {
            throw new Error(`Prompt '${title}' not found`);
        }
        throw new Error(`Failed to delete prompt: ${response.statusText}`);
    }

    const data: PromptDeleteResponse = await response.json();

    if (!data.success) {
        throw new Error(data.message || 'Failed to delete prompt');
    }
}
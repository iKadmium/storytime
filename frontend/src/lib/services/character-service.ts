import type {
	Character,
	CreateCharacterRequest,
	UpdateCharacterRequest,
	CharacterListResponse,
	CharacterResponse,
	CharacterDeleteResponse
} from '../models/character.js';
import { characterSlug } from '../utils/slug.js';

const API_BASE = '';

/**
 * Fetch all characters
 */
export async function fetchCharacters(): Promise<Character[]> {
	const response = await fetch(`${API_BASE}/api/characters`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json'
		}
	});

	if (!response.ok) {
		throw new Error(`Failed to fetch characters: ${response.statusText}`);
	}

	const data: CharacterListResponse = await response.json();

	if (!data.success) {
		throw new Error(data.message || 'Failed to fetch characters');
	}

	return data.data || [];
}

/**
 * Fetch a single character by name
 */
export async function fetchCharacter(name: string): Promise<Character> {
	const slug = characterSlug(name);
	const response = await fetch(`${API_BASE}/api/characters/${slug}`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json'
		}
	});

	if (!response.ok) {
		if (response.status === 404) {
			throw new Error(`Character '${name}' not found`);
		}
		throw new Error(`Failed to fetch character: ${response.statusText}`);
	}

	const data: CharacterResponse = await response.json();

	if (!data.success || !data.data) {
		throw new Error(data.message || 'Failed to fetch character');
	}

	return data.data;
}

/**
 * Create a new character
 */
export async function createCharacter(character: CreateCharacterRequest): Promise<Character> {
	const response = await fetch(`${API_BASE}/api/characters`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(character)
	});

	if (!response.ok) {
		if (response.status === 409) {
			throw new Error(`Character '${character.name}' already exists`);
		}
		throw new Error(`Failed to create character: ${response.statusText}`);
	}

	const data: CharacterResponse = await response.json();

	if (!data.success || !data.data) {
		throw new Error(data.message || 'Failed to create character');
	}

	return data.data;
}

/**
 * Update an existing character
 */
export async function updateCharacter(name: string, updates: UpdateCharacterRequest): Promise<Character> {
	const slug = characterSlug(name);
	const response = await fetch(`${API_BASE}/api/characters/${slug}`, {
		method: 'PUT',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(updates)
	});

	if (!response.ok) {
		if (response.status === 404) {
			throw new Error(`Character '${name}' not found`);
		}
		throw new Error(`Failed to update character: ${response.statusText}`);
	}

	const data: CharacterResponse = await response.json();

	if (!data.success || !data.data) {
		throw new Error(data.message || 'Failed to update character');
	}

	return data.data;
}

/**
 * Delete a character
 */
export async function deleteCharacter(name: string): Promise<void> {
	const slug = characterSlug(name);
	const response = await fetch(`${API_BASE}/api/characters/${slug}`, {
		method: 'DELETE',
		headers: {
			'Content-Type': 'application/json'
		}
	});

	if (!response.ok) {
		if (response.status === 404) {
			throw new Error(`Character '${name}' not found`);
		}
		throw new Error(`Failed to delete character: ${response.statusText}`);
	}

	const data: CharacterDeleteResponse = await response.json();

	if (!data.success) {
		throw new Error(data.message || 'Failed to delete character');
	}
}

/**
 * Generate a URL-safe slug from a string
 * Converts to lowercase, replaces spaces and special characters with hyphens,
 * removes consecutive hyphens, and trims hyphens from ends
 */
export function toSlug(input: string): string {
	return input
		.toLowerCase()
		.replace(/[^a-zA-Z0-9\s-]/g, '') // Remove special characters except spaces and hyphens
		.replace(/[\s_]+/g, '-') // Replace spaces and underscores with hyphens
		.replace(/-+/g, '-') // Replace multiple consecutive hyphens with single hyphen
		.replace(/^-+|-+$/g, ''); // Trim hyphens from start and end
}

/**
 * Generate a slug for a character based on their name
 */
export function characterSlug(name: string): string {
	return toSlug(name);
}

/**
 * Generate a slug for a prompt based on its title
 */
export function promptSlug(title: string): string {
	return toSlug(title);
}

/**
 * Generate a slug for a job based on character and prompt
 * Format: {character_slug}-{prompt_slug}
 */
export function jobSlug(character: string, prompt: string): string {
	const characterSlugStr = toSlug(character);
	const promptSlugStr = toSlug(prompt);
	return `${characterSlugStr}-${promptSlugStr}`;
}

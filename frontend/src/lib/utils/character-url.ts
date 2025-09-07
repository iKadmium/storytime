/**
 * Utility functions for converting between character names and URL-safe slugs
 */

/**
 * Convert a character name to a URL-safe slug (lowercase, spaces to hyphens)
 */
export function characterToSlug(characterName: string): string {
    return characterName.toLowerCase().replace(/\s+/g, '-');
}

/**
 * Convert a URL slug back to a character name (title case, hyphens to spaces)
 */
export function slugToCharacter(slug: string): string {
    return slug.replace(/-/g, ' ').replace(/\b\w/g, l => l.toUpperCase());
}

/**
 * Find the original character name from a list of chats by matching the slug
 */
export function findCharacterBySlug(slug: string, characters: string[]): string | null {
    const targetSlug = slug.toLowerCase();
    return characters.find(char => characterToSlug(char) === targetSlug) || null;
}
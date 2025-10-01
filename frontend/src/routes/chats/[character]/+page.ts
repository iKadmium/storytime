import type { PageLoad } from './$types';
import { getChat } from '$lib/services/chat-service';
import { error } from '@sveltejs/kit';
import { slugToCharacter } from '$lib/utils/character-url';

export const load: PageLoad = async ({ params, fetch }) => {
	try {
		// Convert URL slug back to original character name format
		const originalCharacterName = slugToCharacter(params.character);

		const chat = await getChat(params.character, fetch);
		return {
			chat,
			character: originalCharacterName,
			urlCharacter: params.character
		};
	} catch (e) {
		console.error('Failed to load chat:', e);
		throw error(404, `Chat with character "${params.character}" not found`);
	}
};

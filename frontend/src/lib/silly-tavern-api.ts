// import type { CsrfTokenResponse } from './models/silly-tavern/csrf-token';
// import type { Chatterbox, Settings, SettingsResponse } from './models/silly-tavern/settings';

// export const pluginId = 'storytime';
// export const apiBaseUrl = '/api';

// export async function getCsrfToken(): Promise<string> {
// 	try {
// 		const response = await fetch(`${apiBaseUrl}/csrf-token`, {
// 			method: 'GET',
// 			headers: {
// 				'Content-Type': 'application/json'
// 			}
// 		});
// 		const data = (await response.json()) as CsrfTokenResponse;
// 		return data.token;
// 	} catch (e) {
// 		console.error('Failed to fetch CSRF token', e);
// 		throw e;
// 	}
// }

// export async function getChatterboxSettings(csrfToken: string): Promise<Chatterbox> {
// 	try {
// 		const response = await fetch(`${apiBaseUrl}/api/settings/get`, {
// 			method: 'POST',
// 			headers: {
// 				'Content-Type': 'application/json',
// 				'X-CSRF-Token': csrfToken || ''
// 			}
// 		});
// 		const data = (await response.json()) as SettingsResponse;
// 		const settings = JSON.parse(data.settings) as Settings;
// 		const chatterboxSettings = settings.extension_settings.tts.Chatterbox;
// 		return chatterboxSettings;
// 	} catch (e) {
// 		console.error('Button click failed', e);
// 		throw e;
// 	}
// }

// export async function getCharacterList(csrfToken: string): Promise<string[]> {
// 	try {
// 		const response = await fetch(`${apiBaseUrl}/api/characters/all`, {
// 			method: 'POST',
// 			headers: {
// 				'Content-Type': 'application/json',
// 				'X-CSRF-Token': csrfToken || ''
// 			}
// 		});
// 		if (!response.ok) {
// 			throw new Error(`Error fetching character list: ${response.statusText}`);
// 		}
// 		const data = (await response.json()) as string[];
// 		return data;
// 	} catch (e) {
// 		console.error('Failed to fetch character list', e);
// 		throw e;
// 	}
// }

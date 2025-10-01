import cronstrue from 'cronstrue';

export function formatCadence(cadence: string): string {
	try {
		const thing = cronstrue.toString(cadence, { use24HourTimeFormat: false });
		return thing;
	} catch (error) {
		return 'Invalid cron expression: ' + (error as Error).message;
	}
}

import cronstrue from "cronstrue";

export function formatCadence(cadence: string): string {
	try {
		const thing = new cronstrue(cadence.trim(), {});
		return thing.toString();
	} catch (error) {
		return 'Invalid cron expression: ' + (error as Error).message;
	}
}

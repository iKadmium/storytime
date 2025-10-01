export function formatCadence(cadence: string): string {
	// Simple cron expression formatter for display
	const parts = cadence.split(' ');
	if (parts.length === 5) {
		const [min, hour, _day, _month, dow] = parts;

		// Format day of week
		let dowDisplay = dow;
		if (dow === '1-5') dowDisplay = 'Weekdays';
		else if (dow === '0,6') dowDisplay = 'Weekends';
		else if (dow === '*') dowDisplay = 'Daily';

		// Format time
		let timeDisplay = '';
		if (hour.includes(',')) {
			const hours = hour.split(',');
			timeDisplay = hours.map((h) => `${h}:${min.padStart(2, '0')}`).join(', ');
		} else if (hour !== '*') {
			timeDisplay = `${hour}:${min.padStart(2, '0')}`;
		}

		return timeDisplay && dowDisplay !== 'Daily' ? `${timeDisplay} on ${dowDisplay}` : cadence;
	}
	return cadence;
}

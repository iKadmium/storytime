<script lang="ts">
	import { toSlug } from '$lib/utils/slug';

	let {
		message,
		characterName,
		messageIndex,
		onMarkAsRead,
		onDelete
	}: {
		message: {
			text: string[];
			audio: string[];
			images: string[];
			timestamp: string;
			read?: boolean;
		};
		characterName: string;
		messageIndex: number;
		onMarkAsRead?: (index: number) => void;
		onDelete?: (index: number) => void;
	} = $props();

	function getInitials(name: string): string {
		return name
			.split(' ')
			.map((word) => word.charAt(0).toUpperCase())
			.join('')
			.substring(0, 2);
	}

	function getAudioUrl(character: string, audioId: string): string {
		return `/audio/${encodeURIComponent(toSlug(character))}/${audioId}.mp3`;
	}

	function formatTimestamp(timestamp: string): string {
		const messageDate = new Date(timestamp);
		const now = new Date();

		// Check if it's today
		const isToday = messageDate.toDateString() === now.toDateString();

		if (isToday) {
			// Show only time for today's messages
			return messageDate.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
		}

		// Check if it's this year
		const isThisYear = messageDate.getFullYear() === now.getFullYear();

		if (isThisYear) {
			// Show month, day and time for this year's messages (no year)
			return messageDate.toLocaleDateString([], {
				month: 'short',
				day: 'numeric',
				hour: '2-digit',
				minute: '2-digit'
			});
		}

		// Show full date and time for older messages
		return messageDate.toLocaleDateString([], {
			year: 'numeric',
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}
</script>

<div class="message character" class:unread={!message.read} data-message-index={messageIndex} id="message-{messageIndex}">
	<div class="message-avatar">
		<span class="avatar-text">{getInitials(characterName)}</span>
	</div>
	<div class="message-content">
		<div class="message-meta">
			<span class="message-timestamp">{formatTimestamp(message.timestamp)}</span>
		</div>
		{#each message.text as messageLine, messageLineIndex (messageLineIndex)}
			<div class="message-text">
				{messageLine}
			</div>
		{/each}

		{#if message.audio.length > 0}
			<div class="message-audio">
				{#each message.audio as audioId, audioIndex (audioIndex)}
					<audio controls>
						<source src={getAudioUrl(characterName, audioId)} type="audio/mpeg" />
						Audio not supported
					</audio>
				{/each}
			</div>
		{/if}

		{#if message.images.length > 0}
			<div class="message-images">
				{#each message.images as imageFile, imageIndex (imageIndex)}
					<img src={imageFile} alt="Message attachment" />
				{/each}
			</div>
		{/if}

		<div class="message-actions">
			{#if !message.read}
				<span class="unread-indicator">New</span>
				{#if onMarkAsRead}
					<button class="mark-read-btn" onclick={() => onMarkAsRead?.(messageIndex)} title="Mark as read"> ✓ </button>
				{/if}
			{:else}
				<span class="read-indicator">Read</span>
			{/if}
			{#if onDelete}
				<button class="delete-btn" onclick={() => onDelete?.(messageIndex)} title="Delete message"> 🗑️ </button>
			{/if}
		</div>
	</div>
</div>

<style>
	.message {
		display: flex;
		align-items: flex-start;
		gap: 12px;
		margin-bottom: 4px;
	}

	.message-avatar {
		flex-shrink: 0;
		width: 36px;
		height: 36px;
		border-radius: 50%;
		background-color: var(--color-primary-100);
		display: flex;
		align-items: center;
		justify-content: center;
		margin-top: 2px;
	}

	.message-avatar .avatar-text {
		font-size: 14px;
		font-weight: 600;
		color: var(--color-primary-700);
	}

	.message-content {
		flex: 1;
		display: flex;
		flex-direction: column;
		max-width: calc(100% - 48px);
		min-width: 200px;
		gap: 8px;
	}

	.message-text {
		padding: 16px 20px;
		border-radius: 18px 18px 18px 4px;
		line-height: 1.5;
		word-wrap: break-word;
		background-color: white;
		color: var(--color-surface-900);
		border: 1px solid var(--color-surface-300);
		box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
	}

	.message-meta {
		margin-top: 6px;
		padding: 0 4px;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.message-timestamp {
		font-size: 12px;
		color: var(--color-surface-500);
		font-weight: 400;
	}

	.message-audio {
		margin-top: 8px;
	}

	.message-images {
		margin-top: 8px;
		display: flex;
		flex-wrap: wrap;
		gap: 8px;
	}

	.message-images img {
		max-width: 200px;
		max-height: 200px;
		border-radius: 8px;
		object-fit: cover;
	}

	.message.unread {
		border-left: 4px solid var(--color-primary-500);
		padding-left: 12px;
		background-color: var(--color-primary-50);
		border-radius: 4px;
	}

	.unread-indicator {
		font-size: 12px;
		color: var(--color-primary-600);
		font-weight: 600;
		background-color: var(--color-primary-100);
		padding: 2px 8px;
		border-radius: 12px;
	}

	.read-indicator {
		font-size: 12px;
		color: var(--color-surface-500);
		font-weight: 500;
	}

	.mark-read-btn {
		padding: 4px 8px;
		background-color: var(--color-primary-600);
		color: white;
		border: none;
		border-radius: 4px;
		font-size: 12px;
		cursor: pointer;
		transition: background-color 0.2s ease;
	}

	.mark-read-btn:hover {
		background-color: var(--color-primary-700);
	}

	.message-actions {
		display: flex;
		align-items: center;
		gap: 8px;
		margin-top: 4px;
	}

	.delete-btn {
		padding: 4px 6px;
		background-color: var(--color-error-600);
		color: white;
		border: none;
		border-radius: 4px;
		font-size: 12px;
		cursor: pointer;
		transition: background-color 0.2s ease;
		opacity: 0.8;
	}

	.delete-btn:hover {
		background-color: var(--color-error-700);
		opacity: 1;
	}

	@media (max-width: 768px) {
		.message-content {
			max-width: 90%;
		}
	}

	/* Dark mode support */
	@media (prefers-color-scheme: dark) {
		.message-text {
			background-color: var(--color-surface-700);
			color: var(--color-surface-100);
			border-color: var(--color-surface-600);
		}

		.message-timestamp {
			color: var(--color-surface-500);
		}

		.message-avatar {
			background-color: var(--color-primary-900);
		}

		.message-avatar .avatar-text {
			color: var(--color-primary-300);
		}

		.message.unread {
			background-color: var(--color-primary-900);
			border-left-color: var(--color-primary-400);
		}

		.unread-indicator {
			background-color: var(--color-primary-800);
			color: var(--color-primary-200);
		}

		.read-indicator {
			color: var(--color-surface-400);
		}

		.mark-read-btn {
			background-color: var(--color-primary-700);
		}

		.mark-read-btn:hover {
			background-color: var(--color-primary-600);
		}

		.delete-btn {
			background-color: var(--color-error-700);
		}

		.delete-btn:hover {
			background-color: var(--color-error-600);
		}
	}
</style>

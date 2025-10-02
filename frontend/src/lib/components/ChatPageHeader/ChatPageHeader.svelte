<script lang="ts">
	let {
		characterName,
		messageCount,
		unreadCount,
		onMarkAllRead
	}: {
		characterName: string;
		messageCount: number;
		unreadCount: number;
		onMarkAllRead?: () => void;
	} = $props();
</script>

<header class="page-header">
	<div class="header-left">
		<a href="/" class="back-btn" title="Back to home" aria-label="Home">
			<svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
				<path d="M15 18L9 12L15 6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
			</svg>
		</a>
		<div class="chat-info">
			<h1>{characterName}</h1>
			<span class="message-count">{messageCount} messages, {unreadCount} unread</span>
		</div>
	</div>

	<div class="chat-actions">
		{#if unreadCount > 0 && onMarkAllRead}
			<button class="mark-all-read-btn" onclick={onMarkAllRead} title="Mark all messages as read"> Read </button>
		{/if}
	</div>
</header>

<style>
	.page-header {
		display: flex;
		align-items: center;
		padding: 16px 20px;
		background-color: white;
		border-bottom: 1px solid var(--color-surface-300);
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.header-left {
		display: flex;
		align-items: center;
		flex: 1;
		gap: 12px;
	}

	.back-btn {
		display: none;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		border-radius: 8px;
		color: var(--color-surface-600);
		transition: all 0.2s ease;
		text-decoration: none;
	}

	.back-btn:hover {
		background-color: var(--color-surface-100);
		color: var(--color-surface-900);
	}

	.chat-info {
		flex: 1;
	}

	.chat-info h1 {
		margin: 0;
		font-size: 20px;
		font-weight: 600;
		color: var(--color-surface-900);
	}

	.message-count {
		font-size: 14px;
		color: var(--color-surface-600);
	}

	.chat-actions {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.mark-all-read-btn {
		padding: 8px 16px;
		background-color: var(--color-primary-600);
		color: white;
		border: none;
		border-radius: 6px;
		font-size: 14px;
		font-weight: 500;
		cursor: pointer;
		transition: background-color 0.2s ease;
		white-space: nowrap;
	}

	.mark-all-read-btn:hover {
		background-color: var(--color-primary-700);
	}

	@media (max-width: 768px) {
		.page-header {
			padding: 12px 16px;
		}

		.back-btn {
			display: flex;
		}
	}

	/* Dark mode support */
	@media (prefers-color-scheme: dark) {
		.page-header {
			background-color: var(--color-surface-800);
			border-bottom-color: var(--color-surface-600);
		}

		.back-btn {
			color: var(--color-surface-400);
		}

		.back-btn:hover {
			background-color: var(--color-surface-700);
			color: var(--color-surface-100);
		}

		.chat-info h1 {
			color: var(--color-surface-100);
		}

		.message-count {
			color: var(--color-surface-400);
		}

		.mark-all-read-btn {
			background-color: var(--color-primary-700);
		}

		.mark-all-read-btn:hover {
			background-color: var(--color-primary-600);
		}
	}
</style>

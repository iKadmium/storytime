<script lang="ts">
	import type { ChatListItem as ChatListItemType } from '$lib/models/chat';
	import ChatListItem from '$lib/components/ChatListItem/ChatListItem.svelte';

	let {
		chats,
		selectedChatId,
		loading = false,
		error = null
	}: {
		chats: ChatListItemType[];
		selectedChatId: string;
		loading?: boolean;
		error?: string | null;
	} = $props();
</script>

<div class="chat-sidebar">
	<header class="chat-header">
		<h1>Chats</h1>
	</header>

	<div class="chat-list-container">
		{#if loading}
			<div class="loading-state">
				<p>Loading chats...</p>
			</div>
		{:else if error}
			<div class="error-state">
				<p>Error: {error}</p>
			</div>
		{:else if chats.length === 0}
			<div class="empty-state">
				<p>No chats yet</p>
				<small>Start a conversation with a character to see chats here</small>
			</div>
		{:else}
			<div class="chat-list">
				{#each chats as chatItem (chatItem.character)}
					<ChatListItem chat={chatItem} isSelected={selectedChatId === chatItem.character} />
				{/each}
			</div>
		{/if}
	</div>
</div>

<style>
	.chat-sidebar {
		width: 320px;
		min-width: 280px;
		max-width: 400px;
		border-right: 1px solid var(--color-surface-300);
		background-color: white;
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	.chat-header {
		padding: 20px 16px 16px;
		border-bottom: 1px solid var(--color-surface-200);
		background-color: var(--color-surface-50);
	}

	.chat-header h1 {
		margin: 0;
		font-size: 24px;
		font-weight: 700;
		color: var(--color-surface-900);
	}

	.chat-list-container {
		flex: 1;
		overflow-y: auto;
		min-height: 0;
	}

	.chat-list {
		display: flex;
		flex-direction: column;
	}

	.loading-state,
	.error-state,
	.empty-state {
		padding: 20px;
		text-align: center;
		color: var(--color-surface-600);
	}

	.error-state {
		color: var(--color-error-600);
	}

	.empty-state small {
		display: block;
		margin-top: 8px;
		color: var(--color-surface-500);
		font-size: 14px;
	}

	@media (max-width: 768px) {
		.chat-sidebar {
			width: 100%;
			max-width: none;
			height: 40vh;
			border-right: none;
			border-bottom: 1px solid var(--color-surface-300);
		}

		.chat-header {
			padding: 16px;
		}

		.chat-header h1 {
			font-size: 20px;
		}
	}

	/* Dark mode support */
	@media (prefers-color-scheme: dark) {
		.chat-sidebar {
			background-color: var(--color-surface-800);
			border-right-color: var(--color-surface-600);
		}

		.chat-header {
			background-color: var(--color-surface-700);
			border-bottom-color: var(--color-surface-600);
		}

		.chat-header h1 {
			color: var(--color-surface-100);
		}

		.loading-state,
		.empty-state {
			color: var(--color-surface-400);
		}

		.empty-state small {
			color: var(--color-surface-500);
		}
	}
</style>

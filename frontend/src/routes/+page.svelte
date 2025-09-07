<script lang="ts">
	import { onMount } from 'svelte';
	import ChatListItem from '$lib/components/ChatListItem/ChatListItem.svelte';
	import type { ChatListItem as ChatListItemType } from '$lib/models/chat';
	import { getChatListItems } from '$lib/services/chat-service';

	let chats: ChatListItemType[] = $state([]);
	let selectedChatId: string | null = $state(null);
	let loading = $state(true);
	let error = $state<string | null>(null);

	onMount(async () => {
		try {
			chats = await getChatListItems();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load chats';
			console.error('Error loading chats:', e);
		} finally {
			loading = false;
		}
	});
</script>

<div class="chat-app">
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
					{#each chats as chat (chat.character)}
						<ChatListItem {chat} isSelected={selectedChatId === chat.character} />
					{/each}
				</div>
			{/if}
		</div>
	</div>

	<div class="chat-main">
		<div class="welcome-screen">
			<h2>Welcome to Storytime</h2>
			<p>Select a chat from the sidebar to start messaging</p>
			<div class="features">
				<div class="feature">
					<h3>Chat with Characters</h3>
					<p>Engage in conversations with various characters from your stories</p>
				</div>
				<div class="feature">
					<h3>Voice & Audio</h3>
					<p>Experience rich conversations with text-to-speech and audio messages</p>
				</div>
				<div class="feature">
					<h3>Visual Stories</h3>
					<p>Share images and create immersive storytelling experiences</p>
				</div>
			</div>
		</div>
	</div>
</div>

<style>
	.chat-app {
		display: flex;
		height: calc(100vh - 60px); /* Account for navbar height */
		max-height: calc(100vh - 60px);
		background-color: var(--color-surface-50);
	}

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

	.chat-main {
		flex: 1;
		display: flex;
		flex-direction: column;
		overflow: hidden;
		background-color: var(--color-surface-100);
	}

	.welcome-screen {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 40px;
		text-align: center;
		background-color: var(--color-surface-50);
	}

	.welcome-screen h2 {
		margin: 0 0 16px 0;
		font-size: 28px;
		font-weight: 600;
		color: var(--color-surface-800);
	}

	.welcome-screen > p {
		margin: 0 0 30px 0;
		font-size: 16px;
		color: var(--color-surface-600);
		max-width: 400px;
	}

	.features {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
		gap: 20px;
		max-width: 700px;
		width: 100%;
	}

	.feature {
		background: white;
		padding: 20px;
		border-radius: 12px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
		border: 1px solid var(--color-surface-200);
	}

	.feature h3 {
		margin: 0 0 8px 0;
		font-size: 16px;
		font-weight: 600;
		color: var(--color-surface-800);
	}

	.feature p {
		margin: 0;
		font-size: 14px;
		color: var(--color-surface-600);
		line-height: 1.4;
	}

	/* Responsive design for mobile */
	@media (max-width: 768px) {
		.chat-app {
			flex-direction: column;
			height: calc(100vh - 60px);
		}

		.chat-sidebar {
			width: 100%;
			max-width: none;
			height: 40vh;
			border-right: none;
			border-bottom: 1px solid var(--color-surface-300);
		}

		.chat-main {
			height: 60vh;
		}

		.chat-header {
			padding: 16px;
		}

		.chat-header h1 {
			font-size: 20px;
		}

		.welcome-screen {
			padding: 20px;
		}

		.welcome-screen h2 {
			font-size: 24px;
		}

		.welcome-screen > p {
			font-size: 14px;
			margin-bottom: 24px;
		}

		.features {
			gap: 16px;
		}

		.feature {
			padding: 16px;
		}

		.feature h3 {
			font-size: 14px;
		}

		.feature p {
			font-size: 13px;
		}
	}

	/* Dark mode support */
	@media (prefers-color-scheme: dark) {
		.chat-app {
			background-color: var(--color-surface-900);
		}

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

		.chat-main {
			background-color: var(--color-surface-800);
		}

		.welcome-screen {
			background-color: var(--color-surface-800);
		}

		.welcome-screen h2 {
			color: var(--color-surface-100);
		}

		.welcome-screen > p {
			color: var(--color-surface-300);
		}

		.feature {
			background-color: var(--color-surface-700);
			border-color: var(--color-surface-600);
		}

		.feature h3 {
			color: var(--color-surface-100);
		}

		.feature p {
			color: var(--color-surface-300);
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

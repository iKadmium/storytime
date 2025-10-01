<script lang="ts">
	import { onMount } from 'svelte';
	import type { PageData } from './$types';
	import type { ChatListItem as ChatListItemType } from '$lib/models/chat';
	import ChatListItem from '$lib/components/ChatListItem/ChatListItem.svelte';
	import { getChatListItems } from '$lib/services/chat-service';
	import { toSlug } from '$lib/utils/slug';

	interface Props {
		data: PageData;
	}

	let { data }: Props = $props();
	let chat = $derived(data.chat);
	let chats: ChatListItemType[] = $state([]);
	let loading = $state(true);
	let error = $state<string | null>(null);

	// Get the current selected chat from URL
	let selectedChatId = $derived(chat.character);

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

	function getInitials(name: string): string {
		return name
			.split(' ')
			.map((word) => word.charAt(0).toUpperCase())
			.join('')
			.substring(0, 2);
	}
</script>

<svelte:head>
	<title>Chat with {chat.character} - Storytime</title>
</svelte:head>

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
					{#each chats as chatItem (chatItem.character)}
						<ChatListItem chat={chatItem} isSelected={selectedChatId === chatItem.character} />
					{/each}
				</div>
			{/if}
		</div>
	</div>

	<div class="chat-page">
		<header class="page-header">
			<div class="chat-info">
				<h1>{chat.character}</h1>
				<span class="message-count">{chat.messages.length} messages</span>
			</div>

			<div class="chat-actions">
				<!-- Placeholder for future actions like settings, etc. -->
			</div>
		</header>

		<main class="chat-messages">
			{#if chat.messages.length === 0}
				<div class="empty-chat">
					<p>No messages yet</p>
					<small>Start the conversation with {chat.character}</small>
				</div>
			{:else}
				<div class="messages-container">
					{#each chat.messages as message, index (index)}
						<div class="message character">
							<div class="message-avatar">
								<span class="avatar-text">{getInitials(chat.character)}</span>
							</div>
							<div class="message-content">
								{#each message.text as messageLine, index (index)}
									<div class="message-text">
										{messageLine}
									</div>
								{/each}

								{#if message.audio.length > 0}
									<div class="message-audio">
										{#each message.audio as audioId, index (index)}
											{@const audioUrl = `/audio/${encodeURIComponent(toSlug(chat.character))}/${audioId}.mp3`}
											<audio controls>
												<source src={audioUrl} type="audio/mpeg" />
												Audio not supported
											</audio>
										{/each}
									</div>
								{/if}

								{#if message.images.length > 0}
									<div class="message-images">
										{#each message.images as imageFile, index (index)}
											<img src={imageFile} alt="Message attachment" />
										{/each}
									</div>
								{/if}
							</div>

							<div class="message-meta">
								<span class="message-sender">{chat.character}</span>
								<!-- Future: Add timestamp when available in backend data -->
							</div>
						</div>
					{/each}
				</div>
			{/if}
		</main>
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

	.chat-page {
		display: flex;
		flex-direction: column;
		flex: 1;
		background-color: var(--color-surface-50);
		overflow: hidden;
	}

	.page-header {
		display: flex;
		align-items: center;
		padding: 16px 20px;
		background-color: white;
		border-bottom: 1px solid var(--color-surface-300);
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
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

	.chat-messages {
		flex: 1;
		overflow-y: auto;
		padding: 20px;
		display: flex;
		flex-direction: column;
	}

	.empty-chat {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		text-align: center;
		color: var(--color-surface-600);
	}

	.empty-chat small {
		margin-top: 8px;
		color: var(--color-surface-500);
	}

	.messages-container {
		display: flex;
		flex-direction: column;
		gap: 16px;
		max-width: 800px;
		margin: 0 auto;
		width: 100%;
	}

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
	}

	.message-sender {
		font-size: 13px;
		color: var(--color-surface-600);
		font-weight: 500;
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

		.chat-page {
			height: 60vh;
		}

		.chat-header {
			padding: 16px;
		}

		.chat-header h1 {
			font-size: 20px;
		}

		.page-header {
			padding: 12px 16px;
		}

		.chat-messages {
			padding: 16px;
		}

		.message-content {
			max-width: 90%;
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

		.loading-state,
		.empty-state {
			color: var(--color-surface-400);
		}

		.empty-state small {
			color: var(--color-surface-500);
		}

		.chat-page {
			background-color: var(--color-surface-900);
		}

		.page-header {
			background-color: var(--color-surface-800);
			border-bottom-color: var(--color-surface-600);
		}

		.chat-info h1 {
			color: var(--color-surface-100);
		}

		.message-count {
			color: var(--color-surface-400);
		}

		.message-text {
			background-color: var(--color-surface-700);
			color: var(--color-surface-100);
			border-color: var(--color-surface-600);
		}

		.message-sender {
			color: var(--color-surface-400);
		}

		.message-avatar {
			background-color: var(--color-primary-900);
		}

		.message-avatar .avatar-text {
			color: var(--color-primary-300);
		}
	}
</style>

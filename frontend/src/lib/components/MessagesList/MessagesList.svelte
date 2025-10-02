<script lang="ts">
	import MessageItem from '$lib/components/MessageItem/MessageItem.svelte';

	let {
		messages,
		characterName,
		onMarkMessageAsRead,
		onDeleteMessage
	}: {
		messages: Array<{
			text: string[];
			audio: string[];
			images: string[];
			timestamp: string;
			read?: boolean;
		}>;
		characterName: string;
		onMarkMessageAsRead?: (index: number) => void;
		onDeleteMessage?: (index: number) => void;
	} = $props();
</script>

<main class="chat-messages">
	{#if messages.length === 0}
		<div class="empty-chat">
			<p>No messages yet</p>
			<small>Start the conversation with {characterName}</small>
		</div>
	{:else}
		<div class="messages-container">
			{#each messages as message, index (index)}
				<MessageItem {message} {characterName} messageIndex={index} onMarkAsRead={onMarkMessageAsRead} onDelete={onDeleteMessage} />
			{/each}
		</div>
	{/if}
</main>

<style>
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

	@media (max-width: 768px) {
		.chat-messages {
			padding: 16px;
		}
	}
</style>

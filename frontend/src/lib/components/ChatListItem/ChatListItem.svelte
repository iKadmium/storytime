<script lang="ts">
	import type { ChatListItem } from '$lib/models/chat';
	import { characterToSlug } from '$lib/utils/character-url';

	interface Props {
		chat: ChatListItem;
		isSelected?: boolean;
	}

	let { chat, isSelected = false }: Props = $props();

	function truncateMessage(message: string, maxLength: number = 60): string {
		if (message.length <= maxLength) return message;
		return message.substring(0, maxLength) + '...';
	}

	function getInitials(name: string): string {
		return name
			.split(' ')
			.map((word) => word.charAt(0).toUpperCase())
			.join('')
			.substring(0, 2);
	}
</script>

<a
	href="/chats/{encodeURIComponent(characterToSlug(chat.character))}"
	class="chat-list-item"
	class:selected={isSelected}
>
	<div class="avatar">
		<span class="avatar-text">{getInitials(chat.character)}</span>
	</div>

	<div class="chat-content">
		<div class="chat-header">
			<h3 class="character-name">{chat.character}</h3>
			{#if chat.messageCount > 0}
				<span class="message-count">{chat.messageCount}</span>
			{/if}
		</div>

		<div class="last-message">
			{truncateMessage(chat.lastMessage || 'No messages yet')}
		</div>

		{#if chat.lastActivity}
			<div class="last-activity">
				{chat.lastActivity.toLocaleString()}
			</div>
		{/if}
	</div>
</a>

<style>
	.chat-list-item {
		display: flex;
		align-items: center;
		padding: 12px 16px;
		border-bottom: 1px solid var(--color-surface-300);
		cursor: pointer;
		transition: background-color 0.2s ease;
		background-color: var(--color-surface-50);
		text-decoration: none;
		color: inherit;
	}

	.chat-list-item:hover {
		background-color: var(--color-surface-100);
	}

	.chat-list-item.selected {
		background-color: var(--color-primary-100);
		border-left: 3px solid var(--color-primary-500);
	}

	.avatar {
		width: 48px;
		height: 48px;
		border-radius: 50%;
		background: linear-gradient(135deg, var(--color-primary-400), var(--color-secondary-400));
		display: flex;
		align-items: center;
		justify-content: center;
		margin-right: 12px;
		flex-shrink: 0;
	}

	.avatar-text {
		color: white;
		font-weight: 600;
		font-size: 16px;
	}

	.chat-content {
		flex: 1;
		min-width: 0; /* Allows text truncation */
	}

	.chat-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 4px;
	}

	.character-name {
		font-size: 16px;
		font-weight: 600;
		color: var(--color-surface-900);
		margin: 0;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.message-count {
		background-color: var(--color-primary-500);
		color: white;
		border-radius: 12px;
		padding: 2px 8px;
		font-size: 12px;
		font-weight: 500;
		min-width: 20px;
		text-align: center;
	}

	.last-message {
		color: var(--color-surface-600);
		font-size: 14px;
		line-height: 1.3;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
		margin-bottom: 2px;
	}

	.last-activity {
		color: var(--color-surface-500);
		font-size: 12px;
	}

	@media (max-width: 768px) {
		.chat-list-item {
			padding: 10px 12px;
		}

		.avatar {
			width: 40px;
			height: 40px;
			margin-right: 10px;
		}

		.avatar-text {
			font-size: 14px;
		}

		.character-name {
			font-size: 14px;
		}

		.last-message {
			font-size: 13px;
		}
	}
</style>

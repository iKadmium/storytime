<script lang="ts">
	import { onMount } from 'svelte';
	import type { PageProps } from './$types';
	import type { ChatListItem as ChatListItemType } from '$lib/models/chat';
	import ChatSidebar from '$lib/components/ChatSidebar/ChatSidebar.svelte';
	import ChatPageHeader from '$lib/components/ChatPageHeader/ChatPageHeader.svelte';
	import MessagesList from '$lib/components/MessagesList/MessagesList.svelte';
	import { getChatListItems, markMessageAsRead, markAllMessagesAsRead, deleteMessage } from '$lib/services/chat-service';

	let props: PageProps = $props();
	let chat = $derived(props.data.chat);
	let chats = $state<ChatListItemType[]>([]);
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

	// Effect to scroll to first unread when loading is complete and chat has messages
	$effect(() => {
		if (!loading && chat.messages.length > 0) {
			// Use a timeout to ensure DOM is fully rendered
			setTimeout(() => {
				scrollToFirstUnread();
			}, 250);
		}
	});

	function scrollToFirstUnread() {
		// Find the first unread message
		const firstUnreadMessage = document.querySelector('.message.unread');
		if (firstUnreadMessage) {
			// Get the chat messages container
			const messagesContainer = document.querySelector('.chat-messages');
			if (messagesContainer) {
				// Get the position of the first unread message relative to the container
				const messageRect = firstUnreadMessage.getBoundingClientRect();
				const containerRect = messagesContainer.getBoundingClientRect();
				const scrollOffset = messageRect.top - containerRect.top + messagesContainer.scrollTop - 80; // 80px padding from top

				messagesContainer.scrollTo({
					top: Math.max(0, scrollOffset), // Ensure we don't scroll to negative values
					behavior: 'smooth'
				});
			} else {
				// Fallback to scrollIntoView if container not found
				firstUnreadMessage.scrollIntoView({
					behavior: 'smooth',
					block: 'start'
				});
			}
		} else {
			// If no unread messages, scroll to the bottom to show latest messages
			const messagesContainer = document.querySelector('.chat-messages');
			if (messagesContainer) {
				messagesContainer.scrollTop = messagesContainer.scrollHeight;
			}
		}
	}

	async function handleMarkMessageAsRead(messageIndex: number) {
		try {
			chat.messages[messageIndex].read = true;
			chat = await markMessageAsRead(chat.character, messageIndex);
			// Re-fetch chats to update the unread count
			chats = await getChatListItems();
		} catch (e) {
			console.error('Error marking message as read:', e);
		}
	}

	async function handleMarkAllAsRead() {
		try {
			chat = await markAllMessagesAsRead(chat.character);
			// Re-fetch chats to update the unread count
			chats = await getChatListItems();
		} catch (e) {
			console.error('Error marking all messages as read:', e);
		}
	}

	async function handleDeleteMessage(messageIndex: number) {
		// Show confirmation dialog
		const confirmed = confirm(
			'Are you sure you want to delete this message? This action cannot be undone and will also delete any associated audio or image files.'
		);

		if (!confirmed) {
			return;
		}

		try {
			chat = await deleteMessage(chat.character, messageIndex);
			// Re-fetch chats to update the message count
			chats = await getChatListItems();
		} catch (e) {
			console.error('Error deleting message:', e);
			alert('Failed to delete message. Please try again.');
		}
	}
</script>

<svelte:head>
	<title>Chat with {chat.character} - Storytime</title>
</svelte:head>

<div class="chat-app">
	<div class="sidebar-container" class:mobile-hidden={chat && chat.character}>
		<ChatSidebar {chats} {selectedChatId} {loading} {error} />
	</div>

	<div class="chat-page">
		<ChatPageHeader
			characterName={chat.character}
			messageCount={chat.messages.length}
			onMarkAllRead={handleMarkAllAsRead}
			unreadCount={chat.messages.filter((m) => !m.read).length}
		/>

		<MessagesList messages={chat.messages} characterName={chat.character} onMarkMessageAsRead={handleMarkMessageAsRead} onDeleteMessage={handleDeleteMessage} />
	</div>
</div>

<style>
	.chat-app {
		display: flex;
		height: calc(100vh - 60px); /* Account for navbar height */
		max-height: calc(100vh - 60px);
		background-color: var(--color-surface-50);
	}

	.chat-page {
		display: flex;
		flex-direction: column;
		flex: 1;
		background-color: var(--color-surface-50);
		overflow: hidden;
	}

	.sidebar-container {
		display: block;
	}

	@media (max-width: 768px) {
		.chat-app {
			flex-direction: column;
			height: calc(100vh - 60px);
		}

		.chat-page {
			height: 100vh; /* Take full height when sidebar is hidden */
		}

		.sidebar-container.mobile-hidden {
			display: none;
		}
	}

	/* Dark mode support */
	@media (prefers-color-scheme: dark) {
		.chat-app {
			background-color: var(--color-surface-900);
		}

		.chat-page {
			background-color: var(--color-surface-900);
		}
	}
</style>

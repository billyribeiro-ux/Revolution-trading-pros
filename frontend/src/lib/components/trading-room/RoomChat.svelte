<!--
	RoomChat Component
	===============================================================================
	Apple ICT 11+ Principal Engineer Implementation

	Flexible chat integration for trading rooms:
	- Discord widget embed support
	- Custom WebSocket chat (real-time messages)
	- Zoom/GoToWebinar integration placeholder
	- Mobile-responsive collapsible panel

	@version 1.0.0 - January 2026
	@author Revolution Trading Pros
-->
<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { browser } from '$app/environment';
	import RtpIcon from '$lib/components/icons/RtpIcon.svelte';
	import { authStore } from '$lib/stores/auth.svelte';

	// ===============================================================================
	// PROPS
	// ===============================================================================

	type ChatProvider = 'discord' | 'websocket' | 'zoom' | 'external';

	interface Props {
		roomSlug: string;
		roomName?: string;
		provider?: ChatProvider;
		discordServerId?: string;
		discordChannelId?: string;
		externalChatUrl?: string;
		collapsed?: boolean;
		showHeader?: boolean;
	}

	let {
		roomSlug,
		roomName = 'Trading Room',
		provider = 'websocket',
		discordServerId = '',
		discordChannelId = '',
		externalChatUrl = '',
		collapsed = $bindable(false),
		showHeader = true
	}: Props = $props();

	// ===============================================================================
	// STATE
	// ===============================================================================

	interface ChatMessage {
		id: string;
		userId: string;
		userName: string;
		userAvatar?: string;
		content: string;
		timestamp: Date;
		isOwn: boolean;
	}

	let messages = $state<ChatMessage[]>([]);
	let newMessage = $state('');
	let isConnected = $state(false);
	let isConnecting = $state(true);
	let error = $state<string | null>(null);
	let chatContainer = $state<HTMLElement | null>(null);
	let ws: WebSocket | null = null;

	// User info
	const user = $derived(authStore.user);
	const userId = $derived(user?.id || '');
	const userName = $derived(user?.name || user?.email?.split('@')[0] || 'Guest');

	// ===============================================================================
	// WEBSOCKET CHAT
	// ===============================================================================

	function connectWebSocket(): void {
		if (!browser || provider !== 'websocket') return;

		try {
			const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
			const wsUrl = `${protocol}//${window.location.host}/api/realtime/ws?rooms=${roomSlug}`;

			ws = new WebSocket(wsUrl);

			ws.onopen = () => {
				console.log(`[RoomChat] Connected to ${roomSlug}`);
				isConnected = true;
				isConnecting = false;
				error = null;
			};

			ws.onmessage = (event) => {
				try {
					const data = JSON.parse(event.data);

					if (data.type === 'ChatMessage' && data.payload) {
						const msg: ChatMessage = {
							id: data.payload.id || crypto.randomUUID(),
							userId: data.payload.user_id,
							userName: data.payload.user_name || 'Anonymous',
							userAvatar: data.payload.user_avatar,
							content: data.payload.content,
							timestamp: new Date(data.payload.timestamp || Date.now()),
							isOwn: data.payload.user_id === userId
						};
						messages = [...messages, msg];
						scrollToBottom();
					}
				} catch (err) {
					console.error('[RoomChat] Error parsing message:', err);
				}
			};

			ws.onclose = () => {
				console.log(`[RoomChat] Disconnected from ${roomSlug}`);
				isConnected = false;
				// Attempt to reconnect after 5 seconds
				setTimeout(connectWebSocket, 5000);
			};

			ws.onerror = () => {
				error = 'Connection error. Retrying...';
				isConnecting = false;
				isConnected = false;
			};
		} catch (err) {
			console.error('[RoomChat] Error connecting:', err);
			error = 'Failed to connect to chat';
			isConnecting = false;
		}
	}

	function sendMessage(): void {
		if (!newMessage.trim() || !ws || ws.readyState !== WebSocket.OPEN) return;

		const message = {
			action: 'ChatMessage',
			data: {
				room: roomSlug,
				user_id: userId,
				user_name: userName,
				content: newMessage.trim()
			}
		};

		ws.send(JSON.stringify(message));
		newMessage = '';
	}

	function handleKeydown(e: KeyboardEvent): void {
		if (e.key === 'Enter' && !e.shiftKey) {
			e.preventDefault();
			sendMessage();
		}
	}

	function scrollToBottom(): void {
		if (chatContainer) {
			const container = chatContainer;
			requestAnimationFrame(() => {
				if (container) {
					container.scrollTop = container.scrollHeight;
				}
			});
		}
	}

	function formatTime(date: Date): string {
		return date.toLocaleTimeString('en-US', {
			hour: 'numeric',
			minute: '2-digit'
		});
	}

	function toggleCollapse(): void {
		collapsed = !collapsed;
	}

	// ===============================================================================
	// DISCORD EMBED URL
	// ===============================================================================

	const discordWidgetUrl = $derived.by(() => {
		if (provider !== 'discord' || !discordServerId) return '';
		const params = new URLSearchParams({
			id: discordServerId,
			theme: 'light'
		});
		if (discordChannelId) {
			params.set('channel', discordChannelId);
		}
		return `https://discord.com/widget?${params.toString()}`;
	});

	// ===============================================================================
	// LIFECYCLE
	// ===============================================================================

	onMount(() => {
		if (provider === 'websocket') {
			connectWebSocket();
		} else {
			isConnecting = false;
			isConnected = true;
		}
	});

	onDestroy(() => {
		if (ws) {
			ws.close();
			ws = null;
		}
	});
</script>

<div class="room-chat" class:room-chat--collapsed={collapsed}>
	{#if showHeader}
		<header class="chat-header">
			<div class="chat-header-content">
				<RtpIcon name="message-circle" size={18} />
				<span class="chat-title">{roomName} Chat</span>
				{#if isConnected}
					<span class="status-dot status-dot--online" title="Connected"></span>
				{:else}
					<span class="status-dot status-dot--offline" title="Disconnected"></span>
				{/if}
			</div>
			<button
				class="collapse-btn"
				onclick={toggleCollapse}
				aria-label={collapsed ? 'Expand chat' : 'Collapse chat'}
			>
				<RtpIcon name={collapsed ? 'chevron-up' : 'chevron-down'} size={16} />
			</button>
		</header>
	{/if}

	{#if !collapsed}
		<div class="chat-body">
			{#if provider === 'discord'}
				<!-- Discord Widget Embed -->
				{#if discordWidgetUrl}
					<iframe
						src={discordWidgetUrl}
						width="100%"
						height="100%"
						sandbox="allow-popups allow-popups-to-escape-sandbox allow-same-origin allow-scripts"
						title="Discord Chat"
						class="discord-widget"
					></iframe>
				{:else}
					<div class="chat-placeholder">
						<RtpIcon name="message-circle" size={40} />
						<p>Discord chat not configured</p>
					</div>
				{/if}
			{:else if provider === 'external'}
				<!-- External Chat Link -->
				<div class="chat-external">
					<RtpIcon name="external-link" size={40} />
					<p>Chat is available in an external window</p>
					{#if externalChatUrl}
						<a
							href={externalChatUrl}
							target="_blank"
							rel="noopener noreferrer"
							class="btn btn-primary"
						>
							Open Chat
							<RtpIcon name="external-link" size={14} />
						</a>
					{/if}
				</div>
			{:else if provider === 'zoom'}
				<!-- Zoom/GoToWebinar Chat Placeholder -->
				<div class="chat-placeholder">
					<RtpIcon name="video" size={40} />
					<p>Chat is available in the live webinar window</p>
				</div>
			{:else}
				<!-- WebSocket Chat -->
				{#if isConnecting}
					<div class="chat-loading">
						<div class="loading-spinner"></div>
						<p>Connecting to chat...</p>
					</div>
				{:else if error}
					<div class="chat-error">
						<RtpIcon name="alert-circle" size={24} />
						<p>{error}</p>
						<button class="btn btn-retry" onclick={connectWebSocket}>Retry</button>
					</div>
				{:else}
					<div class="chat-messages" bind:this={chatContainer}>
						{#if messages.length === 0}
							<div class="chat-empty">
								<p>No messages yet. Be the first to say hello!</p>
							</div>
						{:else}
							{#each messages as message (message.id)}
								<div class="chat-message" class:chat-message--own={message.isOwn}>
									<div class="message-header">
										<span class="message-author">{message.userName}</span>
										<span class="message-time">{formatTime(message.timestamp)}</span>
									</div>
									<div class="message-content">{message.content}</div>
								</div>
							{/each}
						{/if}
					</div>

					<div class="chat-input-container">
						<textarea
							bind:value={newMessage}
							onkeydown={handleKeydown}
							placeholder="Type a message..."
							rows="1"
							class="chat-input"
							disabled={!isConnected}
						></textarea>
						<button
							class="send-btn"
							onclick={sendMessage}
							disabled={!newMessage.trim() || !isConnected}
							aria-label="Send message"
						>
							<RtpIcon name="send" size={18} />
						</button>
					</div>
				{/if}
			{/if}
		</div>
	{/if}
</div>

<style>
	/* ===============================================================================
	 * ROOM CHAT - 2026 Mobile-First Design
	 * =============================================================================== */

	.room-chat {
		display: flex;
		flex-direction: column;
		background: #fff;
		border: 1px solid #e2e8f0;
		border-radius: 12px;
		overflow: hidden;
		font-family: 'Montserrat', sans-serif;
		height: 400px;
		max-height: 60dvh;
	}

	.room-chat--collapsed {
		height: auto;
		max-height: none;
	}

	/* Header */
	.chat-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 12px 16px;
		background: linear-gradient(135deg, #143e59 0%, #1a5276 100%);
		color: #fff;
		flex-shrink: 0;
	}

	.chat-header-content {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.chat-title {
		font-size: 14px;
		font-weight: 600;
	}

	.status-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		margin-left: 4px;
	}

	.status-dot--online {
		background: #22c55e;
		box-shadow: 0 0 0 2px rgba(34, 197, 94, 0.3);
	}

	.status-dot--offline {
		background: #94a3b8;
	}

	.collapse-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		background: rgba(255, 255, 255, 0.1);
		border: none;
		border-radius: 6px;
		color: #fff;
		cursor: pointer;
		transition: background 0.2s;
		-webkit-tap-highlight-color: transparent;
	}

	.collapse-btn:hover {
		background: rgba(255, 255, 255, 0.2);
	}

	/* Body */
	.chat-body {
		flex: 1;
		display: flex;
		flex-direction: column;
		min-height: 0;
		overflow: hidden;
	}

	/* Discord Widget */
	.discord-widget {
		border: none;
		flex: 1;
	}

	/* Placeholder states */
	.chat-placeholder,
	.chat-external,
	.chat-loading,
	.chat-error,
	.chat-empty {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 30px;
		text-align: center;
		color: #64748b;
		gap: 12px;
	}

	.chat-placeholder p,
	.chat-external p,
	.chat-error p {
		font-size: 14px;
		margin: 0;
	}

	.chat-empty p {
		font-size: 13px;
		margin: 0;
	}

	.loading-spinner {
		width: 32px;
		height: 32px;
		border: 3px solid #e2e8f0;
		border-top-color: #143e59;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Messages */
	.chat-messages {
		flex: 1;
		overflow-y: auto;
		padding: 12px 16px;
		display: flex;
		flex-direction: column;
		gap: 12px;
		-webkit-overflow-scrolling: touch;
	}

	.chat-message {
		max-width: 85%;
		padding: 10px 14px;
		background: #f1f5f9;
		border-radius: 12px;
		border-bottom-left-radius: 4px;
	}

	.chat-message--own {
		align-self: flex-end;
		background: #143e59;
		color: #fff;
		border-bottom-left-radius: 12px;
		border-bottom-right-radius: 4px;
	}

	.message-header {
		display: flex;
		align-items: center;
		gap: 8px;
		margin-bottom: 4px;
	}

	.message-author {
		font-size: 12px;
		font-weight: 600;
		color: #334155;
	}

	.chat-message--own .message-author {
		color: rgba(255, 255, 255, 0.9);
	}

	.message-time {
		font-size: 10px;
		color: #94a3b8;
	}

	.chat-message--own .message-time {
		color: rgba(255, 255, 255, 0.6);
	}

	.message-content {
		font-size: 13px;
		line-height: 1.5;
		word-wrap: break-word;
	}

	/* Input */
	.chat-input-container {
		display: flex;
		align-items: flex-end;
		gap: 8px;
		padding: 12px 16px;
		border-top: 1px solid #e2e8f0;
		background: #f8fafc;
	}

	.chat-input {
		flex: 1;
		padding: 10px 14px;
		border: 1px solid #e2e8f0;
		border-radius: 20px;
		font-size: 13px;
		font-family: inherit;
		resize: none;
		min-height: 40px;
		max-height: 100px;
		line-height: 1.4;
	}

	.chat-input:focus {
		outline: none;
		border-color: #143e59;
		box-shadow: 0 0 0 3px rgba(20, 62, 89, 0.1);
	}

	.chat-input:disabled {
		background: #f1f5f9;
		color: #94a3b8;
	}

	.send-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		background: #143e59;
		border: none;
		border-radius: 50%;
		color: #fff;
		cursor: pointer;
		transition: all 0.2s;
		flex-shrink: 0;
		-webkit-tap-highlight-color: transparent;
	}

	.send-btn:hover:not(:disabled) {
		background: #1a5276;
		transform: scale(1.05);
	}

	.send-btn:disabled {
		background: #94a3b8;
		cursor: not-allowed;
	}

	/* Buttons */
	.btn {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 10px 20px;
		font-size: 13px;
		font-weight: 600;
		text-decoration: none;
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
		-webkit-tap-highlight-color: transparent;
	}

	.btn-primary {
		background: #143e59;
		color: #fff;
	}

	.btn-primary:hover {
		background: #1a5276;
	}

	.btn-retry {
		background: #f1f5f9;
		color: #334155;
	}

	.btn-retry:hover {
		background: #e2e8f0;
	}

	/* ===============================================================================
	 * RESPONSIVE
	 * =============================================================================== */

	@media (min-width: 640px) {
		.room-chat {
			height: 450px;
		}

		.chat-header {
			padding: 14px 20px;
		}

		.chat-title {
			font-size: 15px;
		}

		.chat-messages {
			padding: 16px 20px;
		}

		.chat-input-container {
			padding: 14px 20px;
		}
	}

	@media (min-width: 1024px) {
		.room-chat {
			height: 500px;
			max-height: none;
		}
	}

	/* Reduced motion */
	@media (prefers-reduced-motion: reduce) {
		.loading-spinner {
			animation: none;
		}

		.send-btn,
		.collapse-btn,
		.btn {
			transition: none;
		}
	}
</style>

<script lang="ts">
	/**
	 * TradingRoomShell - Layout shell for live trading rooms
	 * Optimized for real-time video, chat, and alerts
	 * 
	 * @version 2.0.0
	 * @author Revolution Trading Pros
	 */
	import { page } from '$app/stores';
	import { user, isAuthenticated } from '$lib/stores/auth';
	import {
		IconArrowLeft,
		IconMaximize,
		IconMinimize,
		IconVolume,
		IconVolumeOff,
		IconUsers,
		IconMail,
		IconBell,
		IconSettings,
		IconChevronDown
	} from '@tabler/icons-svelte';

	export let roomName = 'Trading Room';
	export let roomType: 'day-trading' | 'swing-trading' | 'small-accounts' = 'day-trading';
	export let viewerCount = 0;
	export let isLive = false;

	let isChatOpen = true;
	let isAlertsOpen = true;
	let isFullscreen = false;
	let isMuted = false;
	let sidePanelTab: 'chat' | 'alerts' = 'chat';

	function toggleFullscreen() {
		if (!document.fullscreenElement) {
			document.documentElement.requestFullscreen();
			isFullscreen = true;
		} else {
			document.exitFullscreen();
			isFullscreen = false;
		}
	}

	function toggleMute() {
		isMuted = !isMuted;
	}

	function toggleSidePanel() {
		isChatOpen = !isChatOpen;
	}

	const roomTypeLabels = {
		'day-trading': 'Day Trading',
		'swing-trading': 'Swing Trading',
		'small-accounts': 'Small Accounts'
	};
</script>

<div class="trading-room-shell" class:fullscreen={isFullscreen}>
	<!-- Top Bar -->
	<header class="room-header">
		<div class="header-left">
			<a href="/live-trading-rooms" class="back-btn">
				<IconArrowLeft size={20} />
			</a>
			<div class="room-info">
				<h1 class="room-name">{roomName}</h1>
				<div class="room-meta">
					<span class="room-type">{roomTypeLabels[roomType]}</span>
					{#if isLive}
						<span class="live-badge">
							<span class="live-dot"></span>
							LIVE
						</span>
					{/if}
					<span class="viewer-count">
						<IconUsers size={14} />
						{viewerCount.toLocaleString()}
					</span>
				</div>
			</div>
		</div>

		<div class="header-right">
			<button class="control-btn" on:click={toggleMute} title={isMuted ? 'Unmute' : 'Mute'}>
				{#if isMuted}
					<IconVolumeOff size={20} />
				{:else}
					<IconVolume size={20} />
				{/if}
			</button>
			<button class="control-btn" on:click={toggleFullscreen} title={isFullscreen ? 'Exit Fullscreen' : 'Fullscreen'}>
				{#if isFullscreen}
					<IconMinimize size={20} />
				{:else}
					<IconMaximize size={20} />
				{/if}
			</button>
			<button class="control-btn desktop-only" on:click={toggleSidePanel} title="Toggle Panel">
				<IconMail size={20} />
			</button>
		</div>
	</header>

	<!-- Main Content -->
	<div class="room-content">
		<!-- Video Area -->
		<main class="video-area">
			<slot name="video">
				<div class="video-placeholder">
					<p>Video player will be rendered here</p>
				</div>
			</slot>
		</main>

		<!-- Side Panel -->
		{#if isChatOpen}
			<aside class="side-panel">
				<!-- Panel Tabs -->
				<div class="panel-tabs">
					<button 
						class="panel-tab" 
						class:active={sidePanelTab === 'chat'}
						on:click={() => sidePanelTab = 'chat'}
					>
						<IconMail size={16} />
						Chat
					</button>
					<button 
						class="panel-tab" 
						class:active={sidePanelTab === 'alerts'}
						on:click={() => sidePanelTab = 'alerts'}
					>
						<IconBell size={16} />
						Alerts
					</button>
				</div>

				<!-- Panel Content -->
				<div class="panel-content">
					{#if sidePanelTab === 'chat'}
						<slot name="chat">
							<div class="panel-placeholder">
								<IconMail size={32} />
								<p>Chat will be rendered here</p>
							</div>
						</slot>
					{:else}
						<slot name="alerts">
							<div class="panel-placeholder">
								<IconBell size={32} />
								<p>Alerts will be rendered here</p>
							</div>
						</slot>
					{/if}
				</div>
			</aside>
		{/if}
	</div>

	<!-- Mobile Bottom Bar -->
	<div class="mobile-bottom-bar">
		<button 
			class="mobile-tab" 
			class:active={sidePanelTab === 'chat'}
			on:click={() => { sidePanelTab = 'chat'; isChatOpen = true; }}
		>
			<IconMail size={20} />
			<span>Chat</span>
		</button>
		<button 
			class="mobile-tab" 
			class:active={sidePanelTab === 'alerts'}
			on:click={() => { sidePanelTab = 'alerts'; isChatOpen = true; }}
		>
			<IconBell size={20} />
			<span>Alerts</span>
		</button>
	</div>
</div>

<style>
	.trading-room-shell {
		display: flex;
		flex-direction: column;
		height: 100vh;
		background: var(--color-rtp-bg, #0a101c);
		color: var(--color-rtp-text, #e2e8f0);
	}

	.trading-room-shell.fullscreen {
		position: fixed;
		inset: 0;
		z-index: var(--z-max, 1000);
	}

	/* Header */
	.room-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.75rem 1rem;
		background: var(--color-rtp-surface, #1e293b);
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
		flex-shrink: 0;
	}

	.header-left {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.back-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		background: rgba(99, 102, 241, 0.1);
		border-radius: var(--radius-md, 0.5rem);
		color: var(--color-rtp-muted, #94a3b8);
		transition: all 0.2s;
	}

	.back-btn:hover {
		background: rgba(99, 102, 241, 0.2);
		color: var(--color-rtp-text, #f1f5f9);
	}

	.room-info {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.room-name {
		font-size: 1.125rem;
		font-weight: 700;
		margin: 0;
		color: var(--color-rtp-text, #f1f5f9);
	}

	.room-meta {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		font-size: 0.8125rem;
	}

	.room-type {
		color: var(--color-rtp-muted, #64748b);
	}

	.live-badge {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.125rem 0.5rem;
		background: rgba(239, 68, 68, 0.2);
		border-radius: var(--radius-full, 9999px);
		color: #f87171;
		font-weight: 600;
		font-size: 0.6875rem;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.live-dot {
		width: 6px;
		height: 6px;
		background: #ef4444;
		border-radius: 50%;
		animation: pulse 2s infinite;
	}

	@keyframes pulse {
		0%, 100% { opacity: 1; }
		50% { opacity: 0.5; }
	}

	.viewer-count {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		color: var(--color-rtp-muted, #64748b);
	}

	.header-right {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.control-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		background: rgba(99, 102, 241, 0.1);
		border: none;
		border-radius: var(--radius-md, 0.5rem);
		color: var(--color-rtp-muted, #94a3b8);
		cursor: pointer;
		transition: all 0.2s;
	}

	.control-btn:hover {
		background: rgba(99, 102, 241, 0.2);
		color: var(--color-rtp-text, #f1f5f9);
	}

	/* Main Content */
	.room-content {
		display: flex;
		flex: 1;
		overflow: hidden;
	}

	.video-area {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #000;
		min-height: 0;
	}

	.video-placeholder {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 100%;
		height: 100%;
		color: var(--color-rtp-muted, #64748b);
	}

	/* Side Panel */
	.side-panel {
		width: 360px;
		display: flex;
		flex-direction: column;
		background: var(--color-rtp-surface, #1e293b);
		border-left: 1px solid rgba(99, 102, 241, 0.1);
	}

	.panel-tabs {
		display: flex;
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
	}

	.panel-tab {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.875rem;
		background: none;
		border: none;
		color: var(--color-rtp-muted, #64748b);
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.panel-tab:hover {
		color: var(--color-rtp-text, #e2e8f0);
		background: rgba(99, 102, 241, 0.05);
	}

	.panel-tab.active {
		color: var(--color-rtp-primary, #818cf8);
		background: rgba(99, 102, 241, 0.1);
		border-bottom: 2px solid var(--color-rtp-primary, #6366f1);
	}

	.panel-content {
		flex: 1;
		overflow: hidden;
	}

	.panel-placeholder {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 100%;
		gap: 1rem;
		color: var(--color-rtp-muted, #475569);
	}

	/* Mobile Bottom Bar */
	.mobile-bottom-bar {
		display: none;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
		background: var(--color-rtp-surface, #1e293b);
	}

	.mobile-tab {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.25rem;
		padding: 0.75rem;
		background: none;
		border: none;
		color: var(--color-rtp-muted, #64748b);
		font-size: 0.75rem;
		cursor: pointer;
	}

	.mobile-tab.active {
		color: var(--color-rtp-primary, #818cf8);
	}

	.desktop-only {
		display: flex;
	}

	/* Responsive */
	@media (max-width: 1024px) {
		.side-panel {
			position: fixed;
			right: 0;
			top: 0;
			bottom: 0;
			z-index: var(--z-modal, 500);
			width: 100%;
			max-width: 400px;
		}

		.mobile-bottom-bar {
			display: flex;
		}

		.desktop-only {
			display: none;
		}

		.room-name {
			font-size: 1rem;
		}
	}

	@media (max-width: 640px) {
		.room-header {
			padding: 0.5rem;
		}

		.room-name {
			font-size: 0.9375rem;
		}

		.room-meta {
			font-size: 0.75rem;
		}

		.control-btn {
			width: 36px;
			height: 36px;
		}
	}
</style>

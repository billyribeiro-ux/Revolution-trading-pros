<script lang="ts">
	/**
	 * TradingRoomShell - Full-screen layout for live trading rooms
	 *
	 * Provides a full-screen, immersive layout for live trading room experiences
	 * with support for video streams, chat panels, alerts, and trading tools.
	 *
	 * @version 1.0.0
	 * @author Revolution Trading Pros
	 */

	import { browser } from '$app/environment';
	import { user } from '$lib/stores/auth.svelte';
	import IconSettings from '@tabler/icons-svelte/icons/settings';
	import IconMaximize from '@tabler/icons-svelte/icons/maximize';
	import IconMinimize from '@tabler/icons-svelte/icons/minimize';
	import IconLayoutSidebar from '@tabler/icons-svelte/icons/layout-sidebar';
	import IconLayoutSidebarRight from '@tabler/icons-svelte/icons/layout-sidebar-right';
	import type { Snippet } from 'svelte';

	interface Props {
		roomName?: string;
		roomIcon?: string;
		showSidebar?: boolean;
		showRightPanel?: boolean;
		header?: Snippet;
		sidebar?: Snippet;
		rightPanel?: Snippet;
		children: Snippet;
	}

	let {
		roomName = 'Live Trading Room',
		roomIcon = '📊',
		showSidebar = true,
		showRightPanel = false,
		header,
		sidebar,
		rightPanel,
		children
	}: Props = $props();

	let sidebarOpen = $state(false);
	let rightPanelOpen = $state(false);

	// Sync with prop changes
	$effect(() => {
		sidebarOpen = showSidebar;
	});

	$effect(() => {
		rightPanelOpen = showRightPanel;
	});
	let isFullscreen = $state(false);

	function toggleSidebar() {
		sidebarOpen = !sidebarOpen;
	}

	function toggleRightPanel() {
		rightPanelOpen = !rightPanelOpen;
	}

	// SSR-safe: Guard document access with browser check
	async function toggleFullscreen() {
		if (!browser) return;
		if (!document.fullscreenElement) {
			await document.documentElement.requestFullscreen();
			isFullscreen = true;
		} else {
			await document.exitFullscreen();
			isFullscreen = false;
		}
	}

	// SSR-safe: Guard document access with browser check
	function handleFullscreenChange() {
		if (!browser) return;
		isFullscreen = !!document.fullscreenElement;
	}
</script>

<svelte:document onfullscreenchange={handleFullscreenChange} />

<div class="trading-room-shell" class:fullscreen={isFullscreen}>
	<!-- Header Bar -->
	<header class="shell-header">
		<div class="header-left">
			<button class="icon-btn" onclick={toggleSidebar} title="Toggle sidebar">
				<IconLayoutSidebar size={20} />
			</button>
			<div class="room-info">
				<span class="room-icon">{roomIcon}</span>
				<span class="room-name">{roomName}</span>
			</div>
		</div>

		{#if header}
			<div class="header-center">
				{@render header()}
			</div>
		{/if}

		<div class="header-right">
			{#if rightPanel}
				<button class="icon-btn" onclick={toggleRightPanel} title="Toggle panel">
					<IconLayoutSidebarRight size={20} />
				</button>
			{/if}
			<button class="icon-btn" onclick={toggleFullscreen} title="Toggle fullscreen">
				{#if isFullscreen}
					<IconMinimize size={20} />
				{:else}
					<IconMaximize size={20} />
				{/if}
			</button>
			<button class="icon-btn" title="Settings">
				<IconSettings size={20} />
			</button>
			<div class="user-menu">
				<div class="user-avatar">
					{$user?.name?.[0]?.toUpperCase() || 'U'}
				</div>
			</div>
		</div>
	</header>

	<!-- Main Layout -->
	<div class="shell-body">
		<!-- Left Sidebar -->
		{#if sidebar && sidebarOpen}
			<aside class="shell-sidebar">
				{@render sidebar()}
			</aside>
		{/if}

		<!-- Main Content -->
		<main class="shell-main">
			{@render children()}
		</main>

		<!-- Right Panel -->
		{#if rightPanel && rightPanelOpen}
			<aside class="shell-right-panel">
				{@render rightPanel()}
			</aside>
		{/if}
	</div>
</div>

<style>
	.trading-room-shell {
		display: flex;
		flex-direction: column;
		height: 100vh;
		width: 100vw;
		background: var(--color-rtp-bg, #0f172a);
		color: var(--color-rtp-text, #e2e8f0);
		overflow: hidden;
	}

	.trading-room-shell.fullscreen {
		position: fixed;
		top: 0;
		left: 0;
		z-index: 9999;
	}

	/* Header */
	.shell-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		height: 48px;
		padding: 0 16px;
		background: var(--color-rtp-surface, #1e293b);
		border-bottom: 1px solid var(--color-rtp-border, #334155);
		flex-shrink: 0;
	}

	.header-left,
	.header-right {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.header-center {
		flex: 1;
		display: flex;
		justify-content: center;
	}

	.room-info {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.room-icon {
		font-size: 20px;
	}

	.room-name {
		font-weight: 600;
		font-size: 14px;
	}

	.icon-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		border: none;
		background: transparent;
		color: var(--color-rtp-text-muted, #94a3b8);
		border-radius: 6px;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.icon-btn:hover {
		background: var(--color-rtp-hover, rgba(255, 255, 255, 0.1));
		color: var(--color-rtp-text, #e2e8f0);
	}

	.user-menu {
		display: flex;
		align-items: center;
	}

	.user-avatar {
		width: 28px;
		height: 28px;
		border-radius: 50%;
		background: linear-gradient(
			135deg,
			var(--color-rtp-primary, #3b82f6) 0%,
			var(--color-rtp-primary-dark, #2563eb) 100%
		);
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 12px;
		font-weight: 600;
		color: white;
	}

	/* Body Layout */
	.shell-body {
		display: flex;
		flex: 1;
		overflow: hidden;
	}

	.shell-sidebar {
		width: 260px;
		background: var(--color-rtp-surface, #1e293b);
		border-right: 1px solid var(--color-rtp-border, #334155);
		overflow-y: auto;
		flex-shrink: 0;
	}

	.shell-main {
		flex: 1;
		overflow: auto;
		display: flex;
		flex-direction: column;
	}

	.shell-right-panel {
		width: 320px;
		background: var(--color-rtp-surface, #1e293b);
		border-left: 1px solid var(--color-rtp-border, #334155);
		overflow-y: auto;
		flex-shrink: 0;
	}

	/* Responsive */
	@media (max-width: 1024px) {
		.shell-sidebar {
			position: fixed;
			top: 48px;
			left: 0;
			bottom: 0;
			z-index: 100;
			transform: translateX(0);
			transition: transform 0.2s ease;
		}

		.shell-right-panel {
			position: fixed;
			top: 48px;
			right: 0;
			bottom: 0;
			z-index: 100;
			transform: translateX(0);
			transition: transform 0.2s ease;
		}
	}

	@media (max-width: 640px) {
		.room-name {
			display: none;
		}

		.shell-sidebar,
		.shell-right-panel {
			width: 100%;
		}
	}
</style>

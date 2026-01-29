<script lang="ts">
	/**
	 * Admin Layout - Dashboard shell for admin area
	 *
	 * @version 4.1.0 - Svelte 5 Runes
	 * @author Revolution Trading Pros
	 */
	import '$lib/styles/main.css';

	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import { isAuthenticated } from '$lib/stores/auth.svelte';
	import { unreadCount } from '$lib/stores/notifications.svelte';
	import { keyboard } from '$lib/stores/keyboard.svelte';

	import IconMenu2 from '@tabler/icons-svelte/icons/menu-2';
	import IconBell from '@tabler/icons-svelte/icons/bell';
	import IconSearch from '@tabler/icons-svelte/icons/search';
	import IconPlugConnected from '@tabler/icons-svelte/icons/plug-connected';
	import IconCommand from '@tabler/icons-svelte/icons/command';

	import { AdminSidebar } from '$lib/components/layout';
	import Toast from '$lib/components/Toast.svelte';
	import CommandPalette from '$lib/components/CommandPalette.svelte';
	import NotificationCenter from '$lib/components/NotificationCenter.svelte';
	import KeyboardShortcutsHelp from '$lib/components/KeyboardShortcutsHelp.svelte';
	import RateLimitIndicator from '$lib/components/RateLimitIndicator.svelte';
	import ConnectionHealthPanel from '$lib/components/ConnectionHealthPanel.svelte';
	import OfflineIndicator from '$lib/components/OfflineIndicator.svelte';

	import type { Snippet } from 'svelte';

	// Props
	let { children }: { children: Snippet } = $props();

	// State
	let isSidebarOpen = $state(false);
	let isCommandPaletteOpen = $state(false);
	let isNotificationCenterOpen = $state(false);
	let isKeyboardHelpOpen = $state(false);
	let isConnectionHealthOpen = $state(false);

	// Auth guard
	$effect(() => {
		if (browser && !$isAuthenticated) {
			goto('/login?redirect=/admin');
		}
	});

	// Keyboard shortcuts
	$effect(() => {
		if (!browser) return;

		keyboard.init({
			search: () => (isCommandPaletteOpen = true),
			'search-alt': () => (isCommandPaletteOpen = true),
			help: () => (isKeyboardHelpOpen = true),
			'goto-dashboard': () => goto('/admin'),
			'goto-analytics': () => goto('/admin/analytics'),
			'goto-blog': () => goto('/admin/blog'),
			'goto-settings': () => goto('/admin/settings'),
			escape: () => {
				isCommandPaletteOpen = false;
				isNotificationCenterOpen = false;
				isKeyboardHelpOpen = false;
				isConnectionHealthOpen = false;
			}
		});

		return () => keyboard.destroy();
	});

	// Functions
	function toggleSidebar() {
		isSidebarOpen = !isSidebarOpen;
	}

	function closeSidebar() {
		isSidebarOpen = false;
	}

	function formatPageTitle(pathname: string): string {
		const segments = pathname.split('/').filter(Boolean);
		if (segments.length <= 1) return 'Dashboard';

		const lastSegment = segments[segments.length - 1];

		const titleMap: Record<string, string> = {
			blog: 'Blog Posts',
			categories: 'Categories',
			media: 'Media Library',
			members: 'Members',
			segments: 'Segments',
			subscriptions: 'Subscriptions',
			products: 'Products',
			coupons: 'Coupons',
			crm: 'CRM',
			campaigns: 'Email Campaigns',
			templates: 'Email Templates',
			smtp: 'Email Settings',
			seo: 'SEO Settings',
			analytics: 'Analytics',
			behavior: 'Behavior Tracking',
			users: 'Admin Users',
			settings: 'Settings',
			popups: 'Popups',
			forms: 'Forms',
			videos: 'Videos',
			resources: 'Resources',
			indicators: 'Indicators',
			'site-health': 'Site Health',
			connections: 'API Connections'
		};

		return (
			titleMap[lastSegment] ||
			lastSegment.charAt(0).toUpperCase() + lastSegment.slice(1).replace(/-/g, ' ')
		);
	}
</script>

<svelte:head>
	<title>Admin Dashboard | Revolution Trading Pros</title>
</svelte:head>

<div class="admin-layout admin">
	<!-- Sidebar -->
	<AdminSidebar isOpen={isSidebarOpen} onclose={closeSidebar} />

	<!-- Main Content -->
	<div class="admin-main">
		<!-- Header -->
		<header class="admin-header">
			<button
				type="button"
				class="mobile-menu-btn"
				onclick={toggleSidebar}
				aria-label="Toggle sidebar"
			>
				<IconMenu2 size={24} />
			</button>

			<div class="header-title">
				<h1>{formatPageTitle($page.url.pathname)}</h1>
			</div>

			<div class="header-actions">
				<!-- Search -->
				<button
					type="button"
					class="header-btn"
					onclick={() => (isCommandPaletteOpen = true)}
					title="Search (⌘K)"
					aria-label="Open search"
				>
					<IconSearch size={18} />
					<span class="btn-label desktop-only">Search</span>
					<kbd class="kbd desktop-only">⌘K</kbd>
				</button>

				<!-- Notifications -->
				<button
					type="button"
					class="header-btn notification-btn"
					onclick={() => (isNotificationCenterOpen = true)}
					title="Notifications"
					aria-label="Open notifications"
				>
					<IconBell size={18} />
					{#if $unreadCount > 0}
						<span class="notification-badge">{$unreadCount > 9 ? '9+' : $unreadCount}</span>
					{/if}
				</button>

				<!-- Connection Health -->
				<button
					type="button"
					class="header-btn"
					onclick={() => (isConnectionHealthOpen = true)}
					title="API Connection Status"
					aria-label="View API connection status"
				>
					<IconPlugConnected size={18} />
				</button>

				<!-- Rate Limit -->
				<RateLimitIndicator />

				<!-- Keyboard Shortcuts -->
				<button
					type="button"
					class="header-btn desktop-only"
					onclick={() => (isKeyboardHelpOpen = true)}
					title="Keyboard Shortcuts (?)"
					aria-label="View keyboard shortcuts"
				>
					<IconCommand size={18} />
				</button>

				<a href="/" class="view-site-btn" title="View main website">View Site</a>
			</div>
		</header>

		<!-- Content -->
		<main id="main-content" class="admin-content">
			{@render children()}
		</main>
	</div>
</div>

<!-- Global Components -->
<Toast />
<CommandPalette bind:isOpen={isCommandPaletteOpen} />
<NotificationCenter bind:isOpen={isNotificationCenterOpen} />
<KeyboardShortcutsHelp bind:isOpen={isKeyboardHelpOpen} />
<ConnectionHealthPanel bind:isOpen={isConnectionHealthOpen} />
<OfflineIndicator />

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   ADMIN LAYOUT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.admin-layout {
		display: flex;
		background: var(--admin-bg);
		color: var(--admin-text-primary);
		transition:
			background-color 0.3s ease,
			color 0.3s ease;
	}

	.admin-main {
		flex: 1;
		margin-left: var(--admin-sidebar-width, 240px);
		display: flex;
		flex-direction: column;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   HEADER
	   ═══════════════════════════════════════════════════════════════════════════ */

	.admin-header {
		height: 70px;
		background: var(--admin-header-bg);
		border-bottom: 1px solid var(--admin-header-border);
		box-shadow: var(--admin-header-shadow);
		display: flex;
		align-items: center;
		padding: 0 var(--space-8);
		gap: var(--space-4);
		position: sticky;
		top: 0;
		z-index: var(--z-sticky);
		transition:
			var(--transition-colors),
			box-shadow var(--duration-normal) var(--ease-default);
	}

	.mobile-menu-btn {
		display: none;
		background: transparent;
		border: none;
		color: var(--admin-text-muted);
		cursor: pointer;
		padding: var(--space-2);
		border-radius: var(--radius-md);
		transition: var(--transition-all);
	}

	.mobile-menu-btn:hover {
		background: var(--admin-btn-bg-hover);
		color: var(--admin-accent-primary);
	}

	.mobile-menu-btn:focus-visible {
		box-shadow: var(--admin-focus-ring);
		outline: none;
	}

	.header-title h1 {
		font-family: var(--font-display);
		font-size: var(--text-xl);
		font-weight: var(--font-semibold);
		color: var(--admin-text-primary);
		text-transform: capitalize;
		margin: 0;
		letter-spacing: var(--tracking-tight);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   HEADER ACTIONS
	   ═══════════════════════════════════════════════════════════════════════════ */

	.header-actions {
		margin-left: auto;
		display: flex;
		align-items: center;
		gap: var(--space-3);
	}

	.header-btn {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-2) var(--space-3);
		background: var(--admin-btn-bg);
		border: 1px solid var(--admin-btn-border);
		border-radius: var(--radius-md);
		color: var(--admin-btn-text);
		cursor: pointer;
		transition: var(--transition-all);
		position: relative;
	}

	.header-btn:hover {
		background: var(--admin-btn-bg-hover);
		border-color: var(--admin-btn-border-hover);
		color: var(--admin-btn-text-hover);
	}

	.header-btn:focus-visible {
		box-shadow: var(--admin-focus-ring);
		outline: none;
	}

	.header-btn:active {
		background: var(--admin-btn-bg-active);
		transform: scale(0.98);
	}

	.btn-label {
		font-family: var(--font-sans);
		font-size: var(--text-sm);
		font-weight: var(--font-medium);
		letter-spacing: var(--tracking-normal);
	}

	.kbd {
		padding: var(--space-1) var(--space-2);
		background: var(--admin-kbd-bg);
		border: 1px solid var(--admin-kbd-border);
		border-radius: var(--radius-sm);
		font-size: var(--text-xs);
		font-weight: var(--font-semibold);
		color: var(--admin-kbd-text);
		font-family: var(--font-mono);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   NOTIFICATION BADGE
	   ═══════════════════════════════════════════════════════════════════════════ */

	.notification-btn {
		position: relative;
	}

	.notification-badge {
		position: absolute;
		top: -4px;
		right: -4px;
		min-width: 18px;
		height: 18px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--color-loss);
		border-radius: var(--radius-full);
		font-size: var(--text-xs);
		font-weight: var(--font-bold);
		color: var(--color-bg-card);
		padding: 0 var(--space-1);
		box-shadow: var(--shadow-loss);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   VIEW SITE BUTTON
	   ═══════════════════════════════════════════════════════════════════════════ */

	.view-site-btn {
		padding: var(--space-2) var(--space-4);
		background: var(--color-watching-bg);
		color: var(--color-watching-text);
		text-decoration: none;
		border-radius: var(--radius-md);
		font-family: var(--font-sans);
		font-weight: var(--font-medium);
		font-size: var(--text-sm);
		letter-spacing: var(--tracking-normal);
		border: 1px solid var(--color-watching-border);
		transition: var(--transition-all);
	}

	.view-site-btn:hover {
		background: var(--color-watching);
		border-color: var(--color-watching);
		color: var(--color-bg-card);
		transform: translateY(-1px);
	}

	.view-site-btn:focus-visible {
		box-shadow: 0 0 0 3px var(--color-watching-bg);
		outline: none;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CONTENT AREA
	   ═══════════════════════════════════════════════════════════════════════════ */

	.admin-content {
		flex: 1;
		padding: var(--space-8);
		overflow-y: auto;
		background: var(--admin-bg);
	}

	.desktop-only {
		display: inline-flex;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE - Tablet (< lg: 1024px)
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: calc(var(--breakpoint-lg) - 1px)) {
		.admin-main {
			margin-left: 0;
		}

		.mobile-menu-btn {
			display: block;
		}

		.admin-content {
			padding: var(--space-6);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE - Mobile Landscape (< md: 768px)
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: calc(var(--breakpoint-md) - 1px)) {
		.desktop-only {
			display: none !important;
		}

		.header-btn .btn-label,
		.header-btn .kbd {
			display: none;
		}

		.header-btn {
			padding: var(--space-2);
		}

		.view-site-btn {
			padding: var(--space-2) var(--space-3);
			font-size: var(--text-sm);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE - Mobile Portrait (< sm: 640px)
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: calc(var(--breakpoint-sm) - 1px)) {
		.admin-header {
			padding: 0 var(--space-4);
			height: 60px;
		}

		.header-title h1 {
			font-size: var(--text-lg);
		}

		.admin-content {
			padding: var(--space-4);
		}

		.header-actions {
			gap: var(--space-2);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE - Extra Small Mobile
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 380px) {
		.admin-header {
			padding: 0 var(--space-3);
			height: 56px;
		}

		.header-title h1 {
			font-size: var(--text-base);
		}

		.header-actions {
			gap: var(--space-1);
		}

		.header-btn {
			padding: var(--space-2);
		}

		.view-site-btn {
			padding: var(--space-2) var(--space-3);
			font-size: var(--text-xs);
		}

		.admin-content {
			padding: var(--space-3);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   TOUCH DEVICES - 44pt minimum touch targets
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (hover: none) and (pointer: coarse) {
		.header-btn,
		.mobile-menu-btn,
		.view-site-btn {
			min-height: 44px;
			min-width: 44px;
		}

		.header-btn {
			padding: var(--space-3);
		}

		.view-site-btn {
			padding: var(--space-3) var(--space-4);
		}

		.header-actions {
			gap: var(--space-2);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ACCESSIBILITY
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.admin-layout,
		.admin-header,
		.header-btn,
		.mobile-menu-btn,
		.view-site-btn {
			transition: none;
		}

		.view-site-btn:hover,
		.mobile-menu-btn:hover {
			transform: none;
		}
	}

	@media (prefers-contrast: high) {
		.admin-header {
			border-bottom-width: 2px;
		}

		.header-btn,
		.view-site-btn {
			border-width: 2px;
		}

		.header-title h1 {
			font-weight: var(--font-extrabold);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   PRINT
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media print {
		.admin-header,
		.mobile-menu-btn {
			display: none !important;
		}

		.admin-main {
			margin-left: 0;
		}

		.admin-content {
			padding: 0;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   LANDSCAPE - Short Viewport
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-height: 500px) and (orientation: landscape) {
		.admin-header {
			height: 50px;
			padding: 0 var(--space-4);
		}

		.header-title h1 {
			font-size: var(--text-base);
		}

		.admin-content {
			padding: var(--space-3) var(--space-4);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   GLOBAL TAB PANEL UTILITIES - Layout Shift Prevention
	   ICT 7 Principal Engineer Grade - Svelte 5 Best Practices
	   ═══════════════════════════════════════════════════════════════════════════ */

	:global(.admin-tab-container) {
		position: relative;
		min-height: 400px;
		contain: layout style;
		isolation: isolate;
	}

	:global(.admin-tab-panel) {
		position: absolute;
		inset: 0;
		width: 100%;
		contain: content;
		opacity: 0;
		visibility: hidden;
		transform: translateY(8px);
		transition: 
			opacity 0.2s ease,
			visibility 0.2s ease,
			transform 0.2s ease;
		z-index: 0;
		pointer-events: none;
	}

	:global(.admin-tab-panel.active) {
		position: relative;
		opacity: 1;
		visibility: visible;
		transform: translateY(0);
		z-index: 1;
		pointer-events: auto;
	}

	@media (prefers-reduced-motion: reduce) {
		:global(.admin-tab-panel) {
			transition: none;
			transform: none;
		}
		:global(.admin-tab-panel:not(.active)) {
			display: none;
		}
	}
</style>

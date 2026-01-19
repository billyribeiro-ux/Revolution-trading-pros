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
			<button type="button" class="mobile-menu-btn" onclick={toggleSidebar} aria-label="Toggle sidebar">
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
		margin-left: 240px;
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
		padding: 0 2rem;
		gap: 1rem;
		position: sticky;
		top: 0;
		z-index: 30;
		transition:
			background-color 0.3s ease,
			border-color 0.3s ease,
			box-shadow 0.3s ease;
	}

	.mobile-menu-btn {
		display: none;
		background: transparent;
		border: none;
		color: var(--admin-text-muted);
		cursor: pointer;
		padding: 0.5rem;
		border-radius: var(--radius-md, 0.5rem);
		transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
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
		font-family: var(--font-heading), 'Montserrat', -apple-system, BlinkMacSystemFont, sans-serif;
		font-size: 1.375rem;
		font-weight: 600;
		color: var(--admin-text-primary);
		text-transform: capitalize;
		margin: 0;
		letter-spacing: -0.005em;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   HEADER ACTIONS
	   ═══════════════════════════════════════════════════════════════════════════ */

	.header-actions {
		margin-left: auto;
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.header-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		background: var(--admin-btn-bg);
		border: 1px solid var(--admin-btn-border);
		border-radius: var(--radius-md, 0.5rem);
		color: var(--admin-btn-text);
		cursor: pointer;
		transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
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
		font-family: var(--font-body), 'Roboto', sans-serif;
		font-size: 0.8125rem;
		font-weight: 500;
		letter-spacing: 0.01em;
	}

	.kbd {
		padding: 0.125rem 0.375rem;
		background: var(--admin-kbd-bg);
		border: 1px solid var(--admin-kbd-border);
		border-radius: 4px;
		font-size: 0.6875rem;
		font-weight: 600;
		color: var(--admin-kbd-text);
		font-family: inherit;
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
		background: linear-gradient(135deg, #ef4444, #dc2626);
		border-radius: 9px;
		font-size: 0.625rem;
		font-weight: 700;
		color: white;
		padding: 0 4px;
		box-shadow: 0 2px 4px rgba(220, 38, 38, 0.3);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   VIEW SITE BUTTON
	   ═══════════════════════════════════════════════════════════════════════════ */

	.view-site-btn {
		padding: 0.5rem 1rem;
		background: rgba(230, 184, 0, 0.15);
		color: #f0f6fc;
		text-decoration: none;
		border-radius: var(--radius-md, 0.5rem);
		font-family: var(--font-body), 'Roboto', sans-serif;
		font-weight: 500;
		font-size: 0.875rem;
		letter-spacing: 0.01em;
		border: 1px solid rgba(230, 184, 0, 0.4);
		transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
	}

	.view-site-btn:hover {
		background: rgba(230, 184, 0, 0.25);
		border-color: #e6b800;
		color: #ffffff;
		transform: translateY(-1px);
	}

	.view-site-btn:focus-visible {
		box-shadow: 0 0 0 3px rgba(230, 184, 0, 0.35);
		outline: none;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CONTENT AREA
	   ═══════════════════════════════════════════════════════════════════════════ */

	.admin-content {
		flex: 1;
		padding: 2rem;
		overflow-y: auto;
		background: var(--admin-bg);
	}

	.desktop-only {
		display: inline-flex;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE - Tablet
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 1024px) {
		.admin-main {
			margin-left: 0;
		}

		.mobile-menu-btn {
			display: block;
		}

		.admin-content {
			padding: 1.5rem;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE - Mobile Landscape
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 768px) {
		.desktop-only {
			display: none !important;
		}

		.header-btn .btn-label,
		.header-btn .kbd {
			display: none;
		}

		.header-btn {
			padding: 0.5rem;
		}

		.view-site-btn {
			padding: 0.5rem 0.875rem;
			font-size: 0.8125rem;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE - Mobile Portrait
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 640px) {
		.admin-header {
			padding: 0 1rem;
			height: 60px;
		}

		.header-title h1 {
			font-size: 1.25rem;
		}

		.admin-content {
			padding: 1rem;
		}

		.header-actions {
			gap: 0.375rem;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE - Extra Small Mobile
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 380px) {
		.admin-header {
			padding: 0 0.75rem;
			height: 56px;
		}

		.header-title h1 {
			font-size: 1.1rem;
		}

		.header-actions {
			gap: 0.25rem;
		}

		.header-btn {
			padding: 0.375rem;
		}

		.view-site-btn {
			padding: 0.375rem 0.625rem;
			font-size: 0.75rem;
		}

		.admin-content {
			padding: 0.75rem;
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
			padding: 0.625rem;
		}

		.view-site-btn {
			padding: 0.625rem 1rem;
		}

		.header-actions {
			gap: 0.5rem;
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
			font-weight: 800;
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
			padding: 0 1rem;
		}

		.header-title h1 {
			font-size: 1.1rem;
		}

		.admin-content {
			padding: 0.75rem 1rem;
		}
	}
</style>
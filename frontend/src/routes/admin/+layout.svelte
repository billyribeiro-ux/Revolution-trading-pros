<script lang="ts">
	/**
	 * Admin Layout - Dashboard shell for admin area
	 * Uses extracted AdminSidebar component for cleaner architecture
	 *
	 * Updated to Svelte 5 runes syntax (November 2025)
	 * Enhanced with Apple ICT9+ Enterprise Components
	 *
	 * @version 4.0.0
	 * @author Revolution Trading Pros
	 */
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { isAuthenticated } from '$lib/stores/auth';
	import { themeStore, type Theme } from '$lib/stores/theme';
	import { unreadCount } from '$lib/stores/notifications';
	import { keyboardStore } from '$lib/stores/keyboard';
	import {
		IconMenu2,
		IconSun,
		IconMoon,
		IconDeviceDesktop,
		IconBell,
		IconSearch,
		IconPlug,
		IconKeyboard
	} from '@tabler/icons-svelte';
	import { browser } from '$app/environment';
	import { AdminSidebar } from '$lib/components/layout';
	import Toast from '$lib/components/Toast.svelte';
	import CommandPalette from '$lib/components/CommandPalette.svelte';
	import NotificationCenter from '$lib/components/NotificationCenter.svelte';
	import KeyboardShortcutsHelp from '$lib/components/KeyboardShortcutsHelp.svelte';
	import RateLimitIndicator from '$lib/components/RateLimitIndicator.svelte';
	import ConnectionHealthPanel from '$lib/components/ConnectionHealthPanel.svelte';
	import OfflineIndicator from '$lib/components/OfflineIndicator.svelte';
	import type { Snippet } from 'svelte';

	// Theme icon mapping
	function getThemeIcon(theme: Theme) {
		switch (theme) {
			case 'light': return IconSun;
			case 'dark': return IconMoon;
			case 'auto': return IconDeviceDesktop;
		}
	}

	function getThemeLabel(theme: Theme) {
		switch (theme) {
			case 'light': return 'Light';
			case 'dark': return 'Dark';
			case 'auto': return 'Auto';
		}
	}

	// Svelte 5: Props with children snippet
	let { children }: { children: Snippet } = $props();

	// Svelte 5 state runes
	let isSidebarOpen = $state(false);
	let isCommandPaletteOpen = $state(false);
	let isNotificationCenterOpen = $state(false);
	let isKeyboardHelpOpen = $state(false);
	let isConnectionHealthOpen = $state(false);

	// Check if user is admin - Svelte 5 effect
	$effect(() => {
		if (browser && !$isAuthenticated) {
			goto('/login?redirect=/admin');
		}
	});

	// Register keyboard shortcuts
	$effect(() => {
		if (!browser) return;

		// Register shortcut handlers
		keyboardStore.registerAction('search', () => {
			isCommandPaletteOpen = true;
		});

		keyboardStore.registerAction('show-shortcuts', () => {
			isKeyboardHelpOpen = true;
		});

		keyboardStore.registerAction('goto-dashboard', () => {
			goto('/admin');
		});

		keyboardStore.registerAction('goto-analytics', () => {
			goto('/admin/analytics');
		});

		keyboardStore.registerAction('goto-content', () => {
			goto('/admin/blog');
		});

		keyboardStore.registerAction('goto-settings', () => {
			goto('/admin/settings');
		});

		// Global keyboard listener
		const handleKeyDown = (e: KeyboardEvent) => {
			// Cmd/Ctrl + K for command palette
			if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
				e.preventDefault();
				isCommandPaletteOpen = true;
			}
		};

		window.addEventListener('keydown', handleKeyDown);

		return () => {
			window.removeEventListener('keydown', handleKeyDown);
		};
	});

	function toggleSidebar() {
		isSidebarOpen = !isSidebarOpen;
	}

	function closeSidebar() {
		isSidebarOpen = false;
	}

	// Format page title from pathname
	function formatPageTitle(pathname: string): string {
		const segments = pathname.split('/').filter(Boolean);
		if (segments.length <= 1) return 'Dashboard';
		
		const lastSegment = segments[segments.length - 1];
		// Handle special cases
		const titleMap: Record<string, string> = {
			'blog': 'Blog Posts',
			'categories': 'Categories',
			'media': 'Media Library',
			'members': 'Members',
			'segments': 'Segments',
			'subscriptions': 'Subscriptions',
			'products': 'Products',
			'coupons': 'Coupons',
			'crm': 'CRM',
			'campaigns': 'Email Campaigns',
			'templates': 'Email Templates',
			'smtp': 'Email Settings',
			'seo': 'SEO Settings',
			'analytics': 'Analytics',
			'behavior': 'Behavior Tracking',
			'users': 'Admin Users',
			'settings': 'Settings',
			'popups': 'Popups',
			'forms': 'Forms',
			'videos': 'Videos',
			'site-health': 'Site Health',
			'connections': 'API Connections'
		};
		
		return titleMap[lastSegment] || lastSegment.charAt(0).toUpperCase() + lastSegment.slice(1).replace(/-/g, ' ');
	}
</script>

<svelte:head>
	<title>Admin Dashboard | Revolution Trading Pros</title>
</svelte:head>

<div class="admin-layout">
	<!-- Sidebar Component -->
	<AdminSidebar isOpen={isSidebarOpen} onclose={closeSidebar} />

	<!-- Main Content -->
	<div class="admin-main">
		<!-- Top Bar -->
		<header class="admin-header">
			<button class="mobile-menu-btn" onclick={toggleSidebar}>
				<IconMenu2 size={24} />
			</button>
			<div class="header-title">
				<h1>{formatPageTitle($page.url.pathname)}</h1>
			</div>
			<div class="header-actions">
				<!-- Search Button -->
				<button
					class="header-btn"
					onclick={() => isCommandPaletteOpen = true}
					title="Search (⌘K)"
				>
					<IconSearch size={18} />
					<span class="btn-label desktop-only">Search</span>
					<kbd class="kbd desktop-only">⌘K</kbd>
				</button>

				<!-- Notifications Button -->
				<button
					class="header-btn notification-btn"
					onclick={() => isNotificationCenterOpen = true}
					title="Notifications"
				>
					<IconBell size={18} />
					{#if $unreadCount > 0}
						<span class="notification-badge">{$unreadCount > 9 ? '9+' : $unreadCount}</span>
					{/if}
				</button>

				<!-- Connection Health Button -->
				<button
					class="header-btn"
					onclick={() => isConnectionHealthOpen = true}
					title="API Connections"
				>
					<IconPlug size={18} />
				</button>

				<!-- Rate Limit Indicator -->
				<RateLimitIndicator />

				<!-- Theme Toggle -->
				<button
					class="theme-toggle"
					onclick={() => themeStore.cycle()}
					title="Theme: {getThemeLabel($themeStore)} (click to cycle)"
				>
					<svelte:component this={getThemeIcon($themeStore)} size={20} />
					<span class="theme-label desktop-only">{getThemeLabel($themeStore)}</span>
				</button>

				<!-- Keyboard Shortcuts -->
				<button
					class="header-btn desktop-only"
					onclick={() => isKeyboardHelpOpen = true}
					title="Keyboard Shortcuts"
				>
					<IconKeyboard size={18} />
				</button>

				<a href="/" class="view-site-btn" target="_blank">View Site</a>
			</div>
		</header>

		<!-- Page Content -->
		<main class="admin-content">
			{@render children()}
		</main>
	</div>
</div>

<!-- Toast Notifications -->
<Toast />

<!-- Global Modals & Overlays -->
<CommandPalette bind:isOpen={isCommandPaletteOpen} />
<NotificationCenter bind:isOpen={isNotificationCenterOpen} />
<KeyboardShortcutsHelp bind:isOpen={isKeyboardHelpOpen} />
<ConnectionHealthPanel bind:isOpen={isConnectionHealthOpen} />
<OfflineIndicator />

<style>
	.admin-layout {
		display: flex;
		min-height: 100vh;
		background: var(--color-rtp-bg, #0f172a);
		color: var(--color-rtp-text, #e2e8f0);
	}

	/* Main Content */
	.admin-main {
		flex: 1;
		margin-left: 240px;
		display: flex;
		flex-direction: column;
		min-height: 100vh;
	}

	.admin-header {
		height: 70px;
		background: var(--color-rtp-surface, #1e293b);
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
		display: flex;
		align-items: center;
		padding: 0 2rem;
		gap: 1rem;
		position: sticky;
		top: 0;
		z-index: 30;
	}

	.mobile-menu-btn {
		display: none;
		background: none;
		border: none;
		color: var(--color-rtp-muted, #94a3b8);
		cursor: pointer;
		padding: 0.5rem;
		border-radius: var(--radius-md, 0.5rem);
		transition: all 0.2s;
	}

	.mobile-menu-btn:hover {
		background: rgba(99, 102, 241, 0.1);
		color: var(--color-rtp-primary, #818cf8);
	}

	.header-title h1 {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--color-rtp-text, #f1f5f9);
		text-transform: capitalize;
		margin: 0;
	}

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
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: var(--radius-md, 0.5rem);
		color: var(--color-rtp-muted, #94a3b8);
		cursor: pointer;
		transition: all 0.2s ease;
		position: relative;
	}

	.header-btn:hover {
		background: rgba(99, 102, 241, 0.2);
		border-color: rgba(99, 102, 241, 0.4);
		color: var(--color-rtp-primary, #a5b4fc);
	}

	.btn-label {
		font-size: 0.8125rem;
		font-weight: 500;
	}

	.kbd {
		padding: 0.125rem 0.375rem;
		background: rgba(99, 102, 241, 0.15);
		border-radius: 4px;
		font-size: 0.6875rem;
		font-weight: 600;
		color: #a5b4fc;
		font-family: inherit;
	}

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
	}

	.theme-toggle {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.875rem;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: var(--radius-md, 0.5rem);
		color: var(--color-rtp-muted, #94a3b8);
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.theme-toggle:hover {
		background: rgba(99, 102, 241, 0.2);
		border-color: rgba(99, 102, 241, 0.4);
		color: var(--color-rtp-primary, #a5b4fc);
	}

	.theme-label {
		font-size: 0.8125rem;
		font-weight: 500;
	}

	.desktop-only {
		display: inline-flex;
	}

	.view-site-btn {
		padding: 0.625rem 1.25rem;
		background: rgba(99, 102, 241, 0.1);
		color: var(--color-rtp-primary, #a5b4fc);
		text-decoration: none;
		border-radius: var(--radius-md, 0.5rem);
		font-weight: 600;
		font-size: 0.9375rem;
		border: 1px solid rgba(99, 102, 241, 0.2);
		transition: all 0.2s;
	}

	.view-site-btn:hover {
		background: rgba(99, 102, 241, 0.2);
		border-color: rgba(99, 102, 241, 0.4);
	}

	.admin-content {
		flex: 1;
		padding: 2rem;
		overflow-y: auto;
	}

	/* Responsive */
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

	@media (max-width: 640px) {
		.admin-header {
			padding: 0 1rem;
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

		.theme-toggle {
			padding: 0.5rem;
		}

		.theme-label {
			display: none;
		}
	}
</style>

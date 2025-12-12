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
	import { keyboard } from '$lib/stores/keyboard';
	import IconMenu2 from '@tabler/icons-svelte/icons/menu-2';
	import IconSunHigh from '@tabler/icons-svelte/icons/sun-high';
	import IconMoon from '@tabler/icons-svelte/icons/moon';
	import IconDeviceDesktop from '@tabler/icons-svelte/icons/device-desktop';
	import IconBell from '@tabler/icons-svelte/icons/bell';
	import IconSearch from '@tabler/icons-svelte/icons/search';
	import IconPlugConnected from '@tabler/icons-svelte/icons/plug-connected';
	import IconCommand from '@tabler/icons-svelte/icons/command';
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
			case 'light': return IconSunHigh;
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

	// Derived theme icon component
	let ThemeIcon = $derived(getThemeIcon($themeStore));

	// Check if user is admin - Svelte 5 effect
	$effect(() => {
		if (browser && !$isAuthenticated) {
			goto('/login?redirect=/admin');
		}
	});

	// Initialize keyboard shortcuts
	$effect(() => {
		if (!browser) return;

		// Initialize keyboard shortcuts with custom actions
		keyboard.init({
			'search': () => {
				isCommandPaletteOpen = true;
			},
			'search-alt': () => {
				isCommandPaletteOpen = true;
			},
			'help': () => {
				isKeyboardHelpOpen = true;
			},
			'goto-dashboard': () => {
				goto('/admin');
			},
			'goto-analytics': () => {
				goto('/admin/analytics');
			},
			'goto-blog': () => {
				goto('/admin/blog');
			},
			'goto-settings': () => {
				goto('/admin/settings');
			},
			'escape': () => {
				isCommandPaletteOpen = false;
				isNotificationCenterOpen = false;
				isKeyboardHelpOpen = false;
				isConnectionHealthOpen = false;
			}
		});

		return () => {
			keyboard.destroy();
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
					<IconPlugConnected size={18} />
				</button>

				<!-- Rate Limit Indicator -->
				<RateLimitIndicator />

				<!-- Theme Toggle -->
				<button
					class="theme-toggle"
					onclick={() => themeStore.cycle()}
					title="Theme: {getThemeLabel($themeStore)} (click to cycle)"
				>
					<ThemeIcon size={20} />
					<span class="theme-label desktop-only">{getThemeLabel($themeStore)}</span>
				</button>

				<!-- Keyboard Shortcuts -->
				<button
					class="header-btn desktop-only"
					onclick={() => isKeyboardHelpOpen = true}
					title="Keyboard Shortcuts"
				>
					<IconCommand size={18} />
				</button>

				<a href="/" class="view-site-btn">View Site</a>
			</div>
		</header>

		<!-- Page Content -->
		<main class="admin-content">
			{@render children?.()}
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
	/* ═══════════════════════════════════════════════════════════════════════════
	 * ADMIN LAYOUT - Netflix L11+ Principal Engineer Grade
	 * Uses CSS custom properties for bulletproof light/dark theme support
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.admin-layout {
		display: flex;
		min-height: 100vh;
		background: var(--admin-bg);
		color: var(--admin-text-primary);
		transition: background-color 0.3s ease, color 0.3s ease;
	}

	/* Main Content Area */
	.admin-main {
		flex: 1;
		margin-left: 240px;
		display: flex;
		flex-direction: column;
		min-height: 100vh;
	}

	/* Header - Theme-aware styling */
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
		transition: background-color 0.3s ease, border-color 0.3s ease, box-shadow 0.3s ease;
	}

	/* Mobile Menu Button */
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

	/* Header Title */
	.header-title h1 {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--admin-text-primary);
		text-transform: capitalize;
		margin: 0;
		letter-spacing: -0.01em;
	}

	/* Header Actions Container */
	.header-actions {
		margin-left: auto;
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	/* Header Button - Theme-aware */
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

	/* Button Label */
	.btn-label {
		font-size: 0.8125rem;
		font-weight: 500;
	}

	/* Keyboard Shortcut Indicator - Uses CSS variables from app.css */
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

	/* Notification Button */
	.notification-btn {
		position: relative;
	}

	/* Notification Badge - Always visible (red) */
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

	/* Theme Toggle Button */
	.theme-toggle {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.875rem;
		background: var(--admin-btn-bg);
		border: 1px solid var(--admin-btn-border);
		border-radius: var(--radius-md, 0.5rem);
		color: var(--admin-btn-text);
		cursor: pointer;
		transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
	}

	.theme-toggle:hover {
		background: var(--admin-btn-bg-hover);
		border-color: var(--admin-btn-border-hover);
		color: var(--admin-btn-text-hover);
	}

	.theme-toggle:focus-visible {
		box-shadow: var(--admin-focus-ring);
		outline: none;
	}

	.theme-label {
		font-size: 0.8125rem;
		font-weight: 500;
	}

	/* Desktop-only elements */
	.desktop-only {
		display: inline-flex;
	}

	/* View Site Button - Accent styled */
	.view-site-btn {
		padding: 0.625rem 1.25rem;
		background: var(--admin-accent-primary-soft);
		color: var(--admin-accent-primary);
		text-decoration: none;
		border-radius: var(--radius-md, 0.5rem);
		font-weight: 600;
		font-size: 0.9375rem;
		border: 1px solid var(--admin-border-interactive);
		transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
	}

	.view-site-btn:hover {
		background: var(--admin-accent-primary-muted);
		transform: translateY(-1px);
	}

	.view-site-btn:focus-visible {
		box-shadow: var(--admin-focus-ring);
		outline: none;
	}

	/* Main Content Area */
	.admin-content {
		flex: 1;
		padding: 2rem;
		overflow-y: auto;
		background: var(--admin-bg);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * RESPONSIVE BREAKPOINTS
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Tablet and below */
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

	/* Mobile landscape and below */
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

	/* Mobile portrait */
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

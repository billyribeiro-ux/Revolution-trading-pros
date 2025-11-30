<script lang="ts">
	/**
	 * Admin Layout - Dashboard shell for admin area
	 * Uses extracted AdminSidebar component for cleaner architecture
	 *
	 * Updated to Svelte 5 runes syntax (November 2025)
	 *
	 * @version 3.0.0
	 * @author Revolution Trading Pros
	 */
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { isAuthenticated } from '$lib/stores/auth';
	import { IconMenu2 } from '@tabler/icons-svelte';
	import { browser } from '$app/environment';
	import { AdminSidebar } from '$lib/components/layout';
	import Toast from '$lib/components/Toast.svelte';

	// Svelte 5 state rune
	let isSidebarOpen = $state(false);

	// Check if user is admin - Svelte 5 effect
	$effect(() => {
		if (browser && !$isAuthenticated) {
			goto('/login?redirect=/admin');
		}
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
			'videos': 'Videos'
		};
		
		return titleMap[lastSegment] || lastSegment.charAt(0).toUpperCase() + lastSegment.slice(1).replace(/-/g, ' ');
	}
</script>

<svelte:head>
	<title>Admin Dashboard | Revolution Trading Pros</title>
</svelte:head>

<div class="admin-layout">
	<!-- Sidebar Component -->
	<AdminSidebar isOpen={isSidebarOpen} on:close={closeSidebar} />

	<!-- Main Content -->
	<div class="admin-main">
		<!-- Top Bar -->
		<header class="admin-header">
			<button class="mobile-menu-btn" on:click={toggleSidebar}>
				<IconMenu2 size={24} />
			</button>
			<div class="header-title">
				<h1>{formatPageTitle($page.url.pathname)}</h1>
			</div>
			<div class="header-actions">
				<a href="/" class="view-site-btn" target="_blank">View Site</a>
			</div>
		</header>

		<!-- Page Content -->
		<main class="admin-content">
			<slot />
		</main>
	</div>
</div>

<!-- Toast Notifications -->
<Toast />

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
	}
</style>

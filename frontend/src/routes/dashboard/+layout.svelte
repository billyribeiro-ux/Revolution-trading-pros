<script lang="ts">
	/**
	 * Dashboard Layout - Svelte 5 / SvelteKit Implementation
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Exact match of WordPress Revolution Trading dashboard structure:
	 * - .dashboard (root container)
	 * - .dashboard__sidebar (aside - left navigation)
	 * - .dashboard__main (main content area)
	 * - .dashboard__header (page header with title + actions)
	 * - .dashboard__content (content wrapper)
	 * - .dashboard__overlay (mobile menu overlay)
	 * - .dashboard__toggle (mobile toggle button)
	 *
	 * @version 4.0.0 (WordPress-exact / December 2025)
	 */

	import { browser } from '$app/environment';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { authStore, isAuthenticated, user } from '$lib/stores/auth';
	import { getUserMemberships, type UserMembership } from '$lib/api/user-memberships';
	import DashboardSidebar from '$lib/components/dashboard/DashboardSidebar.svelte';
	import MobileToggle from '$lib/components/dashboard/MobileToggle.svelte';
	import type { Snippet } from 'svelte';

	// ═══════════════════════════════════════════════════════════════════════════
	// TYPES & PROPS
	// ═══════════════════════════════════════════════════════════════════════════

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE (Svelte 5 Runes)
	// ═══════════════════════════════════════════════════════════════════════════

	let memberships = $state<UserMembership[]>([]);
	let isLoading = $state(true);
	let isSidebarOpen = $state(false);
	let error = $state<string | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	const userName = $derived($user?.name || ($user?.email ? $user.email.split('@')[0] : null) || 'My Account');
	const userEmail = $derived($user?.email || '');
	const userAvatar = $derived($user?.avatar || '');
	const currentPath = $derived($page.url.pathname);

	// ═══════════════════════════════════════════════════════════════════════════
	// EFFECTS
	// ═══════════════════════════════════════════════════════════════════════════

	// Authentication guard and data loading
	$effect(() => {
		if (!browser) return;
		if ($authStore.isInitializing) return;

		if (!$isAuthenticated) {
			const redirectUrl = encodeURIComponent($page.url.pathname + $page.url.search);
			goto(`/login?redirect=${redirectUrl}`, { replaceState: true });
			return;
		}

		loadMemberships();
	});

	// Close sidebar on route change
	$effect(() => {
		if (browser && isSidebarOpen && currentPath) {
			closeSidebar();
		}
	});

	// Handle body scroll lock (WordPress: html--dashboard-menu-open)
	$effect(() => {
		if (!browser) return;

		if (isSidebarOpen) {
			document.documentElement.classList.add('html--dashboard-menu-open');
			document.body.style.overflow = 'hidden';
		} else {
			document.documentElement.classList.remove('html--dashboard-menu-open');
			document.body.style.overflow = '';
		}

		return () => {
			document.documentElement.classList.remove('html--dashboard-menu-open');
			document.body.style.overflow = '';
		};
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	async function loadMemberships(): Promise<void> {
		if (!$isAuthenticated) return;

		isLoading = true;
		error = null;

		try {
			const data = await getUserMemberships();
			memberships = data.memberships;
		} catch (e) {
			console.error('Failed to load memberships:', e);
			error = e instanceof Error ? e.message : 'Failed to load memberships';
		} finally {
			isLoading = false;
		}
	}

	function toggleSidebar(): void {
		isSidebarOpen = !isSidebarOpen;
	}

	function closeSidebar(): void {
		isSidebarOpen = false;
	}
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     HEAD
     ═══════════════════════════════════════════════════════════════════════════ -->

<svelte:head>
	<title>{$page.data?.title || 'Dashboard'} | Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<!-- ═══════════════════════════════════════════════════════════════════════════
     TEMPLATE - WordPress Exact Structure
     ═══════════════════════════════════════════════════════════════════════════ -->

{#if $isAuthenticated || $authStore.isInitializing}
	<!-- WordPress: .dashboard (root container) -->
	<div class="dashboard" class:dashboard--menu-open={isSidebarOpen}>

		<!-- WordPress: .dashboard__sidebar (aside) -->
		<aside class="dashboard__sidebar" class:is-open={isSidebarOpen}>
			<DashboardSidebar
				{memberships}
				bind:isMobileOpen={isSidebarOpen}
				onCloseMobile={closeSidebar}
				{userName}
				{userEmail}
				{userAvatar}
			/>
		</aside>

		<!-- WordPress: .dashboard__main -->
		<main class="dashboard__main" aria-label="Dashboard content">
			{#if isLoading}
				<div class="dashboard-loading" aria-busy="true" aria-label="Loading dashboard">
					<div class="loading-spinner"></div>
					<p>Loading your dashboard...</p>
				</div>
			{:else if error}
				<div class="dashboard-error" role="alert">
					<div class="error-icon">!</div>
					<p class="error-message">{error}</p>
					<button class="retry-btn" onclick={loadMemberships}>
						Try Again
					</button>
				</div>
			{:else}
				{@render children()}
			{/if}
		</main>

		<!-- WordPress: .dashboard__overlay -->
		{#if isSidebarOpen}
			<button
				class="dashboard__overlay"
				onclick={closeSidebar}
				aria-label="Close navigation"
				tabindex="-1"
				data-toggle-dashboard-menu
			></button>
		{/if}

		<!-- WordPress: .dashboard__toggle (footer positioned) -->
		<footer class="dashboard__toggle">
			<MobileToggle
				bind:isOpen={isSidebarOpen}
				onToggle={toggleSidebar}
				position="left"
			/>
		</footer>
	</div>
{:else}
	<div class="dashboard-loading" aria-busy="true">
		<div class="loading-spinner"></div>
		<p>Redirecting to login...</p>
	</div>
{/if}

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES - WordPress Exact CSS
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   CSS CUSTOM PROPERTIES (WordPress Reference)
	   ═══════════════════════════════════════════════════════════════════════════ */

	:root {
		--dashboard-bg: #f4f4f4;
		--dashboard-sidebar-bg: #0f2d41;
		--dashboard-sidebar-width: 280px;
		--dashboard-header-height: 80px;
		--dashboard-content-min-height: calc(100vh - var(--dashboard-header-height));
		--dashboard-transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
		--loading-color: #0984ae;
		--error-color: #ef4444;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD ROOT (WordPress: .dashboard)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard {
		display: flex;
		flex-flow: row nowrap;
		min-height: 100vh;
		background: var(--dashboard-bg);
		position: relative;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD SIDEBAR (WordPress: .dashboard__sidebar)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__sidebar {
		flex: 0 0 var(--dashboard-sidebar-width);
		width: var(--dashboard-sidebar-width);
		background-color: var(--dashboard-sidebar-bg);
		min-height: 100vh;
		position: sticky;
		top: 0;
		z-index: 100;
		transition: var(--dashboard-transition);
		overflow-y: auto;
		overflow-x: hidden;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD MAIN (WordPress: .dashboard__main)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__main {
		flex: 1 1 auto;
		width: calc(100% - var(--dashboard-sidebar-width));
		min-height: var(--dashboard-content-min-height);
		background: var(--dashboard-bg);
		position: relative;
		transition: var(--dashboard-transition);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD OVERLAY (WordPress: .dashboard__overlay)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__overlay {
		background-color: rgba(0, 0, 0, 0.65);
		bottom: 0;
		left: 0;
		position: fixed;
		right: 0;
		top: 0;
		z-index: 100009;
		border: none;
		cursor: pointer;
		-webkit-appearance: none;
		-moz-appearance: none;
		appearance: none;
		transition: opacity 0.3s ease-in-out;
		display: none;
	}

	.dashboard__overlay:focus {
		outline: none;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD TOGGLE FOOTER (WordPress: .dashboard__toggle)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__toggle {
		display: none;
		position: fixed;
		bottom: 0;
		left: 0;
		right: 0;
		z-index: 100008;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   LOADING STATE
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard-loading {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 400px;
		color: #666;
		gap: 16px;
	}

	.loading-spinner {
		width: 44px;
		height: 44px;
		border: 3px solid #e5e7eb;
		border-top-color: var(--loading-color);
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	.dashboard-loading p {
		font-size: 14px;
		color: #6b7280;
		margin: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ERROR STATE
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard-error {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 400px;
		gap: 16px;
		padding: 24px;
		text-align: center;
	}

	.error-icon {
		width: 48px;
		height: 48px;
		border-radius: 50%;
		background: #fef2f2;
		color: var(--error-color);
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 24px;
		font-weight: 700;
	}

	.error-message {
		color: #374151;
		font-size: 14px;
		margin: 0;
		max-width: 300px;
	}

	.retry-btn {
		background: var(--loading-color);
		color: white;
		border: none;
		padding: 10px 24px;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 500;
		cursor: pointer;
		transition: background 0.2s ease;
	}

	.retry-btn:hover {
		background: #076787;
	}

	.retry-btn:focus-visible {
		outline: 2px solid var(--loading-color);
		outline-offset: 2px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE - MOBILE (WordPress Reference: max-width 980px)
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media screen and (max-width: 980px) {
		.dashboard__sidebar {
			position: fixed;
			top: 0;
			left: 0;
			bottom: 0;
			width: 70%;
			max-width: 280px;
			transform: translateX(-100%);
			opacity: 0;
			visibility: hidden;
			z-index: 100010;
		}

		.dashboard__sidebar.is-open {
			transform: translateX(0);
			opacity: 1;
			visibility: visible;
		}

		.dashboard__main {
			width: 100%;
			padding-bottom: 70px; /* Space for mobile toggle */
		}

		.dashboard__overlay {
			display: block;
		}

		.dashboard__toggle {
			display: block;
		}
	}

	@media screen and (max-width: 480px) {
		.dashboard__sidebar {
			width: 85%;
			max-width: none;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   GLOBAL STYLES (WordPress: html--dashboard-menu-open)
	   ═══════════════════════════════════════════════════════════════════════════ */

	:global(.html--dashboard-menu-open) {
		overflow: hidden;
	}

	:global(.html--dashboard-menu-open body) {
		overflow: hidden;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   REDUCED MOTION
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.dashboard__sidebar,
		.dashboard__main,
		.dashboard__overlay,
		.loading-spinner {
			transition: none;
			animation: none;
		}
	}
</style>

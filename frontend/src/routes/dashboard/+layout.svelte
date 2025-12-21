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
	<!-- WordPress EXACT: .dashboard (root container) -->
	<div class="dashboard" class:dashboard--menu-open={isSidebarOpen}>

		<!-- WordPress EXACT: .dashboard__sidebar (aside) - Contains nav, toggle, and overlay -->
		<aside class="dashboard__sidebar" class:is-open={isSidebarOpen}>
			<DashboardSidebar
				{memberships}
				bind:isMobileOpen={isSidebarOpen}
				onCloseMobile={closeSidebar}
				{userName}
				{userEmail}
				{userAvatar}
			/>

			<!-- WordPress EXACT: .dashboard__toggle (footer inside sidebar) -->
			<footer class="dashboard__toggle">
				<button
					class="dashboard__toggle-button"
					onclick={toggleSidebar}
					data-toggle-dashboard-menu
					aria-label={isSidebarOpen ? 'Close Dashboard Menu' : 'Open Dashboard Menu'}
					aria-expanded={isSidebarOpen}
				>
					<div class="dashboard__toggle-button-icon">
						<span></span>
						<span></span>
						<span></span>
					</div>
					<span class="framework__toggle-button-label">Dashboard Menu</span>
				</button>
			</footer>

			<!-- WordPress EXACT: .dashboard__overlay (inside sidebar) -->
			<div
				class="dashboard__overlay"
				class:is-visible={isSidebarOpen}
				onclick={closeSidebar}
				onkeydown={(e) => e.key === 'Enter' && closeSidebar()}
				role="button"
				tabindex={isSidebarOpen ? 0 : -1}
				aria-label="Close navigation"
				data-toggle-dashboard-menu
			></div>
		</aside>

		<!-- WordPress EXACT: .dashboard__main -->
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

	/* WordPress EXACT: CSS Custom Properties */
	:root {
		--dashboard-bg: #f4f4f4;
		--dashboard-sidebar-bg: #0f2d41;
		--dashboard-toggle-bg: #0d2532;
		--dashboard-sidebar-width: 280px;
		--dashboard-toggle-height: 50px;
		--dashboard-transition: all 0.3s ease-in-out;
		--loading-color: #0984ae;
		--error-color: #ef4444;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD ROOT (WordPress EXACT: .dashboard)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard {
		display: flex;
		flex-flow: row nowrap;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD SIDEBAR (WordPress EXACT: .dashboard__sidebar)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__sidebar {
		display: flex;
		flex: 0 0 auto;
		flex-flow: row nowrap;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD MAIN (WordPress EXACT: .dashboard__main)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__main {
		flex: 1 1 auto;
		min-width: 0;
		background-color: var(--dashboard-bg);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD OVERLAY (WordPress EXACT: .dashboard__overlay)
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
		opacity: 0;
		visibility: hidden;
		transition: opacity 0.3s ease-in-out, visibility 0.3s ease-in-out;
	}

	.dashboard__overlay.is-visible {
		opacity: 1;
		visibility: visible;
	}

	.dashboard__overlay:focus {
		outline: none;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD TOGGLE FOOTER (WordPress EXACT: .dashboard__toggle)
	   Full-width footer at bottom - sidebar ends where this starts
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__toggle {
		background-color: var(--dashboard-toggle-bg);
		height: var(--dashboard-toggle-height);
		position: fixed;
		bottom: 0;
		left: 0;
		right: 0;
		z-index: 100011; /* Above sidebar (100010) so footer is on top */
	}

	/* WordPress EXACT: .dashboard__toggle-button */
	.dashboard__toggle-button {
		background: none;
		color: #fff;
		height: var(--dashboard-toggle-height);
		padding: 0 20px 0 60px;
		position: relative;
		border: none;
		cursor: pointer;
		display: flex;
		align-items: center;
		gap: 12px;
		width: 100%;
	}

	/* WordPress EXACT: .dashboard__toggle-button-icon (hamburger) */
	.dashboard__toggle-button-icon {
		position: absolute;
		top: 50%;
		left: 20px;
		margin-top: -9px;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		width: 24px;
		height: 18px;
	}

	.dashboard__toggle-button-icon span {
		display: block;
		width: 100%;
		height: 2px;
		background: #fff;
		border-radius: 1px;
		transition: var(--dashboard-transition);
	}

	/* Animate to X when open */
	.dashboard--menu-open .dashboard__toggle-button-icon span:nth-child(1) {
		transform: translateY(8px) rotate(45deg);
	}

	.dashboard--menu-open .dashboard__toggle-button-icon span:nth-child(2) {
		opacity: 0;
	}

	.dashboard--menu-open .dashboard__toggle-button-icon span:nth-child(3) {
		transform: translateY(-8px) rotate(-45deg);
	}

	/* WordPress EXACT: .framework__toggle-button-label */
	.framework__toggle-button-label {
		font-size: 14px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.5px;
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
	   RESPONSIVE - DESKTOP (WordPress EXACT: min-width 1280px)
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media screen and (min-width: 1280px) {
		/* Hide toggle on desktop */
		.dashboard__toggle {
			display: none;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE - MOBILE (WordPress EXACT: below 1280px)
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media screen and (max-width: 1279px) {
		.dashboard__main {
			padding-bottom: var(--dashboard-toggle-height);
		}
	}

	@media screen and (max-width: 480px) {
		/* Mobile-specific adjustments if needed */
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
		.dashboard__toggle-button,
		.dashboard__toggle-button-icon span,
		.loading-spinner {
			transition: none;
			animation: none;
		}
	}
</style>

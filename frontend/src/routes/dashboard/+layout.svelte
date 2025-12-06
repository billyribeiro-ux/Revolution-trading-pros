<script lang="ts">
	/**
	 * Dashboard Layout - Svelte 5 / SvelteKit Implementation
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Wraps all dashboard routes with:
	 * - Sidebar navigation with mobile support
	 * - Authentication guard
	 * - User memberships loading with caching
	 * - Svelte 5 runes
	 *
	 * @version 3.0.0 (Svelte 5 / December 2025)
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
	let isSidebarCollapsed = $state(false);
	let error = $state<string | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	const userName = $derived($user?.name || $user?.email?.split('@')[0] || 'My Account');
	const userAvatar = $derived($user?.avatar || '');
	const currentPath = $derived($page.url.pathname);

	// ═══════════════════════════════════════════════════════════════════════════
	// EFFECTS
	// ═══════════════════════════════════════════════════════════════════════════

	// Authentication guard and data loading
	$effect(() => {
		// Skip during SSR
		if (!browser) return;

		// Wait for auth initialization
		if ($authStore.isInitializing) return;

		// Redirect to login if not authenticated
		if (!$isAuthenticated) {
			const redirectUrl = encodeURIComponent($page.url.pathname + $page.url.search);
			goto(`/login?redirect=${redirectUrl}`, { replaceState: true });
			return;
		}

		// Load memberships
		loadMemberships();
	});

	// Close sidebar on route change
	$effect(() => {
		if (browser && isSidebarOpen && currentPath) {
			closeSidebar();
		}
	});

	// Handle body scroll lock
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

	function handleSidebarCollapseToggle(): void {
		isSidebarCollapsed = !isSidebarCollapsed;
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
     TEMPLATE
     ═══════════════════════════════════════════════════════════════════════════ -->

{#if $isAuthenticated || $authStore.isInitializing}
	<div class="woocommerce-account woocommerce-page logged-in">
		<div class="woocommerce">
			<!-- Sidebar Navigation -->
			<DashboardSidebar
				{memberships}
				bind:isCollapsed={isSidebarCollapsed}
				bind:isMobileOpen={isSidebarOpen}
				onToggleCollapse={handleSidebarCollapseToggle}
				onCloseMobile={closeSidebar}
				{userAvatar}
				{userName}
			/>

			<!-- Main Content Area -->
			<main
				class="woocommerce-MyAccount-content"
				class:sidebar-collapsed={isSidebarCollapsed}
				role="main"
				aria-label="Dashboard content"
			>
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
	</div>

	<!-- Mobile Toggle Button -->
	<MobileToggle
		bind:isOpen={isSidebarOpen}
		onToggle={toggleSidebar}
		position="left"
	/>
{:else}
	<div class="dashboard-loading" aria-busy="true">
		<div class="loading-spinner"></div>
		<p>Redirecting to login...</p>
	</div>
{/if}

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   CSS CUSTOM PROPERTIES
	   ═══════════════════════════════════════════════════════════════════════════ */

	:root {
		--dashboard-bg: #f4f4f4;
		--dashboard-content-min-height: calc(100vh - 80px);
		--dashboard-transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
		--loading-color: #0984ae;
		--error-color: #ef4444;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MAIN LAYOUT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.woocommerce-account {
		min-height: 100vh;
		background: var(--dashboard-bg);
	}

	.woocommerce-account .woocommerce {
		display: flex;
		margin: 0 auto;
		width: 100%;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MAIN CONTENT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.woocommerce-MyAccount-content {
		flex: 1;
		width: 100%;
		background: var(--dashboard-bg);
		position: relative;
		min-height: var(--dashboard-content-min-height);
		transition: var(--dashboard-transition);
	}

	.woocommerce-MyAccount-content.sidebar-collapsed {
		/* Adjust content when sidebar is collapsed */
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
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media screen and (max-width: 980px) {
		.woocommerce-MyAccount-content {
			width: 100%;
			padding-bottom: 70px; /* Space for mobile toggle */
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   GLOBAL STYLES
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
		.woocommerce-MyAccount-content,
		.loading-spinner {
			transition: none;
			animation: none;
		}
	}
</style>

<!--
	Dashboard Layout - Member Dashboard with Sidebar
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation

	CLIENT-SIDE AUTH PATTERN:
	- Auth is checked client-side using isAuthenticated store
	- Redirects to login if not authenticated
	- Fetches memberships data client-side when authenticated
	- Shows loading skeleton during auth check

	Svelte 5 Features:
	- $state() for reactive state management
	- $effect() for side effects (auth check, data fetching)
	- $derived() for computed values
	- Snippet for children rendering

	@version 3.0.0 - ICT 11+ Client-Side Auth
	@author Revolution Trading Pros
-->
<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { browser } from '$app/environment';
	import { onMount, type Snippet } from 'svelte';
	import { isAuthenticated, isInitializing, user, authStore } from '$lib/stores/auth';
	import { getUserMemberships, type CategorizedMemberships } from '$lib/api/user-memberships';
	import DashboardSidebar from '$lib/components/dashboard/DashboardSidebar.svelte';

	// ═══════════════════════════════════════════════════════════════════════════
	// PROPS
	// ═══════════════════════════════════════════════════════════════════════════

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	// Auth state
	let isCheckingAuth = $state(true);
	let isLoadingData = $state(false);

	// Sidebar state
	let sidebarCollapsed = $state(false);

	// Memberships data
	let membershipsData = $state<CategorizedMemberships | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	// User data for sidebar
	const userData = $derived({
		id: $user?.id?.toString() ?? '',
		email: $user?.email ?? '',
		name: $user?.name ?? $user?.email?.split('@')[0] ?? 'Member',
		avatar: $user?.avatar_url ?? null,
		memberships: membershipsData?.memberships
			?.filter(m => m.status === 'active')
			?.map(m => m.slug) ?? []
	});

	// Show content only when authenticated
	const showContent = $derived(!isCheckingAuth && $isAuthenticated);

	// ═══════════════════════════════════════════════════════════════════════════
	// AUTH CHECK & DATA LOADING
	// ICT11+ Pattern: Wait for root layout auth initialization to complete
	// ═══════════════════════════════════════════════════════════════════════════

	onMount(() => {
		checkAuthAndLoadData();
	});

	async function checkAuthAndLoadData(): Promise<void> {
		if (!browser) return;

		isCheckingAuth = true;

		// ICT11+ Pattern: Wait for auth initialization to complete (max 6 seconds)
		// The root layout's initializeAuth() sets isInitializing to false when done
		const maxWaitTime = 6000;
		const checkInterval = 100;
		let waited = 0;

		while ($isInitializing && waited < maxWaitTime) {
			await new Promise(resolve => setTimeout(resolve, checkInterval));
			waited += checkInterval;
		}

		// Check if authenticated after initialization completes
		if (!$isAuthenticated) {
			// Not authenticated - redirect to login
			const currentPath = page?.url?.pathname ?? '/dashboard';
			const redirectUrl = `/login?redirect=${encodeURIComponent(currentPath)}`;
			await goto(redirectUrl, { replaceState: true });
			return;
		}

		isCheckingAuth = false;

		// Load memberships data
		await loadMembershipsData();
	}

	async function loadMembershipsData(): Promise<void> {
		if (!$isAuthenticated) return;

		isLoadingData = true;

		try {
			membershipsData = await getUserMemberships();
		} catch (error) {
			console.error('[Dashboard] Failed to load memberships:', error);
			membershipsData = null;
		} finally {
			isLoadingData = false;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// WATCH AUTH CHANGES
	// ICT11+ Pattern: Only redirect after initialization is complete
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		// If user logs out while on dashboard, redirect to login
		// Only redirect if not currently initializing or checking auth
		if (!isCheckingAuth && !$isInitializing && !$isAuthenticated && browser) {
			const currentPath = page?.url?.pathname ?? '/dashboard';
			goto(`/login?redirect=${encodeURIComponent(currentPath)}`, { replaceState: true });
		}
	});
</script>

<svelte:head>
	<meta name="robots" content="noindex, nofollow" />
	<title>Member Dashboard | Revolution Trading Pros</title>
	<link href="https://fonts.googleapis.com/css?family=Open+Sans:300,regular,600,700" rel="stylesheet" />
</svelte:head>

<!-- Loading State -->
{#if isCheckingAuth}
	<div class="dashboard-loading">
		<div class="dashboard-loading__container">
			<div class="dashboard-loading__spinner"></div>
			<p class="dashboard-loading__text">Loading your dashboard...</p>
		</div>
	</div>

<!-- Authenticated Content -->
{:else if showContent}
	<div class="dashboard">
		<!-- Sidebar Navigation with bindable collapsed state -->
		<DashboardSidebar user={userData} bind:collapsed={sidebarCollapsed} />

		<!-- Main Content Area - margin adjusts based on sidebar state -->
		<main class="dashboard__main" class:is-collapsed={sidebarCollapsed}>
			{#if isLoadingData}
				<div class="dashboard__loading-overlay">
					<div class="dashboard__loading-spinner"></div>
				</div>
			{/if}
			{@render children()}
		</main>
	</div>
{/if}

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * Loading State
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard-loading {
		display: flex;
		align-items: center;
		justify-content: center;
		min-height: 100vh;
		background-color: #efefef;
	}

	.dashboard-loading__container {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1rem;
	}

	.dashboard-loading__spinner {
		width: 48px;
		height: 48px;
		border: 4px solid #e0e0e0;
		border-top-color: #0a2335;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	.dashboard-loading__text {
		color: #666;
		font-size: 1rem;
		font-family: 'Open Sans', sans-serif;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Dashboard Layout Container
	 * Matches WordPress Simpler Trading reference
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard {
		display: flex;
		min-height: 100vh;
		position: relative;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Main Content Area
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__main {
		flex: 1;
		margin-left: 280px;
		min-height: 100vh;
		background-color: #efefef;
		transition: margin-left 0.3s cubic-bezier(0.4, 0, 0.2, 1);
		position: relative;
	}

	/* Collapsed sidebar state - main content shifts */
	.dashboard__main.is-collapsed {
		margin-left: 80px;
	}

	/* Loading overlay for data fetching */
	.dashboard__loading-overlay {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: rgba(239, 239, 239, 0.8);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 10;
	}

	.dashboard__loading-spinner {
		width: 32px;
		height: 32px;
		border: 3px solid #e0e0e0;
		border-top-color: #0a2335;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Responsive - Mobile (≤768px)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 768px) {
		.dashboard__main {
			margin-left: 0;
			padding-top: 64px; /* Space for mobile toggle button */
		}

		.dashboard__main.is-collapsed {
			margin-left: 0;
		}

		/* Remove extra padding when content has its own header */
		.dashboard__main:has(.dashboard__header) {
			padding-top: 64px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Responsive - Tablet (769-1024px)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	@media (min-width: 769px) and (max-width: 1024px) {
		.dashboard__main:not(.is-collapsed) {
			margin-left: 240px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Reduced Motion
	 * ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.dashboard__main {
			transition: none;
		}

		.dashboard-loading__spinner,
		.dashboard__loading-spinner {
			animation: none;
		}
	}
</style>

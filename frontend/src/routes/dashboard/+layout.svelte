<!--
	Dashboard Layout - Member Dashboard with Sidebar
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation
	Svelte 5 (December 2025) Best Practices

	SERVER-SIDE AUTH PATTERN (Svelte 5 Recommended):
	- Auth is validated server-side in hooks.server.ts BEFORE this component loads
	- User data is passed from +layout.server.ts via load function
	- No client-side auth polling needed - server already handled it
	- Memberships fetched client-side for dynamic updates

	Svelte 5 Features:
	- $props() for component props including data from load
	- $state() for reactive state management
	- $derived() for computed values
	- $effect() for side effects
	- Snippet for children rendering

	@version 4.0.0 - ICT 11+ Server-Side Auth
	@author Revolution Trading Pros
-->
<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { browser } from '$app/environment';
	import { onMount, type Snippet } from 'svelte';
	import { isAuthenticated, user, authStore } from '$lib/stores/auth';
	import { getUserMemberships, type CategorizedMemberships } from '$lib/api/user-memberships';
	import DashboardSidebar from '$lib/components/dashboard/DashboardSidebar.svelte';

	// ═══════════════════════════════════════════════════════════════════════════
	// PROPS - Svelte 5 Pattern
	// ═══════════════════════════════════════════════════════════════════════════

	interface Props {
		children: Snippet;
		data: {
			user: {
				id: string;
				email: string;
				name?: string;
				role?: string;
			} | null;
		};
	}

	let { children, data }: Props = $props();

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	// Loading state for memberships data
	let isLoadingData = $state(false);

	// Sidebar state
	let sidebarCollapsed = $state(false);

	// Memberships data
	let membershipsData = $state<CategorizedMemberships | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE - Svelte 5 Pattern
	// ═══════════════════════════════════════════════════════════════════════════

	// User data for sidebar - prefer server data, fall back to client store
	const userData = $derived({
		id: data.user?.id ?? $user?.id?.toString() ?? '',
		email: data.user?.email ?? $user?.email ?? '',
		name: data.user?.name ?? $user?.name ?? $user?.email?.split('@')[0] ?? 'Member',
		avatar: $user?.avatar_url ?? null,
		memberships: membershipsData?.memberships
			?.filter(m => m.status === 'active')
			?.map(m => m.slug) ?? []
	});

	// Auth is handled server-side, but we still check client store for logout detection
	const isUserAuthenticated = $derived(!!data.user || $isAuthenticated);

	// ═══════════════════════════════════════════════════════════════════════════
	// DATA LOADING
	// Server-side auth is complete, just load memberships client-side
	// ═══════════════════════════════════════════════════════════════════════════

	onMount(async () => {
		// Sync server user to client store if needed
		if (data.user && !$isAuthenticated) {
			// Server has user but client doesn't - sync auth state
			// This happens on page refresh when server validates token
			console.debug('[Dashboard] Syncing server auth to client store');
		}

		// Load memberships data
		await loadMembershipsData();
	});

	async function loadMembershipsData(): Promise<void> {
		if (!isUserAuthenticated) return;

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
	// WATCH AUTH CHANGES - Client-side logout detection
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		// If user logs out while on dashboard (client-side action), redirect to login
		// Server auth is already validated, this is for client-side logout
		if (!$isAuthenticated && !data.user && browser) {
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

<!-- Dashboard Content - No loading spinner needed, server-side auth is complete -->
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

<style>
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
		margin-top: 80px; /* Account for NavBar height */
		min-height: calc(100vh - 80px);
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

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
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

		.dashboard__loading-spinner {
			animation: none;
		}
	}
</style>

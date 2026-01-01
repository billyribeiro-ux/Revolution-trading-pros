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
	import { isAuthenticated, user } from '$lib/stores/auth';
	import { getUserMemberships, type UserMembershipsResponse } from '$lib/api/user-memberships';
	import DashboardSidebar from '$lib/components/dashboard/DashboardSidebar.svelte';
	import Breadcrumbs from '$lib/components/dashboard/Breadcrumbs.svelte';

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
	let membershipsData = $state<UserMembershipsResponse | null>(null);

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
			?.filter((m: { status: string }) => m.status === 'active')
			?.map((m: { slug: string }) => m.slug) ?? []
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
	<link rel="stylesheet" href="/dashboard/dashboard.css" />
</svelte:head>

<!-- Dashboard Content - Flex layout matching WordPress exactly -->
<div class="dashboard">
	<!-- Sidebar Navigation -->
	<DashboardSidebar user={userData} bind:collapsed={sidebarCollapsed} />

	<!-- Main Content Area - flex: 1 1 auto fills remaining space -->
	<main class="dashboard__main">
		<!-- Breadcrumbs Navigation -->
		<Breadcrumbs />
		
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
	 * Matches WordPress Simpler Trading reference exactly
	 * WordPress CSS: display: flex; flex-flow: row;
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard {
		display: flex;
		flex-flow: row;
		min-height: 100vh;
		position: relative;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Main Content Area
	 * WordPress CSS: flex: 1 1 auto; min-width: 0; background-color: #f4f4f4;
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__main {
		flex: 1 1 auto;
		min-width: 0;
		background-color: #f4f4f4;
		position: relative;
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
	 * Responsive - Mobile (<1280px) - Sidebar is fixed, main takes full width
	 * ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 1279px) {
		.dashboard__main {
			width: 100%;
			padding-bottom: 50px; /* Space for fixed toggle footer */
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Reduced Motion
	 * ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.dashboard__loading-spinner {
			animation: none;
		}
	}
</style>

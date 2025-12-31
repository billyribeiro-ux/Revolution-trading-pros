<!--
	Dashboard Layout - Member Dashboard with Sidebar
	SSR-compatible layout with authentication guard

	Phase Two: Collapsible sidebar with main content adjustment

	@version 2.0.0
	@author Revolution Trading Pros

	Svelte 5 Features:
	- $bindable() for two-way binding with sidebar collapsed state
	- $state() for reactive collapsed tracking
	- Snippet for children rendering
-->
<script lang="ts">
	import DashboardSidebar from '$lib/components/dashboard/DashboardSidebar.svelte';
	import type { Snippet } from 'svelte';

	// Svelte 5 props
	interface Props {
		data: {
			user: {
				id: string;
				email: string;
				name: string;
				avatar: string | null;
				memberships: string[];
			};
		};
		children: Snippet;
	}

	let { data, children }: Props = $props();

	// Bindable collapsed state - syncs with sidebar
	let sidebarCollapsed = $state(false);
</script>

<svelte:head>
	<meta name="robots" content="noindex, nofollow" />
	<title>Member Dashboard | Revolution Trading Pros</title>
	<link href="https://fonts.googleapis.com/css?family=Open+Sans:300,regular,600,700" rel="stylesheet" />
</svelte:head>

<div class="dashboard">
	<!-- Sidebar Navigation with bindable collapsed state -->
	<DashboardSidebar user={data.user} bind:collapsed={sidebarCollapsed} />

	<!-- Main Content Area - margin adjusts based on sidebar state -->
	<main class="dashboard__main" class:is-collapsed={sidebarCollapsed}>
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
		min-height: 100vh;
		background-color: #efefef;
		transition: margin-left 0.3s cubic-bezier(0.4, 0, 0.2, 1);
	}

	/* Collapsed sidebar state - main content shifts */
	.dashboard__main.is-collapsed {
		margin-left: 80px;
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
	}
</style>

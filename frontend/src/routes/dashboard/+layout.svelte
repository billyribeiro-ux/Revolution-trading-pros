<!--
	Dashboard Layout - Member Dashboard with Sidebar
	SSR-compatible layout with authentication guard

	@version 1.0.0
	@author Revolution Trading Pros
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
</script>

<svelte:head>
	<meta name="robots" content="noindex, nofollow" />
	<title>Member Dashboard | Revolution Trading Pros</title>
</svelte:head>

<div class="dashboard">
	<!-- Sidebar Navigation -->
	<DashboardSidebar user={data.user} />

	<!-- Main Content Area -->
	<main class="dashboard__main">
		{@render children()}
	</main>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * Dashboard Layout Container
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard {
		display: flex;
		min-height: 100vh;
		background-color: #0f172a;
		color: #ffffff;
	}

	/* Main Content Area */
	.dashboard__main {
		flex: 1;
		margin-left: 260px;
		min-height: 100vh;
		background: linear-gradient(135deg, #0f172a 0%, #1e293b 50%, #0f172a 100%);
		transition: margin-left 0.3s cubic-bezier(0.4, 0, 0.2, 1);
	}

	/* When sidebar is collapsed */
	:global(.dashboard__sidebar.is-collapsed) ~ .dashboard__main {
		margin-left: 80px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Responsive Styles
	 * ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 1024px) {
		.dashboard__main {
			margin-left: 0;
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

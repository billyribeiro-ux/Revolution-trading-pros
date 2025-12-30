<script lang="ts">
	/**
	 * Dashboard Layout - Main Shell
	 *
	 * Svelte 5 Best Practices:
	 * - SSR-safe with proper hydration
	 * - Uses $props() for data from load function
	 * - Uses $state() for client-side UI state
	 * - Imports global dashboard styles
	 *
	 * Structure matches Simpler Trading reference (core file)
	 */

	import '$lib/styles/dashboard.css';
	import { DashboardSidebar } from '$lib/components/dashboard';
	import { browser } from '$app/environment';
	import { onMount } from 'svelte';
	import type { Snippet } from 'svelte';

	// Props from +layout.ts load function
	interface Props {
		data: {
			user: {
				id: number;
				name: string;
				email: string;
				avatar?: string;
			};
		};
		children: Snippet;
	}

	let { data, children }: Props = $props();

	// UI State - client-side only for hydration safety
	let sidebarCollapsed = $state(false);
	let sidebarOpen = $state(false);
	let mounted = $state(false);

	// Mock memberships - in production, fetch from API
	const memberships = [
		{ slug: 'mastering-the-trade', name: 'Mastering the Trade' },
		{ slug: 'simpler-showcase', name: 'Simpler Showcase' }
	];

	// Toggle sidebar collapse (desktop)
	function toggleSidebarCollapse() {
		sidebarCollapsed = !sidebarCollapsed;
		// Persist preference
		if (browser) {
			localStorage.setItem('dashboard-sidebar-collapsed', String(sidebarCollapsed));
		}
	}

	// Toggle sidebar open (mobile)
	function toggleSidebarOpen() {
		sidebarOpen = !sidebarOpen;
		// Prevent body scroll when sidebar is open on mobile
		if (browser) {
			document.body.style.overflow = sidebarOpen ? 'hidden' : '';
		}
	}

	// Restore sidebar state from localStorage
	onMount(() => {
		mounted = true;
		if (browser) {
			const savedCollapsed = localStorage.getItem('dashboard-sidebar-collapsed');
			if (savedCollapsed === 'true') {
				sidebarCollapsed = true;
			}
		}
	});

	// Close mobile sidebar on route change
	$effect(() => {
		if (browser && sidebarOpen) {
			sidebarOpen = false;
			document.body.style.overflow = '';
		}
	});
</script>

<div class="dashboard" class:sidebar-collapsed={sidebarCollapsed}>
	<!-- Sidebar Navigation -->
	<DashboardSidebar
		user={{
			name: data.user?.name || 'Member',
			email: data.user?.email || '',
			avatar: data.user?.avatar
		}}
		{memberships}
		isCollapsed={sidebarCollapsed}
		isOpen={sidebarOpen}
		onToggleCollapse={toggleSidebarCollapse}
		onToggleOpen={toggleSidebarOpen}
	/>

	<!-- Main Content Area -->
	<main class="dashboard__main" class:sidebar-collapsed={sidebarCollapsed}>
		{@render children()}
	</main>
</div>

<style>
	/* Layout-specific overrides */
	.dashboard {
		/* Ensure proper stacking context */
		position: relative;
		isolation: isolate;
	}

	/* Smooth transitions when sidebar collapses */
	.dashboard.sidebar-collapsed .dashboard__main {
		margin-left: var(--dashboard-sidebar-collapsed);
	}

	@media (max-width: 991px) {
		.dashboard.sidebar-collapsed .dashboard__main {
			margin-left: 0;
		}
	}
</style>

<script lang="ts">
	/**
	 * Analytics Layout - Shared navigation for analytics section
	 * Updated to Svelte 5 syntax (November 2025)
	 */
	import { page } from '$app/state';
	import type { Snippet } from 'svelte';

	// Svelte 5: Props - no destructuring
	interface Props {
		children: Snippet;
	}
	let props: Props = $props();

	// FIX-2026-04-26 (audit 08-analytics §P1-7): added goals, heatmaps,
	// recordings — all three exist as `+page.svelte` siblings but were
	// previously only reachable via the dashboard's Quick Actions panel or
	// deep links from elsewhere.
	const navItems = [
		{ href: '/admin/analytics', label: 'Dashboard', icon: '📊' },
		{ href: '/admin/analytics/events', label: 'Events', icon: '⚡' },
		{ href: '/admin/analytics/funnels', label: 'Funnels', icon: '🔻' },
		{ href: '/admin/analytics/cohorts', label: 'Cohorts', icon: '👥' },
		{ href: '/admin/analytics/segments', label: 'Segments', icon: '🎯' },
		{ href: '/admin/analytics/attribution', label: 'Attribution', icon: '📍' },
		{ href: '/admin/analytics/goals', label: 'Goals', icon: '🎯' },
		{ href: '/admin/analytics/heatmaps', label: 'Heatmaps', icon: '🔥' },
		{ href: '/admin/analytics/recordings', label: 'Recordings', icon: '🎥' },
		{ href: '/admin/analytics/reports', label: 'Reports', icon: '📈' }
	];

	// Svelte 5: Derived state
	let currentPath = $derived(page.url.pathname);
</script>

<div class="bg-gray-50">
	<!-- Secondary Navigation -->
	<div class="bg-white border-b border-gray-200 sticky top-0 z-10">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
			<nav class="flex items-center gap-1 -mb-px overflow-x-auto">
				{#each navItems as item (item.href)}
					<a
						href={item.href}
						class="flex items-center gap-2 px-4 py-4 text-sm font-medium border-b-2 whitespace-nowrap transition-colors
							{currentPath === item.href
							? 'border-blue-600 text-blue-600'
							: 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'}"
					>
						<span>{item.icon}</span>
						{item.label}
					</a>
				{/each}
			</nav>
		</div>
	</div>

	{@render props.children()}
</div>

<script lang="ts">
	/**
	 * Analytics Layout - Shared navigation for the analytics section.
	 * Uses the admin token layer (no Tailwind utilities, no emoji icons).
	 */
	import { page } from '$app/state';
	import type { Snippet } from 'svelte';
	import IconChartBar from '@tabler/icons-svelte-runes/icons/chart-bar';
	import IconBolt from '@tabler/icons-svelte-runes/icons/bolt';
	import IconFilter from '@tabler/icons-svelte-runes/icons/filter';
	import IconUsers from '@tabler/icons-svelte-runes/icons/users';
	import IconUsersGroup from '@tabler/icons-svelte-runes/icons/users-group';
	import IconRoute from '@tabler/icons-svelte-runes/icons/route';
	import IconTarget from '@tabler/icons-svelte-runes/icons/target';
	import IconFlame from '@tabler/icons-svelte-runes/icons/flame';
	import IconVideo from '@tabler/icons-svelte-runes/icons/video';
	import IconFileAnalytics from '@tabler/icons-svelte-runes/icons/file-analytics';

	interface Props {
		children: Snippet;
	}
	let props: Props = $props();

	const navItems = [
		{ href: '/admin/analytics', label: 'Dashboard', icon: IconChartBar },
		{ href: '/admin/analytics/events', label: 'Events', icon: IconBolt },
		{ href: '/admin/analytics/funnels', label: 'Funnels', icon: IconFilter },
		{ href: '/admin/analytics/cohorts', label: 'Cohorts', icon: IconUsers },
		{ href: '/admin/analytics/segments', label: 'Segments', icon: IconUsersGroup },
		{ href: '/admin/analytics/attribution', label: 'Attribution', icon: IconRoute },
		{ href: '/admin/analytics/goals', label: 'Goals', icon: IconTarget },
		{ href: '/admin/analytics/heatmaps', label: 'Heatmaps', icon: IconFlame },
		{ href: '/admin/analytics/recordings', label: 'Recordings', icon: IconVideo },
		{ href: '/admin/analytics/reports', label: 'Reports', icon: IconFileAnalytics }
	];

	let currentPath = $derived(page.url.pathname);
</script>

<div class="analytics-shell">
	<nav class="analytics-tabs" aria-label="Analytics sections">
		{#each navItems as item (item.href)}
			{@const Icon = item.icon}
			{@const active = currentPath === item.href}
			<a
				href={item.href}
				class={{ 'analytics-tab': true, active }}
				aria-current={active ? 'page' : undefined}
			>
				<Icon size={18} aria-hidden="true" />
				<span>{item.label}</span>
			</a>
		{/each}
	</nav>

	{@render props.children()}
</div>

<style>
	.analytics-shell {
		background: var(--bg-base, #0d1117);
	}

	.analytics-tabs {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		overflow-x: auto;
		padding: 0 1rem;
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-default, #30363d);
		border-radius: var(--radius-lg, 8px);
		margin-bottom: 1.5rem;
		scrollbar-width: thin;
	}

	.analytics-tab {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		min-height: 48px;
		padding: 0 1rem;
		border-bottom: 2px solid transparent;
		color: var(--text-secondary, #8b949e);
		font-size: 0.875rem;
		font-weight: 500;
		text-decoration: none;
		white-space: nowrap;
		transition:
			color 0.2s ease,
			border-color 0.2s ease,
			background-color 0.2s ease;
	}

	.analytics-tab:hover {
		color: var(--text-primary, #f0f6fc);
		background: var(--bg-hover, #252b33);
	}

	.analytics-tab.active {
		color: var(--primary-500, #e6b800);
		border-bottom-color: var(--primary-500, #e6b800);
	}

	.analytics-tab:focus-visible {
		outline: none;
		box-shadow: var(--admin-focus-ring, 0 0 0 3px rgba(230, 184, 0, 0.35));
	}

	@media (prefers-reduced-motion: reduce) {
		.analytics-tab {
			transition: none;
		}
	}
</style>

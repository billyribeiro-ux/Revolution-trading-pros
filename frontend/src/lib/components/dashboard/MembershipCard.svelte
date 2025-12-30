<script lang="ts">
	/**
	 * MembershipCard - Displays a single membership with actions
	 *
	 * Svelte 5 Runes:
	 * - $props() for component props
	 *
	 * Matches Simpler Trading core file structure:
	 * - membership-card class
	 * - Header with icon and title
	 * - Actions with Dashboard and Trading Room links
	 */

	import IconChartLine from '@tabler/icons-svelte/icons/chart-line';

	interface Props {
		name: string;
		slug: string;
		dashboardUrl?: string;
		tradingRoomUrl?: string;
		icon?: string;
	}

	let {
		name,
		slug,
		dashboardUrl,
		tradingRoomUrl,
		icon
	}: Props = $props();

	// Default URLs if not provided
	const defaultDashboardUrl = `/dashboard/${slug}`;
	const finalDashboardUrl = dashboardUrl || defaultDashboardUrl;
</script>

<article class="membership-card membership-card--options">
	<a href={finalDashboardUrl} class="membership-card__header">
		<span class="mem_icon">
			<span class="membership-card__icon">
				<IconChartLine size={24} />
			</span>
		</span>
		<span class="mem_div">{name}</span>
	</a>

	<div class="membership-card__actions">
		<a href={finalDashboardUrl}>Dashboard</a>
		{#if tradingRoomUrl}
			<a
				href={tradingRoomUrl}
				target="_blank"
				rel="noopener noreferrer nofollow"
			>
				Trading Room
			</a>
		{:else}
			<a href={finalDashboardUrl}>View Content</a>
		{/if}
	</div>
</article>

<style>
	/* Component-scoped styles that extend global dashboard.css */
	.mem_icon {
		display: flex;
		align-items: center;
	}

	.mem_div {
		font-size: 16px;
		font-weight: 600;
		color: var(--dashboard-text-primary);
	}
</style>

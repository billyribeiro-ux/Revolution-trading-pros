<script lang="ts">
	/**
	 * MembershipCard - Simpler Trading Style
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Svelte 5 component for displaying a membership card with:
	 * - Blue circular icon (#0984ae)
	 * - Card hover lift effect
	 * - Dashboard + Action links
	 *
	 * @version 1.0.0 - Svelte 5 with $props()
	 */
	import type { Snippet } from 'svelte';

	interface Props {
		/** Membership name displayed in card */
		name: string;
		/** URL slug for the membership */
		slug: string;
		/** Type of membership (trading-room, course, indicator, etc.) */
		type?: string;
		/** Custom dashboard URL (defaults to /dashboard/{slug}) */
		dashboardUrl?: string;
		/** Custom action URL */
		actionUrl?: string;
		/** Action button label */
		actionLabel?: string;
		/** Whether action opens in new tab */
		actionNewTab?: boolean;
		/** Optional icon snippet */
		icon?: Snippet;
		/** Special styling variant */
		variant?: 'default' | 'simpler-showcase' | 'weekly-watchlist';
	}

	let {
		name,
		slug,
		type = 'membership',
		dashboardUrl,
		actionUrl,
		actionLabel,
		actionNewTab = false,
		icon,
		variant = 'default'
	}: Props = $props();

	// Computed URLs
	let computedDashboardUrl = $derived(dashboardUrl || `/dashboard/${slug}/`);
	let computedActionUrl = $derived(actionUrl || `/dashboard/${slug}/`);

	// Computed action label based on type
	let computedActionLabel = $derived(() => {
		if (actionLabel) return actionLabel;
		switch (type) {
			case 'trading-room': return 'Trading Room';
			case 'alert-service': return 'View Alerts';
			case 'course': return 'View Course';
			case 'indicator': return 'Download';
			default: return 'Access';
		}
	});
</script>

<article class="membership-card" class:membership-card--showcase={variant === 'simpler-showcase'}>
	<a href={computedDashboardUrl} class="membership-card__header">
		<span class="membership-card__icon-wrap">
			<span class="membership-card__icon" class:membership-card__icon--showcase={variant === 'simpler-showcase'}>
				{#if icon}
					{@render icon()}
				{:else}
					<!-- Default chart icon -->
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
						<path d="M3 3v18h18"></path>
						<path d="m19 9-5 5-4-4-3 3"></path>
					</svg>
				{/if}
			</span>
		</span>
		<span class="membership-card__name">{name}</span>
	</a>
	<div class="membership-card__actions">
		<a href={computedDashboardUrl}>Dashboard</a>
		{#if actionNewTab}
			<a href={computedActionUrl} target="_blank" rel="noopener noreferrer">{computedActionLabel()}</a>
		{:else}
			<a href={computedActionUrl}>{computedActionLabel()}</a>
		{/if}
	</div>
</article>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   MEMBERSHIP CARD - Exact Simpler Trading Match
	   All styles scoped to this component
	   ═══════════════════════════════════════════════════════════════════════════ */
	.membership-card {
		background-color: #fff;
		border: 1px solid #dbdbdb;
		border-radius: 8px;
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1);
		transition: box-shadow 0.2s ease-in-out, transform 0.2s ease-in-out;
		overflow: hidden;
	}

	.membership-card:hover {
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.15);
		transform: translateY(-2px);
	}

	/* Header Link */
	.membership-card__header {
		display: flex;
		align-items: center;
		padding: 20px;
		color: #333;
		font-weight: 600;
		font-size: 15px;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.4;
		text-decoration: none;
		transition: background-color 0.15s ease-in-out;
	}

	.membership-card__header:hover {
		background-color: #f4f4f4;
	}

	.membership-card__header:visited {
		color: #333;
	}

	/* Icon Wrapper */
	.membership-card__icon-wrap {
		flex-shrink: 0;
		margin-right: 12px;
	}

	/* Blue Circular Icon */
	.membership-card__icon {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 50px;
		height: 50px;
		background-color: #0984ae;
		border-radius: 50%;
		color: #fff;
		box-shadow: 0 10px 20px rgba(9, 132, 174, 0.25);
	}

	.membership-card__icon :global(svg) {
		width: 24px;
		height: 24px;
		color: #fff;
	}

	/* Simpler Showcase special styling */
	.membership-card__icon--showcase {
		background-color: #000 !important;
		color: #f90 !important;
	}

	.membership-card__icon--showcase :global(svg) {
		color: #f90 !important;
	}

	/* Card Name */
	.membership-card__name {
		flex: 1;
		white-space: normal;
		word-wrap: break-word;
	}

	/* Action Links */
	.membership-card__actions {
		display: flex;
		border-top: 1px solid #ededed;
	}

	.membership-card__actions a {
		flex: 1 1 auto;
		padding: 15px 10px;
		color: #0984ae;
		font-size: 14px;
		font-family: 'Open Sans', sans-serif;
		text-align: center;
		text-decoration: none;
		transition: background-color 0.15s ease-in-out;
	}

	.membership-card__actions a:hover {
		background-color: #f4f4f4;
	}

	.membership-card__actions a + a {
		border-left: 1px solid #ededed;
	}
</style>

<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * EmptyState Component - No Data Display
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * Displays empty state messages for various data sections:
	 * - No alerts
	 * - No positions
	 * - No trades
	 * - No videos
	 * - Generic empty states
	 *
	 * @version 1.0.0
	 * @standards Apple Principal Engineer ICT 7+ | WCAG 2.1 AA
	 */

	type EmptyVariant = 'alerts' | 'positions' | 'trades' | 'videos' | 'default';

	interface Props {
		variant?: EmptyVariant;
		title?: string;
		description?: string;
		actionLabel?: string;
		onAction?: () => void;
	}

	const { variant = 'default', title, description, actionLabel, onAction }: Props = $props();

	const variantConfig: Record<EmptyVariant, { icon: string; title: string; description: string }> =
		{
			alerts: {
				icon: 'bell',
				title: 'No alerts yet',
				description: 'New trading alerts will appear here when they are posted.'
			},
			positions: {
				icon: 'chart',
				title: 'No active positions',
				description: 'Your open positions will be displayed here.'
			},
			trades: {
				icon: 'document',
				title: 'No trades found',
				description: 'Trades matching your criteria will appear here.'
			},
			videos: {
				icon: 'video',
				title: 'No videos available',
				description: 'Educational content and replays will appear here.'
			},
			default: {
				icon: 'inbox',
				title: 'No data',
				description: 'There is no data to display at this time.'
			}
		};

	const config = $derived(variantConfig[variant]);
	const displayTitle = $derived(title ?? config.title);
	const displayDescription = $derived(description ?? config.description);
	const iconType = $derived(config.icon);
</script>

<div class="empty-state" role="status">
	<div class="empty-icon">
		{#if iconType === 'bell'}
			<svg
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="1.5"
				width="48"
				height="48"
			>
				<path
					d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"
				/>
			</svg>
		{:else if iconType === 'chart'}
			<svg
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="1.5"
				width="48"
				height="48"
			>
				<path d="M7 12l3-3 3 3 4-4M8 21l4-4 4 4M3 4h18M4 4h16v12a1 1 0 01-1 1H5a1 1 0 01-1-1V4z" />
			</svg>
		{:else if iconType === 'document'}
			<svg
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="1.5"
				width="48"
				height="48"
			>
				<path
					d="M9 17v-2m3 2v-4m3 4v-6m2 10H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
				/>
			</svg>
		{:else if iconType === 'video'}
			<svg
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="1.5"
				width="48"
				height="48"
			>
				<path
					d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"
				/>
			</svg>
		{:else}
			<svg
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="1.5"
				width="48"
				height="48"
			>
				<path
					d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4"
				/>
			</svg>
		{/if}
	</div>

	<h3 class="empty-title">{displayTitle}</h3>
	<p class="empty-description">{displayDescription}</p>

	{#if actionLabel && onAction}
		<button type="button" class="empty-action" onclick={onAction}>
			{actionLabel}
		</button>
	{/if}
</div>

<style>
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		text-align: center;
		padding: var(--space-12) var(--space-6);
	}

	.empty-icon {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 80px;
		height: 80px;
		background: var(--color-bg-subtle);
		border-radius: var(--radius-xl);
		margin-bottom: var(--space-5);
		color: var(--color-text-muted);
		opacity: 0.7;
	}

	.empty-title {
		font-size: var(--text-lg);
		font-weight: var(--font-semibold);
		color: var(--color-text-primary);
		margin: 0 0 var(--space-2) 0;
	}

	.empty-description {
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
		margin: 0 0 var(--space-5) 0;
		max-width: 320px;
		line-height: var(--leading-relaxed);
	}

	.empty-action {
		display: inline-flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-2) var(--space-5);
		background: var(--color-brand-primary);
		color: white;
		border: none;
		border-radius: var(--radius-md);
		font-size: var(--text-sm);
		font-weight: var(--font-semibold);
		cursor: pointer;
		transition: var(--transition-colors);
	}

	.empty-action:hover {
		background: var(--color-brand-primary-hover);
	}

	.empty-action:focus-visible {
		outline: 2px solid var(--color-brand-primary);
		outline-offset: 2px;
	}

	.empty-action:active {
		transform: scale(0.98);
	}

	/* Responsive */
	@media (max-width: 480px) {
		.empty-state {
			padding: var(--space-8) var(--space-4);
		}

		.empty-icon {
			width: 64px;
			height: 64px;
		}

		.empty-icon svg {
			width: 32px;
			height: 32px;
		}
	}
</style>

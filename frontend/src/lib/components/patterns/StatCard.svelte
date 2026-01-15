<script lang="ts">
	/**
	 * StatCard - Metric display card with trend indicator
	 * 
	 * @version 2.0.0
	 * @author Revolution Trading Pros
	 */
	import { IconTrendingUp, IconTrendingDown, IconMinus } from '$lib/icons';
	import type { ComponentType } from 'svelte';

	interface Props {
		title: string;
		value: string | number;
		subtitle?: string;
		trend?: number | null;
		trendLabel?: string;
		icon?: ComponentType | null;
		iconColor?: 'primary' | 'success' | 'warning' | 'error' | 'info';
		loading?: boolean;
	}

	let {
		title,
		value,
		subtitle = '',
		trend = null,
		trendLabel = '',
		icon = null,
		iconColor = 'primary',
		loading = false
	}: Props = $props();

	let trendDirection = $derived(trend === null ? 'neutral' : trend > 0 ? 'up' : trend < 0 ? 'down' : 'neutral');
	let formattedTrend = $derived(trend !== null ? `${trend > 0 ? '+' : ''}${trend}%` : '');

	const iconColors = {
		primary: 'bg-rtp-primary-soft text-rtp-primary',
		success: 'bg-rtp-success-soft text-rtp-success',
		warning: 'bg-rtp-warning-soft text-rtp-warning',
		error: 'bg-rtp-error-soft text-rtp-error',
		info: 'bg-rtp-info-soft text-rtp-info'
	};
</script>

<div class="stat-card">
	{#if icon}
		{@const IconComponent = icon}
		<div class="stat-icon {iconColors[iconColor]}">
			<IconComponent size={24} />
		</div>
	{/if}
	
	<div class="stat-content">
		{#if loading}
			<div class="skeleton-value"></div>
			<div class="skeleton-title"></div>
		{:else}
			<div class="stat-value">{value}</div>
			<div class="stat-title">{title}</div>
			
			{#if trend !== null || subtitle}
				<div class="stat-footer">
					{#if trend !== null}
						<span class="trend trend-{trendDirection}">
							{#if trendDirection === 'up'}
								<IconTrendingUp size={14} />
							{:else if trendDirection === 'down'}
								<IconTrendingDown size={14} />
							{:else}
								<IconMinus size={14} />
							{/if}
							{formattedTrend}
						</span>
					{/if}
					{#if trendLabel || subtitle}
						<span class="trend-label">{trendLabel || subtitle}</span>
					{/if}
				</div>
			{/if}
		{/if}
	</div>
</div>

<style>
	.stat-card {
		display: flex;
		align-items: flex-start;
		gap: 1rem;
		padding: 1.25rem;
		background: var(--color-rtp-surface, rgba(30, 41, 59, 0.6));
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: var(--radius-xl, 1rem);
		transition: all 0.2s;
	}

	.stat-card:hover {
		border-color: rgba(99, 102, 241, 0.2);
		transform: translateY(-2px);
	}

	.stat-icon {
		width: 52px;
		height: 52px;
		border-radius: var(--radius-lg, 0.75rem);
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.stat-content {
		flex: 1;
		min-width: 0;
	}

	.stat-value {
		font-size: 1.75rem;
		font-weight: 700;
		color: var(--color-rtp-text, #f1f5f9);
		line-height: 1.2;
	}

	.stat-title {
		font-size: 0.875rem;
		color: var(--color-rtp-muted, #64748b);
		margin-top: 0.25rem;
	}

	.stat-footer {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-top: 0.75rem;
	}

	.trend {
		display: inline-flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.125rem 0.5rem;
		border-radius: var(--radius-full, 9999px);
		font-size: 0.75rem;
		font-weight: 600;
	}

	.trend-up {
		background: rgba(34, 197, 94, 0.15);
		color: #4ade80;
	}

	.trend-down {
		background: rgba(239, 68, 68, 0.15);
		color: #f87171;
	}

	.trend-neutral {
		background: rgba(148, 163, 184, 0.15);
		color: #94a3b8;
	}

	.trend-label {
		font-size: 0.75rem;
		color: var(--color-rtp-muted, #64748b);
	}

	/* Loading skeleton */
	.skeleton-value {
		height: 2rem;
		width: 60%;
		background: linear-gradient(90deg, rgba(99, 102, 241, 0.1) 25%, rgba(99, 102, 241, 0.2) 50%, rgba(99, 102, 241, 0.1) 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: var(--radius-md, 0.5rem);
	}

	.skeleton-title {
		height: 1rem;
		width: 40%;
		margin-top: 0.5rem;
		background: linear-gradient(90deg, rgba(99, 102, 241, 0.1) 25%, rgba(99, 102, 241, 0.2) 50%, rgba(99, 102, 241, 0.1) 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: var(--radius-sm, 0.375rem);
	}

	@keyframes shimmer {
		0% { background-position: 200% 0; }
		100% { background-position: -200% 0; }
	}
</style>

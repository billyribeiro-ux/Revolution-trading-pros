<script lang="ts">
	/**
	 * Skeleton Loader Component - Apple ICT9+ Design
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * Provides beautiful skeleton loading states for various UI patterns.
	 * Uses shimmer animation for Apple-level polish.
	 */

	interface Props {
		/** Type of skeleton to display */
		variant?:
			| 'card'
			| 'metric'
			| 'table-row'
			| 'list-item'
			| 'text'
			| 'avatar'
			| 'chart'
			| 'dashboard';
		/** Number of items to render */
		count?: number;
		/** Custom height */
		height?: string;
		/** Custom width */
		width?: string;
		/** Grid columns for dashboard variant */
		columns?: number;
	}

	let { variant = 'card', count = 1, height, width, columns = 4 }: Props = $props();
</script>

{#each Array(count) as _}
	{#if variant === 'metric'}
		<div class="skeleton skeleton-metric" style:height style:width>
			<div class="skeleton-icon"></div>
			<div class="skeleton-content">
				<div class="skeleton-line large"></div>
				<div class="skeleton-line small"></div>
			</div>
		</div>
	{:else if variant === 'card'}
		<div class="skeleton skeleton-card" style:height style:width>
			<div class="skeleton-header">
				<div class="skeleton-icon"></div>
				<div class="skeleton-title-area">
					<div class="skeleton-line medium"></div>
					<div class="skeleton-line small"></div>
				</div>
			</div>
			<div class="skeleton-body">
				<div class="skeleton-line"></div>
				<div class="skeleton-line"></div>
				<div class="skeleton-line short"></div>
			</div>
		</div>
	{:else if variant === 'table-row'}
		<div class="skeleton skeleton-table-row" style:height style:width>
			<div class="skeleton-cell avatar"></div>
			<div class="skeleton-cell wide"></div>
			<div class="skeleton-cell medium"></div>
			<div class="skeleton-cell narrow"></div>
			<div class="skeleton-cell narrow"></div>
		</div>
	{:else if variant === 'list-item'}
		<div class="skeleton skeleton-list-item" style:height style:width>
			<div class="skeleton-icon small"></div>
			<div class="skeleton-content">
				<div class="skeleton-line medium"></div>
				<div class="skeleton-line small light"></div>
			</div>
			<div class="skeleton-action"></div>
		</div>
	{:else if variant === 'text'}
		<div class="skeleton skeleton-text" style:height style:width>
			<div class="skeleton-line"></div>
			<div class="skeleton-line"></div>
			<div class="skeleton-line short"></div>
		</div>
	{:else if variant === 'avatar'}
		<div class="skeleton skeleton-avatar" style:height style:width></div>
	{:else if variant === 'chart'}
		<div class="skeleton skeleton-chart" style:height style:width>
			<div class="skeleton-chart-header">
				<div class="skeleton-line medium"></div>
			</div>
			<div class="skeleton-chart-bars">
				{#each Array(7) as _}
					<div class="skeleton-bar" style:height="{30 + Math.random() * 60}%"></div>
				{/each}
			</div>
		</div>
	{:else if variant === 'dashboard'}
		<div class="skeleton-dashboard" style="--columns: {columns}">
			<div class="skeleton-grid">
				{#each Array(columns) as _}
					<div class="skeleton skeleton-metric">
						<div class="skeleton-icon"></div>
						<div class="skeleton-content">
							<div class="skeleton-line large"></div>
							<div class="skeleton-line small"></div>
						</div>
					</div>
				{/each}
			</div>
			<div class="skeleton-main">
				<div class="skeleton skeleton-chart" style:height="300px">
					<div class="skeleton-chart-header">
						<div class="skeleton-line medium"></div>
					</div>
					<div class="skeleton-chart-bars">
						{#each Array(10) as _}
							<div class="skeleton-bar" style:height="{30 + Math.random() * 60}%"></div>
						{/each}
					</div>
				</div>
			</div>
		</div>
	{:else}
		<div class="skeleton" style:height style:width></div>
	{/if}
{/each}

<style>
	/* Base Skeleton */
	.skeleton {
		background: linear-gradient(
			90deg,
			rgba(148, 163, 184, 0.08) 0%,
			rgba(148, 163, 184, 0.15) 50%,
			rgba(148, 163, 184, 0.08) 100%
		);
		background-size: 200% 100%;
		animation: shimmer 1.8s ease-in-out infinite;
		border-radius: 12px;
	}

	@keyframes shimmer {
		0% {
			background-position: 200% 0;
		}
		100% {
			background-position: -200% 0;
		}
	}

	/* Metric Skeleton */
	.skeleton-metric {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		min-height: 100px;
	}

	.skeleton-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		background: linear-gradient(
			90deg,
			rgba(148, 163, 184, 0.1) 0%,
			rgba(148, 163, 184, 0.2) 50%,
			rgba(148, 163, 184, 0.1) 100%
		);
		background-size: 200% 100%;
		animation: shimmer 1.8s ease-in-out infinite;
		flex-shrink: 0;
	}

	.skeleton-icon.small {
		width: 36px;
		height: 36px;
		border-radius: 10px;
	}

	.skeleton-content {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.skeleton-line {
		height: 12px;
		border-radius: 6px;
		background: linear-gradient(
			90deg,
			rgba(148, 163, 184, 0.1) 0%,
			rgba(148, 163, 184, 0.2) 50%,
			rgba(148, 163, 184, 0.1) 100%
		);
		background-size: 200% 100%;
		animation: shimmer 1.8s ease-in-out infinite;
		width: 100%;
	}

	.skeleton-line.large {
		height: 24px;
		width: 60%;
	}

	.skeleton-line.medium {
		width: 80%;
	}

	.skeleton-line.small {
		height: 10px;
		width: 50%;
	}

	.skeleton-line.short {
		width: 40%;
	}

	.skeleton-line.light {
		opacity: 0.6;
	}

	/* Card Skeleton */
	.skeleton-card {
		padding: 1.5rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		min-height: 200px;
	}

	.skeleton-header {
		display: flex;
		align-items: center;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.skeleton-title-area {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.skeleton-body {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	/* Table Row Skeleton */
	.skeleton-table-row {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem;
		background: rgba(30, 41, 59, 0.4);
		border: 1px solid rgba(148, 163, 184, 0.05);
		margin-bottom: 0.5rem;
	}

	.skeleton-cell {
		height: 14px;
		border-radius: 4px;
		background: linear-gradient(
			90deg,
			rgba(148, 163, 184, 0.1) 0%,
			rgba(148, 163, 184, 0.2) 50%,
			rgba(148, 163, 184, 0.1) 100%
		);
		background-size: 200% 100%;
		animation: shimmer 1.8s ease-in-out infinite;
	}

	.skeleton-cell.avatar {
		width: 36px;
		height: 36px;
		border-radius: 50%;
		flex-shrink: 0;
	}

	.skeleton-cell.wide {
		flex: 2;
	}

	.skeleton-cell.medium {
		flex: 1;
	}

	.skeleton-cell.narrow {
		width: 80px;
		flex-shrink: 0;
	}

	/* List Item Skeleton */
	.skeleton-list-item {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem 1.25rem;
		background: rgba(30, 41, 59, 0.5);
		border: 1px solid rgba(148, 163, 184, 0.08);
		margin-bottom: 0.5rem;
	}

	.skeleton-action {
		width: 60px;
		height: 32px;
		border-radius: 8px;
		background: linear-gradient(
			90deg,
			rgba(148, 163, 184, 0.1) 0%,
			rgba(148, 163, 184, 0.2) 50%,
			rgba(148, 163, 184, 0.1) 100%
		);
		background-size: 200% 100%;
		animation: shimmer 1.8s ease-in-out infinite;
	}

	/* Text Skeleton */
	.skeleton-text {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		padding: 1rem;
	}

	/* Avatar Skeleton */
	.skeleton-avatar {
		width: 48px;
		height: 48px;
		border-radius: 50%;
	}

	/* Chart Skeleton */
	.skeleton-chart {
		padding: 1.5rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		display: flex;
		flex-direction: column;
		min-height: 200px;
	}

	.skeleton-chart-header {
		margin-bottom: 1.5rem;
	}

	.skeleton-chart-bars {
		flex: 1;
		display: flex;
		align-items: flex-end;
		gap: 0.75rem;
		padding-top: 1rem;
	}

	.skeleton-bar {
		flex: 1;
		background: linear-gradient(
			180deg,
			rgba(148, 163, 184, 0.15) 0%,
			rgba(148, 163, 184, 0.08) 100%
		);
		border-radius: 4px 4px 0 0;
		animation: pulse 2s ease-in-out infinite;
	}

	@keyframes pulse {
		0%,
		100% {
			opacity: 0.6;
		}
		50% {
			opacity: 1;
		}
	}

	/* Dashboard Skeleton */
	.skeleton-dashboard {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.skeleton-grid {
		display: grid;
		grid-template-columns: repeat(var(--columns, 4), 1fr);
		gap: 1rem;
	}

	.skeleton-main {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	/* Responsive */
	@media (max-width: 1024px) {
		.skeleton-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 640px) {
		.skeleton-grid {
			grid-template-columns: 1fr;
		}

		.skeleton-table-row {
			flex-wrap: wrap;
		}

		.skeleton-cell.narrow {
			width: 60px;
		}
	}
</style>

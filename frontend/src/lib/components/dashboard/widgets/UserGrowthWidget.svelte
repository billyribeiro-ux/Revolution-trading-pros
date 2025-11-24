<script lang="ts">
	export let data: any;
	export let config: {
		period?: string;
		show_total?: boolean;
		show_growth?: boolean;
		highlight_threshold?: number;
		format?: 'compact' | 'full';
	} = {};

	$: currentUsers = data?.current_users || 0;
	$: totalUsers = data?.total_users || 0;
	$: growthPercentage = data?.growth_percentage || 0;
	$: showTotal = config.show_total !== false;
	$: showGrowth = config.show_growth !== false;
	$: isHighGrowth = config.highlight_threshold
		? growthPercentage >= config.highlight_threshold
		: false;

	function formatNumber(num: number): string {
		if (config.format === 'compact' && num >= 1000) {
			return (num / 1000).toFixed(1) + 'K';
		}
		return num.toLocaleString();
	}
</script>

<div class="user-growth">
	<div class="stats-grid" class:single={!showTotal}>
		<div class="stat-item">
			<div class="stat-label">New Users</div>
			<div class="stat-value">{formatNumber(currentUsers)}</div>
		</div>
		{#if showTotal}
			<div class="stat-item">
				<div class="stat-label">Total Users</div>
				<div class="stat-value">{formatNumber(totalUsers)}</div>
			</div>
		{/if}
	</div>

	{#if showGrowth}
		<div class="growth-badge" class:positive={growthPercentage >= 0} class:highlight={isHighGrowth}>
			<span>{growthPercentage >= 0 ? '+' : ''}{growthPercentage.toFixed(1)}%</span>
			<span class="growth-label">vs previous {config.period || 'period'}</span>
		</div>
	{/if}
</div>

<style>
	.user-growth {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.stats-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1rem;
	}

	.stats-grid.single {
		grid-template-columns: 1fr;
	}

	.stat-item {
		text-align: center;
	}

	.stat-label {
		font-size: 0.75rem;
		color: #6b7280;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-bottom: 0.5rem;
	}

	.stat-value {
		font-size: 1.75rem;
		font-weight: 700;
		color: #1f2937;
	}

	.growth-badge {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 0.75rem;
		border-radius: 8px;
		font-weight: 600;
	}

	.growth-badge.positive {
		background: #d1fae5;
		color: #065f46;
	}

	.growth-badge:not(.positive) {
		background: #fee2e2;
		color: #991b1b;
	}

	.growth-badge.highlight {
		background: linear-gradient(135deg, #10b981, #059669);
		color: white;
		box-shadow: 0 4px 12px rgba(16, 185, 129, 0.3);
	}

	.growth-label {
		font-size: 0.75rem;
		font-weight: 400;
		margin-top: 0.25rem;
		opacity: 0.8;
	}
</style>

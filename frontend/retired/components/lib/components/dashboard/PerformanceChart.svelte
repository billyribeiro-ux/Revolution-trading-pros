<script lang="ts">
	/**
	 * PerformanceChart - Real-time 30-Day P&L Chart
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Renders an interactive SVG chart from real trade data
	 *
	 * @version 2.0.0 - ICT 11 Principal Engineer Grade
	 * @requires Svelte 5.0+ / SvelteKit 2.0+
	 */

	interface ChartDataPoint {
		date: string;
		pnl: number;
		cumulative: number;
	}

	interface Props {
		data: ChartDataPoint[];
		totalPnl: string;
		wins: number;
		losses: number;
		winRate: number;
		isLoading?: boolean;
		dataSource?: 'backend' | 'mock' | null;
	}

	let { data = [], totalPnl = '$0', wins = 0, losses = 0, winRate = 0, isLoading = false, dataSource = null }: Props = $props();

	// Chart dimensions
	const width = 280;
	const height = 120;
	const padding = { top: 10, right: 10, bottom: 20, left: 10 };
	const chartWidth = width - padding.left - padding.right;
	const chartHeight = height - padding.top - padding.bottom;

	// Calculate chart path from data
	const chartPath = $derived(() => {
		if (!data || data.length === 0) {
			// Generate placeholder data
			return generatePlaceholderPath();
		}

		const values = data.map(d => d.cumulative);
		const minVal = Math.min(...values, 0);
		const maxVal = Math.max(...values, 0);
		const range = maxVal - minVal || 1;

		const points = data.map((d, i) => {
			const x = padding.left + (i / (data.length - 1)) * chartWidth;
			const y = padding.top + chartHeight - ((d.cumulative - minVal) / range) * chartHeight;
			return `${x},${y}`;
		});

		return `M ${points.join(' L ')}`;
	});

	// Generate gradient stops based on P&L
	const gradientColor = $derived(() => {
		if (!data || data.length === 0) return { start: '#22c55e', end: '#22c55e20' };
		const lastValue = data[data.length - 1]?.cumulative || 0;
		if (lastValue >= 0) {
			return { start: '#22c55e', end: '#22c55e20' };
		}
		return { start: '#ef4444', end: '#ef444420' };
	});

	// Generate area fill path
	const areaPath = $derived(() => {
		if (!data || data.length === 0) {
			return generatePlaceholderArea();
		}

		const values = data.map(d => d.cumulative);
		const minVal = Math.min(...values, 0);
		const maxVal = Math.max(...values, 0);
		const range = maxVal - minVal || 1;

		const points = data.map((d, i) => {
			const x = padding.left + (i / (data.length - 1)) * chartWidth;
			const y = padding.top + chartHeight - ((d.cumulative - minVal) / range) * chartHeight;
			return `${x},${y}`;
		});

		const firstX = padding.left;
		const lastX = padding.left + chartWidth;
		const bottomY = padding.top + chartHeight;

		return `M ${firstX},${bottomY} L ${points.join(' L ')} L ${lastX},${bottomY} Z`;
	});

	function generatePlaceholderPath(): string {
		// Generate a realistic upward trend for demo
		const points = [];
		let cumulative = 0;
		for (let i = 0; i < 30; i++) {
			cumulative += Math.random() * 800 - 200; // Random daily P&L
			const x = padding.left + (i / 29) * chartWidth;
			const y = padding.top + chartHeight - ((cumulative + 10000) / 25000) * chartHeight;
			points.push(`${x},${Math.max(padding.top, Math.min(padding.top + chartHeight, y))}`);
		}
		return `M ${points.join(' L ')}`;
	}

	function generatePlaceholderArea(): string {
		const linePath = generatePlaceholderPath();
		const firstX = padding.left;
		const lastX = padding.left + chartWidth;
		const bottomY = padding.top + chartHeight;
		return `M ${firstX},${bottomY} ${linePath.substring(2)} L ${lastX},${bottomY} Z`;
	}
</script>

<div class="performance-chart">
	<h3>30-Day Performance</h3>
	
	{#if dataSource === 'mock'}
		<span class="data-badge">Demo</span>
	{/if}

	{#if isLoading}
		<div class="loading-skeleton">
			<div class="skeleton-chart"></div>
		</div>
	{:else}
		<div class="chart-container">
			<svg viewBox="0 0 {width} {height}" preserveAspectRatio="xMidYMid meet">
				<defs>
					<linearGradient id="chartGradient" x1="0%" y1="0%" x2="0%" y2="100%">
						<stop offset="0%" stop-color={gradientColor().start} stop-opacity="0.3" />
						<stop offset="100%" stop-color={gradientColor().end} stop-opacity="0.05" />
					</linearGradient>
				</defs>
				
				<!-- Area fill -->
				<path d={areaPath()} fill="url(#chartGradient)" />
				
				<!-- Line -->
				<path 
					d={chartPath()} 
					fill="none" 
					stroke={gradientColor().start}
					stroke-width="2.5"
					stroke-linecap="round"
					stroke-linejoin="round"
				/>
				
				<!-- Zero line -->
				<line 
					x1={padding.left} 
					y1={padding.top + chartHeight / 2} 
					x2={padding.left + chartWidth} 
					y2={padding.top + chartHeight / 2}
					stroke="#e5e7eb"
					stroke-width="1"
					stroke-dasharray="4,4"
				/>
			</svg>
		</div>

		<div class="total-pnl" class:positive={!totalPnl.startsWith('-')}>
			{totalPnl}
		</div>

		<div class="stats-row">
			<div class="stat">
				<span class="stat-value wins">{wins}</span>
				<span class="stat-label">Wins</span>
			</div>
			<div class="stat">
				<span class="stat-value losses">{losses}</span>
				<span class="stat-label">Losses</span>
			</div>
			<div class="stat">
				<span class="stat-value">{winRate.toFixed(0)}%</span>
				<span class="stat-label">Win Rate</span>
			</div>
		</div>
	{/if}
</div>

<style>
	.performance-chart {
		background: #fff;
		border-radius: 12px;
		padding: 20px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
		position: relative;
	}

	h3 {
		font-size: 14px;
		font-weight: 700;
		color: #333;
		margin: 0 0 16px 0;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.data-badge {
		position: absolute;
		top: 16px;
		right: 16px;
		padding: 2px 8px;
		background: #fef3c7;
		color: #92400e;
		border-radius: 9999px;
		font-size: 10px;
		font-weight: 600;
	}

	.chart-container {
		margin-bottom: 16px;
	}

	.chart-container svg {
		width: 100%;
		height: auto;
		display: block;
	}

	.total-pnl {
		font-size: 28px;
		font-weight: 700;
		color: #22c55e;
		text-align: center;
		margin-bottom: 16px;
	}

	.total-pnl:not(.positive) {
		color: #ef4444;
	}

	.stats-row {
		display: flex;
		justify-content: space-between;
		border-top: 1px solid #f0f0f0;
		padding-top: 16px;
	}

	.stat {
		text-align: center;
		flex: 1;
	}

	.stat-value {
		display: block;
		font-size: 18px;
		font-weight: 700;
		color: #333;
		margin-bottom: 4px;
	}

	.stat-value.wins {
		color: #22c55e;
	}

	.stat-value.losses {
		color: #ef4444;
	}

	.stat-label {
		font-size: 11px;
		color: #888;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.loading-skeleton {
		padding: 20px 0;
	}

	.skeleton-chart {
		height: 120px;
		background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 8px;
	}

	@keyframes shimmer {
		0% { background-position: 200% 0; }
		100% { background-position: -200% 0; }
	}
</style>

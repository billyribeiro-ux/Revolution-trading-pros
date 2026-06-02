<script lang="ts">
	/**
	 * Bandwidth Savings Dashboard - Apple ICT 7 Grade Analytics
	 *
	 * Principal Engineer Grade implementation featuring:
	 * - Real-time bandwidth savings visualization
	 * - Storage optimization trend analysis
	 * - Format distribution with compression metrics
	 * - Performance insights with recommendations
	 * - Cost and CO2 savings estimates
	 * - Connection-aware data loading (NO MOCK DATA)
	 *
	 * Svelte 5 Runes: $state, $derived, $effect
	 *
	 * @since January 2026
	 */
	import { tweened } from 'svelte/motion';
	import { cubicOut } from 'svelte/easing';
	import { fly, scale } from 'svelte/transition';
	import { getMediaAnalytics } from './media-analytics.remote';
	import type { BandwidthData, TimeRange } from './media-analytics.types';

	// FIX-2026-04-26: Tabler icons replace 17 raw inline <svg> blocks.
	import IconChevronLeft from '@tabler/icons-svelte-runes/icons/chevron-left';
	import IconDownload from '@tabler/icons-svelte-runes/icons/download';
	import IconLoader from '@tabler/icons-svelte-runes/icons/loader';
	import IconSettings from '@tabler/icons-svelte-runes/icons/settings';
	import IconRefresh from '@tabler/icons-svelte-runes/icons/refresh';
	import IconBolt from '@tabler/icons-svelte-runes/icons/bolt';
	import IconCurrencyDollar from '@tabler/icons-svelte-runes/icons/currency-dollar';
	import IconLeaf from '@tabler/icons-svelte-runes/icons/leaf';
	import IconArrowUp from '@tabler/icons-svelte-runes/icons/arrow-up';
	import IconPhoto from '@tabler/icons-svelte-runes/icons/photo';
	import IconCircleCheck from '@tabler/icons-svelte-runes/icons/circle-check';
	import IconBox from '@tabler/icons-svelte-runes/icons/box';
	import IconBookmark from '@tabler/icons-svelte-runes/icons/bookmark';
	import IconActivity from '@tabler/icons-svelte-runes/icons/activity';
	import IconCircleCheckFilled from '@tabler/icons-svelte-runes/icons/circle-check-filled';
	import IconInfoCircle from '@tabler/icons-svelte-runes/icons/info-circle';

	let timeRange = $state<TimeRange>('30d');

	// One reactive query drives the whole dashboard: changing `timeRange`
	// re-fetches (and dedupes identical ranges); with experimental.async on, the
	// result is server-resolved on first paint. Same-named `$derived` views keep
	// every chart's markup unchanged.
	const analyticsQuery = $derived(getMediaAnalytics(timeRange));
	const data = $derived(analyticsQuery.current);

	const overview = $derived(data?.overview ?? null);
	const bandwidthData = $derived(data?.bandwidth ?? []);
	const formatStats = $derived(data?.formats ?? []);
	const isLoading = $derived(analyticsQuery.loading);
	const isConnected = $derived(data?.hasData ?? false);
	const connectionError = $derived(
		analyticsQuery.error
			? 'Failed to connect to media analytics service. Please check your connection settings.'
			: null
	);

	// Animated values
	const savingsPercent = tweened(0, { duration: 1500, easing: cubicOut });
	const totalSavings = tweened(0, { duration: 1500, easing: cubicOut });
	const co2Saved = tweened(0, { duration: 1500, easing: cubicOut });

	// Drive the tweened headline numbers off the (derived) overview. This is a
	// genuine side-effect — syncing imperative motion stores to reactive data —
	// not state mirroring, so `$effect` is the right tool.
	$effect(() => {
		if (overview) {
			savingsPercent.set(overview.savingsPercent);
			totalSavings.set(overview.totalSavings);
			co2Saved.set(overview.co2Saved);
		}
	});

	// Helpers

	function formatBytes(bytes: number): string {
		if (bytes < 1024) return bytes.toFixed(0) + ' B';
		if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
		if (bytes < 1024 * 1024 * 1024) return (bytes / 1024 / 1024).toFixed(2) + ' MB';
		return (bytes / 1024 / 1024 / 1024).toFixed(2) + ' GB';
	}

	function formatNumber(n: number): string {
		return new Intl.NumberFormat().format(Math.round(n));
	}

	function formatCurrency(n: number): string {
		return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD' }).format(n);
	}

	function getChartPath(data: BandwidthData[], key: 'original' | 'optimized' | 'savings'): string {
		if (data.length === 0) return '';

		const maxValue = Math.max(...data.map((d) => d.original));
		const width = 100;
		const height = 100;

		const points = data.map((d, i) => {
			const x = (i / (data.length - 1)) * width;
			const y = height - (d[key] / maxValue) * height;
			return `${x},${y}`;
		});

		return `M${points.join(' L')}`;
	}

	function getChartArea(data: BandwidthData[], key: 'original' | 'optimized' | 'savings'): string {
		if (data.length === 0) return '';

		const maxValue = Math.max(...data.map((d) => d.original));
		const width = 100;
		const height = 100;

		const points = data.map((d, i) => {
			const x = (i / (data.length - 1)) * width;
			const y = height - (d[key] / maxValue) * height;
			return `${x},${y}`;
		});

		return `M0,${height} L${points.join(' L')} L${width},${height} Z`;
	}

	const maxBandwidth = $derived(
		bandwidthData.length > 0 ? Math.max(...bandwidthData.map((d) => d.original)) : 0
	);
</script>

<svelte:head>
	<title>Bandwidth Analytics | Admin</title>
</svelte:head>

<div class="analytics-dashboard">
	<!-- Header -->
	<header class="header">
		<div class="header-left">
			<a href="/admin/media" class="back-link" aria-label="Back to Media Library">
				<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: chevron-left (back) -->
				<IconChevronLeft size={20} aria-hidden="true" />
			</a>
			<div>
				<h1>Bandwidth Analytics</h1>
				<p class="subtitle">Track your optimization savings and performance</p>
			</div>
		</div>

		<div class="header-right">
			<!-- Time Range Selector -->
			<div class="time-selector">
				{#each ['7d', '30d', '90d', '1y'] as range (range)}
					<button
						class:active={timeRange === range}
						onclick={() => (timeRange = range as TimeRange)}
					>
						{range === '7d'
							? '7 Days'
							: range === '30d'
								? '30 Days'
								: range === '90d'
									? '90 Days'
									: '1 Year'}
					</button>
				{/each}
			</div>

			<button class="btn-export" onclick={() => {}}>
				<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: download (export) -->
				<IconDownload size={16} aria-hidden="true" />
				Export
			</button>
		</div>
	</header>

	{#if isLoading}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>Loading analytics...</p>
		</div>
	{:else if !isConnected || connectionError}
		<!-- NOT CONNECTED STATE - No fake data -->
		<div class="not-connected-state">
			<div class="not-connected-card">
				<div class="not-connected-icon">
					<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: loader (not connected) -->
					<IconLoader size={48} aria-hidden="true" />
				</div>
				<h2>Media Analytics Not Connected</h2>
				<p>
					{connectionError ||
						'Connect your image optimization service to view real bandwidth savings and analytics data.'}
				</p>
				<div class="not-connected-actions">
					<a href="/admin/connections" class="btn-primary">
						<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: settings (connect service) -->
						<IconSettings size={16} aria-hidden="true" />
						Connect Service
					</a>
					<button class="btn-secondary" onclick={() => analyticsQuery.refresh()}>
						<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: refresh (retry) -->
						<IconRefresh size={16} aria-hidden="true" />
						Retry
					</button>
				</div>
				<div class="service-info">
					<h3>Supported Services</h3>
					<ul>
						<li>Cloudflare Images</li>
						<li>Imgix</li>
						<li>Cloudinary</li>
						<li>Custom CDN with analytics</li>
					</ul>
				</div>
			</div>
		</div>
	{:else if overview}
		<!-- Hero Stats -->
		<div class="hero-stats">
			<div class="hero-card savings" transition:fly={{ y: 20, duration: 500 }}>
				<div class="hero-icon">
					<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: bolt (savings) -->
					<IconBolt size={32} aria-hidden="true" />
				</div>
				<div class="hero-content">
					<div class="hero-value">{formatBytes($totalSavings)}</div>
					<div class="hero-label">Total Bandwidth Saved</div>
					<div class="hero-change positive">
						<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: arrow-up (change) -->
						<IconArrowUp size={16} aria-hidden="true" />
						{$savingsPercent.toFixed(1)}% compression
					</div>
				</div>
				<div class="hero-ring">
					<svg aria-hidden="true" viewBox="0 0 100 100">
						<circle
							cx="50"
							cy="50"
							r="45"
							fill="none"
							stroke="currentColor"
							stroke-width="8"
							opacity="0.1"
						/>
						<circle
							cx="50"
							cy="50"
							r="45"
							fill="none"
							stroke="currentColor"
							stroke-width="8"
							stroke-dasharray={`${$savingsPercent * 2.83} 283`}
							stroke-linecap="round"
							transform="rotate(-90 50 50)"
						/>
					</svg>
					<span>{Math.round($savingsPercent)}%</span>
				</div>
			</div>

			<div class="hero-card cost" transition:fly={{ y: 20, duration: 500, delay: 100 }}>
				<div class="hero-icon">
					<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: currency-dollar (cost) -->
					<IconCurrencyDollar size={32} aria-hidden="true" />
				</div>
				<div class="hero-content">
					<div class="hero-value">{formatCurrency(overview.estimatedCostSavings)}</div>
					<div class="hero-label">Estimated Cost Savings</div>
					<div class="hero-detail">per month at $0.02/GB</div>
				</div>
			</div>

			<div class="hero-card eco" transition:fly={{ y: 20, duration: 500, delay: 200 }}>
				<div class="hero-icon">
					<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: leaf (eco) -->
					<IconLeaf size={32} aria-hidden="true" />
				</div>
				<div class="hero-content">
					<div class="hero-value">{$co2Saved.toFixed(1)} kg</div>
					<div class="hero-label">CO2 Emissions Saved</div>
					<div class="hero-detail">
						equivalent to planting {Math.round(overview.co2Saved * 0.8)} trees
					</div>
				</div>
			</div>
		</div>

		<!-- Stats Grid -->
		<div class="stats-grid">
			<div class="stat-card" transition:fly={{ y: 20, duration: 400, delay: 300 }}>
				<div class="stat-icon blue">
					<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: photo (total images) -->
					<IconPhoto size={24} aria-hidden="true" />
				</div>
				<div class="stat-info">
					<span class="stat-value">{formatNumber(overview.totalImages)}</span>
					<span class="stat-label">Total Images</span>
				</div>
			</div>

			<div class="stat-card" transition:fly={{ y: 20, duration: 400, delay: 350 }}>
				<div class="stat-icon green">
					<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: circle-check (optimized) -->
					<IconCircleCheck size={24} aria-hidden="true" />
				</div>
				<div class="stat-info">
					<span class="stat-value">{formatNumber(overview.optimizedImages)}</span>
					<span class="stat-label">Optimized</span>
				</div>
			</div>

			<div class="stat-card" transition:fly={{ y: 20, duration: 400, delay: 400 }}>
				<div class="stat-icon purple">
					<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: box (original size) -->
					<IconBox size={24} aria-hidden="true" />
				</div>
				<div class="stat-info">
					<span class="stat-value">{formatBytes(overview.totalOriginal)}</span>
					<span class="stat-label">Original Size</span>
				</div>
			</div>

			<div class="stat-card" transition:fly={{ y: 20, duration: 400, delay: 450 }}>
				<div class="stat-icon orange">
					<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: bookmark (optimized size) -->
					<IconBookmark size={24} aria-hidden="true" />
				</div>
				<div class="stat-info">
					<span class="stat-value">{formatBytes(overview.totalOptimized)}</span>
					<span class="stat-label">Optimized Size</span>
				</div>
			</div>

			<div class="stat-card" transition:fly={{ y: 20, duration: 400, delay: 500 }}>
				<div class="stat-icon cyan">
					<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: activity (compression) -->
					<IconActivity size={24} aria-hidden="true" />
				</div>
				<div class="stat-info">
					<span class="stat-value">{overview.avgCompressionRatio.toFixed(1)}x</span>
					<span class="stat-label">Avg Compression</span>
				</div>
			</div>
		</div>

		<!-- Bandwidth Chart -->
		<div class="chart-section" transition:fly={{ y: 20, duration: 400, delay: 550 }}>
			<div class="section-header">
				<h2>Bandwidth Over Time</h2>
				<div class="chart-legend">
					<span class="legend-item original">
						<span class="dot"></span>
						Original
					</span>
					<span class="legend-item optimized">
						<span class="dot"></span>
						Optimized
					</span>
					<span class="legend-item savings">
						<span class="dot"></span>
						Savings
					</span>
				</div>
			</div>

			<div class="chart-container">
				<svg aria-hidden="true" viewBox="0 0 100 100" preserveAspectRatio="none" class="chart">
					<!-- Grid lines -->
					<line
						x1="0"
						y1="25"
						x2="100"
						y2="25"
						stroke="currentColor"
						stroke-width="0.2"
						opacity="0.2"
					/>
					<line
						x1="0"
						y1="50"
						x2="100"
						y2="50"
						stroke="currentColor"
						stroke-width="0.2"
						opacity="0.2"
					/>
					<line
						x1="0"
						y1="75"
						x2="100"
						y2="75"
						stroke="currentColor"
						stroke-width="0.2"
						opacity="0.2"
					/>

					<!-- Areas -->
					<path
						d={getChartArea(bandwidthData, 'original')}
						fill="url(#originalGradient)"
						opacity="0.3"
					/>
					<path
						d={getChartArea(bandwidthData, 'optimized')}
						fill="url(#optimizedGradient)"
						opacity="0.5"
					/>

					<!-- Lines -->
					<path
						d={getChartPath(bandwidthData, 'original')}
						fill="none"
						stroke="#E6B800"
						stroke-width="0.5"
					/>
					<path
						d={getChartPath(bandwidthData, 'optimized')}
						fill="none"
						stroke="#10b981"
						stroke-width="0.5"
					/>

					<!-- Gradients -->
					<defs>
						<linearGradient id="originalGradient" x1="0" y1="0" x2="0" y2="1">
							<stop offset="0%" stop-color="#E6B800" />
							<stop offset="100%" stop-color="#E6B800" stop-opacity="0" />
						</linearGradient>
						<linearGradient id="optimizedGradient" x1="0" y1="0" x2="0" y2="1">
							<stop offset="0%" stop-color="#10b981" />
							<stop offset="100%" stop-color="#10b981" stop-opacity="0" />
						</linearGradient>
					</defs>
				</svg>

				<!-- Y-axis labels -->
				<div class="y-axis">
					<span>{formatBytes(maxBandwidth)}</span>
					<span>{formatBytes(maxBandwidth * 0.75)}</span>
					<span>{formatBytes(maxBandwidth * 0.5)}</span>
					<span>{formatBytes(maxBandwidth * 0.25)}</span>
					<span>0</span>
				</div>

				<!-- X-axis labels -->
				<div class="x-axis">
					{#if bandwidthData.length > 0}
						<span>{bandwidthData[0]?.date ?? ''}</span>
						<span>{bandwidthData[Math.floor(bandwidthData.length / 2)]?.date ?? ''}</span>
						<span>{bandwidthData[bandwidthData.length - 1]?.date ?? ''}</span>
					{/if}
				</div>
			</div>
		</div>

		<!-- Format Distribution -->
		<div class="format-section" transition:fly={{ y: 20, duration: 400, delay: 600 }}>
			<div class="section-header">
				<h2>Format Distribution</h2>
			</div>

			<div class="format-grid">
				{#each formatStats as stat, i (stat.format)}
					<div class="format-card" transition:scale={{ duration: 300, delay: 650 + i * 50 }}>
						<div class="format-header">
							<span class="format-name">{stat.format}</span>
							<span class="format-count">{stat.count} images</span>
						</div>

						<div class="format-bar">
							<div class="bar-original" style="width: 100%">
								<span class="bar-label">{formatBytes(stat.originalSize)}</span>
							</div>
							<div class="bar-optimized" style="width: {100 - stat.savings}%">
								<span class="bar-label">{formatBytes(stat.optimizedSize)}</span>
							</div>
						</div>

						<div class="format-savings">
							<span class="savings-value">{stat.savings.toFixed(1)}%</span>
							<span class="savings-label">savings</span>
						</div>
					</div>
				{/each}
			</div>
		</div>

		<!-- Performance Insights -->
		<div class="insights-section" transition:fly={{ y: 20, duration: 400, delay: 700 }}>
			<div class="section-header">
				<h2>Performance Insights</h2>
			</div>

			<div class="insights-grid">
				<div class="insight-card">
					<div class="insight-icon green">
						<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: circle-check-filled (excellent) -->
						<IconCircleCheckFilled size={24} aria-hidden="true" />
					</div>
					<div class="insight-content">
						<h3>Excellent Compression</h3>
						<p>
							Your images are {overview.avgCompressionRatio.toFixed(1)}x smaller on average. This
							puts you in the top 10% of optimized sites.
						</p>
					</div>
				</div>

				<div class="insight-card">
					<div class="insight-icon blue">
						<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: bolt (fast load) -->
						<IconBolt size={20} aria-hidden="true" />
					</div>
					<div class="insight-content">
						<h3>Fast Load Times</h3>
						<p>
							Optimized images load {(
								((overview.totalOriginal / overview.totalOptimized) * 100) /
								100
							).toFixed(1)}x faster, improving Core Web Vitals significantly.
						</p>
					</div>
				</div>

				<div class="insight-card">
					<div class="insight-icon orange">
						<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: info-circle (recommendation) -->
						<IconInfoCircle size={24} aria-hidden="true" />
					</div>
					<div class="insight-content">
						<h3>Recommendation</h3>
						<p>
							Consider enabling AVIF for browsers that support it. This could improve compression by
							an additional 20-30%.
						</p>
					</div>
				</div>
			</div>
		</div>
	{/if}
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
     Base Layout
     ═══════════════════════════════════════════════════════════════════════════ */
	.analytics-dashboard {
		background: #f5f5f7;
		font-family:
			-apple-system, BlinkMacSystemFont, 'SF Pro Display', 'Segoe UI', Roboto, sans-serif;
		padding-bottom: 48px;
	}

	/* Header */
	.header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 16px 24px;
		background: rgba(255, 255, 255, 0.8);
		backdrop-filter: blur(20px);
		-webkit-backdrop-filter: blur(20px);
		border-bottom: 1px solid rgba(0, 0, 0, 0.1);
		position: sticky;
		top: 0;
		z-index: 100;
	}

	.header-left {
		display: flex;
		align-items: center;
		gap: 16px;
	}

	.back-link {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		background: #f5f5f7;
		border-radius: 8px;
		color: #1d1d1f;
		transition: all 0.2s;
	}

	.back-link:hover {
		background: #e8e8ed;
	}

	/* FIX-2026-04-26: :global() so styles reach Tabler's internal <svg> */
	.back-link :global(svg) {
		width: 20px;
		height: 20px;
	}

	.header h1 {
		font-size: 24px;
		font-weight: 700;
		color: #1d1d1f;
		margin: 0;
	}

	.subtitle {
		font-size: 14px;
		color: #86868b;
		margin: 4px 0 0 0;
	}

	.header-right {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.time-selector {
		display: flex;
		background: #f5f5f7;
		border-radius: 8px;
		padding: 2px;
	}

	.time-selector button {
		padding: 8px 14px;
		background: transparent;
		border: none;
		border-radius: 6px;
		font-size: 13px;
		font-weight: 500;
		color: #86868b;
		cursor: pointer;
		transition: all 0.2s;
	}

	.time-selector button.active {
		background: white;
		color: #1d1d1f;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.btn-export {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 10px 16px;
		background: #f5f5f7;
		border: none;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 500;
		color: #1d1d1f;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-export:hover {
		background: #e8e8ed;
	}

	/* FIX-2026-04-26: :global() so styles reach Tabler's internal <svg> */
	.btn-export :global(svg) {
		width: 16px;
		height: 16px;
	}

	/* Loading State */
	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 400px;
		gap: 16px;
		color: #86868b;
	}

	/* Not Connected State - NO MOCK DATA */
	.not-connected-state {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 48px 24px;
		min-height: 500px;
	}

	.not-connected-card {
		max-width: 480px;
		text-align: center;
		padding: 48px;
		background: white;
		border-radius: 20px;
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
	}

	.not-connected-icon {
		width: 80px;
		height: 80px;
		margin: 0 auto 24px;
		background: linear-gradient(135deg, #f97316 0%, #ea580c 100%);
		border-radius: 20px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
	}

	/* FIX-2026-04-26: :global() so styles reach Tabler's internal <svg> */
	.not-connected-icon :global(svg) {
		width: 40px;
		height: 40px;
	}

	.not-connected-card h2 {
		font-size: 24px;
		font-weight: 700;
		color: #1d1d1f;
		margin: 0 0 12px;
	}

	.not-connected-card > p {
		font-size: 15px;
		color: #86868b;
		line-height: 1.6;
		margin: 0 0 24px;
	}

	.not-connected-actions {
		display: flex;
		gap: 12px;
		justify-content: center;
		margin-bottom: 32px;
	}

	.not-connected-actions .btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 12px 24px;
		background: linear-gradient(135deg, #0071e3 0%, #0077ed 100%);
		color: white;
		font-size: 14px;
		font-weight: 600;
		border-radius: 10px;
		text-decoration: none;
		transition: all 0.2s;
	}

	.not-connected-actions .btn-primary:hover {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(0, 113, 227, 0.3);
	}

	.not-connected-actions .btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 12px 24px;
		background: #f5f5f7;
		color: #1d1d1f;
		font-size: 14px;
		font-weight: 500;
		border: none;
		border-radius: 10px;
		cursor: pointer;
		transition: all 0.2s;
	}

	.not-connected-actions .btn-secondary:hover {
		background: #e8e8ed;
	}

	.service-info {
		padding-top: 24px;
		border-top: 1px solid #e5e5e5;
	}

	.service-info h3 {
		font-size: 13px;
		font-weight: 600;
		color: #86868b;
		margin: 0 0 12px;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.service-info ul {
		list-style: none;
		padding: 0;
		margin: 0;
		display: flex;
		flex-wrap: wrap;
		gap: 8px;
		justify-content: center;
	}

	.service-info li {
		font-size: 13px;
		color: #1d1d1f;
		padding: 6px 12px;
		background: #f5f5f7;
		border-radius: 6px;
	}

	.spinner {
		width: 32px;
		height: 32px;
		border: 3px solid rgba(0, 0, 0, 0.1);
		border-top-color: #0071e3;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Hero Stats */
	.hero-stats {
		display: grid;
		grid-template-columns: 2fr 1fr 1fr;
		gap: 16px;
		padding: 24px;
	}

	.hero-card {
		display: flex;
		align-items: center;
		gap: 20px;
		padding: 24px;
		background: white;
		border-radius: 16px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
		position: relative;
		overflow: hidden;
	}

	.hero-card.savings {
		background: linear-gradient(135deg, #10b981 0%, #059669 100%);
		color: white;
	}

	.hero-card.cost {
		background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);
		color: white;
	}

	.hero-card.eco {
		background: linear-gradient(135deg, var(--primary-500) 0%, var(--primary-600) 100%);
		color: var(--bg-base);
	}

	.hero-icon {
		width: 56px;
		height: 56px;
		background: rgba(255, 255, 255, 0.2);
		border-radius: 14px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	/* FIX-2026-04-26: :global() so styles reach Tabler's internal <svg> */
	.hero-icon :global(svg) {
		width: 28px;
		height: 28px;
	}

	.hero-content {
		flex: 1;
	}

	.hero-value {
		font-size: 32px;
		font-weight: 700;
		line-height: 1.2;
	}

	.hero-label {
		font-size: 14px;
		opacity: 0.9;
		margin-top: 4px;
	}

	.hero-change {
		display: flex;
		align-items: center;
		gap: 4px;
		font-size: 13px;
		margin-top: 8px;
		background: rgba(255, 255, 255, 0.2);
		padding: 4px 10px;
		border-radius: 12px;
		width: fit-content;
	}

	/* FIX-2026-04-26: :global() so styles reach Tabler's internal <svg> */
	.hero-change :global(svg) {
		width: 14px;
		height: 14px;
	}

	.hero-detail {
		font-size: 12px;
		opacity: 0.8;
		margin-top: 8px;
	}

	.hero-ring {
		position: relative;
		width: 80px;
		height: 80px;
	}

	.hero-ring svg {
		width: 100%;
		height: 100%;
	}

	.hero-ring span {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 18px;
		font-weight: 700;
	}

	/* Stats Grid */
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(5, 1fr);
		gap: 16px;
		padding: 0 24px;
	}

	.stat-card {
		display: flex;
		align-items: center;
		gap: 16px;
		padding: 20px;
		background: white;
		border-radius: 12px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.stat-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	/* FIX-2026-04-26: :global() so styles reach Tabler's internal <svg> */
	.stat-icon :global(svg) {
		width: 24px;
		height: 24px;
	}

	.stat-icon.blue {
		background: #dbeafe;
		color: #2563eb;
	}
	.stat-icon.green {
		background: #d1fae5;
		color: #059669;
	}
	.stat-icon.purple {
		background: rgba(230, 184, 0, 0.15);
		color: var(--primary-500);
	}
	.stat-icon.orange {
		background: #fef3c7;
		color: #d97706;
	}
	.stat-icon.cyan {
		background: #cffafe;
		color: #0891b2;
	}

	.stat-info {
		display: flex;
		flex-direction: column;
	}

	.stat-value {
		font-size: 20px;
		font-weight: 600;
		color: #1d1d1f;
	}

	.stat-label {
		font-size: 12px;
		color: #86868b;
	}

	/* Chart Section */
	.chart-section {
		margin: 24px;
		padding: 24px;
		background: white;
		border-radius: 16px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.section-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 24px;
	}

	.section-header h2 {
		font-size: 18px;
		font-weight: 600;
		color: #1d1d1f;
		margin: 0;
	}

	.chart-legend {
		display: flex;
		gap: 20px;
	}

	.legend-item {
		display: flex;
		align-items: center;
		gap: 6px;
		font-size: 13px;
		color: #86868b;
	}

	.legend-item .dot {
		width: 10px;
		height: 10px;
		border-radius: 50%;
	}

	.legend-item.original .dot {
		background: var(--primary-500);
	}
	.legend-item.optimized .dot {
		background: #10b981;
	}
	.legend-item.savings .dot {
		background: #f59e0b;
	}

	.chart-container {
		position: relative;
		height: 300px;
		padding-left: 60px;
		padding-bottom: 40px;
	}

	.chart {
		width: 100%;
		height: 100%;
		color: #1d1d1f;
	}

	.y-axis {
		position: absolute;
		left: 0;
		top: 0;
		bottom: 40px;
		width: 60px;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		padding-right: 12px;
		text-align: right;
		font-size: 11px;
		color: #86868b;
	}

	.x-axis {
		position: absolute;
		bottom: 0;
		left: 60px;
		right: 0;
		height: 40px;
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		padding-top: 12px;
		font-size: 11px;
		color: #86868b;
	}

	/* Format Section */
	.format-section {
		margin: 0 24px 24px;
		padding: 24px;
		background: white;
		border-radius: 16px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.format-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 16px;
	}

	.format-card {
		padding: 20px;
		background: #f5f5f7;
		border-radius: 12px;
	}

	.format-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 16px;
	}

	.format-name {
		font-size: 16px;
		font-weight: 600;
		color: #1d1d1f;
	}

	.format-count {
		font-size: 13px;
		color: #86868b;
	}

	.format-bar {
		position: relative;
		height: 32px;
		margin-bottom: 12px;
	}

	.bar-original,
	.bar-optimized {
		position: absolute;
		top: 0;
		left: 0;
		height: 100%;
		border-radius: 6px;
		display: flex;
		align-items: center;
		padding: 0 12px;
	}

	.bar-original {
		background: rgba(230, 184, 0, 0.2);
		width: 100%;
	}

	.bar-optimized {
		background: linear-gradient(90deg, #10b981, #34d399);
	}

	.bar-label {
		font-size: 11px;
		font-weight: 500;
		color: #1d1d1f;
		white-space: nowrap;
	}

	.bar-optimized .bar-label {
		color: white;
	}

	.format-savings {
		display: flex;
		align-items: baseline;
		gap: 6px;
	}

	.savings-value {
		font-size: 24px;
		font-weight: 700;
		color: #10b981;
	}

	.savings-label {
		font-size: 13px;
		color: #86868b;
	}

	/* Insights Section */
	.insights-section {
		margin: 0 24px;
		padding: 24px;
		background: white;
		border-radius: 16px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.insights-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 16px;
	}

	.insight-card {
		display: flex;
		gap: 16px;
		padding: 20px;
		background: #f5f5f7;
		border-radius: 12px;
	}

	.insight-icon {
		width: 40px;
		height: 40px;
		border-radius: 10px;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	/* FIX-2026-04-26: :global() so styles reach Tabler's internal <svg> */
	.insight-icon :global(svg) {
		width: 20px;
		height: 20px;
	}

	.insight-icon.green {
		background: #d1fae5;
		color: #059669;
	}
	.insight-icon.blue {
		background: #dbeafe;
		color: #2563eb;
	}
	.insight-icon.orange {
		background: #fef3c7;
		color: #d97706;
	}

	.insight-content h3 {
		font-size: 14px;
		font-weight: 600;
		color: #1d1d1f;
		margin: 0 0 6px 0;
	}

	.insight-content p {
		font-size: 13px;
		color: #86868b;
		margin: 0;
		line-height: 1.5;
	}

	/* Responsive */
	@media (max-width: 1199.98px) {
		.hero-stats {
			grid-template-columns: 1fr;
		}

		.stats-grid {
			grid-template-columns: repeat(3, 1fr);
		}

		.format-grid {
			grid-template-columns: 1fr;
		}

		.insights-grid {
			grid-template-columns: 1fr;
		}
	}

	@media (max-width: 767.98px) {
		.header {
			flex-direction: column;
			align-items: flex-start;
			gap: 16px;
		}

		.header-right {
			width: 100%;
			justify-content: space-between;
		}

		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}

		.hero-value {
			font-size: 24px;
		}

		.chart-container {
			height: 200px;
		}
	}
</style>

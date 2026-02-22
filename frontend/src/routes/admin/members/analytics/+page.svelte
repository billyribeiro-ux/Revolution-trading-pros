<script lang="ts">
import { logger } from '$lib/utils/logger';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import {
		IconArrowLeft,
		IconChartBar,
		IconChartLine,
		IconTrendingUp,
		IconTrendingDown,
		IconUsers,
		IconCurrencyDollar,
		IconRefresh,
		IconDownload,
		IconCalendar,
		IconSettings
	} from '$lib/icons';

	// Analytics data
	let loading = $state(true);
	let dateRange = $state('30d');

	// Connection status - NO MOCK DATA
	let _isConnected = false;
	let connectionError: string | null = $state(null);
	let hasData = $state(false);

	// Metrics - Start with null values, no fake data
	let metrics = $state({
		totalMembers: null as number | null,
		memberGrowth: null as number | null,
		mrr: null as number | null,
		mrrGrowth: null as number | null,
		churnRate: null as number | null,
		churnChange: null as number | null,
		avgLtv: null as number | null,
		ltvGrowth: null as number | null
	});

	// Chart data - Empty arrays, no fake data
	let growthData: { month: string; members: number; new: number; churned: number }[] = $state([]);
	let cohortData: {
		cohort: string;
		m0: number;
		m1: number;
		m2: number;
		m3: number;
		m4: number;
		m5: number;
	}[] = $state([]);
	let revenueData: {
		month: string;
		mrr: number;
		expansion: number;
		contraction: number;
		churn: number;
	}[] = $state([]);
	let churnReasons: { reason: string; count: number; percentage: number }[] = $state([]);
	let segmentData: { segment: string; count: number; revenue: number; churnRate: number }[] =
		$state([]);

	onMount(async () => {
		await loadAnalytics();
	});

	async function loadAnalytics() {
		loading = true;
		connectionError = null;

		try {
			// Call REAL API - no mock data fallback
			const [metricsRes, growthRes, cohortRes, revenueRes, churnRes, segmentRes] =
				await Promise.allSettled([
					fetch(`/api/admin/members/analytics/metrics?range=${dateRange}`),
					fetch(`/api/admin/members/analytics/growth?range=${dateRange}`),
					fetch(`/api/admin/members/analytics/cohorts?range=${dateRange}`),
					fetch(`/api/admin/members/analytics/revenue?range=${dateRange}`),
					fetch(`/api/admin/members/analytics/churn-reasons?range=${dateRange}`),
					fetch(`/api/admin/members/analytics/segments?range=${dateRange}`)
				]);

			let dataReceived = false;

			// Process metrics
			if (metricsRes.status === 'fulfilled' && metricsRes.value.ok) {
				const data = await metricsRes.value.json();
				if (data && typeof data.totalMembers === 'number') {
					metrics = data;
					dataReceived = true;
				}
			}

			// Process growth data
			if (growthRes.status === 'fulfilled' && growthRes.value.ok) {
				const data = await growthRes.value.json();
				if (Array.isArray(data) && data.length > 0) {
					growthData = data;
					dataReceived = true;
				}
			}

			// Process cohort data
			if (cohortRes.status === 'fulfilled' && cohortRes.value.ok) {
				const data = await cohortRes.value.json();
				if (Array.isArray(data) && data.length > 0) {
					cohortData = data;
					dataReceived = true;
				}
			}

			// Process revenue data
			if (revenueRes.status === 'fulfilled' && revenueRes.value.ok) {
				const data = await revenueRes.value.json();
				if (Array.isArray(data) && data.length > 0) {
					revenueData = data;
					dataReceived = true;
				}
			}

			// Process churn reasons
			if (churnRes.status === 'fulfilled' && churnRes.value.ok) {
				const data = await churnRes.value.json();
				if (Array.isArray(data) && data.length > 0) {
					churnReasons = data;
					dataReceived = true;
				}
			}

			// Process segment data
			if (segmentRes.status === 'fulfilled' && segmentRes.value.ok) {
				const data = await segmentRes.value.json();
				if (Array.isArray(data) && data.length > 0) {
					segmentData = data;
					dataReceived = true;
				}
			}

			_isConnected = dataReceived;
			hasData = dataReceived;

			if (!dataReceived) {
				connectionError =
					'Member analytics data is not available. Ensure your analytics service is connected and configured.';
			}
		} catch (err) {
			logger.error('Failed to load member analytics:', err);
			_isConnected = false;
			hasData = false;
			connectionError =
				'Failed to connect to analytics service. Please check your connection settings.';
			// NO MOCK DATA - Show connection error instead
		} finally {
			loading = false;
		}
	}

	function formatCurrency(amount: number): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD',
			minimumFractionDigits: 0,
			maximumFractionDigits: 0
		}).format(amount);
	}

	function formatNumber(num: number): string {
		return new Intl.NumberFormat('en-US').format(num);
	}

	function getRetentionColor(value: number): string {
		if (value === 0) return 'bg-slate-800/50';
		if (value >= 90) return 'bg-emerald-500/60';
		if (value >= 80) return 'bg-emerald-500/40';
		if (value >= 70) return 'bg-yellow-500/40';
		if (value >= 60) return 'bg-orange-500/40';
		return 'bg-red-500/40';
	}

	function getMaxValue(data: number[]): number {
		return Math.max(...data);
	}
</script>

<svelte:head>
	<title>Member Analytics | Revolution Trading Pros</title>
</svelte:head>

<div class="analytics-page">
	<!-- Header -->
	<div class="page-header">
		<button class="back-btn" onclick={() => goto('/admin/members')}>
			<IconArrowLeft size={20} />
			Back to Members
		</button>

		<div class="header-content">
			<div class="header-title">
				<div class="title-icon">
					<IconChartBar size={28} />
				</div>
				<div>
					<h1>Member Analytics</h1>
					<p class="subtitle">Comprehensive insights and metrics</p>
				</div>
			</div>

			<div class="header-actions">
				<div class="date-filter">
					<IconCalendar size={18} />
					<select bind:value={dateRange} onchange={loadAnalytics}>
						<option value="7d">Last 7 days</option>
						<option value="30d">Last 30 days</option>
						<option value="90d">Last 90 days</option>
						<option value="12m">Last 12 months</option>
					</select>
				</div>
				<button class="btn-secondary" onclick={loadAnalytics}>
					<IconRefresh size={18} />
					Refresh
				</button>
				<button class="btn-primary">
					<IconDownload size={18} />
					Export Report
				</button>
			</div>
		</div>
	</div>

	{#if loading}
		<div class="loading-grid">
			{#each [1, 2, 3, 4] as _}
				<div class="skeleton skeleton-metric"></div>
			{/each}
		</div>
		<div class="skeleton skeleton-chart"></div>
	{:else if !hasData || connectionError}
		<!-- NOT CONNECTED STATE - No fake data -->
		<div class="not-connected-state">
			<div class="not-connected-card">
				<div class="not-connected-icon">
					<IconChartBar size={40} />
				</div>
				<h2>Member Analytics Not Available</h2>
				<p>
					{connectionError ||
						'Connect your analytics service to view real member metrics, growth data, and churn analysis.'}
				</p>
				<div class="not-connected-actions">
					<a href="/admin/connections" class="btn-connect">
						<IconSettings size={18} />
						Connect Analytics
					</a>
					<button class="btn-retry" onclick={loadAnalytics}>
						<IconRefresh size={18} />
						Retry
					</button>
				</div>
				<div class="analytics-features">
					<h3>Available when connected:</h3>
					<ul>
						<li>Real-time member metrics & growth trends</li>
						<li>Monthly Recurring Revenue (MRR) tracking</li>
						<li>Cohort retention analysis</li>
						<li>Churn reason insights</li>
						<li>Segment performance breakdowns</li>
					</ul>
				</div>
			</div>
		</div>
	{:else}
		<!-- Key Metrics -->
		<div class="metrics-grid">
			<div class="metric-card">
				<div class="metric-header">
					<div class="metric-icon purple">
						<IconUsers size={24} />
					</div>
					<div class="metric-trend {(metrics.memberGrowth ?? 0) >= 0 ? 'positive' : 'negative'}">
						{#if (metrics.memberGrowth ?? 0) >= 0}
							<IconTrendingUp size={16} />
						{:else}
							<IconTrendingDown size={16} />
						{/if}
						{Math.abs(metrics.memberGrowth ?? 0)}%
					</div>
				</div>
				<div class="metric-value">{formatNumber(metrics.totalMembers ?? 0)}</div>
				<div class="metric-label">Total Members</div>
			</div>

			<div class="metric-card">
				<div class="metric-header">
					<div class="metric-icon emerald">
						<IconCurrencyDollar size={24} />
					</div>
					<div class="metric-trend {(metrics.mrrGrowth ?? 0) >= 0 ? 'positive' : 'negative'}">
						{#if (metrics.mrrGrowth ?? 0) >= 0}
							<IconTrendingUp size={16} />
						{:else}
							<IconTrendingDown size={16} />
						{/if}
						{Math.abs(metrics.mrrGrowth ?? 0)}%
					</div>
				</div>
				<div class="metric-value">{formatCurrency(metrics.mrr ?? 0)}</div>
				<div class="metric-label">Monthly Recurring Revenue</div>
			</div>

			<div class="metric-card">
				<div class="metric-header">
					<div class="metric-icon {(metrics.churnChange ?? 0) <= 0 ? 'emerald' : 'red'}">
						<IconChartLine size={24} />
					</div>
					<div class="metric-trend {(metrics.churnChange ?? 0) <= 0 ? 'positive' : 'negative'}">
						{#if (metrics.churnChange ?? 0) <= 0}
							<IconTrendingDown size={16} />
						{:else}
							<IconTrendingUp size={16} />
						{/if}
						{Math.abs(metrics.churnChange ?? 0)}%
					</div>
				</div>
				<div class="metric-value">{metrics.churnRate}%</div>
				<div class="metric-label">Churn Rate</div>
			</div>

			<div class="metric-card">
				<div class="metric-header">
					<div class="metric-icon blue">
						<IconChartLine size={24} />
					</div>
					<div class="metric-trend {(metrics.ltvGrowth ?? 0) >= 0 ? 'positive' : 'negative'}">
						{#if (metrics.ltvGrowth ?? 0) >= 0}
							<IconTrendingUp size={16} />
						{:else}
							<IconTrendingDown size={16} />
						{/if}
						{Math.abs(metrics.ltvGrowth ?? 0)}%
					</div>
				</div>
				<div class="metric-value">{formatCurrency(metrics.avgLtv ?? 0)}</div>
				<div class="metric-label">Average LTV</div>
			</div>
		</div>

		<!-- Charts Grid -->
		<div class="charts-grid">
			<!-- Member Growth Chart -->
			<div class="chart-card">
				<div class="chart-header">
					<h3>Member Growth</h3>
					<div class="chart-legend">
						<span class="legend-item"><span class="legend-dot purple"></span> Total</span>
						<span class="legend-item"><span class="legend-dot emerald"></span> New</span>
						<span class="legend-item"><span class="legend-dot red"></span> Churned</span>
					</div>
				</div>
				<div class="chart-body">
					<div class="bar-chart">
						{#each growthData as data}
							<div class="bar-group">
								<div class="bar-container">
									<div
										class="bar bar-members"
										style="height: {(data.members / getMaxValue(growthData.map((d) => d.members))) *
											100}%"
									></div>
								</div>
								<div class="bar-mini-group">
									<div
										class="bar bar-new"
										style="height: {(data.new / getMaxValue(growthData.map((d) => d.new))) * 60}px"
									></div>
									<div
										class="bar bar-churned"
										style="height: {(data.churned / getMaxValue(growthData.map((d) => d.churned))) *
											60}px"
									></div>
								</div>
								<span class="bar-label">{data.month}</span>
							</div>
						{/each}
					</div>
				</div>
			</div>

			<!-- MRR Trend Chart -->
			<div class="chart-card">
				<div class="chart-header">
					<h3>Revenue Trend</h3>
					<div class="chart-legend">
						<span class="legend-item"><span class="legend-dot emerald"></span> MRR</span>
					</div>
				</div>
				<div class="chart-body">
					<div class="line-chart">
						<svg viewBox="0 0 600 200" class="line-chart-svg">
							<!-- Grid lines -->
							{#each [0, 1, 2, 3, 4] as i}
								<line
									x1="0"
									y1={i * 50}
									x2="600"
									y2={i * 50}
									stroke="rgba(148, 163, 184, 0.1)"
									stroke-dasharray="4"
								/>
							{/each}

							<!-- Area fill -->
							<path
								d="M 0 200 {revenueData
									.map((d, i) => {
										const x = (i / (revenueData.length - 1)) * 600;
										const y = 200 - (d.mrr / 100000) * 180;
										return `L ${x} ${y}`;
									})
									.join(' ')} L 600 200 Z"
								fill="url(#gradient)"
								opacity="0.3"
							/>

							<!-- Line -->
							<path
								d="M {revenueData
									.map((d, i) => {
										const x = (i / (revenueData.length - 1)) * 600;
										const y = 200 - (d.mrr / 100000) * 180;
										return `${i === 0 ? 'M' : 'L'} ${x} ${y}`;
									})
									.join(' ')}"
								fill="none"
								stroke="#34d399"
								stroke-width="3"
								stroke-linecap="round"
								stroke-linejoin="round"
							/>

							<!-- Points -->
							{#each revenueData as d, i}
								<circle
									cx={(i / (revenueData.length - 1)) * 600}
									cy={200 - (d.mrr / 100000) * 180}
									r="5"
									fill="#34d399"
								/>
							{/each}

							<!-- Gradient definition -->
							<defs>
								<linearGradient id="gradient" x1="0%" y1="0%" x2="0%" y2="100%">
									<stop offset="0%" style="stop-color:#34d399;stop-opacity:0.4" />
									<stop offset="100%" style="stop-color:#34d399;stop-opacity:0" />
								</linearGradient>
							</defs>
						</svg>
						<div class="line-chart-labels">
							{#each revenueData as d}
								<span>{d.month}</span>
							{/each}
						</div>
					</div>
				</div>
			</div>
		</div>

		<!-- Cohort Analysis -->
		<div class="chart-card full-width">
			<div class="chart-header">
				<h3>Retention Cohort Analysis</h3>
				<p class="chart-subtitle">Monthly retention rates by signup cohort</p>
			</div>
			<div class="chart-body">
				<div class="cohort-table">
					<table>
						<thead>
							<tr>
								<th>Cohort</th>
								<th>Month 0</th>
								<th>Month 1</th>
								<th>Month 2</th>
								<th>Month 3</th>
								<th>Month 4</th>
								<th>Month 5</th>
							</tr>
						</thead>
						<tbody>
							{#each cohortData as row}
								<tr>
									<td class="cohort-name">{row.cohort}</td>
									<td class={getRetentionColor(row.m0)}>{row.m0}%</td>
									<td class={getRetentionColor(row.m1)}>{row.m1 || '-'}%</td>
									<td class={getRetentionColor(row.m2)}>{row.m2 || '-'}%</td>
									<td class={getRetentionColor(row.m3)}>{row.m3 || '-'}%</td>
									<td class={getRetentionColor(row.m4)}>{row.m4 || '-'}%</td>
									<td class={getRetentionColor(row.m5)}>{row.m5 || '-'}%</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
			</div>
		</div>

		<!-- Bottom Row -->
		<div class="bottom-grid">
			<!-- Churn Reasons -->
			<div class="chart-card">
				<div class="chart-header">
					<h3>Churn Reasons</h3>
				</div>
				<div class="chart-body">
					<div class="reasons-list">
						{#each churnReasons as reason}
							<div class="reason-item">
								<div class="reason-info">
									<span class="reason-name">{reason.reason}</span>
									<span class="reason-count">{reason.count} members</span>
								</div>
								<div class="reason-bar-container">
									<div class="reason-bar" style="width: {reason.percentage}%"></div>
								</div>
								<span class="reason-percentage">{reason.percentage}%</span>
							</div>
						{/each}
					</div>
				</div>
			</div>

			<!-- Segment Performance -->
			<div class="chart-card">
				<div class="chart-header">
					<h3>Segment Performance</h3>
				</div>
				<div class="chart-body">
					<div class="segment-table">
						<table>
							<thead>
								<tr>
									<th>Segment</th>
									<th>Members</th>
									<th>Revenue</th>
									<th>Churn</th>
								</tr>
							</thead>
							<tbody>
								{#each segmentData as segment}
									<tr>
										<td class="segment-name">{segment.segment}</td>
										<td>{formatNumber(segment.count)}</td>
										<td class="revenue">{formatCurrency(segment.revenue)}</td>
										<td
											class="churn-rate {segment.churnRate <= 2
												? 'low'
												: segment.churnRate <= 5
													? 'medium'
													: 'high'}"
										>
											{segment.churnRate}%
										</td>
									</tr>
								{/each}
							</tbody>
						</table>
					</div>
				</div>
			</div>
		</div>
	{/if}
</div>

<style>
	.analytics-page {
		padding: 2rem;
		max-width: 1600px;
		margin: 0 auto;
	}

	/* Header */
	.back-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0;
		color: #94a3b8;
		background: none;
		border: none;
		cursor: pointer;
		font-size: 0.875rem;
		margin-bottom: 1rem;
		transition: color 0.2s;
	}

	.back-btn:hover {
		color: var(--primary-400);
	}

	.header-content {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 2rem;
	}

	.header-title {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.title-icon {
		width: 56px;
		height: 56px;
		background: linear-gradient(135deg, var(--primary-500) 0%, var(--primary-600) 100%);
		border-radius: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
	}

	.header-title h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0;
	}

	.subtitle {
		color: #64748b;
		margin: 0.25rem 0 0;
	}

	.header-actions {
		display: flex;
		gap: 0.75rem;
		align-items: center;
	}

	.date-filter {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 10px;
		color: #94a3b8;
	}

	.date-filter select {
		background: none;
		border: none;
		color: #f1f5f9;
		font-size: 0.875rem;
		cursor: pointer;
	}

	/* Buttons */
	.btn-primary,
	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1.125rem;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
		font-size: 0.875rem;
	}

	.btn-primary {
		background: linear-gradient(135deg, var(--primary-500) 0%, var(--primary-600) 100%);
		color: var(--bg-base);
	}

	.btn-secondary {
		background: rgba(148, 163, 184, 0.1);
		color: #94a3b8;
		border: 1px solid rgba(148, 163, 184, 0.2);
	}

	/* Loading */
	.loading-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.skeleton {
		background: linear-gradient(
			90deg,
			rgba(148, 163, 184, 0.1) 25%,
			rgba(148, 163, 184, 0.2) 50%,
			rgba(148, 163, 184, 0.1) 75%
		);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 16px;
	}

	.skeleton-metric {
		height: 140px;
	}

	.skeleton-chart {
		height: 400px;
	}

	@keyframes shimmer {
		0% {
			background-position: 200% 0;
		}
		100% {
			background-position: -200% 0;
		}
	}

	/* Not Connected State - NO MOCK DATA */
	.not-connected-state {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 3rem 1.5rem;
		min-height: 400px;
	}

	.not-connected-card {
		max-width: 500px;
		text-align: center;
		padding: 3rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 20px;
	}

	.not-connected-icon {
		width: 80px;
		height: 80px;
		margin: 0 auto 1.5rem;
		background: linear-gradient(135deg, rgba(251, 146, 60, 0.2), rgba(249, 115, 22, 0.3));
		border-radius: 20px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #fb923c;
	}

	.not-connected-card h2 {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.75rem;
	}

	.not-connected-card > p {
		font-size: 0.9375rem;
		color: #94a3b8;
		line-height: 1.6;
		margin: 0 0 1.5rem;
	}

	.not-connected-actions {
		display: flex;
		gap: 0.75rem;
		justify-content: center;
		margin-bottom: 2rem;
	}

	.btn-connect {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: linear-gradient(135deg, var(--primary-500) 0%, var(--primary-600) 100%);
		color: var(--bg-base);
		font-size: 0.875rem;
		font-weight: 600;
		border-radius: 10px;
		text-decoration: none;
		transition: all 0.2s;
	}

	.btn-connect:hover {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(230, 184, 0, 0.3);
	}

	.btn-retry {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: rgba(148, 163, 184, 0.1);
		color: #94a3b8;
		font-size: 0.875rem;
		font-weight: 500;
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 10px;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-retry:hover {
		background: rgba(148, 163, 184, 0.15);
		color: #f1f5f9;
	}

	.analytics-features {
		padding-top: 1.5rem;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
	}

	.analytics-features h3 {
		font-size: 0.75rem;
		font-weight: 600;
		color: #64748b;
		margin: 0 0 1rem;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.analytics-features ul {
		list-style: none;
		padding: 0;
		margin: 0;
		text-align: left;
	}

	.analytics-features li {
		font-size: 0.875rem;
		color: #cbd5e1;
		padding: 0.5rem 0;
		padding-left: 1.5rem;
		position: relative;
	}

	.analytics-features li::before {
		content: '✓';
		position: absolute;
		left: 0;
		color: #34d399;
	}

	/* Metrics Grid */
	.metrics-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.metric-card {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
		padding: 1.5rem;
	}

	.metric-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 1rem;
	}

	.metric-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.metric-icon.purple {
		background: rgba(230, 184, 0, 0.15);
		color: var(--primary-400);
	}
	.metric-icon.emerald {
		background: rgba(16, 185, 129, 0.15);
		color: #34d399;
	}
	.metric-icon.blue {
		background: rgba(59, 130, 246, 0.15);
		color: #60a5fa;
	}
	.metric-icon.red {
		background: rgba(239, 68, 68, 0.15);
		color: #f87171;
	}

	.metric-trend {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-size: 0.8125rem;
		font-weight: 600;
	}

	.metric-trend.positive {
		color: #34d399;
	}
	.metric-trend.negative {
		color: #f87171;
	}

	.metric-value {
		font-size: 2rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 0.25rem;
	}

	.metric-label {
		font-size: 0.8125rem;
		color: #64748b;
	}

	/* Charts Grid */
	.charts-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1.5rem;
		margin-bottom: 1.5rem;
	}

	.chart-card {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
		overflow: hidden;
	}

	.chart-card.full-width {
		grid-column: 1 / -1;
	}

	.chart-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.25rem 1.5rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.chart-header h3 {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.chart-subtitle {
		font-size: 0.8125rem;
		color: #64748b;
		margin: 0.25rem 0 0;
	}

	.chart-legend {
		display: flex;
		gap: 1rem;
	}

	.legend-item {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.75rem;
		color: #94a3b8;
	}

	.legend-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
	}

	.legend-dot.purple {
		background: var(--primary-400);
	}
	.legend-dot.emerald {
		background: #34d399;
	}
	.legend-dot.red {
		background: #f87171;
	}

	.chart-body {
		padding: 1.5rem;
	}

	/* Bar Chart */
	.bar-chart {
		display: flex;
		align-items: flex-end;
		justify-content: space-between;
		height: 200px;
		gap: 0.5rem;
	}

	.bar-group {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
	}

	.bar-container {
		width: 100%;
		height: 140px;
		display: flex;
		align-items: flex-end;
		justify-content: center;
	}

	.bar {
		border-radius: 4px 4px 0 0;
		transition: height 0.3s ease;
	}

	.bar-members {
		width: 60%;
		background: linear-gradient(180deg, var(--primary-400), var(--primary-500));
	}

	.bar-mini-group {
		display: flex;
		gap: 4px;
		height: 60px;
		align-items: flex-end;
	}

	.bar-new {
		width: 16px;
		background: #34d399;
	}

	.bar-churned {
		width: 16px;
		background: #f87171;
	}

	.bar-label {
		font-size: 0.75rem;
		color: #64748b;
	}

	/* Line Chart */
	.line-chart {
		height: 200px;
	}

	.line-chart-svg {
		width: 100%;
		height: 180px;
	}

	.line-chart-labels {
		display: flex;
		justify-content: space-between;
		padding: 0.5rem 0;
		font-size: 0.75rem;
		color: #64748b;
	}

	/* Cohort Table */
	.cohort-table {
		overflow-x: auto;
	}

	.cohort-table table {
		width: 100%;
		border-collapse: collapse;
	}

	.cohort-table th,
	.cohort-table td {
		padding: 0.875rem 1rem;
		text-align: center;
		font-size: 0.8125rem;
	}

	.cohort-table th {
		color: #94a3b8;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		font-size: 0.6875rem;
	}

	.cohort-table td {
		color: #f1f5f9;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
	}

	.cohort-name {
		text-align: left;
		font-weight: 500;
		color: var(--primary-400);
	}

	/* Bottom Grid */
	.bottom-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1.5rem;
	}

	/* Reasons List */
	.reasons-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.reason-item {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.reason-info {
		width: 140px;
		flex-shrink: 0;
	}

	.reason-name {
		display: block;
		font-size: 0.875rem;
		color: #f1f5f9;
	}

	.reason-count {
		font-size: 0.75rem;
		color: #64748b;
	}

	.reason-bar-container {
		flex: 1;
		height: 8px;
		background: rgba(148, 163, 184, 0.1);
		border-radius: 4px;
		overflow: hidden;
	}

	.reason-bar {
		height: 100%;
		background: linear-gradient(90deg, #f87171, #ef4444);
		border-radius: 4px;
	}

	.reason-percentage {
		width: 40px;
		text-align: right;
		font-size: 0.875rem;
		font-weight: 600;
		color: #f87171;
	}

	/* Segment Table */
	.segment-table table {
		width: 100%;
		border-collapse: collapse;
	}

	.segment-table th,
	.segment-table td {
		padding: 0.875rem 1rem;
		text-align: left;
		font-size: 0.875rem;
	}

	.segment-table th {
		color: #94a3b8;
		font-weight: 600;
		font-size: 0.75rem;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.segment-table td {
		color: #cbd5e1;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
	}

	.segment-name {
		font-weight: 500;
		color: #f1f5f9;
	}

	.segment-table .revenue {
		color: #34d399;
		font-weight: 600;
	}

	.churn-rate.low {
		color: #34d399;
	}
	.churn-rate.medium {
		color: #fbbf24;
	}
	.churn-rate.high {
		color: #f87171;
	}

	@media (max-width: 1200px) {
		.metrics-grid {
			grid-template-columns: repeat(2, 1fr);
		}

		.charts-grid {
			grid-template-columns: 1fr;
		}

		.bottom-grid {
			grid-template-columns: 1fr;
		}
	}

	@media (max-width: 768px) {
		.header-content {
			flex-direction: column;
			gap: 1rem;
			align-items: flex-start;
		}

		.header-actions {
			flex-wrap: wrap;
		}

		.loading-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}
</style>

<script lang="ts">
	/**
	 * Admin Consent Analytics Dashboard
	 *
	 * Visual dashboard for monitoring consent metrics:
	 * - Accept/reject rates
	 * - Time to decision
	 * - Category breakdown
	 * - A/B test results
	 * - Audit log viewer
	 */
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import {
		getAnalyticsSummary,
		getAuditLog,
		getAuditStats,
		scanCookies,
		exportConsentData
	} from '$lib/consent';
	import { getABTestAnalytics, exportABTestData } from '$lib/consent/ab-testing';
	import { getVersionInfo } from '$lib/consent/versioning';
	import type { ConsentAnalytics, ConsentAuditEntry } from '$lib/consent';

	let analytics: ConsentAnalytics | null = null;
	let auditLog: ConsentAuditEntry[] = [];
	let auditStats: ReturnType<typeof getAuditStats> | null = null;
	let cookieScan: ReturnType<typeof scanCookies> | null = null;
	let abTestAnalytics: ReturnType<typeof getABTestAnalytics> = [];
	let versionInfo = getVersionInfo();
	let insights: string[] = [];

	let activeTab: 'overview' | 'audit' | 'cookies' | 'ab-tests' = 'overview';

	onMount(() => {
		if (browser) {
			loadData();
		}
	});

	function loadData() {
		const summary = getAnalyticsSummary();
		analytics = summary.analytics;
		insights = summary.insights;

		auditLog = getAuditLog();
		auditStats = getAuditStats();
		cookieScan = scanCookies();
		abTestAnalytics = getABTestAnalytics();
	}

	async function handleExportData() {
		const data = await exportConsentData();
		downloadJSON(data, 'consent-data-export.json');
	}

	function handleExportABData() {
		const data = exportABTestData();
		downloadJSON(data, 'ab-test-export.json');
	}

	function downloadJSON(data: string, filename: string) {
		const blob = new Blob([data], { type: 'application/json' });
		const url = URL.createObjectURL(blob);
		const a = document.createElement('a');
		a.href = url;
		a.download = filename;
		document.body.appendChild(a);
		a.click();
		document.body.removeChild(a);
		URL.revokeObjectURL(url);
	}

	function formatDate(date: string): string {
		return new Date(date).toLocaleString();
	}

	function formatPercent(value: number): string {
		return `${(value * 100).toFixed(1)}%`;
	}

	function formatDuration(ms: number): string {
		if (ms < 1000) return `${ms}ms`;
		return `${(ms / 1000).toFixed(1)}s`;
	}
</script>

<svelte:head>
	<title>Consent Analytics | Admin</title>
</svelte:head>

<div class="admin-consent">
	<div class="admin-page-container">
		<!-- Animated Background -->
		<div class="bg-effects">
			<div class="bg-blob bg-blob-1"></div>
			<div class="bg-blob bg-blob-2"></div>
			<div class="bg-blob bg-blob-3"></div>
		</div>

		<header class="header">
			<div class="header-content">
				<h1>Consent Analytics Dashboard</h1>
				<p>Monitor consent metrics, audit logs, and A/B test performance</p>
			</div>
			<div class="header-actions">
				<button class="btn btn-secondary" onclick={loadData}>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="16"
						height="16"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" />
						<path d="M3 3v5h5" />
						<path d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16" />
						<path d="M16 21h5v-5" />
					</svg>
					Refresh
				</button>
				<button class="btn btn-primary" onclick={handleExportData}>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="16"
						height="16"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
						<polyline points="7 10 12 15 17 10" />
						<line x1="12" y1="15" x2="12" y2="3" />
					</svg>
					Export All Data
				</button>
			</div>
		</header>

		<!-- Quick Actions -->
		<div class="quick-actions">
			<a href="/admin/consent/settings" class="action-card">
				<span class="action-icon">⚙️</span>
				<span class="action-label">Settings</span>
				<span class="action-desc">Configure consent system</span>
			</a>
			<a href="/admin/consent/templates" class="action-card">
				<span class="action-icon">🎨</span>
				<span class="action-label">Templates</span>
				<span class="action-desc">Customize banner design</span>
			</a>
		</div>

		<!-- Tabs -->
		<nav class="tabs">
			<button
				class="tab"
				class:active={activeTab === 'overview'}
				onclick={() => (activeTab = 'overview')}
			>
				Overview
			</button>
			<button
				class="tab"
				class:active={activeTab === 'audit'}
				onclick={() => (activeTab = 'audit')}
			>
				Audit Log
			</button>
			<button
				class="tab"
				class:active={activeTab === 'cookies'}
				onclick={() => (activeTab = 'cookies')}
			>
				Cookies
			</button>
			<button
				class="tab"
				class:active={activeTab === 'ab-tests'}
				onclick={() => (activeTab = 'ab-tests')}
			>
				A/B Tests
			</button>
		</nav>

		<!-- Overview Tab -->
		{#if activeTab === 'overview'}
			<div class="tab-content">
				<!-- Key Metrics -->
				{#if analytics}
					<section class="section">
						<h2>Key Metrics</h2>
						<div class="metrics-grid">
							<div class="metric-card">
								<span class="metric-value">{analytics.totalInteractions}</span>
								<span class="metric-label">Total Interactions</span>
							</div>
							<div class="metric-card highlight-green">
								<span class="metric-value">{formatPercent(analytics.acceptAllRate)}</span>
								<span class="metric-label">Accept All Rate</span>
							</div>
							<div class="metric-card highlight-red">
								<span class="metric-value">{formatPercent(analytics.rejectAllRate)}</span>
								<span class="metric-label">Reject All Rate</span>
							</div>
							<div class="metric-card">
								<span class="metric-value">{formatPercent(analytics.customRate)}</span>
								<span class="metric-label">Custom Preferences</span>
							</div>
							<div class="metric-card">
								<span class="metric-value">{formatDuration(analytics.avgTimeToDecision)}</span>
								<span class="metric-label">Avg. Decision Time</span>
							</div>
							<div class="metric-card">
								<span class="metric-value">{analytics.bannerImpressions}</span>
								<span class="metric-label">Banner Impressions</span>
							</div>
						</div>
					</section>

					<!-- Category Rates -->
					<section class="section">
						<h2>Category Consent Rates</h2>
						<div class="category-bars">
							{#each Object.entries(analytics.categoryRates) as [category, rate]}
								<div class="category-bar-item">
									<div class="category-bar-header">
										<span class="category-name">{category}</span>
										<span class="category-value">{formatPercent(rate)}</span>
									</div>
									<div class="category-bar-track">
										<div
											class="category-bar-fill"
											style:width={`${rate * 100}%`}
											class:low={rate < 0.3}
											class:medium={rate >= 0.3 && rate < 0.6}
											class:high={rate >= 0.6}
										></div>
									</div>
								</div>
							{/each}
						</div>
					</section>
				{/if}

				<!-- Insights -->
				{#if insights.length > 0}
					<section class="section">
						<h2>Insights</h2>
						<div class="insights-list">
							{#each insights as insight}
								<div class="insight-item">
									<span class="insight-icon">💡</span>
									<span>{insight}</span>
								</div>
							{/each}
						</div>
					</section>
				{/if}

				<!-- Version Info -->
				<section class="section">
					<h2>Policy Version</h2>
					<div class="version-info">
						<p><strong>Current Version:</strong> {versionInfo.version}</p>
						<p><strong>Last Updated:</strong> {versionInfo.formattedDate}</p>
					</div>
				</section>
			</div>
		{/if}

		<!-- Audit Log Tab -->
		{#if activeTab === 'audit'}
			<div class="tab-content">
				{#if auditStats}
					<section class="section">
						<h2>Audit Statistics</h2>
						<div class="metrics-grid small">
							<div class="metric-card">
								<span class="metric-value">{auditStats.totalEntries}</span>
								<span class="metric-label">Total Entries</span>
							</div>
							<div class="metric-card">
								<span class="metric-value">{auditStats.consentGivenCount}</span>
								<span class="metric-label">Consent Given</span>
							</div>
							<div class="metric-card">
								<span class="metric-value">{auditStats.consentUpdatedCount}</span>
								<span class="metric-label">Consent Updated</span>
							</div>
							<div class="metric-card">
								<span class="metric-value">{auditStats.consentRevokedCount}</span>
								<span class="metric-label">Consent Revoked</span>
							</div>
						</div>
					</section>
				{/if}

				<section class="section">
					<h2>Audit Log ({auditLog.length} entries)</h2>
					{#if auditLog.length > 0}
						<div class="table-wrapper">
							<table class="data-table">
								<thead>
									<tr>
										<th>Timestamp</th>
										<th>Action</th>
										<th>Method</th>
										<th>Categories</th>
									</tr>
								</thead>
								<tbody>
									{#each auditLog.slice().reverse() as entry}
										<tr>
											<td>{formatDate(entry.timestamp)}</td>
											<td>
												<span
													class="action-badge"
													class:given={entry.action === 'consent_given'}
													class:updated={entry.action === 'consent_updated'}
													class:revoked={entry.action === 'consent_revoked'}
												>
													{entry.action.replace('consent_', '')}
												</span>
											</td>
											<td>{entry.method || 'N/A'}</td>
											<td>
												{#if entry.categories}
													<span class="categories-list">
														{Object.entries(entry.categories)
															.filter(([, v]) => v)
															.map(([k]) => k)
															.join(', ') || 'None'}
													</span>
												{/if}
											</td>
										</tr>
									{/each}
								</tbody>
							</table>
						</div>
					{:else}
						<p class="empty-state">No audit entries yet.</p>
					{/if}
				</section>
			</div>
		{/if}

		<!-- Cookies Tab -->
		{#if activeTab === 'cookies'}
			<div class="tab-content">
				{#if cookieScan}
					<section class="section">
						<h2>Cookie Summary</h2>
						<div class="metrics-grid small">
							<div class="metric-card">
								<span class="metric-value">{cookieScan.totalCookies}</span>
								<span class="metric-label">Total Cookies</span>
							</div>
							<div class="metric-card highlight-green">
								<span class="metric-value">{cookieScan.categorizedCookies}</span>
								<span class="metric-label">Categorized</span>
							</div>
							<div class="metric-card highlight-yellow">
								<span class="metric-value">{cookieScan.uncategorizedCookies}</span>
								<span class="metric-label">Uncategorized</span>
							</div>
						</div>
					</section>

					{#each Object.entries(cookieScan.byCategory) as [category, cookies]}
						{#if cookies.length > 0}
							<section class="section">
								<h2 class="category-header">
									<span class="category-badge {category}">{category}</span>
									({cookies.length} cookies)
								</h2>
								<div class="table-wrapper">
									<table class="data-table">
										<thead>
											<tr>
												<th>Name</th>
												<th>Purpose</th>
												<th>Duration</th>
												<th>Type</th>
											</tr>
										</thead>
										<tbody>
											{#each cookies as cookie}
												<tr>
													<td><code>{cookie.name}</code></td>
													<td>{cookie.purpose || 'Unknown'}</td>
													<td>{cookie.duration || 'Session'}</td>
													<td>{cookie.type || 'Unknown'}</td>
												</tr>
											{/each}
										</tbody>
									</table>
								</div>
							</section>
						{/if}
					{/each}
				{:else}
					<p class="empty-state">Loading cookie data...</p>
				{/if}
			</div>
		{/if}

		<!-- A/B Tests Tab -->
		{#if activeTab === 'ab-tests'}
			<div class="tab-content">
				<section class="section">
					<div class="section-header">
						<h2>A/B Test Results</h2>
						<button class="btn btn-secondary btn-sm" onclick={handleExportABData}>
							Export A/B Data
						</button>
					</div>

					{#if abTestAnalytics.length > 0}
						<div class="table-wrapper">
							<table class="data-table">
								<thead>
									<tr>
										<th>Variant</th>
										<th>Impressions</th>
										<th>Accept Rate</th>
										<th>Reject Rate</th>
										<th>Customize Rate</th>
										<th>Avg. Decision Time</th>
									</tr>
								</thead>
								<tbody>
									{#each abTestAnalytics as variant}
										<tr>
											<td><strong>{variant.variantId}</strong></td>
											<td>{variant.impressions}</td>
											<td class="highlight-green">{formatPercent(variant.acceptRate)}</td>
											<td class="highlight-red">{formatPercent(variant.rejectRate)}</td>
											<td>{formatPercent(variant.customizeRate)}</td>
											<td>{formatDuration(variant.avgTimeToDecision)}</td>
										</tr>
									{/each}
								</tbody>
							</table>
						</div>
					{:else}
						<p class="empty-state">
							No A/B test data yet. Tests will be recorded as users interact with the consent
							banner.
						</p>
					{/if}
				</section>
			</div>
		{/if}
	</div>
	<!-- End admin-page-container -->
</div>

<style>
	.dashboard {
		background: #0a101c;
		color: #e2e8f0;
		padding: 2rem;
	}

	.header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
		flex-wrap: wrap;
		gap: 1rem;
	}

	.header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		margin: 0;
	}

	.header p {
		color: #64748b;
		margin: 0.25rem 0 0;
	}

	.header-actions {
		display: flex;
		gap: 0.75rem;
	}

	.btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		border-radius: 8px;
		font-weight: 500;
		font-size: 0.875rem;
		cursor: pointer;
		border: none;
		transition: all 0.2s;
	}

	.btn-primary {
		background: linear-gradient(135deg, #0ea5e9, #06b6d4);
		color: white;
	}

	.btn-secondary {
		background: rgba(255, 255, 255, 0.1);
		color: white;
		border: 1px solid rgba(255, 255, 255, 0.2);
	}

	.btn-sm {
		padding: 0.375rem 0.75rem;
		font-size: 0.8rem;
	}

	.btn:hover {
		transform: translateY(-1px);
	}

	.quick-actions {
		display: flex;
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.action-card {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
		padding: 1rem 1.5rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 10px;
		text-decoration: none;
		color: inherit;
		transition: all 0.2s;
	}

	.action-card:hover {
		background: rgba(14, 165, 233, 0.1);
		border-color: rgba(14, 165, 233, 0.3);
		transform: translateY(-2px);
	}

	.action-icon {
		font-size: 1.5rem;
	}

	.action-label {
		font-weight: 600;
		font-size: 0.9rem;
		color: #f1f5f9;
	}

	.action-desc {
		font-size: 0.75rem;
		color: #64748b;
	}

	.tabs {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 2rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
		padding-bottom: 0;
	}

	.tab {
		padding: 0.75rem 1.5rem;
		background: transparent;
		border: none;
		color: #64748b;
		cursor: pointer;
		font-size: 0.875rem;
		font-weight: 500;
		border-bottom: 2px solid transparent;
		margin-bottom: -1px;
		transition: all 0.2s;
	}

	.tab:hover {
		color: #e2e8f0;
	}

	.tab.active {
		color: #0ea5e9;
		border-bottom-color: #0ea5e9;
	}

	.tab-content {
		animation: fadeIn 0.2s ease;
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	.section {
		margin-bottom: 2rem;
	}

	.section h2 {
		font-size: 1.1rem;
		font-weight: 600;
		margin-bottom: 1rem;
		color: #f1f5f9;
	}

	.section-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.metrics-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(160px, 1fr));
		gap: 1rem;
	}

	.metrics-grid.small {
		grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
	}

	.metric-card {
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		padding: 1.25rem;
		text-align: center;
	}

	.metric-value {
		display: block;
		font-size: 2rem;
		font-weight: 700;
		color: #f1f5f9;
		line-height: 1.2;
	}

	.metric-label {
		font-size: 0.75rem;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		margin-top: 0.25rem;
		display: block;
	}

	.highlight-green .metric-value {
		color: #4ade80;
	}
	.highlight-red .metric-value {
		color: #f87171;
	}
	.highlight-yellow .metric-value {
		color: #fbbf24;
	}

	.category-bars {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.category-bar-item {
		background: rgba(255, 255, 255, 0.05);
		padding: 1rem;
		border-radius: 8px;
	}

	.category-bar-header {
		display: flex;
		justify-content: space-between;
		margin-bottom: 0.5rem;
	}

	.category-name {
		text-transform: capitalize;
		font-weight: 500;
	}

	.category-value {
		color: #0ea5e9;
		font-weight: 600;
	}

	.category-bar-track {
		height: 8px;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 4px;
		overflow: hidden;
	}

	.category-bar-fill {
		height: 100%;
		border-radius: 4px;
		transition: width 0.3s ease;
	}

	.category-bar-fill.low {
		background: #ef4444;
	}
	.category-bar-fill.medium {
		background: #f59e0b;
	}
	.category-bar-fill.high {
		background: #22c55e;
	}

	.insights-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.insight-item {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		padding: 1rem;
		background: rgba(14, 165, 233, 0.1);
		border: 1px solid rgba(14, 165, 233, 0.2);
		border-radius: 8px;
	}

	.insight-icon {
		font-size: 1.25rem;
	}

	.version-info {
		background: rgba(255, 255, 255, 0.05);
		padding: 1rem;
		border-radius: 8px;
	}

	.version-info p {
		margin: 0.25rem 0;
	}

	.table-wrapper {
		overflow-x: auto;
	}

	.data-table {
		width: 100%;
		border-collapse: collapse;
		font-size: 0.875rem;
	}

	.data-table th,
	.data-table td {
		padding: 0.75rem;
		text-align: left;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	}

	.data-table th {
		font-weight: 600;
		color: #94a3b8;
		font-size: 0.75rem;
		text-transform: uppercase;
	}

	.data-table code {
		background: rgba(255, 255, 255, 0.1);
		padding: 0.2rem 0.4rem;
		border-radius: 4px;
		font-size: 0.8rem;
	}

	.action-badge {
		padding: 0.2rem 0.5rem;
		border-radius: 4px;
		font-size: 0.75rem;
		text-transform: capitalize;
		background: rgba(148, 163, 184, 0.2);
	}

	.action-badge.given {
		background: rgba(34, 197, 94, 0.2);
		color: #4ade80;
	}
	.action-badge.updated {
		background: rgba(59, 130, 246, 0.2);
		color: #60a5fa;
	}
	.action-badge.revoked {
		background: rgba(239, 68, 68, 0.2);
		color: #f87171;
	}

	.category-badge {
		padding: 0.25rem 0.75rem;
		border-radius: 4px;
		font-size: 0.875rem;
		text-transform: capitalize;
	}

	.category-badge.necessary {
		background: rgba(34, 197, 94, 0.2);
		color: #4ade80;
	}
	.category-badge.analytics {
		background: rgba(59, 130, 246, 0.2);
		color: #60a5fa;
	}
	.category-badge.marketing {
		background: rgba(168, 85, 247, 0.2);
		color: #c084fc;
	}
	.category-badge.preferences {
		background: rgba(245, 158, 11, 0.2);
		color: #fbbf24;
	}
	.category-badge.unknown {
		background: rgba(148, 163, 184, 0.2);
		color: #94a3b8;
	}

	.empty-state {
		text-align: center;
		color: #64748b;
		padding: 3rem;
		background: rgba(255, 255, 255, 0.02);
		border-radius: 8px;
	}

	.highlight-green {
		color: #4ade80;
	}
	.highlight-red {
		color: #f87171;
	}

	@media (max-width: 768px) {
		.dashboard {
			padding: 1rem;
		}

		.header {
			flex-direction: column;
		}

		.tabs {
			overflow-x: auto;
		}

		.metrics-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}
</style>

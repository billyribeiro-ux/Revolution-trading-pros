<script lang="ts">
	/**
	 * Reports - Custom Report Builder
	 *
	 * Create, schedule, and manage custom analytics reports
	 * with multiple visualization options.
	 */
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { analyticsApi } from '$lib/api/analytics';
	import { connections, getIsAnalyticsConnected } from '$lib/stores/connections.svelte';
	import ServiceConnectionStatus from '$lib/components/admin/ServiceConnectionStatus.svelte';
	import { toastStore } from '$lib/stores/toast.svelte';

	interface Report {
		id: string;
		name: string;
		description?: string;
		type: 'dashboard' | 'table' | 'chart' | 'mixed';
		schedule?: {
			frequency: 'daily' | 'weekly' | 'monthly';
			recipients: string[];
		};
		created_at: string;
		last_run?: string;
		status: 'active' | 'paused' | 'draft';
	}

	let reports = $state<Report[]>([]);
	let loading = $state(true);
	let connectionLoading = $state(true);
	let error = $state<string | null>(null);
	let showCreateModal = $state(false);
	let activeFilter = $state<'all' | 'active' | 'scheduled' | 'draft'>('all');

	const filterOptions: Array<{ value: typeof activeFilter; label: string }> = [
		{ value: 'all', label: 'All Reports' },
		{ value: 'active', label: 'Active' },
		{ value: 'scheduled', label: 'Scheduled' },
		{ value: 'draft', label: 'Drafts' }
	];

	// New report form
	let newReport = $state({
		name: '',
		description: '',
		type: 'dashboard' as 'dashboard' | 'table' | 'chart' | 'mixed',
		metrics: [] as string[],
		dimensions: [] as string[],
		schedule: false,
		frequency: 'weekly' as 'daily' | 'weekly' | 'monthly',
		recipients: ''
	});

	// Available metrics
	const availableMetrics = [
		{ value: 'revenue', label: 'Revenue' },
		{ value: 'users', label: 'Users' },
		{ value: 'sessions', label: 'Sessions' },
		{ value: 'page_views', label: 'Page Views' },
		{ value: 'conversions', label: 'Conversions' },
		{ value: 'conversion_rate', label: 'Conversion Rate' },
		{ value: 'arpu', label: 'ARPU' },
		{ value: 'ltv', label: 'LTV' },
		{ value: 'churn_rate', label: 'Churn Rate' },
		{ value: 'retention_rate', label: 'Retention Rate' }
	];

	// Available dimensions
	const availableDimensions = [
		{ value: 'date', label: 'Date' },
		{ value: 'channel', label: 'Channel' },
		{ value: 'country', label: 'Country' },
		{ value: 'device', label: 'Device' },
		{ value: 'browser', label: 'Browser' },
		{ value: 'page', label: 'Page' },
		{ value: 'campaign', label: 'Campaign' },
		{ value: 'segment', label: 'Segment' }
	];

	async function loadReports() {
		loading = true;
		error = null;
		try {
			const response = await analyticsApi.getReports();
			reports = response.reports || [];
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load reports';
		} finally {
			loading = false;
		}
	}

	async function createReport() {
		try {
			await analyticsApi.createReport({
				name: newReport.name,
				description: newReport.description,
				type: newReport.type,
				config: {
					metrics: newReport.metrics,
					dimensions: newReport.dimensions
				},
				...(newReport.schedule && {
					schedule: {
						frequency: newReport.frequency,
						recipients: newReport.recipients.split(',').map((e) => e.trim())
					}
				})
			});
			showCreateModal = false;
			newReport = {
				name: '',
				description: '',
				type: 'dashboard',
				metrics: [],
				dimensions: [],
				schedule: false,
				frequency: 'weekly',
				recipients: ''
			};
			loadReports();
		} catch (e) {
			toastStore.error(e instanceof Error ? e.message : 'Failed to create report');
		}
	}

	function toggleMetric(metric: string) {
		if (newReport.metrics.includes(metric)) {
			newReport.metrics = newReport.metrics.filter((m) => m !== metric);
		} else {
			newReport.metrics = [...newReport.metrics, metric];
		}
	}

	function toggleDimension(dimension: string) {
		if (newReport.dimensions.includes(dimension)) {
			newReport.dimensions = newReport.dimensions.filter((d) => d !== dimension);
		} else {
			newReport.dimensions = [...newReport.dimensions, dimension];
		}
	}

	function formatDate(dateStr: string): string {
		return new Date(dateStr).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric'
		});
	}

	// FIX-2026-04-26 (P1-3): $derived restores reactivity past helper's `untrack`.
	let isAnalyticsConnected = $derived(getIsAnalyticsConnected());

	// FIX-2026-04-26 (P1-1): onMount replaces the $effect cascade pattern.
	onMount(() => {
		if (!browser) return;

		(async () => {
			try {
				await connections.load();
			} catch (e) {
				if (import.meta.env.DEV) {
					console.error('[Reports] Failed to load connection status:', e);
				}
			} finally {
				connectionLoading = false;
			}

			if (getIsAnalyticsConnected()) {
				await loadReports();
			} else {
				loading = false;
			}
		})();
	});

	const filteredReports = $derived(
		reports.filter((report) => {
			if (activeFilter === 'all') return true;
			if (activeFilter === 'active') return report.status === 'active';
			if (activeFilter === 'scheduled') return report.schedule;
			if (activeFilter === 'draft') return report.status === 'draft';
			return true;
		})
	);

	// Report type icons
	const typeIcons: Record<string, string> = {
		dashboard: '📊',
		table: '📋',
		chart: '📈',
		mixed: '🔀'
	};

	function getFilterClass(value: typeof activeFilter): Record<string, boolean> {
		return {
			'filter-button': true,
			'filter-button--active': activeFilter === value
		};
	}

	function getReportStatusClass(status: Report['status']): Record<string, boolean> {
		return {
			'status-pill': true,
			'status-pill--active': status === 'active',
			'status-pill--draft': status === 'draft',
			'status-pill--paused': status === 'paused'
		};
	}

	function getToggleClass(selected: boolean, tone: 'blue' | 'purple'): Record<string, boolean> {
		return {
			'toggle-chip': true,
			'toggle-chip--blue': selected && tone === 'blue',
			'toggle-chip--purple': selected && tone === 'purple'
		};
	}
</script>

<svelte:head>
	<title>Reports | Analytics</title>
</svelte:head>

<div class="reports-page">
	<div class="reports-container">
		<!-- Header -->
		<div class="page-header">
			<div>
				<h1>Reports</h1>
				<p>Create and manage custom analytics reports</p>
			</div>
			{#if isAnalyticsConnected}
				<button type="button" onclick={() => (showCreateModal = true)} class="create-button">
					Create Report
				</button>
			{/if}
		</div>

		<!-- Connection Check -->
		{#if connectionLoading}
			<div class="loading-state">
				<div class="spinner-shell">
					<div class="spinner-track"></div>
					<div class="spinner"></div>
				</div>
			</div>
		{:else if !isAnalyticsConnected}
			<ServiceConnectionStatus feature="analytics" variant="card" showFeatures={true} />
		{:else}
			<!-- Filters -->
			<div class="filter-row">
				{#each filterOptions as filter (filter.value)}
					<button
						type="button"
						onclick={() => (activeFilter = filter.value)}
						class={getFilterClass(filter.value)}
					>
						{filter.label}
					</button>
				{/each}
			</div>

			{#if loading}
				<div class="loading-state">
					<div class="loader"></div>
				</div>
			{:else if error}
				<div class="error-card">
					<p>{error}</p>
					<button type="button" onclick={loadReports}> Retry </button>
				</div>
			{:else if filteredReports.length === 0}
				<div class="empty-card">
					<div class="empty-icon">📊</div>
					<h3>No Reports Yet</h3>
					<p>Create your first custom report</p>
					<button
						type="button"
						onclick={() => (showCreateModal = true)}
						class="primary-button primary-button--large"
					>
						Create Your First Report
					</button>
				</div>
			{:else}
				<!-- Reports Grid -->
				<div class="reports-grid">
					{#each filteredReports as report (report.id)}
						<div class="report-card">
							<div class="report-header">
								<div class="report-title-row">
									<span class="report-icon">{typeIcons[report.type] || '📊'}</span>
									<div>
										<h3>{report.name}</h3>
										<span class={getReportStatusClass(report.status)}>
											{report.status}
										</span>
									</div>
								</div>
								<button type="button" class="menu-button" aria-label="Report actions">⋮</button>
							</div>

							{#if report.description}
								<p class="report-description">{report.description}</p>
							{/if}

							<div class="report-meta">
								<div>
									<span>Created</span>
									<span>{formatDate(report.created_at)}</span>
								</div>
								{#if report.last_run}
									<div>
										<span>Last Run</span>
										<span>{formatDate(report.last_run)}</span>
									</div>
								{/if}
								{#if report.schedule}
									<div>
										<span>Schedule</span>
										<span class="capitalize-text">{report.schedule.frequency}</span>
									</div>
								{/if}
							</div>

							<div class="report-actions">
								<a href="/admin/analytics/reports/{report.id}" class="primary-button report-link">
									View Report
								</a>
								<button type="button" class="secondary-button"> Edit </button>
							</div>
						</div>
					{/each}
				</div>
			{/if}
		{/if}
	</div>
</div>

<!-- Create Report Modal -->
{#if showCreateModal}
	<div class="modal-overlay">
		<div class="modal-panel" role="dialog" aria-modal="true" aria-labelledby="create-report-title">
			<div class="modal-header">
				<div class="modal-header-row">
					<h2 id="create-report-title">Create Report</h2>
					<button
						type="button"
						onclick={() => (showCreateModal = false)}
						class="close-button"
						aria-label="Close create report modal"
					>
						✕
					</button>
				</div>
			</div>

			<div class="modal-body">
				<!-- Basic Info -->
				<div class="field-stack">
					<div>
						<label class="field-label" for="report-name">Name</label>
						<input
							type="text"
							id="report-name"
							name="report-name"
							bind:value={newReport.name}
							placeholder="e.g., Weekly Revenue Report"
							class="field-control"
						/>
					</div>
					<div>
						<label class="field-label" for="report-description">Description</label>
						<textarea
							id="report-description"
							bind:value={newReport.description}
							placeholder="Describe this report..."
							rows={2}
							class="field-control"
						></textarea>
					</div>
					<div>
						<label class="field-label" for="report-type">Type</label>
						<select id="report-type" bind:value={newReport.type} class="field-control">
							<option value="dashboard">Dashboard</option>
							<option value="table">Table</option>
							<option value="chart">Chart</option>
							<option value="mixed">Mixed</option>
						</select>
					</div>
				</div>

				<!-- Metrics -->
				<div>
					<span class="field-label field-label--static">Metrics</span>
					<div class="chip-row">
						{#each availableMetrics as metric (metric.value)}
							<button
								type="button"
								onclick={() => toggleMetric(metric.value)}
								class={getToggleClass(newReport.metrics.includes(metric.value), 'blue')}
							>
								{metric.label}
							</button>
						{/each}
					</div>
				</div>

				<!-- Dimensions -->
				<div>
					<span class="field-label field-label--static">Dimensions</span>
					<div class="chip-row">
						{#each availableDimensions as dimension (dimension.value)}
							<button
								type="button"
								onclick={() => toggleDimension(dimension.value)}
								class={getToggleClass(newReport.dimensions.includes(dimension.value), 'purple')}
							>
								{dimension.label}
							</button>
						{/each}
					</div>
				</div>

				<!-- Schedule -->
				<div class="schedule-section">
					<label class="checkbox-row">
						<input
							id="page-newreport-schedule"
							name="page-newreport-schedule"
							type="checkbox"
							bind:checked={newReport.schedule}
							class="checkbox-control"
						/>
						<span>Schedule this report</span>
					</label>

					{#if newReport.schedule}
						<div class="schedule-fields">
							<div>
								<label for="report-frequency" class="field-label">Frequency</label>
								<select
									id="report-frequency"
									bind:value={newReport.frequency}
									class="field-control"
								>
									<option value="daily">Daily</option>
									<option value="weekly">Weekly</option>
									<option value="monthly">Monthly</option>
								</select>
							</div>
							<div>
								<label for="report-recipients" class="field-label">
									Recipients (comma-separated)
								</label>
								<input
									id="report-recipients"
									name="report-recipients"
									type="text"
									bind:value={newReport.recipients}
									placeholder="email@example.com, another@example.com"
									class="field-control"
								/>
							</div>
						</div>
					{/if}
				</div>
			</div>

			<div class="modal-footer">
				<button type="button" onclick={() => (showCreateModal = false)} class="secondary-button">
					Cancel
				</button>
				<button
					type="button"
					onclick={createReport}
					disabled={!newReport.name || newReport.metrics.length === 0}
					class="primary-button"
				>
					Create Report
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.reports-page {
		min-height: 100vh;
		background: linear-gradient(135deg, #020617, #0f172a 48%, #020617);
		color: #ffffff;
	}

	.reports-container {
		max-width: 80rem;
		margin: 0 auto;
		padding: 2rem 1rem;
	}

	.page-header,
	.header-row,
	.filter-row,
	.report-header,
	.report-title-row,
	.report-meta div,
	.report-actions,
	.modal-header-row,
	.modal-footer,
	.checkbox-row {
		display: flex;
		align-items: center;
	}

	.page-header {
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		margin: 0;
		font-size: 1.5rem;
		font-weight: 700;
		line-height: 1.25;
	}

	.page-header p {
		margin: 0.25rem 0 0;
		color: #94a3b8;
		font-size: 0.875rem;
		line-height: 1.5;
	}

	.create-button,
	.primary-button,
	.secondary-button,
	.filter-button,
	.toggle-chip,
	.error-card button {
		border-radius: 8px;
		font: inherit;
		cursor: pointer;
		transition:
			background-color 0.2s ease,
			border-color 0.2s ease,
			color 0.2s ease,
			opacity 0.2s ease,
			box-shadow 0.2s ease;
	}

	.create-button {
		border: 0;
		background: linear-gradient(90deg, #9333ea, #2563eb);
		box-shadow: 0 12px 24px rgba(37, 99, 235, 0.22);
		color: #ffffff;
		padding: 0.5rem 1rem;
		font-size: 0.875rem;
		font-weight: 600;
	}

	.create-button:hover {
		background: linear-gradient(90deg, #a855f7, #3b82f6);
	}

	.loading-state {
		display: grid;
		padding-block: 5rem;
		place-items: center;
	}

	.spinner-shell {
		position: relative;
		width: 3rem;
		height: 3rem;
	}

	.spinner-track,
	.spinner,
	.loader {
		border-radius: 999px;
	}

	.spinner-track,
	.spinner {
		position: absolute;
		inset: 0;
		border: 4px solid rgba(168, 85, 247, 0.2);
	}

	.spinner {
		border-color: #a855f7;
		border-top-color: transparent;
		animation: spin 0.8s linear infinite;
	}

	.loader {
		width: 2rem;
		height: 2rem;
		border: 4px solid #2563eb;
		border-top-color: transparent;
		animation: spin 0.8s linear infinite;
	}

	.filter-row {
		flex-wrap: wrap;
		gap: 0.5rem;
		margin-bottom: 1.5rem;
	}

	.filter-button {
		border: 1px solid rgba(255, 255, 255, 0.1);
		background: rgba(255, 255, 255, 0.05);
		color: #94a3b8;
		padding: 0.5rem 1rem;
		font-size: 0.875rem;
		font-weight: 600;
	}

	.filter-button:hover,
	.filter-button--active {
		background: #ffffff;
		color: #0f172a;
	}

	.error-card,
	.empty-card,
	.report-card {
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		background: #ffffff;
		color: #111827;
	}

	.error-card {
		padding: 1.5rem;
		text-align: center;
	}

	.error-card p {
		margin: 0;
		color: #dc2626;
	}

	.error-card button {
		margin-top: 1rem;
		border: 0;
		background: #dc2626;
		color: #ffffff;
		padding: 0.5rem 1rem;
	}

	.error-card button:hover {
		background: #b91c1c;
	}

	.empty-card {
		padding: 3rem;
		text-align: center;
	}

	.empty-icon {
		margin-bottom: 1rem;
		font-size: 2.5rem;
		line-height: 1;
	}

	.empty-card h3 {
		margin: 0 0 0.5rem;
		font-size: 1.125rem;
		font-weight: 600;
		line-height: 1.4;
	}

	.empty-card p {
		margin: 0 0 1.5rem;
		color: #6b7280;
	}

	.reports-grid {
		display: grid;
		grid-template-columns: minmax(0, 1fr);
		gap: 1.5rem;
	}

	.report-card {
		padding: 1.5rem;
		transition: box-shadow 0.2s ease;
	}

	.report-card:hover {
		box-shadow: 0 14px 28px rgba(15, 23, 42, 0.14);
	}

	.report-header {
		align-items: flex-start;
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 1rem;
	}

	.report-title-row {
		gap: 0.75rem;
	}

	.report-icon {
		font-size: 1.5rem;
		line-height: 1;
	}

	.report-card h3 {
		margin: 0 0 0.25rem;
		font-weight: 700;
		line-height: 1.4;
	}

	.status-pill {
		display: inline-flex;
		border-radius: 4px;
		padding: 0.125rem 0.5rem;
		font-size: 0.75rem;
		line-height: 1.4;
		text-transform: capitalize;
	}

	.status-pill--active {
		background: #dcfce7;
		color: #15803d;
	}

	.status-pill--draft {
		background: #f3f4f6;
		color: #374151;
	}

	.status-pill--paused {
		background: #fef3c7;
		color: #a16207;
	}

	.menu-button,
	.close-button {
		border: 0;
		background: transparent;
		color: #9ca3af;
		cursor: pointer;
	}

	.menu-button {
		font-size: 1.25rem;
		line-height: 1;
	}

	.menu-button:hover,
	.close-button:hover {
		color: #4b5563;
	}

	.report-description {
		display: -webkit-box;
		line-clamp: 2;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
		margin: 0 0 1rem;
		color: #6b7280;
		font-size: 0.875rem;
		line-height: 1.5;
	}

	.report-meta {
		display: grid;
		gap: 0.5rem;
		color: #6b7280;
		font-size: 0.875rem;
	}

	.report-meta div {
		justify-content: space-between;
		gap: 1rem;
	}

	.capitalize-text {
		text-transform: capitalize;
	}

	.report-actions {
		gap: 0.5rem;
		margin-top: 1rem;
		border-top: 1px solid #f3f4f6;
		padding-top: 1rem;
	}

	.primary-button {
		border: 0;
		background: #2563eb;
		color: #ffffff;
		padding: 0.5rem 1rem;
		text-decoration: none;
	}

	.primary-button:hover {
		background: #1d4ed8;
	}

	.primary-button:disabled {
		cursor: not-allowed;
		opacity: 0.5;
	}

	.primary-button--large {
		display: inline-block;
		padding: 0.75rem 1.5rem;
	}

	.report-link {
		flex: 1 1 auto;
		text-align: center;
		font-size: 0.875rem;
	}

	.secondary-button {
		border: 1px solid #d1d5db;
		background: #ffffff;
		color: #374151;
		padding: 0.5rem 1rem;
	}

	.secondary-button:hover {
		background: #f9fafb;
	}

	.modal-overlay {
		position: fixed;
		z-index: 50;
		inset: 0;
		display: grid;
		background: rgba(0, 0, 0, 0.5);
		padding: 1rem;
		place-items: center;
	}

	.modal-panel {
		width: min(100%, 42rem);
		max-height: 90vh;
		overflow-y: auto;
		border-radius: 16px;
		background: #ffffff;
		color: #111827;
	}

	.modal-header,
	.modal-body,
	.modal-footer {
		padding: 1.5rem;
	}

	.modal-header {
		border-bottom: 1px solid #f3f4f6;
	}

	.modal-header-row,
	.modal-footer {
		justify-content: space-between;
		gap: 1rem;
	}

	.modal-header h2 {
		margin: 0;
		font-size: 1.25rem;
		font-weight: 700;
		line-height: 1.3;
	}

	.close-button {
		font-size: 1.5rem;
		line-height: 1;
	}

	.modal-body {
		display: grid;
		gap: 1.5rem;
	}

	.field-stack {
		display: grid;
		gap: 1rem;
	}

	.field-label {
		display: block;
		margin-bottom: 0.25rem;
		color: #374151;
		font-size: 0.875rem;
		font-weight: 600;
		line-height: 1.4;
	}

	.field-label--static {
		margin-bottom: 0.5rem;
	}

	.field-control {
		width: 100%;
		border: 1px solid #d1d5db;
		border-radius: 8px;
		background: #ffffff;
		color: #111827;
		padding: 0.5rem 1rem;
		font: inherit;
		line-height: 1.5;
	}

	.field-control:focus {
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.18);
		outline: none;
	}

	.chip-row {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.toggle-chip {
		border: 0;
		background: #f3f4f6;
		color: #374151;
		padding: 0.375rem 0.75rem;
		font-size: 0.875rem;
	}

	.toggle-chip:hover {
		background: #e5e7eb;
	}

	.toggle-chip--blue,
	.toggle-chip--blue:hover {
		background: #2563eb;
		color: #ffffff;
	}

	.toggle-chip--purple,
	.toggle-chip--purple:hover {
		background: #9333ea;
		color: #ffffff;
	}

	.schedule-section,
	.modal-footer {
		border-top: 1px solid #f3f4f6;
	}

	.schedule-section {
		padding-top: 1.5rem;
	}

	.checkbox-row {
		gap: 0.75rem;
		color: #111827;
		font-weight: 600;
		cursor: pointer;
	}

	.checkbox-control {
		width: 1rem;
		height: 1rem;
		accent-color: #2563eb;
	}

	.schedule-fields {
		display: grid;
		gap: 1rem;
		margin-top: 1rem;
		padding-left: 1.75rem;
	}

	.modal-footer {
		justify-content: flex-end;
	}

	@media (min-width: 640px) {
		.reports-container {
			padding-inline: 1.5rem;
		}
	}

	@media (min-width: 768px) {
		.reports-grid {
			grid-template-columns: repeat(2, minmax(0, 1fr));
		}
	}

	@media (min-width: 1024px) {
		.reports-container {
			padding-inline: 2rem;
		}

		.reports-grid {
			grid-template-columns: repeat(3, minmax(0, 1fr));
		}
	}

	@media (max-width: 767.98px) {
		.page-header {
			align-items: flex-start;
			flex-direction: column;
		}

		.create-button {
			width: 100%;
		}

		.modal-footer {
			flex-direction: column-reverse;
		}

		.modal-footer button {
			width: 100%;
		}
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>

<script lang="ts">
	/**
	 * FormReport Component (FluentForms 6.1.0 Pro - August 2025)
	 *
	 * Report Module for generating and viewing form submission reports,
	 * with the ability to download reports in PDF format.
	 */

	interface ReportData {
		form_id: number;
		form_title: string;
		total_submissions: number;
		date_range: {
			start: string;
			end: string;
		};
		field_reports: FieldReport[];
		conversion_rate?: number;
		avg_completion_time?: string;
		top_sources?: { source: string; count: number }[];
	}

	interface FieldReport {
		field_name: string;
		field_label: string;
		field_type: string;
		responses: number;
		data: FieldData[];
	}

	interface FieldData {
		value: string;
		count: number;
		percentage: number;
	}

	interface Props {
		formId: number;
		dateRange?: { start: string; end: string };
		showCharts?: boolean;
		allowExport?: boolean;
		variant?: 'full' | 'summary' | 'compact';
	}

	let {
		formId,
		dateRange,
		showCharts = true,
		allowExport = true,
		variant = 'full'
	}: Props = $props();

	let reportData = $state<ReportData | null>(null);
	let isLoading = $state(true);
	let error = $state<string | null>(null);
	let isExporting = $state(false);
	let activeFieldIndex = $state(0);

	async function loadReport() {
		isLoading = true;
		error = null;

		try {
			const params = new URLSearchParams();
			if (dateRange?.start) params.append('start_date', dateRange.start);
			if (dateRange?.end) params.append('end_date', dateRange.end);

			const response = await fetch(`/api/forms/${formId}/report?${params.toString()}`);

			if (!response.ok) {
				throw new Error('Failed to load report');
			}

			const result = await response.json();
			reportData = result.data;
		} catch {
			error = 'Failed to load report. Please try again.';
		} finally {
			isLoading = false;
		}
	}

	async function exportPdf() {
		if (isExporting) return;

		isExporting = true;

		try {
			const params = new URLSearchParams();
			if (dateRange?.start) params.append('start_date', dateRange.start);
			if (dateRange?.end) params.append('end_date', dateRange.end);

			const response = await fetch(`/api/forms/${formId}/report/pdf?${params.toString()}`);

			if (!response.ok) {
				throw new Error('Failed to export report');
			}

			const blob = await response.blob();
			const url = URL.createObjectURL(blob);
			const a = document.createElement('a');
			a.href = url;
			a.download = `form-report-${formId}-${new Date().toISOString().split('T')[0]}.pdf`;
			document.body.appendChild(a);
			a.click();
			document.body.removeChild(a);
			URL.revokeObjectURL(url);
		} catch {
			error = 'Failed to export report. Please try again.';
		} finally {
			isExporting = false;
		}
	}

	function getBarWidth(percentage: number): string {
		return `${Math.max(percentage, 2)}%`;
	}

	function getBarColor(index: number): string {
		const colors = ['#E6B800', '#10b981', '#f59e0b', '#ef4444', '#B38F00', '#ec4899', '#06b6d4'];
		return colors[index % colors.length];
	}

	$effect(() => {
		loadReport();
	});
</script>

<div class="form-report" class:variant-summary={variant === 'summary'} class:variant-compact={variant === 'compact'}>
	{#if isLoading}
		<div class="loading-state">
			<span class="spinner"></span>
			<span>Loading report...</span>
		</div>
	{:else if error}
		<div class="error-state">
			<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<circle cx="12" cy="12" r="10"></circle>
				<line x1="12" y1="8" x2="12" y2="12"></line>
				<line x1="12" y1="16" x2="12.01" y2="16"></line>
			</svg>
			<span>{error}</span>
			<button type="button" class="retry-btn" onclick={loadReport}>Try Again</button>
		</div>
	{:else if reportData}
		<div class="report-header">
			<div class="report-title">
				<h2>{reportData.form_title}</h2>
				<p class="date-range">
					{reportData.date_range.start} - {reportData.date_range.end}
				</p>
			</div>
			{#if allowExport}
				<button type="button" class="export-btn" onclick={exportPdf} disabled={isExporting}>
					{#if isExporting}
						<span class="spinner small"></span>
						Exporting...
					{:else}
						<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
							<polyline points="7 10 12 15 17 10"></polyline>
							<line x1="12" y1="15" x2="12" y2="3"></line>
						</svg>
						Export PDF
					{/if}
				</button>
			{/if}
		</div>

		<!-- Summary Stats -->
		<div class="stats-grid">
			<div class="stat-card">
				<div class="stat-icon submissions">
					<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
						<polyline points="14 2 14 8 20 8"></polyline>
						<line x1="16" y1="13" x2="8" y2="13"></line>
						<line x1="16" y1="17" x2="8" y2="17"></line>
					</svg>
				</div>
				<div class="stat-content">
					<div class="stat-value">{reportData.total_submissions.toLocaleString()}</div>
					<div class="stat-label">Total Submissions</div>
				</div>
			</div>

			{#if reportData.conversion_rate !== undefined}
				<div class="stat-card">
					<div class="stat-icon conversion">
						<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<polyline points="23 6 13.5 15.5 8.5 10.5 1 18"></polyline>
							<polyline points="17 6 23 6 23 12"></polyline>
						</svg>
					</div>
					<div class="stat-content">
						<div class="stat-value">{reportData.conversion_rate.toFixed(1)}%</div>
						<div class="stat-label">Conversion Rate</div>
					</div>
				</div>
			{/if}

			{#if reportData.avg_completion_time}
				<div class="stat-card">
					<div class="stat-icon time">
						<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<circle cx="12" cy="12" r="10"></circle>
							<polyline points="12 6 12 12 16 14"></polyline>
						</svg>
					</div>
					<div class="stat-content">
						<div class="stat-value">{reportData.avg_completion_time}</div>
						<div class="stat-label">Avg. Completion Time</div>
					</div>
				</div>
			{/if}

			<div class="stat-card">
				<div class="stat-icon fields">
					<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<rect x="3" y="3" width="7" height="7"></rect>
						<rect x="14" y="3" width="7" height="7"></rect>
						<rect x="14" y="14" width="7" height="7"></rect>
						<rect x="3" y="14" width="7" height="7"></rect>
					</svg>
				</div>
				<div class="stat-content">
					<div class="stat-value">{reportData.field_reports.length}</div>
					<div class="stat-label">Fields Analyzed</div>
				</div>
			</div>
		</div>

		{#if variant !== 'compact'}
			<!-- Field Reports -->
			{#if showCharts && reportData.field_reports.length > 0}
				<div class="field-reports">
					<h3>Field Analysis</h3>

					<div class="field-tabs">
						{#each reportData.field_reports as field, index}
							<button
								type="button"
								class="field-tab"
								class:active={activeFieldIndex === index}
								onclick={() => (activeFieldIndex = index)}
							>
								{field.field_label}
							</button>
						{/each}
					</div>

					{#if reportData.field_reports[activeFieldIndex]}
						{@const field = reportData.field_reports[activeFieldIndex]}
						<div class="field-report-card">
							<div class="field-header">
								<h4>{field.field_label}</h4>
								<span class="response-count">{field.responses} responses</span>
							</div>

							<div class="field-chart">
								{#each field.data.slice(0, 10) as item, itemIndex}
									<div class="chart-row">
										<div class="chart-label" title={item.value}>
											{item.value || '(Empty)'}
										</div>
										<div class="chart-bar-container">
											<div
												class="chart-bar"
												style="width: {getBarWidth(item.percentage)}; background-color: {getBarColor(itemIndex)}"
											></div>
											<span class="chart-value">{item.count} ({item.percentage.toFixed(1)}%)</span>
										</div>
									</div>
								{/each}
							</div>
						</div>
					{/if}
				</div>
			{/if}

			<!-- Top Sources -->
			{#if reportData.top_sources && reportData.top_sources.length > 0}
				<div class="top-sources">
					<h3>Top Traffic Sources</h3>
					<div class="sources-list">
						{#each reportData.top_sources as source, index}
							<div class="source-item">
								<span class="source-rank">#{index + 1}</span>
								<span class="source-name">{source.source}</span>
								<span class="source-count">{source.count}</span>
							</div>
						{/each}
					</div>
				</div>
			{/if}
		{/if}
	{:else}
		<div class="empty-state">
			<svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1">
				<path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
				<polyline points="14 2 14 8 20 8"></polyline>
			</svg>
			<p>No report data available</p>
		</div>
	{/if}
</div>

<style>
	.form-report {
		background-color: white;
		border: 1px solid #e5e7eb;
		border-radius: 0.75rem;
		overflow: hidden;
	}

	.report-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		padding: 1.5rem;
		border-bottom: 1px solid #e5e7eb;
	}

	.report-title h2 {
		margin: 0;
		font-size: 1.25rem;
		font-weight: 600;
		color: #111827;
	}

	.date-range {
		margin: 0.25rem 0 0;
		font-size: 0.875rem;
		color: #6b7280;
	}

	.export-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background-color: #dc2626;
		color: white;
		border: none;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.export-btn:hover:not(:disabled) {
		background-color: #b91c1c;
	}

	.export-btn:disabled {
		opacity: 0.7;
		cursor: not-allowed;
	}

	.stats-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 1rem;
		padding: 1.5rem;
	}

	.stat-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem;
		background-color: #f9fafb;
		border-radius: 0.5rem;
	}

	.stat-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 48px;
		height: 48px;
		border-radius: 0.5rem;
		color: white;
	}

	.stat-icon.submissions { background-color: #E6B800; }
	.stat-icon.conversion { background-color: #10b981; }
	.stat-icon.time { background-color: #f59e0b; }
	.stat-icon.fields { background-color: #B38F00; }

	.stat-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: #111827;
	}

	.stat-label {
		font-size: 0.8125rem;
		color: #6b7280;
	}

	.field-reports {
		padding: 1.5rem;
		border-top: 1px solid #e5e7eb;
	}

	.field-reports h3 {
		margin: 0 0 1rem;
		font-size: 1rem;
		font-weight: 600;
		color: #374151;
	}

	.field-tabs {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		margin-bottom: 1rem;
	}

	.field-tab {
		padding: 0.5rem 0.75rem;
		background-color: #f3f4f6;
		border: 1px solid #e5e7eb;
		border-radius: 0.375rem;
		font-size: 0.8125rem;
		color: #6b7280;
		cursor: pointer;
		transition: all 0.15s;
	}

	.field-tab:hover {
		background-color: #e5e7eb;
	}

	.field-tab.active {
		background-color: #E6B800;
		border-color: #E6B800;
		color: white;
	}

	.field-report-card {
		background-color: #f9fafb;
		border-radius: 0.5rem;
		padding: 1rem;
	}

	.field-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.field-header h4 {
		margin: 0;
		font-size: 0.9375rem;
		font-weight: 600;
		color: #374151;
	}

	.response-count {
		font-size: 0.8125rem;
		color: #6b7280;
	}

	.chart-row {
		display: flex;
		align-items: center;
		gap: 1rem;
		margin-bottom: 0.75rem;
	}

	.chart-label {
		width: 120px;
		font-size: 0.8125rem;
		color: #374151;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.chart-bar-container {
		flex: 1;
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.chart-bar {
		height: 24px;
		border-radius: 0.25rem;
		transition: width 0.3s ease-out;
	}

	.chart-value {
		font-size: 0.75rem;
		color: #6b7280;
		white-space: nowrap;
	}

	.top-sources {
		padding: 1.5rem;
		border-top: 1px solid #e5e7eb;
	}

	.top-sources h3 {
		margin: 0 0 1rem;
		font-size: 1rem;
		font-weight: 600;
		color: #374151;
	}

	.sources-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.source-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem;
		background-color: #f9fafb;
		border-radius: 0.375rem;
	}

	.source-rank {
		font-size: 0.75rem;
		font-weight: 600;
		color: #9ca3af;
	}

	.source-name {
		flex: 1;
		font-size: 0.875rem;
		color: #374151;
	}

	.source-count {
		font-size: 0.875rem;
		font-weight: 600;
		color: #E6B800;
	}

	/* States */
	.loading-state,
	.error-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 1rem;
		padding: 3rem;
		color: #6b7280;
		text-align: center;
	}

	.retry-btn {
		padding: 0.5rem 1rem;
		background-color: #E6B800;
		color: white;
		border: none;
		border-radius: 0.375rem;
		cursor: pointer;
	}

	.spinner {
		width: 24px;
		height: 24px;
		border: 3px solid rgba(230, 184, 0, 0.2);
		border-top-color: #E6B800;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	.spinner.small {
		width: 16px;
		height: 16px;
		border-width: 2px;
		border-color: rgba(255, 255, 255, 0.3);
		border-top-color: white;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Variants */
	.variant-summary .field-reports,
	.variant-summary .top-sources {
		display: none;
	}

	.variant-compact {
		border: none;
		background: none;
	}

	.variant-compact .report-header {
		padding: 0 0 1rem;
		border: none;
	}

	.variant-compact .stats-grid {
		padding: 0;
	}
</style>

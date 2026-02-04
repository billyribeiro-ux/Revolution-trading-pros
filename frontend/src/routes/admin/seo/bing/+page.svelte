<script lang="ts">
	import { browser } from '$app/environment';
	import { toastStore } from '$lib/stores/toast.svelte';
	import {
		IconWorld as IconBrandBing,
		IconRocket,
		IconChartBar,
		IconCheck,
		IconX,
		IconRefresh,
		IconSend,
		IconClock,
		IconTrendingUp,
		IconExternalLink,
		IconBolt,
		IconWorld,
		IconSearch,
		IconFileText
	} from '$lib/icons';
	import {
		bingSeoApi,
		type BingSeoStats,
		type BingSubmission,
		type BingSearchPerformance
	} from '$lib/api/bing-seo';

	// State
	let loading = $state(true);
	let stats = $state<BingSeoStats | null>(null);
	let recentSubmissions = $state<BingSubmission[]>([]);
	let topQueries = $state<BingSearchPerformance[]>([]);

	// URL submission
	let urlToSubmit = $state('');
	let submitting = $state(false);
	let batchUrls = $state('');
	let showBatchModal = $state(false);

	$effect(() => {
		if (browser) {
			loadData();
		}
	});

	async function loadData() {
		loading = true;
		try {
			const [statsData, submissionsData, queriesData] = await Promise.all([
				bingSeoApi.getStats().catch(() => null),
				bingSeoApi.getSubmissions({ per_page: 10 }).catch(() => ({ submissions: [], total: 0 })),
				bingSeoApi.getTopQueries(10).catch(() => [])
			]);

			stats = statsData;
			recentSubmissions = submissionsData.submissions;
			topQueries = queriesData;
		} catch (err) {
			console.error('Failed to load Bing SEO data:', err);
		} finally {
			loading = false;
		}
	}

	async function handleSubmitUrl() {
		if (!urlToSubmit) {
			toastStore.error('Please enter a URL to submit');
			return;
		}

		// Validate URL
		try {
			new URL(urlToSubmit);
		} catch {
			toastStore.error('Please enter a valid URL');
			return;
		}

		submitting = true;
		try {
			const result = await bingSeoApi.submitUrl(urlToSubmit);
			if (result.success) {
				toastStore.success('URL submitted to Bing IndexNow! It will be indexed within minutes.');
				urlToSubmit = '';
				await loadData();
			} else {
				toastStore.error(result.message || 'Failed to submit URL');
			}
		} catch (err: any) {
			toastStore.error(err.message || 'Failed to submit URL');
		} finally {
			submitting = false;
		}
	}

	async function handleBatchSubmit() {
		const urls = batchUrls
			.split('\n')
			.map((u) => u.trim())
			.filter(Boolean);

		if (urls.length === 0) {
			toastStore.error('Please enter at least one URL');
			return;
		}

		// Validate URLs
		const invalidUrls = urls.filter((url) => {
			try {
				new URL(url);
				return false;
			} catch {
				return true;
			}
		});

		if (invalidUrls.length > 0) {
			toastStore.error(
				`Invalid URLs: ${invalidUrls.slice(0, 3).join(', ')}${invalidUrls.length > 3 ? '...' : ''}`
			);
			return;
		}

		submitting = true;
		try {
			const result = await bingSeoApi.submitBatch(urls);
			toastStore.success(`Submitted ${result.submitted} URLs to Bing IndexNow!`);
			batchUrls = '';
			showBatchModal = false;
			await loadData();
		} catch (err: any) {
			toastStore.error(err.message || 'Failed to submit URLs');
		} finally {
			submitting = false;
		}
	}

	async function handleSubmitSitemap() {
		try {
			const result = await bingSeoApi.submitSitemap();
			if (result.success) {
				toastStore.success('Sitemap submitted to Bing successfully!');
			} else {
				toastStore.error(result.message || 'Failed to submit sitemap');
			}
		} catch (err: any) {
			toastStore.error(err.message || 'Failed to submit sitemap');
		}
	}

	function formatDate(dateStr: string | null): string {
		if (!dateStr) return 'Never';
		return new Date(dateStr).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}
</script>

<svelte:head>
	<title>Bing SEO & IndexNow | Admin</title>
</svelte:head>

<div class="bing-seo-page">
	<!-- Header -->
	<div class="page-header">
		<div class="header-content">
			<div class="header-title">
				<div class="title-icon bing">
					<IconBrandBing size={28} />
				</div>
				<div>
					<h1>Bing SEO & IndexNow</h1>
					<p class="subtitle">
						Instant URL indexing - get your content in Bing search within minutes
					</p>
				</div>
			</div>

			<div class="header-actions">
				<button class="btn-secondary" onclick={loadData}>
					<IconRefresh size={18} />
					Refresh
				</button>
				<button class="btn-secondary" onclick={() => (showBatchModal = true)}>
					<IconFileText size={18} />
					Batch Submit
				</button>
				<button class="btn-primary bing" onclick={handleSubmitSitemap}>
					<IconWorld size={18} />
					Submit Sitemap
				</button>
			</div>
		</div>
	</div>

	<!-- Competitive Advantage Banner -->
	<div class="advantage-banner">
		<div class="advantage-icon">
			<IconBolt size={32} />
		</div>
		<div class="advantage-content">
			<h3>Massive Competitive Advantage</h3>
			<p>
				While your competitors wait <strong>days or weeks</strong> for Bing to crawl and index their
				content, IndexNow gets your pages indexed in <strong>minutes</strong>. Most websites
				completely ignore Bing optimization - this gives you a significant edge.
			</p>
		</div>
	</div>

	{#if loading}
		<div class="loading-grid">
			{#each [1, 2, 3, 4] as _}
				<div class="skeleton skeleton-metric"></div>
			{/each}
		</div>
	{:else}
		<!-- Stats -->
		<div class="stats-grid">
			<div class="stat-card">
				<div class="stat-icon blue">
					<IconSend size={24} />
				</div>
				<div class="stat-content">
					<div class="stat-value">{stats?.total_submissions?.toLocaleString() || 0}</div>
					<div class="stat-label">Total Submissions</div>
				</div>
			</div>
			<div class="stat-card">
				<div class="stat-icon emerald">
					<IconCheck size={24} />
				</div>
				<div class="stat-content">
					<div class="stat-value">{stats?.successful_submissions?.toLocaleString() || 0}</div>
					<div class="stat-label">Successful</div>
				</div>
			</div>
			<div class="stat-card">
				<div class="stat-icon purple">
					<IconTrendingUp size={24} />
				</div>
				<div class="stat-content">
					<div class="stat-value">{stats?.success_rate?.toFixed(1) || 0}%</div>
					<div class="stat-label">Success Rate</div>
				</div>
			</div>
			<div class="stat-card">
				<div class="stat-icon yellow">
					<IconClock size={24} />
				</div>
				<div class="stat-content">
					<div class="stat-value">{stats?.submissions_today || 0}</div>
					<div class="stat-label">Today</div>
				</div>
			</div>
		</div>

		<!-- URL Submission -->
		<div class="submit-section">
			<div class="submit-card">
				<div class="submit-header">
					<IconRocket size={24} />
					<h2>Submit URL to IndexNow</h2>
				</div>
				<p class="submit-description">
					Enter a URL to instantly notify Bing about new or updated content. Your page will be
					indexed within minutes instead of waiting for the next crawl.
				</p>
				<div class="submit-form">
					<label for="url-to-submit" class="sr-only">URL to submit</label>
					<input
						type="url"
						id="url-to-submit" name="url-to-submit"
						bind:value={urlToSubmit}
						placeholder="https://yourdomain.com/new-page"
						onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && handleSubmitUrl()}
					/>
					<button class="btn-primary bing" onclick={handleSubmitUrl} disabled={submitting}>
						{#if submitting}
							Submitting...
						{:else}
							<IconSend size={18} />
							Submit to Bing
						{/if}
					</button>
				</div>
			</div>
		</div>

		<!-- Two Column Layout -->
		<div class="two-columns">
			<!-- Recent Submissions -->
			<div class="column-card">
				<div class="card-header">
					<h3>Recent Submissions</h3>
					<a href="/admin/seo/bing/history" class="view-all">View All</a>
				</div>

				{#if recentSubmissions.length === 0}
					<div class="empty-state small">
						<IconSend size={32} stroke={1} />
						<p>No submissions yet</p>
					</div>
				{:else}
					<div class="submissions-list">
						{#each recentSubmissions as submission}
							<div class="submission-item">
								<div class="submission-status" class:success={submission.success}>
									{#if submission.success}
										<IconCheck size={16} />
									{:else}
										<IconX size={16} />
									{/if}
								</div>
								<div class="submission-info">
									<div class="submission-url" title={submission.url}>
										{submission.url}
									</div>
									<div class="submission-meta">
										<span class="submission-type">{submission.submission_type}</span>
										<span class="submission-time">{formatDate(submission.created_at)}</span>
									</div>
								</div>
							</div>
						{/each}
					</div>
				{/if}
			</div>

			<!-- Top Queries -->
			<div class="column-card">
				<div class="card-header">
					<h3>Top Bing Queries</h3>
					<span class="header-note">Last 30 days</span>
				</div>

				{#if topQueries.length === 0}
					<div class="empty-state small">
						<IconSearch size={32} stroke={1} />
						<p>No search data yet</p>
						<small>Connect Bing Webmaster Tools to see search performance</small>
					</div>
				{:else}
					<div class="queries-list">
						{#each topQueries as query}
							<div class="query-item">
								<div class="query-info">
									<div class="query-text">{query.query}</div>
									<div class="query-url">{query.page_url}</div>
								</div>
								<div class="query-stats">
									<div class="query-stat">
										<span class="stat-value">{query.impressions.toLocaleString()}</span>
										<span class="stat-label">Impressions</span>
									</div>
									<div class="query-stat">
										<span class="stat-value">{query.clicks.toLocaleString()}</span>
										<span class="stat-label">Clicks</span>
									</div>
									<div class="query-stat">
										<span class="stat-value">{query.position.toFixed(1)}</span>
										<span class="stat-label">Position</span>
									</div>
								</div>
							</div>
						{/each}
					</div>
				{/if}
			</div>
		</div>

		<!-- Setup Guide -->
		<div class="setup-guide">
			<h3>Quick Setup Guide</h3>
			<div class="setup-steps">
				<div class="setup-step completed">
					<div class="step-number">1</div>
					<div class="step-content">
						<h4>IndexNow Key Generated</h4>
						<p>Your IndexNow API key is automatically generated and configured.</p>
					</div>
				</div>
				<div class="setup-step">
					<div class="step-number">2</div>
					<div class="step-content">
						<h4>Verify Key File</h4>
						<p>
							Ensure <code>{'{your-key}'}.txt</code> is accessible at your domain root.
							<a href="https://www.bing.com/indexnow" target="_blank" rel="noopener">
								Learn more <IconExternalLink size={14} />
							</a>
						</p>
					</div>
				</div>
				<div class="setup-step">
					<div class="step-number">3</div>
					<div class="step-content">
						<h4>Connect Bing Webmaster Tools (Optional)</h4>
						<p>
							Add your Bing Webmaster API key in settings for search performance data.
							<a href="https://www.bing.com/webmasters/" target="_blank" rel="noopener">
								Get API Key <IconExternalLink size={14} />
							</a>
						</p>
					</div>
				</div>
			</div>
		</div>
	{/if}
</div>

<!-- Batch Submit Modal -->
{#if showBatchModal}
	<div
		class="modal-overlay"
		role="button"
		tabindex="0"
		aria-label="Close modal"
		onclick={() => (showBatchModal = false)}
		onkeydown={(e: KeyboardEvent) => {
			if (e.key === 'Escape' || e.key === 'Enter' || e.key === ' ') showBatchModal = false;
		}}
	>
		<div
			class="modal-content"
			role="dialog"
			aria-modal="true"
			tabindex="-1"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
		>
			<div class="modal-header">
				<h2>Batch URL Submission</h2>
				<button class="close-btn" onclick={() => (showBatchModal = false)}>
					<IconX size={20} />
				</button>
			</div>

			<div class="modal-body">
				<p class="batch-description">
					Enter one URL per line. You can submit up to 10,000 URLs at once.
				</p>
				<label for="batch-urls" class="sr-only">Batch URLs</label>
				<textarea
					id="batch-urls"
					bind:value={batchUrls}
					placeholder="https://yourdomain.com/page-1
https://yourdomain.com/page-2
https://yourdomain.com/page-3"
					rows="10"
				></textarea>
				<div class="batch-count">
					{batchUrls.split('\n').filter((u) => u.trim()).length} URLs
				</div>
			</div>

			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showBatchModal = false)}>Cancel</button>
				<button class="btn-primary bing" onclick={handleBatchSubmit} disabled={submitting}>
					{#if submitting}
						Submitting...
					{:else}
						<IconSend size={18} />
						Submit All
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.bing-seo-page {
		padding: 2rem;
		max-width: 1400px;
		margin: 0 auto;
	}

	.page-header {
		margin-bottom: 1.5rem;
	}

	.header-content {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.header-title {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.title-icon {
		width: 56px;
		height: 56px;
		border-radius: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
	}

	.title-icon.bing {
		background: linear-gradient(135deg, #008373, #00a99d);
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
	}

	/* Advantage Banner */
	.advantage-banner {
		display: flex;
		align-items: center;
		gap: 1.5rem;
		padding: 1.5rem;
		background: linear-gradient(135deg, rgba(0, 131, 115, 0.15), rgba(0, 169, 157, 0.1));
		border: 1px solid rgba(0, 169, 157, 0.3);
		border-radius: 16px;
		margin-bottom: 2rem;
	}

	.advantage-icon {
		width: 64px;
		height: 64px;
		background: linear-gradient(135deg, #008373, #00a99d);
		border-radius: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
		flex-shrink: 0;
	}

	.advantage-content h3 {
		font-size: 1.125rem;
		font-weight: 600;
		color: #00d4aa;
		margin: 0 0 0.5rem;
	}

	.advantage-content p {
		color: #94a3b8;
		margin: 0;
		line-height: 1.6;
	}

	.advantage-content strong {
		color: #f1f5f9;
	}

	/* Stats */
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.stat-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
	}

	.stat-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.stat-icon.blue {
		background: rgba(59, 130, 246, 0.15);
		color: #60a5fa;
	}
	.stat-icon.emerald {
		background: rgba(16, 185, 129, 0.15);
		color: #34d399;
	}
	.stat-icon.purple {
		background: rgba(230, 184, 0, 0.15);
		color: var(--primary-500);
	}
	.stat-icon.yellow {
		background: rgba(251, 191, 36, 0.15);
		color: #fbbf24;
	}

	.stat-content .stat-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.stat-content .stat-label {
		font-size: 0.8125rem;
		color: #64748b;
	}

	/* Submit Section */
	.submit-section {
		margin-bottom: 2rem;
	}

	.submit-card {
		padding: 2rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
	}

	.submit-header {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		color: #f1f5f9;
		margin-bottom: 0.75rem;
	}

	.submit-header h2 {
		font-size: 1.25rem;
		font-weight: 600;
		margin: 0;
	}

	.submit-description {
		color: #94a3b8;
		margin: 0 0 1.5rem;
	}

	.submit-form {
		display: flex;
		gap: 0.75rem;
	}

	.submit-form input {
		flex: 1;
		padding: 0.875rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 10px;
		color: #f1f5f9;
		font-size: 0.9375rem;
	}

	.submit-form input:focus {
		outline: none;
		border-color: rgba(0, 169, 157, 0.5);
	}

	/* Two Columns */
	.two-columns {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1.5rem;
		margin-bottom: 2rem;
	}

	.column-card {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
		padding: 1.5rem;
	}

	.card-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
		padding-bottom: 0.75rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.card-header h3 {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.view-all,
	.header-note {
		font-size: 0.8125rem;
		color: #64748b;
	}

	.view-all:hover {
		color: #00d4aa;
	}

	/* Submissions List */
	.submissions-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.submission-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem;
		background: rgba(15, 23, 42, 0.4);
		border-radius: 8px;
	}

	.submission-status {
		width: 28px;
		height: 28px;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(239, 68, 68, 0.15);
		color: #f87171;
		flex-shrink: 0;
	}

	.submission-status.success {
		background: rgba(16, 185, 129, 0.15);
		color: #34d399;
	}

	.submission-info {
		flex: 1;
		min-width: 0;
	}

	.submission-url {
		font-size: 0.875rem;
		color: #f1f5f9;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.submission-meta {
		display: flex;
		gap: 0.75rem;
		font-size: 0.75rem;
		color: #64748b;
		margin-top: 0.25rem;
	}

	.submission-type {
		background: rgba(148, 163, 184, 0.1);
		padding: 0.125rem 0.375rem;
		border-radius: 4px;
	}

	/* Queries List */
	.queries-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.query-item {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.75rem;
		background: rgba(15, 23, 42, 0.4);
		border-radius: 8px;
	}

	.query-info {
		flex: 1;
		min-width: 0;
	}

	.query-text {
		font-size: 0.875rem;
		font-weight: 500;
		color: #f1f5f9;
	}

	.query-url {
		font-size: 0.75rem;
		color: #64748b;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.query-stats {
		display: flex;
		gap: 1rem;
	}

	.query-stat {
		text-align: center;
	}

	.query-stat .stat-value {
		font-size: 0.875rem;
		font-weight: 600;
		color: #f1f5f9;
	}

	.query-stat .stat-label {
		font-size: 0.625rem;
		color: #64748b;
		text-transform: uppercase;
	}

	/* Setup Guide */
	.setup-guide {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
		padding: 1.5rem;
	}

	.setup-guide h3 {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 1.5rem;
	}

	.setup-steps {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.setup-step {
		display: flex;
		gap: 1rem;
		padding: 1rem;
		background: rgba(15, 23, 42, 0.4);
		border-radius: 10px;
		border: 1px solid rgba(148, 163, 184, 0.1);
	}

	.setup-step.completed {
		border-color: rgba(16, 185, 129, 0.3);
		background: rgba(16, 185, 129, 0.05);
	}

	.step-number {
		width: 32px;
		height: 32px;
		border-radius: 50%;
		background: rgba(148, 163, 184, 0.2);
		color: #94a3b8;
		display: flex;
		align-items: center;
		justify-content: center;
		font-weight: 600;
		flex-shrink: 0;
	}

	.setup-step.completed .step-number {
		background: rgba(16, 185, 129, 0.2);
		color: #34d399;
	}

	.step-content h4 {
		font-size: 0.9375rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 0.25rem;
	}

	.step-content p {
		font-size: 0.8125rem;
		color: #94a3b8;
		margin: 0;
	}

	.step-content code {
		background: rgba(148, 163, 184, 0.15);
		padding: 0.125rem 0.375rem;
		border-radius: 4px;
		font-family: monospace;
		font-size: 0.8125rem;
	}

	.step-content a {
		color: #00d4aa;
		display: inline-flex;
		align-items: center;
		gap: 0.25rem;
	}

	/* Empty State */
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 2rem;
		color: #64748b;
		text-align: center;
	}

	.empty-state.small {
		padding: 1.5rem;
	}

	.empty-state p {
		margin: 0.5rem 0 0;
	}

	.empty-state small {
		font-size: 0.75rem;
		margin-top: 0.25rem;
	}

	/* Loading */
	.loading-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
		margin-bottom: 2rem;
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
		height: 100px;
	}

	@keyframes shimmer {
		0% {
			background-position: 200% 0;
		}
		100% {
			background-position: -200% 0;
		}
	}

	/* Buttons */
	.btn-primary,
	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
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

	.btn-primary.bing {
		background: linear-gradient(135deg, #008373, #00a99d);
		color: white;
	}

	.btn-secondary {
		background: rgba(148, 163, 184, 0.1);
		color: #94a3b8;
		border: 1px solid rgba(148, 163, 184, 0.2);
	}

	.btn-primary:disabled,
	.btn-secondary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	/* Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.8);
		backdrop-filter: blur(4px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 50;
		padding: 2rem;
	}

	.modal-content {
		background: #1e293b;
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 20px;
		width: 100%;
		max-width: 600px;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.modal-header h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.close-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(148, 163, 184, 0.1);
		border: none;
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
	}

	.modal-body {
		padding: 1.5rem;
	}

	.batch-description {
		color: #94a3b8;
		margin: 0 0 1rem;
	}

	.modal-body textarea {
		width: 100%;
		padding: 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 10px;
		color: #f1f5f9;
		font-size: 0.875rem;
		font-family: monospace;
		resize: vertical;
	}

	.modal-body textarea:focus {
		outline: none;
		border-color: rgba(0, 169, 157, 0.5);
	}

	.batch-count {
		text-align: right;
		font-size: 0.8125rem;
		color: #64748b;
		margin-top: 0.5rem;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1.5rem;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
	}

	@media (max-width: 768px) {
		.stats-grid,
		.loading-grid {
			grid-template-columns: repeat(2, 1fr);
		}

		.two-columns {
			grid-template-columns: 1fr;
		}

		.header-content {
			flex-direction: column;
			gap: 1rem;
			align-items: flex-start;
		}

		.submit-form {
			flex-direction: column;
		}
	}

	.sr-only {
		position: absolute;
		width: 1px;
		height: 1px;
		padding: 0;
		margin: -1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		white-space: nowrap;
		border-width: 0;
	}
</style>

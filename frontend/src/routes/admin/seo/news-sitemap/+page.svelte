<script lang="ts">
	import { browser } from '$app/environment';
	import { IconNews, IconRefresh, IconExternalLink, IconCheck, IconX, IconClock } from '$lib/icons';

	// State using Svelte 5 runes
	let articles = $state<any[]>([]);
	let settings = $state({
		enabled: true,
		publicationName: 'Revolution Trading Pros',
		language: 'en',
		postTypes: ['blog'] as string[],
		excludeCategories: [] as string[],
		maxAge: 48 // hours
	});
	let loading = $state(false);
	let lastGenerated = $state<string | null>(null);

	// Computed
	let sitemapUrl = $derived(
		`${typeof window !== 'undefined' ? window.location.origin : ''}/news-sitemap.xml`
	);

	$effect(() => {
		if (browser) {
			loadArticles();
		}
	});

	async function loadArticles() {
		loading = true;
		try {
			// In production, fetch from API
			articles = [
				{
					id: 1,
					title: 'S&P 500 Technical Analysis: Key Levels to Watch This Week',
					url: '/blog/market-analysis-november-2025',
					publishedAt: new Date(Date.now() - 2 * 60 * 60 * 1000).toISOString(),
					inSitemap: true
				},
				{
					id: 2,
					title: 'Best Options Trading Strategies for Volatile Markets',
					url: '/blog/options-trading-strategies-2025',
					publishedAt: new Date(Date.now() - 8 * 60 * 60 * 1000).toISOString(),
					inSitemap: true
				},
				{
					id: 3,
					title: 'Essential Day Trading Tips for Beginners',
					url: '/blog/day-trading-tips-beginners',
					publishedAt: new Date(Date.now() - 24 * 60 * 60 * 1000).toISOString(),
					inSitemap: true
				},
				{
					id: 4,
					title: 'How to Identify Momentum Stocks for Swing Trading',
					url: '/blog/swing-trading-momentum-stocks',
					publishedAt: new Date(Date.now() - 36 * 60 * 60 * 1000).toISOString(),
					inSitemap: true
				}
			];
			lastGenerated = new Date().toISOString();
		} finally {
			loading = false;
		}
	}

	async function regenerateSitemap() {
		loading = true;
		try {
			// In production, call API to regenerate
			await new Promise((resolve) => setTimeout(resolve, 1000));
			lastGenerated = new Date().toISOString();
			alert('News sitemap regenerated successfully!');
		} finally {
			loading = false;
		}
	}

	function getTimeAgo(dateStr: string): string {
		const date = new Date(dateStr);
		const now = new Date();
		const diff = now.getTime() - date.getTime();
		const hours = Math.floor(diff / (1000 * 60 * 60));

		if (hours < 1) return 'Just now';
		if (hours === 1) return '1 hour ago';
		if (hours < 24) return `${hours} hours ago`;
		if (hours < 48) return 'Yesterday';
		return `${Math.floor(hours / 24)} days ago`;
	}
</script>

<svelte:head>
	<title>News Sitemap | SEO</title>
</svelte:head>

<div class="news-sitemap-page">
	<header class="page-header">
		<div>
			<h1>
				<IconNews size={28} />
				Google News Sitemap
			</h1>
			<p>Manage your Google News sitemap for better news indexing</p>
		</div>
		<div class="header-actions">
			<a href={sitemapUrl} target="_blank" rel="noopener" class="btn-secondary">
				<IconExternalLink size={18} />
				View Sitemap
			</a>
			<button class="btn-primary" onclick={regenerateSitemap} disabled={loading}>
				<IconRefresh size={18} class={loading ? 'spinning' : ''} />
				{loading ? 'Regenerating...' : 'Regenerate'}
			</button>
		</div>
	</header>

	<div class="settings-card">
		<h2>Sitemap Settings</h2>
		<div class="settings-grid">
			<div class="setting-group">
				<label>
					<input
						id="news-sitemap-enabled"
						name="news-sitemap-enabled"
						type="checkbox"
						bind:checked={settings.enabled}
					/>
					Enable News Sitemap
				</label>
			</div>

			<div class="setting-group">
				<label for="pubName">Publication Name</label>
				<input type="text" id="pubName" name="pubName" bind:value={settings.publicationName} />
			</div>

			<div class="setting-group">
				<label for="language">Language Code</label>
				<input type="text" id="language" name="language" bind:value={settings.language} placeholder="en" />
			</div>

			<div class="setting-group">
				<label for="maxAge">Max Article Age (hours)</label>
				<input type="number" id="maxAge" name="maxAge" bind:value={settings.maxAge} min="1" max="48" />
				<span class="hint">Google News requires articles within 48 hours</span>
			</div>
		</div>
	</div>

	<div class="articles-section">
		<div class="section-header">
			<h2>Articles in Sitemap ({articles.length})</h2>
			{#if lastGenerated}
				<span class="last-updated">
					<IconClock size={16} />
					Last updated: {getTimeAgo(lastGenerated)}
				</span>
			{/if}
		</div>

		{#if loading}
			<div class="loading">Loading articles...</div>
		{:else if articles.length === 0}
			<div class="empty-state">
				<IconNews size={48} />
				<p>No articles found within the last {settings.maxAge} hours</p>
			</div>
		{:else}
			<div class="articles-list">
				{#each articles as article}
					<div class="article-item">
						<div class="article-status" class:included={article.inSitemap}>
							{#if article.inSitemap}
								<IconCheck size={16} />
							{:else}
								<IconX size={16} />
							{/if}
						</div>
						<div class="article-info">
							<h4>{article.title}</h4>
							<div class="article-meta">
								<span class="article-url">{article.url}</span>
								<span class="article-date">{getTimeAgo(article.publishedAt)}</span>
							</div>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</div>

	<div class="info-section">
		<h3>About Google News Sitemaps</h3>
		<div class="info-grid">
			<div class="info-card">
				<h4>48-Hour Requirement</h4>
				<p>
					Google News only indexes articles published within the last 48 hours. Older content is
					automatically excluded.
				</p>
			</div>
			<div class="info-card">
				<h4>Publication Name</h4>
				<p>
					Must match your site name exactly as registered in Google Publisher Center for proper
					attribution.
				</p>
			</div>
			<div class="info-card">
				<h4>Stock Tickers</h4>
				<p>
					Financial articles can include stock ticker symbols for better categorization in Google
					Finance.
				</p>
			</div>
		</div>
	</div>
</div>

<style>
	.news-sitemap-page {
		padding: 2rem;
		max-width: 1200px;
		margin: 0 auto;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		font-size: 1.75rem;
		font-weight: 700;
		color: #1a1a1a;
		margin-bottom: 0.5rem;
	}

	.page-header p {
		color: #666;
		font-size: 0.95rem;
	}

	.header-actions {
		display: flex;
		gap: 1rem;
	}

	.btn-primary,
	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: 6px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
	}

	.btn-primary {
		background: #3b82f6;
		color: white;
		border: none;
	}

	.btn-primary:hover:not(:disabled) {
		background: #2563eb;
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-secondary {
		background: white;
		color: #374151;
		border: 1px solid #e5e5e5;
	}

	.btn-secondary:hover {
		background: #f9fafb;
	}

	:global(.spinning) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	.settings-card {
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 12px;
		padding: 1.5rem;
		margin-bottom: 2rem;
	}

	.settings-card h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #1a1a1a;
		margin-bottom: 1.5rem;
	}

	.settings-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
		gap: 1.5rem;
	}

	.setting-group {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.setting-group label {
		font-weight: 500;
		color: #374151;
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.setting-group input[type='text'],
	.setting-group input[type='number'] {
		padding: 0.75rem;
		border: 1px solid #e5e5e5;
		border-radius: 6px;
		font-size: 0.95rem;
	}

	.setting-group input:focus {
		outline: none;
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.hint {
		font-size: 0.85rem;
		color: #666;
	}

	.articles-section {
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 12px;
		padding: 1.5rem;
		margin-bottom: 2rem;
	}

	.section-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.5rem;
	}

	.section-header h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #1a1a1a;
		margin: 0;
	}

	.last-updated {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.85rem;
		color: #666;
	}

	.loading,
	.empty-state {
		text-align: center;
		padding: 3rem;
		color: #666;
	}

	.empty-state p {
		margin-top: 1rem;
	}

	.articles-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.article-item {
		display: flex;
		align-items: flex-start;
		gap: 1rem;
		padding: 1rem;
		background: #f9fafb;
		border-radius: 8px;
	}

	.article-status {
		width: 24px;
		height: 24px;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #fee2e2;
		color: #ef4444;
		flex-shrink: 0;
	}

	.article-status.included {
		background: #dcfce7;
		color: #16a34a;
	}

	.article-info {
		flex: 1;
		min-width: 0;
	}

	.article-info h4 {
		font-size: 0.95rem;
		font-weight: 600;
		color: #1a1a1a;
		margin: 0 0 0.5rem;
	}

	.article-meta {
		display: flex;
		gap: 1.5rem;
		font-size: 0.85rem;
		color: #666;
	}

	.article-url {
		color: #3b82f6;
	}

	.info-section h3 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #1a1a1a;
		margin-bottom: 1rem;
	}

	.info-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
		gap: 1.5rem;
	}

	.info-card {
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 8px;
		padding: 1.5rem;
	}

	.info-card h4 {
		font-size: 1rem;
		font-weight: 600;
		color: #1a1a1a;
		margin-bottom: 0.5rem;
	}

	.info-card p {
		color: #666;
		line-height: 1.6;
		font-size: 0.9rem;
	}

	@media (max-width: 768px) {
		.news-sitemap-page {
			padding: 1rem;
		}

		.page-header {
			flex-direction: column;
			gap: 1rem;
		}

		.header-actions {
			width: 100%;
			flex-direction: column;
		}

		.section-header {
			flex-direction: column;
			gap: 0.5rem;
			align-items: flex-start;
		}

		.article-meta {
			flex-direction: column;
			gap: 0.25rem;
		}
	}
</style>

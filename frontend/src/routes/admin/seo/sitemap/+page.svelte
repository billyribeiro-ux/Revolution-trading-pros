<script lang="ts">
	import { onMount } from 'svelte';
	import {
		IconRefresh,
		IconDownload,
		IconUpload,
		IconFileText,
		IconCheck,
		IconX
	} from '$lib/icons';

	let stats: any = null;
	let generating = false;
	let submitting = false;
	let showSitemapXml = false;
	let sitemapContent = '';

	onMount(() => {
		loadStats();
	});

	async function loadStats() {
		try {
			const response = await fetch('/api/seo/sitemap/stats');
			stats = await response.json();
		} catch (error) {
			console.error('Failed to load stats:', error);
		}
	}

	async function generateSitemap() {
		generating = true;
		try {
			const response = await fetch('/api/seo/sitemap/generate', { method: 'POST' });
			sitemapContent = await response.text();
			showSitemapXml = true;
			loadStats();
		} catch (error) {
			console.error('Failed to generate sitemap:', error);
			alert('Failed to generate sitemap');
		} finally {
			generating = false;
		}
	}

	async function submitToSearchEngines() {
		submitting = true;
		try {
			const sitemapUrl = `${window.location.origin}/sitemap.xml`;
			const response = await fetch('/api/seo/sitemap/submit', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ sitemap_url: sitemapUrl })
			});

			if (response.ok) {
				alert('Sitemap submitted to search engines successfully!');
			}
		} catch (error) {
			console.error('Failed to submit sitemap:', error);
			alert('Failed to submit sitemap to search engines');
		} finally {
			submitting = false;
		}
	}

	function downloadSitemap() {
		const blob = new Blob([sitemapContent], { type: 'application/xml' });
		const url = URL.createObjectURL(blob);
		const a = document.createElement('a');
		a.href = url;
		a.download = 'sitemap.xml';
		a.click();
		URL.revokeObjectURL(url);
	}
</script>

<svelte:head>
	<title>Sitemap Manager | SEO</title>
</svelte:head>

<div class="sitemap-page">
	<header class="page-header">
		<div>
			<h1>XML Sitemap</h1>
			<p>Generate and submit sitemaps to search engines</p>
		</div>
	</header>

	{#if stats}
		<div class="stats-grid">
			<div class="stat-card">
				<IconFileText size={32} />
				<div class="stat-value">{stats.total_urls || 0}</div>
				<div class="stat-label">Total URLs</div>
			</div>
			<div class="stat-card">
				<div class="stat-value">
					{stats.last_generated
						? new Date(stats.last_generated * 1000).toLocaleDateString()
						: 'Never'}
				</div>
				<div class="stat-label">Last Generated</div>
			</div>
			<div class="stat-card">
				<div class="stat-value">
					{stats.file_size ? (stats.file_size / 1024).toFixed(1) + ' KB' : '0 KB'}
				</div>
				<div class="stat-label">File Size</div>
			</div>
		</div>
	{/if}

	<div class="actions-section">
		<h2>Actions</h2>
		<div class="action-buttons">
			<button class="action-btn primary" onclick={generateSitemap} disabled={generating}>
				<IconRefresh size={20} class={generating ? 'spinning' : ''} />
				{generating ? 'Generating...' : 'Generate Sitemap'}
			</button>

			<button
				class="action-btn"
				onclick={submitToSearchEngines}
				disabled={submitting || !stats?.total_urls}
			>
				<IconUpload size={20} />
				{submitting ? 'Submitting...' : 'Submit to Search Engines'}
			</button>

			{#if sitemapContent}
				<button class="action-btn" onclick={downloadSitemap}>
					<IconDownload size={20} />
					Download XML
				</button>
			{/if}
		</div>
	</div>

	<div class="info-section">
		<h2>About XML Sitemaps</h2>
		<div class="info-grid">
			<div class="info-card">
				<h4>What is a Sitemap?</h4>
				<p>
					An XML sitemap helps search engines discover and index your website's pages more
					efficiently.
				</p>
			</div>
			<div class="info-card">
				<h4>When to Generate?</h4>
				<p>
					Generate a new sitemap whenever you add, update, or remove significant content from your
					site.
				</p>
			</div>
			<div class="info-card">
				<h4>Automatic Submission</h4>
				<p>
					Your sitemap is automatically submitted to Google and Bing when you click "Submit to
					Search Engines".
				</p>
			</div>
		</div>
	</div>
</div>

{#if showSitemapXml && sitemapContent}
	<div
		class="modal-overlay"
		role="button"
		tabindex="0"
		onclick={() => (showSitemapXml = false)}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showSitemapXml = false)}
	>
		<div
			class="modal"
			role="dialog"
			tabindex="0"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showSitemapXml = false)}
		>
			<div class="modal-header">
				<h3>Generated Sitemap</h3>
				<button class="close-btn" onclick={() => (showSitemapXml = false)}>×</button>
			</div>
			<div class="modal-body">
				<pre><code>{sitemapContent}</code></pre>
			</div>
		</div>
	</div>
{/if}

<style>
	.sitemap-page {
		padding: 2rem;
		max-width: 1200px;
		margin: 0 auto;
	}

	.page-header {
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #1a1a1a;
		margin-bottom: 0.5rem;
	}

	.page-header p {
		color: #666;
		font-size: 0.95rem;
	}

	.stats-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 1.5rem;
		margin-bottom: 3rem;
	}

	.stat-card {
		background: white;
		padding: 1.5rem;
		border-radius: 8px;
		border: 1px solid #e5e5e5;
		text-align: center;
	}

	.stat-value {
		font-size: 1.75rem;
		font-weight: 700;
		color: #1a1a1a;
		margin: 0.75rem 0 0.5rem;
	}

	.stat-label {
		color: #666;
		font-size: 0.9rem;
	}

	.actions-section {
		margin-bottom: 3rem;
	}

	.actions-section h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #1a1a1a;
		margin-bottom: 1rem;
	}

	.action-buttons {
		display: flex;
		gap: 1rem;
		flex-wrap: wrap;
	}

	.action-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 1rem 1.5rem;
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 8px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.action-btn.primary {
		background: #3b82f6;
		color: white;
		border-color: #3b82f6;
	}

	.action-btn:hover:not(:disabled) {
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
	}

	.action-btn.primary:hover:not(:disabled) {
		background: #2563eb;
	}

	.action-btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
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

	.info-section h2 {
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
		padding: 1.5rem;
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 8px;
	}

	.info-card h4 {
		font-size: 1rem;
		font-weight: 600;
		color: #1a1a1a;
		margin-bottom: 0.75rem;
	}

	.info-card p {
		color: #666;
		line-height: 1.6;
		font-size: 0.95rem;
	}

	.modal-overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 1rem;
	}

	.modal {
		background: white;
		border-radius: 12px;
		width: 100%;
		max-width: 800px;
		max-height: 90vh;
		display: flex;
		flex-direction: column;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem;
		border-bottom: 1px solid #e5e5e5;
	}

	.modal-header h3 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #1a1a1a;
		margin: 0;
	}

	.close-btn {
		background: none;
		border: none;
		font-size: 2rem;
		color: #666;
		cursor: pointer;
		line-height: 1;
	}

	.modal-body {
		padding: 1.5rem;
		overflow: auto;
		max-height: calc(90vh - 100px);
	}

	.modal-body pre {
		background: #1e293b;
		color: #e2e8f0;
		padding: 1.5rem;
		border-radius: 6px;
		overflow: auto;
		font-size: 0.85rem;
		line-height: 1.6;
	}

	.modal-body code {
		font-family: 'Courier New', monospace;
	}
</style>

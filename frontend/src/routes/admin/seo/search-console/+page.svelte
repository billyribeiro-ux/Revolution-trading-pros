<script lang="ts">
	import { onMount } from 'svelte';
	import {
		IconBrandGoogle,
		IconRefresh,
		IconDownload,
		IconPlugConnected,
		IconPlugConnectedX
	} from '$lib/icons';

	// State using Svelte 5 runes
	let status = $state<any>(null);
	let sites = $state<any[]>([]);
	// @ts-expect-error write-only state
	let loading = $state(false);
	let importing = $state(false);

	onMount(() => {
		checkStatus();
	});

	async function checkStatus() {
		loading = true;
		try {
			const response = await fetch('/api/seo/gsc/status');
			status = await response.json();

			if (status.connected) {
				loadSites();
			}
		} catch (error) {
			console.error('Failed to check status:', error);
		} finally {
			loading = false;
		}
	}

	async function loadSites() {
		try {
			const response = await fetch('/api/seo/gsc/sites');
			const data = await response.json();
			sites = data.sites || [];
		} catch (error) {
			console.error('Failed to load sites:', error);
		}
	}

	async function connect() {
		try {
			const response = await fetch('/api/seo/gsc/auth-url');
			const data = await response.json();

			if (data.auth_url) {
				window.location.href = data.auth_url;
			}
		} catch (error) {
			console.error('Failed to get auth URL:', error);
			alert('Failed to initiate connection');
		}
	}

	async function importAnalytics() {
		const startDate = new Date(Date.now() - 30 * 24 * 60 * 60 * 1000).toISOString().split('T')[0];
		const endDate = new Date().toISOString().split('T')[0];

		importing = true;
		try {
			const response = await fetch('/api/seo/gsc/import-analytics', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ start_date: startDate, end_date: endDate })
			});

			const data = await response.json();
			alert(`Successfully imported ${data.count} analytics records`);
		} catch (error) {
			console.error('Failed to import analytics:', error);
			alert('Failed to import analytics data');
		} finally {
			importing = false;
		}
	}

	async function updateKeywords() {
		try {
			const response = await fetch('/api/seo/gsc/update-keywords', { method: 'POST' });
			const data = await response.json();
			alert(`Updated ${data.count} keywords`);
		} catch (error) {
			console.error('Failed to update keywords:', error);
			alert('Failed to update keywords');
		}
	}
</script>

<svelte:head>
	<title>Google Search Console | SEO</title>
</svelte:head>

<div class="gsc-page">
	<header class="page-header">
		<div>
			<h1><IconBrandGoogle size={32} /> Google Search Console</h1>
			<p>Connect and sync data from Google Search Console</p>
		</div>
	</header>

	<div class="connection-status">
		{#if status}
			{#if status.connected}
				<div class="status-card connected">
					<IconPlugConnected size={48} />
					<h3>Connected</h3>
					<p>Your site is connected to Google Search Console</p>
					{#if status.site_info}
						<div class="site-info">
							<strong>Site:</strong>
							{status.site_info.site_url}<br />
							<strong>Permission:</strong>
							{status.site_info.permission_level}
						</div>
					{/if}
				</div>
			{:else}
				<div class="status-card disconnected">
					<IconPlugConnectedX size={48} />
					<h3>Not Connected</h3>
					<p>Connect to Google Search Console to import analytics data</p>
					<button class="btn-primary" onclick={connect}>
						<IconBrandGoogle size={20} />
						Connect with Google
					</button>
				</div>
			{/if}
		{/if}
	</div>

	{#if status?.connected}
		<div class="actions-section">
			<h2>Actions</h2>
			<div class="action-buttons">
				<button class="action-btn" onclick={importAnalytics} disabled={importing}>
					<IconDownload size={20} />
					{importing ? 'Importing...' : 'Import Analytics Data'}
				</button>

				<button class="action-btn" onclick={updateKeywords}>
					<IconRefresh size={20} />
					Update Keyword Rankings
				</button>
			</div>
		</div>

		{#if sites.length > 0}
			<div class="sites-section">
				<h2>Available Sites</h2>
				<div class="sites-list">
					{#each sites as site}
						<div class="site-card">
							<div class="site-url">{site.site_url}</div>
							<div class="site-permission">{site.permission_level}</div>
						</div>
					{/each}
				</div>
			</div>
		{/if}
	{/if}
</div>

<style>
	.gsc-page {
		padding: 2rem;
		max-width: 1200px;
		margin: 0 auto;
	}

	.page-header {
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

	.connection-status {
		margin-bottom: 3rem;
	}

	.status-card {
		padding: 3rem;
		background: white;
		border-radius: 12px;
		border: 2px solid;
		text-align: center;
	}

	.status-card.connected {
		border-color: #16a34a;
		background: #f0fdf4;
	}

	.status-card.disconnected {
		border-color: #e5e5e5;
	}

	.status-card h3 {
		font-size: 1.5rem;
		font-weight: 600;
		color: #1a1a1a;
		margin: 1rem 0 0.5rem;
	}

	.status-card p {
		color: #666;
		margin-bottom: 1.5rem;
	}

	.site-info {
		margin-top: 1.5rem;
		padding: 1rem;
		background: white;
		border-radius: 6px;
		text-align: left;
		font-size: 0.9rem;
		color: #666;
	}

	.site-info strong {
		color: #1a1a1a;
	}

	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: #3b82f6;
		color: white;
		border: none;
		border-radius: 8px;
		font-weight: 500;
		cursor: pointer;
		transition: background 0.2s;
	}

	.btn-primary:hover {
		background: #2563eb;
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

	.action-btn:hover:not(:disabled) {
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
	}

	.action-btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.sites-section h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #1a1a1a;
		margin-bottom: 1rem;
	}

	.sites-list {
		display: grid;
		gap: 1rem;
	}

	.site-card {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.25rem;
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 8px;
	}

	.site-url {
		font-weight: 500;
		color: #1a1a1a;
	}

	.site-permission {
		font-size: 0.85rem;
		color: #666;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}
</style>

<script lang="ts">
	import { onMount } from 'svelte';
	import IconWebhook from '@tabler/icons-svelte/icons/webhook';
	import IconPlus from '@tabler/icons-svelte/icons/plus';
	import IconSearch from '@tabler/icons-svelte/icons/search';
	import IconEdit from '@tabler/icons-svelte/icons/edit';
	import IconTrash from '@tabler/icons-svelte/icons/trash';
	import IconRefresh from '@tabler/icons-svelte/icons/refresh';
	import IconPlayerPlay from '@tabler/icons-svelte/icons/player-play';
	import IconCheck from '@tabler/icons-svelte/icons/check';
	import IconX from '@tabler/icons-svelte/icons/x';
	import IconActivity from '@tabler/icons-svelte/icons/activity';
	import IconAlertTriangle from '@tabler/icons-svelte/icons/alert-triangle';
	import { crmAPI } from '$lib/api/crm';
	import type { Webhook } from '$lib/crm/types';

	let webhooks = $state<Webhook[]>([]);
	let isLoading = $state(true);
	let error = $state('');
	let searchQuery = $state('');
	let testingWebhook = $state<string | null>(null);

	let stats = $state({
		total: 0,
		active: 0,
		totalTriggers: 0,
		totalFailures: 0
	});

	async function loadWebhooks() {
		isLoading = true;
		error = '';

		try {
			const response = await crmAPI.getWebhooks();
			webhooks = response.data || [];

			stats = {
				total: webhooks.length,
				active: webhooks.filter((w) => w.is_active).length,
				totalTriggers: webhooks.reduce((sum, w) => sum + w.trigger_count, 0),
				totalFailures: webhooks.reduce((sum, w) => sum + w.failure_count, 0)
			};
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load webhooks';
		} finally {
			isLoading = false;
		}
	}

	async function deleteWebhook(id: string) {
		if (!confirm('Are you sure you want to delete this webhook?')) return;

		try {
			await crmAPI.deleteWebhook(id);
			await loadWebhooks();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to delete webhook';
		}
	}

	async function testWebhook(id: string) {
		testingWebhook = id;
		try {
			const result = await crmAPI.testWebhook(id);
			if (result.success) {
				alert(`Test successful! Response code: ${result.response_code}`);
			} else {
				alert(`Test failed! Response code: ${result.response_code}`);
			}
		} catch (err) {
			alert(err instanceof Error ? err.message : 'Failed to test webhook');
		} finally {
			testingWebhook = null;
		}
	}

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function formatNumber(num: number): string {
		return num.toLocaleString();
	}

	let filteredWebhooks = $derived(
		webhooks.filter((webhook) => {
			if (searchQuery) {
				const query = searchQuery.toLowerCase();
				return (
					webhook.name.toLowerCase().includes(query) || webhook.url.toLowerCase().includes(query)
				);
			}
			return true;
		})
	);

	onMount(() => {
		loadWebhooks();
	});
</script>

<svelte:head>
	<title>Webhooks - FluentCRM Pro</title>
</svelte:head>

<div class="webhooks-page">
	<!-- Header -->
	<div class="page-header">
		<div>
			<h1>Webhooks</h1>
			<p class="page-description">Send real-time notifications to external services</p>
		</div>
		<div class="header-actions">
			<button class="btn-refresh" onclick={() => loadWebhooks()} disabled={isLoading}>
				<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
			</button>
			<a href="/admin/crm/webhooks/new" class="btn-primary">
				<IconPlus size={18} />
				New Webhook
			</a>
		</div>
	</div>

	<!-- Stats Cards -->
	<div class="stats-grid">
		<div class="stat-card">
			<div class="stat-icon blue">
				<IconWebhook size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{stats.total}</span>
				<span class="stat-label">Total Webhooks</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon green">
				<IconCheck size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{stats.active}</span>
				<span class="stat-label">Active</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon purple">
				<IconActivity size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatNumber(stats.totalTriggers)}</span>
				<span class="stat-label">Total Triggers</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon red">
				<IconAlertTriangle size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatNumber(stats.totalFailures)}</span>
				<span class="stat-label">Failures</span>
			</div>
		</div>
	</div>

	<!-- Search -->
	<div class="filters-bar">
		<div class="search-box">
			<IconSearch size={18} />
			<input type="text" placeholder="Search webhooks..." bind:value={searchQuery} />
		</div>
	</div>

	<!-- Webhooks List -->
	{#if isLoading}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>Loading webhooks...</p>
		</div>
	{:else if error}
		<div class="error-state">
			<p>{error}</p>
			<button onclick={() => loadWebhooks()}>Try Again</button>
		</div>
	{:else if filteredWebhooks.length === 0}
		<div class="empty-state">
			<IconWebhook size={48} />
			<h3>No webhooks configured</h3>
			<p>Create webhooks to send real-time notifications to external services</p>
			<a href="/admin/crm/webhooks/new" class="btn-primary">
				<IconPlus size={18} />
				Create Webhook
			</a>
		</div>
	{:else}
		<div class="webhooks-list">
			{#each filteredWebhooks as webhook}
				<div class="webhook-card">
					<div class="webhook-header">
						<div class="webhook-status" class:active={webhook.is_active}>
							{webhook.is_active ? 'Active' : 'Inactive'}
						</div>
						<div class="webhook-info">
							<h3 class="webhook-name">{webhook.name}</h3>
							<p class="webhook-url">{webhook.url}</p>
						</div>
					</div>
					<div class="webhook-events">
						<span class="events-label">Events:</span>
						<div class="events-list">
							{#each webhook.events.slice(0, 3) as event}
								<span class="event-badge">{event.replace(/_/g, ' ')}</span>
							{/each}
							{#if webhook.events.length > 3}
								<span class="event-more">+{webhook.events.length - 3} more</span>
							{/if}
						</div>
					</div>
					<div class="webhook-stats">
						<div class="stat-item">
							<span class="stat-number">{formatNumber(webhook.trigger_count)}</span>
							<span class="stat-text">triggers</span>
						</div>
						<div class="stat-item">
							<span class="stat-number failure">{formatNumber(webhook.failure_count)}</span>
							<span class="stat-text">failures</span>
						</div>
						{#if webhook.last_triggered_at}
							<div class="stat-item">
								<span class="stat-text">Last triggered: {formatDate(webhook.last_triggered_at)}</span>
							</div>
						{/if}
					</div>
					<div class="webhook-actions">
						<button
							class="btn-icon"
							title="Test Webhook"
							onclick={() => testWebhook(webhook.id)}
							disabled={testingWebhook === webhook.id}
						>
							{#if testingWebhook === webhook.id}
								<IconRefresh size={16} class="spinning" />
							{:else}
								<IconPlayerPlay size={16} />
							{/if}
						</button>
						<a href="/admin/crm/webhooks/{webhook.id}/logs" class="btn-icon" title="View Logs">
							<IconActivity size={16} />
						</a>
						<a href="/admin/crm/webhooks/{webhook.id}/edit" class="btn-icon" title="Edit">
							<IconEdit size={16} />
						</a>
						<button class="btn-icon danger" title="Delete" onclick={() => deleteWebhook(webhook.id)}>
							<IconTrash size={16} />
						</button>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.webhooks-page {
		max-width: 1200px;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
		flex-wrap: wrap;
		gap: 1rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.25rem 0;
	}

	.page-description {
		color: #64748b;
		margin: 0;
	}

	.header-actions {
		display: flex;
		gap: 0.75rem;
	}

	.btn-refresh {
		width: 42px;
		height: 42px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(30, 41, 59, 0.8);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 10px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-refresh:hover {
		background: rgba(99, 102, 241, 0.2);
		color: #818cf8;
	}

	.btn-refresh :global(.spinning) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		color: white;
		border: none;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(99, 102, 241, 0.4);
	}

	.stats-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
		margin-bottom: 2rem;
	}

	@media (max-width: 1024px) {
		.stats-grid { grid-template-columns: repeat(2, 1fr); }
	}

	@media (max-width: 640px) {
		.stats-grid { grid-template-columns: 1fr; }
	}

	.stat-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 14px;
	}

	.stat-icon {
		width: 52px;
		height: 52px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.stat-icon.blue { background: rgba(59, 130, 246, 0.15); color: #60a5fa; }
	.stat-icon.green { background: rgba(34, 197, 94, 0.15); color: #4ade80; }
	.stat-icon.purple { background: rgba(139, 92, 246, 0.15); color: #a78bfa; }
	.stat-icon.red { background: rgba(239, 68, 68, 0.15); color: #f87171; }

	.stat-content {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.stat-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.stat-label {
		font-size: 0.8rem;
		color: #64748b;
	}

	.filters-bar {
		display: flex;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 10px;
		flex: 1;
		max-width: 400px;
	}

	.search-box :global(svg) {
		color: #64748b;
	}

	.search-box input {
		flex: 1;
		padding: 0.75rem 0;
		background: transparent;
		border: none;
		color: #e2e8f0;
		font-size: 0.9rem;
		outline: none;
	}

	.search-box input::placeholder {
		color: #64748b;
	}

	.webhooks-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.webhook-card {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 14px;
		padding: 1.5rem;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.webhook-header {
		display: flex;
		align-items: flex-start;
		gap: 1rem;
	}

	.webhook-status {
		padding: 0.25rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 600;
		background: rgba(100, 116, 139, 0.2);
		color: #94a3b8;
	}

	.webhook-status.active {
		background: rgba(34, 197, 94, 0.15);
		color: #4ade80;
	}

	.webhook-info {
		flex: 1;
	}

	.webhook-name {
		font-size: 1.1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 0.25rem 0;
	}

	.webhook-url {
		font-size: 0.85rem;
		color: #64748b;
		margin: 0;
		font-family: monospace;
		word-break: break-all;
	}

	.webhook-events {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		flex-wrap: wrap;
	}

	.events-label {
		font-size: 0.8rem;
		color: #64748b;
	}

	.events-list {
		display: flex;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.event-badge {
		padding: 0.25rem 0.5rem;
		background: rgba(99, 102, 241, 0.1);
		border-radius: 4px;
		font-size: 0.7rem;
		color: #818cf8;
		text-transform: capitalize;
	}

	.event-more {
		font-size: 0.75rem;
		color: #64748b;
	}

	.webhook-stats {
		display: flex;
		gap: 2rem;
		padding-top: 0.75rem;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
	}

	.stat-item {
		display: flex;
		align-items: baseline;
		gap: 0.25rem;
	}

	.stat-number {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
	}

	.stat-number.failure {
		color: #f87171;
	}

	.stat-text {
		font-size: 0.8rem;
		color: #64748b;
	}

	.webhook-actions {
		display: flex;
		gap: 0.5rem;
		justify-content: flex-end;
	}

	.btn-icon {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
	}

	.btn-icon:hover {
		background: rgba(99, 102, 241, 0.1);
		color: #818cf8;
	}

	.btn-icon:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-icon.danger:hover {
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
		border-color: rgba(239, 68, 68, 0.3);
	}

	.loading-state, .error-state, .empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		text-align: center;
		color: #64748b;
	}

	.empty-state :global(svg) {
		color: #475569;
		margin-bottom: 1rem;
	}

	.empty-state h3 {
		color: #e2e8f0;
		margin: 0 0 0.5rem 0;
	}

	.empty-state p {
		margin: 0 0 1.5rem 0;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(99, 102, 241, 0.2);
		border-top-color: #6366f1;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 1rem;
	}
</style>

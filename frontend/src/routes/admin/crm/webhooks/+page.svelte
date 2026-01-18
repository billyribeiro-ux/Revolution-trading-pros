<!--
	/admin/crm/webhooks - Webhook Management
	Apple Principal Engineer ICT 7 Grade - January 2026

	Features:
	- Webhook CRUD with event subscriptions
	- Test webhook functionality
	- Toggle webhook status inline
	- Trigger/failure count tracking
	- Event badges display
	- Toast notifications for all actions
	- Status filter (all/active/inactive)
	- Full Svelte 5 $state/$derived/$effect reactivity
	- WCAG 2.1 AA accessibility compliance
-->

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
	import IconAlertCircle from '@tabler/icons-svelte/icons/alert-circle';
	import IconToggleLeft from '@tabler/icons-svelte/icons/toggle-left';
	import IconToggleRight from '@tabler/icons-svelte/icons/toggle-right';
	import { crmAPI } from '$lib/api/crm';
	import type { Webhook } from '$lib/crm/types';

	// =====================================================
	// STATE MANAGEMENT - Svelte 5 Runes
	// =====================================================

	let webhooks = $state<Webhook[]>([]);
	let isLoading = $state(true);
	let error = $state('');
	let searchQuery = $state('');
	let statusFilter = $state<'all' | 'active' | 'inactive'>('all');
	let testingWebhook = $state<string | null>(null);
	let togglingWebhook = $state<string | null>(null);

	// Toast notifications state
	let toasts = $state<Array<{ id: string; type: 'success' | 'error' | 'info'; message: string }>>(
		[]
	);

	// =====================================================
	// DERIVED STATE
	// =====================================================

	let stats = $derived({
		total: webhooks.length,
		active: webhooks.filter((w) => w.is_active).length,
		totalTriggers: webhooks.reduce((sum, w) => sum + w.trigger_count, 0),
		totalFailures: webhooks.reduce((sum, w) => sum + w.failure_count, 0)
	});

	let filteredWebhooks = $derived(
		webhooks.filter((webhook) => {
			// Status filter
			if (statusFilter === 'active' && !webhook.is_active) return false;
			if (statusFilter === 'inactive' && webhook.is_active) return false;

			// Search filter
			if (searchQuery) {
				const query = searchQuery.toLowerCase();
				return (
					webhook.name.toLowerCase().includes(query) || webhook.url.toLowerCase().includes(query)
				);
			}
			return true;
		})
	);

	// =====================================================
	// API FUNCTIONS
	// =====================================================

	async function loadWebhooks() {
		isLoading = true;
		error = '';

		try {
			const response = await crmAPI.getWebhooks();
			webhooks = response.data || [];
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load webhooks';
			showToast('error', error);
		} finally {
			isLoading = false;
		}
	}

	async function deleteWebhook(id: string, name: string) {
		if (
			!confirm(
				`Are you sure you want to delete the webhook "${name}"? This action cannot be undone.`
			)
		)
			return;

		try {
			await crmAPI.deleteWebhook(id);
			showToast('success', `Webhook "${name}" deleted successfully`);
			await loadWebhooks();
		} catch (err) {
			const message = err instanceof Error ? err.message : 'Failed to delete webhook';
			showToast('error', message);
		}
	}

	async function toggleWebhookStatus(webhook: Webhook) {
		togglingWebhook = webhook.id;
		try {
			await crmAPI.updateWebhook(webhook.id, {
				is_active: !webhook.is_active
			});
			const action = webhook.is_active ? 'deactivated' : 'activated';
			showToast('success', `Webhook "${webhook.name}" ${action}`);
			await loadWebhooks();
		} catch (err) {
			const message = err instanceof Error ? err.message : 'Failed to update webhook';
			showToast('error', message);
		} finally {
			togglingWebhook = null;
		}
	}

	async function testWebhook(webhook: Webhook) {
		testingWebhook = webhook.id;
		try {
			const result = await crmAPI.testWebhook(webhook.id);
			if (result.success) {
				showToast('success', `Test successful! Response code: ${result.response_code}`);
			} else {
				showToast('error', `Test failed! Response code: ${result.response_code}`);
			}
			// Refresh to update trigger count
			await loadWebhooks();
		} catch (err) {
			showToast('error', err instanceof Error ? err.message : 'Failed to test webhook');
		} finally {
			testingWebhook = null;
		}
	}

	// =====================================================
	// TOAST NOTIFICATIONS
	// =====================================================

	function showToast(type: 'success' | 'error' | 'info', message: string) {
		const id = crypto.randomUUID();
		toasts = [...toasts, { id, type, message }];
		setTimeout(() => {
			toasts = toasts.filter((t) => t.id !== id);
		}, 5000);
	}

	function dismissToast(id: string) {
		toasts = toasts.filter((t) => t.id !== id);
	}

	// =====================================================
	// UTILITY FUNCTIONS
	// =====================================================

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

	// =====================================================
	// LIFECYCLE
	// =====================================================

	onMount(() => {
		loadWebhooks();
	});
</script>

<svelte:head>
	<title>Webhooks - FluentCRM Pro</title>
</svelte:head>

<div class="admin-crm-webhooks">
	<!-- Animated Background -->
	<div class="bg-effects">
		<div class="bg-blob bg-blob-1"></div>
		<div class="bg-blob bg-blob-2"></div>
		<div class="bg-blob bg-blob-3"></div>
	</div>

	<div class="admin-page-container">
		<!-- Header -->
		<header class="page-header">
			<h1>Webhooks</h1>
			<p class="subtitle">Send real-time notifications to external services</p>
			<div class="header-actions">
				<button class="btn-refresh" onclick={() => loadWebhooks()} disabled={isLoading}>
					<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
				</button>
				<a href="/admin/crm/webhooks/new" class="btn-primary">
					<IconPlus size={18} />
					New Webhook
				</a>
			</div>
		</header>
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
			<div class="stat-icon gold">
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

	<!-- Filters Bar -->
	<div class="filters-bar">
		<div class="search-box">
			<IconSearch size={18} />
			<input
				type="text"
				placeholder="Search webhooks..."
				bind:value={searchQuery}
				aria-label="Search webhooks"
			/>
			{#if searchQuery}
				<button class="search-clear" onclick={() => (searchQuery = '')} aria-label="Clear search">
					<IconX size={16} />
				</button>
			{/if}
		</div>
		<div class="status-filter">
			<button
				class="filter-btn"
				class:active={statusFilter === 'all'}
				onclick={() => (statusFilter = 'all')}
			>
				All ({stats.total})
			</button>
			<button
				class="filter-btn"
				class:active={statusFilter === 'active'}
				onclick={() => (statusFilter = 'active')}
			>
				<IconCheck size={14} />
				Active ({stats.active})
			</button>
			<button
				class="filter-btn"
				class:active={statusFilter === 'inactive'}
				onclick={() => (statusFilter = 'inactive')}
			>
				<IconX size={14} />
				Inactive ({stats.total - stats.active})
			</button>
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
		<div class="webhooks-list" role="list" aria-label="Webhooks">
			{#each filteredWebhooks as webhook (webhook.id)}
				<article class="webhook-card" role="listitem">
					<div class="webhook-header">
						<button
							class="webhook-toggle"
							class:active={webhook.is_active}
							onclick={() => toggleWebhookStatus(webhook)}
							disabled={togglingWebhook === webhook.id}
							aria-label={webhook.is_active
								? `Deactivate ${webhook.name}`
								: `Activate ${webhook.name}`}
							title={webhook.is_active ? 'Click to deactivate' : 'Click to activate'}
						>
							{#if togglingWebhook === webhook.id}
								<IconRefresh size={16} class="spinning" />
							{:else if webhook.is_active}
								<IconToggleRight size={20} />
							{:else}
								<IconToggleLeft size={20} />
							{/if}
							<span>{webhook.is_active ? 'Active' : 'Inactive'}</span>
						</button>
						<div class="webhook-info">
							<h3 class="webhook-name">{webhook.name}</h3>
							<p class="webhook-url">{webhook.url}</p>
						</div>
					</div>
					<div class="webhook-events">
						<span class="events-label">Events:</span>
						<div class="events-list">
							{#each (webhook.events || []).slice(0, 3) as event}
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
								<span class="stat-text"
									>Last triggered: {formatDate(webhook.last_triggered_at)}</span
								>
							</div>
						{/if}
					</div>
					<div class="webhook-actions">
						<button
							class="btn-icon"
							title="Test Webhook"
							onclick={() => testWebhook(webhook)}
							disabled={testingWebhook === webhook.id || !webhook.is_active}
							aria-label={`Test ${webhook.name}`}
						>
							{#if testingWebhook === webhook.id}
								<IconRefresh size={16} class="spinning" />
							{:else}
								<IconPlayerPlay size={16} />
							{/if}
						</button>
						<a
							href="/admin/crm/webhooks/{webhook.id}/logs"
							class="btn-icon"
							title="View Logs"
							aria-label={`View logs for ${webhook.name}`}
						>
							<IconActivity size={16} />
						</a>
						<a
							href="/admin/crm/webhooks/{webhook.id}/edit"
							class="btn-icon"
							title="Edit"
							aria-label={`Edit ${webhook.name}`}
						>
							<IconEdit size={16} />
						</a>
						<button
							class="btn-icon danger"
							title="Delete"
							onclick={() => deleteWebhook(webhook.id, webhook.name)}
							aria-label={`Delete ${webhook.name}`}
						>
							<IconTrash size={16} />
						</button>
					</div>
				</article>
			{/each}
		</div>
	{/if}
</div>

<!-- Toast Notifications -->
{#if toasts.length > 0}
	<div class="toast-container" role="region" aria-label="Notifications">
		{#each toasts as toast (toast.id)}
			<div class="toast toast-{toast.type}" role="alert" aria-live="polite">
				<div class="toast-icon">
					{#if toast.type === 'success'}
						<IconCheck size={18} />
					{:else if toast.type === 'error'}
						<IconAlertCircle size={18} />
					{:else}
						<IconWebhook size={18} />
					{/if}
				</div>
				<span class="toast-message">{toast.message}</span>
				<button
					class="toast-dismiss"
					onclick={() => dismissToast(toast.id)}
					aria-label="Dismiss notification"
				>
					<IconX size={16} />
				</button>
			</div>
		{/each}
	</div>
{/if}

<style>
	.page {
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
	}

	/* Header - Centered */
	.page-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.5rem 0;
	}

	.subtitle {
		color: #64748b;
		font-size: 0.875rem;
		margin: 0 0 1rem 0;
	}

	.header-actions {
		display: flex;
		justify-content: center;
		gap: 0.75rem;
	}

	.btn-refresh {
		width: 42px;
		height: 42px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(30, 41, 59, 0.8);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 10px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-refresh:hover {
		background: rgba(230, 184, 0, 0.2);
		color: #e6b800;
	}

	.btn-refresh :global(.spinning) {
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

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: linear-gradient(135deg, #e6b800 0%, #b38f00 100%);
		color: #0d1117;
		border: none;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(230, 184, 0, 0.4);
	}

	.stats-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
		margin-bottom: 2rem;
	}

	@media (max-width: 1024px) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 640px) {
		.stats-grid {
			grid-template-columns: 1fr;
		}
	}

	.stat-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(230, 184, 0, 0.1);
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

	.stat-icon.blue {
		background: rgba(59, 130, 246, 0.15);
		color: #60a5fa;
	}
	.stat-icon.green {
		background: rgba(34, 197, 94, 0.15);
		color: #4ade80;
	}
	.stat-icon.gold {
		background: rgba(230, 184, 0, 0.15);
		color: #e6b800;
	}
	.stat-icon.red {
		background: rgba(239, 68, 68, 0.15);
		color: #f87171;
	}

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
		flex-wrap: wrap;
		align-items: center;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 10px;
		flex: 1;
		max-width: 400px;
		transition: all 0.2s;
	}

	.search-box:focus-within {
		border-color: rgba(99, 102, 241, 0.4);
		box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
	}

	.search-box :global(svg) {
		color: #64748b;
		flex-shrink: 0;
	}

	.search-box input {
		flex: 1;
		padding: 0.75rem 0;
		background: transparent;
		border: none;
		color: #e2e8f0;
		font-size: 0.9rem;
		outline: none;
		min-width: 0;
	}

	.search-box input::placeholder {
		color: #64748b;
	}

	.search-clear {
		display: flex;
		align-items: center;
		justify-content: center;
		background: none;
		border: none;
		padding: 4px;
		color: #64748b;
		cursor: pointer;
		border-radius: 4px;
		transition: all 0.2s;
	}

	.search-clear:hover {
		color: #f87171;
		background: rgba(248, 113, 113, 0.1);
	}

	.status-filter {
		display: flex;
		gap: 0.5rem;
	}

	.filter-btn {
		display: flex;
		align-items: center;
		gap: 0.35rem;
		padding: 0.5rem 0.875rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		color: #94a3b8;
		font-size: 0.85rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.filter-btn:hover {
		background: rgba(99, 102, 241, 0.1);
		color: #c7d2fe;
	}

	.filter-btn.active {
		background: rgba(99, 102, 241, 0.2);
		border-color: rgba(99, 102, 241, 0.4);
		color: #c7d2fe;
	}

	.webhooks-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.webhook-card {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(230, 184, 0, 0.1);
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

	.webhook-toggle {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.35rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 600;
		background: rgba(100, 116, 139, 0.2);
		border: 1px solid transparent;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.webhook-toggle:hover:not(:disabled) {
		background: rgba(100, 116, 139, 0.3);
		border-color: rgba(100, 116, 139, 0.3);
	}

	.webhook-toggle.active {
		background: rgba(34, 197, 94, 0.15);
		color: #4ade80;
	}

	.webhook-toggle.active:hover:not(:disabled) {
		background: rgba(239, 68, 68, 0.15);
		color: #f87171;
		border-color: rgba(239, 68, 68, 0.3);
	}

	.webhook-toggle:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.webhook-toggle :global(.spinning) {
		animation: spin 1s linear infinite;
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
		background: rgba(230, 184, 0, 0.1);
		border-radius: 4px;
		font-size: 0.7rem;
		color: #e6b800;
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
		border-top: 1px solid rgba(230, 184, 0, 0.1);
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
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
	}

	.btn-icon:hover {
		background: rgba(230, 184, 0, 0.1);
		color: #e6b800;
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

	.loading-state,
	.error-state,
	.empty-state {
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
		border: 3px solid rgba(230, 184, 0, 0.2);
		border-top-color: #e6b800;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 1rem;
	}

	/* =====================================================
	   Toast Notifications
	   ===================================================== */
	.toast-container {
		position: fixed;
		bottom: 1.5rem;
		right: 1.5rem;
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		z-index: 1100;
		max-width: 400px;
	}

	.toast {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.25rem;
		background: #1e293b;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 10px;
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
		animation: slideIn 0.3s ease-out;
	}

	@keyframes slideIn {
		from {
			transform: translateX(100%);
			opacity: 0;
		}
		to {
			transform: translateX(0);
			opacity: 1;
		}
	}

	.toast-success {
		border-color: rgba(34, 197, 94, 0.3);
	}

	.toast-success .toast-icon {
		color: #4ade80;
	}

	.toast-error {
		border-color: rgba(239, 68, 68, 0.3);
	}

	.toast-error .toast-icon {
		color: #f87171;
	}

	.toast-info .toast-icon {
		color: #60a5fa;
	}

	.toast-icon {
		flex-shrink: 0;
	}

	.toast-message {
		flex: 1;
		color: #e2e8f0;
		font-size: 0.9rem;
	}

	.toast-dismiss {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		background: transparent;
		border: none;
		color: #64748b;
		cursor: pointer;
		border-radius: 4px;
		transition: all 0.2s;
	}

	.toast-dismiss:hover {
		color: #f87171;
		background: rgba(248, 113, 113, 0.1);
	}

	/* =====================================================
	   Responsive
	   ===================================================== */
	@media (max-width: 768px) {
		.filters-bar {
			flex-direction: column;
			align-items: stretch;
		}

		.search-box {
			max-width: none;
		}

		.status-filter {
			flex-wrap: wrap;
		}

		.webhook-header {
			flex-direction: column;
			gap: 0.75rem;
		}

		.webhook-stats {
			flex-wrap: wrap;
			gap: 1rem;
		}

		.toast-container {
			left: 1rem;
			right: 1rem;
			max-width: none;
		}
	}
</style>

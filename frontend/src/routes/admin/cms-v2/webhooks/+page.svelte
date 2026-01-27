<!--
	CMS v2 Webhooks Management
	═══════════════════════════════════════════════════════════════════════════════

	Manage webhook configurations for external integrations:
	- Create/edit/delete webhooks
	- View delivery history
	- Test webhook endpoints
	- Monitor success/failure rates

	@version 1.0.0
	@author Revolution Trading Pros
	@since January 2026
-->

<script lang="ts">
	import { browser } from '$app/environment';
	import { fly, scale } from 'svelte/transition';
	import {
		IconWebhook,
		IconPlus,
		IconTrash,
		IconRefresh,
		IconCheck,
		IconAlertTriangle,
		IconClock,
		IconExternalLink,
		IconEdit,
		IconEye
	} from '$lib/icons';

	// ═══════════════════════════════════════════════════════════════════════════
	// Types
	// ═══════════════════════════════════════════════════════════════════════════

	interface Webhook {
		id: string;
		name: string;
		url: string;
		events: string[];
		secret: string | null;
		headers: Record<string, string> | null;
		contentTypes: string[] | null;
		isActive: boolean;
		retryCount: number;
		timeoutSeconds: number;
		successCount: number;
		failureCount: number;
		lastTriggeredAt: string | null;
		createdAt: string;
		updatedAt: string;
	}

	interface WebhookDelivery {
		id: string;
		webhookId: string;
		eventType: string;
		contentId: string | null;
		payload: Record<string, unknown>;
		status: 'pending' | 'delivered' | 'failed' | 'retrying';
		attempts: number;
		responseStatus: number | null;
		responseBody: string | null;
		responseTimeMs: number | null;
		errorMessage: string | null;
		createdAt: string;
		deliveredAt: string | null;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// State
	// ═══════════════════════════════════════════════════════════════════════════

	let webhooks = $state<Webhook[]>([]);
	let selectedWebhook = $state<Webhook | null>(null);
	let deliveries = $state<WebhookDelivery[]>([]);

	let isLoading = $state(true);
	let isSaving = $state(false);
	let error = $state<string | null>(null);
	let successMessage = $state<string | null>(null);

	let showCreateModal = $state(false);
	let showDeliveriesModal = $state(false);
	let showTestModal = $state(false);

	// Form state
	let formName = $state('');
	let formUrl = $state('');
	let formEvents = $state<string[]>(['content.published']);
	let formSecret = $state('');
	let formRetryCount = $state(3);
	let formTimeoutSeconds = $state(30);
	let formIsActive = $state(true);
	let isEditing = $state(false);
	let editingId = $state<string | null>(null);

	// Event options
	const eventOptions = [
		{ value: 'content.created', label: 'Content Created' },
		{ value: 'content.updated', label: 'Content Updated' },
		{ value: 'content.published', label: 'Content Published' },
		{ value: 'content.unpublished', label: 'Content Unpublished' },
		{ value: 'content.deleted', label: 'Content Deleted' },
		{ value: 'asset.uploaded', label: 'Asset Uploaded' },
		{ value: 'asset.deleted', label: 'Asset Deleted' }
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// Lifecycle
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		if (browser) {
			loadWebhooks();
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// Data Loading
	// ═══════════════════════════════════════════════════════════════════════════

	async function loadWebhooks() {
		isLoading = true;
		try {
			const response = await fetch('/api/admin/cms-v2/webhooks', {
				credentials: 'include'
			});
			if (!response.ok) throw new Error('Failed to load webhooks');
			webhooks = await response.json();
		} catch (e: any) {
			error = e.message;
		} finally {
			isLoading = false;
		}
	}

	async function loadDeliveries(webhookId: string) {
		try {
			const response = await fetch(`/api/admin/cms-v2/webhooks/${webhookId}/deliveries?limit=50`, {
				credentials: 'include'
			});
			if (!response.ok) throw new Error('Failed to load deliveries');
			deliveries = await response.json();
		} catch (e: any) {
			error = e.message;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Actions
	// ═══════════════════════════════════════════════════════════════════════════

	function openCreateModal() {
		resetForm();
		isEditing = false;
		editingId = null;
		showCreateModal = true;
	}

	function openEditModal(webhook: Webhook) {
		formName = webhook.name;
		formUrl = webhook.url;
		formEvents = [...webhook.events];
		formSecret = webhook.secret || '';
		formRetryCount = webhook.retryCount;
		formTimeoutSeconds = webhook.timeoutSeconds;
		formIsActive = webhook.isActive;
		isEditing = true;
		editingId = webhook.id;
		showCreateModal = true;
	}

	function resetForm() {
		formName = '';
		formUrl = '';
		formEvents = ['content.published'];
		formSecret = '';
		formRetryCount = 3;
		formTimeoutSeconds = 30;
		formIsActive = true;
	}

	async function saveWebhook() {
		if (!formName.trim() || !formUrl.trim() || formEvents.length === 0) {
			error = 'Name, URL, and at least one event are required';
			return;
		}

		isSaving = true;
		error = null;

		try {
			const payload = {
				name: formName.trim(),
				url: formUrl.trim(),
				events: formEvents,
				secret: formSecret.trim() || null,
				retryCount: formRetryCount,
				timeoutSeconds: formTimeoutSeconds,
				isActive: formIsActive
			};

			const url = isEditing
				? `/api/admin/cms-v2/webhooks/${editingId}`
				: '/api/admin/cms-v2/webhooks';

			const response = await fetch(url, {
				method: isEditing ? 'PUT' : 'POST',
				headers: { 'Content-Type': 'application/json' },
				credentials: 'include',
				body: JSON.stringify(payload)
			});

			if (!response.ok) {
				const data = await response.json();
				throw new Error(data.error || 'Failed to save webhook');
			}

			successMessage = isEditing ? 'Webhook updated' : 'Webhook created';
			setTimeout(() => successMessage = null, 3000);
			showCreateModal = false;
			await loadWebhooks();
		} catch (e: any) {
			error = e.message;
		} finally {
			isSaving = false;
		}
	}

	async function deleteWebhook(webhook: Webhook) {
		if (!confirm(`Delete webhook "${webhook.name}"? This cannot be undone.`)) return;

		try {
			const response = await fetch(`/api/admin/cms-v2/webhooks/${webhook.id}`, {
				method: 'DELETE',
				credentials: 'include'
			});

			if (!response.ok) throw new Error('Failed to delete webhook');

			successMessage = 'Webhook deleted';
			setTimeout(() => successMessage = null, 3000);
			await loadWebhooks();
		} catch (e: any) {
			error = e.message;
		}
	}

	async function toggleWebhook(webhook: Webhook) {
		try {
			const response = await fetch(`/api/admin/cms-v2/webhooks/${webhook.id}`, {
				method: 'PUT',
				headers: { 'Content-Type': 'application/json' },
				credentials: 'include',
				body: JSON.stringify({ isActive: !webhook.isActive })
			});

			if (!response.ok) throw new Error('Failed to update webhook');

			await loadWebhooks();
		} catch (e: any) {
			error = e.message;
		}
	}

	async function testWebhook(webhook: Webhook) {
		try {
			const response = await fetch(`/api/admin/cms-v2/webhooks/${webhook.id}/test`, {
				method: 'POST',
				credentials: 'include'
			});

			if (!response.ok) throw new Error('Test failed');

			successMessage = 'Test webhook sent successfully';
			setTimeout(() => successMessage = null, 3000);
		} catch (e: any) {
			error = e.message;
		}
	}

	async function retryDelivery(delivery: WebhookDelivery) {
		try {
			const response = await fetch(`/api/admin/cms-v2/webhooks/deliveries/${delivery.id}/retry`, {
				method: 'POST',
				credentials: 'include'
			});

			if (!response.ok) throw new Error('Retry failed');

			successMessage = 'Delivery queued for retry';
			setTimeout(() => successMessage = null, 3000);
			await loadDeliveries(delivery.webhookId);
		} catch (e: any) {
			error = e.message;
		}
	}

	function openDeliveriesModal(webhook: Webhook) {
		selectedWebhook = webhook;
		loadDeliveries(webhook.id);
		showDeliveriesModal = true;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Helpers
	// ═══════════════════════════════════════════════════════════════════════════

	function formatDate(dateString: string | null): string {
		if (!dateString) return 'Never';
		return new Date(dateString).toLocaleString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function getSuccessRate(webhook: Webhook): number {
		const total = webhook.successCount + webhook.failureCount;
		if (total === 0) return 100;
		return Math.round((webhook.successCount / total) * 100);
	}

	function getStatusColor(status: string): string {
		switch (status) {
			case 'delivered': return 'green';
			case 'pending': return 'blue';
			case 'retrying': return 'yellow';
			case 'failed': return 'red';
			default: return 'gray';
		}
	}

	function toggleEvent(event: string) {
		if (formEvents.includes(event)) {
			formEvents = formEvents.filter(e => e !== event);
		} else {
			formEvents = [...formEvents, event];
		}
	}

	function generateSecret() {
		const array = new Uint8Array(32);
		crypto.getRandomValues(array);
		formSecret = Array.from(array, b => b.toString(16).padStart(2, '0')).join('');
	}
</script>

<svelte:head>
	<title>Webhooks | CMS v2</title>
</svelte:head>

<div class="webhooks-page">
	<!-- Header -->
	<header class="page-header" in:fly={{ y: -20, duration: 300 }}>
		<div class="header-content">
			<div class="header-icon">
				<IconWebhook size={28} />
			</div>
			<div>
				<h1 class="page-title">Webhooks</h1>
				<p class="page-subtitle">Manage external integrations and event notifications</p>
			</div>
		</div>

		<button class="btn-primary" onclick={openCreateModal}>
			<IconPlus size={18} />
			Create Webhook
		</button>
	</header>

	{#if error}
		<div class="error-banner" in:scale={{ duration: 200 }}>
			<IconAlertTriangle size={18} />
			<span>{error}</span>
			<button onclick={() => error = null}>Dismiss</button>
		</div>
	{/if}

	{#if successMessage}
		<div class="success-banner" in:scale={{ duration: 200 }}>
			<IconCheck size={18} />
			<span>{successMessage}</span>
		</div>
	{/if}

	<!-- Stats -->
	<div class="stats-grid" in:fly={{ y: 20, duration: 300, delay: 50 }}>
		<div class="stat-card">
			<span class="stat-value">{webhooks.length}</span>
			<span class="stat-label">Total Webhooks</span>
		</div>
		<div class="stat-card">
			<span class="stat-value">{webhooks.filter(w => w.isActive).length}</span>
			<span class="stat-label">Active</span>
		</div>
		<div class="stat-card">
			<span class="stat-value">{webhooks.reduce((sum, w) => sum + w.successCount, 0)}</span>
			<span class="stat-label">Successful Deliveries</span>
		</div>
		<div class="stat-card">
			<span class="stat-value">{webhooks.reduce((sum, w) => sum + w.failureCount, 0)}</span>
			<span class="stat-label">Failed Deliveries</span>
		</div>
	</div>

	<!-- Webhooks List -->
	<div class="webhooks-list" in:fly={{ y: 20, duration: 300, delay: 100 }}>
		{#if isLoading}
			<div class="loading-state">
				<div class="spinner"></div>
				<p>Loading webhooks...</p>
			</div>
		{:else if webhooks.length === 0}
			<div class="empty-state">
				<IconWebhook size={48} />
				<h3>No Webhooks</h3>
				<p>Create a webhook to start receiving event notifications.</p>
				<button class="btn-primary" onclick={openCreateModal}>
					<IconPlus size={18} />
					Create First Webhook
				</button>
			</div>
		{:else}
			{#each webhooks as webhook (webhook.id)}
				<div class="webhook-card" class:inactive={!webhook.isActive}>
					<div class="webhook-header">
						<div class="webhook-info">
							<h3 class="webhook-name">{webhook.name}</h3>
							<a href={webhook.url} target="_blank" class="webhook-url">
								{webhook.url}
								<IconExternalLink size={14} />
							</a>
						</div>
						<div class="webhook-status">
							<button
								class="status-toggle"
								class:active={webhook.isActive}
								onclick={() => toggleWebhook(webhook)}
								title={webhook.isActive ? 'Click to disable' : 'Click to enable'}
							>
								{webhook.isActive ? 'Active' : 'Inactive'}
							</button>
						</div>
					</div>

					<div class="webhook-events">
						{#each webhook.events as event}
							<span class="event-badge">{event}</span>
						{/each}
					</div>

					<div class="webhook-stats">
						<div class="stat">
							<span class="stat-value">{webhook.successCount}</span>
							<span class="stat-label">Success</span>
						</div>
						<div class="stat">
							<span class="stat-value">{webhook.failureCount}</span>
							<span class="stat-label">Failed</span>
						</div>
						<div class="stat">
							<span class="stat-value">{getSuccessRate(webhook)}%</span>
							<span class="stat-label">Rate</span>
						</div>
						<div class="stat">
							<span class="stat-value">{formatDate(webhook.lastTriggeredAt)}</span>
							<span class="stat-label">Last Triggered</span>
						</div>
					</div>

					<div class="webhook-actions">
						<button class="btn-icon" onclick={() => openDeliveriesModal(webhook)} title="View Deliveries">
							<IconEye size={16} />
						</button>
						<button class="btn-icon" onclick={() => testWebhook(webhook)} title="Send Test">
							<IconRefresh size={16} />
						</button>
						<button class="btn-icon" onclick={() => openEditModal(webhook)} title="Edit">
							<IconEdit size={16} />
						</button>
						<button class="btn-icon danger" onclick={() => deleteWebhook(webhook)} title="Delete">
							<IconTrash size={16} />
						</button>
					</div>
				</div>
			{/each}
		{/if}
	</div>
</div>

<!-- Create/Edit Modal -->
{#if showCreateModal}
	<div
		class="modal-backdrop"
		role="presentation"
		onclick={() => showCreateModal = false}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showCreateModal = false)}
	>
		<div
			class="modal"
			role="dialog"
			aria-modal="true"
			aria-labelledby="webhook-modal-title"
			tabindex="-1"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
			in:scale={{ duration: 200 }}
		>
			<div class="modal-header">
				<h3 id="webhook-modal-title">{isEditing ? 'Edit Webhook' : 'Create Webhook'}</h3>
				<button class="btn-close" onclick={() => showCreateModal = false}>×</button>
			</div>
			<div class="modal-body">
				<div class="field-group">
					<label for="webhook-name" class="field-label required">Name</label>
					<input
						id="webhook-name"
						type="text"
						bind:value={formName}
						placeholder="My Webhook"
						class="field-input"
					/>
				</div>

				<div class="field-group">
					<label for="webhook-url" class="field-label required">URL</label>
					<input
						id="webhook-url"
						type="url"
						bind:value={formUrl}
						placeholder="https://api.example.com/webhook"
						class="field-input"
					/>
				</div>

				<div class="field-group">
					<span id="events-label" class="field-label required">Events</span>
					<div class="event-options" role="group" aria-labelledby="events-label">
						{#each eventOptions as option}
							<button
								type="button"
								class="event-option"
								class:selected={formEvents.includes(option.value)}
								onclick={() => toggleEvent(option.value)}
							>
								{option.label}
							</button>
						{/each}
					</div>
				</div>

				<div class="field-group">
					<label for="webhook-secret" class="field-label">Secret (for HMAC signing)</label>
					<div class="secret-input-group">
						<input
							id="webhook-secret"
							type="text"
							bind:value={formSecret}
							placeholder="Leave empty for no signing"
							class="field-input"
						/>
						<button type="button" class="btn-generate" onclick={generateSecret}>
							Generate
						</button>
					</div>
				</div>

				<div class="field-row">
					<div class="field-group">
						<label for="webhook-retry" class="field-label">Retry Count</label>
						<input
							id="webhook-retry"
							type="number"
							bind:value={formRetryCount}
							min="0"
							max="10"
							class="field-input"
						/>
					</div>
					<div class="field-group">
						<label for="webhook-timeout" class="field-label">Timeout (seconds)</label>
						<input
							id="webhook-timeout"
							type="number"
							bind:value={formTimeoutSeconds}
							min="5"
							max="60"
							class="field-input"
						/>
					</div>
				</div>

				<div class="field-group">
					<label class="field-checkbox">
						<input type="checkbox" bind:checked={formIsActive} />
						<span>Active</span>
					</label>
				</div>
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => showCreateModal = false}>
					Cancel
				</button>
				<button class="btn-primary" onclick={saveWebhook} disabled={isSaving}>
					{isSaving ? 'Saving...' : isEditing ? 'Update' : 'Create'}
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Deliveries Modal -->
{#if showDeliveriesModal && selectedWebhook}
	<div
		class="modal-backdrop"
		role="presentation"
		onclick={() => showDeliveriesModal = false}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showDeliveriesModal = false)}
	>
		<div
			class="modal modal-large"
			role="dialog"
			aria-modal="true"
			aria-labelledby="deliveries-modal-title"
			tabindex="-1"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
			in:scale={{ duration: 200 }}
		>
			<div class="modal-header">
				<h3 id="deliveries-modal-title">Delivery History - {selectedWebhook.name}</h3>
				<button class="btn-close" onclick={() => showDeliveriesModal = false}>×</button>
			</div>
			<div class="modal-body">
				{#if deliveries.length === 0}
					<div class="empty-deliveries">
						<p>No delivery history yet.</p>
					</div>
				{:else}
					<div class="deliveries-list">
						{#each deliveries as delivery (delivery.id)}
							<div class="delivery-item">
								<div class="delivery-status status-{getStatusColor(delivery.status)}">
									{#if delivery.status === 'delivered'}
										<IconCheck size={16} />
									{:else if delivery.status === 'failed'}
										<IconAlertTriangle size={16} />
									{:else}
										<IconClock size={16} />
									{/if}
								</div>
								<div class="delivery-info">
									<span class="delivery-event">{delivery.eventType}</span>
									<span class="delivery-date">{formatDate(delivery.createdAt)}</span>
									{#if delivery.responseStatus}
										<span class="delivery-response">
											HTTP {delivery.responseStatus} · {delivery.responseTimeMs}ms
										</span>
									{/if}
									{#if delivery.errorMessage}
										<span class="delivery-error">{delivery.errorMessage}</span>
									{/if}
								</div>
								<div class="delivery-actions">
									{#if delivery.status === 'failed'}
										<button class="btn-retry" onclick={() => retryDelivery(delivery)}>
											Retry
										</button>
									{/if}
								</div>
							</div>
						{/each}
					</div>
				{/if}
			</div>
		</div>
	</div>
{/if}

<style>
	.webhooks-page {
		padding: 1.5rem;
		max-width: 1200px;
		margin: 0 auto;
	}

	/* Header */
	.page-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 1.5rem;
	}

	.header-content {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.header-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 48px;
		height: 48px;
		background: linear-gradient(135deg, rgba(230, 184, 0, 0.2), rgba(230, 184, 0, 0.1));
		border-radius: 0.75rem;
		color: #e6b800;
	}

	.page-title {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0;
	}

	.page-subtitle {
		font-size: 0.875rem;
		color: #64748b;
		margin: 0.25rem 0 0 0;
	}

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: linear-gradient(135deg, #e6b800, #d4a600);
		border: none;
		border-radius: 0.5rem;
		color: #0f172a;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(230, 184, 0, 0.3);
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	/* Banners */
	.error-banner, .success-banner {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.25rem;
		border-radius: 0.5rem;
		margin-bottom: 1.5rem;
	}

	.error-banner {
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #f87171;
	}

	.success-banner {
		background: rgba(16, 185, 129, 0.1);
		border: 1px solid rgba(16, 185, 129, 0.3);
		color: #34d399;
	}

	.error-banner button {
		margin-left: auto;
		padding: 0.375rem 0.75rem;
		background: transparent;
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 0.375rem;
		color: #f87171;
		cursor: pointer;
	}

	/* Stats */
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.stat-card {
		display: flex;
		flex-direction: column;
		padding: 1rem;
		background: rgba(30, 41, 59, 0.3);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.75rem;
	}

	.stat-card .stat-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.stat-card .stat-label {
		font-size: 0.75rem;
		color: #64748b;
	}

	/* Webhooks List */
	.webhooks-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.loading-state, .empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem;
		background: rgba(30, 41, 59, 0.3);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.75rem;
		color: #64748b;
		text-align: center;
	}

	.empty-state h3 {
		font-size: 1.125rem;
		color: #f1f5f9;
		margin: 1rem 0 0.25rem 0;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(230, 184, 0, 0.2);
		border-top-color: #e6b800;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	/* Webhook Card */
	.webhook-card {
		padding: 1.25rem;
		background: rgba(30, 41, 59, 0.3);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.75rem;
		transition: border-color 0.15s;
	}

	.webhook-card:hover {
		border-color: rgba(230, 184, 0, 0.3);
	}

	.webhook-card.inactive {
		opacity: 0.6;
	}

	.webhook-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 0.75rem;
	}

	.webhook-name {
		font-size: 1.0625rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.webhook-url {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.8125rem;
		color: #64748b;
		text-decoration: none;
	}

	.webhook-url:hover {
		color: #e6b800;
	}

	.status-toggle {
		padding: 0.375rem 0.75rem;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 0.375rem;
		color: #f87171;
		font-size: 0.75rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.15s;
	}

	.status-toggle.active {
		background: rgba(16, 185, 129, 0.1);
		border-color: rgba(16, 185, 129, 0.3);
		color: #34d399;
	}

	.webhook-events {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		margin-bottom: 1rem;
	}

	.event-badge {
		padding: 0.25rem 0.5rem;
		background: rgba(230, 184, 0, 0.1);
		border-radius: 0.25rem;
		font-size: 0.6875rem;
		font-weight: 600;
		color: #e6b800;
	}

	.webhook-stats {
		display: flex;
		gap: 1.5rem;
		padding-top: 0.75rem;
		border-top: 1px solid rgba(51, 65, 85, 0.3);
		margin-bottom: 0.75rem;
	}

	.webhook-stats .stat {
		display: flex;
		flex-direction: column;
	}

	.webhook-stats .stat-value {
		font-size: 0.9375rem;
		font-weight: 600;
		color: #f1f5f9;
	}

	.webhook-stats .stat-label {
		font-size: 0.6875rem;
		color: #64748b;
	}

	.webhook-actions {
		display: flex;
		gap: 0.5rem;
		justify-content: flex-end;
	}

	.btn-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		background: rgba(51, 65, 85, 0.3);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.375rem;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-icon:hover {
		background: rgba(51, 65, 85, 0.5);
		color: #f1f5f9;
	}

	.btn-icon.danger:hover {
		background: rgba(239, 68, 68, 0.1);
		border-color: rgba(239, 68, 68, 0.3);
		color: #f87171;
	}

	/* Modal */
	.modal-backdrop {
		position: fixed;
		inset: 0;
		z-index: 1000;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.6);
		backdrop-filter: blur(4px);
	}

	.modal {
		width: 100%;
		max-width: 500px;
		max-height: 90vh;
		overflow-y: auto;
		background: #1e293b;
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.75rem;
		box-shadow: 0 25px 50px rgba(0, 0, 0, 0.5);
	}

	.modal-large {
		max-width: 700px;
	}

	.modal-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem 1.25rem;
		border-bottom: 1px solid rgba(51, 65, 85, 0.5);
	}

	.modal-header h3 {
		font-size: 1.0625rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.btn-close {
		background: transparent;
		border: none;
		color: #64748b;
		font-size: 1.5rem;
		cursor: pointer;
	}

	.modal-body {
		padding: 1.25rem;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1rem 1.25rem;
		border-top: 1px solid rgba(51, 65, 85, 0.5);
	}

	.btn-secondary {
		padding: 0.625rem 1rem;
		background: rgba(51, 65, 85, 0.3);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.5rem;
		color: #94a3b8;
		font-weight: 500;
		cursor: pointer;
	}

	.btn-secondary:hover {
		background: rgba(51, 65, 85, 0.5);
		color: #f1f5f9;
	}

	/* Form Fields */
	.field-group {
		margin-bottom: 1rem;
	}

	.field-label {
		display: block;
		font-size: 0.8125rem;
		font-weight: 600;
		color: #cbd5e1;
		margin-bottom: 0.5rem;
	}

	.field-label.required::after {
		content: ' *';
		color: #ef4444;
	}

	.field-input {
		width: 100%;
		padding: 0.625rem 0.875rem;
		background: rgba(0, 0, 0, 0.2);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.5rem;
		color: #f1f5f9;
		font-size: 0.9375rem;
	}

	.field-input:focus {
		outline: none;
		border-color: #e6b800;
	}

	.field-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	.secret-input-group {
		display: flex;
		gap: 0.5rem;
	}

	.btn-generate {
		padding: 0.625rem 0.875rem;
		background: rgba(51, 65, 85, 0.3);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.5rem;
		color: #94a3b8;
		font-size: 0.8125rem;
		cursor: pointer;
		white-space: nowrap;
	}

	.btn-generate:hover {
		background: rgba(51, 65, 85, 0.5);
		color: #f1f5f9;
	}

	.event-options {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.event-option {
		padding: 0.5rem 0.75rem;
		background: rgba(51, 65, 85, 0.3);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.375rem;
		color: #94a3b8;
		font-size: 0.8125rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.event-option:hover {
		background: rgba(51, 65, 85, 0.5);
	}

	.event-option.selected {
		background: rgba(230, 184, 0, 0.15);
		border-color: rgba(230, 184, 0, 0.3);
		color: #e6b800;
	}

	.field-checkbox {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.875rem;
		color: #f1f5f9;
		cursor: pointer;
	}

	.field-checkbox input {
		width: 1rem;
		height: 1rem;
		accent-color: #e6b800;
	}

	/* Deliveries */
	.empty-deliveries {
		text-align: center;
		padding: 2rem;
		color: #64748b;
	}

	.deliveries-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.delivery-item {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 0.75rem;
		background: rgba(0, 0, 0, 0.2);
		border-radius: 0.5rem;
	}

	.delivery-status {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		border-radius: 50%;
	}

	.delivery-status.status-green {
		background: rgba(16, 185, 129, 0.2);
		color: #34d399;
	}

	.delivery-status.status-red {
		background: rgba(239, 68, 68, 0.2);
		color: #f87171;
	}

	.delivery-status.status-yellow {
		background: rgba(245, 158, 11, 0.2);
		color: #fbbf24;
	}

	.delivery-status.status-blue {
		background: rgba(59, 130, 246, 0.2);
		color: #60a5fa;
	}

	.delivery-info {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.delivery-event {
		font-size: 0.875rem;
		font-weight: 600;
		color: #f1f5f9;
	}

	.delivery-date {
		font-size: 0.75rem;
		color: #64748b;
	}

	.delivery-response {
		font-size: 0.75rem;
		color: #94a3b8;
	}

	.delivery-error {
		font-size: 0.75rem;
		color: #f87171;
	}

	.btn-retry {
		padding: 0.375rem 0.75rem;
		background: rgba(51, 65, 85, 0.3);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.375rem;
		color: #94a3b8;
		font-size: 0.75rem;
		cursor: pointer;
	}

	.btn-retry:hover {
		background: rgba(51, 65, 85, 0.5);
		color: #f1f5f9;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}

		.webhook-stats {
			flex-wrap: wrap;
		}

		.field-row {
			grid-template-columns: 1fr;
		}
	}
</style>

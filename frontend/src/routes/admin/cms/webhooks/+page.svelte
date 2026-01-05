<!--
	CMS Webhooks - Apple ICT 11+ Principal Engineer Grade
	10/10 Integration Configuration System
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import { fly, scale, fade } from 'svelte/transition';
	import { backOut } from 'svelte/easing';
	import {
		IconWebhook,
		IconArrowLeft,
		IconPlus,
		IconTrash,
		IconEdit,
		IconCheck,
		IconX,
		IconRefresh,
		IconExternalLink,
		IconShield,
		IconActivity,
		IconAlertTriangle,
		IconClock,
		IconSend
	} from '$lib/icons';

	let mounted = false;
	let isLoading = true;
	let webhooks: any[] = [];
	let showCreateModal = false;
	let editingWebhook: any = null;

	// Form state
	let formData = {
		name: '',
		url: '',
		secret: '',
		events: [] as string[],
		is_active: true
	};

	const availableEvents = [
		'content.created',
		'content.updated',
		'content.deleted',
		'content.published',
		'content.unpublished',
		'workflow.transition',
		'user.registered'
	];

	async function fetchWebhooks() {
		isLoading = true;
		try {
			const response = await fetch('/api/admin/cms/webhooks', {
				credentials: 'include'
			});
			if (response.ok) {
				webhooks = await response.json();
			}
		} catch (e) {
			console.error('Failed to fetch webhooks:', e);
		} finally {
			isLoading = false;
		}
	}

	async function createWebhook() {
		try {
			const response = await fetch('/api/admin/cms/webhooks', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				credentials: 'include',
				body: JSON.stringify(formData)
			});
			if (response.ok) {
				fetchWebhooks();
				closeModal();
			}
		} catch (e) {
			console.error('Failed to create webhook:', e);
		}
	}

	async function updateWebhook() {
		if (!editingWebhook) return;
		try {
			const response = await fetch(`/api/admin/cms/webhooks/${editingWebhook.id}`, {
				method: 'PUT',
				headers: { 'Content-Type': 'application/json' },
				credentials: 'include',
				body: JSON.stringify(formData)
			});
			if (response.ok) {
				fetchWebhooks();
				closeModal();
			}
		} catch (e) {
			console.error('Failed to update webhook:', e);
		}
	}

	async function deleteWebhook(id: number) {
		if (!confirm('Are you sure you want to delete this webhook?')) return;
		try {
			const response = await fetch(`/api/admin/cms/webhooks/${id}`, {
				method: 'DELETE',
				credentials: 'include'
			});
			if (response.ok) {
				fetchWebhooks();
			}
		} catch (e) {
			console.error('Failed to delete webhook:', e);
		}
	}

	async function testWebhook(id: number) {
		try {
			const response = await fetch(`/api/admin/cms/webhooks/${id}/test`, {
				method: 'POST',
				credentials: 'include'
			});
			if (response.ok) {
				alert('Test webhook sent successfully!');
			}
		} catch (e) {
			console.error('Failed to test webhook:', e);
		}
	}

	function openCreateModal() {
		formData = { name: '', url: '', secret: '', events: [], is_active: true };
		editingWebhook = null;
		showCreateModal = true;
	}

	function openEditModal(webhook: any) {
		formData = {
			name: webhook.name,
			url: webhook.url,
			secret: '',
			events: webhook.events || [],
			is_active: webhook.is_active
		};
		editingWebhook = webhook;
		showCreateModal = true;
	}

	function closeModal() {
		showCreateModal = false;
		editingWebhook = null;
	}

	function toggleEvent(event: string) {
		if (formData.events.includes(event)) {
			formData.events = formData.events.filter(e => e !== event);
		} else {
			formData.events = [...formData.events, event];
		}
	}

	function formatDate(dateStr: string): string {
		return new Date(dateStr).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	onMount(() => {
		mounted = true;
		fetchWebhooks();
	});
</script>

<div class="webhooks-page" class:mounted>
	<!-- Header -->
	<header class="page-header" in:fly={{ y: -20, duration: 500 }}>
		<div class="header-left">
			<a href="/admin/cms" class="back-link">
				<IconArrowLeft size={18} />
				<span>Back to CMS</span>
			</a>
			<div class="header-title">
				<div class="header-icon">
					<IconWebhook size={24} />
				</div>
				<div>
					<h1>Webhooks</h1>
					<p>Configure external integrations and notifications</p>
				</div>
			</div>
		</div>

		<button class="btn-create" onclick={openCreateModal}>
			<IconPlus size={18} />
			Create Webhook
		</button>
	</header>

	<!-- Webhooks List -->
	<section class="webhooks-container" in:fly={{ y: 20, duration: 500, delay: 100 }}>
		{#if isLoading}
			<div class="loading-state">
				<div class="spinner"></div>
				<p>Loading webhooks...</p>
			</div>
		{:else if webhooks.length === 0}
			<div class="empty-state" in:scale={{ duration: 400 }}>
				<div class="empty-icon">
					<IconWebhook size={48} />
				</div>
				<h3>No webhooks configured</h3>
				<p>Create your first webhook to start receiving notifications</p>
				<button class="btn-empty-action" onclick={openCreateModal}>
					<IconPlus size={16} />
					Create Webhook
				</button>
			</div>
		{:else}
			<div class="webhooks-grid">
				{#each webhooks as webhook, i}
					<div
						class="webhook-card"
						class:inactive={!webhook.is_active}
						in:fly={{ y: 20, duration: 400, delay: i * 50 }}
					>
						<div class="card-header">
							<div class="webhook-status" class:active={webhook.is_active}>
								<span class="status-dot"></span>
								{webhook.is_active ? 'Active' : 'Inactive'}
							</div>
							<div class="card-actions">
								<button class="action-btn" title="Test" onclick={() => testWebhook(webhook.id)}>
									<IconSend size={16} />
								</button>
								<button class="action-btn" title="Edit" onclick={() => openEditModal(webhook)}>
									<IconEdit size={16} />
								</button>
								<button class="action-btn delete" title="Delete" onclick={() => deleteWebhook(webhook.id)}>
									<IconTrash size={16} />
								</button>
							</div>
						</div>

						<div class="card-body">
							<h3 class="webhook-name">{webhook.name}</h3>
							<div class="webhook-url">
								<IconExternalLink size={14} />
								<span>{webhook.url}</span>
							</div>
						</div>

						<div class="webhook-events">
							{#each (webhook.events || []).slice(0, 3) as event}
								<span class="event-tag">{event}</span>
							{/each}
							{#if (webhook.events || []).length > 3}
								<span class="event-more">+{webhook.events.length - 3} more</span>
							{/if}
						</div>

						<div class="card-footer">
							<div class="webhook-stats">
								<div class="stat">
									<IconCheck size={14} />
									<span>{webhook.success_count || 0}</span>
								</div>
								<div class="stat failed">
									<IconX size={14} />
									<span>{webhook.failure_count || 0}</span>
								</div>
							</div>
							{#if webhook.last_triggered_at}
								<div class="last-triggered">
									<IconClock size={14} />
									<span>{formatDate(webhook.last_triggered_at)}</span>
								</div>
							{/if}
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</section>

	<!-- Create/Edit Modal -->
	{#if showCreateModal}
		<div class="modal-overlay" onclick={closeModal} transition:fade={{ duration: 200 }}></div>
		<div class="modal" in:fly={{ y: 50, duration: 400 }}>
			<div class="modal-header">
				<h2>{editingWebhook ? 'Edit Webhook' : 'Create Webhook'}</h2>
				<button class="close-btn" onclick={closeModal}>
					<IconX size={18} />
				</button>
			</div>

			<form class="modal-body" onsubmit={(e) => { e.preventDefault(); editingWebhook ? updateWebhook() : createWebhook(); }}>
				<div class="form-group">
					<label for="name">Name</label>
					<input
						id="name"
						type="text"
						placeholder="My Webhook"
						bind:value={formData.name}
						required
					/>
				</div>

				<div class="form-group">
					<label for="url">URL</label>
					<input
						id="url"
						type="url"
						placeholder="https://example.com/webhook"
						bind:value={formData.url}
						required
					/>
				</div>

				<div class="form-group">
					<label for="secret">
						Secret Key
						{#if editingWebhook}
							<span class="optional">(leave blank to keep current)</span>
						{/if}
					</label>
					<input
						id="secret"
						type="password"
						placeholder="Enter secret key"
						bind:value={formData.secret}
					/>
					<p class="form-hint">Used to sign webhook payloads with HMAC-SHA256</p>
				</div>

				<div class="form-group">
					<label>Events</label>
					<div class="events-grid">
						{#each availableEvents as event}
							<button
								type="button"
								class="event-toggle"
								class:selected={formData.events.includes(event)}
								onclick={() => toggleEvent(event)}
							>
								{#if formData.events.includes(event)}
									<IconCheck size={14} />
								{/if}
								{event}
							</button>
						{/each}
					</div>
				</div>

				<div class="form-group toggle-group">
					<label for="active">Active</label>
					<button
						type="button"
						class="toggle-switch"
						class:active={formData.is_active}
						onclick={() => formData.is_active = !formData.is_active}
					>
						<span class="toggle-knob"></span>
					</button>
				</div>

				<div class="modal-actions">
					<button type="button" class="btn-cancel" onclick={closeModal}>
						Cancel
					</button>
					<button type="submit" class="btn-submit">
						{editingWebhook ? 'Save Changes' : 'Create Webhook'}
					</button>
				</div>
			</form>
		</div>
	{/if}
</div>

<style>
	.webhooks-page {
		max-width: 1400px;
		margin: 0 auto;
		padding: 0 1.5rem 3rem;
		opacity: 0;
		transform: translateY(10px);
		transition: all 0.5s ease;
	}

	.webhooks-page.mounted {
		opacity: 1;
		transform: translateY(0);
	}

	/* Header */
	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		padding: 1.5rem 0 2rem;
		flex-wrap: wrap;
		gap: 1.5rem;
	}

	.back-link {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.85rem;
		font-weight: 500;
		color: #6366f1;
		text-decoration: none;
		margin-bottom: 1rem;
	}

	.header-title {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.header-icon {
		width: 56px;
		height: 56px;
		border-radius: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, rgba(139, 92, 246, 0.12) 0%, rgba(139, 92, 246, 0.06) 100%);
		color: #7c3aed;
	}

	.header-title h1 {
		font-size: 1.75rem;
		font-weight: 800;
		color: #1e293b;
		margin: 0;
	}

	.header-title p {
		font-size: 0.9rem;
		color: #64748b;
		margin: 0.25rem 0 0 0;
	}

	.btn-create {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.875rem 1.5rem;
		background: linear-gradient(135deg, #6366f1 0%, #4f46e5 100%);
		border: none;
		border-radius: 12px;
		font-size: 0.9rem;
		font-weight: 600;
		color: #ffffff;
		cursor: pointer;
		transition: all 0.25s;
	}

	.btn-create:hover {
		transform: translateY(-2px);
		box-shadow: 0 8px 25px rgba(99, 102, 241, 0.3);
	}

	/* Webhooks Container */
	.webhooks-container {
		background: #ffffff;
		border: 1px solid rgba(0, 0, 0, 0.06);
		border-radius: 20px;
		padding: 1.5rem;
		min-height: 400px;
	}

	.webhooks-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(360px, 1fr));
		gap: 1.25rem;
	}

	.webhook-card {
		background: linear-gradient(135deg, rgba(248, 250, 252, 0.9) 0%, rgba(255, 255, 255, 0.7) 100%);
		border: 1px solid rgba(0, 0, 0, 0.05);
		border-radius: 18px;
		padding: 1.5rem;
		transition: all 0.25s ease;
	}

	.webhook-card:hover {
		border-color: rgba(139, 92, 246, 0.25);
		box-shadow: 0 8px 30px rgba(0, 0, 0, 0.06);
	}

	.webhook-card.inactive {
		opacity: 0.6;
	}

	.card-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.webhook-status {
		display: flex;
		align-items: center;
		gap: 0.4rem;
		font-size: 0.75rem;
		font-weight: 600;
		color: #94a3b8;
	}

	.status-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: #cbd5e1;
	}

	.webhook-status.active {
		color: #059669;
	}

	.webhook-status.active .status-dot {
		background: #10b981;
		box-shadow: 0 0 8px rgba(16, 185, 129, 0.4);
	}

	.card-actions {
		display: flex;
		gap: 0.4rem;
	}

	.action-btn {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #ffffff;
		border: 1px solid rgba(0, 0, 0, 0.08);
		border-radius: 8px;
		color: #64748b;
		cursor: pointer;
		transition: all 0.2s;
	}

	.action-btn:hover {
		background: #f8fafc;
		color: #6366f1;
		border-color: rgba(99, 102, 241, 0.3);
	}

	.action-btn.delete:hover {
		color: #dc2626;
		border-color: rgba(239, 68, 68, 0.3);
	}

	.card-body {
		margin-bottom: 1rem;
	}

	.webhook-name {
		font-size: 1.1rem;
		font-weight: 700;
		color: #1e293b;
		margin: 0 0 0.5rem 0;
	}

	.webhook-url {
		display: flex;
		align-items: center;
		gap: 0.4rem;
		font-size: 0.8rem;
		color: #64748b;
		word-break: break-all;
	}

	.webhook-events {
		display: flex;
		flex-wrap: wrap;
		gap: 0.4rem;
		margin-bottom: 1rem;
	}

	.event-tag {
		padding: 0.3rem 0.6rem;
		background: rgba(139, 92, 246, 0.1);
		border-radius: 6px;
		font-size: 0.7rem;
		font-weight: 500;
		color: #7c3aed;
	}

	.event-more {
		padding: 0.3rem 0.6rem;
		font-size: 0.7rem;
		color: #94a3b8;
	}

	.card-footer {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding-top: 1rem;
		border-top: 1px solid rgba(0, 0, 0, 0.04);
	}

	.webhook-stats {
		display: flex;
		gap: 1rem;
	}

	.stat {
		display: flex;
		align-items: center;
		gap: 0.3rem;
		font-size: 0.8rem;
		color: #059669;
	}

	.stat.failed {
		color: #dc2626;
	}

	.last-triggered {
		display: flex;
		align-items: center;
		gap: 0.4rem;
		font-size: 0.75rem;
		color: #94a3b8;
	}

	/* Empty & Loading */
	.loading-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem;
		text-align: center;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid #f1f5f9;
		border-top-color: #6366f1;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	.empty-icon {
		width: 80px;
		height: 80px;
		border-radius: 20px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, rgba(139, 92, 246, 0.1) 0%, rgba(139, 92, 246, 0.05) 100%);
		color: #a78bfa;
		margin-bottom: 1rem;
	}

	.empty-state h3 {
		font-size: 1.1rem;
		font-weight: 700;
		color: #475569;
		margin: 0 0 0.5rem 0;
	}

	.empty-state p {
		font-size: 0.9rem;
		color: #94a3b8;
		margin: 0 0 1.5rem 0;
	}

	.btn-empty-action {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: linear-gradient(135deg, #6366f1 0%, #4f46e5 100%);
		border: none;
		border-radius: 10px;
		font-size: 0.85rem;
		font-weight: 600;
		color: #ffffff;
		cursor: pointer;
	}

	/* Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.4);
		backdrop-filter: blur(4px);
		z-index: 99;
	}

	.modal {
		position: fixed;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 90%;
		max-width: 520px;
		max-height: 90vh;
		background: #ffffff;
		border-radius: 24px;
		box-shadow: 0 25px 80px rgba(0, 0, 0, 0.15);
		z-index: 100;
		overflow: hidden;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem;
		border-bottom: 1px solid rgba(0, 0, 0, 0.06);
	}

	.modal-header h2 {
		font-size: 1.25rem;
		font-weight: 700;
		color: #1e293b;
		margin: 0;
	}

	.close-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #f8fafc;
		border: none;
		border-radius: 10px;
		color: #64748b;
		cursor: pointer;
	}

	.modal-body {
		padding: 1.5rem;
		max-height: calc(90vh - 140px);
		overflow-y: auto;
	}

	.form-group {
		margin-bottom: 1.25rem;
	}

	.form-group label {
		display: block;
		font-size: 0.85rem;
		font-weight: 600;
		color: #334155;
		margin-bottom: 0.5rem;
	}

	.form-group .optional {
		font-weight: 400;
		color: #94a3b8;
		font-size: 0.8rem;
	}

	.form-group input {
		width: 100%;
		padding: 0.875rem 1rem;
		background: #f8fafc;
		border: 1px solid rgba(0, 0, 0, 0.08);
		border-radius: 12px;
		font-size: 0.9rem;
		color: #1e293b;
		transition: all 0.2s;
	}

	.form-group input:focus {
		outline: none;
		border-color: #6366f1;
		box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.15);
	}

	.form-hint {
		font-size: 0.75rem;
		color: #94a3b8;
		margin: 0.4rem 0 0 0;
	}

	.events-grid {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.event-toggle {
		display: flex;
		align-items: center;
		gap: 0.4rem;
		padding: 0.5rem 0.875rem;
		background: #f8fafc;
		border: 1px solid rgba(0, 0, 0, 0.08);
		border-radius: 8px;
		font-size: 0.8rem;
		color: #64748b;
		cursor: pointer;
		transition: all 0.2s;
	}

	.event-toggle:hover {
		background: #f1f5f9;
	}

	.event-toggle.selected {
		background: rgba(99, 102, 241, 0.1);
		border-color: rgba(99, 102, 241, 0.3);
		color: #6366f1;
	}

	.toggle-group {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.toggle-switch {
		width: 48px;
		height: 28px;
		background: #e2e8f0;
		border: none;
		border-radius: 14px;
		cursor: pointer;
		position: relative;
		transition: background 0.2s;
	}

	.toggle-switch.active {
		background: #6366f1;
	}

	.toggle-knob {
		position: absolute;
		top: 3px;
		left: 3px;
		width: 22px;
		height: 22px;
		background: #ffffff;
		border-radius: 50%;
		box-shadow: 0 2px 6px rgba(0, 0, 0, 0.15);
		transition: transform 0.2s;
	}

	.toggle-switch.active .toggle-knob {
		transform: translateX(20px);
	}

	.modal-actions {
		display: flex;
		gap: 0.75rem;
		margin-top: 1.5rem;
	}

	.btn-cancel {
		flex: 1;
		padding: 0.875rem 1.25rem;
		background: #f1f5f9;
		border: none;
		border-radius: 12px;
		font-size: 0.9rem;
		font-weight: 600;
		color: #64748b;
		cursor: pointer;
	}

	.btn-submit {
		flex: 1;
		padding: 0.875rem 1.25rem;
		background: linear-gradient(135deg, #6366f1 0%, #4f46e5 100%);
		border: none;
		border-radius: 12px;
		font-size: 0.9rem;
		font-weight: 600;
		color: #ffffff;
		cursor: pointer;
	}

	@media (max-width: 768px) {
		.webhooks-grid {
			grid-template-columns: 1fr;
		}
	}
</style>

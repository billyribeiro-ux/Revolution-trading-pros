<!--
	/admin/crm/webhooks/new - Create New Webhook
	Apple Principal Engineer ICT 7 Grade - January 2026

	Features:
	- Complete webhook creation form
	- Event subscription selection
	- Custom headers configuration
	- URL validation
	- Secret key generation
	- Full Svelte 5 $state/$derived/$effect reactivity
-->

<script lang="ts">
	import { goto } from '$app/navigation';
	import IconWebhook from '@tabler/icons-svelte/icons/webhook';
	import IconArrowLeft from '@tabler/icons-svelte/icons/arrow-left';
	import IconPlus from '@tabler/icons-svelte/icons/plus';
	import IconTrash from '@tabler/icons-svelte/icons/trash';
	import IconRefresh from '@tabler/icons-svelte/icons/refresh';
	import IconCheck from '@tabler/icons-svelte/icons/check';
	import IconAlertCircle from '@tabler/icons-svelte/icons/alert-circle';
	import IconKey from '@tabler/icons-svelte/icons/key';
	import IconX from '@tabler/icons-svelte/icons/x';
	import { onMount } from 'svelte';
	import { crmAPI } from '$lib/api/crm';
	import type { WebhookEvent } from '$lib/crm/types';

	// =====================================================
	// STATE MANAGEMENT - Svelte 5 Runes
	// =====================================================

	let name = $state('');
	let url = $state('');
	let secret = $state('');
	let isActive = $state(true);
	let selectedEvents = $state<Set<WebhookEvent>>(new Set());
	let customHeaders = $state<Array<{ key: string; value: string }>>([]);

	let availableEvents = $state<Record<string, string>>({});
	let isLoadingEvents = $state(true);
	let isSaving = $state(false);
	let error = $state('');
	let fieldErrors = $state<Record<string, string>>({});

	// Toast notification state
	let toasts = $state<Array<{ id: string; type: 'success' | 'error' | 'info'; message: string }>>(
		[]
	);

	// =====================================================
	// DERIVED STATE
	// =====================================================

	let isFormValid = $derived(
		name.trim().length > 0 && url.trim().length > 0 && isValidUrl(url) && selectedEvents.size > 0
	);

	let selectedCount = $derived(selectedEvents.size);
	let totalEvents = $derived(Object.keys(availableEvents).length);

	// =====================================================
	// API FUNCTIONS
	// =====================================================

	async function loadEvents() {
		isLoadingEvents = true;
		try {
			availableEvents = await crmAPI.getWebhookEvents();
		} catch (err) {
			showToast('error', 'Failed to load webhook events');
			// Provide default events as fallback
			availableEvents = {
				contact_created: 'Contact Created',
				contact_updated: 'Contact Updated',
				contact_deleted: 'Contact Deleted',
				tag_applied: 'Tag Applied',
				tag_removed: 'Tag Removed',
				list_applied: 'List Applied',
				list_removed: 'List Removed',
				email_opened: 'Email Opened',
				email_clicked: 'Email Clicked',
				contact_unsubscribed: 'Contact Unsubscribed',
				deal_created: 'Deal Created',
				deal_won: 'Deal Won',
				deal_lost: 'Deal Lost'
			};
		} finally {
			isLoadingEvents = false;
		}
	}

	async function handleSubmit() {
		// Reset errors
		fieldErrors = {};
		error = '';

		// Validate fields
		if (!name.trim()) {
			fieldErrors.name = 'Webhook name is required';
		}
		if (!url.trim()) {
			fieldErrors.url = 'Webhook URL is required';
		} else if (!isValidUrl(url)) {
			fieldErrors.url = 'Please enter a valid URL (must start with https://)';
		}
		if (selectedEvents.size === 0) {
			fieldErrors.events = 'Please select at least one event';
		}

		if (Object.keys(fieldErrors).length > 0) {
			return;
		}

		isSaving = true;

		try {
			// Build headers object
			const headers: Record<string, string> = {};
			customHeaders.forEach((h) => {
				if (h.key.trim() && h.value.trim()) {
					headers[h.key.trim()] = h.value.trim();
				}
			});

			await crmAPI.createWebhook({
				name: name.trim(),
				url: url.trim(),
				secret: secret.trim() || undefined,
				is_active: isActive,
				events: Array.from(selectedEvents),
				headers: Object.keys(headers).length > 0 ? headers : undefined
			});

			showToast('success', 'Webhook created successfully');
			setTimeout(() => {
				goto('/admin/crm/webhooks');
			}, 1000);
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to create webhook';
			showToast('error', error);
		} finally {
			isSaving = false;
		}
	}

	// =====================================================
	// HELPER FUNCTIONS
	// =====================================================

	function isValidUrl(urlString: string): boolean {
		try {
			const url = new URL(urlString);
			return url.protocol === 'https:' || url.protocol === 'http:';
		} catch {
			return false;
		}
	}

	function generateSecret() {
		const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
		let result = '';
		for (let i = 0; i < 32; i++) {
			result += chars.charAt(Math.floor(Math.random() * chars.length));
		}
		secret = result;
	}

	function toggleEvent(event: WebhookEvent) {
		const newSet = new Set(selectedEvents);
		if (newSet.has(event)) {
			newSet.delete(event);
		} else {
			newSet.add(event);
		}
		selectedEvents = newSet;
	}

	function selectAllEvents() {
		selectedEvents = new Set(Object.keys(availableEvents) as WebhookEvent[]);
	}

	function clearAllEvents() {
		selectedEvents = new Set();
	}

	function addHeader() {
		customHeaders = [...customHeaders, { key: '', value: '' }];
	}

	function removeHeader(index: number) {
		customHeaders = customHeaders.filter((_, i) => i !== index);
	}

	function updateHeaderKey(index: number, key: string) {
		customHeaders = customHeaders.map((h, i) => (i === index ? { ...h, key } : h));
	}

	function updateHeaderValue(index: number, value: string) {
		customHeaders = customHeaders.map((h, i) => (i === index ? { ...h, value } : h));
	}

	function formatEventName(event: string): string {
		return event.replace(/_/g, ' ').replace(/\b\w/g, (c) => c.toUpperCase());
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
	// LIFECYCLE
	// =====================================================

	onMount(() => {
		loadEvents();
	});
</script>

<svelte:head>
	<title>Create Webhook - FluentCRM Pro</title>
</svelte:head>

<div class="webhook-form-page">
	<!-- Header -->
	<header class="page-header">
		<div class="header-left">
			<a href="/admin/crm/webhooks" class="back-link">
				<IconArrowLeft size={20} />
				<span>Back to Webhooks</span>
			</a>
			<h1>Create New Webhook</h1>
			<p class="page-description">
				Configure a webhook to send real-time notifications to external services
			</p>
		</div>
	</header>

	<form
		class="webhook-form"
		onsubmit={(e) => {
			e.preventDefault();
			handleSubmit();
		}}
	>
		<!-- Basic Info Section -->
		<section class="form-section">
			<h2 class="section-title">Basic Information</h2>

			<div class="form-group">
				<label for="name">
					Webhook Name <span class="required">*</span>
				</label>
				<input
					type="text"
					id="name"
					bind:value={name}
					placeholder="e.g., Slack Notifications, Zapier Integration"
					class:error={fieldErrors.name}
				/>
				{#if fieldErrors.name}
					<span class="field-error">{fieldErrors.name}</span>
				{/if}
			</div>

			<div class="form-group">
				<label for="url">
					Webhook URL <span class="required">*</span>
				</label>
				<input
					type="url"
					id="url"
					bind:value={url}
					placeholder="https://api.example.com/webhook"
					class:error={fieldErrors.url}
				/>
				{#if fieldErrors.url}
					<span class="field-error">{fieldErrors.url}</span>
				{:else}
					<span class="field-hint">The URL that will receive POST requests when events occur</span>
				{/if}
			</div>

			<div class="form-group">
				<label for="secret"> Secret Key (Optional) </label>
				<div class="input-with-action">
					<input
						type="text"
						id="secret"
						bind:value={secret}
						placeholder="Used to sign webhook payloads"
					/>
					<button
						type="button"
						class="btn-generate"
						onclick={generateSecret}
						title="Generate Secret"
					>
						<IconKey size={16} />
						Generate
					</button>
				</div>
				<span class="field-hint">Use this to verify webhook authenticity on your server</span>
			</div>

			<div class="form-group">
				<label class="toggle-label">
					<input id="page-isactive" name="page-isactive" type="checkbox" bind:checked={isActive} class="toggle-input" />
					<span class="toggle-switch"></span>
					<span class="toggle-text">Active</span>
				</label>
				<span class="field-hint">Inactive webhooks will not receive any events</span>
			</div>
		</section>

		<!-- Events Section -->
		<section class="form-section">
			<div class="section-header">
				<h2 class="section-title">Event Subscriptions</h2>
				<div class="section-actions">
					<button type="button" class="btn-text" onclick={selectAllEvents}>Select All</button>
					<button type="button" class="btn-text" onclick={clearAllEvents}>Clear All</button>
				</div>
			</div>

			{#if fieldErrors.events}
				<div class="field-error section-error">{fieldErrors.events}</div>
			{/if}

			<p class="section-description">
				Select the events that will trigger this webhook ({selectedCount} of {totalEvents} selected)
			</p>

			{#if isLoadingEvents}
				<div class="events-loading">
					<div class="spinner"></div>
					<span>Loading events...</span>
				</div>
			{:else}
				<div class="events-grid">
					{#each Object.entries(availableEvents) as [event, label]}
						<button
							type="button"
							class="event-chip"
							class:selected={selectedEvents.has(event as WebhookEvent)}
							onclick={() => toggleEvent(event as WebhookEvent)}
						>
							{#if selectedEvents.has(event as WebhookEvent)}
								<IconCheck size={14} />
							{/if}
							<span>{label || formatEventName(event)}</span>
						</button>
					{/each}
				</div>
			{/if}
		</section>

		<!-- Custom Headers Section -->
		<section class="form-section">
			<div class="section-header">
				<h2 class="section-title">Custom Headers (Optional)</h2>
				<button type="button" class="btn-add-header" onclick={addHeader}>
					<IconPlus size={16} />
					Add Header
				</button>
			</div>
			<p class="section-description">
				Add custom HTTP headers to include with each webhook request
			</p>

			{#if customHeaders.length > 0}
				<div class="headers-list">
					{#each customHeaders as header, index}
						<div class="header-row">
							<input
								id="page-header-name" name="page-header-name" type="text"
								placeholder="Header Name"
								value={header.key}
								oninput={(e) => updateHeaderKey(index, e.currentTarget.value)}
							/>
							<input
								id="page-header-value" name="page-header-value" type="text"
								placeholder="Header Value"
								value={header.value}
								oninput={(e) => updateHeaderValue(index, e.currentTarget.value)}
							/>
							<button
								type="button"
								class="btn-remove-header"
								onclick={() => removeHeader(index)}
								title="Remove Header"
							>
								<IconTrash size={16} />
							</button>
						</div>
					{/each}
				</div>
			{:else}
				<div class="no-headers">No custom headers configured</div>
			{/if}
		</section>

		<!-- Form Actions -->
		<div class="form-actions">
			<a href="/admin/crm/webhooks" class="btn-cancel">Cancel</a>
			<button type="submit" class="btn-submit" disabled={isSaving || !isFormValid}>
				{#if isSaving}
					<IconRefresh size={18} class="spinning" />
					Creating...
				{:else}
					<IconWebhook size={18} />
					Create Webhook
				{/if}
			</button>
		</div>
	</form>
</div>

<!-- Toast Notifications -->
{#if toasts.length > 0}
	<div class="toast-container" role="region" aria-label="Notifications">
		{#each toasts as toast (toast.id)}
			<div class="toast toast-{toast.type}" role="alert">
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
				<button class="toast-dismiss" onclick={() => dismissToast(toast.id)}>
					<IconX size={16} />
				</button>
			</div>
		{/each}
	</div>
{/if}

<style>
	.webhook-form-page {
		max-width: 900px;
	}

	/* =====================================================
	   Header
	   ===================================================== */
	.page-header {
		margin-bottom: 2rem;
	}

	.back-link {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		color: #94a3b8;
		text-decoration: none;
		font-size: 0.9rem;
		margin-bottom: 1rem;
		transition: color 0.2s;
	}

	.back-link:hover {
		color: #c7d2fe;
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

	/* =====================================================
	   Form Sections
	   ===================================================== */
	.webhook-form {
		display: flex;
		flex-direction: column;
		gap: 2rem;
	}

	.form-section {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 14px;
		padding: 1.5rem;
	}

	.section-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.5rem;
	}

	.section-title {
		font-size: 1.1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 0.5rem 0;
	}

	.section-header .section-title {
		margin: 0;
	}

	.section-description {
		font-size: 0.9rem;
		color: #64748b;
		margin: 0 0 1rem 0;
	}

	.section-actions {
		display: flex;
		gap: 0.75rem;
	}

	.btn-text {
		background: none;
		border: none;
		color: #818cf8;
		font-size: 0.85rem;
		cursor: pointer;
		padding: 0.25rem 0.5rem;
		transition: color 0.2s;
	}

	.btn-text:hover {
		color: #c7d2fe;
	}

	.section-error {
		margin-bottom: 1rem;
	}

	/* =====================================================
	   Form Groups
	   ===================================================== */
	.form-group {
		margin-bottom: 1.25rem;
	}

	.form-group:last-child {
		margin-bottom: 0;
	}

	.form-group label {
		display: block;
		font-size: 0.9rem;
		font-weight: 500;
		color: #e2e8f0;
		margin-bottom: 0.5rem;
	}

	.required {
		color: #f87171;
	}

	.form-group input[type='text'],
	.form-group input[type='url'] {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 10px;
		color: #e2e8f0;
		font-size: 0.9rem;
		transition: all 0.2s;
	}

	.form-group input:focus {
		outline: none;
		border-color: rgba(99, 102, 241, 0.5);
		box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
	}

	.form-group input.error {
		border-color: rgba(239, 68, 68, 0.5);
	}

	.form-group input::placeholder {
		color: #64748b;
	}

	.field-error {
		display: block;
		color: #f87171;
		font-size: 0.8rem;
		margin-top: 0.35rem;
	}

	.field-hint {
		display: block;
		color: #64748b;
		font-size: 0.8rem;
		margin-top: 0.35rem;
	}

	.input-with-action {
		display: flex;
		gap: 0.75rem;
	}

	.input-with-action input {
		flex: 1;
	}

	.btn-generate {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		background: rgba(99, 102, 241, 0.2);
		border: 1px solid rgba(99, 102, 241, 0.3);
		border-radius: 10px;
		color: #c7d2fe;
		font-size: 0.85rem;
		cursor: pointer;
		transition: all 0.2s;
		white-space: nowrap;
	}

	.btn-generate:hover {
		background: rgba(99, 102, 241, 0.3);
	}

	/* =====================================================
	   Toggle Switch
	   ===================================================== */
	.toggle-label {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		cursor: pointer;
	}

	.toggle-input {
		display: none;
	}

	.toggle-switch {
		position: relative;
		width: 44px;
		height: 24px;
		background: rgba(100, 116, 139, 0.3);
		border-radius: 12px;
		transition: background 0.2s;
	}

	.toggle-switch::after {
		content: '';
		position: absolute;
		top: 2px;
		left: 2px;
		width: 20px;
		height: 20px;
		background: #94a3b8;
		border-radius: 50%;
		transition: all 0.2s;
	}

	.toggle-input:checked + .toggle-switch {
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
	}

	.toggle-input:checked + .toggle-switch::after {
		left: 22px;
		background: white;
	}

	.toggle-text {
		color: #e2e8f0;
		font-weight: 500;
	}

	/* =====================================================
	   Events Grid
	   ===================================================== */
	.events-loading {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		color: #64748b;
		padding: 1rem;
	}

	.events-grid {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.event-chip {
		display: flex;
		align-items: center;
		gap: 0.35rem;
		padding: 0.5rem 0.875rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 9999px;
		color: #94a3b8;
		font-size: 0.8rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.event-chip:hover {
		border-color: rgba(99, 102, 241, 0.4);
		color: #c7d2fe;
	}

	.event-chip.selected {
		background: rgba(99, 102, 241, 0.2);
		border-color: rgba(99, 102, 241, 0.5);
		color: #c7d2fe;
	}

	/* =====================================================
	   Custom Headers
	   ===================================================== */
	.btn-add-header {
		display: flex;
		align-items: center;
		gap: 0.35rem;
		padding: 0.5rem 0.75rem;
		background: transparent;
		border: 1px solid rgba(99, 102, 241, 0.3);
		border-radius: 8px;
		color: #818cf8;
		font-size: 0.85rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-add-header:hover {
		background: rgba(99, 102, 241, 0.1);
	}

	.headers-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.header-row {
		display: flex;
		gap: 0.75rem;
	}

	.header-row input {
		flex: 1;
		padding: 0.65rem 0.875rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		color: #e2e8f0;
		font-size: 0.85rem;
		transition: all 0.2s;
	}

	.header-row input:focus {
		outline: none;
		border-color: rgba(99, 102, 241, 0.5);
	}

	.header-row input::placeholder {
		color: #64748b;
	}

	.btn-remove-header {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 38px;
		background: transparent;
		border: 1px solid rgba(239, 68, 68, 0.2);
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-remove-header:hover {
		background: rgba(239, 68, 68, 0.1);
		border-color: rgba(239, 68, 68, 0.4);
		color: #f87171;
	}

	.no-headers {
		padding: 1rem;
		text-align: center;
		color: #64748b;
		font-size: 0.9rem;
	}

	/* =====================================================
	   Form Actions
	   ===================================================== */
	.form-actions {
		display: flex;
		justify-content: flex-end;
		gap: 1rem;
		padding-top: 1rem;
	}

	.btn-cancel {
		padding: 0.75rem 1.5rem;
		background: transparent;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 10px;
		color: #94a3b8;
		font-size: 0.9rem;
		font-weight: 500;
		text-decoration: none;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-cancel:hover {
		background: rgba(99, 102, 241, 0.1);
		color: #e2e8f0;
	}

	.btn-submit {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		border: none;
		border-radius: 10px;
		color: white;
		font-size: 0.9rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-submit:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(99, 102, 241, 0.4);
	}

	.btn-submit:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-submit :global(.spinning) {
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

	/* =====================================================
	   Loading Spinner
	   ===================================================== */
	.spinner {
		width: 20px;
		height: 20px;
		border: 2px solid rgba(99, 102, 241, 0.2);
		border-top-color: var(--primary-500);
		border-radius: 50%;
		animation: spin 1s linear infinite;
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
	@media (max-width: 640px) {
		.section-header {
			flex-direction: column;
			align-items: flex-start;
			gap: 0.75rem;
		}

		.input-with-action {
			flex-direction: column;
		}

		.header-row {
			flex-direction: column;
		}

		.btn-remove-header {
			width: 100%;
			padding: 0.65rem;
		}

		.form-actions {
			flex-direction: column-reverse;
		}

		.btn-cancel,
		.btn-submit {
			width: 100%;
			justify-content: center;
		}
	}
</style>

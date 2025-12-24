<script lang="ts">
	/**
	 * Membership Details Page - ICT 11+ Principal Engineer Pattern
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Displays membership details with cancel functionality.
	 * Members can cancel their own subscriptions from here.
	 *
	 * @version 1.0.0
	 */
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import {
		getUserMemberships,
		cancelSubscription,
		type UserMembership,
		type UserMembershipsResponse
	} from '$lib/api/user-memberships';
	import DynamicIcon from '$lib/components/DynamicIcon.svelte';
	import IconArrowLeft from '@tabler/icons-svelte/icons/arrow-left';
	import IconCalendar from '@tabler/icons-svelte/icons/calendar';
	import IconCreditCard from '@tabler/icons-svelte/icons/credit-card';
	import IconCheck from '@tabler/icons-svelte/icons/check';
	import IconX from '@tabler/icons-svelte/icons/x';
	import IconAlertTriangle from '@tabler/icons-svelte/icons/alert-triangle';
	import IconExternalLink from '@tabler/icons-svelte/icons/external-link';

	let slug = $derived($page.params.slug);
	let membership = $state<UserMembership | null>(null);
	let loading = $state(true);
	let error = $state<string | null>(null);

	// Cancel modal state
	let showCancelModal = $state(false);
	let cancelImmediately = $state(false);
	let cancelReason = $state('');
	let cancelling = $state(false);
	let cancelError = $state<string | null>(null);

	onMount(async () => {
		await loadMembership();
	});

	async function loadMembership() {
		loading = true;
		error = null;
		try {
			const data: UserMembershipsResponse = await getUserMemberships();
			membership = data.memberships.find((m) => m.slug === slug) || null;

			if (!membership) {
				error = 'Membership not found';
			}
		} catch (err) {
			console.error('Failed to load membership:', err);
			error = 'Failed to load membership details';
		} finally {
			loading = false;
		}
	}

	async function handleCancel() {
		if (!membership) return;

		cancelling = true;
		cancelError = null;

		try {
			await cancelSubscription(membership.id, {
				cancel_immediately: cancelImmediately,
				reason: cancelReason || undefined
			});

			showCancelModal = false;
			// Reload to show updated status
			await loadMembership();
		} catch (err) {
			console.error('Failed to cancel subscription:', err);
			cancelError = 'Failed to cancel subscription. Please try again.';
		} finally {
			cancelling = false;
		}
	}

	function formatDate(dateString?: string): string {
		if (!dateString) return 'N/A';
		return new Date(dateString).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		});
	}

	function formatPrice(price?: number, interval?: string): string {
		if (!price) return 'Free';
		const formatted = new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD'
		}).format(price);
		return interval ? `${formatted}/${interval}` : formatted;
	}

	function getAccessUrl(m: UserMembership): string {
		if (m.type === 'trading-room') {
			return `/trading-room/${m.slug}`;
		}
		return `/dashboard/${m.slug}/alerts`;
	}

	function getStatusBadgeClass(status: string): string {
		switch (status) {
			case 'active':
				return 'status-badge--active';
			case 'expiring':
				return 'status-badge--warning';
			case 'cancelled':
			case 'expired':
				return 'status-badge--error';
			default:
				return 'status-badge--default';
		}
	}
</script>

<svelte:head>
	<title>{membership?.name || 'Membership'} | Revolution Trading Pros</title>
</svelte:head>

<!-- DASHBOARD CONTENT WRAPPER WITH PANEL 2 -->
<div class="dashboard__content">
	<div class="dashboard__content-main">
		<div class="membership-detail">
			<!-- Back Button -->
			<a href="/dashboard" class="back-link">
				<IconArrowLeft size={18} />
				Back to Dashboard
			</a>

	{#if loading}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>Loading membership details...</p>
		</div>
	{:else if error}
		<div class="error-state">
			<IconAlertTriangle size={48} />
			<h2>Error</h2>
			<p>{error}</p>
			<button class="btn btn-primary" onclick={() => goto('/dashboard')}>
				Return to Dashboard
			</button>
		</div>
	{:else if membership}
		<!-- Membership Header -->
		<header class="membership-header">
			<div class="membership-icon">
				<DynamicIcon name={membership.icon} size={48} />
			</div>
			<div class="membership-info">
				<h1>{membership.name}</h1>
				<span class="status-badge {getStatusBadgeClass(membership.status)}">
					{membership.status}
				</span>
			</div>
		</header>

		<!-- Membership Details Cards -->
		<div class="details-grid">
			<!-- Billing Info -->
			<div class="detail-card">
				<h3><IconCreditCard size={20} /> Billing</h3>
				<dl>
					<div class="detail-row">
						<dt>Price</dt>
						<dd>{formatPrice(membership.price, membership.interval)}</dd>
					</div>
					<div class="detail-row">
						<dt>Billing Cycle</dt>
						<dd>{membership.interval || 'One-time'}</dd>
					</div>
					<div class="detail-row">
						<dt>Subscription Type</dt>
						<dd>{membership.membershipType || 'Active'}</dd>
					</div>
				</dl>
			</div>

			<!-- Dates -->
			<div class="detail-card">
				<h3><IconCalendar size={20} /> Dates</h3>
				<dl>
					<div class="detail-row">
						<dt>Start Date</dt>
						<dd>{formatDate(membership.startDate)}</dd>
					</div>
					<div class="detail-row">
						<dt>Next Billing</dt>
						<dd>{formatDate(membership.nextBillingDate)}</dd>
					</div>
					{#if membership.daysUntilExpiry !== undefined}
						<div class="detail-row">
							<dt>Days Remaining</dt>
							<dd class:warning={membership.daysUntilExpiry <= 7}>
								{membership.daysUntilExpiry} days
							</dd>
						</div>
					{/if}
				</dl>
			</div>

			<!-- Features -->
			{#if membership.features && membership.features.length > 0}
				<div class="detail-card features-card">
					<h3><IconCheck size={20} /> Included Features</h3>
					<ul class="features-list">
						{#each membership.features as feature}
							<li>
								<IconCheck size={16} class="feature-check" />
								{feature}
							</li>
						{/each}
					</ul>
				</div>
			{/if}
		</div>

		<!-- Actions -->
		<div class="actions-section">
			<h3>Quick Actions</h3>
			<div class="action-buttons">
				{#if membership.type === 'trading-room'}
					<a href={getAccessUrl(membership)} target="_blank" class="btn btn-primary">
						<IconExternalLink size={18} />
						Enter Trading Room
					</a>
				{:else}
					<a href={getAccessUrl(membership)} class="btn btn-primary">
						View Alerts
					</a>
				{/if}

				{#if membership.status === 'active'}
					<button class="btn btn-danger-outline" onclick={() => (showCancelModal = true)}>
						Cancel Subscription
					</button>
				{/if}
			</div>
		</div>
	{/if}
		</div>
	</div>

<!-- PANEL 2: SECONDARY SIDEBAR (Content Sidebar) - Visible on membership pages, matches SimplerMasteringTheTradeDashboard -->
<aside class="dashboard__content-sidebar">
	<section class="content-sidebar__section">
		<h4 class="content-sidebar__heading">Trading Room Schedule
			<p class="pssubject" style="font-size: 10px;margin-top: 15px;text-transform: initial;">Schedule is subject to change.</p>
		</h4>
		<div class="script-container">
			<p style="padding: 10px; color: #666; font-size: 13px;">Trading room schedule will be displayed here.</p>
		</div>
	</section>
	
	<section class="content-sidebar__section">
		<h4 class="content-sidebar__heading">Quick Links</h4>
		<ul class="link-list">
			<li><a href="https://intercom.help/simpler-trading/en/" target="_blank">Support</a></li>
			<li><a href="/tutorials" target="_blank">Platform Tutorials</a></li>
			<li><a href="/blog" target="_blank">Simpler Blog</a></li>
		</ul>
	</section>
</aside>

</div>

<!-- Cancel Modal -->
{#if showCancelModal && membership}
	<div 
		class="modal-overlay" 
		onclick={() => (showCancelModal = false)}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showCancelModal = false)}
		role="button"
		tabindex="-1"
		aria-label="Close modal"
	>
		<div 
			class="modal-content" 
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
			role="dialog" 
			aria-modal="true"
			aria-labelledby="modal-title"
			tabindex="0"
		>
			<div class="modal-header">
				<h2 id="modal-title">Cancel Subscription</h2>
				<button class="close-btn" onclick={() => (showCancelModal = false)}>
					<IconX size={20} />
				</button>
			</div>

			<div class="modal-body">
				<div class="cancel-warning">
					<IconAlertTriangle size={24} />
					<p>Are you sure you want to cancel <strong>{membership.name}</strong>?</p>
				</div>

				<div class="cancel-options">
					<label class="radio-option">
						<input type="radio" name="cancelType" bind:group={cancelImmediately} value={false} />
						<span class="radio-label">
							<strong>Cancel at end of billing period</strong>
							<small>You'll retain access until {formatDate(membership.nextBillingDate)}</small>
						</span>
					</label>

					<label class="radio-option">
						<input type="radio" name="cancelType" bind:group={cancelImmediately} value={true} />
						<span class="radio-label">
							<strong>Cancel immediately</strong>
							<small>Access will end right away (no refund)</small>
						</span>
					</label>
				</div>

				<div class="form-group">
					<label for="cancelReason">Reason for cancelling (optional)</label>
					<textarea
						id="cancelReason"
						bind:value={cancelReason}
						rows="3"
						placeholder="Help us improve by sharing why you're cancelling..."
					></textarea>
				</div>

				{#if cancelError}
					<div class="error-message">{cancelError}</div>
				{/if}
			</div>

			<div class="modal-footer">
				<button class="btn btn-secondary" onclick={() => (showCancelModal = false)}>
					Keep Subscription
				</button>
				<button class="btn btn-danger" onclick={handleCancel} disabled={cancelling}>
					{cancelling ? 'Cancelling...' : 'Confirm Cancellation'}
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.membership-detail {
		padding: 30px;
		max-width: 1000px;
		margin: 0 auto;
	}

	.back-link {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		color: #1e73be;
		text-decoration: none;
		font-size: 14px;
		font-weight: 500;
		margin-bottom: 24px;
	}

	.back-link:hover {
		text-decoration: underline;
	}

	/* Loading & Error States */
	.loading-state,
	.error-state {
		text-align: center;
		padding: 60px 20px;
		color: #666;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid #e5e7eb;
		border-top-color: #0984ae;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
		margin: 0 auto 16px;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.error-state {
		color: #dc3545;
	}

	.error-state h2 {
		color: #333;
		margin: 16px 0 8px;
	}

	/* Header */
	.membership-header {
		display: flex;
		align-items: center;
		gap: 20px;
		background: #fff;
		padding: 24px;
		border-radius: 8px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
		margin-bottom: 24px;
	}

	.membership-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 80px;
		height: 80px;
		background: #0984ae;
		border-radius: 16px;
		color: #fff;
	}

	.membership-info h1 {
		font-size: 24px;
		font-weight: 600;
		color: #333;
		margin: 0 0 8px;
	}

	.status-badge {
		display: inline-block;
		padding: 4px 12px;
		border-radius: 20px;
		font-size: 12px;
		font-weight: 600;
		text-transform: capitalize;
	}

	.status-badge--active {
		background: #d4edda;
		color: #155724;
	}

	.status-badge--warning {
		background: #fff3cd;
		color: #856404;
	}

	.status-badge--error {
		background: #f8d7da;
		color: #721c24;
	}

	.status-badge--default {
		background: #e9ecef;
		color: #495057;
	}

	/* Details Grid */
	.details-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
		gap: 20px;
		margin-bottom: 24px;
	}

	.detail-card {
		background: #fff;
		padding: 20px;
		border-radius: 8px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
	}

	.detail-card h3 {
		display: flex;
		align-items: center;
		gap: 8px;
		font-size: 16px;
		font-weight: 600;
		color: #333;
		margin: 0 0 16px;
		padding-bottom: 12px;
		border-bottom: 1px solid #e5e7eb;
	}

	.detail-card dl {
		margin: 0;
	}

	.detail-row {
		display: flex;
		justify-content: space-between;
		padding: 8px 0;
		border-bottom: 1px solid #f3f4f6;
	}

	.detail-row:last-child {
		border-bottom: none;
	}

	.detail-row dt {
		color: #6b7280;
		font-size: 14px;
	}

	.detail-row dd {
		color: #333;
		font-weight: 500;
		font-size: 14px;
		margin: 0;
	}

	.detail-row dd.warning {
		color: #f59e0b;
	}

	/* Features */
	.features-card {
		grid-column: 1 / -1;
	}

	.features-list {
		list-style: none;
		padding: 0;
		margin: 0;
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 12px;
	}

	.features-list li {
		display: flex;
		align-items: center;
		gap: 8px;
		font-size: 14px;
		color: #333;
	}

	:global(.feature-check) {
		color: #10b981;
		flex-shrink: 0;
	}

	/* Actions */
	.actions-section {
		background: #fff;
		padding: 20px;
		border-radius: 8px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
	}

	.actions-section h3 {
		font-size: 16px;
		font-weight: 600;
		color: #333;
		margin: 0 0 16px;
	}

	.action-buttons {
		display: flex;
		gap: 12px;
		flex-wrap: wrap;
	}

	/* Buttons */
	.btn {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 10px 20px;
		border-radius: 6px;
		font-size: 14px;
		font-weight: 600;
		text-decoration: none;
		cursor: pointer;
		transition: all 0.15s;
		border: none;
	}

	.btn-primary {
		background: #0984ae;
		color: #fff;
	}

	.btn-primary:hover {
		background: #076787;
	}

	.btn-secondary {
		background: #e5e7eb;
		color: #374151;
	}

	.btn-secondary:hover {
		background: #d1d5db;
	}

	.btn-danger {
		background: #dc3545;
		color: #fff;
	}

	.btn-danger:hover {
		background: #c82333;
	}

	.btn-danger:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-danger-outline {
		background: transparent;
		color: #dc3545;
		border: 1px solid #dc3545;
	}

	.btn-danger-outline:hover {
		background: #dc3545;
		color: #fff;
	}

	/* Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 20px;
	}

	.modal-content {
		background: #fff;
		border-radius: 12px;
		width: 100%;
		max-width: 480px;
		max-height: 90vh;
		overflow-y: auto;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 20px;
		border-bottom: 1px solid #e5e7eb;
	}

	.modal-header h2 {
		font-size: 18px;
		font-weight: 600;
		color: #333;
		margin: 0;
	}

	.close-btn {
		background: none;
		border: none;
		color: #6b7280;
		cursor: pointer;
		padding: 4px;
	}

	.close-btn:hover {
		color: #333;
	}

	.modal-body {
		padding: 20px;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 12px;
		padding: 20px;
		border-top: 1px solid #e5e7eb;
	}

	/* Cancel Form */
	.cancel-warning {
		display: flex;
		align-items: flex-start;
		gap: 12px;
		padding: 16px;
		background: #fef3c7;
		border-radius: 8px;
		margin-bottom: 20px;
		color: #92400e;
	}

	.cancel-warning p {
		margin: 0;
		font-size: 14px;
	}

	.cancel-options {
		display: flex;
		flex-direction: column;
		gap: 12px;
		margin-bottom: 20px;
	}

	.radio-option {
		display: flex;
		align-items: flex-start;
		gap: 12px;
		padding: 12px;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		cursor: pointer;
		transition: border-color 0.15s;
	}

	.radio-option:hover {
		border-color: #0984ae;
	}

	.radio-option input[type='radio'] {
		margin-top: 4px;
	}

	.radio-label {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.radio-label strong {
		font-size: 14px;
		color: #333;
	}

	.radio-label small {
		font-size: 12px;
		color: #6b7280;
	}

	.form-group {
		margin-bottom: 16px;
	}

	.form-group label {
		display: block;
		font-size: 14px;
		font-weight: 500;
		color: #333;
		margin-bottom: 8px;
	}

	.form-group textarea {
		width: 100%;
		padding: 10px 12px;
		border: 1px solid #e5e7eb;
		border-radius: 6px;
		font-size: 14px;
		font-family: inherit;
		resize: vertical;
	}

	.form-group textarea:focus {
		outline: none;
		border-color: #0984ae;
	}

	.error-message {
		padding: 12px;
		background: #fef2f2;
		border-radius: 6px;
		color: #dc3545;
		font-size: 14px;
	}

	@media (max-width: 640px) {
		.membership-header {
			flex-direction: column;
			text-align: center;
		}

		.action-buttons {
			flex-direction: column;
		}

		.action-buttons .btn {
			width: 100%;
			justify-content: center;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD CONTENT LAYOUT - Support for Panel 2
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard__content {
		display: flex;
		flex-direction: row;
		width: 100%;
	}

	.dashboard__content-main {
		flex: 1 1 auto;
		min-width: 0;
	}

	/* Panel 2 Sidebar Styles */
	.dashboard__content-sidebar {
		display: none;
		width: 260px;
		flex: 0 0 auto;
		margin-top: -1px;
		background: #fff;
		border-right: 1px solid #dbdbdb;
		border-top: 1px solid #dbdbdb;
		font-family: 'Open Sans', sans-serif;
		font-size: 14px;
		line-height: 1.6;
	}

	@media (min-width: 1080px) {
		.dashboard__content-sidebar {
			display: block;
		}
	}

	.content-sidebar__section {
		padding: 20px 30px 20px 20px;
		border-bottom: 1px solid #dbdbdb;
	}

	.content-sidebar__heading {
		padding: 15px 20px;
		margin: -20px -30px 20px -20px;
		font-size: 14px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		color: #333;
		background: #ededed;
		border-bottom: 1px solid #dbdbdb;
		line-height: 1.4;
	}

	.link-list {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.link-list li {
		margin: 0;
		padding: 0;
		border-top: 1px solid #dbdbdb;
	}

	.link-list li:first-child {
		border-top: none;
	}

	.link-list a {
		display: block;
		padding: 15px 20px;
		font-size: 14px;
		font-weight: 400;
		color: #666;
		text-decoration: none;
		border-radius: 5px;
		transition: all 0.15s ease-in-out;
	}

	.link-list a:hover {
		background-color: #f4f4f4;
		color: #0984ae;
	}
</style>

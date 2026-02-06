<script lang="ts">
	/**
	 * Admin Subscription Plans Management
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Apple ICT 7+ Principal Engineer Grade - January 2026
	 *
	 * Full CRUD for membership plans with Stripe Price ID management.
	 * Supports the new subscription variants system (monthly/quarterly/annual).
	 */

	import { onMount } from 'svelte';
	import { getAuthToken } from '$lib/stores/auth.svelte';

	// Types
	interface SubscriptionPlan {
		id: number;
		name: string;
		slug: string;
		display_name?: string;
		description?: string;
		price: number;
		billing_cycle: string;
		interval_count?: number;
		is_active: boolean;
		stripe_price_id?: string;
		stripe_product_id?: string;
		features?: string[];
		trial_days?: number;
		room_id?: number;
		room_name?: string;
		savings_percent?: number;
		is_popular?: boolean;
		sort_order?: number;
		created_at: string;
		updated_at: string;
	}

	// State
	let plans = $state<SubscriptionPlan[]>([]);
	let loading = $state(true);
	let error = $state('');
	let successMessage = $state('');

	// Edit modal state
	let showEditModal = $state(false);
	let editingPlan = $state<SubscriptionPlan | null>(null);
	let saving = $state(false);

	// Filter state
	let filterActive = $state<'all' | 'active' | 'inactive'>('all');
	let filterBillingCycle = $state<'all' | 'monthly' | 'quarterly' | 'annual'>('all');
	let searchQuery = $state('');

	// Derived
	let filteredPlans = $derived(
		plans.filter((plan) => {
			if (filterActive === 'active' && !plan.is_active) return false;
			if (filterActive === 'inactive' && plan.is_active) return false;
			if (filterBillingCycle !== 'all' && plan.billing_cycle !== filterBillingCycle) return false;
			if (searchQuery) {
				const q = searchQuery.toLowerCase();
				return (
					plan.name.toLowerCase().includes(q) ||
					plan.slug.toLowerCase().includes(q) ||
					(plan.stripe_price_id || '').toLowerCase().includes(q)
				);
			}
			return true;
		})
	);

	let planStats = $derived({
		total: plans.length,
		active: plans.filter((p) => p.is_active).length,
		withStripeId: plans.filter((p) => p.stripe_price_id).length,
		missingStripeId: plans.filter((p) => !p.stripe_price_id && p.is_active).length
	});

	onMount(() => {
		loadPlans();
	});

	async function loadPlans() {
		loading = true;
		error = '';

		try {
			const token = getAuthToken();
			const response = await fetch('/api/admin/subscriptions/plans?per_page=100', {
				headers: {
					Authorization: `Bearer ${token}`,
					'Content-Type': 'application/json'
				},
				credentials: 'include'
			});

			if (!response.ok) {
				throw new Error(`Failed to load plans: ${response.status}`);
			}

			const data = await response.json();
			plans = data.data || [];
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load plans';
			console.error('[Plans] Load error:', err);
		} finally {
			loading = false;
		}
	}

	function openEditModal(plan: SubscriptionPlan) {
		editingPlan = { ...plan };
		showEditModal = true;
		successMessage = '';
	}

	function closeEditModal() {
		showEditModal = false;
		editingPlan = null;
	}

	async function savePlan() {
		if (!editingPlan) return;

		saving = true;
		error = '';

		try {
			const token = getAuthToken();
			const response = await fetch(`/api/admin/subscriptions/plans/${editingPlan.id}`, {
				method: 'PUT',
				headers: {
					Authorization: `Bearer ${token}`,
					'Content-Type': 'application/json'
				},
				credentials: 'include',
				body: JSON.stringify({
					name: editingPlan.name,
					description: editingPlan.description,
					price: editingPlan.price,
					billing_cycle: editingPlan.billing_cycle,
					is_active: editingPlan.is_active,
					stripe_price_id: editingPlan.stripe_price_id || null,
					trial_days: editingPlan.trial_days
				})
			});

			if (!response.ok) {
				const errData = await response.json().catch(() => ({}));
				throw new Error(errData.error || `Failed to save: ${response.status}`);
			}

			successMessage = `Plan "${editingPlan.name}" updated successfully!`;
			closeEditModal();
			await loadPlans();

			// Clear success message after 3 seconds
			setTimeout(() => (successMessage = ''), 3000);
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to save plan';
		} finally {
			saving = false;
		}
	}

	async function togglePlanActive(plan: SubscriptionPlan) {
		try {
			const token = getAuthToken();
			const response = await fetch(`/api/admin/subscriptions/plans/${plan.id}`, {
				method: 'PUT',
				headers: {
					Authorization: `Bearer ${token}`,
					'Content-Type': 'application/json'
				},
				credentials: 'include',
				body: JSON.stringify({ is_active: !plan.is_active })
			});

			if (!response.ok) throw new Error('Failed to toggle status');

			await loadPlans();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to update plan';
		}
	}

	function formatPrice(price: number): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD'
		}).format(price);
	}

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric'
		});
	}

	function getBillingCycleLabel(cycle: string): string {
		switch (cycle) {
			case 'monthly':
				return 'Monthly';
			case 'quarterly':
				return 'Quarterly';
			case 'annual':
			case 'yearly':
				return 'Annual';
			default:
				return cycle;
		}
	}

	function getBillingCycleColor(cycle: string): string {
		switch (cycle) {
			case 'monthly':
				return 'bg-blue-500/10 text-blue-400 border-blue-500/20';
			case 'quarterly':
				return 'bg-purple-500/10 text-purple-400 border-purple-500/20';
			case 'annual':
			case 'yearly':
				return 'bg-emerald-500/10 text-emerald-400 border-emerald-500/20';
			default:
				return 'bg-slate-500/10 text-slate-400 border-slate-500/20';
		}
	}
</script>

<svelte:head>
	<title>Subscription Plans - Admin</title>
</svelte:head>

<div class="admin-plans">
	<div class="admin-page-container">
		<!-- Background Effects -->
		<div class="bg-effects">
			<div class="bg-blob bg-blob-1"></div>
			<div class="bg-blob bg-blob-2"></div>
		</div>

		<!-- Header -->
		<header class="page-header">
			<div class="header-content">
				<div>
					<h1>Subscription Plans</h1>
					<p class="subtitle">Manage membership plans and Stripe integration</p>
				</div>
				<a href="/admin/subscriptions" class="btn-secondary">
					<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M15 19l-7-7 7-7"
						/>
					</svg>
					Back to Subscriptions
				</a>
			</div>
		</header>

		<!-- Success Message -->
		{#if successMessage}
			<div class="success-banner">
				<svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
					<path
						fill-rule="evenodd"
						d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
						clip-rule="evenodd"
					/>
				</svg>
				{successMessage}
			</div>
		{/if}

		<!-- Error Banner -->
		{#if error}
			<div class="error-banner">
				<svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
					<path
						fill-rule="evenodd"
						d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
						clip-rule="evenodd"
					/>
				</svg>
				{error}
				<button onclick={() => (error = '')} class="ml-auto text-red-300 hover:text-white">×</button
				>
			</div>
		{/if}

		<!-- Stats Cards -->
		<div class="stats-grid">
			<div class="stat-card">
				<div class="stat-icon bg-blue-500/10">
					<svg class="w-6 h-6 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"
						/>
					</svg>
				</div>
				<div class="stat-content">
					<p class="stat-label">Total Plans</p>
					<p class="stat-value">{planStats.total}</p>
				</div>
			</div>

			<div class="stat-card">
				<div class="stat-icon bg-emerald-500/10">
					<svg
						class="w-6 h-6 text-emerald-400"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
						/>
					</svg>
				</div>
				<div class="stat-content">
					<p class="stat-label">Active Plans</p>
					<p class="stat-value">{planStats.active}</p>
				</div>
			</div>

			<div class="stat-card">
				<div class="stat-icon bg-purple-500/10">
					<svg
						class="w-6 h-6 text-purple-400"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z"
						/>
					</svg>
				</div>
				<div class="stat-content">
					<p class="stat-label">With Stripe ID</p>
					<p class="stat-value">{planStats.withStripeId}</p>
				</div>
			</div>

			<div class="stat-card" class:warning={planStats.missingStripeId > 0}>
				<div class="stat-icon bg-orange-500/10">
					<svg
						class="w-6 h-6 text-orange-400"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
						/>
					</svg>
				</div>
				<div class="stat-content">
					<p class="stat-label">Missing Stripe ID</p>
					<p class="stat-value">{planStats.missingStripeId}</p>
				</div>
			</div>
		</div>

		<!-- Filters -->
		<div class="filters-section">
			<div class="filters-row">
				<div class="filter-group">
					<label for="search">Search</label>
					<input
						type="text"
						id="search"
						name="search"
						bind:value={searchQuery}
						placeholder="Search plans..."
						class="filter-input"
					/>
				</div>

				<div class="filter-group">
					<label for="status">Status</label>
					<select id="status" bind:value={filterActive} class="filter-select">
						<option value="all">All Status</option>
						<option value="active">Active Only</option>
						<option value="inactive">Inactive Only</option>
					</select>
				</div>

				<div class="filter-group">
					<label for="billing">Billing Cycle</label>
					<select id="billing" bind:value={filterBillingCycle} class="filter-select">
						<option value="all">All Cycles</option>
						<option value="monthly">Monthly</option>
						<option value="quarterly">Quarterly</option>
						<option value="annual">Annual</option>
					</select>
				</div>
			</div>
		</div>

		<!-- Plans Table -->
		<div class="table-container">
			{#if loading}
				<div class="loading-state">
					<div class="spinner"></div>
					<p>Loading plans...</p>
				</div>
			{:else if filteredPlans.length === 0}
				<div class="empty-state">
					<svg
						class="w-12 h-12 text-slate-500"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"
						/>
					</svg>
					<p>No plans found</p>
				</div>
			{:else}
				<table class="plans-table">
					<thead>
						<tr>
							<th>Plan</th>
							<th>Price</th>
							<th>Billing</th>
							<th>Stripe Price ID</th>
							<th>Status</th>
							<th>Actions</th>
						</tr>
					</thead>
					<tbody>
						{#each filteredPlans as plan (plan.id)}
							<tr class:inactive={!plan.is_active}>
								<td>
									<div class="plan-info">
										<span class="plan-name">{plan.name}</span>
										<span class="plan-slug">{plan.slug}</span>
									</div>
								</td>
								<td class="price-cell">{formatPrice(plan.price)}</td>
								<td>
									<span class="billing-badge {getBillingCycleColor(plan.billing_cycle)}">
										{getBillingCycleLabel(plan.billing_cycle)}
									</span>
								</td>
								<td class="stripe-cell">
									{#if plan.stripe_price_id}
										<code class="stripe-id">{plan.stripe_price_id}</code>
									{:else}
										<span class="missing-stripe">
											<svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
												<path
													fill-rule="evenodd"
													d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z"
													clip-rule="evenodd"
												/>
											</svg>
											Not Set
										</span>
									{/if}
								</td>
								<td>
									<button
										class="status-toggle"
										class:active={plan.is_active}
										onclick={() => togglePlanActive(plan)}
										title={plan.is_active ? 'Click to deactivate' : 'Click to activate'}
									>
										<span class="status-dot"></span>
										{plan.is_active ? 'Active' : 'Inactive'}
									</button>
								</td>
								<td>
									<button class="btn-edit" onclick={() => openEditModal(plan)}> Edit </button>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			{/if}
		</div>

		<!-- Stripe Setup Guide -->
		<div class="help-section">
			<h3>Setting Up Stripe Price IDs</h3>
			<ol>
				<li>
					Go to <a href="https://dashboard.stripe.com/products" target="_blank" rel="noopener"
						>Stripe Dashboard → Products</a
					>
				</li>
				<li>Create a product for each trading room (e.g., "Explosive Swings")</li>
				<li>Add prices: Monthly, Quarterly, Annual with matching amounts</li>
				<li>Copy each Price ID (starts with <code>price_</code>)</li>
				<li>Click "Edit" on the plan above and paste the Stripe Price ID</li>
			</ol>
		</div>
	</div>
</div>

<!-- Edit Modal -->
{#if showEditModal && editingPlan}
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<div
		class="modal-overlay"
		onclick={closeEditModal}
		onkeydown={(e) => {
			if (e.key === 'Escape' || e.key === 'Enter' || e.key === ' ') {
				e.preventDefault();
				closeEditModal();
			}
		}}
		role="dialog"
		aria-modal="true"
		aria-label="Close modal"
		tabindex="0"
	>
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div class="modal" onmousedown={(e) => e.stopPropagation()} role="document">
			<div class="modal-header">
				<h2>Edit Plan</h2>
				<button class="modal-close" onclick={closeEditModal}>×</button>
			</div>

			<div class="modal-body">
				<div class="form-group">
					<label for="edit-name">Plan Name</label>
					<input
						type="text"
						id="edit-name"
						name="edit-name"
						bind:value={editingPlan.name}
						class="form-input"
					/>
				</div>

				<div class="form-row">
					<div class="form-group">
						<label for="edit-price">Price ($)</label>
						<input
							type="number"
							id="edit-price"
							name="edit-price"
							bind:value={editingPlan.price}
							step="0.01"
							min="0"
							class="form-input"
						/>
					</div>

					<div class="form-group">
						<label for="edit-billing">Billing Cycle</label>
						<select id="edit-billing" bind:value={editingPlan.billing_cycle} class="form-input">
							<option value="monthly">Monthly</option>
							<option value="quarterly">Quarterly</option>
							<option value="annual">Annual</option>
						</select>
					</div>
				</div>

				<div class="form-group">
					<label for="edit-stripe">
						Stripe Price ID
						<span class="label-hint">Required for payments</span>
					</label>
					<input
						type="text"
						id="edit-stripe"
						name="edit-stripe"
						bind:value={editingPlan.stripe_price_id}
						placeholder="price_1ABC123..."
						class="form-input font-mono"
					/>
					<p class="form-hint">
						Get this from <a
							href="https://dashboard.stripe.com/products"
							target="_blank"
							rel="noopener">Stripe Dashboard</a
						>
					</p>
				</div>

				<div class="form-group">
					<label for="edit-description">Description</label>
					<textarea
						id="edit-description"
						bind:value={editingPlan.description}
						rows="3"
						class="form-input"
					></textarea>
				</div>

				<div class="form-group">
					<label for="edit-trial">Trial Days</label>
					<input
						type="number"
						id="edit-trial"
						name="edit-trial"
						bind:value={editingPlan.trial_days}
						min="0"
						class="form-input"
						style="max-width: 120px"
					/>
				</div>

				<div class="form-group">
					<label class="checkbox-label">
						<input
							id="page-editingplan-is-active"
							name="page-editingplan-is-active"
							type="checkbox"
							bind:checked={editingPlan.is_active}
						/>
						<span>Plan is Active</span>
					</label>
				</div>
			</div>

			<div class="modal-footer">
				<button class="btn-cancel" onclick={closeEditModal} disabled={saving}> Cancel </button>
				<button class="btn-save" onclick={savePlan} disabled={saving}>
					{#if saving}
						Saving...
					{:else}
						Save Changes
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.admin-plans {
		background: linear-gradient(135deg, #0f172a 0%, #1e293b 50%, #0f172a 100%);
		color: white;
		position: relative;
	}

	.admin-page-container {
		position: relative;
		z-index: 10;
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem 1rem;
	}

	.bg-effects {
		position: fixed;
		inset: 0;
		pointer-events: none;
		overflow: hidden;
	}

	.bg-blob {
		position: absolute;
		border-radius: 50%;
		filter: blur(80px);
		opacity: 0.12;
	}

	.bg-blob-1 {
		width: 500px;
		height: 500px;
		top: -150px;
		right: -150px;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
	}

	.bg-blob-2 {
		width: 400px;
		height: 400px;
		bottom: -100px;
		left: -100px;
		background: linear-gradient(135deg, #10b981, #14b8a6);
	}

	/* Header */
	.page-header {
		margin-bottom: 2rem;
	}

	.header-content {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		flex-wrap: wrap;
		gap: 1rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.25rem;
	}

	.subtitle {
		color: #64748b;
		font-size: 0.9rem;
		margin: 0;
	}

	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: rgba(71, 85, 105, 0.5);
		border: 1px solid rgba(100, 116, 139, 0.3);
		border-radius: 8px;
		color: #cbd5e1;
		font-size: 0.875rem;
		font-weight: 500;
		text-decoration: none;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		background: rgba(71, 85, 105, 0.7);
		color: white;
	}

	/* Banners */
	.success-banner,
	.error-banner {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.25rem;
		border-radius: 10px;
		margin-bottom: 1.5rem;
		font-size: 0.9rem;
	}

	.success-banner {
		background: rgba(16, 185, 129, 0.15);
		border: 1px solid rgba(16, 185, 129, 0.3);
		color: #34d399;
	}

	.error-banner {
		background: rgba(239, 68, 68, 0.15);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #f87171;
	}

	/* Stats Grid */
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.stat-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 12px;
		backdrop-filter: blur(8px);
	}

	.stat-card.warning {
		border-color: rgba(245, 158, 11, 0.4);
	}

	.stat-icon {
		width: 48px;
		height: 48px;
		border-radius: 10px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.stat-label {
		font-size: 0.8rem;
		color: #94a3b8;
		margin: 0 0 0.25rem;
	}

	.stat-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: white;
		margin: 0;
	}

	/* Filters */
	.filters-section {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 12px;
		padding: 1.25rem;
		margin-bottom: 1.5rem;
	}

	.filters-row {
		display: grid;
		grid-template-columns: 1fr auto auto;
		gap: 1rem;
		align-items: end;
	}

	.filter-group label {
		display: block;
		font-size: 0.75rem;
		font-weight: 600;
		color: #94a3b8;
		margin-bottom: 0.5rem;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.filter-input,
	.filter-select {
		width: 100%;
		padding: 0.625rem 0.875rem;
		background: rgba(15, 23, 42, 0.8);
		border: 1px solid rgba(71, 85, 105, 0.5);
		border-radius: 8px;
		color: white;
		font-size: 0.875rem;
	}

	.filter-input:focus,
	.filter-select:focus {
		outline: none;
		border-color: #6366f1;
	}

	.filter-select {
		min-width: 150px;
	}

	/* Table */
	.table-container {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 12px;
		overflow: hidden;
		margin-bottom: 1.5rem;
	}

	.plans-table {
		width: 100%;
		border-collapse: collapse;
	}

	.plans-table th {
		padding: 1rem 1.25rem;
		text-align: left;
		font-size: 0.75rem;
		font-weight: 600;
		color: #94a3b8;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		background: rgba(15, 23, 42, 0.5);
		border-bottom: 1px solid rgba(51, 65, 85, 0.5);
	}

	.plans-table td {
		padding: 1rem 1.25rem;
		border-bottom: 1px solid rgba(51, 65, 85, 0.3);
		vertical-align: middle;
	}

	.plans-table tr:last-child td {
		border-bottom: none;
	}

	.plans-table tr.inactive {
		opacity: 0.6;
	}

	.plans-table tr:hover {
		background: rgba(51, 65, 85, 0.2);
	}

	.plan-info {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.plan-name {
		font-weight: 600;
		color: white;
	}

	.plan-slug {
		font-size: 0.8rem;
		color: #64748b;
		font-family: monospace;
	}

	.price-cell {
		font-weight: 600;
		color: #10b981;
	}

	.billing-badge {
		display: inline-flex;
		padding: 0.25rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 600;
		border: 1px solid;
	}

	.stripe-cell {
		font-family: monospace;
	}

	.stripe-id {
		font-size: 0.8rem;
		background: rgba(99, 102, 241, 0.15);
		padding: 0.25rem 0.5rem;
		border-radius: 4px;
		color: #a5b4fc;
	}

	.missing-stripe {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		color: #f59e0b;
		font-size: 0.85rem;
		font-family: inherit;
	}

	.status-toggle {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.375rem 0.75rem;
		background: rgba(71, 85, 105, 0.3);
		border: 1px solid rgba(71, 85, 105, 0.5);
		border-radius: 9999px;
		color: #94a3b8;
		font-size: 0.8rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.status-toggle.active {
		background: rgba(16, 185, 129, 0.15);
		border-color: rgba(16, 185, 129, 0.3);
		color: #34d399;
	}

	.status-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: currentColor;
	}

	.btn-edit {
		padding: 0.5rem 1rem;
		background: rgba(99, 102, 241, 0.15);
		border: 1px solid rgba(99, 102, 241, 0.3);
		border-radius: 6px;
		color: #a5b4fc;
		font-size: 0.85rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-edit:hover {
		background: rgba(99, 102, 241, 0.25);
		color: white;
	}

	/* Loading & Empty States */
	.loading-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		color: #64748b;
		gap: 1rem;
	}

	.spinner {
		width: 32px;
		height: 32px;
		border: 3px solid rgba(99, 102, 241, 0.2);
		border-top-color: #6366f1;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Help Section */
	.help-section {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 12px;
		padding: 1.5rem;
	}

	.help-section h3 {
		font-size: 1rem;
		font-weight: 600;
		color: #e2e8f0;
		margin: 0 0 1rem;
	}

	.help-section ol {
		margin: 0;
		padding-left: 1.25rem;
		color: #94a3b8;
		font-size: 0.9rem;
		line-height: 1.8;
	}

	.help-section a {
		color: #6366f1;
		text-decoration: none;
	}

	.help-section a:hover {
		text-decoration: underline;
	}

	.help-section code {
		background: rgba(99, 102, 241, 0.15);
		padding: 0.125rem 0.375rem;
		border-radius: 3px;
		font-size: 0.85em;
		color: #a5b4fc;
	}

	/* Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.75);
		backdrop-filter: blur(4px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 100;
		padding: 1rem;
	}

	.modal {
		background: #1e293b;
		border: 1px solid rgba(51, 65, 85, 0.8);
		border-radius: 16px;
		width: 100%;
		max-width: 500px;
		max-height: 90vh;
		overflow-y: auto;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.25rem 1.5rem;
		border-bottom: 1px solid rgba(51, 65, 85, 0.5);
	}

	.modal-header h2 {
		font-size: 1.125rem;
		font-weight: 600;
		color: white;
		margin: 0;
	}

	.modal-close {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: none;
		color: #64748b;
		font-size: 1.5rem;
		cursor: pointer;
		border-radius: 6px;
		transition: all 0.2s;
	}

	.modal-close:hover {
		background: rgba(71, 85, 105, 0.5);
		color: white;
	}

	.modal-body {
		padding: 1.5rem;
	}

	.form-group {
		margin-bottom: 1.25rem;
	}

	.form-group:last-child {
		margin-bottom: 0;
	}

	.form-group label {
		display: block;
		font-size: 0.875rem;
		font-weight: 500;
		color: #e2e8f0;
		margin-bottom: 0.5rem;
	}

	.label-hint {
		font-weight: 400;
		color: #64748b;
		font-size: 0.8rem;
	}

	.form-input {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.8);
		border: 1px solid rgba(71, 85, 105, 0.5);
		border-radius: 8px;
		color: white;
		font-size: 0.9rem;
	}

	.form-input:focus {
		outline: none;
		border-color: #6366f1;
	}

	.form-input.font-mono {
		font-family: monospace;
	}

	.form-hint {
		font-size: 0.8rem;
		color: #64748b;
		margin: 0.5rem 0 0;
	}

	.form-hint a {
		color: #6366f1;
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		cursor: pointer;
	}

	.checkbox-label input {
		width: 18px;
		height: 18px;
		accent-color: #6366f1;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1.25rem 1.5rem;
		border-top: 1px solid rgba(51, 65, 85, 0.5);
	}

	.btn-cancel,
	.btn-save {
		padding: 0.75rem 1.5rem;
		border-radius: 8px;
		font-size: 0.9rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-cancel {
		background: transparent;
		border: 1px solid rgba(71, 85, 105, 0.5);
		color: #94a3b8;
	}

	.btn-cancel:hover:not(:disabled) {
		background: rgba(71, 85, 105, 0.3);
		color: white;
	}

	.btn-save {
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		border: none;
		color: white;
	}

	.btn-save:hover:not(:disabled) {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(99, 102, 241, 0.4);
	}

	.btn-save:disabled,
	.btn-cancel:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.filters-row {
			grid-template-columns: 1fr;
		}

		.form-row {
			grid-template-columns: 1fr;
		}

		.table-container {
			overflow-x: auto;
		}

		.plans-table {
			min-width: 700px;
		}
	}
</style>

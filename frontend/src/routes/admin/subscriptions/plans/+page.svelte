<script lang="ts">
	/**
	 * Admin Subscription Plans Management
	 * Apple ICT 7+ Principal Engineer Grade - January 2026
	 *
	 * Full CRUD for membership plans with Stripe Price ID management.
	 * Supports the new subscription variants system (monthly/quarterly/annual).
	 */

	import { onMount } from 'svelte';
	// FIX-2026-04-26 (audit 02 §P3-6): no longer reading the auth token here —
	// the SvelteKit proxy reads `rtp_access_token` directly from cookies.
	// FIX-2026-04-26: Tabler icons replace raw inline <svg> blocks.
	import IconChevronLeft from '@tabler/icons-svelte-runes/icons/chevron-left';
	import IconCircleCheckFilled from '@tabler/icons-svelte-runes/icons/circle-check-filled';
	import IconCircleXFilled from '@tabler/icons-svelte-runes/icons/circle-x-filled';
	// FIX-2026-04-26: was `import IconLayers from '@tabler/icons-svelte-runes/icons/layers'`
	// — that icon path doesn't exist in @tabler/icons-svelte-runes@3.41.1 (only
	// layers-difference, layers-intersect, layers-linked, layers-off,
	// layers-selected, layers-subtract are exported). Build was failing with
	// "Rollup cannot resolve" since commit 99b9583d2 (the Tabler-SVG sweep).
	// Using `layers-linked` as the closest semantic match for a "Total Plans"
	// stat icon.
	import IconLayers from '@tabler/icons-svelte-runes/icons/layers-linked';
	import IconAlertTriangle from '@tabler/icons-svelte-runes/icons/alert-triangle';
	import IconCircleCheck from '@tabler/icons-svelte-runes/icons/circle-check';
	import IconCreditCard from '@tabler/icons-svelte-runes/icons/credit-card';
	import IconClipboard from '@tabler/icons-svelte-runes/icons/clipboard';
	import {
		getPlans,
		getPriceHistory,
		changePlanPrice,
		updatePlan,
		setPlanActive
	} from './plans.remote';
	import type { ApplyTo, PriceHistoryEntry, SubscriptionPlan } from './plans.types';

	// State
	let plans = $state<SubscriptionPlan[]>([]);
	let loading = $state(true);
	let error = $state('');
	let successMessage = $state('');

	// Edit modal state
	let showEditModal = $state(false);
	let editingPlan = $state<SubscriptionPlan | null>(null);
	let saving = $state(false);

	// Price-change modal state (Stripe-syncing, no dashboard required)
	let showPriceModal = $state(false);
	let priceTargetPlan = $state<SubscriptionPlan | null>(null);
	let priceAmount = $state(0); // dollars in the input; converted to cents at submit
	let priceInterval = $state<'month' | 'year' | 'one_time'>('month');
	let priceApplyTo = $state<ApplyTo>('new_only');
	let priceSubmitting = $state(false);
	let showPriceConfirm = $state(false);
	let priceHistory = $state<PriceHistoryEntry[]>([]);
	let priceHistoryLoading = $state(false);

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
			// `.refresh()` (not a bare `await`): `getPlans()` is argument-less, so
			// it's cached under one key — after a mutation, a plain re-read would
			// return the stale list. refresh() forces a fresh fetch.
			const q = getPlans();
			await q.refresh();
			plans = q.current ?? [];
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

	// ── Price-change modal ──────────────────────────────────────────────────
	function intervalForPlan(plan: SubscriptionPlan): 'month' | 'year' | 'one_time' {
		switch (plan.billing_cycle) {
			case 'annual':
			case 'yearly':
				return 'year';
			case 'monthly':
			case 'quarterly':
				return 'month';
			default:
				return 'month';
		}
	}

	async function openPriceModal(plan: SubscriptionPlan) {
		priceTargetPlan = plan;
		priceAmount = Math.round(plan.price * 100) / 100;
		priceInterval = intervalForPlan(plan);
		priceApplyTo = 'new_only';
		showPriceConfirm = false;
		showPriceModal = true;
		successMessage = '';
		await loadPriceHistory(plan.id);
	}

	function closePriceModal() {
		showPriceModal = false;
		priceTargetPlan = null;
		showPriceConfirm = false;
		priceHistory = [];
	}

	async function loadPriceHistory(planId: number) {
		priceHistoryLoading = true;
		try {
			// refresh() so reopening a plan's modal after a price change shows the
			// new entry rather than the cached history.
			const q = getPriceHistory(planId);
			await q.refresh();
			priceHistory = q.current ?? [];
		} catch (err) {
			console.error('[Plans] Failed to load price history', err);
			priceHistory = [];
		} finally {
			priceHistoryLoading = false;
		}
	}

	async function submitPriceChange() {
		if (!priceTargetPlan) return;
		if (priceAmount <= 0) {
			error = 'Amount must be greater than zero';
			return;
		}

		priceSubmitting = true;
		error = '';
		try {
			const amountCents = Math.round(priceAmount * 100);
			const data = await changePlanPrice({
				planId: priceTargetPlan.id,
				amount_cents: amountCents,
				currency: 'usd',
				billing_interval: priceInterval,
				apply_to: priceApplyTo
			});

			const planName = priceTargetPlan.name;
			let detail = '';
			if (priceApplyTo === 'new_only') {
				detail = 'New members only — existing subscribers stay on the old price.';
			} else if (priceApplyTo === 'next_renewal') {
				detail = `${data.subscriptions_migrated} existing subscriber(s) will switch at next renewal (no proration).`;
			} else {
				detail = `${data.subscriptions_migrated} existing subscriber(s) migrated immediately with proration.`;
			}
			if (data.subscriptions_failed > 0) {
				detail += ` ${data.subscriptions_failed} migration(s) failed — see audit log.`;
			}
			successMessage = `Price for "${planName}" updated. ${detail}`;
			closePriceModal();
			await loadPlans();
			setTimeout(() => (successMessage = ''), 6000);
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to change price';
		} finally {
			priceSubmitting = false;
		}
	}

	async function savePlan() {
		if (!editingPlan) return;

		saving = true;
		error = '';

		try {
			// FIX-2026-04-26 (audit 02 §P3-6): cookie-authed proxy.
			// FIX (audit FULL_REPO_AUDIT_2026-05-17 §P0-5, MONEY_PATH_DIG F2):
			// the backend `UpdatePlanRequest` expects integer cents
			// (`price_cents: Option<i64>`, subscriptions_admin.rs:132-144) and
			// silently ignores an unknown dollars `price` key — so the old
			// hand-built `price: editingPlan.price` body dropped every price
			// change. Send `price_cents` instead. `Math.round` is mandatory:
			// `19.99 * 100` is `1998.9999…` in float; truncation corrupts a
			// cent. This mirrors the single normalizer chokepoint in
			// `subscriptionPlansApi` (admin.ts::normalizePlanPayload) so there
			// is no second un-normalized path.
			await updatePlan({
				planId: editingPlan.id,
				payload: {
					name: editingPlan.name,
					description: editingPlan.description,
					price_cents: Math.round(Number(editingPlan.price) * 100),
					billing_cycle: editingPlan.billing_cycle,
					is_active: editingPlan.is_active,
					stripe_price_id: editingPlan.stripe_price_id || null,
					trial_days: editingPlan.trial_days,
					trial_period_days: editingPlan.trial_period_days ?? null,
					trial_requires_payment_method: editingPlan.trial_requires_payment_method ?? true
				}
			});

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
			await setPlanActive({ planId: plan.id, is_active: !plan.is_active });
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

	function _formatDate(dateString: string): string {
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

	function getBillingCycleClass(cycle: string): string {
		switch (cycle) {
			case 'monthly':
				return 'billing-monthly';
			case 'quarterly':
				return 'billing-quarterly';
			case 'annual':
			case 'yearly':
				return 'billing-annual';
			default:
				return 'billing-default';
		}
	}
</script>

<svelte:head>
	<title>Subscription Plans - Admin</title>
</svelte:head>

<div class="admin-plans">
	<div class="admin-page-container">
		<!-- Background Effects -->
		<div class="background-effects">
			<div class="background-blob background-blob-1"></div>
			<div class="background-blob background-blob-2"></div>
		</div>

		<!-- Header -->
		<header class="page-header">
			<div class="header-content">
				<div>
					<h1>Subscription Plans</h1>
					<p class="subtitle">Manage membership plans and Stripe integration</p>
				</div>
				<a href="/admin/subscriptions" class="btn-secondary">
					<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: chevron-left (back) -->
					<IconChevronLeft size={20} aria-hidden="true" />
					Back to Subscriptions
				</a>
			</div>
		</header>

		<!-- Success Message -->
		{#if successMessage}
			<div class="success-banner">
				<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: circle-check success -->
				<IconCircleCheckFilled size={20} aria-hidden="true" />
				{successMessage}
			</div>
		{/if}

		<!-- Error Banner -->
		{#if error}
			<div class="error-banner">
				<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: circle-x error -->
				<IconCircleXFilled size={20} aria-hidden="true" />
				{error}
				<button onclick={() => (error = '')} class="banner-close" aria-label="Dismiss error"
					>×</button
				>
			</div>
		{/if}

		<!-- Stats Cards -->
		<div class="stats-grid">
			<div class="stat-card">
				<div class="stat-icon stat-icon-total">
					<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: layers (archive) -->
					<IconLayers size={24} aria-hidden="true" />
				</div>
				<div class="stat-content">
					<p class="stat-label">Total Plans</p>
					<p class="stat-value">{planStats.total}</p>
				</div>
			</div>

			<div class="stat-card">
				<div class="stat-icon stat-icon-active">
					<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: circle-check (active plans stat) -->
					<IconCircleCheck size={24} aria-hidden="true" />
				</div>
				<div class="stat-content">
					<p class="stat-label">Active Plans</p>
					<p class="stat-value">{planStats.active}</p>
				</div>
			</div>

			<div class="stat-card">
				<div class="stat-icon stat-icon-stripe">
					<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: credit-card (stripe stat) -->
					<IconCreditCard size={24} aria-hidden="true" />
				</div>
				<div class="stat-content">
					<p class="stat-label">With Stripe ID</p>
					<p class="stat-value">{planStats.withStripeId}</p>
				</div>
			</div>

			<div class={['stat-card', { warning: planStats.missingStripeId > 0 }]}>
				<div class="stat-icon stat-icon-warning">
					<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: alert-triangle (missing stripe stat) -->
					<IconAlertTriangle size={24} aria-hidden="true" />
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
					<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: clipboard (no plans empty state) -->
					<IconClipboard size={48} aria-hidden="true" />
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
							<tr class={{ inactive: !plan.is_active }}>
								<td>
									<div class="plan-info">
										<span class="plan-name">{plan.name}</span>
										<span class="plan-slug">{plan.slug}</span>
									</div>
								</td>
								<td class="price-cell">{formatPrice(plan.price)}</td>
								<td>
									<span class={['billing-badge', getBillingCycleClass(plan.billing_cycle)]}>
										{getBillingCycleLabel(plan.billing_cycle)}
									</span>
								</td>
								<td class="stripe-cell">
									{#if plan.stripe_price_id}
										<code class="stripe-id">{plan.stripe_price_id}</code>
									{:else}
										<span class="missing-stripe">
											<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: alert-triangle warning -->
											<IconAlertTriangle size={16} aria-hidden="true" />
											Not Set
										</span>
									{/if}
								</td>
								<td>
									<button
										class={['status-toggle', { active: plan.is_active }]}
										onclick={() => togglePlanActive(plan)}
										title={plan.is_active ? 'Click to deactivate' : 'Click to activate'}
									>
										<span class="status-dot"></span>
										{plan.is_active ? 'Active' : 'Inactive'}
									</button>
								</td>
								<td>
									<div class="row-actions">
										<button class="btn-edit" onclick={() => openEditModal(plan)}>Edit</button>
										<button
											class="btn-price"
											onclick={() => openPriceModal(plan)}
											title="Change Stripe price with grandfathering options"
										>
											Change Price
										</button>
									</div>
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
	<div
		class="modal-overlay"
		onclick={(e) => {
			if (e.target === e.currentTarget) closeEditModal();
		}}
		onkeydown={(e) => {
			if (e.key === 'Escape') closeEditModal();
		}}
		role="dialog"
		aria-modal="true"
		aria-label="Edit plan"
		tabindex="-1"
	>
		<div class="modal" role="document">
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
						class="form-input stripe-price-input"
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
						class="form-input trial-days-input"
					/>
				</div>

				<div class="form-group">
					<label for="edit-trial-period">Stripe Trial Period</label>
					<select
						id="edit-trial-period"
						bind:value={editingPlan.trial_period_days}
						class="form-input trial-period-select"
					>
						<option value={null}>None</option>
						<option value={7}>7 days</option>
						<option value={14}>14 days</option>
						<option value={30}>30 days</option>
					</select>
				</div>

				{#if editingPlan.trial_period_days}
					<div class="form-group">
						<label class="checkbox-label">
							<input
								id="edit-trial-requires-pm"
								name="edit-trial-requires-pm"
								type="checkbox"
								bind:checked={editingPlan.trial_requires_payment_method}
							/>
							<span>Require payment method to start trial</span>
						</label>
						<small class="trial-note">
							Uncheck to allow card-free trials (card collected only if trial converts)
						</small>
					</div>
				{/if}

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

<!-- Change Price Modal -->
{#if showPriceModal && priceTargetPlan}
	<div
		class="modal-overlay"
		onclick={(e) => {
			if (e.target === e.currentTarget) closePriceModal();
		}}
		onkeydown={(e) => {
			if (e.key === 'Escape') closePriceModal();
		}}
		role="dialog"
		aria-modal="true"
		aria-label="Change plan price"
		tabindex="-1"
	>
		<div class="modal price-modal" role="document">
			<div class="modal-header">
				<h2>Change Price — {priceTargetPlan.name}</h2>
				<button class="modal-close" onclick={closePriceModal} aria-label="Close">×</button>
			</div>

			<div class="modal-body">
				{#if !showPriceConfirm}
					<div class="form-row">
						<div class="form-group">
							<label for="price-amount">New Price (USD)</label>
							<input
								id="price-amount"
								type="number"
								step="0.01"
								min="0.01"
								bind:value={priceAmount}
								class="form-input"
							/>
							<p class="form-hint">
								Stored as {Math.round(priceAmount * 100)} cents in Stripe.
							</p>
						</div>

						<div class="form-group">
							<label for="price-interval">Billing Interval</label>
							<select id="price-interval" bind:value={priceInterval} class="form-input">
								<option value="month">Monthly</option>
								<option value="year">Yearly</option>
								<option value="one_time">One-time (no recurring)</option>
							</select>
						</div>
					</div>

					<fieldset class="apply-to-group">
						<legend>Who does this new price apply to?</legend>

						<label class={['apply-option', { selected: priceApplyTo === 'new_only' }]}>
							<input type="radio" name="apply-to" value="new_only" bind:group={priceApplyTo} />
							<span class="apply-content">
								<strong>New members only</strong>
								<span class="apply-help">
									Existing subscribers stay on the old price forever (grandfathered). Only checkouts
									started after this change will see the new price.
								</span>
							</span>
						</label>

						<label class={['apply-option', { selected: priceApplyTo === 'next_renewal' }]}>
							<input type="radio" name="apply-to" value="next_renewal" bind:group={priceApplyTo} />
							<span class="apply-content">
								<strong>Everyone, on next renewal</strong>
								<span class="apply-help">
									Existing subscribers move to the new price at their next billing date. No
									proration. Renewal date is preserved.
								</span>
							</span>
						</label>

						<label class={['apply-option', { selected: priceApplyTo === 'immediate_proration' }]}>
							<input
								type="radio"
								name="apply-to"
								value="immediate_proration"
								bind:group={priceApplyTo}
							/>
							<span class="apply-content">
								<strong>Everyone, immediately (with proration)</strong>
								<span class="apply-help">
									Existing subscribers switch right now. Stripe issues a prorated charge or credit
									on their next invoice for the unused portion of the old price.
								</span>
							</span>
						</label>
					</fieldset>

					<div class="price-history-section">
						<h3>Recent Price Changes</h3>
						{#if priceHistoryLoading}
							<p class="form-hint">Loading history…</p>
						{:else if priceHistory.length === 0}
							<p class="form-hint">No previous price changes for this plan.</p>
						{:else}
							<table class="history-table">
								<thead>
									<tr>
										<th>Changed</th>
										<th>From → To</th>
										<th>Apply To</th>
										<th>Migrated</th>
									</tr>
								</thead>
								<tbody>
									{#each priceHistory.slice(0, 5) as h (h.id)}
										<tr>
											<td>{new Date(h.changed_at).toLocaleString()}</td>
											<td class="price-history-amount">
												{h.old_amount_cents !== null
													? `${(h.old_amount_cents / 100).toFixed(2)}`
													: '—'}
												→ ${(h.new_amount_cents / 100).toFixed(2)}
											</td>
											<td>{h.apply_to.replace('_', ' ')}</td>
											<td>
												{h.subscriptions_migrated}
												{#if h.subscriptions_failed > 0}
													<span class="failure-badge">+{h.subscriptions_failed} failed</span>
												{/if}
											</td>
										</tr>
									{/each}
								</tbody>
							</table>
						{/if}
					</div>
				{:else}
					<!-- Confirmation step -->
					<div class="confirm-block">
						<h3>Confirm price change</h3>
						<dl class="confirm-list">
							<dt>Plan</dt>
							<dd>{priceTargetPlan.name}</dd>
							<dt>New price</dt>
							<dd>${priceAmount.toFixed(2)} / {priceInterval}</dd>
							<dt>Apply to</dt>
							<dd>
								{#if priceApplyTo === 'new_only'}
									New members only (existing subscribers grandfathered)
								{:else if priceApplyTo === 'next_renewal'}
									Everyone, on next renewal (no proration)
								{:else}
									Everyone, immediately (Stripe issues proration)
								{/if}
							</dd>
						</dl>
						{#if priceApplyTo === 'immediate_proration'}
							<p class="warn-text">
								Heads up: Stripe will charge or credit existing subscribers right away. Make sure
								your CS team is briefed.
							</p>
						{/if}
					</div>
				{/if}
			</div>

			<div class="modal-footer">
				{#if !showPriceConfirm}
					<button class="btn-cancel" onclick={closePriceModal} disabled={priceSubmitting}>
						Cancel
					</button>
					<button
						class="btn-save"
						onclick={() => (showPriceConfirm = true)}
						disabled={priceSubmitting || priceAmount <= 0}
					>
						Continue
					</button>
				{:else}
					<button
						class="btn-cancel"
						onclick={() => (showPriceConfirm = false)}
						disabled={priceSubmitting}
					>
						Back
					</button>
					<button
						class={['btn-save', { 'btn-destructive': priceApplyTo === 'immediate_proration' }]}
						onclick={submitPriceChange}
						disabled={priceSubmitting}
					>
						{#if priceSubmitting}
							Applying…
						{:else}
							Apply Price Change
						{/if}
					</button>
				{/if}
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

	.background-effects {
		position: fixed;
		inset: 0;
		pointer-events: none;
		overflow: hidden;
	}

	.background-blob {
		position: absolute;
		border-radius: 50%;
		filter: blur(80px);
		opacity: 0.12;
	}

	.background-blob-1 {
		width: 500px;
		height: 500px;
		top: -150px;
		right: -150px;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
	}

	.background-blob-2 {
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

	.banner-close {
		margin-left: auto;
		border: 0;
		background: transparent;
		color: #fca5a5;
		font-size: 1.25rem;
		line-height: 1;
		cursor: pointer;
		transition: color 0.2s;
	}

	.banner-close:hover,
	.banner-close:focus-visible {
		color: #fff;
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

	.stat-icon-total {
		background: rgba(59, 130, 246, 0.1);
		color: #60a5fa;
	}

	.stat-icon-active {
		background: rgba(16, 185, 129, 0.1);
		color: #34d399;
	}

	.stat-icon-stripe {
		background: rgba(168, 85, 247, 0.1);
		color: #c084fc;
	}

	.stat-icon-warning {
		background: rgba(249, 115, 22, 0.1);
		color: #fb923c;
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

	.billing-monthly {
		background: rgba(59, 130, 246, 0.1);
		border-color: rgba(59, 130, 246, 0.2);
		color: #60a5fa;
	}

	.billing-quarterly {
		background: rgba(168, 85, 247, 0.1);
		border-color: rgba(168, 85, 247, 0.2);
		color: #c084fc;
	}

	.billing-annual {
		background: rgba(16, 185, 129, 0.1);
		border-color: rgba(16, 185, 129, 0.2);
		color: #34d399;
	}

	.billing-default {
		background: rgba(100, 116, 139, 0.1);
		border-color: rgba(100, 116, 139, 0.2);
		color: #94a3b8;
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

	.row-actions {
		display: flex;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.btn-price {
		padding: 0.5rem 1rem;
		background: rgba(16, 185, 129, 0.15);
		border: 1px solid rgba(16, 185, 129, 0.3);
		border-radius: 6px;
		color: #6ee7b7;
		font-size: 0.85rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-price:hover {
		background: rgba(16, 185, 129, 0.25);
		color: white;
	}

	/* Price-change modal */
	.price-modal {
		max-width: 720px;
	}

	.apply-to-group {
		border: 1px solid rgba(71, 85, 105, 0.4);
		border-radius: 10px;
		padding: 1rem 1.25rem;
		margin: 1rem 0 0;
	}

	.apply-to-group legend {
		font-size: 0.85rem;
		font-weight: 600;
		color: #cbd5e1;
		padding: 0 0.5rem;
	}

	.apply-option {
		display: flex;
		gap: 0.75rem;
		padding: 0.75rem;
		border-radius: 8px;
		cursor: pointer;
		border: 1px solid transparent;
		transition:
			background 0.15s,
			border-color 0.15s;
	}

	.apply-option:hover {
		background: rgba(99, 102, 241, 0.06);
	}

	.apply-option.selected {
		background: rgba(99, 102, 241, 0.12);
		border-color: rgba(99, 102, 241, 0.4);
	}

	.apply-content {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.apply-content strong {
		color: #f1f5f9;
		font-size: 0.95rem;
	}

	.apply-help {
		color: #94a3b8;
		font-size: 0.85rem;
		line-height: 1.45;
	}

	.price-history-section {
		margin-top: 1.5rem;
	}

	.price-history-section h3 {
		font-size: 0.95rem;
		font-weight: 600;
		color: #e2e8f0;
		margin: 0 0 0.5rem;
	}

	.history-table {
		width: 100%;
		border-collapse: collapse;
		font-size: 0.85rem;
	}

	.history-table th,
	.history-table td {
		text-align: left;
		padding: 0.5rem 0.6rem;
		border-bottom: 1px solid rgba(51, 65, 85, 0.5);
		color: #cbd5e1;
	}

	.history-table th {
		color: #94a3b8;
		font-weight: 500;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		font-size: 0.7rem;
	}

	.failure-badge {
		margin-left: 0.25rem;
		color: #f87171;
		font-size: 0.75rem;
	}

	.confirm-block dl.confirm-list {
		display: grid;
		grid-template-columns: 140px 1fr;
		gap: 0.5rem 1rem;
		margin: 1rem 0;
	}

	.confirm-block dt {
		color: #94a3b8;
		font-size: 0.85rem;
	}

	.confirm-block dd {
		margin: 0;
		color: #f1f5f9;
		font-size: 0.95rem;
	}

	.warn-text {
		color: #fbbf24;
		background: rgba(251, 191, 36, 0.1);
		border: 1px solid rgba(251, 191, 36, 0.3);
		border-radius: 8px;
		padding: 0.75rem 1rem;
		font-size: 0.875rem;
	}

	.btn-destructive {
		background: linear-gradient(135deg, #ef4444, #b91c1c);
		border: 1px solid rgba(220, 38, 38, 0.5);
		color: #fff;
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

	.form-input.stripe-price-input,
	.price-history-amount {
		font-family: monospace;
	}

	.trial-days-input {
		max-width: 120px;
	}

	.trial-period-select {
		max-width: 160px;
	}

	.trial-note {
		display: block;
		margin-top: 4px;
		color: var(--color-text-muted, #888);
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
	@media (max-width: 767.98px) {
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

<!--
	/admin/memberships - Membership Plan Management
	Apple Principal Engineer ICT 7 Grade - January 2026

	Features:
	- Full membership plan CRUD operations
	- Plan pricing and billing cycle management
	- Feature list management per plan
	- Status toggling (active/inactive)
	- Subscriber statistics per plan
	- Full Svelte 5 $state/$derived/$effect reactivity
-->

<script lang="ts">
	/**
	 * Membership Plans Management - Apple ICT 7 Principal Engineer Grade
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Comprehensive membership plan management with:
	 * - Plan listing with stats and status
	 * - Create, edit, and delete operations
	 * - Feature management per plan
	 * - Pricing and billing configuration
	 * - Subscriber analytics
	 *
	 * @version 1.0.0 (January 2026)
	 */

	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import {
		IconCrown,
		IconPlus,
		IconSearch,
		IconFilter,
		IconEdit,
		IconEye,
		IconTrash,
		IconCheck,
		IconX,
		IconChevronDown,
		IconRefresh,
		IconUsers,
		IconCurrencyDollar,
		IconStar,
		IconToggleLeft,
		IconToggleRight,
		IconCopy,
		IconChartBar
	} from '$lib/icons';
	import { adminFetch } from '$lib/utils/adminFetch';

	// ═══════════════════════════════════════════════════════════════════════════
	// TYPES
	// ═══════════════════════════════════════════════════════════════════════════

	interface MembershipFeature {
		feature_code: string;
		feature_name: string;
		description?: string;
	}

	interface MembershipPlan {
		id: string;
		name: string;
		slug: string;
		description?: string;
		price: number;
		billing_cycle: 'monthly' | 'quarterly' | 'annual';
		is_active: boolean;
		features: MembershipFeature[];
		subscriber_count?: number;
		revenue?: number;
		created_at: string;
		updated_at: string;
	}

	interface PlanStats {
		total_plans: number;
		active_plans: number;
		total_subscribers: number;
		total_mrr: number;
		top_plan?: string;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE - Svelte 5 Runes
	// ═══════════════════════════════════════════════════════════════════════════

	let plans = $state<MembershipPlan[]>([]);
	let stats = $state<PlanStats>({
		total_plans: 0,
		active_plans: 0,
		total_subscribers: 0,
		total_mrr: 0
	});
	let isLoading = $state(true);
	let error = $state('');
	let searchQuery = $state('');
	let selectedStatus = $state<string>('all');
	let selectedCycle = $state<string>('all');
	let sortBy = $state<string>('created_at');
	let sortOrder = $state<'asc' | 'desc'>('desc');
	let showFilters = $state(false);

	// Modal states
	let showEditModal = $state(false);
	let showDeleteModal = $state(false);
	let showPreviewModal = $state(false);
	let editingPlan = $state<MembershipPlan | null>(null);
	let deletingPlan = $state<MembershipPlan | null>(null);
	let previewingPlan = $state<MembershipPlan | null>(null);

	// Form state
	let formData = $state({
		name: '',
		slug: '',
		description: '',
		price: 0,
		billing_cycle: 'monthly' as 'monthly' | 'quarterly' | 'annual',
		is_active: true,
		features: [{ feature_code: 'feature_1', feature_name: '', description: '' }] as MembershipFeature[]
	});
	let formLoading = $state(false);
	let formError = $state('');

	// ═══════════════════════════════════════════════════════════════════════════
	// CONSTANTS
	// ═══════════════════════════════════════════════════════════════════════════

	const statusOptions = [
		{ value: 'all', label: 'All Plans' },
		{ value: 'active', label: 'Active' },
		{ value: 'inactive', label: 'Inactive' }
	];

	const cycleOptions = [
		{ value: 'all', label: 'All Cycles' },
		{ value: 'monthly', label: 'Monthly' },
		{ value: 'quarterly', label: 'Quarterly' },
		{ value: 'annual', label: 'Annual' }
	];

	const sortOptions = [
		{ value: 'created_at', label: 'Date Created' },
		{ value: 'name', label: 'Name' },
		{ value: 'price', label: 'Price' },
		{ value: 'subscriber_count', label: 'Subscribers' }
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE - Svelte 5 $derived
	// ═══════════════════════════════════════════════════════════════════════════

	let filteredPlans = $derived.by(() => {
		let result = [...plans];

		// Search filter
		if (searchQuery.trim()) {
			const query = searchQuery.toLowerCase();
			result = result.filter(
				(plan) =>
					plan.name?.toLowerCase().includes(query) ||
					plan.slug?.toLowerCase().includes(query) ||
					plan.description?.toLowerCase().includes(query)
			);
		}

		// Status filter
		if (selectedStatus !== 'all') {
			const isActive = selectedStatus === 'active';
			result = result.filter((plan) => plan.is_active === isActive);
		}

		// Billing cycle filter
		if (selectedCycle !== 'all') {
			result = result.filter((plan) => plan.billing_cycle === selectedCycle);
		}

		// Sorting
		result.sort((a, b) => {
			let aVal: any = a[sortBy as keyof MembershipPlan];
			let bVal: any = b[sortBy as keyof MembershipPlan];

			if (sortBy === 'name') {
				aVal = aVal?.toLowerCase() || '';
				bVal = bVal?.toLowerCase() || '';
			}

			if (aVal === undefined || aVal === null) aVal = sortOrder === 'asc' ? Infinity : -Infinity;
			if (bVal === undefined || bVal === null) bVal = sortOrder === 'asc' ? Infinity : -Infinity;

			if (sortOrder === 'asc') {
				return aVal > bVal ? 1 : -1;
			}
			return aVal < bVal ? 1 : -1;
		});

		return result;
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// EFFECTS - Svelte 5 $effect
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		// Recalculate stats when plans change
		if (plans.length > 0) {
			stats = {
				total_plans: plans.length,
				active_plans: plans.filter((p) => p.is_active).length,
				total_subscribers: plans.reduce((sum, p) => sum + (p.subscriber_count || 0), 0),
				total_mrr: plans.reduce((sum, p) => {
					if (!p.is_active) return sum;
					const subscribers = p.subscriber_count || 0;
					let monthlyPrice = p.price;
					if (p.billing_cycle === 'quarterly') monthlyPrice = p.price / 3;
					if (p.billing_cycle === 'annual') monthlyPrice = p.price / 12;
					return sum + (monthlyPrice * subscribers);
				}, 0),
				top_plan: plans.reduce((top, p) =>
					(p.subscriber_count || 0) > (top?.subscriber_count || 0) ? p : top
				, plans[0])?.name
			};
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// API FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	async function loadData() {
		isLoading = true;
		error = '';

		try {
			const data = await adminFetch('/api/admin/membership-plans');
			plans = data?.data || data?.plans || data || [];
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load membership plans';
			console.error('Load plans error:', err);
		} finally {
			isLoading = false;
		}
	}

	async function updatePlan() {
		if (!editingPlan) return;
		formLoading = true;
		formError = '';

		try {
			const validFeatures = formData.features.filter((f) => f.feature_name.trim());
			const payload = {
				name: formData.name.trim(),
				slug: formData.slug.trim(),
				description: formData.description.trim(),
				price: formData.price,
				billing_cycle: formData.billing_cycle,
				is_active: formData.is_active,
				features: validFeatures
			};

			await adminFetch(`/api/admin/membership-plans/${editingPlan.id}`, {
				method: 'PUT',
				body: JSON.stringify(payload)
			});
			showEditModal = false;
			editingPlan = null;
			resetForm();
			await loadData();
		} catch (err) {
			formError = err instanceof Error ? err.message : 'Failed to update plan';
			console.error('Update plan error:', err);
		} finally {
			formLoading = false;
		}
	}

	async function deletePlan() {
		if (!deletingPlan) return;

		try {
			await adminFetch(`/api/admin/membership-plans/${deletingPlan.id}`, {
				method: 'DELETE'
			});
			showDeleteModal = false;
			deletingPlan = null;
			await loadData();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to delete plan';
			console.error('Delete plan error:', err);
		}
	}

	async function togglePlanStatus(plan: MembershipPlan) {
		try {
			await adminFetch(`/api/admin/membership-plans/${plan.id}`, {
				method: 'PATCH',
				body: JSON.stringify({ is_active: !plan.is_active })
			});
			await loadData();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to update plan status';
			console.error('Toggle status error:', err);
		}
	}

	async function duplicatePlan(plan: MembershipPlan) {
		try {
			const payload = {
				name: `${plan.name} (Copy)`,
				slug: `${plan.slug}-copy`,
				description: plan.description,
				price: plan.price,
				billing_cycle: plan.billing_cycle,
				is_active: false,
				features: plan.features
			};

			await adminFetch('/api/admin/membership-plans', {
				method: 'POST',
				body: JSON.stringify(payload)
			});
			await loadData();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to duplicate plan';
			console.error('Duplicate plan error:', err);
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// HELPERS
	// ═══════════════════════════════════════════════════════════════════════════

	function resetForm() {
		formData = {
			name: '',
			slug: '',
			description: '',
			price: 0,
			billing_cycle: 'monthly',
			is_active: true,
			features: [{ feature_code: 'feature_1', feature_name: '', description: '' }]
		};
		formError = '';
	}

	function openEditModal(plan: MembershipPlan) {
		editingPlan = plan;
		formData = {
			name: plan.name,
			slug: plan.slug,
			description: plan.description || '',
			price: plan.price,
			billing_cycle: plan.billing_cycle,
			is_active: plan.is_active,
			features: plan.features.length > 0
				? [...plan.features]
				: [{ feature_code: 'feature_1', feature_name: '', description: '' }]
		};
		showEditModal = true;
	}

	function openDeleteModal(plan: MembershipPlan) {
		deletingPlan = plan;
		showDeleteModal = true;
	}

	function openPreviewModal(plan: MembershipPlan) {
		previewingPlan = plan;
		showPreviewModal = true;
	}

	function addFeature() {
		formData.features = [
			...formData.features,
			{
				feature_code: `feature_${formData.features.length + 1}`,
				feature_name: '',
				description: ''
			}
		];
	}

	function removeFeature(index: number) {
		formData.features = formData.features.filter((_, i) => i !== index);
	}

	function formatCurrency(amount: number): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD',
			minimumFractionDigits: 0
		}).format(amount);
	}

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric'
		});
	}

	function getBillingLabel(cycle: string): string {
		return {
			monthly: '/month',
			quarterly: '/quarter',
			annual: '/year'
		}[cycle] || '/month';
	}

	function getBillingBadge(cycle: string): string {
		return {
			monthly: 'Monthly',
			quarterly: 'Quarterly',
			annual: 'Annual'
		}[cycle] || cycle;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// LIFECYCLE
	// ═══════════════════════════════════════════════════════════════════════════

	onMount(() => {
		loadData();
	});
</script>

<svelte:head>
	<title>Membership Plans | Admin Dashboard</title>
</svelte:head>

<div class="page">
	<div class="admin-page-container">
	<!-- Page Header - Centered -->
	<div class="page-header">
		<h1>Membership Plans</h1>
		<p class="subtitle">Manage subscription tiers, pricing, and plan features</p>
		<div class="header-actions">
			<button class="btn-secondary" onclick={loadData} disabled={isLoading}>
				<IconRefresh size={18} />
				Refresh
			</button>
			<button class="btn-primary" onclick={() => goto('/admin/memberships/create')}>
				<IconPlus size={18} />
				Add Plan
			</button>
		</div>
	</div>

	<!-- Stats Overview -->
	<section class="stats-grid">
		<div class="stat-card highlight">
			<div class="stat-icon amber">
				<IconCrown size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{stats.total_plans}</span>
				<span class="stat-label">Total Plans</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon emerald">
				<IconCheck size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{stats.active_plans}</span>
				<span class="stat-label">Active Plans</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon blue">
				<IconUsers size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{stats.total_subscribers.toLocaleString()}</span>
				<span class="stat-label">Total Subscribers</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon purple">
				<IconCurrencyDollar size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatCurrency(stats.total_mrr)}</span>
				<span class="stat-label">Monthly Revenue</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon cyan">
				<IconStar size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value truncate">{stats.top_plan || 'N/A'}</span>
				<span class="stat-label">Top Plan</span>
			</div>
		</div>
	</section>

	<!-- Search and Filter Bar -->
	<div class="search-filter-bar">
		<div class="search-box">
			<IconSearch size={18} />
			<input
				type="text"
				placeholder="Search plans..."
				bind:value={searchQuery}
				class="search-input"
			/>
		</div>
		<button class="btn-filter" onclick={() => (showFilters = !showFilters)}>
			<IconFilter size={18} />
			Filters
			<IconChevronDown size={16} class={showFilters ? 'rotate' : ''} />
		</button>
	</div>

	<!-- Filters Panel -->
	{#if showFilters}
		<div class="filters-panel">
			<div class="filter-group">
				<label for="filter-status">Status</label>
				<select id="filter-status" bind:value={selectedStatus}>
					{#each statusOptions as option}
						<option value={option.value}>{option.label}</option>
					{/each}
				</select>
			</div>
			<div class="filter-group">
				<label for="filter-cycle">Billing Cycle</label>
				<select id="filter-cycle" bind:value={selectedCycle}>
					{#each cycleOptions as option}
						<option value={option.value}>{option.label}</option>
					{/each}
				</select>
			</div>
			<div class="filter-group">
				<label for="filter-sort">Sort By</label>
				<select id="filter-sort" bind:value={sortBy}>
					{#each sortOptions as option}
						<option value={option.value}>{option.label}</option>
					{/each}
				</select>
			</div>
			<div class="filter-group">
				<label for="filter-order">Order</label>
				<select id="filter-order" bind:value={sortOrder}>
					<option value="desc">Highest First</option>
					<option value="asc">Lowest First</option>
				</select>
			</div>
			<button
				class="btn-clear-filters"
				onclick={() => {
					selectedStatus = 'all';
					selectedCycle = 'all';
					sortBy = 'created_at';
					sortOrder = 'desc';
					searchQuery = '';
				}}
			>
				Clear Filters
			</button>
		</div>
	{/if}

	<!-- Plans Grid -->
	<div class="plans-container">
		{#if isLoading}
			<div class="loading-state">
				<div class="spinner"></div>
				<p>Loading membership plans...</p>
			</div>
		{:else if error}
			<div class="error-state">
				<p>{error}</p>
				<button onclick={loadData}>Try Again</button>
			</div>
		{:else if filteredPlans.length === 0}
			<div class="empty-state">
				<IconCrown size={48} />
				<h3>No membership plans found</h3>
				<p>Create your first membership plan to start monetizing your content</p>
				<button class="btn-primary" onclick={() => goto('/admin/memberships/create')}>
					<IconPlus size={18} />
					Add Plan
				</button>
			</div>
		{:else}
			<div class="plans-grid">
				{#each filteredPlans as plan (plan.id)}
					<div class="plan-card" class:inactive={!plan.is_active}>
						<div class="plan-header">
							<div class="plan-status">
								<button
									class="status-toggle"
									class:active={plan.is_active}
									onclick={() => togglePlanStatus(plan)}
									title={plan.is_active ? 'Deactivate plan' : 'Activate plan'}
								>
									{#if plan.is_active}
										<IconToggleRight size={24} />
									{:else}
										<IconToggleLeft size={24} />
									{/if}
								</button>
								<span class="status-label" class:active={plan.is_active}>
									{plan.is_active ? 'Active' : 'Inactive'}
								</span>
							</div>
							<div class="plan-actions">
								<button class="btn-icon" title="Preview" onclick={() => openPreviewModal(plan)}>
									<IconEye size={16} />
								</button>
								<button class="btn-icon" title="Edit" onclick={() => openEditModal(plan)}>
									<IconEdit size={16} />
								</button>
								<button class="btn-icon" title="Duplicate" onclick={() => duplicatePlan(plan)}>
									<IconCopy size={16} />
								</button>
								<button class="btn-icon danger" title="Delete" onclick={() => openDeleteModal(plan)}>
									<IconTrash size={16} />
								</button>
							</div>
						</div>

						<div class="plan-body">
							<div class="plan-crown">
								<IconCrown size={28} />
							</div>
							<h3 class="plan-name">{plan.name}</h3>
							<div class="plan-pricing">
								<span class="plan-price">{formatCurrency(plan.price)}</span>
								<span class="plan-cycle">{getBillingLabel(plan.billing_cycle)}</span>
							</div>
							<span class="cycle-badge">{getBillingBadge(plan.billing_cycle)}</span>

							{#if plan.description}
								<p class="plan-description">{plan.description}</p>
							{/if}

							<div class="plan-features">
								{#each plan.features.slice(0, 3) as feature}
									<div class="feature-item">
										<IconCheck size={14} />
										<span>{feature.feature_name}</span>
									</div>
								{/each}
								{#if plan.features.length > 3}
									<div class="feature-more">+{plan.features.length - 3} more features</div>
								{/if}
							</div>
						</div>

						<div class="plan-footer">
							<div class="plan-stat">
								<IconUsers size={16} />
								<span>{plan.subscriber_count || 0} subscribers</span>
							</div>
							<div class="plan-stat">
								<IconChartBar size={16} />
								<span>{formatCurrency(plan.revenue || 0)} revenue</span>
							</div>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</div>
	</div><!-- End admin-page-container -->
</div>

<!-- Edit Plan Modal -->
{#if showEditModal && editingPlan}
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions a11y_click_events_have_key_events -->
	<div class="modal-overlay" onclick={() => (showEditModal = false)} onkeydown={(e) => e.key === 'Escape' && (showEditModal = false)} role="dialog" aria-modal="true" tabindex="-1">
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<div
			class="modal modal-large"
			onclick={(e) => e.stopPropagation()}
			role="document"
		>
			<div class="modal-header">
				<h3>
					<IconEdit size={20} />
					Edit Membership Plan
				</h3>
				<button class="modal-close" onclick={() => (showEditModal = false)}>
					<IconX size={20} />
				</button>
			</div>
			<div class="modal-body">
				{#if formError}
					<div class="form-error">{formError}</div>
				{/if}
				<div class="form-grid">
					<div class="form-group">
						<label for="edit_name">Plan Name *</label>
						<input
							id="edit_name"
							type="text"
							bind:value={formData.name}
							required
						/>
					</div>
					<div class="form-group">
						<label for="edit_slug">URL Slug *</label>
						<input
							id="edit_slug"
							type="text"
							bind:value={formData.slug}
							required
						/>
					</div>
					<div class="form-group full-width">
						<label for="edit_description">Description</label>
						<textarea
							id="edit_description"
							rows="2"
							bind:value={formData.description}
						></textarea>
					</div>
					<div class="form-group">
						<label for="edit_price">Price (USD) *</label>
						<input
							id="edit_price"
							type="number"
							min="0"
							step="0.01"
							bind:value={formData.price}
							required
						/>
					</div>
					<div class="form-group">
						<label for="edit_cycle">Billing Cycle</label>
						<select id="edit_cycle" bind:value={formData.billing_cycle}>
							<option value="monthly">Monthly</option>
							<option value="quarterly">Quarterly</option>
							<option value="annual">Annual</option>
						</select>
					</div>
					<div class="form-group full-width">
						<label class="checkbox-label">
							<input type="checkbox" bind:checked={formData.is_active} />
							<span>Active (visible to customers)</span>
						</label>
					</div>

					<!-- Features Section -->
					<div class="form-group full-width">
						<div class="features-header">
							<span class="label-text">Features</span>
							<button type="button" class="btn-add-feature" onclick={addFeature}>
								<IconPlus size={14} />
								Add Feature
							</button>
						</div>
						<div class="features-list-edit">
							{#each formData.features as feature, index}
								<div class="feature-row">
									<input
										type="text"
										bind:value={feature.feature_name}
										placeholder="Feature name"
										class="feature-input"
									/>
									<input
										type="text"
										bind:value={feature.description}
										placeholder="Description (optional)"
										class="feature-desc"
									/>
									{#if formData.features.length > 1}
										<button
											type="button"
											class="btn-remove-feature"
											onclick={() => removeFeature(index)}
										>
											<IconX size={14} />
										</button>
									{/if}
								</div>
							{/each}
						</div>
					</div>
				</div>
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showEditModal = false)}>
					Cancel
				</button>
				<button
					class="btn-primary"
					onclick={updatePlan}
					disabled={formLoading || !formData.name || !formData.slug}
				>
					{#if formLoading}
						Saving...
					{:else}
						<IconCheck size={18} />
						Save Changes
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Delete Confirmation Modal -->
{#if showDeleteModal && deletingPlan}
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions a11y_click_events_have_key_events -->
	<div class="modal-overlay" onclick={() => (showDeleteModal = false)} onkeydown={(e) => e.key === 'Escape' && (showDeleteModal = false)} role="dialog" aria-modal="true" tabindex="-1">
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<div
			class="modal modal-small"
			onclick={(e) => e.stopPropagation()}
			role="document"
		>
			<div class="modal-header">
				<h3>
					<IconTrash size={20} />
					Delete Plan
				</h3>
				<button class="modal-close" onclick={() => (showDeleteModal = false)}>
					<IconX size={20} />
				</button>
			</div>
			<div class="modal-body">
				<p class="confirm-text">
					Are you sure you want to delete <strong>{deletingPlan.name}</strong>?
					{#if deletingPlan.subscriber_count && deletingPlan.subscriber_count > 0}
						<br /><br />
						<span class="warning-text">Warning: This plan has {deletingPlan.subscriber_count} active subscribers!</span>
					{/if}
				</p>
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showDeleteModal = false)}>
					Cancel
				</button>
				<button class="btn-danger" onclick={deletePlan}>
					<IconTrash size={18} />
					Delete Plan
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Preview Modal -->
{#if showPreviewModal && previewingPlan}
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions a11y_click_events_have_key_events -->
	<div class="modal-overlay" onclick={() => (showPreviewModal = false)} onkeydown={(e) => e.key === 'Escape' && (showPreviewModal = false)} role="dialog" aria-modal="true" tabindex="-1">
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<div
			class="modal modal-preview"
			onclick={(e) => e.stopPropagation()}
			role="document"
		>
			<div class="modal-header">
				<h3>
					<IconEye size={20} />
					Plan Preview
				</h3>
				<button class="modal-close" onclick={() => (showPreviewModal = false)}>
					<IconX size={20} />
				</button>
			</div>
			<div class="modal-body preview-body">
				<div class="preview-card">
					<div class="preview-crown">
						<IconCrown size={32} />
					</div>
					<h4 class="preview-name">{previewingPlan.name}</h4>
					<div class="preview-pricing">
						<span class="preview-price">{formatCurrency(previewingPlan.price)}</span>
						<span class="preview-cycle">{getBillingLabel(previewingPlan.billing_cycle)}</span>
					</div>
					{#if previewingPlan.description}
						<p class="preview-description">{previewingPlan.description}</p>
					{/if}
					<div class="preview-features">
						{#each previewingPlan.features as feature}
							<div class="preview-feature">
								<IconCheck size={16} />
								<span>{feature.feature_name}</span>
							</div>
						{/each}
					</div>
					<button class="preview-btn" disabled>
						Subscribe Now
					</button>
				</div>
			</div>
		</div>
	</div>
{/if}

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   MEMBERSHIPS PAGE - Email Templates Styling
	   ═══════════════════════════════════════════════════════════════════════════ */

	.page {
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
	}

	/* Page Header - Centered */
	.page-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0;
	}

	.subtitle {
		color: #64748b;
		font-size: 0.875rem;
		margin: 0.25rem 0 0;
	}

	.header-actions {
		display: flex;
		justify-content: center;
		gap: 0.75rem;
		margin-top: 1rem;
	}

	/* Stats Grid */
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(5, 1fr);
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.stat-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem;
		background: rgba(30, 41, 59, 0.4);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 8px;
		transition: all 0.2s;
	}

	.stat-card:hover {
		border-color: rgba(99, 102, 241, 0.3);
	}

	.stat-card.highlight {
		background: rgba(99, 102, 241, 0.1);
		border-color: rgba(99, 102, 241, 0.2);
	}

	.stat-icon {
		width: 44px;
		height: 44px;
		border-radius: 8px;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.stat-icon.amber { background: rgba(99, 102, 241, 0.15); color: #a5b4fc; }
	.stat-icon.emerald { background: rgba(34, 197, 94, 0.15); color: #22c55e; }
	.stat-icon.blue { background: rgba(59, 130, 246, 0.15); color: #60a5fa; }
	.stat-icon.purple { background: rgba(139, 92, 246, 0.15); color: #a78bfa; }
	.stat-icon.cyan { background: rgba(6, 182, 212, 0.15); color: #22d3ee; }

	.stat-content {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
		min-width: 0;
	}

	.stat-value {
		font-size: 1.25rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.stat-value.truncate {
		font-size: 0.875rem;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.stat-label {
		font-size: 0.75rem;
		color: #64748b;
	}

	/* Search and Filter Bar */
	.search-filter-bar {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 1rem;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		position: relative;
	}

	.search-box :global(svg) {
		position: absolute;
		left: 0.75rem;
		color: #64748b;
	}

	.search-input {
		width: 280px;
		padding: 0.625rem 1rem 0.625rem 2.5rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 0.875rem;
	}

	.search-input:focus {
		outline: none;
		border-color: rgba(99, 102, 241, 0.5);
	}

	.search-input::placeholder {
		color: #64748b;
	}

	.btn-filter {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.4rem 0.75rem;
		background: rgba(100, 116, 139, 0.2);
		border: 1px solid rgba(100, 116, 139, 0.3);
		border-radius: 4px;
		color: #cbd5e1;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-filter:hover {
		opacity: 0.9;
	}

	.btn-filter :global(.rotate) {
		transform: rotate(180deg);
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 0.5rem 1rem;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		color: white;
		border: none;
		border-radius: 6px;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover:not(:disabled) {
		opacity: 0.9;
	}

	.btn-primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* Filters Panel */
	.filters-panel {
		display: flex;
		justify-content: center;
		align-items: flex-end;
		gap: 1rem;
		padding: 1rem;
		margin-bottom: 1rem;
		background: rgba(30, 41, 59, 0.4);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 8px;
		flex-wrap: wrap;
	}

	.filter-group {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
		min-width: 140px;
	}

	.filter-group label {
		font-size: 0.75rem;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.filter-group select {
		padding: 0.5rem 0.75rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 6px;
		color: #f1f5f9;
		font-size: 0.875rem;
		cursor: pointer;
	}

	.filter-group select:focus {
		outline: none;
		border-color: rgba(99, 102, 241, 0.5);
	}

	.btn-clear-filters {
		padding: 0.5rem 0.75rem;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 6px;
		color: #f87171;
		font-size: 0.8125rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-clear-filters:hover {
		background: rgba(239, 68, 68, 0.2);
	}

	/* Plans Container */
	.plans-container {
		background: rgba(30, 41, 59, 0.4);
		border-radius: 8px;
		padding: 1.5rem;
	}

	.plans-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
		gap: 1.25rem;
	}

	/* Plan Card */
	.plan-card {
		background: rgba(30, 41, 59, 0.4);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 8px;
		overflow: hidden;
		transition: all 0.2s;
	}

	.plan-card:hover {
		border-color: rgba(99, 102, 241, 0.3);
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.2);
	}

	.plan-card.inactive {
		opacity: 0.7;
	}

	.plan-card.inactive:hover {
		opacity: 1;
	}

	.plan-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.75rem 1rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.plan-status {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.status-toggle {
		display: flex;
		padding: 0.25rem;
		background: transparent;
		border: none;
		color: #64748b;
		cursor: pointer;
		transition: color 0.2s;
	}

	.status-toggle.active {
		color: #22c55e;
	}

	.status-toggle:hover {
		color: #a5b4fc;
	}

	.status-label {
		font-size: 0.75rem;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
	}

	.status-label.active {
		color: #22c55e;
	}

	.plan-actions {
		display: flex;
		gap: 0.25rem;
	}

	.btn-icon {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 6px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-icon:hover {
		background: rgba(99, 102, 241, 0.15);
		border-color: rgba(99, 102, 241, 0.3);
		color: #a5b4fc;
	}

	.btn-icon.danger:hover {
		background: rgba(239, 68, 68, 0.15);
		border-color: rgba(239, 68, 68, 0.3);
		color: #f87171;
	}

	.plan-body {
		padding: 1.5rem 1rem;
		text-align: center;
	}

	.plan-crown {
		display: inline-flex;
		padding: 0.75rem;
		background: rgba(99, 102, 241, 0.15);
		border-radius: 50%;
		color: #a5b4fc;
		margin-bottom: 1rem;
	}

	.plan-name {
		margin: 0 0 0.75rem;
		font-size: 1.125rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.plan-pricing {
		display: flex;
		align-items: baseline;
		justify-content: center;
		gap: 0.25rem;
		margin-bottom: 0.5rem;
	}

	.plan-price {
		font-size: 2rem;
		font-weight: 700;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		background-clip: text;
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
	}

	.plan-cycle {
		font-size: 0.875rem;
		color: #64748b;
	}

	.cycle-badge {
		display: inline-block;
		padding: 0.25rem 0.5rem;
		background: rgba(99, 102, 241, 0.15);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 6px;
		font-size: 0.75rem;
		font-weight: 500;
		color: #a5b4fc;
		text-transform: capitalize;
		margin-bottom: 1rem;
	}

	.plan-description {
		margin: 0 0 1rem;
		font-size: 0.8125rem;
		color: #94a3b8;
		line-height: 1.5;
	}

	.plan-features {
		text-align: left;
		padding: 0 0.5rem;
	}

	.feature-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.375rem 0;
		font-size: 0.8125rem;
		color: #e2e8f0;
	}

	.feature-item :global(svg) {
		color: #a5b4fc;
		flex-shrink: 0;
	}

	.feature-more {
		padding: 0.375rem 0;
		font-size: 0.75rem;
		color: #64748b;
		font-style: italic;
	}

	.plan-footer {
		display: flex;
		justify-content: space-between;
		padding: 0.75rem 1rem;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
		background: rgba(15, 23, 42, 0.3);
	}

	.plan-stat {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.75rem;
		color: #64748b;
	}

	.plan-stat :global(svg) {
		color: #475569;
	}

	/* States */
	.loading-state,
	.error-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		text-align: center;
		color: #94a3b8;
	}

	.empty-state :global(svg) {
		color: #475569;
		margin-bottom: 1rem;
	}

	.empty-state h3 {
		color: #f1f5f9;
		margin: 0 0 0.5rem;
	}

	.empty-state p {
		margin: 0 0 1.5rem;
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

	/* Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.7);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 1.25rem;
	}

	.modal {
		width: 100%;
		max-width: 560px;
		max-height: 90vh;
		overflow-y: auto;
		background: rgba(30, 41, 59, 0.95);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 12px;
	}

	.modal-small {
		max-width: 420px;
	}

	.modal-large {
		max-width: 640px;
	}

	.modal-preview {
		max-width: 400px;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.25rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.modal-header h3 {
		display: flex;
		align-items: center;
		gap: 0.625rem;
		margin: 0;
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
	}

	.modal-header h3 :global(svg) {
		color: #a5b4fc;
	}

	.modal-close {
		display: flex;
		padding: 0.5rem;
		background: transparent;
		border: none;
		color: #64748b;
		cursor: pointer;
		transition: color 0.2s;
	}

	.modal-close:hover {
		color: #f1f5f9;
	}

	.modal-body {
		padding: 1.25rem;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1.25rem;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
	}

	.form-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1rem;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
	}

	.form-group.full-width {
		grid-column: 1 / -1;
	}

	.form-group label {
		font-size: 0.75rem;
		font-weight: 600;
		color: #94a3b8;
	}

	.form-group input,
	.form-group select,
	.form-group textarea {
		padding: 0.625rem 0.875rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 6px;
		color: #f1f5f9;
		font-size: 0.875rem;
		font-family: inherit;
	}

	.form-group input:focus,
	.form-group select:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: rgba(99, 102, 241, 0.5);
	}

	.form-group textarea {
		resize: vertical;
		min-height: 60px;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.625rem;
		cursor: pointer;
		color: #e2e8f0;
	}

	.checkbox-label input {
		width: 18px;
		height: 18px;
	}

	.form-error {
		margin-bottom: 1rem;
		padding: 0.75rem;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 6px;
		color: #f87171;
		font-size: 0.875rem;
	}

	.confirm-text {
		margin: 0;
		font-size: 0.875rem;
		color: #94a3b8;
		line-height: 1.6;
	}

	.confirm-text strong {
		color: #f1f5f9;
	}

	.warning-text {
		color: #f87171;
		font-weight: 600;
	}

	/* Features Edit */
	.features-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.75rem;
	}

	.btn-add-feature {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.75rem;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid rgba(99, 102, 241, 0.3);
		border-radius: 6px;
		color: #a5b4fc;
		font-size: 0.75rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-add-feature:hover {
		background: rgba(99, 102, 241, 0.2);
	}

	.features-list-edit {
		display: flex;
		flex-direction: column;
		gap: 0.625rem;
	}

	.feature-row {
		display: flex;
		gap: 0.625rem;
	}

	.feature-input {
		flex: 1;
		padding: 0.5rem 0.75rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 6px;
		color: #f1f5f9;
		font-size: 0.8125rem;
	}

	.feature-input:focus {
		outline: none;
		border-color: rgba(99, 102, 241, 0.5);
	}

	.feature-desc {
		flex: 1.5;
		padding: 0.5rem 0.75rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 6px;
		color: #f1f5f9;
		font-size: 0.8125rem;
	}

	.feature-desc:focus {
		outline: none;
		border-color: rgba(99, 102, 241, 0.5);
	}

	.btn-remove-feature {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 6px;
		color: #f87171;
		cursor: pointer;
		flex-shrink: 0;
		transition: all 0.2s;
	}

	.btn-remove-feature:hover {
		background: rgba(239, 68, 68, 0.2);
	}

	.btn-secondary {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 0.4rem 0.6rem;
		background: rgba(100, 116, 139, 0.2);
		border: 1px solid rgba(100, 116, 139, 0.3);
		color: #cbd5e1;
		border-radius: 4px;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		opacity: 0.9;
	}

	.btn-danger {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.4rem 0.6rem;
		background: rgba(239, 68, 68, 0.2);
		color: #f87171;
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 4px;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-danger:hover {
		opacity: 0.9;
	}

	/* Preview Modal */
	.preview-body {
		display: flex;
		justify-content: center;
		padding: 1.5rem;
	}

	.preview-card {
		width: 100%;
		max-width: 300px;
		padding: 2rem 1.5rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.3);
		border-radius: 12px;
		text-align: center;
	}

	.preview-crown {
		display: inline-flex;
		padding: 1rem;
		background: rgba(99, 102, 241, 0.15);
		border-radius: 50%;
		color: #a5b4fc;
		margin-bottom: 1.25rem;
	}

	.preview-name {
		margin: 0 0 1rem;
		font-size: 1.25rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.preview-pricing {
		display: flex;
		align-items: baseline;
		justify-content: center;
		gap: 0.375rem;
		margin-bottom: 1.25rem;
	}

	.preview-price {
		font-size: 2rem;
		font-weight: 700;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		background-clip: text;
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
	}

	.preview-cycle {
		font-size: 0.875rem;
		color: #94a3b8;
	}

	.preview-description {
		margin: 0 0 1.5rem;
		font-size: 0.8125rem;
		color: #cbd5e1;
		line-height: 1.6;
	}

	.preview-features {
		text-align: left;
		margin-bottom: 1.5rem;
	}

	.preview-feature {
		display: flex;
		align-items: center;
		gap: 0.625rem;
		padding: 0.5rem 0;
		font-size: 0.8125rem;
		color: #f1f5f9;
	}

	.preview-feature :global(svg) {
		color: #a5b4fc;
		flex-shrink: 0;
	}

	.preview-btn {
		width: 100%;
		padding: 0.75rem;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		color: white;
		border: none;
		border-radius: 8px;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: not-allowed;
		opacity: 0.8;
	}

	/* Responsive */
	@media (max-width: 1400px) {
		.stats-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	@media (max-width: 1024px) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}

		.plans-grid {
			grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		}
	}

	@media (max-width: 768px) {
		.page {
			padding: 1rem;
		}

		.page-header h1 {
			font-size: 1.5rem;
		}

		.header-actions {
			flex-direction: column;
		}

		.stats-grid {
			grid-template-columns: 1fr;
		}

		.search-filter-bar {
			flex-direction: column;
		}

		.search-input {
			width: 100%;
		}

		.filters-panel {
			flex-direction: column;
			align-items: stretch;
		}

		.filter-group {
			width: 100%;
		}

		.plans-grid {
			grid-template-columns: 1fr;
		}

		.form-grid {
			grid-template-columns: 1fr;
		}

		.feature-row {
			flex-direction: column;
		}
	}
</style>

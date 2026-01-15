<!--
	/admin/crm/automations - Automation Funnels Management
	Apple Principal Engineer ICT 7 Grade - January 2026

	Features:
	- Automation funnel CRUD with stats dashboard
	- Trigger-based filtering (contact_created, tag_applied, etc.)
	- Status filtering (draft, active, paused)
	- Duplicate, export, import, delete functionality
	- Completion rate tracking
	- Activate/Pause toggle controls
	- Add contacts to funnel modal
	- Auto-refresh on filter changes via $effect
	- Full Svelte 5 $state/$derived/$effect reactivity
-->

<script lang="ts">
	import { onMount, untrack } from 'svelte';
	import IconRoute from '@tabler/icons-svelte/icons/route';
	import IconPlus from '@tabler/icons-svelte/icons/plus';
	import IconSearch from '@tabler/icons-svelte/icons/search';
	import IconEdit from '@tabler/icons-svelte/icons/edit';
	import IconTrash from '@tabler/icons-svelte/icons/trash';
	import IconEye from '@tabler/icons-svelte/icons/eye';
	import IconPlayerPlay from '@tabler/icons-svelte/icons/player-play';
	import IconPlayerPause from '@tabler/icons-svelte/icons/player-pause';
	import IconCopy from '@tabler/icons-svelte/icons/copy';
	import IconRefresh from '@tabler/icons-svelte/icons/refresh';
	import IconUsers from '@tabler/icons-svelte/icons/users';
	import IconChartBar from '@tabler/icons-svelte/icons/chart-bar';
	import IconDownload from '@tabler/icons-svelte/icons/download';
	import IconUpload from '@tabler/icons-svelte/icons/upload';
	import IconBolt from '@tabler/icons-svelte/icons/bolt';
	import IconShare from '@tabler/icons-svelte/icons/share';
	import IconX from '@tabler/icons-svelte/icons/x';
	import IconCheck from '@tabler/icons-svelte/icons/check';
	import IconAlertCircle from '@tabler/icons-svelte/icons/alert-circle';
	import IconUserPlus from '@tabler/icons-svelte/icons/user-plus';
	import IconFilter from '@tabler/icons-svelte/icons/filter';
	import { crmAPI } from '$lib/api/crm';
	import type { AutomationFunnel, FunnelFilters, FunnelStatus, TriggerType } from '$lib/crm/types';

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let funnels = $state<AutomationFunnel[]>([]);
	let isLoading = $state(true);
	let error = $state('');
	let successMessage = $state('');
	let searchQuery = $state('');
	let selectedStatus = $state<FunnelStatus | 'all'>('all');
	let selectedTrigger = $state<TriggerType | 'all'>('all');
	let isInitialized = $state(false);

	let stats = $state({
		total: 0,
		active: 0,
		totalSubscribers: 0,
		completed: 0
	});

	// Action States
	let actionInProgress = $state<string | null>(null);

	// Modal States
	let showImportModal = $state(false);
	let showAddContactsModal = $state(false);
	let selectedFunnel = $state<AutomationFunnel | null>(null);

	// Import Form State
	let importForm = $state({
		file: null as File | null,
		jsonData: '',
		isLoading: false,
		error: '',
		success: false
	});

	// Add Contacts Form State
	let addContactsForm = $state({
		contactIds: '',
		isLoading: false,
		error: '',
		success: false,
		addedCount: 0
	});

	const statusOptions = [
		{ value: 'all', label: 'All Automations' },
		{ value: 'draft', label: 'Draft' },
		{ value: 'active', label: 'Active' },
		{ value: 'paused', label: 'Paused' }
	] as const;

	const triggerOptions = [
		{ value: 'all', label: 'All Triggers' },
		{ value: 'contact_created', label: 'Contact Created' },
		{ value: 'tag_applied', label: 'Tag Applied' },
		{ value: 'tag_removed', label: 'Tag Removed' },
		{ value: 'list_applied', label: 'List Applied' },
		{ value: 'list_removed', label: 'List Removed' },
		{ value: 'form_submitted', label: 'Form Submitted' },
		{ value: 'order_completed', label: 'Order Completed' },
		{ value: 'subscription_started', label: 'Subscription Started' },
		{ value: 'user_login', label: 'User Login' },
		{ value: 'custom_event', label: 'Custom Event' }
	] as const;

	const triggerLabels: Record<string, string> = {
		contact_created: 'Contact Created',
		tag_applied: 'Tag Applied',
		tag_removed: 'Tag Removed',
		list_applied: 'List Applied',
		list_removed: 'List Removed',
		contact_status_changed: 'Status Changed',
		form_submitted: 'Form Submitted',
		order_completed: 'Order Completed',
		order_refunded: 'Order Refunded',
		subscription_started: 'Subscription Started',
		subscription_cancelled: 'Subscription Cancelled',
		user_login: 'User Login',
		user_registered: 'User Registered',
		birthday: 'Birthday',
		sequence_completed: 'Sequence Completed',
		link_clicked: 'Link Clicked',
		email_opened: 'Email Opened',
		custom_event: 'Custom Event'
	};

	// ═══════════════════════════════════════════════════════════════════════════
	// API FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	async function loadFunnels() {
		isLoading = true;
		error = '';

		try {
			const filters: FunnelFilters = {
				search: searchQuery || undefined,
				status: selectedStatus !== 'all' ? selectedStatus : undefined,
				trigger_type: selectedTrigger !== 'all' ? selectedTrigger : undefined
			};

			const response = await crmAPI.getAutomationFunnels(filters);
			funnels = response.data || [];

			stats = {
				total: funnels.length,
				active: funnels.filter(f => f.status === 'active').length,
				totalSubscribers: funnels.reduce((sum, f) => sum + f.subscribers_count, 0),
				completed: funnels.reduce((sum, f) => sum + f.completed_count, 0)
			};
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load automations';
		} finally {
			isLoading = false;
		}
	}

	async function toggleFunnelStatus(funnel: AutomationFunnel) {
		actionInProgress = funnel.id;
		error = '';

		try {
			const newStatus: FunnelStatus = funnel.status === 'active' ? 'paused' : 'active';
			await crmAPI.updateAutomationFunnel(funnel.id, { status: newStatus });
			showSuccess(`Automation ${newStatus === 'active' ? 'activated' : 'paused'} successfully`);
			await loadFunnels();
		} catch (err) {
			error = err instanceof Error ? err.message : `Failed to ${funnel.status === 'active' ? 'pause' : 'activate'} automation`;
		} finally {
			actionInProgress = null;
		}
	}

	async function duplicateFunnel(id: string) {
		actionInProgress = id;
		error = '';

		try {
			await crmAPI.duplicateAutomationFunnel(id);
			showSuccess('Automation duplicated successfully');
			await loadFunnels();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to duplicate automation';
		} finally {
			actionInProgress = null;
		}
	}

	async function deleteFunnel(id: string) {
		if (!confirm('Are you sure you want to delete this automation? This action cannot be undone.')) return;

		actionInProgress = id;
		error = '';

		try {
			await crmAPI.deleteAutomationFunnel(id);
			showSuccess('Automation deleted successfully');
			await loadFunnels();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to delete automation';
		} finally {
			actionInProgress = null;
		}
	}

	async function exportFunnel(id: string) {
		actionInProgress = id;
		error = '';

		try {
			const data = await crmAPI.exportAutomationFunnel(id);
			const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
			const url = URL.createObjectURL(blob);
			const a = document.createElement('a');
			a.href = url;
			a.download = `automation-${id}-${Date.now()}.json`;
			a.click();
			URL.revokeObjectURL(url);
			showSuccess('Automation exported successfully');
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to export automation';
		} finally {
			actionInProgress = null;
		}
	}

	async function importFunnel() {
		if (!importForm.jsonData.trim()) {
			importForm.error = 'Please paste automation JSON data';
			return;
		}

		importForm.isLoading = true;
		importForm.error = '';
		importForm.success = false;

		try {
			const data = JSON.parse(importForm.jsonData);
			await crmAPI.importAutomationFunnel(data);
			importForm.success = true;
			showSuccess('Automation imported successfully');
			await loadFunnels();
			setTimeout(() => {
				closeImportModal();
			}, 1500);
		} catch (err) {
			if (err instanceof SyntaxError) {
				importForm.error = 'Invalid JSON format. Please check your data.';
			} else {
				importForm.error = err instanceof Error ? err.message : 'Failed to import automation';
			}
		} finally {
			importForm.isLoading = false;
		}
	}

	async function addContactsToFunnel() {
		if (!selectedFunnel || !addContactsForm.contactIds.trim()) {
			addContactsForm.error = 'Please enter contact IDs';
			return;
		}

		addContactsForm.isLoading = true;
		addContactsForm.error = '';
		addContactsForm.success = false;

		try {
			const contactIds = addContactsForm.contactIds
				.split(/[,\n]/)
				.map(id => id.trim())
				.filter(id => id.length > 0);

			if (contactIds.length === 0) {
				addContactsForm.error = 'Please enter valid contact IDs';
				return;
			}

			const result = await crmAPI.addToFunnel(selectedFunnel.id, contactIds);
			addContactsForm.success = true;
			addContactsForm.addedCount = result.added_count;
			showSuccess(`${result.added_count} contact(s) added to automation`);
			await loadFunnels();
			setTimeout(() => {
				closeAddContactsModal();
			}, 2000);
		} catch (err) {
			addContactsForm.error = err instanceof Error ? err.message : 'Failed to add contacts';
		} finally {
			addContactsForm.isLoading = false;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// MODAL FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	function openImportModal() {
		importForm = {
			file: null,
			jsonData: '',
			isLoading: false,
			error: '',
			success: false
		};
		showImportModal = true;
	}

	function closeImportModal() {
		showImportModal = false;
		importForm = {
			file: null,
			jsonData: '',
			isLoading: false,
			error: '',
			success: false
		};
	}

	function openAddContactsModal(funnel: AutomationFunnel) {
		selectedFunnel = funnel;
		addContactsForm = {
			contactIds: '',
			isLoading: false,
			error: '',
			success: false,
			addedCount: 0
		};
		showAddContactsModal = true;
	}

	function closeAddContactsModal() {
		showAddContactsModal = false;
		selectedFunnel = null;
		addContactsForm = {
			contactIds: '',
			isLoading: false,
			error: '',
			success: false,
			addedCount: 0
		};
	}

	function handleModalKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape') {
			if (showImportModal) closeImportModal();
			if (showAddContactsModal) closeAddContactsModal();
		}
	}

	function handleModalBackdropClick(event: MouseEvent) {
		if (event.target === event.currentTarget) {
			if (showImportModal) closeImportModal();
			if (showAddContactsModal) closeAddContactsModal();
		}
	}

	async function handleFileUpload(event: Event) {
		const target = event.target as HTMLInputElement;
		const file = target.files?.[0];

		if (file) {
			importForm.file = file;
			try {
				const text = await file.text();
				importForm.jsonData = text;
				importForm.error = '';
			} catch (err) {
				importForm.error = 'Failed to read file';
			}
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// HELPERS
	// ═══════════════════════════════════════════════════════════════════════════

	function showSuccess(message: string) {
		successMessage = message;
		setTimeout(() => {
			successMessage = '';
		}, 3000);
	}

	function formatNumber(num: number): string {
		return num.toLocaleString();
	}

	function getCompletionRate(funnel: AutomationFunnel): string {
		if (funnel.subscribers_count === 0) return '0%';
		return ((funnel.completed_count / funnel.subscribers_count) * 100).toFixed(1) + '%';
	}

	function getStatusColor(status: FunnelStatus): string {
		const colors: Record<FunnelStatus, string> = {
			draft: 'bg-slate-500/20 text-slate-400',
			active: 'bg-emerald-500/20 text-emerald-400',
			paused: 'bg-amber-500/20 text-amber-400'
		};
		return colors[status];
	}

	function getStatusIcon(status: FunnelStatus) {
		const icons: Record<FunnelStatus, typeof IconEdit> = {
			draft: IconEdit,
			active: IconPlayerPlay,
			paused: IconPlayerPause
		};
		return icons[status];
	}

	function getTriggerColor(trigger: string): string {
		const colorMap: Record<string, string> = {
			contact_created: 'bg-blue-500/20 text-blue-400',
			tag_applied: 'bg-emerald-500/20 text-emerald-400',
			tag_removed: 'bg-red-500/20 text-red-400',
			form_submitted: 'bg-purple-500/20 text-purple-400',
			order_completed: 'bg-amber-500/20 text-amber-400',
			subscription_started: 'bg-cyan-500/20 text-cyan-400'
		};
		return colorMap[trigger] || 'bg-slate-500/20 text-slate-400';
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let filteredFunnels = $derived(
		funnels.filter(funnel => {
			const matchesSearch = !searchQuery ||
				funnel.title.toLowerCase().includes(searchQuery.toLowerCase());
			const matchesStatus = selectedStatus === 'all' || funnel.status === selectedStatus;
			const matchesTrigger = selectedTrigger === 'all' || funnel.trigger_type === selectedTrigger;
			return matchesSearch && matchesStatus && matchesTrigger;
		})
	);

	let canImport = $derived(
		importForm.jsonData.trim().length > 0 && !importForm.isLoading
	);

	let canAddContacts = $derived(
		addContactsForm.contactIds.trim().length > 0 && !addContactsForm.isLoading
	);

	let hasActiveModal = $derived(showImportModal || showAddContactsModal);

	// ═══════════════════════════════════════════════════════════════════════════
	// EFFECTS
	// ═══════════════════════════════════════════════════════════════════════════

	// Auto-refresh when filters change (after initial load)
	$effect(() => {
		// Track the reactive dependencies
		const currentSearch = searchQuery;
		const currentStatus = selectedStatus;
		const currentTrigger = selectedTrigger;

		// Only reload if already initialized (skip the initial load)
		if (isInitialized) {
			// Use untrack to prevent infinite loops when loadFunnels updates state
			untrack(() => {
				loadFunnels();
			});
		}
	});

	// Auto-dismiss success message
	$effect(() => {
		if (successMessage) {
			const timeout = setTimeout(() => {
				successMessage = '';
			}, 3000);
			return () => clearTimeout(timeout);
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// LIFECYCLE
	// ═══════════════════════════════════════════════════════════════════════════

	onMount(async () => {
		await loadFunnels();
		isInitialized = true;
	});
</script>

<svelte:head>
	<title>Automation Funnels - FluentCRM Pro</title>
</svelte:head>

<!-- Handle escape key for modal -->
<svelte:window onkeydown={hasActiveModal ? handleModalKeydown : undefined} />

<div class="page">
	<!-- Header -->
	<div class="page-header">
		<h1>Automation Funnels</h1>
		<p class="subtitle">Create powerful marketing automations triggered by events</p>
		<div class="header-actions">
			<button class="btn-secondary" onclick={openImportModal} title="Import Automation">
				<IconUpload size={18} />
				Import
			</button>
			<button class="btn-refresh" onclick={() => loadFunnels()} disabled={isLoading}>
				<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
			</button>
			<a href="/admin/crm/automations/new" class="btn-primary">
				<IconPlus size={18} />
				New Automation
			</a>
		</div>
	</div>

	<!-- Success Alert -->
	{#if successMessage}
		<div class="success-alert">
			<IconCheck size={18} />
			<span>{successMessage}</span>
			<button onclick={() => successMessage = ''} aria-label="Dismiss">
				<IconX size={16} />
			</button>
		</div>
	{/if}

	<!-- Error Alert -->
	{#if error}
		<div class="error-alert">
			<IconAlertCircle size={18} />
			<span>{error}</span>
			<button onclick={() => error = ''} aria-label="Dismiss error">
				<IconX size={16} />
			</button>
		</div>
	{/if}

	<!-- Stats Cards -->
	<div class="stats-grid">
		<div class="stat-card">
			<div class="stat-icon blue">
				<IconShare size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatNumber(stats.total)}</span>
				<span class="stat-label">Total Automations</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon green">
				<IconBolt size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatNumber(stats.active)}</span>
				<span class="stat-label">Active</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon purple">
				<IconUsers size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatNumber(stats.totalSubscribers)}</span>
				<span class="stat-label">Total Contacts</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon amber">
				<IconChartBar size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatNumber(stats.completed)}</span>
				<span class="stat-label">Completed</span>
			</div>
		</div>
	</div>

	<!-- Search & Filters -->
	<div class="filters-bar">
		<div class="search-box">
			<IconSearch size={18} />
			<input
				type="text"
				placeholder="Search automations..."
				bind:value={searchQuery}
				aria-label="Search automations"
			/>
			{#if searchQuery}
				<button class="search-clear" onclick={() => searchQuery = ''} aria-label="Clear search">
					<IconX size={14} />
				</button>
			{/if}
		</div>
		<select class="filter-select" bind:value={selectedStatus} aria-label="Filter by status">
			{#each statusOptions as option}
				<option value={option.value}>{option.label}</option>
			{/each}
		</select>
		<select class="filter-select" bind:value={selectedTrigger} aria-label="Filter by trigger">
			{#each triggerOptions as option}
				<option value={option.value}>{option.label}</option>
			{/each}
		</select>
	</div>

	<!-- Automations Table -->
	{#if isLoading && !isInitialized}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>Loading automations...</p>
		</div>
	{:else if filteredFunnels.length === 0}
		<div class="empty-state">
			<IconShare size={48} />
			<h3>No automations found</h3>
			<p>Create your first automation to engage contacts automatically</p>
			<a href="/admin/crm/automations/new" class="btn-primary">
				<IconPlus size={18} />
				Create Automation
			</a>
		</div>
	{:else}
		<div class="table-container" class:loading={isLoading}>
			<table class="data-table">
				<thead>
					<tr>
						<th>Automation</th>
						<th>Status</th>
						<th>Trigger</th>
						<th>Contacts</th>
						<th>Completed</th>
						<th>Completion Rate</th>
						<th>Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each filteredFunnels as funnel (funnel.id)}
						{@const StatusIcon = getStatusIcon(funnel.status)}
						<tr class:action-in-progress={actionInProgress === funnel.id}>
							<td>
								<div class="funnel-cell">
									<div class="funnel-icon">
										<IconShare size={20} />
									</div>
									<div class="funnel-info">
										<span class="funnel-title">{funnel.title}</span>
										<span class="funnel-meta">
											{funnel.actions?.length || 0} actions
										</span>
									</div>
								</div>
							</td>
							<td>
								<span class="status-badge {getStatusColor(funnel.status)}">
									<StatusIcon size={12} />
									{funnel.status}
								</span>
							</td>
							<td>
								<span class="trigger-badge {getTriggerColor(funnel.trigger_type)}">
									{triggerLabels[funnel.trigger_type] || funnel.trigger_type}
								</span>
							</td>
							<td>{formatNumber(funnel.subscribers_count)}</td>
							<td>{formatNumber(funnel.completed_count)}</td>
							<td>
								<span class="rate-value">{getCompletionRate(funnel)}</span>
							</td>
							<td>
								<div class="action-buttons">
									{#if funnel.status !== 'draft'}
										<button
											class="btn-icon"
											title={funnel.status === 'active' ? 'Pause' : 'Activate'}
											onclick={() => toggleFunnelStatus(funnel)}
											disabled={actionInProgress === funnel.id}
										>
											{#if funnel.status === 'active'}
												<IconPlayerPause size={16} />
											{:else}
												<IconPlayerPlay size={16} />
											{/if}
										</button>
									{/if}
									<a href="/admin/crm/automations/{funnel.id}" class="btn-icon" title="View Analytics">
										<IconEye size={16} />
									</a>
									<button
										class="btn-icon"
										title="Add Contacts"
										onclick={() => openAddContactsModal(funnel)}
										disabled={actionInProgress === funnel.id}
									>
										<IconUserPlus size={16} />
									</button>
									<a href="/admin/crm/automations/{funnel.id}/edit" class="btn-icon" title="Edit">
										<IconEdit size={16} />
									</a>
									<button
										class="btn-icon"
										title="Duplicate"
										onclick={() => duplicateFunnel(funnel.id)}
										disabled={actionInProgress === funnel.id}
									>
										<IconCopy size={16} />
									</button>
									<button
										class="btn-icon"
										title="Export"
										onclick={() => exportFunnel(funnel.id)}
										disabled={actionInProgress === funnel.id}
									>
										<IconDownload size={16} />
									</button>
									<button
										class="btn-icon danger"
										title="Delete"
										onclick={() => deleteFunnel(funnel.id)}
										disabled={actionInProgress === funnel.id}
									>
										<IconTrash size={16} />
									</button>
								</div>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
</div>

<!-- Import Automation Modal -->
{#if showImportModal}
	<div
		class="modal-backdrop"
		onclick={handleModalBackdropClick}
		role="dialog"
		aria-modal="true"
		aria-labelledby="import-modal-title"
	>
		<div class="modal-container">
			<div class="modal-header">
				<h2 id="import-modal-title">
					<IconUpload size={20} />
					Import Automation
				</h2>
				<button class="modal-close" onclick={closeImportModal} aria-label="Close modal">
					<IconX size={20} />
				</button>
			</div>

			<div class="modal-body">
				{#if importForm.success}
					<div class="success-message">
						<IconCheck size={20} />
						<span>Automation imported successfully!</span>
					</div>
				{:else}
					<p class="modal-description">
						Import an automation by uploading a JSON file or pasting the JSON data directly.
					</p>

					<div class="form-group">
						<label for="import-file">Upload JSON File</label>
						<div class="file-upload">
							<input
								id="import-file"
								type="file"
								accept=".json"
								onchange={handleFileUpload}
								disabled={importForm.isLoading}
							/>
							<div class="file-upload-label">
								<IconUpload size={18} />
								<span>{importForm.file ? importForm.file.name : 'Choose file or drag here'}</span>
							</div>
						</div>
					</div>

					<div class="form-divider">
						<span>or</span>
					</div>

					<div class="form-group">
						<label for="import-json">Paste JSON Data</label>
						<textarea
							id="import-json"
							placeholder={"Paste JSON automation config here..."}
							bind:value={importForm.jsonData}
							disabled={importForm.isLoading}
							rows="8"
							class:error={importForm.error}
						></textarea>
						{#if importForm.error}
							<span class="error-message">{importForm.error}</span>
						{/if}
					</div>

					<div class="modal-info">
						<IconAlertCircle size={16} />
						<span>The JSON should contain automation configuration including trigger type, conditions, and actions.</span>
					</div>
				{/if}
			</div>

			{#if !importForm.success}
				<div class="modal-footer">
					<button class="btn-secondary" onclick={closeImportModal} disabled={importForm.isLoading}>
						Cancel
					</button>
					<button
						class="btn-primary"
						onclick={importFunnel}
						disabled={!canImport}
					>
						{#if importForm.isLoading}
							<div class="btn-spinner"></div>
							Importing...
						{:else}
							<IconUpload size={16} />
							Import Automation
						{/if}
					</button>
				</div>
			{/if}
		</div>
	</div>
{/if}

<!-- Add Contacts to Funnel Modal -->
{#if showAddContactsModal && selectedFunnel}
	<div
		class="modal-backdrop"
		onclick={handleModalBackdropClick}
		role="dialog"
		aria-modal="true"
		aria-labelledby="add-contacts-modal-title"
	>
		<div class="modal-container">
			<div class="modal-header">
				<h2 id="add-contacts-modal-title">
					<IconUserPlus size={20} />
					Add Contacts to Automation
				</h2>
				<button class="modal-close" onclick={closeAddContactsModal} aria-label="Close modal">
					<IconX size={20} />
				</button>
			</div>

			<div class="modal-body">
				{#if addContactsForm.success}
					<div class="success-message">
						<IconCheck size={20} />
						<span>{addContactsForm.addedCount} contact(s) added to "{selectedFunnel.title}"</span>
					</div>
				{:else}
					<p class="modal-description">
						Add contacts to the automation "<strong>{selectedFunnel.title}</strong>" by entering their IDs below.
					</p>

					<div class="form-group">
						<label for="contact-ids">Contact IDs</label>
						<textarea
							id="contact-ids"
							placeholder="Enter contact IDs separated by commas or new lines...
Example:
contact_123
contact_456
contact_789"
							bind:value={addContactsForm.contactIds}
							disabled={addContactsForm.isLoading}
							rows="6"
							class:error={addContactsForm.error}
						></textarea>
						{#if addContactsForm.error}
							<span class="error-message">{addContactsForm.error}</span>
						{/if}
					</div>

					<div class="modal-info">
						<IconAlertCircle size={16} />
						<span>Contacts will enter the automation and begin processing from the first action.</span>
					</div>
				{/if}
			</div>

			{#if !addContactsForm.success}
				<div class="modal-footer">
					<button class="btn-secondary" onclick={closeAddContactsModal} disabled={addContactsForm.isLoading}>
						Cancel
					</button>
					<button
						class="btn-primary"
						onclick={addContactsToFunnel}
						disabled={!canAddContacts}
					>
						{#if addContactsForm.isLoading}
							<div class="btn-spinner"></div>
							Adding...
						{:else}
							<IconUserPlus size={16} />
							Add Contacts
						{/if}
					</button>
				</div>
			{/if}
		</div>
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

	.btn-refresh:disabled {
		opacity: 0.5;
		cursor: not-allowed;
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

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(99, 102, 241, 0.4);
	}

	.btn-primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
		transform: none;
	}

	.btn-secondary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: rgba(100, 116, 139, 0.2);
		border: 1px solid rgba(100, 116, 139, 0.3);
		color: #cbd5e1;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
	}

	.btn-secondary:hover:not(:disabled) {
		background: rgba(100, 116, 139, 0.3);
		border-color: rgba(100, 116, 139, 0.5);
	}

	.btn-secondary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* Alerts */
	.success-alert {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		background: rgba(34, 197, 94, 0.1);
		border: 1px solid rgba(34, 197, 94, 0.3);
		border-radius: 10px;
		color: #4ade80;
		margin-bottom: 1.5rem;
	}

	.error-alert {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 10px;
		color: #f87171;
		margin-bottom: 1.5rem;
	}

	.success-alert span, .error-alert span {
		flex: 1;
	}

	.success-alert button, .error-alert button {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		background: transparent;
		border: none;
		color: inherit;
		cursor: pointer;
		opacity: 0.7;
		transition: opacity 0.2s;
	}

	.success-alert button:hover, .error-alert button:hover {
		opacity: 1;
	}

	/* Stats Grid */
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
		margin-bottom: 2rem;
	}

	@media (max-width: 1200px) {
		.stats-grid { grid-template-columns: repeat(2, 1fr); }
	}

	@media (max-width: 640px) {
		.stats-grid { grid-template-columns: 1fr; }
		.page { padding: 1rem; }
	}

	.stat-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem;
		background: rgba(30, 41, 59, 0.4);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 8px;
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
	.stat-icon.amber { background: rgba(245, 158, 11, 0.15); color: #fbbf24; }

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

	/* Filters */
	.filters-bar {
		display: flex;
		gap: 1rem;
		margin-bottom: 1.5rem;
		flex-wrap: wrap;
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
	}

	.search-box input::placeholder {
		color: #64748b;
	}

	.search-clear {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 20px;
		height: 20px;
		background: rgba(99, 102, 241, 0.2);
		border: none;
		border-radius: 4px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.search-clear:hover {
		background: rgba(99, 102, 241, 0.3);
		color: #e2e8f0;
	}

	.filter-select {
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 10px;
		color: #e2e8f0;
		font-size: 0.9rem;
		cursor: pointer;
	}

	/* Table */
	.table-container {
		background: rgba(30, 41, 59, 0.4);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 8px;
		overflow: hidden;
		transition: opacity 0.2s;
	}

	.table-container.loading {
		opacity: 0.6;
		pointer-events: none;
	}

	.data-table {
		width: 100%;
		border-collapse: collapse;
	}

	.data-table th {
		padding: 1rem;
		text-align: left;
		font-size: 0.75rem;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		background: rgba(99, 102, 241, 0.05);
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
	}

	.data-table td {
		padding: 1rem;
		font-size: 0.9rem;
		color: #e2e8f0;
		border-bottom: 1px solid rgba(99, 102, 241, 0.05);
	}

	.data-table tbody tr:hover {
		background: rgba(99, 102, 241, 0.05);
	}

	.data-table tbody tr.action-in-progress {
		opacity: 0.5;
		pointer-events: none;
	}

	.funnel-cell {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.funnel-icon {
		width: 40px;
		height: 40px;
		border-radius: 10px;
		background: linear-gradient(135deg, #ec4899, #be185d);
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
		flex-shrink: 0;
	}

	.funnel-info {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.funnel-title {
		font-weight: 600;
		color: #f1f5f9;
	}

	.funnel-meta {
		font-size: 0.75rem;
		color: #64748b;
	}

	.trigger-badge {
		display: inline-block;
		padding: 0.25rem 0.5rem;
		border-radius: 6px;
		font-size: 0.75rem;
		font-weight: 500;
	}

	.status-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.25rem 0.75rem;
		border-radius: 20px;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: capitalize;
	}

	.rate-value {
		font-weight: 600;
		color: #4ade80;
	}

	.action-buttons {
		display: flex;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.btn-icon {
		width: 32px;
		height: 32px;
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

	.btn-icon:hover:not(:disabled) {
		background: rgba(99, 102, 241, 0.1);
		color: #818cf8;
	}

	.btn-icon:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-icon.danger:hover:not(:disabled) {
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
		border-color: rgba(239, 68, 68, 0.3);
	}

	/* States */
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

	/* Modal */
	.modal-backdrop {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.7);
		backdrop-filter: blur(4px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 1rem;
	}

	.modal-container {
		background: #1e293b;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 16px;
		width: 100%;
		max-width: 540px;
		max-height: 90vh;
		overflow: hidden;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
	}

	.modal-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1.25rem 1.5rem;
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
	}

	.modal-header h2 {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin: 0;
		font-size: 1.125rem;
		font-weight: 600;
		color: #f1f5f9;
	}

	.modal-header h2 :global(svg) {
		color: #6366f1;
	}

	.modal-close {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		background: transparent;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.modal-close:hover {
		background: rgba(99, 102, 241, 0.1);
		color: #e2e8f0;
	}

	.modal-body {
		padding: 1.5rem;
		overflow-y: auto;
		max-height: calc(90vh - 140px);
	}

	.modal-description {
		color: #94a3b8;
		margin: 0 0 1.5rem 0;
		line-height: 1.6;
	}

	.modal-description strong {
		color: #e2e8f0;
	}

	.form-group {
		margin-bottom: 1rem;
	}

	.form-group label {
		display: block;
		margin-bottom: 0.5rem;
		font-size: 0.875rem;
		font-weight: 500;
		color: #e2e8f0;
	}

	.form-group input,
	.form-group textarea {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 10px;
		color: #e2e8f0;
		font-size: 0.9rem;
		font-family: inherit;
		outline: none;
		transition: border-color 0.2s;
		resize: vertical;
	}

	.form-group textarea {
		min-height: 120px;
	}

	.form-group input:focus,
	.form-group textarea:focus {
		border-color: rgba(99, 102, 241, 0.5);
	}

	.form-group input.error,
	.form-group textarea.error {
		border-color: rgba(239, 68, 68, 0.5);
	}

	.form-group input:disabled,
	.form-group textarea:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.form-group input::placeholder,
	.form-group textarea::placeholder {
		color: #64748b;
	}

	.file-upload {
		position: relative;
	}

	.file-upload input[type="file"] {
		position: absolute;
		inset: 0;
		width: 100%;
		height: 100%;
		opacity: 0;
		cursor: pointer;
	}

	.file-upload-label {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		padding: 1.5rem;
		background: rgba(15, 23, 42, 0.6);
		border: 2px dashed rgba(99, 102, 241, 0.3);
		border-radius: 10px;
		color: #94a3b8;
		transition: all 0.2s;
	}

	.file-upload:hover .file-upload-label {
		border-color: rgba(99, 102, 241, 0.5);
		background: rgba(99, 102, 241, 0.05);
	}

	.form-divider {
		display: flex;
		align-items: center;
		gap: 1rem;
		margin: 1.5rem 0;
		color: #64748b;
		font-size: 0.8rem;
	}

	.form-divider::before,
	.form-divider::after {
		content: '';
		flex: 1;
		height: 1px;
		background: rgba(99, 102, 241, 0.2);
	}

	.error-message {
		display: block;
		margin-top: 0.5rem;
		font-size: 0.8rem;
		color: #f87171;
	}

	.modal-info {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		background: rgba(99, 102, 241, 0.1);
		border-radius: 8px;
		font-size: 0.8rem;
		color: #94a3b8;
	}

	.modal-info :global(svg) {
		color: #6366f1;
		flex-shrink: 0;
		margin-top: 0.125rem;
	}

	.success-message {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		padding: 1.5rem;
		background: rgba(34, 197, 94, 0.1);
		border: 1px solid rgba(34, 197, 94, 0.3);
		border-radius: 10px;
		color: #4ade80;
		font-weight: 500;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1rem 1.5rem;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
		background: rgba(15, 23, 42, 0.3);
	}

	.btn-spinner {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}
</style>

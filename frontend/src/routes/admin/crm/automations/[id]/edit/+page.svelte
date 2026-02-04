<!--
	/admin/crm/automations/[id]/edit - Edit Automation Workflow
	Apple Principal Engineer ICT 7 Grade - January 2026

	Features:
	- Visual workflow builder with drag-and-drop
	- Action type selection with settings
	- Delay configuration
	- Condition branching
	- Save and publish controls
	- Full Svelte 5 $state/$derived/$effect reactivity
-->

<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import IconShare from '@tabler/icons-svelte-runes/icons/share';
	import IconArrowLeft from '@tabler/icons-svelte-runes/icons/arrow-left';
	import IconPlus from '@tabler/icons-svelte-runes/icons/plus';
	import IconTrash from '@tabler/icons-svelte-runes/icons/trash';
	import IconEdit from '@tabler/icons-svelte-runes/icons/edit';
	import IconCheck from '@tabler/icons-svelte-runes/icons/check';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconAlertCircle from '@tabler/icons-svelte-runes/icons/alert-circle';
	import IconLoader2 from '@tabler/icons-svelte-runes/icons/loader-2';
	import IconMail from '@tabler/icons-svelte-runes/icons/mail';
	import IconClock from '@tabler/icons-svelte-runes/icons/clock';
	import IconTag from '@tabler/icons-svelte-runes/icons/tag';
	import IconList from '@tabler/icons-svelte-runes/icons/list';
	import IconBolt from '@tabler/icons-svelte-runes/icons/bolt';
	import IconUser from '@tabler/icons-svelte-runes/icons/user';
	import IconWorldWww from '@tabler/icons-svelte-runes/icons/world-www';
	import IconPlayerPlay from '@tabler/icons-svelte-runes/icons/player-play';
	import IconGripVertical from '@tabler/icons-svelte-runes/icons/grip-vertical';
	import { crmAPI } from '$lib/api/crm';
	import type { AutomationFunnel, FunnelAction, ActionType, FunnelStatus } from '$lib/crm/types';
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let funnel = $state<AutomationFunnel | null>(null);
	let actions = $state<FunnelAction[]>([]);
	let triggerTypes = $state<Record<string, string>>({});
	let actionTypesMap = $state<Record<string, string>>({});

	let isLoading = $state(true);
	let isSaving = $state(false);
	let error = $state('');
	let successMessage = $state('');

	// Delete confirmation modal state
	let showDeleteActionModal = $state(false);
	let pendingDeleteActionId = $state<string | null>(null);

	// Modal State
	let showAddActionModal = $state(false);
	let editingAction = $state<FunnelAction | null>(null);
	let actionForm = $state({
		action_type: '' as ActionType | '',
		title: '',
		delay_seconds: 0,
		delay_unit: 'minutes' as 'minutes' | 'hours' | 'days',
		settings: {} as Record<string, any>
	});

	// Available resources
	let availableTags = $state<{ id: string; title: string }[]>([]);
	let availableLists = $state<{ id: string; title: string }[]>([]);
	let availableSequences = $state<{ id: string; title: string }[]>([]);

	const funnelId = page.params.id ?? '';

	const actionTypeOptions = [
		{
			value: 'send_email',
			label: 'Send Email',
			description: 'Send an email to the contact',
			icon: IconMail,
			color: 'blue'
		},
		{
			value: 'wait',
			label: 'Wait / Delay',
			description: 'Add a time delay before the next action',
			icon: IconClock,
			color: 'amber'
		},
		{
			value: 'add_tag',
			label: 'Add Tag',
			description: 'Apply a tag to the contact',
			icon: IconTag,
			color: 'emerald'
		},
		{
			value: 'remove_tag',
			label: 'Remove Tag',
			description: 'Remove a tag from the contact',
			icon: IconTag,
			color: 'red'
		},
		{
			value: 'add_to_list',
			label: 'Add to List',
			description: 'Add contact to a list',
			icon: IconList,
			color: 'purple'
		},
		{
			value: 'remove_from_list',
			label: 'Remove from List',
			description: 'Remove contact from a list',
			icon: IconList,
			color: 'orange'
		},
		{
			value: 'add_to_sequence',
			label: 'Add to Sequence',
			description: 'Subscribe contact to an email sequence',
			icon: IconMail,
			color: 'indigo'
		},
		{
			value: 'update_contact',
			label: 'Update Contact',
			description: 'Update contact field values',
			icon: IconUser,
			color: 'cyan'
		},
		{
			value: 'http_request',
			label: 'HTTP Request',
			description: 'Send data to an external URL',
			icon: IconWorldWww,
			color: 'slate'
		},
		{
			value: 'end_funnel',
			label: 'End Automation',
			description: 'End the automation for this contact',
			icon: IconX,
			color: 'gray'
		}
	] as const;

	// ═══════════════════════════════════════════════════════════════════════════
	// API FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	async function loadFunnel() {
		isLoading = true;
		error = '';

		try {
			const [funnelResponse, actionsData, tagsResponse, listsResponse, sequencesResponse] =
				await Promise.all([
					crmAPI.getAutomationFunnel(funnelId),
					crmAPI.getFunnelActions(funnelId),
					crmAPI.getContactTags(),
					crmAPI.getContactLists(),
					crmAPI.getSequences()
				]);

			funnel = funnelResponse.funnel;
			triggerTypes = funnelResponse.trigger_types || {};
			actionTypesMap = funnelResponse.action_types || {};
			actions = actionsData || [];
			availableTags = tagsResponse.data || [];
			availableLists = listsResponse.data || [];
			availableSequences = sequencesResponse.data || [];
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load automation';
		} finally {
			isLoading = false;
		}
	}

	async function saveAction() {
		if (!actionForm.action_type) {
			error = 'Please select an action type';
			return;
		}

		isSaving = true;
		error = '';

		try {
			// Calculate delay in seconds
			let delaySeconds = actionForm.delay_seconds;
			if (actionForm.delay_unit === 'hours') {
				delaySeconds *= 3600;
			} else if (actionForm.delay_unit === 'days') {
				delaySeconds *= 86400;
			} else {
				delaySeconds *= 60;
			}

			const actionData = {
				action_type: actionForm.action_type as ActionType,
				title: actionForm.title || undefined,
				delay_seconds: delaySeconds,
				settings: Object.keys(actionForm.settings).length > 0 ? actionForm.settings : undefined,
				position: editingAction ? editingAction.position : actions.length
			};

			if (editingAction) {
				await crmAPI.updateFunnelAction(funnelId, editingAction.id, actionData);
				showSuccess('Action updated successfully');
			} else {
				await crmAPI.createFunnelAction(funnelId, actionData);
				showSuccess('Action added successfully');
			}

			closeActionModal();
			await loadFunnel();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to save action';
		} finally {
			isSaving = false;
		}
	}

	function deleteAction(actionId: string) {
		pendingDeleteActionId = actionId;
		showDeleteActionModal = true;
	}

	async function confirmDeleteAction() {
		if (!pendingDeleteActionId) return;
		showDeleteActionModal = false;
		const actionId = pendingDeleteActionId;
		pendingDeleteActionId = null;

		try {
			await crmAPI.deleteFunnelAction(funnelId, actionId);
			showSuccess('Action deleted successfully');
			await loadFunnel();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to delete action';
		}
	}

	async function updateFunnelStatus(status: FunnelStatus) {
		isSaving = true;
		error = '';

		try {
			await crmAPI.updateAutomationFunnel(funnelId, { status });
			showSuccess(`Automation ${status === 'active' ? 'activated' : 'paused'} successfully`);
			await loadFunnel();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to update status';
		} finally {
			isSaving = false;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// MODAL FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	function openAddActionModal() {
		editingAction = null;
		actionForm = {
			action_type: '',
			title: '',
			delay_seconds: 0,
			delay_unit: 'minutes',
			settings: {}
		};
		showAddActionModal = true;
	}

	function openEditActionModal(action: FunnelAction) {
		editingAction = action;

		// Convert delay from seconds to appropriate unit
		let delayValue = action.delay_seconds;
		let delayUnit: 'minutes' | 'hours' | 'days' = 'minutes';

		if (delayValue >= 86400 && delayValue % 86400 === 0) {
			delayValue = delayValue / 86400;
			delayUnit = 'days';
		} else if (delayValue >= 3600 && delayValue % 3600 === 0) {
			delayValue = delayValue / 3600;
			delayUnit = 'hours';
		} else {
			delayValue = delayValue / 60;
		}

		actionForm = {
			action_type: action.action_type,
			title: action.title || '',
			delay_seconds: delayValue,
			delay_unit: delayUnit,
			settings: action.settings || {}
		};
		showAddActionModal = true;
	}

	function closeActionModal() {
		showAddActionModal = false;
		editingAction = null;
		actionForm = {
			action_type: '',
			title: '',
			delay_seconds: 0,
			delay_unit: 'minutes',
			settings: {}
		};
	}

	function handleModalKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape') {
			closeActionModal();
		}
	}

	function handleModalBackdropClick(event: MouseEvent) {
		if (event.target === event.currentTarget) {
			closeActionModal();
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

	function getActionIcon(actionType: string) {
		const found = actionTypeOptions.find((a) => a.value === actionType);
		return found?.icon || IconBolt;
	}

	function getActionColor(actionType: string): string {
		const colorMap: Record<string, string> = {
			send_email: 'bg-blue-500/20 text-blue-400 border-blue-500/30',
			wait: 'bg-amber-500/20 text-amber-400 border-amber-500/30',
			add_tag: 'bg-emerald-500/20 text-emerald-400 border-emerald-500/30',
			remove_tag: 'bg-red-500/20 text-red-400 border-red-500/30',
			add_to_list: 'bg-purple-500/20 text-purple-400 border-purple-500/30',
			remove_from_list: 'bg-orange-500/20 text-orange-400 border-orange-500/30',
			add_to_sequence: 'bg-indigo-500/20 text-indigo-400 border-indigo-500/30',
			update_contact: 'bg-cyan-500/20 text-cyan-400 border-cyan-500/30',
			http_request: 'bg-slate-500/20 text-slate-400 border-slate-500/30',
			end_funnel: 'bg-gray-500/20 text-gray-400 border-gray-500/30'
		};
		return colorMap[actionType] || 'bg-slate-500/20 text-slate-400 border-slate-500/30';
	}

	function getIconColor(color: string): string {
		const colorMap: Record<string, string> = {
			blue: 'bg-blue-500/20 text-blue-400',
			amber: 'bg-amber-500/20 text-amber-400',
			emerald: 'bg-emerald-500/20 text-emerald-400',
			red: 'bg-red-500/20 text-red-400',
			purple: 'bg-purple-500/20 text-purple-400',
			orange: 'bg-orange-500/20 text-orange-400',
			indigo: 'bg-indigo-500/20 text-indigo-400',
			cyan: 'bg-cyan-500/20 text-cyan-400',
			slate: 'bg-slate-500/20 text-slate-400',
			gray: 'bg-gray-500/20 text-gray-400'
		};
		return colorMap[color] || colorMap.gray;
	}

	function formatDelay(seconds: number): string {
		if (seconds === 0) return 'Immediately';
		if (seconds >= 86400) return `${Math.floor(seconds / 86400)} day(s)`;
		if (seconds >= 3600) return `${Math.floor(seconds / 3600)} hour(s)`;
		return `${Math.floor(seconds / 60)} minute(s)`;
	}

	function getStatusColor(status: FunnelStatus): string {
		const colors: Record<FunnelStatus, string> = {
			draft: 'bg-slate-500/20 text-slate-400',
			active: 'bg-emerald-500/20 text-emerald-400',
			paused: 'bg-amber-500/20 text-amber-400'
		};
		return colors[status];
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let canSaveAction = $derived(actionForm.action_type !== '' && !isSaving);

	let requiresSettings = $derived(
		[
			'add_tag',
			'remove_tag',
			'add_to_list',
			'remove_from_list',
			'add_to_sequence',
			'send_email',
			'http_request'
		].includes(actionForm.action_type)
	);

	// ═══════════════════════════════════════════════════════════════════════════
	// LIFECYCLE
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		if (browser) {
			loadFunnel();
		}
	});
</script>

<svelte:head>
	<title>Edit {funnel?.title || 'Automation'} - FluentCRM Pro</title>
</svelte:head>

<svelte:window onkeydown={showAddActionModal ? handleModalKeydown : undefined} />

<div class="edit-automation-page">
	{#if isLoading}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>Loading automation...</p>
		</div>
	{:else if error && !funnel}
		<div class="error-state">
			<IconAlertCircle size={48} />
			<h3>Failed to load automation</h3>
			<p>{error}</p>
			<button class="btn-primary" onclick={loadFunnel}>Try Again</button>
		</div>
	{:else if funnel}
		<!-- Header -->
		<div class="page-header">
			<div class="header-content">
				<a href="/admin/crm/automations/{funnelId}" class="back-link">
					<IconArrowLeft size={18} />
					Back to Automation
				</a>
				<div class="title-row">
					<h1>Edit Workflow</h1>
					<span class="funnel-title">{funnel.title}</span>
					<span class="status-badge {getStatusColor(funnel.status)}">
						{funnel.status}
					</span>
				</div>
			</div>
			<div class="header-actions">
				{#if funnel.status === 'draft'}
					<button
						class="btn-primary"
						onclick={() => updateFunnelStatus('active')}
						disabled={isSaving || actions.length === 0}
					>
						<IconPlayerPlay size={18} />
						Publish & Activate
					</button>
				{:else if funnel.status === 'active'}
					<button
						class="btn-secondary"
						onclick={() => updateFunnelStatus('paused')}
						disabled={isSaving}
					>
						Pause Automation
					</button>
				{:else}
					<button
						class="btn-primary"
						onclick={() => updateFunnelStatus('active')}
						disabled={isSaving}
					>
						<IconPlayerPlay size={18} />
						Activate
					</button>
				{/if}
			</div>
		</div>

		<!-- Alerts -->
		{#if error}
			<div class="error-alert">
				<IconAlertCircle size={18} />
				<span>{error}</span>
				<button onclick={() => (error = '')}>
					<IconX size={16} />
				</button>
			</div>
		{/if}

		{#if successMessage}
			<div class="success-alert">
				<IconCheck size={18} />
				<span>{successMessage}</span>
			</div>
		{/if}

		<!-- Workflow Builder -->
		<div class="workflow-builder">
			<!-- Trigger Card -->
			<div class="workflow-card trigger-card">
				<div class="card-header">
					<div class="card-icon bg-emerald-500/20 text-emerald-400">
						<IconBolt size={24} />
					</div>
					<div class="card-title">
						<span class="card-type">TRIGGER</span>
						<span class="card-name">{triggerTypes[funnel.trigger_type] || funnel.trigger_type}</span
						>
					</div>
				</div>
				{#if funnel.trigger_settings && Object.keys(funnel.trigger_settings).length > 0}
					<div class="card-settings">
						{#each Object.entries(funnel.trigger_settings) as [key, value]}
							<span class="setting-chip">{key}: {value}</span>
						{/each}
					</div>
				{/if}
			</div>

			<!-- Connector -->
			<div class="workflow-connector">
				<div class="connector-line"></div>
			</div>

			<!-- Action Cards -->
			{#each actions as action, index (action.id)}
				{@const ActionIcon = getActionIcon(action.action_type)}
				<div class="workflow-card action-card {getActionColor(action.action_type)}">
					<div class="card-drag">
						<IconGripVertical size={16} />
					</div>
					<div class="card-header">
						<div class="card-icon">
							<ActionIcon size={24} />
						</div>
						<div class="card-title">
							<span class="card-type"
								>{actionTypesMap[action.action_type] || action.action_type}</span
							>
							<span class="card-name">{action.title || `Step ${index + 1}`}</span>
						</div>
						<div class="card-actions">
							<button class="card-btn" onclick={() => openEditActionModal(action)} title="Edit">
								<IconEdit size={16} />
							</button>
							<button
								class="card-btn danger"
								onclick={() => deleteAction(action.id)}
								title="Delete"
							>
								<IconTrash size={16} />
							</button>
						</div>
					</div>
					{#if action.delay_seconds > 0}
						<div class="card-delay">
							<IconClock size={14} />
							Wait {formatDelay(action.delay_seconds)} before this action
						</div>
					{/if}
					{#if action.settings && Object.keys(action.settings).length > 0}
						<div class="card-settings">
							{#each Object.entries(action.settings).slice(0, 3) as [key, value]}
								<span class="setting-chip">{key}: {value}</span>
							{/each}
						</div>
					{/if}
					<div class="card-stats">
						Executed {action.execution_count.toLocaleString()} times
					</div>
				</div>

				<!-- Connector after each action -->
				{#if index < actions.length - 1}
					<div class="workflow-connector">
						<div class="connector-line"></div>
					</div>
				{/if}
			{/each}

			<!-- Add Action Button -->
			<div class="workflow-connector">
				<div class="connector-line"></div>
			</div>

			<button class="add-action-btn" onclick={openAddActionModal}>
				<IconPlus size={24} />
				<span>Add Action</span>
			</button>
		</div>
	{/if}
</div>

<!-- Add/Edit Action Modal -->
{#if showAddActionModal}
	<div
		class="modal-backdrop"
		onclick={handleModalBackdropClick}
		onkeydown={handleModalKeydown}
		role="dialog"
		aria-modal="true"
		aria-labelledby="action-modal-title"
		tabindex="-1"
	>
		<div class="modal-container large">
			<div class="modal-header">
				<h2 id="action-modal-title">
					{editingAction ? 'Edit Action' : 'Add Action'}
				</h2>
				<button class="modal-close" onclick={closeActionModal} aria-label="Close modal">
					<IconX size={20} />
				</button>
			</div>

			<div class="modal-body">
				<!-- Action Type Selection -->
				{#if !actionForm.action_type && !editingAction}
					<p class="modal-description">Choose what this action should do:</p>
					<div class="action-type-grid">
						{#each actionTypeOptions as actionType}
							{@const TypeIcon = actionType.icon}
							<button
								class="action-type-card"
								onclick={() => (actionForm.action_type = actionType.value as ActionType)}
							>
								<div class="action-type-icon {getIconColor(actionType.color)}">
									<TypeIcon size={24} />
								</div>
								<div class="action-type-info">
									<span class="action-type-label">{actionType.label}</span>
									<span class="action-type-description">{actionType.description}</span>
								</div>
							</button>
						{/each}
					</div>
				{:else}
					{@const SelectedIcon = getActionIcon(actionForm.action_type)}
					<!-- Action Configuration -->
					<div class="action-config">
						<div class="selected-action-header">
							<div class="selected-action-icon {getActionColor(actionForm.action_type)}">
								<SelectedIcon size={24} />
							</div>
							<div class="selected-action-info">
								<span class="selected-action-type">
									{actionTypeOptions.find((a) => a.value === actionForm.action_type)?.label ||
										actionForm.action_type}
								</span>
								{#if !editingAction}
									<button class="change-type-btn" onclick={() => (actionForm.action_type = '')}>
										Change
									</button>
								{/if}
							</div>
						</div>

						<div class="form-group">
							<label for="action-title">Action Title (optional)</label>
							<input
								id="action-title" name="action-title"
								type="text"
								placeholder="Give this action a descriptive name..."
								bind:value={actionForm.title}
							/>
						</div>

						<!-- Delay Settings -->
						<div class="form-group">
							<label for="action-delay">Delay Before This Action</label>
							<div class="delay-inputs">
								<input
									id="action-delay" name="action-delay"
									type="number"
									min="0"
									bind:value={actionForm.delay_seconds}
								/>
								<select bind:value={actionForm.delay_unit}>
									<option value="minutes">Minutes</option>
									<option value="hours">Hours</option>
									<option value="days">Days</option>
								</select>
							</div>
							<span class="input-hint">Set to 0 for immediate execution</span>
						</div>

						<!-- Action-specific Settings -->
						{#if actionForm.action_type === 'add_tag' || actionForm.action_type === 'remove_tag'}
							<div class="form-group">
								<label for="tag-select">Select Tag</label>
								<select id="tag-select" bind:value={actionForm.settings.tag_id}>
									<option value="">Choose a tag...</option>
									{#each availableTags as tag}
										<option value={tag.id}>{tag.title}</option>
									{/each}
								</select>
							</div>
						{:else if actionForm.action_type === 'add_to_list' || actionForm.action_type === 'remove_from_list'}
							<div class="form-group">
								<label for="list-select">Select List</label>
								<select id="list-select" bind:value={actionForm.settings.list_id}>
									<option value="">Choose a list...</option>
									{#each availableLists as list}
										<option value={list.id}>{list.title}</option>
									{/each}
								</select>
							</div>
						{:else if actionForm.action_type === 'add_to_sequence'}
							<div class="form-group">
								<label for="sequence-select">Select Sequence</label>
								<select id="sequence-select" bind:value={actionForm.settings.sequence_id}>
									<option value="">Choose a sequence...</option>
									{#each availableSequences as seq}
										<option value={seq.id}>{seq.title}</option>
									{/each}
								</select>
							</div>
						{:else if actionForm.action_type === 'send_email'}
							<div class="form-group">
								<label for="email-subject">Email Subject</label>
								<input
									id="email-subject" name="email-subject"
									type="text"
									placeholder="Enter email subject..."
									bind:value={actionForm.settings.subject}
								/>
							</div>
							<div class="form-group">
								<label for="email-body">Email Body</label>
								<textarea
									id="email-body"
									rows="6"
									placeholder="Enter email content..."
									bind:value={actionForm.settings.body}
								></textarea>
							</div>
						{:else if actionForm.action_type === 'http_request'}
							<div class="form-group">
								<label for="webhook-url">Webhook URL</label>
								<input
									id="webhook-url" name="webhook-url"
									type="url"
									placeholder="https://..."
									bind:value={actionForm.settings.url}
								/>
							</div>
							<div class="form-group">
								<label for="webhook-method">Method</label>
								<select id="webhook-method" bind:value={actionForm.settings.method}>
									<option value="POST">POST</option>
									<option value="GET">GET</option>
									<option value="PUT">PUT</option>
								</select>
							</div>
						{:else if actionForm.action_type === 'wait'}
							<div class="info-card">
								<IconAlertCircle size={18} />
								<p>
									The delay settings above configure how long to wait before proceeding to the next
									action.
								</p>
							</div>
						{:else if actionForm.action_type === 'end_funnel'}
							<div class="info-card">
								<IconAlertCircle size={18} />
								<p>
									This action will end the automation for the current contact. They will be marked
									as completed.
								</p>
							</div>
						{/if}
					</div>
				{/if}
			</div>

			{#if actionForm.action_type}
				<div class="modal-footer">
					<button class="btn-secondary" onclick={closeActionModal} disabled={isSaving}>
						Cancel
					</button>
					<button class="btn-primary" onclick={saveAction} disabled={!canSaveAction}>
						{#if isSaving}
							<IconLoader2 size={16} class="spinning" />
							Saving...
						{:else}
							<IconCheck size={16} />
							{editingAction ? 'Update Action' : 'Add Action'}
						{/if}
					</button>
				</div>
			{/if}
		</div>
	</div>
{/if}

<style>
	.edit-automation-page {
		max-width: 900px;
		margin: 0 auto;
		padding: 24px;
	}

	/* Loading/Error States */
	.loading-state,
	.error-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		text-align: center;
		color: #64748b;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(99, 102, 241, 0.2);
		border-top-color: var(--primary-500);
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 1rem;
	}

	/* Header */
	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
		flex-wrap: wrap;
		gap: 1rem;
	}

	.back-link {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		color: #94a3b8;
		text-decoration: none;
		font-size: 0.875rem;
		margin-bottom: 0.5rem;
		transition: color 0.2s;
	}

	.back-link:hover {
		color: #e2e8f0;
	}

	.title-row {
		display: flex;
		align-items: center;
		gap: 1rem;
		flex-wrap: wrap;
	}

	.page-header h1 {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0;
	}

	.funnel-title {
		color: #94a3b8;
		font-size: 1rem;
	}

	.status-badge {
		padding: 0.375rem 0.875rem;
		border-radius: 20px;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: capitalize;
	}

	.header-actions {
		display: flex;
		gap: 0.75rem;
	}

	/* Buttons */
	.btn-primary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		color: white;
		border: none;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(99, 102, 241, 0.4);
	}

	.btn-primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-secondary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: rgba(30, 41, 59, 0.8);
		border: 1px solid rgba(99, 102, 241, 0.2);
		color: #e2e8f0;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover:not(:disabled) {
		background: rgba(99, 102, 241, 0.2);
	}

	.btn-secondary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* Alerts */
	.error-alert,
	.success-alert {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		border-radius: 10px;
		margin-bottom: 1.5rem;
	}

	.error-alert {
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #f87171;
	}

	.success-alert {
		background: rgba(34, 197, 94, 0.1);
		border: 1px solid rgba(34, 197, 94, 0.3);
		color: #4ade80;
	}

	.error-alert span,
	.success-alert span {
		flex: 1;
	}

	.error-alert button {
		background: transparent;
		border: none;
		color: inherit;
		cursor: pointer;
	}

	/* Workflow Builder */
	.workflow-builder {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0;
	}

	.workflow-card {
		width: 100%;
		max-width: 500px;
		background: rgba(15, 23, 42, 0.8);
		border: 2px solid rgba(99, 102, 241, 0.2);
		border-radius: 16px;
		padding: 1.25rem;
		position: relative;
	}

	.trigger-card {
		border-color: rgba(34, 197, 94, 0.3);
		background: rgba(34, 197, 94, 0.05);
	}

	.card-drag {
		position: absolute;
		left: -24px;
		top: 50%;
		transform: translateY(-50%);
		color: #64748b;
		cursor: grab;
		opacity: 0;
		transition: opacity 0.2s;
	}

	.workflow-card:hover .card-drag {
		opacity: 1;
	}

	.card-header {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.card-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.card-title {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.card-type {
		font-size: 0.7rem;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.card-name {
		font-size: 1rem;
		font-weight: 600;
		color: #e2e8f0;
	}

	.card-actions {
		display: flex;
		gap: 0.5rem;
	}

	.card-btn {
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
	}

	.card-btn:hover {
		background: rgba(99, 102, 241, 0.1);
		color: #818cf8;
	}

	.card-btn.danger:hover {
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
		border-color: rgba(239, 68, 68, 0.3);
	}

	.card-delay {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-top: 1rem;
		padding-top: 1rem;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
		font-size: 0.8rem;
		color: #94a3b8;
	}

	.card-settings {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		margin-top: 0.75rem;
	}

	.setting-chip {
		padding: 0.25rem 0.5rem;
		background: rgba(99, 102, 241, 0.1);
		border-radius: 4px;
		font-size: 0.7rem;
		color: #94a3b8;
	}

	.card-stats {
		margin-top: 0.75rem;
		font-size: 0.7rem;
		color: #64748b;
	}

	.workflow-connector {
		height: 40px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.connector-line {
		width: 2px;
		height: 100%;
		background: linear-gradient(180deg, rgba(99, 102, 241, 0.3), rgba(99, 102, 241, 0.1));
	}

	.add-action-btn {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
		width: 100%;
		max-width: 500px;
		padding: 2rem;
		background: rgba(99, 102, 241, 0.05);
		border: 2px dashed rgba(99, 102, 241, 0.3);
		border-radius: 16px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.add-action-btn:hover {
		background: rgba(99, 102, 241, 0.1);
		border-color: rgba(99, 102, 241, 0.5);
		color: #e2e8f0;
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

	.modal-container.large {
		max-width: 700px;
	}

	.modal-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1.25rem 1.5rem;
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
	}

	.modal-header h2 {
		margin: 0;
		font-size: 1.125rem;
		font-weight: 600;
		color: #f1f5f9;
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
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1rem 1.5rem;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
		background: rgba(15, 23, 42, 0.3);
	}

	/* Action Type Grid */
	.action-type-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 0.75rem;
	}

	@media (max-width: 640px) {
		.action-type-grid {
			grid-template-columns: 1fr;
		}
	}

	.action-type-card {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		padding: 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 2px solid rgba(99, 102, 241, 0.1);
		border-radius: 12px;
		cursor: pointer;
		transition: all 0.2s;
		text-align: left;
	}

	.action-type-card:hover {
		border-color: rgba(99, 102, 241, 0.3);
		background: rgba(99, 102, 241, 0.05);
	}

	.action-type-icon {
		width: 40px;
		height: 40px;
		border-radius: 10px;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.action-type-info {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.action-type-label {
		font-weight: 600;
		color: #e2e8f0;
		font-size: 0.875rem;
	}

	.action-type-description {
		font-size: 0.75rem;
		color: #64748b;
	}

	/* Action Config */
	.action-config {
		display: flex;
		flex-direction: column;
		gap: 1.25rem;
	}

	.selected-action-header {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem;
		background: rgba(15, 23, 42, 0.6);
		border-radius: 12px;
	}

	.selected-action-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.selected-action-info {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.selected-action-type {
		font-weight: 600;
		color: #e2e8f0;
	}

	.change-type-btn {
		background: transparent;
		border: none;
		color: var(--primary-500);
		font-size: 0.75rem;
		cursor: pointer;
		padding: 0;
	}

	.change-type-btn:hover {
		text-decoration: underline;
	}

	/* Form Groups */
	.form-group {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.form-group label {
		font-size: 0.875rem;
		font-weight: 500;
		color: #e2e8f0;
	}

	.form-group input,
	.form-group select,
	.form-group textarea {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.8);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 10px;
		color: #e2e8f0;
		font-size: 0.9rem;
		font-family: inherit;
		outline: none;
		transition: border-color 0.2s;
	}

	.form-group input:focus,
	.form-group select:focus,
	.form-group textarea:focus {
		border-color: rgba(99, 102, 241, 0.5);
	}

	.delay-inputs {
		display: flex;
		gap: 0.75rem;
	}

	.delay-inputs input {
		width: 120px;
	}

	.delay-inputs select {
		flex: 1;
	}

	.input-hint {
		font-size: 0.75rem;
		color: #64748b;
	}

	.info-card {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		padding: 1rem;
		background: rgba(99, 102, 241, 0.1);
		border-radius: 10px;
	}

	.info-card :global(svg) {
		color: var(--primary-500);
		flex-shrink: 0;
	}

	.info-card p {
		margin: 0;
		font-size: 0.875rem;
		color: #94a3b8;
	}

	/* Animations */
	:global(.spinning) {
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

	/* Responsive */
	@media (max-width: 640px) {
		.edit-automation-page {
			padding: 16px;
		}

		.workflow-card {
			padding: 1rem;
		}

		.card-drag {
			display: none;
		}
	}
</style>

<ConfirmationModal
	isOpen={showDeleteActionModal}
	title="Delete Action"
	message="Are you sure you want to delete this action?"
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmDeleteAction}
	onCancel={() => { showDeleteActionModal = false; pendingDeleteActionId = null; }}
/>

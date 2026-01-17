<!--
	/admin/crm/automations/new - Create New Automation
	Apple Principal Engineer ICT 7 Grade - January 2026

	Features:
	- Step-by-step automation builder wizard
	- Trigger type selection with descriptions
	- Trigger settings configuration
	- Entry conditions setup
	- Save as draft or publish
	- Full Svelte 5 $state/$derived/$effect reactivity
-->

<script lang="ts">
	import { goto } from '$app/navigation';
	import IconShare from '@tabler/icons-svelte/icons/share';
	import IconArrowLeft from '@tabler/icons-svelte/icons/arrow-left';
	import IconArrowRight from '@tabler/icons-svelte/icons/arrow-right';
	import IconCheck from '@tabler/icons-svelte/icons/check';
	import IconX from '@tabler/icons-svelte/icons/x';
	import IconAlertCircle from '@tabler/icons-svelte/icons/alert-circle';
	import IconUserPlus from '@tabler/icons-svelte/icons/user-plus';
	import IconTag from '@tabler/icons-svelte/icons/tag';
	import IconList from '@tabler/icons-svelte/icons/list';
	import IconForms from '@tabler/icons-svelte/icons/forms';
	import IconShoppingCart from '@tabler/icons-svelte/icons/shopping-cart';
	import IconCreditCard from '@tabler/icons-svelte/icons/credit-card';
	import IconLogin from '@tabler/icons-svelte/icons/login';
	import IconCalendar from '@tabler/icons-svelte/icons/calendar';
	import IconMail from '@tabler/icons-svelte/icons/mail';
	import IconLink from '@tabler/icons-svelte/icons/link';
	import IconCode from '@tabler/icons-svelte/icons/code';
	import IconLoader2 from '@tabler/icons-svelte/icons/loader-2';
	import { crmAPI } from '$lib/api/crm';
	import type { TriggerType, FunnelStatus } from '$lib/crm/types';

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let currentStep = $state(1);
	let isSubmitting = $state(false);
	let error = $state('');
	let successMessage = $state('');

	// Form Data
	let formData = $state({
		title: '',
		description: '',
		trigger_type: '' as TriggerType | '',
		trigger_settings: {} as Record<string, any>,
		conditions: [] as any[],
		status: 'draft' as FunnelStatus
	});

	// Available Tags & Lists (would be loaded from API)
	let availableTags = $state<{ id: string; title: string }[]>([]);
	let availableLists = $state<{ id: string; title: string }[]>([]);

	const triggerTypes = [
		{
			value: 'contact_created',
			label: 'Contact Created',
			description: 'Trigger when a new contact is added to the CRM',
			icon: IconUserPlus,
			color: 'blue'
		},
		{
			value: 'tag_applied',
			label: 'Tag Applied',
			description: 'Trigger when a specific tag is applied to a contact',
			icon: IconTag,
			color: 'green'
		},
		{
			value: 'tag_removed',
			label: 'Tag Removed',
			description: 'Trigger when a specific tag is removed from a contact',
			icon: IconTag,
			color: 'red'
		},
		{
			value: 'list_applied',
			label: 'List Applied',
			description: 'Trigger when a contact is added to a specific list',
			icon: IconList,
			color: 'purple'
		},
		{
			value: 'list_removed',
			label: 'List Removed',
			description: 'Trigger when a contact is removed from a specific list',
			icon: IconList,
			color: 'orange'
		},
		{
			value: 'form_submitted',
			label: 'Form Submitted',
			description: 'Trigger when a contact submits a specific form',
			icon: IconForms,
			color: 'indigo'
		},
		{
			value: 'order_completed',
			label: 'Order Completed',
			description: 'Trigger when a customer completes a purchase',
			icon: IconShoppingCart,
			color: 'amber'
		},
		{
			value: 'subscription_started',
			label: 'Subscription Started',
			description: 'Trigger when a contact starts a new subscription',
			icon: IconCreditCard,
			color: 'cyan'
		},
		{
			value: 'user_login',
			label: 'User Login',
			description: 'Trigger when a user logs into their account',
			icon: IconLogin,
			color: 'slate'
		},
		{
			value: 'birthday',
			label: 'Birthday',
			description: 'Trigger on the contact\'s birthday',
			icon: IconCalendar,
			color: 'pink'
		},
		{
			value: 'email_opened',
			label: 'Email Opened',
			description: 'Trigger when a contact opens a specific email',
			icon: IconMail,
			color: 'teal'
		},
		{
			value: 'link_clicked',
			label: 'Link Clicked',
			description: 'Trigger when a contact clicks a tracked link',
			icon: IconLink,
			color: 'violet'
		},
		{
			value: 'custom_event',
			label: 'Custom Event',
			description: 'Trigger on a custom event via API or webhook',
			icon: IconCode,
			color: 'gray'
		}
	] as const;

	const steps = [
		{ number: 1, title: 'Basic Info', description: 'Name your automation' },
		{ number: 2, title: 'Trigger', description: 'Choose when to start' },
		{ number: 3, title: 'Settings', description: 'Configure trigger' },
		{ number: 4, title: 'Review', description: 'Confirm and save' }
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// API FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	async function loadDependencies() {
		try {
			const [tagsResponse, listsResponse] = await Promise.all([
				crmAPI.getContactTags(),
				crmAPI.getContactLists()
			]);
			availableTags = tagsResponse.data || [];
			availableLists = listsResponse.data || [];
		} catch (err) {
			console.error('Failed to load dependencies:', err);
		}
	}

	async function createAutomation(status: FunnelStatus) {
		if (!validateForm()) return;

		isSubmitting = true;
		error = '';

		try {
			const data = {
				title: formData.title,
				description: formData.description || undefined,
				trigger_type: formData.trigger_type as TriggerType,
				trigger_settings: Object.keys(formData.trigger_settings).length > 0
					? formData.trigger_settings
					: undefined,
				conditions: formData.conditions.length > 0 ? formData.conditions : undefined,
				status
			};

			const result = await crmAPI.createAutomationFunnel(data);
			successMessage = `Automation "${formData.title}" created successfully!`;

			// Navigate to edit page to add actions
			setTimeout(() => {
				goto(`/admin/crm/automations/${result.id}/edit`);
			}, 1500);
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to create automation';
		} finally {
			isSubmitting = false;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// NAVIGATION
	// ═══════════════════════════════════════════════════════════════════════════

	function nextStep() {
		if (validateCurrentStep()) {
			currentStep = Math.min(currentStep + 1, steps.length);
		}
	}

	function prevStep() {
		currentStep = Math.max(currentStep - 1, 1);
	}

	function goToStep(step: number) {
		// Only allow going to completed or current steps
		if (step <= currentStep) {
			currentStep = step;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// VALIDATION
	// ═══════════════════════════════════════════════════════════════════════════

	function validateCurrentStep(): boolean {
		switch (currentStep) {
			case 1:
				if (!formData.title.trim()) {
					error = 'Please enter an automation title';
					return false;
				}
				break;
			case 2:
				if (!formData.trigger_type) {
					error = 'Please select a trigger type';
					return false;
				}
				break;
		}
		error = '';
		return true;
	}

	function validateForm(): boolean {
		if (!formData.title.trim()) {
			error = 'Automation title is required';
			currentStep = 1;
			return false;
		}
		if (!formData.trigger_type) {
			error = 'Trigger type is required';
			currentStep = 2;
			return false;
		}
		return true;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// HELPERS
	// ═══════════════════════════════════════════════════════════════════════════

	function selectTrigger(value: TriggerType) {
		formData.trigger_type = value;
		formData.trigger_settings = {};
	}

	function getTriggerInfo(type: string) {
		return triggerTypes.find(t => t.value === type);
	}

	function getIconColor(color: string): string {
		const colorMap: Record<string, string> = {
			blue: 'bg-blue-500/20 text-blue-400',
			green: 'bg-emerald-500/20 text-emerald-400',
			red: 'bg-red-500/20 text-red-400',
			purple: 'bg-purple-500/20 text-purple-400',
			orange: 'bg-orange-500/20 text-orange-400',
			indigo: 'bg-indigo-500/20 text-indigo-400',
			amber: 'bg-amber-500/20 text-amber-400',
			cyan: 'bg-cyan-500/20 text-cyan-400',
			slate: 'bg-slate-500/20 text-slate-400',
			pink: 'bg-pink-500/20 text-pink-400',
			teal: 'bg-teal-500/20 text-teal-400',
			violet: 'bg-violet-500/20 text-violet-400',
			gray: 'bg-gray-500/20 text-gray-400'
		};
		return colorMap[color] || colorMap.gray;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let canProceed = $derived(
		currentStep === 1 ? formData.title.trim().length > 0 :
		currentStep === 2 ? formData.trigger_type !== '' :
		true
	);

	let selectedTriggerInfo = $derived(
		formData.trigger_type ? getTriggerInfo(formData.trigger_type) : null
	);

	let requiresTriggerSettings = $derived(
		['tag_applied', 'tag_removed', 'list_applied', 'list_removed', 'form_submitted'].includes(formData.trigger_type)
	);

	// ═══════════════════════════════════════════════════════════════════════════
	// LIFECYCLE
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		loadDependencies();
	});
</script>

<svelte:head>
	<title>Create Automation - FluentCRM Pro</title>
</svelte:head>

<div class="new-automation-page">
	<!-- Header -->
	<div class="page-header">
		<a href="/admin/crm/automations" class="back-link">
			<IconArrowLeft size={18} />
			Back to Automations
		</a>
		<h1>
			<IconShare size={28} class="header-icon" />
			Create New Automation
		</h1>
		<p class="page-description">Build a powerful marketing automation triggered by events</p>
	</div>

	<!-- Progress Steps -->
	<div class="progress-steps">
		{#each steps as step}
			<button
				class="step"
				class:active={currentStep === step.number}
				class:completed={currentStep > step.number}
				onclick={() => goToStep(step.number)}
				disabled={step.number > currentStep}
			>
				<span class="step-number">
					{#if currentStep > step.number}
						<IconCheck size={14} />
					{:else}
						{step.number}
					{/if}
				</span>
				<span class="step-info">
					<span class="step-title">{step.title}</span>
					<span class="step-description">{step.description}</span>
				</span>
			</button>
		{/each}
	</div>

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

	<!-- Success Alert -->
	{#if successMessage}
		<div class="success-alert">
			<IconCheck size={18} />
			<span>{successMessage}</span>
		</div>
	{/if}

	<!-- Step Content -->
	<div class="step-content">
		<!-- Step 1: Basic Info -->
		{#if currentStep === 1}
			<div class="form-section">
				<h2>Basic Information</h2>
				<p class="section-description">Give your automation a name and optional description.</p>

				<div class="form-group">
					<label for="title">Automation Title <span class="required">*</span></label>
					<input
						id="title"
						type="text"
						placeholder="e.g., Welcome New Subscribers"
						bind:value={formData.title}
						class:error={error && !formData.title.trim()}
					/>
				</div>

				<div class="form-group">
					<label for="description">Description</label>
					<textarea
						id="description"
						placeholder="Describe what this automation does..."
						bind:value={formData.description}
						rows="3"
					></textarea>
				</div>
			</div>
		{/if}

		<!-- Step 2: Trigger Selection -->
		{#if currentStep === 2}
			<div class="form-section">
				<h2>Select Trigger</h2>
				<p class="section-description">Choose what event will start this automation.</p>

				<div class="trigger-grid">
					{#each triggerTypes as trigger}
						{@const TriggerIcon = trigger.icon}
						<button
							class="trigger-card"
							class:selected={formData.trigger_type === trigger.value}
							onclick={() => selectTrigger(trigger.value as TriggerType)}
						>
							<div class="trigger-icon {getIconColor(trigger.color)}">
								<TriggerIcon size={24} />
							</div>
							<div class="trigger-info">
								<span class="trigger-label">{trigger.label}</span>
								<span class="trigger-description">{trigger.description}</span>
							</div>
							{#if formData.trigger_type === trigger.value}
								<div class="selected-indicator">
									<IconCheck size={16} />
								</div>
							{/if}
						</button>
					{/each}
				</div>
			</div>
		{/if}

		<!-- Step 3: Trigger Settings -->
		{#if currentStep === 3}
			<div class="form-section">
				<h2>Configure Trigger</h2>
				<p class="section-description">
					{#if selectedTriggerInfo}
						Configure settings for "{selectedTriggerInfo.label}" trigger.
					{:else}
						Configure your trigger settings.
					{/if}
				</p>

				{#if formData.trigger_type === 'tag_applied' || formData.trigger_type === 'tag_removed'}
					<div class="form-group">
						<label for="tag-select">Select Tag <span class="required">*</span></label>
						<select
							id="tag-select"
							bind:value={formData.trigger_settings.tag_id}
						>
							<option value="">Choose a tag...</option>
							{#each availableTags as tag}
								<option value={tag.id}>{tag.title}</option>
							{/each}
						</select>
					</div>
				{:else if formData.trigger_type === 'list_applied' || formData.trigger_type === 'list_removed'}
					<div class="form-group">
						<label for="list-select">Select List <span class="required">*</span></label>
						<select
							id="list-select"
							bind:value={formData.trigger_settings.list_id}
						>
							<option value="">Choose a list...</option>
							{#each availableLists as list}
								<option value={list.id}>{list.title}</option>
							{/each}
						</select>
					</div>
				{:else if formData.trigger_type === 'form_submitted'}
					<div class="form-group">
						<label for="form-id">Form ID</label>
						<input
							id="form-id"
							type="text"
							placeholder="Enter form ID..."
							bind:value={formData.trigger_settings.form_id}
						/>
					</div>
				{:else if formData.trigger_type === 'custom_event'}
					<div class="form-group">
						<label for="event-name">Event Name <span class="required">*</span></label>
						<input
							id="event-name"
							type="text"
							placeholder="e.g., user_upgraded"
							bind:value={formData.trigger_settings.event_name}
						/>
						<span class="input-hint">The custom event name that will trigger this automation</span>
					</div>
				{:else}
					<div class="info-card">
						<IconAlertCircle size={20} />
						<p>This trigger type doesn't require additional configuration. You can proceed to the next step.</p>
					</div>
				{/if}

				<div class="advanced-options">
					<h3>Advanced Options</h3>
					<label class="checkbox-group">
						<input
							type="checkbox"
							bind:checked={formData.trigger_settings.run_once}
						/>
						<span>Run only once per contact</span>
					</label>
					<label class="checkbox-group">
						<input
							type="checkbox"
							bind:checked={formData.trigger_settings.skip_if_active}
						/>
						<span>Skip if contact is already in this automation</span>
					</label>
				</div>
			</div>
		{/if}

		<!-- Step 4: Review -->
		{#if currentStep === 4}
			<div class="form-section">
				<h2>Review & Create</h2>
				<p class="section-description">Review your automation settings before saving.</p>

				<div class="review-card">
					<div class="review-item">
						<span class="review-label">Title</span>
						<span class="review-value">{formData.title}</span>
					</div>
					{#if formData.description}
						<div class="review-item">
							<span class="review-label">Description</span>
							<span class="review-value">{formData.description}</span>
						</div>
					{/if}
					<div class="review-item">
						<span class="review-label">Trigger</span>
						<span class="review-value">
							{#if selectedTriggerInfo}
								{@const TriggerIcon = selectedTriggerInfo.icon}
								<span class="trigger-badge">
									<TriggerIcon size={16} />
									{selectedTriggerInfo.label}
								</span>
							{/if}
						</span>
					</div>
					{#if Object.keys(formData.trigger_settings).length > 0}
						<div class="review-item">
							<span class="review-label">Trigger Settings</span>
							<span class="review-value">
								{#if formData.trigger_settings.tag_id}
									Tag: {availableTags.find(t => t.id === formData.trigger_settings.tag_id)?.title || formData.trigger_settings.tag_id}
								{:else if formData.trigger_settings.list_id}
									List: {availableLists.find(l => l.id === formData.trigger_settings.list_id)?.title || formData.trigger_settings.list_id}
								{:else if formData.trigger_settings.event_name}
									Event: {formData.trigger_settings.event_name}
								{/if}
							</span>
						</div>
					{/if}
				</div>

				<div class="info-card">
					<IconAlertCircle size={20} />
					<p>After creating the automation, you'll be redirected to the editor where you can add actions to the workflow.</p>
				</div>
			</div>
		{/if}
	</div>

	<!-- Navigation -->
	<div class="form-navigation">
		<div class="nav-left">
			{#if currentStep > 1}
				<button class="btn-secondary" onclick={prevStep} disabled={isSubmitting}>
					<IconArrowLeft size={18} />
					Previous
				</button>
			{/if}
		</div>
		<div class="nav-right">
			{#if currentStep < steps.length}
				<button class="btn-primary" onclick={nextStep} disabled={!canProceed || isSubmitting}>
					Next
					<IconArrowRight size={18} />
				</button>
			{:else}
				<button
					class="btn-secondary"
					onclick={() => createAutomation('draft')}
					disabled={isSubmitting}
				>
					{#if isSubmitting}
						<IconLoader2 size={18} class="spinning" />
					{/if}
					Save as Draft
				</button>
				<button
					class="btn-primary"
					onclick={() => createAutomation('active')}
					disabled={isSubmitting}
				>
					{#if isSubmitting}
						<IconLoader2 size={18} class="spinning" />
					{:else}
						<IconCheck size={18} />
					{/if}
					Create & Activate
				</button>
			{/if}
		</div>
	</div>
</div>

<style>
	.new-automation-page {
		max-width: 900px;
		margin: 0 auto;
		padding: 24px;
	}

	/* Header */
	.page-header {
		margin-bottom: 2rem;
	}

	.back-link {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		color: #94a3b8;
		text-decoration: none;
		font-size: 0.875rem;
		margin-bottom: 1rem;
		transition: color 0.2s;
	}

	.back-link:hover {
		color: #e2e8f0;
	}

	.page-header h1 {
		display: flex;
		align-items: center;
		gap: 12px;
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.5rem 0;
	}

	.page-header h1 :global(.header-icon) {
		color: #ec4899;
	}

	.page-description {
		color: #64748b;
		margin: 0;
	}

	/* Progress Steps */
	.progress-steps {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 2rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 12px;
		padding: 0.75rem;
	}

	.step {
		flex: 1;
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem;
		background: transparent;
		border: 1px solid transparent;
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.2s;
		text-align: left;
	}

	.step:disabled {
		cursor: not-allowed;
		opacity: 0.5;
	}

	.step:hover:not(:disabled) {
		background: rgba(99, 102, 241, 0.05);
	}

	.step.active {
		background: rgba(99, 102, 241, 0.1);
		border-color: rgba(99, 102, 241, 0.3);
	}

	.step.completed .step-number {
		background: #22c55e;
		color: white;
	}

	.step-number {
		width: 28px;
		height: 28px;
		border-radius: 50%;
		background: rgba(99, 102, 241, 0.2);
		color: #94a3b8;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 0.8rem;
		font-weight: 600;
		flex-shrink: 0;
	}

	.step.active .step-number {
		background: #e6b800;
		color: white;
	}

	.step-info {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.step-title {
		font-size: 0.875rem;
		font-weight: 600;
		color: #e2e8f0;
	}

	.step-description {
		font-size: 0.75rem;
		color: #64748b;
	}

	/* Alerts */
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

	.error-alert span, .success-alert span {
		flex: 1;
	}

	.error-alert button {
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

	.error-alert button:hover {
		opacity: 1;
	}

	/* Step Content */
	.step-content {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 16px;
		padding: 2rem;
		margin-bottom: 1.5rem;
	}

	.form-section h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 0.5rem 0;
	}

	.section-description {
		color: #64748b;
		margin: 0 0 1.5rem 0;
	}

	/* Form Groups */
	.form-group {
		margin-bottom: 1.25rem;
	}

	.form-group label {
		display: block;
		margin-bottom: 0.5rem;
		font-size: 0.875rem;
		font-weight: 500;
		color: #e2e8f0;
	}

	.required {
		color: #f87171;
	}

	.form-group input,
	.form-group textarea,
	.form-group select {
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
	.form-group textarea:focus,
	.form-group select:focus {
		border-color: rgba(99, 102, 241, 0.5);
	}

	.form-group input.error {
		border-color: rgba(239, 68, 68, 0.5);
	}

	.form-group input::placeholder,
	.form-group textarea::placeholder {
		color: #64748b;
	}

	.input-hint {
		display: block;
		margin-top: 0.375rem;
		font-size: 0.75rem;
		color: #64748b;
	}

	/* Trigger Grid */
	.trigger-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1rem;
	}

	@media (max-width: 640px) {
		.trigger-grid {
			grid-template-columns: 1fr;
		}
	}

	.trigger-card {
		position: relative;
		display: flex;
		align-items: flex-start;
		gap: 1rem;
		padding: 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 2px solid rgba(99, 102, 241, 0.1);
		border-radius: 12px;
		cursor: pointer;
		transition: all 0.2s;
		text-align: left;
	}

	.trigger-card:hover {
		border-color: rgba(99, 102, 241, 0.3);
		background: rgba(99, 102, 241, 0.05);
	}

	.trigger-card.selected {
		border-color: #e6b800;
		background: rgba(99, 102, 241, 0.1);
	}

	.trigger-icon {
		width: 48px;
		height: 48px;
		border-radius: 10px;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.trigger-info {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
		flex: 1;
	}

	.trigger-label {
		font-weight: 600;
		color: #f1f5f9;
	}

	.trigger-description {
		font-size: 0.8rem;
		color: #64748b;
		line-height: 1.4;
	}

	.selected-indicator {
		position: absolute;
		top: 0.75rem;
		right: 0.75rem;
		width: 24px;
		height: 24px;
		border-radius: 50%;
		background: #e6b800;
		color: white;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	/* Advanced Options */
	.advanced-options {
		margin-top: 2rem;
		padding-top: 1.5rem;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
	}

	.advanced-options h3 {
		font-size: 0.875rem;
		font-weight: 600;
		color: #e2e8f0;
		margin: 0 0 1rem 0;
	}

	.checkbox-group {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 0.75rem;
		cursor: pointer;
		color: #94a3b8;
		font-size: 0.875rem;
	}

	.checkbox-group input[type="checkbox"] {
		width: 18px;
		height: 18px;
		border-radius: 4px;
		accent-color: #e6b800;
	}

	/* Info Card */
	.info-card {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		padding: 1rem;
		background: rgba(99, 102, 241, 0.1);
		border-radius: 10px;
		margin-top: 1rem;
	}

	.info-card :global(svg) {
		color: #e6b800;
		flex-shrink: 0;
	}

	.info-card p {
		margin: 0;
		font-size: 0.875rem;
		color: #94a3b8;
		line-height: 1.5;
	}

	/* Review Card */
	.review-card {
		background: rgba(15, 23, 42, 0.8);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 12px;
		overflow: hidden;
	}

	.review-item {
		display: flex;
		padding: 1rem 1.25rem;
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
	}

	.review-item:last-child {
		border-bottom: none;
	}

	.review-label {
		width: 140px;
		font-size: 0.875rem;
		color: #64748b;
		flex-shrink: 0;
	}

	.review-value {
		flex: 1;
		font-size: 0.875rem;
		color: #e2e8f0;
	}

	.trigger-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.25rem 0.75rem;
		background: rgba(99, 102, 241, 0.2);
		border-radius: 6px;
		color: #818cf8;
	}

	/* Navigation */
	.form-navigation {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.nav-left, .nav-right {
		display: flex;
		gap: 0.75rem;
	}

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: linear-gradient(135deg, #e6b800, #b38f00);
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
		transform: none;
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

	:global(.spinning) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}

	/* Responsive */
	@media (max-width: 640px) {
		.new-automation-page {
			padding: 16px;
		}

		.progress-steps {
			flex-direction: column;
		}

		.step-content {
			padding: 1.25rem;
		}

		.form-navigation {
			flex-direction: column;
			gap: 1rem;
		}

		.nav-left, .nav-right {
			width: 100%;
		}

		.nav-right {
			flex-direction: column;
		}

		.nav-right button {
			width: 100%;
			justify-content: center;
		}
	}
</style>

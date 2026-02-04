<script lang="ts">
	/**
	 * Scheduled Publishing - Revolution Trading Pros
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 */

	import { browser } from '$app/environment';
	import { scheduledApi, type ScheduledJob } from '$lib/api/video-advanced';
	import IconCalendar from '@tabler/icons-svelte-runes/icons/calendar';
	import IconClock from '@tabler/icons-svelte-runes/icons/clock';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconCheck from '@tabler/icons-svelte-runes/icons/check';
	import IconTrash from '@tabler/icons-svelte-runes/icons/trash';
	import IconPlus from '@tabler/icons-svelte-runes/icons/plus';
	import IconAlertCircle from '@tabler/icons-svelte-runes/icons/alert-circle';

	interface Props {
		resourceType?: 'video' | 'series';
		resourceId?: number;
		resourceTitle?: string;
		onScheduled?: (job: ScheduledJob) => void;
		onClose?: () => void;
	}

	let props: Props = $props();

	// Destructure with defaults for internal use
	const resourceType = $derived(props.resourceType ?? 'video');
	const resourceId = $derived(props.resourceId);
	const resourceTitle = $derived(props.resourceTitle);
	const onScheduled = $derived(props.onScheduled);
	const onClose = $derived(props.onClose);

	let jobs = $state<ScheduledJob[]>([]);
	let isLoading = $state(true);
	let isSaving = $state(false);
	let error = $state('');
	let showForm = $state(false);

	// Form state
	let scheduledDate = $state(getTomorrowDate());
	let scheduledTime = $state('09:00');
	let selectedAction = $state<'publish' | 'unpublish' | 'feature' | 'unfeature'>('publish');
	let timezone = $state('');
	let notifyOnPublish = $state(false);

	const actions = [
		{ value: 'publish', label: 'Publish' },
		{ value: 'unpublish', label: 'Unpublish' },
		{ value: 'feature', label: 'Feature' },
		{ value: 'unfeature', label: 'Unfeature' }
	];

	const timezones = [
		'America/New_York',
		'America/Chicago',
		'America/Denver',
		'America/Los_Angeles',
		'America/Phoenix',
		'Europe/London',
		'Europe/Paris',
		'Asia/Tokyo',
		'Asia/Shanghai',
		'Australia/Sydney',
		'UTC'
	];

	// Initialize on mount
	$effect(() => {
		if (!browser) return;

		// Set timezone client-side only to avoid hydration mismatch
		timezone = Intl.DateTimeFormat().resolvedOptions().timeZone;
		loadJobs();
	});

	function getTomorrowDate(): string {
		const tomorrow = new Date();
		tomorrow.setDate(tomorrow.getDate() + 1);
		return tomorrow.toISOString().split('T')[0];
	}

	async function loadJobs() {
		isLoading = true;
		error = '';

		const params: { resource_type?: string; status?: string } = { status: 'pending' };
		if (resourceId) {
			params.resource_type = resourceType;
		}

		const result = await scheduledApi.list(params);

		if (result.success && result.data) {
			jobs = result.data.filter((j) => !resourceId || j.resource_id === resourceId);
		} else {
			error = result.error || 'Failed to load scheduled jobs';
		}

		isLoading = false;
	}

	async function createJob() {
		if (!resourceId) {
			error = 'No resource selected';
			return;
		}

		const scheduledAt = new Date(`${scheduledDate}T${scheduledTime}`);
		if (scheduledAt <= new Date()) {
			error = 'Scheduled time must be in the future';
			return;
		}

		isSaving = true;
		error = '';

		const result = await scheduledApi.create({
			resource_type: resourceType,
			resource_id: resourceId,
			scheduled_at: scheduledAt.toISOString(),
			timezone,
			action: selectedAction,
			notify_on_publish: notifyOnPublish
		});

		if (result.success && result.data) {
			showForm = false;
			await loadJobs();
			if (onScheduled) {
				onScheduled({
					id: result.data.id,
					resource_type: resourceType,
					resource_id: resourceId,
					scheduled_at: result.data.scheduled_at,
					timezone,
					action: selectedAction,
					status: 'pending',
					notify_on_publish: notifyOnPublish,
					created_at: new Date().toISOString()
				});
			}
		} else {
			error = result.error || 'Failed to create scheduled job';
		}

		isSaving = false;
	}

	async function cancelJob(jobId: number) {
		if (!confirm('Are you sure you want to cancel this scheduled job?')) return;

		const result = await scheduledApi.cancel(jobId);

		if (result.success) {
			await loadJobs();
		} else {
			error = result.error || 'Failed to cancel job';
		}
	}

	function formatDateTime(isoString: string): string {
		const date = new Date(isoString);
		return date.toLocaleString('en-US', {
			dateStyle: 'medium',
			timeStyle: 'short'
		});
	}

	function getActionLabel(action: string): string {
		return action.charAt(0).toUpperCase() + action.slice(1);
	}

	function getActionColor(action: string): string {
		switch (action) {
			case 'publish':
				return '#22c55e';
			case 'unpublish':
				return '#ef4444';
			case 'feature':
				return '#f59e0b';
			case 'unfeature':
				return '#6b7280';
			default:
				return '#E6B800';
		}
	}
</script>

<div class="scheduled-publishing">
	<div class="header">
		<div class="header-left">
			<IconCalendar size={24} />
			<h3>Scheduled Publishing</h3>
		</div>
		{#if onClose}
			<button type="button" class="btn-close" onclick={onClose}>
				<IconX size={20} />
			</button>
		{/if}
	</div>

	{#if resourceTitle}
		<div class="resource-info">
			Scheduling for: <strong>{resourceTitle}</strong>
		</div>
	{/if}

	{#if error}
		<div class="error-message">
			<IconAlertCircle size={16} />
			{error}
		</div>
	{/if}

	{#if isLoading}
		<div class="loading">Loading scheduled jobs...</div>
	{:else}
		<!-- Existing Jobs -->
		{#if jobs.length > 0}
			<div class="jobs-list">
				<h4>Scheduled Jobs</h4>
				{#each jobs as job (job.id)}
					<div class="job-item">
						<div class="job-icon" style="color: {getActionColor(job.action)}">
							<IconClock size={20} />
						</div>
						<div class="job-info">
							<div class="job-action" style="color: {getActionColor(job.action)}">
								{getActionLabel(job.action)}
							</div>
							<div class="job-time">
								{formatDateTime(job.scheduled_at)}
								<span class="job-timezone">({job.timezone})</span>
							</div>
						</div>
						<button
							type="button"
							class="btn-cancel-job"
							onclick={() => cancelJob(job.id)}
							title="Cancel"
						>
							<IconTrash size={16} />
						</button>
					</div>
				{/each}
			</div>
		{/if}

		<!-- Add New Job -->
		{#if !showForm}
			<button
				type="button"
				class="btn-add-schedule"
				onclick={() => (showForm = true)}
				disabled={!resourceId}
			>
				<IconPlus size={18} /> Schedule New Action
			</button>
		{:else}
			<div class="schedule-form">
				<h4>Schedule Action</h4>

				<div class="form-group">
					<label for="action">Action</label>
					<select id="action" bind:value={selectedAction}>
						{#each actions as action (action.value)}
							<option value={action.value}>{action.label}</option>
						{/each}
					</select>
				</div>

				<div class="form-row">
					<div class="form-group">
						<label for="date">Date</label>
						<input type="date" id="date" bind:value={scheduledDate} min={getTomorrowDate()} />
					</div>
					<div class="form-group">
						<label for="time">Time</label>
						<input type="time" id="time" bind:value={scheduledTime} />
					</div>
				</div>

				<div class="form-group">
					<label for="timezone">Timezone</label>
					<select id="timezone" bind:value={timezone}>
						{#each timezones as tz (tz)}
							<option value={tz}>{tz.replace(/_/g, ' ')}</option>
						{/each}
					</select>
				</div>

				<div class="form-group checkbox">
					<label>
						<input type="checkbox" bind:checked={notifyOnPublish} />
						Send notification when published
					</label>
				</div>

				<div class="form-actions">
					<button type="button" class="btn-schedule" onclick={createJob} disabled={isSaving}>
						<IconCheck size={16} />
						{isSaving ? 'Scheduling...' : 'Schedule'}
					</button>
					<button type="button" class="btn-cancel" onclick={() => (showForm = false)}>
						Cancel
					</button>
				</div>
			</div>
		{/if}
	{/if}
</div>

<style>
	.scheduled-publishing {
		background: var(--bg-secondary, #1a1a2e);
		border-radius: 12px;
		padding: 1.5rem;
		max-width: 400px;
		width: 100%;
	}

	.header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.header-left {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.header-left h3 {
		margin: 0;
		font-size: 1.125rem;
	}

	.btn-close {
		background: none;
		border: none;
		color: var(--text-secondary);
		cursor: pointer;
		padding: 0.25rem;
	}

	.resource-info {
		font-size: 0.875rem;
		color: var(--text-secondary);
		margin-bottom: 1rem;
		padding: 0.75rem;
		background: var(--bg-tertiary, #252542);
		border-radius: 8px;
	}

	.error-message {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		background: #ef44441a;
		border: 1px solid #ef4444;
		color: #ef4444;
		padding: 0.75rem;
		border-radius: 8px;
		margin-bottom: 1rem;
		font-size: 0.875rem;
	}

	.loading {
		text-align: center;
		padding: 2rem;
		color: var(--text-secondary);
	}

	.jobs-list {
		margin-bottom: 1rem;
	}

	.jobs-list h4 {
		margin: 0 0 0.75rem;
		font-size: 0.875rem;
		color: var(--text-secondary);
	}

	.job-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem;
		background: var(--bg-tertiary, #252542);
		border-radius: 8px;
		margin-bottom: 0.5rem;
	}

	.job-icon {
		flex-shrink: 0;
	}

	.job-info {
		flex: 1;
	}

	.job-action {
		font-weight: 600;
		font-size: 0.875rem;
	}

	.job-time {
		font-size: 0.75rem;
		color: var(--text-secondary);
	}

	.job-timezone {
		opacity: 0.7;
	}

	.btn-cancel-job {
		background: none;
		border: none;
		color: var(--text-secondary);
		cursor: pointer;
		padding: 0.375rem;
		border-radius: 4px;
	}

	.btn-cancel-job:hover {
		color: #ef4444;
		background: #ef44441a;
	}

	.btn-add-schedule {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		width: 100%;
		padding: 0.75rem;
		background: var(--primary, #e6b800);
		color: #0d1117;
		border: none;
		border-radius: 8px;
		cursor: pointer;
		font-size: 0.875rem;
	}

	.btn-add-schedule:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.schedule-form {
		background: var(--bg-tertiary, #252542);
		padding: 1rem;
		border-radius: 8px;
	}

	.schedule-form h4 {
		margin: 0 0 1rem;
		font-size: 0.875rem;
	}

	.form-group {
		margin-bottom: 1rem;
	}

	.form-group label {
		display: block;
		font-size: 0.75rem;
		color: var(--text-secondary);
		margin-bottom: 0.375rem;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.form-group input,
	.form-group select {
		width: 100%;
		padding: 0.625rem;
		background: var(--bg-primary, #0f0f1a);
		border: 1px solid var(--border-color, #333);
		border-radius: 6px;
		color: var(--text-primary, white);
		font-size: 0.875rem;
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 0.75rem;
	}

	.form-group.checkbox label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.875rem;
		color: var(--text-primary, white);
		text-transform: none;
		letter-spacing: normal;
		cursor: pointer;
	}

	.form-group.checkbox input {
		width: auto;
	}

	.form-actions {
		display: flex;
		gap: 0.75rem;
		margin-top: 1rem;
	}

	.btn-schedule {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.375rem;
		padding: 0.625rem;
		background: var(--primary, #e6b800);
		color: #0d1117;
		border: none;
		border-radius: 6px;
		cursor: pointer;
		font-size: 0.875rem;
	}

	.btn-schedule:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-cancel {
		padding: 0.625rem 1rem;
		background: none;
		border: 1px solid var(--border-color, #333);
		color: var(--text-secondary);
		border-radius: 6px;
		cursor: pointer;
		font-size: 0.875rem;
	}

	.btn-cancel:hover {
		background: var(--bg-hover, #ffffff1a);
	}
</style>

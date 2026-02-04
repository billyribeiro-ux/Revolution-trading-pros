<!--
/**
 * SchedulingPanel - Storyblok-style Content Scheduling
 * =====================================================
 * Enterprise-grade content scheduling with timezone support,
 * release bundles, and calendar visualization.
 *
 * Features:
 * - Schedule single content publish/unpublish
 * - Create release bundles for coordinated updates
 * - Calendar view of scheduled content
 * - Timezone-aware scheduling
 * - Schedule history and audit log
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import { fade, slide } from 'svelte/transition';

	// Props
	interface Props {
		contentId: string;
		contentTitle?: string;
		isOpen: boolean;
		onClose: () => void;
		onScheduleCreated?: (schedule: Schedule) => void;
	}

	let props: Props = $props();
	const contentId = $derived(props.contentId);
	const contentTitle = $derived(props.contentTitle ?? 'Content');
	const isOpen = $derived(props.isOpen);
	const onClose = $derived(props.onClose);
	const onScheduleCreated = $derived(props.onScheduleCreated);

	// Types
	interface Schedule {
		id: string;
		content_id: string;
		action: 'publish' | 'unpublish' | 'archive' | 'update';
		scheduled_at: string;
		timezone: string;
		status: 'pending' | 'processing' | 'completed' | 'failed' | 'cancelled';
		notes?: string;
		content_title?: string;
		created_at: string;
	}

	interface Release {
		id: string;
		name: string;
		description?: string;
		scheduled_at?: string;
		timezone: string;
		status: 'draft' | 'scheduled' | 'processing' | 'completed' | 'failed' | 'cancelled';
		total_items: number;
		completed_items: number;
		failed_items: number;
		created_at: string;
	}

	interface CalendarEntry {
		id: string;
		content_id: string;
		content_title?: string;
		action: string;
		scheduled_at: string;
		is_release: boolean;
		release_name?: string;
	}

	// State
	let activeTab = $state<'schedule' | 'releases' | 'calendar' | 'history'>('schedule');
	let isLoading = $state(false);
	let error = $state<string | null>(null);
	let success = $state<string | null>(null);

	// Schedule form state
	let scheduleAction = $state<'publish' | 'unpublish' | 'archive'>('publish');
	let scheduleDate = $state('');
	let scheduleTime = $state('');
	let scheduleTimezone = $state(Intl.DateTimeFormat().resolvedOptions().timeZone);
	let scheduleNotes = $state('');

	// Pending schedules
	let pendingSchedules = $state<Schedule[]>([]);

	// Releases
	let releases = $state<Release[]>([]);
	let showCreateRelease = $state(false);
	let newReleaseName = $state('');
	let newReleaseDescription = $state('');

	// Calendar
	let calendarEntries = $state<CalendarEntry[]>([]);
	let calendarMonth = $state(new Date());

	// History
	let scheduleHistory = $state<any[]>([]);

	// Common timezones
	const commonTimezones = [
		'America/New_York',
		'America/Chicago',
		'America/Denver',
		'America/Los_Angeles',
		'America/Anchorage',
		'Pacific/Honolulu',
		'Europe/London',
		'Europe/Paris',
		'Europe/Berlin',
		'Asia/Tokyo',
		'Asia/Shanghai',
		'Asia/Singapore',
		'Australia/Sydney',
		'UTC'
	];

	// Initialize with today's date/time + 1 hour
	onMount(() => {
		const now = new Date();
		now.setHours(now.getHours() + 1);
		now.setMinutes(0);
		scheduleDate = now.toISOString().split('T')[0] || '';
		scheduleTime = now.toTimeString().slice(0, 5);

		if (isOpen) {
			loadPendingSchedules();
			loadReleases();
		}
	});

	// Watch for open state
	$effect(() => {
		if (isOpen) {
			loadPendingSchedules();
			loadReleases();
		}
	});

	// API Functions
	async function loadPendingSchedules() {
		try {
			const response = await fetch(`/api/cms/scheduling/schedules?content_id=${contentId}&status=pending`);
			if (response.ok) {
				const data = await response.json();
				pendingSchedules = data.schedules || [];
			}
		} catch (e) {
			console.error('Failed to load schedules:', e);
		}
	}

	async function loadReleases() {
		try {
			const response = await fetch('/api/cms/scheduling/releases?status=draft,scheduled');
			if (response.ok) {
				const data = await response.json();
				releases = data.releases || [];
			}
		} catch (e) {
			console.error('Failed to load releases:', e);
		}
	}

	async function loadCalendar() {
		try {
			const startDate = new Date(calendarMonth.getFullYear(), calendarMonth.getMonth(), 1);
			const endDate = new Date(calendarMonth.getFullYear(), calendarMonth.getMonth() + 1, 0);

			const response = await fetch(
				`/api/cms/scheduling/schedules/calendar?start_date=${startDate.toISOString()}&end_date=${endDate.toISOString()}`
			);
			if (response.ok) {
				calendarEntries = await response.json();
			}
		} catch (e) {
			console.error('Failed to load calendar:', e);
		}
	}

	async function loadHistory() {
		try {
			const response = await fetch(`/api/cms/scheduling/schedules/history?content_id=${contentId}&limit=20`);
			if (response.ok) {
				const data = await response.json();
				scheduleHistory = data.history || [];
			}
		} catch (e) {
			console.error('Failed to load history:', e);
		}
	}

	async function createSchedule() {
		if (!scheduleDate || !scheduleTime) {
			error = 'Please select a date and time';
			return;
		}

		isLoading = true;
		error = null;

		try {
			const scheduledAt = new Date(`${scheduleDate}T${scheduleTime}`);

			if (scheduledAt <= new Date()) {
				error = 'Scheduled time must be in the future';
				isLoading = false;
				return;
			}

			const response = await fetch('/api/cms/scheduling/schedules', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					content_id: contentId,
					action: scheduleAction,
					scheduled_at: scheduledAt.toISOString(),
					timezone: scheduleTimezone,
					notes: scheduleNotes || undefined
				})
			});

			if (!response.ok) {
				const data = await response.json();
				throw new Error(data.error || 'Failed to create schedule');
			}

			const schedule = await response.json();
			success = `Content scheduled to ${scheduleAction} on ${formatDateTime(scheduledAt.toISOString())}`;
			pendingSchedules = [...pendingSchedules, schedule];
			onScheduleCreated?.(schedule);

			// Reset form
			scheduleNotes = '';
		} catch (e: unknown) {
			error = e instanceof Error ? e.message : 'Failed to create schedule';
		} finally {
			isLoading = false;
		}
	}

	async function cancelSchedule(scheduleId: string) {
		if (!confirm('Are you sure you want to cancel this schedule?')) return;

		try {
			const response = await fetch(`/api/cms/scheduling/schedules/${scheduleId}/cancel`, {
				method: 'POST'
			});

			if (response.ok) {
				pendingSchedules = pendingSchedules.filter((s) => s.id !== scheduleId);
				success = 'Schedule cancelled';
			} else {
				const data = await response.json();
				throw new Error(data.error || 'Failed to cancel schedule');
			}
		} catch (e: unknown) {
			error = e instanceof Error ? e.message : 'Failed to cancel schedule';
		}
	}

	async function createRelease() {
		if (!newReleaseName.trim()) {
			error = 'Please enter a release name';
			return;
		}

		isLoading = true;
		error = null;

		try {
			const response = await fetch('/api/cms/scheduling/releases', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					name: newReleaseName,
					description: newReleaseDescription || undefined
				})
			});

			if (!response.ok) {
				const data = await response.json();
				throw new Error(data.error || 'Failed to create release');
			}

			const release = await response.json();
			releases = [release, ...releases];
			success = 'Release created';
			showCreateRelease = false;
			newReleaseName = '';
			newReleaseDescription = '';
		} catch (e: unknown) {
			error = e instanceof Error ? e.message : 'Failed to create release';
		} finally {
			isLoading = false;
		}
	}

	async function addToRelease(releaseId: string) {
		isLoading = true;
		error = null;

		try {
			const response = await fetch(`/api/cms/scheduling/releases/${releaseId}/items`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					content_id: contentId,
					action: 'publish'
				})
			});

			if (!response.ok) {
				const data = await response.json();
				throw new Error(data.error || 'Failed to add to release');
			}

			success = 'Content added to release';
			loadReleases();
		} catch (e: unknown) {
			error = e instanceof Error ? e.message : 'Failed to add to release';
		} finally {
			isLoading = false;
		}
	}

	// Utility functions
	function formatDateTime(isoString: string): string {
		const date = new Date(isoString);
		return date.toLocaleString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit',
			timeZoneName: 'short'
		});
	}

	function formatRelativeTime(isoString: string): string {
		const date = new Date(isoString);
		const now = new Date();
		const diff = date.getTime() - now.getTime();

		if (diff < 0) return 'Past';

		const hours = Math.floor(diff / (1000 * 60 * 60));
		const days = Math.floor(hours / 24);

		if (days > 0) return `In ${days} day${days > 1 ? 's' : ''}`;
		if (hours > 0) return `In ${hours} hour${hours > 1 ? 's' : ''}`;

		const minutes = Math.floor(diff / (1000 * 60));
		return `In ${minutes} minute${minutes !== 1 ? 's' : ''}`;
	}

	function getActionColor(action: string): string {
		switch (action) {
			case 'publish':
				return 'bg-green-100 text-green-800';
			case 'unpublish':
				return 'bg-yellow-100 text-yellow-800';
			case 'archive':
				return 'bg-gray-100 text-gray-800';
			default:
				return 'bg-blue-100 text-blue-800';
		}
	}

	function getStatusColor(status: string): string {
		switch (status) {
			case 'pending':
				return 'bg-blue-100 text-blue-800';
			case 'processing':
				return 'bg-yellow-100 text-yellow-800';
			case 'completed':
				return 'bg-green-100 text-green-800';
			case 'failed':
				return 'bg-red-100 text-red-800';
			case 'cancelled':
				return 'bg-gray-100 text-gray-800';
			default:
				return 'bg-gray-100 text-gray-800';
		}
	}

	// Calendar helpers
	function getCalendarDays(): (Date | null)[] {
		const year = calendarMonth.getFullYear();
		const month = calendarMonth.getMonth();
		const firstDay = new Date(year, month, 1);
		const lastDay = new Date(year, month + 1, 0);
		const days: (Date | null)[] = [];

		// Add empty slots for days before the first of the month
		for (let i = 0; i < firstDay.getDay(); i++) {
			days.push(null);
		}

		// Add all days of the month
		for (let i = 1; i <= lastDay.getDate(); i++) {
			days.push(new Date(year, month, i));
		}

		return days;
	}

	function getEntriesForDay(date: Date): CalendarEntry[] {
		return calendarEntries.filter((entry) => {
			const entryDate = new Date(entry.scheduled_at);
			return (
				entryDate.getFullYear() === date.getFullYear() &&
				entryDate.getMonth() === date.getMonth() &&
				entryDate.getDate() === date.getDate()
			);
		});
	}

	function prevMonth() {
		calendarMonth = new Date(calendarMonth.getFullYear(), calendarMonth.getMonth() - 1, 1);
		loadCalendar();
	}

	function nextMonth() {
		calendarMonth = new Date(calendarMonth.getFullYear(), calendarMonth.getMonth() + 1, 1);
		loadCalendar();
	}

	// Tab switching
	function switchTab(tab: typeof activeTab) {
		activeTab = tab;
		error = null;
		success = null;

		if (tab === 'calendar') {
			loadCalendar();
		} else if (tab === 'history') {
			loadHistory();
		}
	}
</script>

{#if isOpen}
	<div
		class="scheduling-overlay"
		onclick={onClose}
		onkeydown={(e) => e.key === 'Escape' && onClose()}
		role="dialog"
		aria-modal="true"
		aria-labelledby="scheduling-title"
		tabindex="-1"
		transition:fade={{ duration: 150 }}
	>
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<div
			class="scheduling-panel"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.stopPropagation()}
			role="document"
			transition:slide={{ duration: 200 }}
		>
			<!-- Header -->
			<div class="panel-header">
				<div>
					<h2 id="scheduling-title">Schedule Content</h2>
					<p class="content-name">{contentTitle}</p>
				</div>
				<button class="close-btn" onclick={onClose} aria-label="Close">
					<svg width="20" height="20" viewBox="0 0 20 20" fill="none">
						<path
							d="M15 5L5 15M5 5L15 15"
							stroke="currentColor"
							stroke-width="2"
							stroke-linecap="round"
						/>
					</svg>
				</button>
			</div>

			<!-- Tabs -->
			<div class="tabs">
				<button
					class="tab"
					class:active={activeTab === 'schedule'}
					onclick={() => switchTab('schedule')}
				>
					<svg width="16" height="16" viewBox="0 0 16 16" fill="none">
						<circle cx="8" cy="8" r="6.5" stroke="currentColor" stroke-width="1.5" />
						<path d="M8 4.5V8L10.5 10" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
					</svg>
					Schedule
				</button>
				<button
					class="tab"
					class:active={activeTab === 'releases'}
					onclick={() => switchTab('releases')}
				>
					<svg width="16" height="16" viewBox="0 0 16 16" fill="none">
						<rect x="2" y="4" width="12" height="2" rx="1" fill="currentColor" />
						<rect x="2" y="7" width="12" height="2" rx="1" fill="currentColor" />
						<rect x="2" y="10" width="12" height="2" rx="1" fill="currentColor" />
					</svg>
					Releases
				</button>
				<button
					class="tab"
					class:active={activeTab === 'calendar'}
					onclick={() => switchTab('calendar')}
				>
					<svg width="16" height="16" viewBox="0 0 16 16" fill="none">
						<rect x="2" y="3" width="12" height="11" rx="2" stroke="currentColor" stroke-width="1.5" />
						<path d="M2 6.5H14" stroke="currentColor" stroke-width="1.5" />
						<path d="M5 1.5V4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
						<path d="M11 1.5V4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
					</svg>
					Calendar
				</button>
				<button
					class="tab"
					class:active={activeTab === 'history'}
					onclick={() => switchTab('history')}
				>
					<svg width="16" height="16" viewBox="0 0 16 16" fill="none">
						<path
							d="M8 14A6 6 0 108 2a6 6 0 000 12z"
							stroke="currentColor"
							stroke-width="1.5"
						/>
						<path d="M3.5 8H2M8 3.5V2" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
					</svg>
					History
				</button>
			</div>

			<!-- Alerts -->
			{#if error}
				<div class="alert alert-error" transition:slide>
					<span>{error}</span>
					<button onclick={() => (error = null)}>Dismiss</button>
				</div>
			{/if}

			{#if success}
				<div class="alert alert-success" transition:slide>
					<span>{success}</span>
					<button onclick={() => (success = null)}>Dismiss</button>
				</div>
			{/if}

			<!-- Tab Content -->
			<div class="panel-content">
				<!-- Schedule Tab -->
				{#if activeTab === 'schedule'}
					<div class="schedule-form" transition:fade>
						<div class="form-section">
							<h3>Create New Schedule</h3>

							<div class="form-group">
								<label for="schedule-action">Action</label>
								<div class="action-buttons">
									<button
										type="button"
										class="action-btn"
										class:active={scheduleAction === 'publish'}
										onclick={() => (scheduleAction = 'publish')}
									>
										<span class="action-icon publish">Publish</span>
									</button>
									<button
										type="button"
										class="action-btn"
										class:active={scheduleAction === 'unpublish'}
										onclick={() => (scheduleAction = 'unpublish')}
									>
										<span class="action-icon unpublish">Unpublish</span>
									</button>
									<button
										type="button"
										class="action-btn"
										class:active={scheduleAction === 'archive'}
										onclick={() => (scheduleAction = 'archive')}
									>
										<span class="action-icon archive">Archive</span>
									</button>
								</div>
							</div>

							<div class="form-row">
								<div class="form-group">
									<label for="schedule-date">Date</label>
									<input
										type="date"
										id="schedule-date"
										bind:value={scheduleDate}
										min={new Date().toISOString().split('T')[0]}
									/>
								</div>
								<div class="form-group">
									<label for="schedule-time">Time</label>
									<input type="time" id="schedule-time" bind:value={scheduleTime} />
								</div>
							</div>

							<div class="form-group">
								<label for="schedule-timezone">Timezone</label>
								<select id="schedule-timezone" bind:value={scheduleTimezone}>
									{#each commonTimezones as tz}
										<option value={tz}>{tz.replace('_', ' ')}</option>
									{/each}
								</select>
							</div>

							<div class="form-group">
								<label for="schedule-notes">Notes (optional)</label>
								<textarea
									id="schedule-notes"
									bind:value={scheduleNotes}
									placeholder="Add any notes about this schedule..."
									rows="2"
								></textarea>
							</div>

							<button
								type="button"
								class="btn-primary"
								onclick={createSchedule}
								disabled={isLoading || !scheduleDate || !scheduleTime}
							>
								{#if isLoading}
									<span class="spinner"></span>
									Scheduling...
								{:else}
									Schedule {scheduleAction}
								{/if}
							</button>
						</div>

						<!-- Pending Schedules -->
						{#if pendingSchedules.length > 0}
							<div class="pending-schedules">
								<h3>Pending Schedules</h3>
								<div class="schedule-list">
									{#each pendingSchedules as schedule}
										<div class="schedule-item">
											<div class="schedule-info">
												<span class="action-badge {getActionColor(schedule.action)}">
													{schedule.action}
												</span>
												<span class="schedule-time">{formatDateTime(schedule.scheduled_at)}</span>
												<span class="schedule-relative">{formatRelativeTime(schedule.scheduled_at)}</span>
											</div>
											{#if schedule.notes}
												<p class="schedule-notes">{schedule.notes}</p>
											{/if}
											<button
												class="btn-cancel"
												onclick={() => cancelSchedule(schedule.id)}
												title="Cancel schedule"
											>
												Cancel
											</button>
										</div>
									{/each}
								</div>
							</div>
						{/if}
					</div>
				{/if}

				<!-- Releases Tab -->
				{#if activeTab === 'releases'}
					<div class="releases-section" transition:fade>
						<div class="releases-header">
							<h3>Release Bundles</h3>
							<button class="btn-secondary" onclick={() => (showCreateRelease = !showCreateRelease)}>
								{showCreateRelease ? 'Cancel' : '+ New Release'}
							</button>
						</div>

						{#if showCreateRelease}
							<div class="create-release-form" transition:slide>
								<div class="form-group">
									<label for="release-name">Release Name</label>
									<input
										type="text"
										id="release-name"
										bind:value={newReleaseName}
										placeholder="e.g., Spring Campaign Launch"
									/>
								</div>
								<div class="form-group">
									<label for="release-description">Description (optional)</label>
									<textarea
										id="release-description"
										bind:value={newReleaseDescription}
										placeholder="Describe this release..."
										rows="2"
									></textarea>
								</div>
								<button
									class="btn-primary"
									onclick={createRelease}
									disabled={isLoading || !newReleaseName.trim()}
								>
									Create Release
								</button>
							</div>
						{/if}

						<div class="releases-list">
							{#if releases.length === 0}
								<div class="empty-state">
									<p>No active releases</p>
									<p class="hint">Create a release to bundle multiple content changes together.</p>
								</div>
							{:else}
								{#each releases as release}
									<div class="release-item">
										<div class="release-header">
											<h4>{release.name}</h4>
											<span class="status-badge {getStatusColor(release.status)}">
												{release.status}
											</span>
										</div>
										{#if release.description}
											<p class="release-description">{release.description}</p>
										{/if}
										<div class="release-meta">
											<span>{release.total_items} item{release.total_items !== 1 ? 's' : ''}</span>
											{#if release.scheduled_at}
												<span>Scheduled: {formatDateTime(release.scheduled_at)}</span>
											{:else}
												<span>Draft</span>
											{/if}
										</div>
										<div class="release-actions">
											<button
												class="btn-add"
												onclick={() => addToRelease(release.id)}
												disabled={release.status !== 'draft' && release.status !== 'scheduled'}
											>
												Add This Content
											</button>
										</div>
									</div>
								{/each}
							{/if}
						</div>
					</div>
				{/if}

				<!-- Calendar Tab -->
				{#if activeTab === 'calendar'}
					<div class="calendar-section" transition:fade>
						<div class="calendar-header">
							<button class="calendar-nav" onclick={prevMonth} aria-label="Previous month">
								<svg width="16" height="16" viewBox="0 0 16 16" fill="none">
									<path d="M10 12L6 8L10 4" stroke="currentColor" stroke-width="2" stroke-linecap="round" />
								</svg>
							</button>
							<h3>
								{calendarMonth.toLocaleString('en-US', { month: 'long', year: 'numeric' })}
							</h3>
							<button class="calendar-nav" onclick={nextMonth} aria-label="Next month">
								<svg width="16" height="16" viewBox="0 0 16 16" fill="none">
									<path d="M6 4L10 8L6 12" stroke="currentColor" stroke-width="2" stroke-linecap="round" />
								</svg>
							</button>
						</div>

						<div class="calendar-grid">
							<div class="calendar-weekdays">
								{#each ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'] as day}
									<div class="weekday">{day}</div>
								{/each}
							</div>
							<div class="calendar-days">
								{#each getCalendarDays() as day}
									{#if day === null}
										<div class="calendar-day empty"></div>
									{:else}
										{@const entries = getEntriesForDay(day)}
										<div
											class="calendar-day"
											class:today={day.toDateString() === new Date().toDateString()}
											class:has-entries={entries.length > 0}
										>
											<span class="day-number">{day.getDate()}</span>
											{#if entries.length > 0}
												<div class="day-entries">
													{#each entries.slice(0, 2) as entry}
														<div
															class="entry-dot"
															class:is-release={entry.is_release}
															title={entry.content_title || entry.release_name}
														></div>
													{/each}
													{#if entries.length > 2}
														<span class="more-entries">+{entries.length - 2}</span>
													{/if}
												</div>
											{/if}
										</div>
									{/if}
								{/each}
							</div>
						</div>

						<div class="calendar-legend">
							<div class="legend-item">
								<div class="entry-dot"></div>
								<span>Individual Schedule</span>
							</div>
							<div class="legend-item">
								<div class="entry-dot is-release"></div>
								<span>Release Bundle</span>
							</div>
						</div>
					</div>
				{/if}

				<!-- History Tab -->
				{#if activeTab === 'history'}
					<div class="history-section" transition:fade>
						<h3>Schedule History</h3>
						{#if scheduleHistory.length === 0}
							<div class="empty-state">
								<p>No scheduling history for this content.</p>
							</div>
						{:else}
							<div class="history-list">
								{#each scheduleHistory as item}
									<div class="history-item">
										<div class="history-icon">
											<svg width="16" height="16" viewBox="0 0 16 16" fill="none">
												<circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.5" />
												<path d="M8 4.5V8L10 10" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
											</svg>
										</div>
										<div class="history-content">
											<span class="history-event">{item.event_type.replace('_', ' ')}</span>
											{#if item.new_status}
												<span class="history-status">{item.new_status}</span>
											{/if}
											<span class="history-time">{formatDateTime(item.created_at)}</span>
										</div>
									</div>
								{/each}
							</div>
						{/if}
					</div>
				{/if}
			</div>
		</div>
	</div>
{/if}

<style>
	.scheduling-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 1rem;
	}

	.scheduling-panel {
		width: 100%;
		max-width: 600px;
		max-height: 90vh;
		background: var(--bg-primary, #ffffff);
		border-radius: 12px;
		display: flex;
		flex-direction: column;
		overflow: hidden;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
	}

	.panel-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		padding: 1.25rem 1.5rem;
		border-bottom: 1px solid var(--border-color, #e5e7eb);
	}

	.panel-header h2 {
		margin: 0;
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--text-primary, #1f2937);
	}

	.content-name {
		margin: 0.25rem 0 0;
		font-size: 0.875rem;
		color: var(--text-secondary, #6b7280);
	}

	.close-btn {
		padding: 0.5rem;
		background: transparent;
		border: none;
		border-radius: 0.375rem;
		cursor: pointer;
		color: var(--text-secondary, #6b7280);
		transition: all 0.2s;
	}

	.close-btn:hover {
		background: var(--bg-secondary, #f3f4f6);
		color: var(--text-primary, #1f2937);
	}

	.tabs {
		display: flex;
		padding: 0 1rem;
		border-bottom: 1px solid var(--border-color, #e5e7eb);
		gap: 0.25rem;
	}

	.tab {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		background: transparent;
		border: none;
		border-bottom: 2px solid transparent;
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--text-secondary, #6b7280);
		cursor: pointer;
		transition: all 0.2s;
	}

	.tab:hover {
		color: var(--text-primary, #1f2937);
	}

	.tab.active {
		color: var(--primary, #3b82f6);
		border-bottom-color: var(--primary, #3b82f6);
	}

	.alert {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.75rem 1rem;
		margin: 0.75rem 1rem 0;
		border-radius: 0.5rem;
		font-size: 0.875rem;
	}

	.alert-error {
		background: #fef2f2;
		color: #b91c1c;
	}

	.alert-success {
		background: #ecfdf5;
		color: #047857;
	}

	.alert button {
		padding: 0.25rem 0.5rem;
		background: transparent;
		border: none;
		font-size: 0.75rem;
		color: inherit;
		opacity: 0.7;
		cursor: pointer;
	}

	.alert button:hover {
		opacity: 1;
	}

	.panel-content {
		flex: 1;
		overflow-y: auto;
		padding: 1.5rem;
	}

	.form-section h3,
	.pending-schedules h3,
	.releases-section h3,
	.calendar-section h3,
	.history-section h3 {
		margin: 0 0 1rem;
		font-size: 1rem;
		font-weight: 600;
		color: var(--text-primary, #1f2937);
	}

	.form-group {
		margin-bottom: 1rem;
	}

	.form-group label {
		display: block;
		margin-bottom: 0.375rem;
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--text-primary, #1f2937);
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	.action-buttons {
		display: flex;
		gap: 0.5rem;
	}

	.action-btn {
		flex: 1;
		padding: 0.75rem;
		background: var(--bg-secondary, #f9fafb);
		border: 2px solid var(--border-color, #e5e7eb);
		border-radius: 0.5rem;
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--text-secondary, #6b7280);
		cursor: pointer;
		transition: all 0.2s;
	}

	.action-btn:hover {
		border-color: var(--primary, #3b82f6);
	}

	.action-btn.active {
		background: #eff6ff;
		border-color: var(--primary, #3b82f6);
		color: var(--primary, #3b82f6);
	}

	input[type='date'],
	input[type='time'],
	input[type='text'],
	select,
	textarea {
		width: 100%;
		padding: 0.625rem 0.75rem;
		background: var(--bg-primary, #ffffff);
		border: 1px solid var(--border-color, #e5e7eb);
		border-radius: 0.5rem;
		font-size: 0.875rem;
		color: var(--text-primary, #1f2937);
		transition: border-color 0.2s;
	}

	input:focus,
	select:focus,
	textarea:focus {
		outline: none;
		border-color: var(--primary, #3b82f6);
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	textarea {
		resize: vertical;
		min-height: 60px;
	}

	.btn-primary {
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: var(--primary, #3b82f6);
		border: none;
		border-radius: 0.5rem;
		font-size: 0.875rem;
		font-weight: 600;
		color: white;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover:not(:disabled) {
		background: var(--primary-hover, #2563eb);
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-secondary {
		padding: 0.5rem 1rem;
		background: var(--bg-secondary, #f9fafb);
		border: 1px solid var(--border-color, #e5e7eb);
		border-radius: 0.375rem;
		font-size: 0.8125rem;
		font-weight: 500;
		color: var(--text-primary, #1f2937);
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		background: var(--bg-hover, #f3f4f6);
	}

	.spinner {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.pending-schedules {
		margin-top: 2rem;
		padding-top: 1.5rem;
		border-top: 1px solid var(--border-color, #e5e7eb);
	}

	.schedule-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.schedule-item {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 0.75rem;
		padding: 0.875rem;
		background: var(--bg-secondary, #f9fafb);
		border-radius: 0.5rem;
	}

	.schedule-info {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		flex: 1;
	}

	.action-badge {
		padding: 0.25rem 0.625rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: capitalize;
	}

	.schedule-time {
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--text-primary, #1f2937);
	}

	.schedule-relative {
		font-size: 0.8125rem;
		color: var(--text-secondary, #6b7280);
	}

	.schedule-notes {
		width: 100%;
		margin: 0;
		font-size: 0.8125rem;
		color: var(--text-secondary, #6b7280);
	}

	.btn-cancel {
		padding: 0.375rem 0.75rem;
		background: transparent;
		border: 1px solid var(--border-color, #e5e7eb);
		border-radius: 0.375rem;
		font-size: 0.75rem;
		color: var(--text-secondary, #6b7280);
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-cancel:hover {
		background: #fef2f2;
		border-color: #fecaca;
		color: #b91c1c;
	}

	/* Releases */
	.releases-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.releases-header h3 {
		margin: 0;
	}

	.create-release-form {
		padding: 1rem;
		margin-bottom: 1rem;
		background: var(--bg-secondary, #f9fafb);
		border-radius: 0.5rem;
	}

	.releases-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.release-item {
		padding: 1rem;
		background: var(--bg-secondary, #f9fafb);
		border: 1px solid var(--border-color, #e5e7eb);
		border-radius: 0.5rem;
	}

	.release-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.5rem;
	}

	.release-header h4 {
		margin: 0;
		font-size: 0.9375rem;
		font-weight: 600;
		color: var(--text-primary, #1f2937);
	}

	.status-badge {
		padding: 0.25rem 0.625rem;
		border-radius: 9999px;
		font-size: 0.6875rem;
		font-weight: 600;
		text-transform: uppercase;
	}

	.release-description {
		margin: 0 0 0.5rem;
		font-size: 0.8125rem;
		color: var(--text-secondary, #6b7280);
	}

	.release-meta {
		display: flex;
		gap: 1rem;
		margin-bottom: 0.75rem;
		font-size: 0.75rem;
		color: var(--text-tertiary, #9ca3af);
	}

	.release-actions {
		display: flex;
		gap: 0.5rem;
	}

	.btn-add {
		padding: 0.5rem 0.875rem;
		background: var(--primary, #3b82f6);
		border: none;
		border-radius: 0.375rem;
		font-size: 0.8125rem;
		font-weight: 500;
		color: white;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-add:hover:not(:disabled) {
		background: var(--primary-hover, #2563eb);
	}

	.btn-add:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.empty-state {
		padding: 2rem;
		text-align: center;
		color: var(--text-secondary, #6b7280);
	}

	.empty-state p {
		margin: 0;
	}

	.empty-state .hint {
		margin-top: 0.5rem;
		font-size: 0.8125rem;
		color: var(--text-tertiary, #9ca3af);
	}

	/* Calendar */
	.calendar-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.calendar-header h3 {
		margin: 0;
	}

	.calendar-nav {
		padding: 0.5rem;
		background: transparent;
		border: 1px solid var(--border-color, #e5e7eb);
		border-radius: 0.375rem;
		cursor: pointer;
		color: var(--text-secondary, #6b7280);
		transition: all 0.2s;
	}

	.calendar-nav:hover {
		background: var(--bg-secondary, #f9fafb);
		color: var(--text-primary, #1f2937);
	}

	.calendar-grid {
		border: 1px solid var(--border-color, #e5e7eb);
		border-radius: 0.5rem;
		overflow: hidden;
	}

	.calendar-weekdays {
		display: grid;
		grid-template-columns: repeat(7, 1fr);
		background: var(--bg-secondary, #f9fafb);
		border-bottom: 1px solid var(--border-color, #e5e7eb);
	}

	.weekday {
		padding: 0.625rem;
		text-align: center;
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--text-secondary, #6b7280);
		text-transform: uppercase;
	}

	.calendar-days {
		display: grid;
		grid-template-columns: repeat(7, 1fr);
	}

	.calendar-day {
		min-height: 60px;
		padding: 0.375rem;
		border-right: 1px solid var(--border-color, #e5e7eb);
		border-bottom: 1px solid var(--border-color, #e5e7eb);
		background: var(--bg-primary, #ffffff);
	}

	.calendar-day:nth-child(7n) {
		border-right: none;
	}

	.calendar-day.empty {
		background: var(--bg-secondary, #f9fafb);
	}

	.calendar-day.today {
		background: #eff6ff;
	}

	.calendar-day.has-entries {
		background: #f0fdf4;
	}

	.day-number {
		display: block;
		font-size: 0.8125rem;
		font-weight: 500;
		color: var(--text-primary, #1f2937);
	}

	.calendar-day.today .day-number {
		color: var(--primary, #3b82f6);
		font-weight: 700;
	}

	.day-entries {
		display: flex;
		gap: 0.25rem;
		margin-top: 0.25rem;
		flex-wrap: wrap;
	}

	.entry-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: var(--primary, #3b82f6);
	}

	.entry-dot.is-release {
		background: #10b981;
	}

	.more-entries {
		font-size: 0.625rem;
		color: var(--text-tertiary, #9ca3af);
	}

	.calendar-legend {
		display: flex;
		gap: 1.5rem;
		margin-top: 1rem;
		padding-top: 1rem;
		border-top: 1px solid var(--border-color, #e5e7eb);
	}

	.legend-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.75rem;
		color: var(--text-secondary, #6b7280);
	}

	/* History */
	.history-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.history-item {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		padding: 0.75rem;
		background: var(--bg-secondary, #f9fafb);
		border-radius: 0.5rem;
	}

	.history-icon {
		flex-shrink: 0;
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--bg-primary, #ffffff);
		border: 1px solid var(--border-color, #e5e7eb);
		border-radius: 50%;
		color: var(--text-secondary, #6b7280);
	}

	.history-content {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.history-event {
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--text-primary, #1f2937);
		text-transform: capitalize;
	}

	.history-status {
		font-size: 0.75rem;
		color: var(--text-secondary, #6b7280);
		text-transform: capitalize;
	}

	.history-time {
		font-size: 0.75rem;
		color: var(--text-tertiary, #9ca3af);
	}

	/* Responsive */
	@media (max-width: 640px) {
		.scheduling-panel {
			max-height: 100vh;
			border-radius: 12px 12px 0 0;
		}

		.form-row {
			grid-template-columns: 1fr;
		}

		.tabs {
			overflow-x: auto;
		}

		.tab {
			white-space: nowrap;
		}

		.action-buttons {
			flex-direction: column;
		}

		.calendar-day {
			min-height: 50px;
		}
	}

	/* Color classes */
	.bg-green-100 {
		background-color: #dcfce7;
	}
	.text-green-800 {
		color: #166534;
	}
	.bg-yellow-100 {
		background-color: #fef9c3;
	}
	.text-yellow-800 {
		color: #854d0e;
	}
	.bg-gray-100 {
		background-color: #f3f4f6;
	}
	.text-gray-800 {
		color: #1f2937;
	}
	.bg-blue-100 {
		background-color: #dbeafe;
	}
	.text-blue-800 {
		color: #1e40af;
	}
	.bg-red-100 {
		background-color: #fee2e2;
	}
	.text-red-800 {
		color: #991b1b;
	}
</style>

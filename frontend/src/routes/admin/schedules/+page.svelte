<!--
	Admin Schedule Management - Trading Room Schedules
	═══════════════════════════════════════════════════════════════════════════════════
	Apple Principal Engineer ICT 11+ Grade - January 2026

	Features:
	- Per-room schedule management with ROOMS config integration
	- Weekly calendar view with navigation
	- CRUD operations for schedule events
	- Exception handling (holidays, cancellations)
	- Bulk operations and conflict detection
	- Full Svelte 5 runes implementation

	R21-C (2026-05-20): extracted 9 leaf components into _components/.
	Page LOC fell from 2005 → ~600. Behaviour preserved verbatim.
-->
<script lang="ts">
	import { SvelteSet } from 'svelte/reactivity';
	import { onDestroy, onMount } from 'svelte';
	import { ROOMS } from '$lib/config/rooms';
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';
	import IconCalendar from '@tabler/icons-svelte-runes/icons/calendar';
	import IconPlus from '@tabler/icons-svelte-runes/icons/plus';
	import AlertBanner from './_components/AlertBanner.svelte';
	import PageHeaderBar from './_components/PageHeaderBar.svelte';
	import RoomSelector from './_components/RoomSelector.svelte';
	import SchedulesToolbar from './_components/SchedulesToolbar.svelte';
	import BulkActionsBar from './_components/BulkActionsBar.svelte';
	import WeeklyGrid from './_components/WeeklyGrid.svelte';
	import WeekendSection from './_components/WeekendSection.svelte';
	import ScheduleFormModal from './_components/ScheduleFormModal.svelte';
	import type { ScheduleEvent, ScheduleForm } from './_components/types';
	import {
		getSchedules,
		postSchedule,
		putSchedule,
		deleteScheduleById,
		bulkDeleteSchedules,
		bulkUpdateSchedules
	} from './schedules.remote';

	// STATE - Svelte 5 Runes

	// Room selection state
	let selectedRoomId = $state<string>(ROOMS[0]?.id || '');

	// Data state
	let schedules = $state<ScheduleEvent[]>([]);
	let loading = $state(true);
	let saving = $state(false);
	let error = $state<string | null>(null);
	let success = $state<string | null>(null);

	// Calendar navigation
	let currentWeekStart = $state(getStartOfWeek(new Date()));

	// Modal state
	let showModal = $state(false);
	let editingSchedule = $state<ScheduleEvent | null>(null);
	let formData = $state<ScheduleForm>(getDefaultFormData());

	// Bulk selection
	let selectedIds = new SvelteSet<number>();
	// Pure projection of selection size; SvelteSet is reactive for size/has/iteration.
	let showBulkActions = $derived(selectedIds.size > 0);

	// Delete confirmation modal state
	let showDeleteModal = $state(false);
	let showBulkDeleteModal = $state(false);
	let pendingDeleteId = $state<number | null>(null);

	// Filter state
	let filterActive = $state<'all' | 'active' | 'inactive'>('all');
	let filterDay = $state<number | null>(null);

	// CONSTANTS

	const DAYS = ['Sunday', 'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday'];
	const DAY_MS = 24 * 60 * 60 * 1000;

	const TIMEZONES = [
		{ value: 'America/New_York', label: 'Eastern (ET)' },
		{ value: 'America/Chicago', label: 'Central (CT)' },
		{ value: 'America/Denver', label: 'Mountain (MT)' },
		{ value: 'America/Los_Angeles', label: 'Pacific (PT)' },
		{ value: 'UTC', label: 'UTC' }
	];

	const ROOM_TYPES = [
		{ value: 'live' as const, label: 'Live Session' },
		{ value: 'recorded' as const, label: 'Recorded' },
		{ value: 'hybrid' as const, label: 'Hybrid' }
	];

	// DERIVED STATE - Svelte 5 $derived

	/**
	 * Currently selected room from ROOMS config
	 */
	const selectedRoom = $derived(ROOMS.find((r) => r.id === selectedRoomId) || ROOMS[0]);

	/**
	 * Filtered schedules based on active/inactive filter
	 */
	const filteredSchedules = $derived.by(() => {
		let result = schedules;

		if (filterActive === 'active') {
			result = result.filter((s) => s.is_active);
		} else if (filterActive === 'inactive') {
			result = result.filter((s) => !s.is_active);
		}

		if (filterDay !== null) {
			result = result.filter((s) => s.day_of_week === filterDay);
		}

		return result;
	});

	/**
	 * Schedules grouped by day of week
	 */
	const schedulesByDay = $derived.by(() => {
		const grouped: Record<number, ScheduleEvent[]> = {};
		for (let i = 0; i < 7; i++) {
			grouped[i] = [];
		}

		for (const schedule of filteredSchedules) {
			if (grouped[schedule.day_of_week]) {
				grouped[schedule.day_of_week].push(schedule);
			}
		}

		// Sort each day by start time
		for (const day of Object.keys(grouped)) {
			grouped[Number(day)].sort((a, b) => a.start_time.localeCompare(b.start_time));
		}

		return grouped;
	});

	/**
	 * Check if there are any schedules
	 */
	const hasSchedules = $derived(schedules.length > 0);

	/**
	 * Total schedule count for current room
	 */
	const totalSchedules = $derived(schedules.length);

	/**
	 * Active schedule count
	 */
	const activeSchedules = $derived(schedules.filter((s) => s.is_active).length);

	/**
	 * Week dates for calendar header
	 */
	const weekDates = $derived.by(() => {
		const dates: Date[] = [];
		for (let i = 0; i < 7; i++) {
			dates.push(new Date(currentWeekStart.getTime() + i * DAY_MS));
		}
		return dates;
	});

	/**
	 * Format week range for display
	 */
	const weekRangeText = $derived.by(() => {
		const start = weekDates[0];
		const end = weekDates[6];
		const options: Intl.DateTimeFormatOptions = { month: 'short', day: 'numeric' };
		return `${start.toLocaleDateString('en-US', options)} - ${end.toLocaleDateString('en-US', options)}, ${end.getFullYear()}`;
	});

	/**
	 * Detect schedule conflicts (overlapping times on same day)
	 */
	// FIX-2026-04-26 (P1): Convert per-event "HH:MM" + IANA timezone to absolute
	// UTC minutes within the week so two events at 09:30 ET and 09:30 PT (3 hours
	// apart in real time) no longer string-lex compare as conflicting, and 23:30
	// in one zone vs 02:30 in another are correctly detected as adjacent/overlapping.
	function toUtcMinutesForWeek(
		dayOfWeek: number,
		hhmm: string,
		timezone: string
	): { start: number; end: number } | null {
		const [hStr, mStr] = (hhmm || '').split(':');
		const h = parseInt(hStr ?? '');
		const m = parseInt(mStr ?? '');
		if (Number.isNaN(h) || Number.isNaN(m)) return null;

		// Anchor the schedule to a fixed reference week (Sunday, 2024-01-07) so
		// `dayOfWeek` becomes a real wall-clock instant, then ask Intl what the
		// timezone offset is at that instant. Returns minutes-since-week-start in UTC.
		const baseSundayUtcMs = Date.UTC(2024, 0, 7);
		const wallUtcMs = baseSundayUtcMs + dayOfWeek * 86400000 + h * 3600000 + m * 60000;
		try {
			// Intl.DateTimeFormat lets us compute the tz offset at an instant.
			const dtf = new Intl.DateTimeFormat('en-US', {
				timeZone: timezone || 'UTC',
				year: 'numeric',
				month: '2-digit',
				day: '2-digit',
				hour: '2-digit',
				minute: '2-digit',
				second: '2-digit',
				hour12: false
			});
			const parts = dtf.formatToParts(new Date(wallUtcMs));
			const get = (t: string) => parts.find((p) => p.type === t)?.value ?? '0';
			const asUtcOfLocal = Date.UTC(
				parseInt(get('year')),
				parseInt(get('month')) - 1,
				parseInt(get('day')),
				parseInt(get('hour')) % 24,
				parseInt(get('minute')),
				parseInt(get('second'))
			);
			const offsetMs = asUtcOfLocal - wallUtcMs;
			// Real instant = wall - offset (wall is interpreted in the named tz).
			return { start: wallUtcMs - offsetMs, end: 0 };
		} catch {
			return null;
		}
	}

	function scheduleToUtcRange(s: ScheduleEvent): { start: number; end: number } | null {
		const startInfo = toUtcMinutesForWeek(s.day_of_week, s.start_time, s.timezone || 'UTC');
		const endInfo = toUtcMinutesForWeek(s.day_of_week, s.end_time, s.timezone || 'UTC');
		if (!startInfo || !endInfo) return null;
		let start = startInfo.start;
		let end = endInfo.start;
		// Wrap past midnight: 23:30 → 02:30 should not yield negative range.
		if (end <= start) end += 86400000;
		return { start, end };
	}

	const conflicts = $derived.by(() => {
		const conflictPairs: Array<[ScheduleEvent, ScheduleEvent]> = [];
		const ranges = schedules
			.filter((s) => s.is_active)
			.map((s) => ({ s, range: scheduleToUtcRange(s) }))
			.filter(
				(x): x is { s: ScheduleEvent; range: { start: number; end: number } } => x.range !== null
			);
		for (let i = 0; i < ranges.length; i++) {
			for (let j = i + 1; j < ranges.length; j++) {
				const a = ranges[i];
				const b = ranges[j];
				// True overlap on absolute UTC instants.
				if (a.range.start < b.range.end && b.range.start < a.range.end) {
					conflictPairs.push([a.s, b.s]);
				}
			}
		}
		return conflictPairs;
	});

	// EFFECTS - Svelte 5 runes

	// FIX-2026-04-26 (P2): one-shot data load. Was `$effect` keyed on selectedRoomId
	// which fires write-while-reading cascades on first paint (see commit 34a0bd070).
	// We still need to reload when the user switches rooms; that is wired via the
	// <select onchange> handler `selectRoom()` below.
	onMount(() => {
		if (selectedRoomId) {
			void loadSchedules();
		}
	});

	let successTimeout: ReturnType<typeof setTimeout> | null = null;

	function clearSuccessMessage() {
		success = null;
		if (successTimeout) {
			clearTimeout(successTimeout);
			successTimeout = null;
		}
	}

	function setSuccessMessage(message: string) {
		if (successTimeout) clearTimeout(successTimeout);
		success = message;
		successTimeout = setTimeout(clearSuccessMessage, 3000);
	}

	onDestroy(() => {
		if (successTimeout) clearTimeout(successTimeout);
	});

	// HELPER FUNCTIONS

	function getStartOfWeek(date: Date): Date {
		return new Date(date.getFullYear(), date.getMonth(), date.getDate() - date.getDay());
	}

	function getDefaultFormData(): ScheduleForm {
		return {
			room_id: selectedRoomId || ROOMS[0]?.id || '',
			title: '',
			description: '',
			trader_name: '',
			day_of_week: 1,
			start_time: '09:00',
			end_time: '10:00',
			timezone: 'America/New_York',
			room_type: 'live',
			recurrence: 'weekly',
			is_active: true
		};
	}

	function formatTime(time: string): string {
		const [hours, minutes] = time.split(':');
		const h = parseInt(hours);
		const ampm = h >= 12 ? 'PM' : 'AM';
		const displayHour = h % 12 || 12;
		return `${displayHour}:${minutes} ${ampm}`;
	}

	function formatDate(date: Date): string {
		return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
	}

	// API FUNCTIONS

	async function loadSchedules() {
		loading = true;
		error = null;

		try {
			// Auth (incl. 401 → /login) is enforced server-side by
			// admin/+layout.server.ts + hooks.server.ts before this page renders.
			// `.refresh()` (not a bare `await`) because remote queries are cached by
			// argument: after a mutation, re-reading `getSchedules(sameRoomId)` would
			// otherwise return the stale cached list. refresh() forces a fresh fetch.
			const q = getSchedules(selectedRoomId);
			await q.refresh();
			schedules = q.current ?? [];
		} catch (e) {
			console.error('Failed to load schedules:', e);
			error = e instanceof Error ? e.message : 'Failed to load schedules';
			schedules = [];
		} finally {
			loading = false;
		}
	}

	// FIX-2026-04-26 (P1): require start_time < end_time. Compares minutes-of-day
	// since cross-midnight schedules aren't representable in this single-day form
	// (a cross-midnight session must be split into two day_of_week entries).
	function validateForm(form: ScheduleForm): string | null {
		const [sh, sm] = (form.start_time || '').split(':').map((n) => parseInt(n));
		const [eh, em] = (form.end_time || '').split(':').map((n) => parseInt(n));
		if (Number.isNaN(sh) || Number.isNaN(sm) || Number.isNaN(eh) || Number.isNaN(em)) {
			return 'Start and end times are required (HH:MM).';
		}
		const startMin = sh * 60 + sm;
		const endMin = eh * 60 + em;
		if (endMin <= startMin) {
			return 'End time must be after start time.';
		}
		return null;
	}

	async function createSchedule() {
		if (!selectedRoom) return;
		const validationError = validateForm(formData);
		if (validationError) {
			error = validationError;
			return;
		}
		saving = true;
		error = null;

		try {
			await postSchedule({ ...formData, room_id: selectedRoomId });

			setSuccessMessage('Schedule created successfully');
			closeModal();
			await loadSchedules();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to create schedule';
		} finally {
			saving = false;
		}
	}

	async function updateSchedule() {
		if (!editingSchedule) return;
		const validationError = validateForm(formData);
		if (validationError) {
			error = validationError;
			return;
		}
		saving = true;
		error = null;

		try {
			await putSchedule({ id: editingSchedule.id, data: { ...formData } });

			setSuccessMessage('Schedule updated successfully');
			closeModal();
			await loadSchedules();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to update schedule';
		} finally {
			saving = false;
		}
	}

	function deleteSchedule(id: number) {
		pendingDeleteId = id;
		showDeleteModal = true;
	}

	async function confirmDeleteSchedule() {
		if (pendingDeleteId === null) return;
		showDeleteModal = false;
		const id = pendingDeleteId;
		pendingDeleteId = null;
		try {
			await deleteScheduleById(id);

			setSuccessMessage('Schedule deleted successfully');
			selectedIds.delete(id);
			//  mutations are reactive - no reassignment needed
			await loadSchedules();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to delete schedule';
		}
	}

	async function toggleScheduleActive(schedule: ScheduleEvent) {
		try {
			await putSchedule({ id: schedule.id, data: { is_active: !schedule.is_active } });

			setSuccessMessage(`Schedule ${schedule.is_active ? 'deactivated' : 'activated'}`);
			await loadSchedules();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to update schedule';
		}
	}

	async function duplicateSchedule(schedule: ScheduleEvent) {
		try {
			await postSchedule({
				...schedule,
				id: undefined,
				title: `${schedule.title} (Copy)`,
				created_at: undefined,
				updated_at: undefined
			});

			setSuccessMessage('Schedule duplicated successfully');
			await loadSchedules();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to duplicate schedule';
		}
	}

	function bulkDelete() {
		showBulkDeleteModal = true;
	}

	async function confirmBulkDelete() {
		showBulkDeleteModal = false;
		try {
			await bulkDeleteSchedules(Array.from(selectedIds));

			setSuccessMessage(`${selectedIds.size} schedules deleted`);
			selectedIds.clear();
			await loadSchedules();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to delete schedules';
		}
	}

	async function bulkToggleActive(active: boolean) {
		try {
			await bulkUpdateSchedules({ ids: Array.from(selectedIds), data: { is_active: active } });

			setSuccessMessage(`${selectedIds.size} schedules ${active ? 'activated' : 'deactivated'}`);
			selectedIds.clear();
			await loadSchedules();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to update schedules';
		}
	}

	// UI FUNCTIONS

	function selectRoom(roomId: string) {
		selectedRoomId = roomId;
		// FIX-2026-04-26 (P2-7): clear bulk selection so a hidden ID can't be bulk-deleted.
		selectedIds.clear();
		// FIX-2026-04-26 (P2): Was previously $effect(()=>{ if(selectedRoomId) loadSchedules() })
		// — replaced with explicit reload here to break the write-while-reading cascade.
		void loadSchedules();
	}

	function openCreateModal() {
		editingSchedule = null;
		formData = getDefaultFormData();
		showModal = true;
	}

	function openEditModal(schedule: ScheduleEvent) {
		editingSchedule = schedule;
		formData = {
			room_id: schedule.room_id,
			title: schedule.title,
			description: schedule.description || '',
			trader_name: schedule.trader_name || '',
			day_of_week: schedule.day_of_week,
			start_time: schedule.start_time,
			end_time: schedule.end_time,
			timezone: schedule.timezone,
			room_type: schedule.room_type,
			recurrence: schedule.recurrence,
			is_active: schedule.is_active
		};
		showModal = true;
	}

	function closeModal() {
		showModal = false;
		editingSchedule = null;
		formData = getDefaultFormData();
	}

	function handleSubmit() {
		if (editingSchedule) {
			updateSchedule();
		} else {
			createSchedule();
		}
	}

	function navigateWeek(direction: number) {
		currentWeekStart = new Date(currentWeekStart.getTime() + direction * 7 * DAY_MS);
	}

	function goToCurrentWeek() {
		currentWeekStart = getStartOfWeek(new Date());
	}

	function setFilterActive(value: 'all' | 'active' | 'inactive') {
		filterActive = value;
		selectedIds.clear();
	}

	function setFilterDay(value: number | null) {
		filterDay = value;
		selectedIds.clear();
	}

	function toggleSelection(id: number) {
		if (selectedIds.has(id)) {
			selectedIds.delete(id);
		} else {
			selectedIds.add(id);
		}
		//  mutations are reactive - no reassignment needed
	}

	function isConflicting(scheduleId: number): boolean {
		return conflicts.some(([a, b]) => a.id === scheduleId || b.id === scheduleId);
	}

	function addOnDay(dayIndex: number) {
		formData.day_of_week = dayIndex;
		openCreateModal();
	}
</script>

<svelte:head>
	<title>Schedule Management | Admin | RTP</title>
</svelte:head>

<div class="admin-schedules">
	<div class="admin-page-container">
		<!-- Animated Background -->
		<div class="bg-effects">
			<div class="bg-blob bg-blob-1"></div>
			<div class="bg-blob bg-blob-2"></div>
			<div class="bg-blob bg-blob-3"></div>
		</div>

		<PageHeaderBar conflictCount={conflicts.length} onaddSchedule={openCreateModal} />

		<!-- Alerts -->
		{#if error}
			<AlertBanner variant="error" message={error} ondismiss={() => (error = null)} />
		{/if}

		{#if success}
			<AlertBanner variant="success" message={success} ondismiss={clearSuccessMessage} />
		{/if}

		<RoomSelector
			rooms={ROOMS}
			{selectedRoomId}
			{selectedRoom}
			{activeSchedules}
			{totalSchedules}
			onselectRoom={selectRoom}
		/>

		<SchedulesToolbar
			{weekRangeText}
			{filterActive}
			{filterDay}
			days={DAYS}
			onnavigateWeek={navigateWeek}
			ongoToCurrentWeek={goToCurrentWeek}
			onrefresh={loadSchedules}
			onfilterActiveChange={setFilterActive}
			onfilterDayChange={setFilterDay}
		/>

		<!-- Bulk Actions Bar -->
		{#if showBulkActions}
			<BulkActionsBar
				selectedCount={selectedIds.size}
				onbulkToggleActive={bulkToggleActive}
				onbulkDelete={bulkDelete}
				onclear={() => selectedIds.clear()}
			/>
		{/if}

		<!-- Schedule Grid -->
		<div class="schedule-container">
			{#if loading}
				<div class="loading-state">
					<div class="spinner"></div>
					<p>Loading schedules...</p>
				</div>
			{:else if !hasSchedules}
				<div class="empty-state">
					<IconCalendar size={64} stroke={1} />
					<h3>No Schedules Yet</h3>
					<p>Create your first schedule for {selectedRoom?.name || 'this room'}</p>
					<button class="btn btn-primary" onclick={openCreateModal}>
						<IconPlus size={18} />
						Create Schedule
					</button>
				</div>
			{:else}
				<WeeklyGrid
					days={DAYS}
					{schedulesByDay}
					{weekDates}
					{selectedIds}
					{isConflicting}
					{formatDate}
					{formatTime}
					onToggleSelect={toggleSelection}
					onToggleActive={toggleScheduleActive}
					onDuplicate={duplicateSchedule}
					onEdit={openEditModal}
					onDelete={deleteSchedule}
					onAddOnDay={addOnDay}
				/>

				<WeekendSection
					days={DAYS}
					{schedulesByDay}
					{formatTime}
					onEdit={openEditModal}
					onDelete={deleteSchedule}
				/>
			{/if}
		</div>
	</div>
	<!-- End admin-page-container -->
</div>

<ScheduleFormModal
	open={showModal}
	bind:formData
	days={DAYS}
	timezones={TIMEZONES}
	roomTypes={ROOM_TYPES}
	{saving}
	isEdit={editingSchedule !== null}
	onsubmit={handleSubmit}
	onclose={closeModal}
/>

<ConfirmationModal
	isOpen={showDeleteModal}
	title="Delete Schedule"
	message="Are you sure you want to delete this schedule?"
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmDeleteSchedule}
	onCancel={() => {
		showDeleteModal = false;
		pendingDeleteId = null;
	}}
/>

<ConfirmationModal
	isOpen={showBulkDeleteModal}
	title="Delete Schedules"
	message={`Delete ${selectedIds.size} selected schedules?`}
	confirmText="Delete All"
	variant="danger"
	onConfirm={confirmBulkDelete}
	onCancel={() => (showBulkDeleteModal = false)}
/>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * ADMIN SCHEDULES - Apple ICT 11+ Grade Styling
	 * R21-C (2026-05-20): trimmed to selectors used by *this* file only;
	 * sub-component CSS lives in _components/*.svelte.
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.admin-schedules {
		padding: 24px;
		max-width: 1600px;
		margin: 0 auto;
		font-family: 'Montserrat', var(--font-body), sans-serif;
	}

	/* Schedule Container — the white card that wraps the grid + weekend section */
	.schedule-container {
		background: #fff;
		border-radius: 12px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
		overflow: hidden;
	}

	/* States */
	.loading-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 80px 20px;
		text-align: center;
	}

	.loading-state p,
	.empty-state p {
		color: #64748b;
		margin: 8px 0 20px;
	}

	.empty-state h3 {
		font-size: 20px;
		font-weight: 700;
		color: #1e293b;
		margin: 16px 0 8px;
	}

	.empty-state :global(svg) {
		color: #cbd5e1;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid #e2e8f0;
		border-top-color: #143e59;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Buttons used directly by this file (empty-state "Create Schedule") */
	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 10px 20px;
		font-size: 14px;
		font-weight: 600;
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.2s ease;
		border: none;
	}

	.btn-primary {
		background: #143e59;
		color: #fff;
	}

	.btn-primary:hover {
		background: #0d2a3d;
	}

	@media (max-width: 767.98px) {
		.admin-schedules {
			padding: 16px;
		}
	}
</style>

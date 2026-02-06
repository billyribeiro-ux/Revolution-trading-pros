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

	@version 2.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	import { goto } from '$app/navigation';
	import { ROOMS } from '$lib/config/rooms';
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';
	import IconCalendar from '@tabler/icons-svelte-runes/icons/calendar';
	import IconPlus from '@tabler/icons-svelte-runes/icons/plus';
	import IconEdit from '@tabler/icons-svelte-runes/icons/edit';
	import IconTrash from '@tabler/icons-svelte-runes/icons/trash';
	import IconChevronLeft from '@tabler/icons-svelte-runes/icons/chevron-left';
	import IconChevronRight from '@tabler/icons-svelte-runes/icons/chevron-right';
	import IconClock from '@tabler/icons-svelte-runes/icons/clock';
	import IconUser from '@tabler/icons-svelte-runes/icons/user';
	import IconAlertCircle from '@tabler/icons-svelte-runes/icons/alert-circle';
	import IconCheck from '@tabler/icons-svelte-runes/icons/check';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconCopy from '@tabler/icons-svelte-runes/icons/copy';
	import IconRefresh from '@tabler/icons-svelte-runes/icons/refresh';

	// ═══════════════════════════════════════════════════════════════════════════
	// TYPE DEFINITIONS
	// ═══════════════════════════════════════════════════════════════════════════

	interface ScheduleEvent {
		id: number;
		room_id: string;
		title: string;
		description: string | null;
		trader_name: string | null;
		day_of_week: number; // 0 = Sunday, 1 = Monday, etc.
		start_time: string; // HH:MM format
		end_time: string;
		timezone: string;
		is_active: boolean;
		room_type: 'live' | 'recorded' | 'hybrid';
		recurrence: 'weekly' | 'biweekly' | 'monthly' | null;
		exceptions: ScheduleException[];
		created_at: string;
		updated_at: string;
	}

	interface ScheduleException {
		id: number;
		schedule_id: number;
		date: string; // YYYY-MM-DD
		type: 'cancelled' | 'rescheduled' | 'holiday';
		reason: string | null;
		new_start_time?: string;
		new_end_time?: string;
	}

	interface ScheduleForm {
		room_id: string;
		title: string;
		description: string;
		trader_name: string;
		day_of_week: number;
		start_time: string;
		end_time: string;
		timezone: string;
		room_type: 'live' | 'recorded' | 'hybrid';
		recurrence: 'weekly' | 'biweekly' | 'monthly' | null;
		is_active: boolean;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE - Svelte 5 Runes
	// ═══════════════════════════════════════════════════════════════════════════

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
	let selectedIds = $state<Set<number>>(new Set());
	let showBulkActions = $state(false);

	// Delete confirmation modal state
	let showDeleteModal = $state(false);
	let showBulkDeleteModal = $state(false);
	let pendingDeleteId = $state<number | null>(null);

	// Filter state
	let filterActive = $state<'all' | 'active' | 'inactive'>('all');
	let filterDay = $state<number | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// CONSTANTS
	// ═══════════════════════════════════════════════════════════════════════════

	const DAYS = ['Sunday', 'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday'];

	const TIMEZONES = [
		{ value: 'America/New_York', label: 'Eastern (ET)' },
		{ value: 'America/Chicago', label: 'Central (CT)' },
		{ value: 'America/Denver', label: 'Mountain (MT)' },
		{ value: 'America/Los_Angeles', label: 'Pacific (PT)' },
		{ value: 'UTC', label: 'UTC' }
	];

	const ROOM_TYPES = [
		{ value: 'live', label: 'Live Session' },
		{ value: 'recorded', label: 'Recorded' },
		{ value: 'hybrid', label: 'Hybrid' }
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE - Svelte 5 $derived
	// ═══════════════════════════════════════════════════════════════════════════

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
			const date = new Date(currentWeekStart);
			date.setDate(date.getDate() + i);
			dates.push(date);
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
	const conflicts = $derived.by(() => {
		const conflictPairs: Array<[ScheduleEvent, ScheduleEvent]> = [];

		for (const day of Object.keys(schedulesByDay)) {
			const daySchedules = schedulesByDay[Number(day)];
			for (let i = 0; i < daySchedules.length; i++) {
				for (let j = i + 1; j < daySchedules.length; j++) {
					const a = daySchedules[i];
					const b = daySchedules[j];

					// Check for overlap
					if (a.start_time < b.end_time && b.start_time < a.end_time) {
						conflictPairs.push([a, b]);
					}
				}
			}
		}

		return conflictPairs;
	});

	const hasConflicts = $derived(conflicts.length > 0);

	// ═══════════════════════════════════════════════════════════════════════════
	// EFFECTS - Svelte 5 $effect
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Load schedules when selected room changes
	 */
	$effect(() => {
		if (selectedRoomId) {
			loadSchedules();
		}
	});

	/**
	 * Clear success message after delay
	 */
	$effect(() => {
		if (!success) return;

		const timeout = setTimeout(() => {
			success = null;
		}, 3000);

		return () => clearTimeout(timeout);
	});

	/**
	 * Update bulk actions visibility
	 */
	$effect(() => {
		showBulkActions = selectedIds.size > 0;
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// HELPER FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	function getStartOfWeek(date: Date): Date {
		const d = new Date(date);
		const day = d.getDay();
		const diff = d.getDate() - day;
		d.setDate(diff);
		d.setHours(0, 0, 0, 0);
		return d;
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

	// ═══════════════════════════════════════════════════════════════════════════
	// API FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	async function loadSchedules() {
		loading = true;
		error = null;

		try {
			const response = await fetch(`/api/admin/schedules?room_id=${selectedRoomId}`, {
				credentials: 'include'
			});

			if (!response.ok) {
				if (response.status === 401) {
					goto('/login');
					return;
				}
				throw new Error('Failed to load schedules');
			}

			const data = await response.json();
			schedules = data.data || data.schedules || [];
		} catch (e) {
			console.error('Failed to load schedules:', e);
			error = e instanceof Error ? e.message : 'Failed to load schedules';
			schedules = [];
		} finally {
			loading = false;
		}
	}

	async function createSchedule() {
		if (!selectedRoom) return;
		saving = true;
		error = null;

		try {
			const response = await fetch('/api/admin/schedules', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				credentials: 'include',
				body: JSON.stringify({
					...formData,
					room_id: selectedRoomId
				})
			});

			if (!response.ok) {
				const data = await response.json();
				throw new Error(data.error || data.message || 'Failed to create schedule');
			}

			success = 'Schedule created successfully';
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
		saving = true;
		error = null;

		try {
			const response = await fetch(`/api/admin/schedules/${editingSchedule.id}`, {
				method: 'PUT',
				headers: { 'Content-Type': 'application/json' },
				credentials: 'include',
				body: JSON.stringify(formData)
			});

			if (!response.ok) {
				const data = await response.json();
				throw new Error(data.error || data.message || 'Failed to update schedule');
			}

			success = 'Schedule updated successfully';
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
			const response = await fetch(`/api/admin/schedules/${id}`, {
				method: 'DELETE',
				credentials: 'include'
			});

			if (!response.ok) throw new Error('Failed to delete schedule');

			success = 'Schedule deleted successfully';
			selectedIds.delete(id);
			selectedIds = new Set(selectedIds);
			await loadSchedules();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to delete schedule';
		}
	}

	async function toggleScheduleActive(schedule: ScheduleEvent) {
		try {
			const response = await fetch(`/api/admin/schedules/${schedule.id}`, {
				method: 'PUT',
				headers: { 'Content-Type': 'application/json' },
				credentials: 'include',
				body: JSON.stringify({ is_active: !schedule.is_active })
			});

			if (!response.ok) throw new Error('Failed to update schedule');

			success = `Schedule ${schedule.is_active ? 'deactivated' : 'activated'}`;
			await loadSchedules();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to update schedule';
		}
	}

	async function duplicateSchedule(schedule: ScheduleEvent) {
		try {
			const response = await fetch('/api/admin/schedules', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				credentials: 'include',
				body: JSON.stringify({
					...schedule,
					id: undefined,
					title: `${schedule.title} (Copy)`,
					created_at: undefined,
					updated_at: undefined
				})
			});

			if (!response.ok) throw new Error('Failed to duplicate schedule');

			success = 'Schedule duplicated successfully';
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
			const response = await fetch('/api/admin/schedules/bulk-delete', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				credentials: 'include',
				body: JSON.stringify({ ids: Array.from(selectedIds) })
			});

			if (!response.ok) throw new Error('Failed to delete schedules');

			success = `${selectedIds.size} schedules deleted`;
			selectedIds = new Set();
			await loadSchedules();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to delete schedules';
		}
	}

	async function bulkToggleActive(active: boolean) {
		try {
			const response = await fetch('/api/admin/schedules/bulk-update', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				credentials: 'include',
				body: JSON.stringify({
					ids: Array.from(selectedIds),
					data: { is_active: active }
				})
			});

			if (!response.ok) throw new Error('Failed to update schedules');

			success = `${selectedIds.size} schedules ${active ? 'activated' : 'deactivated'}`;
			selectedIds = new Set();
			await loadSchedules();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to update schedules';
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// UI FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	function selectRoom(roomId: string) {
		selectedRoomId = roomId;
		selectedIds = new Set();
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

	function handleOverlayClick(e: MouseEvent) {
		if (e.target === e.currentTarget) {
			closeModal();
		}
	}

	function handleOverlayKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			closeModal();
		}
	}

	function handleSubmit() {
		if (editingSchedule) {
			updateSchedule();
		} else {
			createSchedule();
		}
	}

	function navigateWeek(direction: number) {
		const newDate = new Date(currentWeekStart);
		newDate.setDate(newDate.getDate() + direction * 7);
		currentWeekStart = newDate;
	}

	function goToCurrentWeek() {
		currentWeekStart = getStartOfWeek(new Date());
	}

	function toggleSelection(id: number) {
		if (selectedIds.has(id)) {
			selectedIds.delete(id);
		} else {
			selectedIds.add(id);
		}
		selectedIds = new Set(selectedIds);
	}

	function isConflicting(scheduleId: number): boolean {
		return conflicts.some(([a, b]) => a.id === scheduleId || b.id === scheduleId);
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

		<!-- Header -->
		<header class="page-header">
			<div class="header-content">
				<div class="header-title">
					<IconCalendar size={32} />
					<div>
						<h1>Trading Room Schedules</h1>
						<p class="subtitle">Manage weekly schedules for each trading room and service</p>
					</div>
				</div>
				<div class="header-actions">
					{#if hasConflicts}
						<div class="conflict-warning">
							<IconAlertCircle size={18} />
							<span>{conflicts.length} time conflict{conflicts.length !== 1 ? 's' : ''}</span>
						</div>
					{/if}
					<button class="btn btn-primary" onclick={openCreateModal}>
						<IconPlus size={18} />
						Add Schedule
					</button>
				</div>
			</div>
		</header>

		<!-- Alerts -->
		{#if error}
			<div class="alert alert-error" role="alert">
				<IconAlertCircle size={18} />
				<span>{error}</span>
				<button onclick={() => (error = null)} aria-label="Dismiss">
					<IconX size={18} />
				</button>
			</div>
		{/if}

		{#if success}
			<div class="alert alert-success" role="status">
				<IconCheck size={18} />
				<span>{success}</span>
				<button onclick={() => (success = null)} aria-label="Dismiss">
					<IconX size={18} />
				</button>
			</div>
		{/if}

		<!-- Room Selector Tabs -->
		<div class="room-selector">
			<div class="room-tabs" role="tablist">
				{#each ROOMS as room}
					<button
						class="room-tab"
						class:active={selectedRoomId === room.id}
						onclick={() => selectRoom(room.id)}
						role="tab"
						aria-selected={selectedRoomId === room.id}
						style="--room-color: {room.color}"
					>
						<span class="room-icon">{room.icon}</span>
						<span class="room-name">{room.shortName}</span>
					</button>
				{/each}
			</div>

			<div class="room-info">
				{#if selectedRoom}
					<span class="room-full-name" style="color: {selectedRoom.color}">{selectedRoom.name}</span
					>
					<span class="room-stats">
						{activeSchedules} active / {totalSchedules} total
					</span>
				{/if}
			</div>
		</div>

		<!-- Toolbar -->
		<div class="toolbar">
			<!-- Week Navigation -->
			<div class="week-nav">
				<button class="btn btn-icon" onclick={() => navigateWeek(-1)} title="Previous week">
					<IconChevronLeft size={20} />
				</button>
				<span class="week-label">{weekRangeText}</span>
				<button class="btn btn-icon" onclick={() => navigateWeek(1)} title="Next week">
					<IconChevronRight size={20} />
				</button>
				<button class="btn btn-text" onclick={goToCurrentWeek}>Today</button>
			</div>

			<!-- Filters -->
			<div class="filters">
				<select bind:value={filterActive} class="filter-select">
					<option value="all">All Schedules</option>
					<option value="active">Active Only</option>
					<option value="inactive">Inactive Only</option>
				</select>

				<select bind:value={filterDay} class="filter-select">
					<option value={null}>All Days</option>
					{#each DAYS as day, index}
						<option value={index}>{day}</option>
					{/each}
				</select>

				<button class="btn btn-icon" onclick={loadSchedules} title="Refresh">
					<IconRefresh size={18} />
				</button>
			</div>
		</div>

		<!-- Bulk Actions Bar -->
		{#if showBulkActions}
			<div class="bulk-actions">
				<span class="bulk-count">{selectedIds.size} selected</span>
				<button class="btn btn-sm" onclick={() => bulkToggleActive(true)}>
					<IconCheck size={16} />
					Activate
				</button>
				<button class="btn btn-sm" onclick={() => bulkToggleActive(false)}>
					<IconX size={16} />
					Deactivate
				</button>
				<button class="btn btn-sm btn-danger" onclick={bulkDelete}>
					<IconTrash size={16} />
					Delete
				</button>
				<button class="btn btn-sm btn-text" onclick={() => (selectedIds = new Set())}>
					Clear
				</button>
			</div>
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
				<!-- Weekly Calendar Grid -->
				<div class="weekly-grid">
					{#each [1, 2, 3, 4, 5] as dayIndex}
						{@const daySchedules = schedulesByDay[dayIndex] || []}
						{@const dayDate = weekDates[dayIndex]}
						<div class="day-column">
							<div class="day-header">
								<span class="day-name">{DAYS[dayIndex]}</span>
								<span class="day-date">{formatDate(dayDate)}</span>
							</div>
							<div class="day-events">
								{#each daySchedules as event}
									<div
										class="event-card"
										class:inactive={!event.is_active}
										class:selected={selectedIds.has(event.id)}
										class:conflict={isConflicting(event.id)}
									>
										<div class="event-checkbox">
											<input
												id="page-checkbox"
												name="page-checkbox"
												type="checkbox"
												checked={selectedIds.has(event.id)}
												onchange={() => toggleSelection(event.id)}
												aria-label="Select schedule"
											/>
										</div>
										<div class="event-content">
											<div class="event-time">
												<IconClock size={14} />
												{formatTime(event.start_time)} - {formatTime(event.end_time)}
											</div>
											<div class="event-title">{event.title}</div>
											{#if event.trader_name}
												<div class="event-trader">
													<IconUser size={12} />
													{event.trader_name}
												</div>
											{/if}
											<div class="event-badges">
												<span class="badge badge-{event.room_type}">{event.room_type}</span>
												{#if !event.is_active}
													<span class="badge badge-inactive">Inactive</span>
												{/if}
											</div>
										</div>
										<div class="event-actions">
											<button
												class="btn-icon"
												onclick={() => toggleScheduleActive(event)}
												title={event.is_active ? 'Deactivate' : 'Activate'}
											>
												{#if event.is_active}
													<IconCheck size={16} />
												{:else}
													<IconX size={16} />
												{/if}
											</button>
											<button
												class="btn-icon"
												onclick={() => duplicateSchedule(event)}
												title="Duplicate"
											>
												<IconCopy size={16} />
											</button>
											<button class="btn-icon" onclick={() => openEditModal(event)} title="Edit">
												<IconEdit size={16} />
											</button>
											<button
												class="btn-icon btn-danger"
												onclick={() => deleteSchedule(event.id)}
												title="Delete"
											>
												<IconTrash size={16} />
											</button>
										</div>
									</div>
								{/each}
								{#if daySchedules.length === 0}
									<div class="no-events">
										<span>No events</span>
										<button
											class="btn-link"
											onclick={() => {
												formData.day_of_week = dayIndex;
												openCreateModal();
											}}
										>
											+ Add
										</button>
									</div>
								{/if}
							</div>
						</div>
					{/each}
				</div>

				<!-- Weekend Section -->
				<details class="weekend-section">
					<summary>
						<span>Weekend Schedule</span>
						<span class="weekend-count">
							{(schedulesByDay[0]?.length || 0) + (schedulesByDay[6]?.length || 0)} events
						</span>
					</summary>
					<div class="weekend-grid">
						{#each [6, 0] as dayIndex}
							{@const daySchedules = schedulesByDay[dayIndex] || []}
							<div class="day-column">
								<div class="day-header">
									<span class="day-name">{DAYS[dayIndex]}</span>
								</div>
								<div class="day-events">
									{#each daySchedules as event}
										<div class="event-card" class:inactive={!event.is_active}>
											<div class="event-content">
												<div class="event-time">
													<IconClock size={14} />
													{formatTime(event.start_time)} - {formatTime(event.end_time)}
												</div>
												<div class="event-title">{event.title}</div>
												{#if event.trader_name}
													<div class="event-trader">
														<IconUser size={12} />
														{event.trader_name}
													</div>
												{/if}
											</div>
											<div class="event-actions">
												<button class="btn-icon" onclick={() => openEditModal(event)}>
													<IconEdit size={16} />
												</button>
												<button
													class="btn-icon btn-danger"
													onclick={() => deleteSchedule(event.id)}
												>
													<IconTrash size={16} />
												</button>
											</div>
										</div>
									{/each}
									{#if daySchedules.length === 0}
										<div class="no-events">No weekend events</div>
									{/if}
								</div>
							</div>
						{/each}
					</div>
				</details>
			{/if}
		</div>
	</div>
	<!-- End admin-page-container -->
</div>

<!-- Create/Edit Modal -->
{#if showModal}
	<div
		class="modal-overlay"
		role="dialog"
		aria-modal="true"
		aria-labelledby="modal-title"
		tabindex="-1"
		onclick={handleOverlayClick}
		onkeydown={handleOverlayKeydown}
	>
		<div class="modal" role="document">
			<div class="modal-header">
				<h3 id="modal-title">
					{editingSchedule ? 'Edit Schedule' : 'Create Schedule'}
				</h3>
				<button class="modal-close" onclick={closeModal} aria-label="Close">
					<IconX size={24} />
				</button>
			</div>

			<form
				class="modal-form"
				onsubmit={(e) => {
					e.preventDefault();
					handleSubmit();
				}}
			>
				<div class="form-group">
					<label for="title">Event Title <span class="required">*</span></label>
					<input
						type="text"
						id="title"
						name="title"
						bind:value={formData.title}
						placeholder="e.g., Morning Market Analysis"
						required
					/>
				</div>

				<div class="form-row">
					<div class="form-group">
						<label for="trader_name">Trader/Host</label>
						<input
							type="text"
							id="trader_name"
							name="trader_name"
							bind:value={formData.trader_name}
							placeholder="e.g., Taylor Horton"
						/>
					</div>

					<div class="form-group">
						<label for="room_type">Session Type</label>
						<select id="room_type" bind:value={formData.room_type}>
							{#each ROOM_TYPES as type}
								<option value={type.value}>{type.label}</option>
							{/each}
						</select>
					</div>
				</div>

				<div class="form-row">
					<div class="form-group">
						<label for="day_of_week">Day of Week <span class="required">*</span></label>
						<select id="day_of_week" bind:value={formData.day_of_week} required>
							{#each DAYS as day, index}
								<option value={index}>{day}</option>
							{/each}
						</select>
					</div>

					<div class="form-group">
						<label for="recurrence">Recurrence</label>
						<select id="recurrence" bind:value={formData.recurrence}>
							<option value="weekly">Weekly</option>
							<option value="biweekly">Bi-weekly</option>
							<option value="monthly">Monthly</option>
							<option value={null}>One-time</option>
						</select>
					</div>
				</div>

				<div class="form-row">
					<div class="form-group">
						<label for="start_time">Start Time <span class="required">*</span></label>
						<input
							type="time"
							id="start_time"
							name="start_time"
							bind:value={formData.start_time}
							required
						/>
					</div>

					<div class="form-group">
						<label for="end_time">End Time <span class="required">*</span></label>
						<input
							type="time"
							id="end_time"
							name="end_time"
							bind:value={formData.end_time}
							required
						/>
					</div>
				</div>

				<div class="form-group">
					<label for="timezone">Timezone</label>
					<select id="timezone" bind:value={formData.timezone}>
						{#each TIMEZONES as tz}
							<option value={tz.value}>{tz.label}</option>
						{/each}
					</select>
				</div>

				<div class="form-group">
					<label for="description">Description</label>
					<textarea
						id="description"
						bind:value={formData.description}
						placeholder="Optional description of the session..."
						rows="3"
					></textarea>
				</div>

				<div class="form-group form-checkbox">
					<label>
						<input
							id="page-formdata-is-active"
							name="page-formdata-is-active"
							type="checkbox"
							bind:checked={formData.is_active}
						/>
						<span>Active (visible to members)</span>
					</label>
				</div>

				<div class="modal-actions">
					<button type="button" class="btn btn-secondary" onclick={closeModal}> Cancel </button>
					<button type="submit" class="btn btn-primary" disabled={saving}>
						{#if saving}
							<span class="spinner-sm"></span>
							Saving...
						{:else}
							{editingSchedule ? 'Update Schedule' : 'Create Schedule'}
						{/if}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}

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
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.admin-schedules {
		padding: 24px;
		max-width: 1600px;
		margin: 0 auto;
		font-family: 'Montserrat', var(--font-body), sans-serif;
	}

	/* Header */
	.page-header {
		margin-bottom: 24px;
		padding-bottom: 24px;
		border-bottom: 1px solid #e2e8f0;
	}

	.header-content {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		gap: 24px;
	}

	.header-title {
		display: flex;
		align-items: flex-start;
		gap: 16px;
	}

	.header-title :global(svg) {
		color: #143e59;
		flex-shrink: 0;
		margin-top: 4px;
	}

	.header-title h1 {
		font-size: 28px;
		font-weight: 700;
		color: #1e293b;
		margin: 0 0 4px 0;
	}

	.subtitle {
		color: #64748b;
		font-size: 14px;
		margin: 0;
	}

	.header-actions {
		display: flex;
		align-items: center;
		gap: 16px;
	}

	.conflict-warning {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 8px 12px;
		background: #fef3c7;
		color: #b45309;
		border-radius: 6px;
		font-size: 13px;
		font-weight: 500;
	}

	/* Alerts */
	.alert {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 12px 16px;
		border-radius: 8px;
		margin-bottom: 16px;
		font-size: 14px;
	}

	.alert-error {
		background: #fef2f2;
		color: #dc2626;
		border: 1px solid #fecaca;
	}

	.alert-success {
		background: #f0fdf4;
		color: #16a34a;
		border: 1px solid #bbf7d0;
	}

	.alert button {
		margin-left: auto;
		background: none;
		border: none;
		cursor: pointer;
		opacity: 0.7;
		padding: 4px;
		display: flex;
	}

	.alert button:hover {
		opacity: 1;
	}

	/* Room Selector */
	.room-selector {
		margin-bottom: 20px;
	}

	.room-tabs {
		display: flex;
		gap: 8px;
		flex-wrap: wrap;
		margin-bottom: 12px;
	}

	.room-tab {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 16px;
		background: #fff;
		border: 2px solid #e2e8f0;
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.2s ease;
		font-size: 14px;
		font-weight: 600;
	}

	.room-tab:hover {
		border-color: var(--room-color, #143e59);
		background: #f8fafc;
	}

	.room-tab.active {
		background: var(--room-color, #143e59);
		border-color: var(--room-color, #143e59);
		color: #fff;
	}

	.room-icon {
		font-size: 18px;
	}

	.room-info {
		display: flex;
		align-items: center;
		gap: 16px;
	}

	.room-full-name {
		font-size: 16px;
		font-weight: 600;
	}

	.room-stats {
		font-size: 13px;
		color: #64748b;
	}

	/* Toolbar */
	.toolbar {
		display: flex;
		justify-content: space-between;
		align-items: center;
		gap: 16px;
		margin-bottom: 20px;
		padding: 12px 16px;
		background: #f8fafc;
		border-radius: 8px;
		flex-wrap: wrap;
	}

	.week-nav {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.week-label {
		font-size: 15px;
		font-weight: 600;
		color: #1e293b;
		min-width: 180px;
		text-align: center;
	}

	.filters {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.filter-select {
		padding: 8px 12px;
		border: 1px solid #e2e8f0;
		border-radius: 6px;
		font-size: 13px;
		background: #fff;
		cursor: pointer;
	}

	.filter-select:focus {
		outline: none;
		border-color: #143e59;
	}

	/* Bulk Actions */
	.bulk-actions {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 12px 16px;
		background: #143e59;
		border-radius: 8px;
		margin-bottom: 16px;
	}

	.bulk-count {
		color: #fff;
		font-size: 14px;
		font-weight: 500;
		margin-right: auto;
	}

	.bulk-actions .btn-sm {
		padding: 6px 12px;
		font-size: 13px;
	}

	/* Schedule Container */
	.schedule-container {
		background: #fff;
		border-radius: 12px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
		overflow: hidden;
	}

	/* Weekly Grid */
	.weekly-grid {
		display: grid;
		grid-template-columns: repeat(5, 1fr);
		gap: 1px;
		background: #e2e8f0;
	}

	.day-column {
		background: #fff;
		min-height: 300px;
	}

	.day-header {
		padding: 12px 16px;
		background: #f8fafc;
		border-bottom: 1px solid #e2e8f0;
		text-align: center;
	}

	.day-name {
		display: block;
		font-size: 14px;
		font-weight: 700;
		color: #143e59;
	}

	.day-date {
		display: block;
		font-size: 12px;
		color: #64748b;
		margin-top: 2px;
	}

	.day-events {
		padding: 12px;
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	/* Event Card */
	.event-card {
		background: #fff;
		border: 1px solid #e2e8f0;
		border-radius: 8px;
		padding: 12px;
		position: relative;
		transition: all 0.2s ease;
	}

	.event-card:hover {
		border-color: #94a3b8;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
	}

	.event-card.inactive {
		opacity: 0.6;
		background: #f8fafc;
	}

	.event-card.selected {
		border-color: #143e59;
		background: #f0f9ff;
	}

	.event-card.conflict {
		border-color: #f59e0b;
		background: #fffbeb;
	}

	.event-checkbox {
		position: absolute;
		top: 8px;
		left: 8px;
	}

	.event-checkbox input {
		width: 16px;
		height: 16px;
		cursor: pointer;
	}

	.event-content {
		padding-left: 28px;
	}

	.event-time {
		display: flex;
		align-items: center;
		gap: 4px;
		font-size: 12px;
		font-weight: 600;
		color: #143e59;
		margin-bottom: 4px;
	}

	.event-title {
		font-size: 13px;
		font-weight: 600;
		color: #1e293b;
		margin-bottom: 4px;
	}

	.event-trader {
		display: flex;
		align-items: center;
		gap: 4px;
		font-size: 11px;
		color: #64748b;
		margin-bottom: 8px;
	}

	.event-badges {
		display: flex;
		gap: 4px;
		flex-wrap: wrap;
	}

	.badge {
		display: inline-block;
		padding: 2px 6px;
		font-size: 10px;
		font-weight: 600;
		border-radius: 4px;
		text-transform: uppercase;
	}

	.badge-live {
		background: #dcfce7;
		color: #16a34a;
	}

	.badge-recorded {
		background: #e0e7ff;
		color: #4f46e5;
	}

	.badge-hybrid {
		background: #fef3c7;
		color: #b45309;
	}

	.badge-inactive {
		background: #f1f5f9;
		color: #64748b;
	}

	.event-actions {
		display: flex;
		gap: 4px;
		margin-top: 8px;
		padding-top: 8px;
		border-top: 1px solid #f1f5f9;
	}

	.no-events {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 8px;
		padding: 32px 16px;
		color: #94a3b8;
		font-size: 13px;
	}

	.btn-link {
		background: none;
		border: none;
		color: #143e59;
		font-size: 13px;
		font-weight: 500;
		cursor: pointer;
		text-decoration: underline;
	}

	.btn-link:hover {
		color: #0d2a3d;
	}

	/* Weekend Section */
	.weekend-section {
		margin-top: 1px;
		border-top: 1px solid #e2e8f0;
	}

	.weekend-section summary {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 16px 20px;
		background: #f8fafc;
		cursor: pointer;
		font-size: 14px;
		font-weight: 600;
		color: #475569;
	}

	.weekend-section summary:hover {
		background: #f1f5f9;
	}

	.weekend-count {
		font-size: 12px;
		color: #94a3b8;
		font-weight: 400;
	}

	.weekend-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1px;
		background: #e2e8f0;
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

	.spinner-sm {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: #fff;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
		display: inline-block;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Buttons */
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

	.btn-primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-secondary {
		background: #f1f5f9;
		color: #475569;
	}

	.btn-secondary:hover {
		background: #e2e8f0;
	}

	.btn-danger {
		background: #fef2f2;
		color: #dc2626;
	}

	.btn-danger:hover {
		background: #fee2e2;
	}

	.btn-icon {
		padding: 8px;
		background: #f8fafc;
		border: 1px solid #e2e8f0;
		border-radius: 6px;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		transition: all 0.2s;
	}

	.btn-icon:hover {
		background: #f1f5f9;
		border-color: #cbd5e1;
	}

	.btn-icon.btn-danger {
		background: #fff;
		border-color: #fecaca;
	}

	.btn-icon.btn-danger:hover {
		background: #fef2f2;
	}

	.btn-text {
		background: none;
		border: none;
		color: #143e59;
		font-weight: 600;
		cursor: pointer;
		padding: 8px 12px;
	}

	.btn-text:hover {
		text-decoration: underline;
	}

	/* Modal */
	.modal-overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 20px;
	}

	.modal {
		background: #fff;
		border-radius: 16px;
		width: 100%;
		max-width: 560px;
		max-height: 90vh;
		overflow-y: auto;
		box-shadow: 0 24px 48px rgba(0, 0, 0, 0.2);
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 20px 24px;
		border-bottom: 1px solid #e2e8f0;
	}

	.modal-header h3 {
		font-size: 20px;
		font-weight: 700;
		color: #1e293b;
		margin: 0;
	}

	.modal-close {
		background: none;
		border: none;
		cursor: pointer;
		color: #64748b;
		padding: 4px;
		display: flex;
		border-radius: 6px;
	}

	.modal-close:hover {
		background: #f1f5f9;
		color: #1e293b;
	}

	.modal-form {
		padding: 24px;
	}

	.form-group {
		margin-bottom: 20px;
	}

	.form-group label {
		display: block;
		font-size: 13px;
		font-weight: 600;
		color: #475569;
		margin-bottom: 6px;
	}

	.required {
		color: #dc2626;
	}

	.form-group input,
	.form-group select,
	.form-group textarea {
		width: 100%;
		padding: 10px 14px;
		font-size: 14px;
		border: 1px solid #e2e8f0;
		border-radius: 8px;
		transition:
			border-color 0.2s,
			box-shadow 0.2s;
		background: #fff;
	}

	.form-group input:focus,
	.form-group select:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: #143e59;
		box-shadow: 0 0 0 3px rgba(20, 62, 89, 0.1);
	}

	.form-group textarea {
		resize: vertical;
		min-height: 80px;
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 16px;
	}

	.form-checkbox {
		display: flex;
		align-items: center;
	}

	.form-checkbox label {
		display: flex;
		align-items: center;
		gap: 10px;
		cursor: pointer;
		margin: 0;
	}

	.form-checkbox input[type='checkbox'] {
		width: 18px;
		height: 18px;
		cursor: pointer;
	}

	.form-checkbox span {
		font-weight: 400;
		color: #1e293b;
	}

	.modal-actions {
		display: flex;
		justify-content: flex-end;
		gap: 12px;
		margin-top: 24px;
		padding-top: 20px;
		border-top: 1px solid #e2e8f0;
	}

	/* Responsive */
	@media (max-width: 1200px) {
		.weekly-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	@media (max-width: 900px) {
		.weekly-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 768px) {
		.admin-schedules {
			padding: 16px;
		}

		.header-content {
			flex-direction: column;
		}

		.weekly-grid {
			grid-template-columns: 1fr;
		}

		.weekend-grid {
			grid-template-columns: 1fr;
		}

		.toolbar {
			flex-direction: column;
			align-items: stretch;
		}

		.week-nav {
			justify-content: center;
		}

		.filters {
			justify-content: center;
		}

		.form-row {
			grid-template-columns: 1fr;
		}

		.room-tabs {
			overflow-x: auto;
			flex-wrap: nowrap;
			padding-bottom: 8px;
		}
	}
</style>

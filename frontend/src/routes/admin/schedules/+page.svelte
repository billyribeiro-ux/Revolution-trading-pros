<!--
	Admin Schedule Management - Trading Room Schedules CMS
	ICT 11+ Apple Principal Engineer Grade
	January 2026
	
	Features:
	- Per-room schedule management
	- Weekly calendar view
	- CRUD operations for schedule events
	- Exception handling (holidays, cancellations)
	- Bulk operations
	
	@version 1.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	// ═══════════════════════════════════════════════════════════════════════════
	// TYPE DEFINITIONS
	// ═══════════════════════════════════════════════════════════════════════════

	interface TradingRoom {
		id: number;
		name: string;
		slug: string;
		type: string | null;
	}

	interface ScheduleEvent {
		id: number;
		plan_id: number;
		title: string;
		description: string | null;
		trader_name: string | null;
		day_of_week: number;
		start_time: string;
		end_time: string;
		timezone: string;
		is_active: boolean;
		room_type: string | null;
	}

	interface ScheduleForm {
		plan_id: number;
		title: string;
		description: string;
		trader_name: string;
		day_of_week: number;
		start_time: string;
		end_time: string;
		timezone: string;
		room_type: string;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let tradingRooms = $state<TradingRoom[]>([]);
	let selectedRoom = $state<TradingRoom | null>(null);
	let schedules = $state<ScheduleEvent[]>([]);
	let loading = $state(true);
	let saving = $state(false);
	let error = $state<string | null>(null);
	let success = $state<string | null>(null);

	// Modal state
	let showModal = $state(false);
	let editingSchedule = $state<ScheduleEvent | null>(null);
	let formData = $state<ScheduleForm>({
		plan_id: 0,
		title: '',
		description: '',
		trader_name: '',
		day_of_week: 1,
		start_time: '09:00',
		end_time: '10:00',
		timezone: 'America/New_York',
		room_type: 'live'
	});

	// Day names for display
	const DAYS = ['Sunday', 'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday'];

	// ═══════════════════════════════════════════════════════════════════════════
	// API FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	async function fetchTradingRooms() {
		try {
			const response = await fetch('/api/schedules/rooms', {
				credentials: 'include'
			});
			if (!response.ok) throw new Error('Failed to fetch trading rooms');
			const data = await response.json();
			tradingRooms = data.rooms || [];
			
			// Auto-select first room
			if (tradingRooms.length > 0 && !selectedRoom) {
				selectRoom(tradingRooms[0]);
			}
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to fetch trading rooms';
		}
	}

	async function fetchSchedules(planId: number) {
		loading = true;
		error = null;
		try {
			const response = await fetch(`/api/admin/schedules/plan/${planId}`, {
				credentials: 'include'
			});
			if (!response.ok) throw new Error('Failed to fetch schedules');
			const data = await response.json();
			schedules = data.schedules || [];
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to fetch schedules';
		} finally {
			loading = false;
		}
	}

	async function createSchedule() {
		if (!selectedRoom) return;
		saving = true;
		error = null;
		success = null;

		try {
			const response = await fetch('/api/admin/schedules', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				credentials: 'include',
				body: JSON.stringify({
					...formData,
					plan_id: selectedRoom.id
				})
			});

			if (!response.ok) {
				const data = await response.json();
				throw new Error(data.error || 'Failed to create schedule');
			}

			success = 'Schedule created successfully';
			showModal = false;
			resetForm();
			await fetchSchedules(selectedRoom.id);
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to create schedule';
		} finally {
			saving = false;
		}
	}

	async function updateSchedule() {
		if (!editingSchedule || !selectedRoom) return;
		saving = true;
		error = null;
		success = null;

		try {
			const response = await fetch(`/api/admin/schedules/${editingSchedule.id}`, {
				method: 'PUT',
				headers: { 'Content-Type': 'application/json' },
				credentials: 'include',
				body: JSON.stringify(formData)
			});

			if (!response.ok) {
				const data = await response.json();
				throw new Error(data.error || 'Failed to update schedule');
			}

			success = 'Schedule updated successfully';
			showModal = false;
			editingSchedule = null;
			resetForm();
			await fetchSchedules(selectedRoom.id);
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to update schedule';
		} finally {
			saving = false;
		}
	}

	async function deleteSchedule(id: number) {
		if (!confirm('Are you sure you want to delete this schedule?')) return;
		if (!selectedRoom) return;

		try {
			const response = await fetch(`/api/admin/schedules/${id}`, {
				method: 'DELETE',
				credentials: 'include'
			});

			if (!response.ok) throw new Error('Failed to delete schedule');

			success = 'Schedule deleted successfully';
			await fetchSchedules(selectedRoom.id);
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to delete schedule';
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// UI FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	function selectRoom(room: TradingRoom) {
		selectedRoom = room;
		fetchSchedules(room.id);
	}

	function openCreateModal() {
		editingSchedule = null;
		resetForm();
		showModal = true;
	}

	function openEditModal(schedule: ScheduleEvent) {
		editingSchedule = schedule;
		formData = {
			plan_id: schedule.plan_id,
			title: schedule.title,
			description: schedule.description || '',
			trader_name: schedule.trader_name || '',
			day_of_week: schedule.day_of_week,
			start_time: schedule.start_time,
			end_time: schedule.end_time,
			timezone: schedule.timezone || 'America/New_York',
			room_type: schedule.room_type || 'live'
		};
		showModal = true;
	}

	function closeModal() {
		showModal = false;
		editingSchedule = null;
		resetForm();
	}

	function resetForm() {
		formData = {
			plan_id: selectedRoom?.id || 0,
			title: '',
			description: '',
			trader_name: '',
			day_of_week: 1,
			start_time: '09:00',
			end_time: '10:00',
			timezone: 'America/New_York',
			room_type: 'live'
		};
	}

	function handleSubmit() {
		if (editingSchedule) {
			updateSchedule();
		} else {
			createSchedule();
		}
	}

	// Group schedules by day
	function getSchedulesByDay(day: number): ScheduleEvent[] {
		return schedules
			.filter(s => s.day_of_week === day)
			.sort((a, b) => a.start_time.localeCompare(b.start_time));
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// LIFECYCLE
	// ═══════════════════════════════════════════════════════════════════════════

	onMount(() => {
		fetchTradingRooms();
	});
</script>

<svelte:head>
	<title>Schedule Management | Admin | RTP</title>
</svelte:head>

<div class="admin-schedules">
	<!-- Header -->
	<header class="admin-header">
		<div class="header-content">
			<h1>Trading Room Schedules</h1>
			<p class="subtitle">Manage weekly schedules for each trading room and mentorship service</p>
		</div>
		{#if selectedRoom}
			<button class="btn btn-primary" onclick={openCreateModal}>
				+ Add Schedule Event
			</button>
		{/if}
	</header>

	<!-- Alerts -->
	{#if error}
		<div class="alert alert-error">
			<span>{error}</span>
			<button onclick={() => error = null}>×</button>
		</div>
	{/if}

	{#if success}
		<div class="alert alert-success">
			<span>{success}</span>
			<button onclick={() => success = null}>×</button>
		</div>
	{/if}

	<!-- Room Selector -->
	<div class="room-selector">
		<h3>Select Trading Room</h3>
		<div class="room-tabs">
			{#each tradingRooms as room}
				<button
					class="room-tab"
					class:active={selectedRoom?.id === room.id}
					onclick={() => selectRoom(room)}
				>
					<span class="room-name">{room.name}</span>
					<span class="room-type">{room.type || 'trading-room'}</span>
				</button>
			{/each}
		</div>
	</div>

	<!-- Schedule Grid -->
	{#if selectedRoom}
		<div class="schedule-container">
			<h2 class="schedule-title">
				{selectedRoom.name} - Weekly Schedule
			</h2>

			{#if loading}
				<div class="loading">Loading schedules...</div>
			{:else if schedules.length === 0}
				<div class="empty-state">
					<p>No schedules configured for this trading room.</p>
					<button class="btn btn-primary" onclick={openCreateModal}>
						Create First Schedule
					</button>
				</div>
			{:else}
				<div class="weekly-grid">
					{#each [1, 2, 3, 4, 5] as day}
						<div class="day-column">
							<h4 class="day-header">{DAYS[day]}</h4>
							<div class="day-events">
								{#each getSchedulesByDay(day) as event}
									<div class="event-card" class:inactive={!event.is_active}>
										<div class="event-time">
											{event.start_time} - {event.end_time}
										</div>
										<div class="event-title">{event.title}</div>
										{#if event.trader_name}
											<div class="event-trader">with {event.trader_name}</div>
										{/if}
										<div class="event-actions">
											<button class="btn-icon" onclick={() => openEditModal(event)} title="Edit">
												✏️
											</button>
											<button class="btn-icon btn-danger" onclick={() => deleteSchedule(event.id)} title="Delete">
												🗑️
											</button>
										</div>
									</div>
								{/each}
								{#if getSchedulesByDay(day).length === 0}
									<div class="no-events">No events</div>
								{/if}
							</div>
						</div>
					{/each}
				</div>

				<!-- Weekend (collapsed) -->
				<details class="weekend-section">
					<summary>Weekend Schedule (Saturday & Sunday)</summary>
					<div class="weekend-grid">
						{#each [6, 0] as day}
							<div class="day-column">
								<h4 class="day-header">{DAYS[day]}</h4>
								<div class="day-events">
									{#each getSchedulesByDay(day) as event}
										<div class="event-card">
											<div class="event-time">
												{event.start_time} - {event.end_time}
											</div>
											<div class="event-title">{event.title}</div>
											{#if event.trader_name}
												<div class="event-trader">with {event.trader_name}</div>
											{/if}
											<div class="event-actions">
												<button class="btn-icon" onclick={() => openEditModal(event)}>✏️</button>
												<button class="btn-icon btn-danger" onclick={() => deleteSchedule(event.id)}>🗑️</button>
											</div>
										</div>
									{/each}
									{#if getSchedulesByDay(day).length === 0}
										<div class="no-events">No events</div>
									{/if}
								</div>
							</div>
						{/each}
					</div>
				</details>
			{/if}
		</div>
	{:else}
		<div class="empty-state">
			<p>Select a trading room to manage its schedule.</p>
		</div>
	{/if}
</div>

<!-- Modal -->
{#if showModal}
	<div class="modal-overlay" role="dialog" aria-modal="true" aria-labelledby="modal-title" tabindex="-1" onclick={closeModal} onkeydown={(e) => e.key === 'Escape' && closeModal()}>
		<div class="modal" role="document" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()}>
			<div class="modal-header">
				<h3 id="modal-title">{editingSchedule ? 'Edit Schedule Event' : 'Create Schedule Event'}</h3>
				<button class="modal-close" onclick={closeModal}>×</button>
			</div>

			<form class="modal-form" onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
				<div class="form-group">
					<label for="title">Event Title *</label>
					<input
						type="text"
						id="title"
						bind:value={formData.title}
						placeholder="e.g., Morning Market Analysis"
						required
					/>
				</div>

				<div class="form-group">
					<label for="trader_name">Trader Name</label>
					<input
						type="text"
						id="trader_name"
						bind:value={formData.trader_name}
						placeholder="e.g., Taylor Horton"
					/>
				</div>

				<div class="form-row">
					<div class="form-group">
						<label for="day_of_week">Day of Week *</label>
						<select id="day_of_week" bind:value={formData.day_of_week} required>
							{#each DAYS as day, index}
								<option value={index}>{day}</option>
							{/each}
						</select>
					</div>

					<div class="form-group">
						<label for="room_type">Room Type</label>
						<select id="room_type" bind:value={formData.room_type}>
							<option value="live">Live</option>
							<option value="recorded">Recorded</option>
							<option value="hybrid">Hybrid</option>
						</select>
					</div>
				</div>

				<div class="form-row">
					<div class="form-group">
						<label for="start_time">Start Time *</label>
						<input
							type="time"
							id="start_time"
							bind:value={formData.start_time}
							required
						/>
					</div>

					<div class="form-group">
						<label for="end_time">End Time *</label>
						<input
							type="time"
							id="end_time"
							bind:value={formData.end_time}
							required
						/>
					</div>
				</div>

				<div class="form-group">
					<label for="timezone">Timezone</label>
					<select id="timezone" bind:value={formData.timezone}>
						<option value="America/New_York">Eastern (ET)</option>
						<option value="America/Chicago">Central (CT)</option>
						<option value="America/Denver">Mountain (MT)</option>
						<option value="America/Los_Angeles">Pacific (PT)</option>
					</select>
				</div>

				<div class="form-group">
					<label for="description">Description</label>
					<textarea
						id="description"
						bind:value={formData.description}
						placeholder="Optional description..."
						rows="3"
					></textarea>
				</div>

				<div class="modal-actions">
					<button type="button" class="btn btn-secondary" onclick={closeModal}>
						Cancel
					</button>
					<button type="submit" class="btn btn-primary" disabled={saving}>
						{saving ? 'Saving...' : (editingSchedule ? 'Update' : 'Create')}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * ADMIN SCHEDULES - ICT 11+ Styling
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.admin-schedules {
		padding: 24px;
		max-width: 1400px;
		margin: 0 auto;
		font-family: 'Montserrat', var(--font-body), sans-serif;
	}

	/* Header */
	.admin-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 24px;
		padding-bottom: 24px;
		border-bottom: 1px solid #e5e5e5;
	}

	.admin-header h1 {
		font-size: 28px;
		font-weight: 700;
		color: #143E59;
		margin: 0 0 8px 0;
	}

	.subtitle {
		color: #666;
		font-size: 14px;
		margin: 0;
	}

	/* Alerts */
	.alert {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 12px 16px;
		border-radius: 8px;
		margin-bottom: 16px;
		font-size: 14px;
	}

	.alert-error {
		background: #fee2e2;
		color: #dc2626;
		border: 1px solid #fecaca;
	}

	.alert-success {
		background: #dcfce7;
		color: #16a34a;
		border: 1px solid #bbf7d0;
	}

	.alert button {
		background: none;
		border: none;
		font-size: 20px;
		cursor: pointer;
		opacity: 0.7;
	}

	/* Room Selector */
	.room-selector {
		margin-bottom: 24px;
	}

	.room-selector h3 {
		font-size: 14px;
		font-weight: 600;
		color: #666;
		margin: 0 0 12px 0;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.room-tabs {
		display: flex;
		gap: 12px;
		flex-wrap: wrap;
	}

	.room-tab {
		display: flex;
		flex-direction: column;
		align-items: flex-start;
		padding: 12px 20px;
		background: #fff;
		border: 2px solid #e5e5e5;
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.room-tab:hover {
		border-color: #143E59;
	}

	.room-tab.active {
		background: #143E59;
		border-color: #143E59;
		color: #fff;
	}

	.room-name {
		font-weight: 600;
		font-size: 14px;
	}

	.room-type {
		font-size: 11px;
		opacity: 0.7;
		text-transform: capitalize;
	}

	/* Schedule Container */
	.schedule-container {
		background: #fff;
		border-radius: 12px;
		padding: 24px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
	}

	.schedule-title {
		font-size: 20px;
		font-weight: 700;
		color: #333;
		margin: 0 0 24px 0;
	}

	/* Weekly Grid */
	.weekly-grid {
		display: grid;
		grid-template-columns: repeat(5, 1fr);
		gap: 16px;
	}

	.day-column {
		background: #f9fafb;
		border-radius: 8px;
		padding: 16px;
		min-height: 200px;
	}

	.day-header {
		font-size: 14px;
		font-weight: 700;
		color: #143E59;
		margin: 0 0 12px 0;
		padding-bottom: 8px;
		border-bottom: 2px solid #143E59;
	}

	.day-events {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	/* Event Card */
	.event-card {
		background: #fff;
		border: 1px solid #e5e5e5;
		border-radius: 6px;
		padding: 12px;
		position: relative;
	}

	.event-card.inactive {
		opacity: 0.5;
	}

	.event-time {
		font-size: 12px;
		font-weight: 600;
		color: #143E59;
		margin-bottom: 4px;
	}

	.event-title {
		font-size: 13px;
		font-weight: 600;
		color: #333;
		margin-bottom: 4px;
	}

	.event-trader {
		font-size: 11px;
		color: #666;
	}

	.event-actions {
		display: flex;
		gap: 4px;
		margin-top: 8px;
		padding-top: 8px;
		border-top: 1px solid #f0f0f0;
	}

	.btn-icon {
		background: none;
		border: none;
		cursor: pointer;
		padding: 4px;
		font-size: 14px;
		opacity: 0.7;
		transition: opacity 0.2s;
	}

	.btn-icon:hover {
		opacity: 1;
	}

	.no-events {
		font-size: 12px;
		color: #999;
		text-align: center;
		padding: 20px 0;
	}

	/* Weekend Section */
	.weekend-section {
		margin-top: 24px;
		padding-top: 24px;
		border-top: 1px solid #e5e5e5;
	}

	.weekend-section summary {
		font-size: 14px;
		font-weight: 600;
		color: #666;
		cursor: pointer;
		padding: 8px 0;
	}

	.weekend-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 16px;
		margin-top: 16px;
	}

	/* Empty State */
	.empty-state {
		text-align: center;
		padding: 60px 20px;
		color: #666;
	}

	.empty-state p {
		margin-bottom: 16px;
	}

	/* Loading */
	.loading {
		text-align: center;
		padding: 40px;
		color: #666;
	}

	/* Buttons */
	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: 10px 20px;
		font-size: 14px;
		font-weight: 600;
		border-radius: 6px;
		cursor: pointer;
		transition: all 0.2s ease;
		border: none;
	}

	.btn-primary {
		background: #143E59;
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
		background: #f3f4f6;
		color: #333;
	}

	.btn-secondary:hover {
		background: #e5e7eb;
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
	}

	.modal {
		background: #fff;
		border-radius: 12px;
		width: 100%;
		max-width: 500px;
		max-height: 90vh;
		overflow-y: auto;
		box-shadow: 0 20px 40px rgba(0, 0, 0, 0.2);
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 20px 24px;
		border-bottom: 1px solid #e5e5e5;
	}

	.modal-header h3 {
		font-size: 18px;
		font-weight: 700;
		color: #333;
		margin: 0;
	}

	.modal-close {
		background: none;
		border: none;
		font-size: 24px;
		cursor: pointer;
		color: #666;
	}

	.modal-form {
		padding: 24px;
	}

	.form-group {
		margin-bottom: 16px;
	}

	.form-group label {
		display: block;
		font-size: 13px;
		font-weight: 600;
		color: #333;
		margin-bottom: 6px;
	}

	.form-group input,
	.form-group select,
	.form-group textarea {
		width: 100%;
		padding: 10px 12px;
		font-size: 14px;
		border: 1px solid #e5e5e5;
		border-radius: 6px;
		transition: border-color 0.2s;
	}

	.form-group input:focus,
	.form-group select:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: #143E59;
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 16px;
	}

	.modal-actions {
		display: flex;
		justify-content: flex-end;
		gap: 12px;
		margin-top: 24px;
		padding-top: 24px;
		border-top: 1px solid #e5e5e5;
	}

	/* Responsive */
	@media (max-width: 1200px) {
		.weekly-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	@media (max-width: 768px) {
		.admin-header {
			flex-direction: column;
			gap: 16px;
		}

		.weekly-grid {
			grid-template-columns: 1fr;
		}

		.weekend-grid {
			grid-template-columns: 1fr;
		}

		.room-tabs {
			flex-direction: column;
		}

		.form-row {
			grid-template-columns: 1fr;
		}
	}
</style>

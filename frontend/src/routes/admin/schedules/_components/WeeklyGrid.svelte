<!--
	WeeklyGrid — Mon–Fri columns with day header + events list + empty-state "+ Add".
	Extracted from /admin/schedules in R21-C.
-->
<script lang="ts">
	import ScheduleEventCard from './ScheduleEventCard.svelte';
	import type { ScheduleEvent } from './types';

	interface Props {
		days: readonly string[];
		schedulesByDay: Record<number, ScheduleEvent[]>;
		weekDates: Date[];
		selectedIds: Set<number>;
		isConflicting: (id: number) => boolean;
		formatDate: (d: Date) => string;
		formatTime: (t: string) => string;
		onToggleSelect: (id: number) => void;
		onToggleActive: (event: ScheduleEvent) => void;
		onDuplicate: (event: ScheduleEvent) => void;
		onEdit: (event: ScheduleEvent) => void;
		onDelete: (id: number) => void;
		onAddOnDay: (dayIndex: number) => void;
	}

	let {
		days,
		schedulesByDay,
		weekDates,
		selectedIds,
		isConflicting,
		formatDate,
		formatTime,
		onToggleSelect,
		onToggleActive,
		onDuplicate,
		onEdit,
		onDelete,
		onAddOnDay
	}: Props = $props();
</script>

<div class="weekly-grid">
	{#each [1, 2, 3, 4, 5] as dayIndex (dayIndex)}
		{@const daySchedules = schedulesByDay[dayIndex] || []}
		{@const dayDate = weekDates[dayIndex]}
		<div class="day-column">
			<div class="day-header">
				<span class="day-name">{days[dayIndex]}</span>
				<span class="day-date">{formatDate(dayDate)}</span>
			</div>
			<div class="day-events">
				{#each daySchedules as event (event.id)}
					<ScheduleEventCard
						variant="weekly"
						{event}
						selected={selectedIds.has(event.id)}
						conflict={isConflicting(event.id)}
						{formatTime}
						{onToggleSelect}
						{onToggleActive}
						{onDuplicate}
						{onEdit}
						{onDelete}
					/>
				{/each}
				{#if daySchedules.length === 0}
					<div class="no-events">
						<span>No events</span>
						<button class="btn-link" onclick={() => onAddOnDay(dayIndex)}>+ Add</button>
					</div>
				{/if}
			</div>
		</div>
	{/each}
</div>

<style>
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

	@media (max-width: 1199.98px) {
		.weekly-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	@media (max-width: 900px) {
		.weekly-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 767.98px) {
		.weekly-grid {
			grid-template-columns: 1fr;
		}
	}
</style>

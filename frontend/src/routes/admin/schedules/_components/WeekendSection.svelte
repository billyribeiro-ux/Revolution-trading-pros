<!--
	WeekendSection — collapsible <details> with Sat / Sun compact event cards.
	Extracted from /admin/schedules in R21-C.
-->
<script lang="ts">
	import ScheduleEventCard from './ScheduleEventCard.svelte';
	import type { ScheduleEvent } from './types';

	interface Props {
		days: readonly string[];
		schedulesByDay: Record<number, ScheduleEvent[]>;
		formatTime: (t: string) => string;
		onEdit: (event: ScheduleEvent) => void;
		onDelete: (id: number) => void;
	}

	let { days, schedulesByDay, formatTime, onEdit, onDelete }: Props = $props();

	let weekendCount = $derived((schedulesByDay[0]?.length || 0) + (schedulesByDay[6]?.length || 0));
</script>

<details class="weekend-section">
	<summary>
		<span>Weekend Schedule</span>
		<span class="weekend-count">
			{weekendCount} events
		</span>
	</summary>
	<div class="weekend-grid">
		{#each [6, 0] as dayIndex (dayIndex)}
			{@const daySchedules = schedulesByDay[dayIndex] || []}
			<div class="day-column">
				<div class="day-header">
					<span class="day-name">{days[dayIndex]}</span>
				</div>
				<div class="day-events">
					{#each daySchedules as event (event.id)}
						<ScheduleEventCard variant="weekend" {event} {formatTime} {onEdit} {onDelete} />
					{/each}
					{#if daySchedules.length === 0}
						<div class="no-events">No weekend events</div>
					{/if}
				</div>
			</div>
		{/each}
	</div>
</details>

<style>
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

	.day-column {
		background: #fff;
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

	@media (max-width: 767.98px) {
		.weekend-grid {
			grid-template-columns: 1fr;
		}
	}
</style>

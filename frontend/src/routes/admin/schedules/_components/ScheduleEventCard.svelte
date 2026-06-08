<!--
	ScheduleEventCard — single schedule entry card. Two layouts:
	  - variant="weekly": full card with checkbox, badges, toggle/duplicate/edit/delete.
	  - variant="weekend": compact card with just edit/delete.
	Extracted from /admin/schedules in R21-C.
-->
<script lang="ts">
	import IconClock from '@tabler/icons-svelte-runes/icons/clock';
	import IconUser from '@tabler/icons-svelte-runes/icons/user';
	import IconCheck from '@tabler/icons-svelte-runes/icons/check';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconCopy from '@tabler/icons-svelte-runes/icons/copy';
	import IconEdit from '@tabler/icons-svelte-runes/icons/edit';
	import IconTrash from '@tabler/icons-svelte-runes/icons/trash';
	import type { ScheduleEvent } from './types';

	type Props =
		| {
				variant: 'weekly';
				event: ScheduleEvent;
				selected: boolean;
				conflict: boolean;
				formatTime: (t: string) => string;
				onToggleSelect: (id: number) => void;
				onToggleActive: (event: ScheduleEvent) => void;
				onDuplicate: (event: ScheduleEvent) => void;
				onEdit: (event: ScheduleEvent) => void;
				onDelete: (id: number) => void;
		  }
		| {
				variant: 'weekend';
				event: ScheduleEvent;
				formatTime: (t: string) => string;
				onEdit: (event: ScheduleEvent) => void;
				onDelete: (id: number) => void;
		  };

	let props: Props = $props();
</script>

{#if props.variant === 'weekly'}
	{const { event, selected, conflict, formatTime } = props}
	<div class={['event-card', { inactive: !event.is_active, selected, conflict }]}>
		<div class="event-checkbox">
			<!-- FIX-2026-04-26 (P3-1): unique id+name per event so a11y label
				 association and DOM-test selectors stop colliding. -->
			<input
				id={`schedule-checkbox-${event.id}`}
				name={`schedule-checkbox-${event.id}`}
				type="checkbox"
				checked={selected}
				onchange={() => props.variant === 'weekly' && props.onToggleSelect(event.id)}
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
				<span class={['badge', `badge-${event.room_type}`]}>{event.room_type}</span>
				{#if !event.is_active}
					<span class="badge badge-inactive">Inactive</span>
				{/if}
			</div>
		</div>
		<div class="event-actions">
			<button
				class="btn-icon"
				onclick={() => props.variant === 'weekly' && props.onToggleActive(event)}
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
				onclick={() => props.variant === 'weekly' && props.onDuplicate(event)}
				title="Duplicate"
			>
				<IconCopy size={16} />
			</button>
			<button
				class="btn-icon"
				onclick={() => props.variant === 'weekly' && props.onEdit(event)}
				title="Edit"
			>
				<IconEdit size={16} />
			</button>
			<button
				class="btn-icon btn-danger"
				onclick={() => props.variant === 'weekly' && props.onDelete(event.id)}
				title="Delete"
			>
				<IconTrash size={16} />
			</button>
		</div>
	</div>
{:else}
	{const { event, formatTime } = props}
	<div class={['event-card', { inactive: !event.is_active }]}>
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
			<button class="btn-icon" onclick={() => props.variant === 'weekend' && props.onEdit(event)}>
				<IconEdit size={16} />
			</button>
			<button
				class="btn-icon btn-danger"
				onclick={() => props.variant === 'weekend' && props.onDelete(event.id)}
			>
				<IconTrash size={16} />
			</button>
		</div>
	</div>
{/if}

<style>
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

	.event-card:not(:has(.event-checkbox)) .event-content {
		padding-left: 0;
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
</style>

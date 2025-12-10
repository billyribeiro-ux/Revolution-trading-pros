<!--
/**
 * RoomSelector Component - Learning Center
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Displays available trading rooms with learning centers.
 * Used in admin for selecting which room content belongs to.
 *
 * @version 1.0.0 (December 2025)
 */
-->

<script lang="ts">
	import type { TradingRoom } from '$lib/types/learning-center';
	import { IconCheck } from '$lib/icons';

	interface Props {
		rooms: TradingRoom[];
		selectedRoomIds?: string[];
		onSelect?: (roomIds: string[]) => void;
		multiple?: boolean;
		showDescription?: boolean;
		compact?: boolean;
	}

	let {
		rooms,
		selectedRoomIds = $bindable([]),
		onSelect,
		multiple = true,
		showDescription = true,
		compact = false
	}: Props = $props();

	// Filter rooms that have learning centers
	let availableRooms = $derived(rooms.filter(r => r.hasLearningCenter && r.isActive));

	function handleRoomClick(roomId: string) {
		if (multiple) {
			if (selectedRoomIds.includes(roomId)) {
				selectedRoomIds = selectedRoomIds.filter(id => id !== roomId);
			} else {
				selectedRoomIds = [...selectedRoomIds, roomId];
			}
		} else {
			selectedRoomIds = [roomId];
		}
		onSelect?.(selectedRoomIds);
	}

	function isSelected(roomId: string): boolean {
		return selectedRoomIds.includes(roomId);
	}

	// Type badge colors
	const typeColors: Record<string, string> = {
		'trading-room': '#f97316',
		'alert-service': '#3b82f6',
		mastery: '#8b5cf6',
		course: '#10b981',
		indicator: '#f59e0b'
	};
</script>

<div class="room-selector" class:compact>
	{#if multiple}
		<p class="selector-hint">
			Select the trading room(s) this content belongs to:
		</p>
	{/if}

	<div class="rooms-grid">
		{#each availableRooms as room (room.id)}
			<button
				type="button"
				class="room-card"
				class:selected={isSelected(room.id)}
				class:compact
				onclick={() => handleRoomClick(room.id)}
				aria-pressed={isSelected(room.id)}
			>
				<!-- Selection indicator -->
				<div class="selection-indicator" class:selected={isSelected(room.id)}>
					{#if isSelected(room.id)}
						<IconCheck size={14} />
					{/if}
				</div>

				<!-- Room icon -->
				<div class="room-icon" style="background-color: {room.color || typeColors[room.type] || '#64748b'}">
					<span>{room.icon || '📊'}</span>
				</div>

				<!-- Room info -->
				<div class="room-info">
					<span class="room-name">{room.shortName || room.name}</span>
					{#if showDescription && !compact && room.description}
						<span class="room-description">{room.description}</span>
					{/if}
					<span class="room-type" style="color: {typeColors[room.type] || '#64748b'}">
						{room.type.replace('-', ' ')}
					</span>
				</div>
			</button>
		{/each}
	</div>

	<!-- Selected count -->
	{#if multiple && selectedRoomIds.length > 0}
		<div class="selection-summary">
			<span>{selectedRoomIds.length} room{selectedRoomIds.length !== 1 ? 's' : ''} selected</span>
			<button
				type="button"
				class="clear-selection"
				onclick={() => {
					selectedRoomIds = [];
					onSelect?.([]);
				}}
			>
				Clear
			</button>
		</div>
	{/if}
</div>

<style>
	.room-selector {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.selector-hint {
		margin: 0;
		font-size: 0.875rem;
		color: #94a3b8;
	}

	.rooms-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 0.75rem;
	}

	.room-selector.compact .rooms-grid {
		grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
	}

	/* Room card */
	.room-card {
		display: flex;
		align-items: center;
		gap: 0.875rem;
		padding: 1rem;
		background: #1e293b;
		border: 2px solid #334155;
		border-radius: 10px;
		cursor: pointer;
		text-align: left;
		transition: all 0.2s ease;
	}

	.room-card.compact {
		padding: 0.75rem;
		gap: 0.625rem;
	}

	.room-card:hover {
		border-color: #475569;
		background: #253346;
	}

	.room-card.selected {
		border-color: #f97316;
		background: rgba(249, 115, 22, 0.05);
	}

	/* Selection indicator */
	.selection-indicator {
		width: 22px;
		height: 22px;
		display: flex;
		align-items: center;
		justify-content: center;
		border: 2px solid #475569;
		border-radius: 6px;
		flex-shrink: 0;
		transition: all 0.2s ease;
	}

	.room-card.compact .selection-indicator {
		width: 18px;
		height: 18px;
		border-radius: 4px;
	}

	.selection-indicator.selected {
		background: #f97316;
		border-color: #f97316;
		color: white;
	}

	/* Room icon */
	.room-icon {
		width: 44px;
		height: 44px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 10px;
		font-size: 1.25rem;
		flex-shrink: 0;
	}

	.room-card.compact .room-icon {
		width: 36px;
		height: 36px;
		border-radius: 8px;
		font-size: 1rem;
	}

	/* Room info */
	.room-info {
		display: flex;
		flex-direction: column;
		min-width: 0;
		flex: 1;
	}

	.room-name {
		font-size: 0.9rem;
		font-weight: 600;
		color: white;
		line-height: 1.3;
	}

	.room-card.compact .room-name {
		font-size: 0.8rem;
	}

	.room-description {
		font-size: 0.75rem;
		color: #94a3b8;
		line-height: 1.4;
		margin-top: 0.125rem;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.room-type {
		font-size: 0.65rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-top: 0.25rem;
	}

	/* Selection summary */
	.selection-summary {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.75rem 1rem;
		background: rgba(249, 115, 22, 0.1);
		border-radius: 8px;
		border: 1px solid rgba(249, 115, 22, 0.2);
	}

	.selection-summary span {
		font-size: 0.875rem;
		color: #f97316;
		font-weight: 500;
	}

	.clear-selection {
		background: none;
		border: none;
		padding: 0.25rem 0.5rem;
		color: #94a3b8;
		font-size: 0.8rem;
		cursor: pointer;
		transition: color 0.2s ease;
	}

	.clear-selection:hover {
		color: white;
	}

	/* Responsive */
	@media (max-width: 640px) {
		.rooms-grid {
			grid-template-columns: 1fr;
		}
	}
</style>

<script lang="ts">
	/**
	 * RoomSelector Component
	 * Select one or more trading rooms for lesson assignment
	 */

	import type { TradingRoom } from '$lib/types/learning-center';
	import IconCheck from '@tabler/icons-svelte/icons/check';

	interface Props {
		rooms: TradingRoom[];
		selectedRoomIds: string[];
		onSelect: (roomIds: string[]) => void;
		multiple?: boolean;
		showDescription?: boolean;
	}

	let { rooms, selectedRoomIds, onSelect, multiple = false, showDescription = false }: Props = $props();

	function toggleRoom(roomId: string) {
		if (multiple) {
			const isSelected = selectedRoomIds.includes(roomId);
			if (isSelected) {
				onSelect(selectedRoomIds.filter(id => id !== roomId));
			} else {
				onSelect([...selectedRoomIds, roomId]);
			}
		} else {
			onSelect([roomId]);
		}
	}

	function isSelected(roomId: string): boolean {
		return selectedRoomIds.includes(roomId);
	}
</script>

<div class="room-selector" class:multiple>
	{#if rooms.length === 0}
		<div class="no-rooms">
			<p>No trading rooms available with learning centers.</p>
		</div>
	{:else}
		<div class="room-grid">
			{#each rooms as room (room.id)}
				<button
					type="button"
					class="room-card"
					class:selected={isSelected(room.id)}
					onclick={() => toggleRoom(room.id)}
				>
					<div class="room-header">
						<span class="room-icon">{room.icon || '📚'}</span>
						<span class="room-name">{room.name}</span>
						{#if isSelected(room.id)}
							<span class="check-icon">
								<IconCheck size={18} />
							</span>
						{/if}
					</div>
					{#if showDescription && room.description}
						<p class="room-description">{room.description}</p>
					{/if}
				</button>
			{/each}
		</div>
	{/if}
</div>

<style>
	.room-selector {
		width: 100%;
	}

	.no-rooms {
		padding: 24px;
		text-align: center;
		color: var(--text-muted, #666);
		background: var(--bg-secondary, #f5f5f5);
		border-radius: 8px;
	}

	.room-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 16px;
	}

	.room-card {
		display: flex;
		flex-direction: column;
		align-items: flex-start;
		padding: 16px;
		background: var(--bg-secondary, #f5f5f5);
		border: 2px solid transparent;
		border-radius: 12px;
		cursor: pointer;
		transition: all 0.2s ease;
		text-align: left;
		width: 100%;
	}

	.room-card:hover {
		border-color: var(--primary, #0984ae);
		background: var(--bg-primary, #fff);
	}

	.room-card.selected {
		border-color: var(--primary, #0984ae);
		background: var(--bg-primary, #fff);
		box-shadow: 0 2px 8px rgba(9, 132, 174, 0.15);
	}

	.room-header {
		display: flex;
		align-items: center;
		gap: 12px;
		width: 100%;
	}

	.room-icon {
		font-size: 24px;
	}

	.room-name {
		flex: 1;
		font-weight: 600;
		font-size: 16px;
		color: var(--text-primary, #1a1a1a);
	}

	.check-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		background: var(--primary, #0984ae);
		border-radius: 50%;
		color: white;
	}

	.room-description {
		margin: 8px 0 0;
		font-size: 14px;
		color: var(--text-muted, #666);
		line-height: 1.5;
	}
</style>

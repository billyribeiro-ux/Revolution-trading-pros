<!--
	RoomSelector — tab strip across all trading rooms + selection info line.
	Extracted from /admin/schedules in R21-C.
-->
<script lang="ts">
	import type { Room } from '$lib/config/rooms';

	interface Props {
		rooms: readonly Room[];
		selectedRoomId: string;
		selectedRoom: Room | undefined;
		activeSchedules: number;
		totalSchedules: number;
		onselectRoom: (roomId: string) => void;
	}

	let {
		rooms,
		selectedRoomId,
		selectedRoom,
		activeSchedules,
		totalSchedules,
		onselectRoom
	}: Props = $props();
</script>

<div class="room-selector">
	<div class="room-tabs" role="tablist">
		{#each rooms as room (room.id)}
			<button
				class="room-tab"
				class:active={selectedRoomId === room.id}
				onclick={() => onselectRoom(room.id)}
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
			<span class="room-full-name" style="color: {selectedRoom.color}">{selectedRoom.name}</span>
			<span class="room-stats">
				{activeSchedules} active / {totalSchedules} total
			</span>
		{/if}
	</div>
</div>

<style>
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

	@media (max-width: 767.98px) {
		.room-tabs {
			overflow-x: auto;
			flex-wrap: nowrap;
			padding-bottom: 8px;
		}
	}
</style>

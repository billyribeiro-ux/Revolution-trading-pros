<script lang="ts">
	import type { TradingRoom } from '#lib/api/trading-rooms.js';

	type Props = {
		rooms: TradingRoom[];
		selectedRoom: TradingRoom | null;
		isLoadingRooms: boolean;
		onSelect: (room: TradingRoom) => void;
	};

	const { rooms, selectedRoom, isLoadingRooms, onSelect }: Props = $props();
</script>

{#if isLoadingRooms}
	<div class="room-tabs loading">
		<div class="spinner"></div>
		<span>Loading rooms...</span>
	</div>
{:else if rooms.length > 0}
	<div class="room-tabs" role="tablist" aria-label="Trading rooms">
		{#each rooms as room (room.id)}
			<button
				type="button"
				role="tab"
				aria-selected={selectedRoom?.id === room.id}
				class={['room-tab', { active: selectedRoom?.id === room.id }]}
				onclick={() => onSelect(room)}
			>
				{room.name}
			</button>
		{/each}
	</div>
{/if}

<style>
	/* Room Tabs */
	.room-tabs {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 1.5rem;
		flex-wrap: wrap;
		padding: 0.5rem;
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-muted, #21262d);
		border-radius: var(--radius-lg, 8px);
	}

	.room-tabs.loading {
		justify-content: center;
		padding: 1rem;
	}

	.room-tab {
		padding: 0.75rem 1.5rem;
		background: transparent;
		border: none;
		border-radius: 8px;
		color: var(--text-secondary, #8b949e);
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.room-tab:hover {
		background: var(--bg-hover, #252b33);
		color: var(--text-primary, #f0f6fc);
	}

	.room-tab.active {
		background: var(--primary-500);
		color: var(--bg-base);
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid var(--border-default, #30363d);
		border-top-color: var(--primary-500);
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Mobile - horizontal scroll tabs */
	@media (max-width: 767px) {
		.room-tabs {
			flex-wrap: nowrap;
			overflow-x: auto;
			-webkit-overflow-scrolling: touch;
			scrollbar-width: none;
			gap: 0.375rem;
			padding: 0.375rem;
		}

		.room-tabs::-webkit-scrollbar {
			display: none;
		}

		.room-tab {
			min-height: 44px;
			padding: 0.625rem 1rem;
			font-size: 0.8125rem;
			white-space: nowrap;
			flex-shrink: 0;
		}
	}
</style>

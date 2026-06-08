<script lang="ts">
	import type { TradingRoom } from '$lib/api/trading-rooms';

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
{:else}
	<div class="room-tabs">
		{#each rooms as room (room.id)}
			<button
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
		background: rgba(15, 23, 42, 0.6);
		border-radius: 12px;
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
		color: #94a3b8;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.room-tab:hover {
		background: rgba(230, 184, 0, 0.1);
		color: #e2e8f0;
	}

	.room-tab.active {
		background: linear-gradient(135deg, var(--primary-500) 0%, var(--primary-600) 100%);
		color: var(--bg-base);
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(230, 184, 0, 0.2);
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

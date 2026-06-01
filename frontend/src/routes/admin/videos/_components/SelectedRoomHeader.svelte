<script lang="ts">
	import type { TradingRoom } from '$lib/api/trading-rooms';

	interface Props {
		room: TradingRoom;
		videoCount: number;
		totalViews: number;
		publishedCount: number;
		formatViews: (views: number) => string;
	}

	let { room, videoCount, totalViews, publishedCount, formatViews }: Props = $props();
</script>

<div class="selected-room-header" style:--room-color={room.color}>
	<div class="selected-room-info">
		<h2>{room.name}</h2>
		<span class="room-type-badge"
			>{room.type === 'trading_room' ? 'Trading Room' : 'Alert Service'}</span
		>
	</div>
	<div class="selected-room-stats">
		<div class="stat">
			<span class="stat-value">{videoCount}</span>
			<span class="stat-label">Videos</span>
		</div>
		<div class="stat">
			<span class="stat-value">{formatViews(totalViews)}</span>
			<span class="stat-label">Total Views</span>
		</div>
		<div class="stat">
			<span class="stat-value">{publishedCount}</span>
			<span class="stat-label">Published</span>
		</div>
	</div>
</div>

<style>
	.selected-room-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.25rem;
		background: rgba(22, 27, 34, 0.6);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 14px;
		margin-bottom: 1.5rem;
		border-left: 4px solid var(--room-color, var(--primary-500));
	}

	.selected-room-info h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--text-primary);
		margin: 0 0 0.25rem 0;
	}

	.room-type-badge {
		display: inline-block;
		padding: 0.25rem 0.625rem;
		background: rgba(230, 184, 0, 0.15);
		border-radius: 20px;
		font-size: 0.75rem;
		color: var(--text-secondary);
	}

	.selected-room-stats {
		display: flex;
		gap: 2rem;
	}

	.selected-room-stats .stat {
		text-align: center;
	}

	.selected-room-stats .stat-value {
		display: block;
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--text-primary);
	}

	.selected-room-stats .stat-label {
		font-size: 0.8rem;
		color: var(--text-tertiary);
	}

	@media (max-width: 767.98px) {
		.selected-room-header {
			flex-direction: column;
			gap: 1rem;
			text-align: center;
		}

		.selected-room-stats {
			width: 100%;
			justify-content: space-around;
		}
	}
</style>

<!--
	PageHeader — back link, room badge, title, and room stats panel.
	Extracted from +page.svelte (R8-C) — read-only display, 0 binds.
-->
<script lang="ts">
	import IconChevronLeft from '@tabler/icons-svelte-runes/icons/chevron-left';
	import type { RoomStats } from '$lib/api/room-content';

	interface RoomBadge {
		name?: string;
		color?: string;
		icon?: string;
	}

	interface Props {
		room: RoomBadge | undefined;
		roomStats: RoomStats | null;
		isLoadingStats: boolean;
	}

	const { room, roomStats, isLoadingStats }: Props = $props();
</script>

<header class="page-header">
	<a href="/admin/trading-rooms" class="back-link">
		<IconChevronLeft size={20} />
		<span>All Trading Rooms</span>
	</a>
	<div class="header-row">
		<div class="header-content">
			<div class="room-badge" style="background: {room?.color || '#143E59'}">
				{room?.icon || '📊'}
			</div>
			<div>
				<h1>{room?.name || 'Trading Room'}</h1>
				<p>Manage trade plans, alerts, and weekly videos</p>
			</div>
		</div>
		{#if roomStats && !isLoadingStats}
			<div class="stats-panel">
				<div class="stat-item">
					<span class="stat-value" style="color: #22c55e">{roomStats.win_rate ?? '-'}%</span>
					<span class="stat-label">Win Rate</span>
				</div>
				<div class="stat-item">
					<span class="stat-value">{roomStats.active_trades ?? 0}</span>
					<span class="stat-label">Active</span>
				</div>
				<div class="stat-item">
					<span class="stat-value">{roomStats.closed_this_week ?? 0}</span>
					<span class="stat-label">This Week</span>
				</div>
				<div class="stat-item">
					<span class="stat-value">{roomStats.total_trades ?? 0}</span>
					<span class="stat-label">Total</span>
				</div>
			</div>
		{/if}
	</div>
</header>

<style>
	/* Header */
	.page-header {
		margin-bottom: 32px;
	}

	.back-link {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		color: #64748b;
		text-decoration: none;
		font-size: 14px;
		font-weight: 500;
		margin-bottom: 16px;
		transition: color 0.2s;
	}

	.back-link:hover {
		color: #143e59;
	}

	.header-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
		gap: 24px;
		flex-wrap: wrap;
	}

	.header-content {
		display: flex;
		align-items: center;
		gap: 20px;
	}

	.room-badge {
		width: 64px;
		height: 64px;
		border-radius: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 32px;
	}

	.header-content h1 {
		font-size: 28px;
		font-weight: 700;
		color: #1e293b;
		margin: 0 0 4px 0;
	}

	.header-content p {
		font-size: 15px;
		color: #64748b;
		margin: 0;
	}

	/* Stats Panel */
	.stats-panel {
		display: flex;
		gap: 24px;
		background: #f8fafc;
		padding: 16px 24px;
		border-radius: 12px;
		border: 1px solid #e2e8f0;
	}

	.stat-item {
		display: flex;
		flex-direction: column;
		align-items: center;
		min-width: 60px;
	}

	.stat-value {
		font-size: 24px;
		font-weight: 700;
		color: #1e293b;
		line-height: 1;
	}

	.stat-label {
		font-size: 11px;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-top: 4px;
	}

	@media (max-width: 767.98px) {
		.header-row {
			flex-direction: column;
			align-items: flex-start;
		}

		.stats-panel {
			width: 100%;
			justify-content: space-between;
		}
	}
</style>

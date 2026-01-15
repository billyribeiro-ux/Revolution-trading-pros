<!--
	Trading Rooms Admin - Room Content Management Hub
	═══════════════════════════════════════════════════════════════════════════════════
	Apple Principal Engineer ICT 7+ Grade - January 13, 2026
	
	Central hub for managing all trading room content:
	- Trade Plans (watchlist table)
	- Alerts (entry/exit/update)
	- Weekly Videos
	- Stats
	
	@version 1.0.0
-->
<script lang="ts">
	import { goto } from '$app/navigation';
	import { ROOMS, type Room } from '$lib/config/rooms';
	import IconChartBar from '@tabler/icons-svelte/icons/chart-bar';
	import IconBell from '@tabler/icons-svelte/icons/bell';
	import IconVideo from '@tabler/icons-svelte/icons/video';
	import IconTable from '@tabler/icons-svelte/icons/table';
	import IconArrowRight from '@tabler/icons-svelte/icons/arrow-right';
	import IconFlame from '@tabler/icons-svelte/icons/flame';
	import IconTrendingUp from '@tabler/icons-svelte/icons/trending-up';
	import IconTargetArrow from '@tabler/icons-svelte/icons/target-arrow';

	// ═══════════════════════════════════════════════════════════════════════════════
	// DERIVED STATE - Svelte 5 Runes
	// ═══════════════════════════════════════════════════════════════════════════════

	/**
	 * Filter to rooms that have content management capabilities
	 * Includes alerts-only services and day trading room
	 */
	const managedRooms = $derived(
		ROOMS.filter(r => r.type === 'alerts-only' || r.slug === 'day-trading-room')
	);

	// ═══════════════════════════════════════════════════════════════════════════════
	// HANDLERS
	// ═══════════════════════════════════════════════════════════════════════════════

	function navigateToRoom(room: Room): void {
		goto(`/admin/trading-rooms/${room.slug}`);
	}

	/**
	 * Get the appropriate icon component for a room based on its slug
	 */
	function getIconForRoom(slug: string): typeof IconChartBar {
		switch (slug) {
			case 'explosive-swings':
				return IconFlame;
			case 'spx-profit-pulse':
				return IconTrendingUp;
			case 'day-trading-room':
				return IconChartBar;
			default:
				return IconTargetArrow;
		}
	}
</script>

<svelte:head>
	<title>Trading Rooms Content | Admin</title>
</svelte:head>

<div class="admin-page">
	<!-- Header -->
	<header class="page-header">
		<div class="header-content">
			<h1>Trading Rooms Content</h1>
			<p>Manage trade plans, alerts, and weekly videos for each trading room</p>
		</div>
	</header>

	<!-- Room Cards Grid -->
	<div class="rooms-grid">
		{#each managedRooms as room}
			{@const RoomIcon = getIconForRoom(room.slug)}
			<button 
				class="room-card"
				onclick={() => navigateToRoom(room)}
				style="--room-color: {room.color}"
			>
				<div class="room-icon" style="background: {room.color}">
					<RoomIcon size={32} />
				</div>
				<div class="room-info">
					<h2>{room.name}</h2>
					<span class="room-type">{room.type === 'alerts-only' ? 'Alert Service' : 'Trading Room'}</span>
				</div>
				<div class="room-features">
					<div class="feature">
						<IconTable size={16} />
						<span>Trade Plan</span>
					</div>
					<div class="feature">
						<IconBell size={16} />
						<span>Alerts</span>
					</div>
					<div class="feature">
						<IconVideo size={16} />
						<span>Weekly Video</span>
					</div>
				</div>
				<div class="room-arrow">
					<IconArrowRight size={24} />
				</div>
			</button>
		{/each}
	</div>

	<!-- Quick Stats -->
	<section class="quick-stats">
		<h2>Quick Actions</h2>
		<div class="action-cards">
			<a href="/admin/videos" class="action-card">
				<IconVideo size={24} />
				<span>Video Library</span>
				<p>Manage all room videos</p>
			</a>
			<a href="/admin/watchlist" class="action-card">
				<IconTable size={24} />
				<span>Weekly Watchlist</span>
				<p>Manage watchlist entries</p>
			</a>
			<a href="/admin/schedules" class="action-card">
				<IconChartBar size={24} />
				<span>Schedules</span>
				<p>Room schedules & calendar</p>
			</a>
		</div>
	</section>
</div>

<style>
	.admin-page {
		padding: 32px;
		max-width: 1400px;
		margin: 0 auto;
	}

	.page-header {
		margin-bottom: 40px;
	}

	.header-content h1 {
		font-size: 32px;
		font-weight: 700;
		color: #1e293b;
		margin: 0 0 8px 0;
	}

	.header-content p {
		font-size: 16px;
		color: #64748b;
		margin: 0;
	}

	/* Room Cards Grid */
	.rooms-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(360px, 1fr));
		gap: 24px;
		margin-bottom: 48px;
	}

	.room-card {
		display: flex;
		flex-direction: column;
		align-items: flex-start;
		padding: 28px;
		background: #fff;
		border: 2px solid #e2e8f0;
		border-radius: 16px;
		cursor: pointer;
		transition: all 0.3s ease;
		position: relative;
		text-align: left;
		width: 100%;
	}

	.room-card:hover {
		border-color: var(--room-color);
		box-shadow: 0 12px 40px rgba(0, 0, 0, 0.1);
		transform: translateY(-4px);
	}

	.room-icon {
		width: 64px;
		height: 64px;
		border-radius: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #fff;
		margin-bottom: 20px;
	}

	.room-info {
		margin-bottom: 20px;
	}

	.room-info h2 {
		font-size: 22px;
		font-weight: 700;
		color: #1e293b;
		margin: 0 0 6px 0;
	}

	.room-type {
		font-size: 13px;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.room-features {
		display: flex;
		flex-wrap: wrap;
		gap: 12px;
		margin-bottom: 16px;
	}

	.feature {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 8px 14px;
		background: #f1f5f9;
		border-radius: 8px;
		font-size: 13px;
		font-weight: 500;
		color: #475569;
	}

	.room-arrow {
		position: absolute;
		right: 24px;
		top: 50%;
		transform: translateY(-50%);
		color: #cbd5e1;
		transition: all 0.3s ease;
	}

	.room-card:hover .room-arrow {
		color: var(--room-color);
		transform: translateY(-50%) translateX(4px);
	}

	/* Quick Stats */
	.quick-stats {
		background: #f8fafc;
		border-radius: 16px;
		padding: 32px;
	}

	.quick-stats h2 {
		font-size: 20px;
		font-weight: 700;
		color: #1e293b;
		margin: 0 0 24px 0;
	}

	.action-cards {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(240px, 1fr));
		gap: 20px;
	}

	.action-card {
		display: flex;
		flex-direction: column;
		align-items: flex-start;
		padding: 24px;
		background: #fff;
		border: 1px solid #e2e8f0;
		border-radius: 12px;
		text-decoration: none;
		transition: all 0.2s ease;
	}

	.action-card:hover {
		border-color: #143E59;
		box-shadow: 0 4px 20px rgba(20, 62, 89, 0.1);
	}

	.action-card :global(svg) {
		color: #143E59;
		margin-bottom: 12px;
	}

	.action-card span {
		font-size: 16px;
		font-weight: 600;
		color: #1e293b;
		margin-bottom: 4px;
	}

	.action-card p {
		font-size: 14px;
		color: #64748b;
		margin: 0;
	}

	@media (max-width: 768px) {
		.admin-page {
			padding: 20px;
		}

		.rooms-grid {
			grid-template-columns: 1fr;
		}

		.room-arrow {
			display: none;
		}
	}
</style>

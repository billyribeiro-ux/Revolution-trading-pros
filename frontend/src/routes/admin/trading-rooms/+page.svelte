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

<div class="page">
	<!-- Header -->
	<header class="page-header">
		<h1>Trading Rooms Content</h1>
		<p class="subtitle">Manage trade plans, alerts, and weekly videos for each trading room</p>
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
	/* Page wrapper - email templates style */
	.page {
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
	}

	/* Header CENTERED - email templates style */
	.page-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	/* Title - email templates style */
	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.5rem 0;
	}

	/* Subtitle - email templates style */
	.subtitle {
		color: #64748b;
		font-size: 0.875rem;
		margin: 0;
	}

	/* Room Cards Grid */
	.rooms-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(360px, 1fr));
		gap: 1.5rem;
		margin-bottom: 2rem;
	}

	/* Room cards - email templates style */
	.room-card {
		display: flex;
		flex-direction: column;
		align-items: flex-start;
		padding: 1.5rem;
		background: rgba(30, 41, 59, 0.4);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.3s ease;
		position: relative;
		text-align: left;
		width: 100%;
	}

	.room-card:hover {
		border-color: var(--room-color);
		box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
		transform: translateY(-2px);
	}

	.room-icon {
		width: 56px;
		height: 56px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #fff;
		margin-bottom: 1rem;
	}

	.room-info {
		margin-bottom: 1rem;
	}

	.room-info h2 {
		font-size: 1.25rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.375rem 0;
	}

	.room-type {
		font-size: 0.75rem;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.room-features {
		display: flex;
		flex-wrap: wrap;
		gap: 0.75rem;
		margin-bottom: 1rem;
	}

	.feature {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 0.75rem;
		background: rgba(100, 116, 139, 0.2);
		border-radius: 6px;
		font-size: 0.8125rem;
		font-weight: 500;
		color: #cbd5e1;
	}

	.room-arrow {
		position: absolute;
		right: 1.5rem;
		top: 50%;
		transform: translateY(-50%);
		color: #64748b;
		transition: all 0.3s ease;
	}

	.room-card:hover .room-arrow {
		color: var(--room-color);
		transform: translateY(-50%) translateX(4px);
	}

	/* Quick Stats Section */
	.quick-stats {
		background: rgba(30, 41, 59, 0.4);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 8px;
		padding: 1.5rem;
	}

	.quick-stats h2 {
		font-size: 1.125rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 1.5rem 0;
		text-align: center;
	}

	/* Actions centered - email templates style */
	.action-cards {
		display: flex;
		justify-content: center;
		flex-wrap: wrap;
		gap: 0.75rem;
	}

	.action-card {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 1.25rem 1.5rem;
		background: rgba(100, 116, 139, 0.2);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 8px;
		text-decoration: none;
		transition: all 0.2s ease;
		min-width: 200px;
	}

	.action-card:hover {
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		border-color: transparent;
		box-shadow: 0 4px 20px rgba(99, 102, 241, 0.3);
	}

	.action-card :global(svg) {
		color: #8b5cf6;
		margin-bottom: 0.75rem;
		transition: color 0.2s ease;
	}

	.action-card:hover :global(svg) {
		color: #fff;
	}

	.action-card span {
		font-size: 0.9375rem;
		font-weight: 600;
		color: #f1f5f9;
		margin-bottom: 0.25rem;
		transition: color 0.2s ease;
	}

	.action-card:hover span {
		color: #fff;
	}

	.action-card p {
		font-size: 0.8125rem;
		color: #64748b;
		margin: 0;
		text-align: center;
		transition: color 0.2s ease;
	}

	.action-card:hover p {
		color: rgba(255, 255, 255, 0.8);
	}

	@media (max-width: 768px) {
		.page {
			padding: 1rem;
		}

		.rooms-grid {
			grid-template-columns: 1fr;
		}

		.room-arrow {
			display: none;
		}

		.action-cards {
			flex-direction: column;
			align-items: stretch;
		}

		.action-card {
			min-width: unset;
		}
	}
</style>

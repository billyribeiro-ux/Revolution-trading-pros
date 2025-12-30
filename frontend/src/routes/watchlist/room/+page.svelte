<script lang="ts">
	/**
	 * Watchlist Room Directory
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Lists all available rooms for browsing room-specific watchlist archives.
	 * @version 1.0.0 - December 2025
	 */

	import { ROOMS, getLiveTradingRooms, getAlertsOnlyServices } from '$lib/config/rooms';
	import { DashboardHeader, SectionTitle } from '$lib/components/dashboard';

	const liveTradingRooms = getLiveTradingRooms();
	const alertsOnlyServices = getAlertsOnlyServices();
</script>

<svelte:head>
	<title>Weekly Watchlist by Room | Revolution Trading Pros</title>
	<meta name="description" content="Browse Weekly Watchlist archives by trading room or service." />
</svelte:head>

<!-- DASHBOARD HEADER -->
<DashboardHeader title="Weekly Watchlist by Room" showRules={false}>
	{#snippet rightContent()}
		<a href="/watchlist/latest" class="btn-latest">
			View Latest Watchlist →
		</a>
	{/snippet}
</DashboardHeader>

<!-- ROOM DIRECTORY -->
<div class="dashboard__content">
	<div class="dashboard__content-main">
		<!-- LIVE TRADING ROOMS -->
		<section class="dashboard__content-section">
			<SectionTitle title="Live Trading Rooms" />
			<p class="section-description">
				Access watchlist archives for our live trading rooms.
			</p>
			<div class="room-grid">
				{#each liveTradingRooms as room}
					<a href="/watchlist/room/{room.slug}" class="room-card" style="--room-color: {room.color}">
						<span class="room-card__icon">{room.icon}</span>
						<h3 class="room-card__name">{room.name}</h3>
						<span class="room-card__badge">{room.shortName}</span>
						<span class="room-card__arrow">→</span>
					</a>
				{/each}
			</div>
		</section>

		<!-- ALERTS ONLY SERVICES -->
		<section class="dashboard__content-section">
			<SectionTitle title="Alerts Only Services" />
			<p class="section-description">
				Access watchlist archives for our alerts-only services.
			</p>
			<div class="room-grid">
				{#each alertsOnlyServices as room}
					<a href="/watchlist/room/{room.slug}" class="room-card" style="--room-color: {room.color}">
						<span class="room-card__icon">{room.icon}</span>
						<h3 class="room-card__name">{room.name}</h3>
						<span class="room-card__badge">{room.shortName}</span>
						<span class="room-card__arrow">→</span>
					</a>
				{/each}
			</div>
		</section>

		<!-- ALL WATCHLISTS LINK -->
		<section class="dashboard__content-section-link">
			<a href="/weeklywatchlist/archive" class="view-all-link">
				View All Watchlist Archives (All Rooms) →
			</a>
		</section>
	</div>
</div>

<style>
	/* Latest Button */
	.btn-latest {
		display: inline-block;
		padding: 8px 16px;
		font-size: 12px;
		font-weight: 700;
		background: linear-gradient(135deg, #f97316, #ea580c);
		color: white;
		border-radius: 4px;
		text-decoration: none;
		transition: all 0.2s;
	}

	.btn-latest:hover {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(249, 115, 22, 0.3);
	}

	/* Dashboard Content */
	.dashboard__content {
		display: flex;
		flex-flow: row nowrap;
	}

	.dashboard__content-main {
		flex: 1 1 auto;
		min-width: 0;
	}

	.dashboard__content-section {
		padding: 30px;
		background-color: #fff;
		border-bottom: 1px solid #dbdbdb;
	}

	@media (min-width: 1440px) {
		.dashboard__content-section {
			padding: 40px;
		}
	}

	.section-description {
		font-size: 14px;
		color: #666;
		margin: -10px 0 20px;
		font-family: 'Open Sans', sans-serif;
	}

	/* Room Grid */
	.room-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 16px;
	}

	/* Room Card */
	.room-card {
		display: flex;
		align-items: center;
		gap: 16px;
		padding: 20px;
		background: #fff;
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		text-decoration: none;
		transition: all 0.2s;
		position: relative;
	}

	.room-card:hover {
		border-color: var(--room-color, #0984ae);
		background: linear-gradient(135deg, color-mix(in srgb, var(--room-color) 5%, white), white);
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
	}

	.room-card__icon {
		font-size: 28px;
		flex-shrink: 0;
	}

	.room-card__name {
		flex: 1;
		margin: 0;
		font-size: 16px;
		font-weight: 600;
		color: #333;
		font-family: 'Open Sans', sans-serif;
	}

	.room-card__badge {
		padding: 4px 10px;
		font-size: 11px;
		font-weight: 700;
		background: var(--room-color, #0984ae);
		color: white;
		border-radius: 20px;
	}

	.room-card__arrow {
		font-size: 18px;
		color: #9ca3af;
		transition: all 0.2s;
	}

	.room-card:hover .room-card__arrow {
		color: var(--room-color, #0984ae);
		transform: translateX(3px);
	}

	/* Section Link */
	.dashboard__content-section-link {
		padding: 30px;
		background-color: #f9fafb;
		text-align: center;
	}

	.view-all-link {
		display: inline-block;
		font-size: 14px;
		font-weight: 700;
		color: #0984ae;
		text-decoration: none;
		font-family: 'Open Sans', sans-serif;
		transition: color 0.2s;
	}

	.view-all-link:hover {
		color: #065a75;
		text-decoration: underline;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.dashboard__content-section {
			padding: 20px;
		}

		.room-grid {
			grid-template-columns: 1fr;
		}

		.room-card {
			padding: 16px;
		}

		.room-card__icon {
			font-size: 24px;
		}

		.room-card__name {
			font-size: 14px;
		}
	}
</style>

<script lang="ts">
	/**
	 * Day Trading Room Dashboard - Member Dashboard
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Main hub for Day Trading Room members
	 * Cloned from Simpler Trading's mastering-the-trade structure
	 *
	 * @version 1.0.0
	 */
	import { onMount } from 'svelte';
	import DynamicIcon from '$lib/components/DynamicIcon.svelte';
	import IconArrowLeft from '@tabler/icons-svelte/icons/arrow-left';
	import IconExternalLink from '@tabler/icons-svelte/icons/external-link';
	import IconBell from '@tabler/icons-svelte/icons/bell';
	import IconVideo from '@tabler/icons-svelte/icons/video';
	import IconBook from '@tabler/icons-svelte/icons/book';
	import IconChartCandle from '@tabler/icons-svelte/icons/chart-candle';

	let loading = $state(false);
</script>

<svelte:head>
	<title>Day Trading Room | Revolution Trading Pros</title>
</svelte:head>

<div class="trading-room-dashboard">
	<!-- Back Link -->
	<a href="/dashboard" class="back-link">
		<IconArrowLeft size={18} />
		Back to Dashboard
	</a>

	<!-- Header -->
	<header class="room-header">
		<div class="room-icon">
			<DynamicIcon name="chart-line" size={48} />
		</div>
		<div class="room-info">
			<h1 class="room-title">Day Trading Room</h1>
			<p class="room-subtitle">Live trading sessions, real-time alerts, and daily market analysis</p>
		</div>
	</header>

	<!-- Quick Access Cards -->
	<div class="quick-access-grid">
		<a href="/trading-room/day-trading-room" target="_blank" class="access-card primary">
			<div class="card-icon">
				<IconExternalLink size={28} />
			</div>
			<div class="card-content">
				<h3>Enter Trading Room</h3>
				<p>Join live trading sessions</p>
			</div>
		</a>

		<a href="/dashboard/day-trading-room/alerts" class="access-card">
			<div class="card-icon">
				<IconBell size={28} />
			</div>
			<div class="card-content">
				<h3>Trade Alerts</h3>
				<p>View recent trade alerts</p>
			</div>
		</a>

		<a href="/dashboard/day-trading-room/learning-center" class="access-card">
			<div class="card-icon">
				<IconVideo size={28} />
			</div>
			<div class="card-content">
				<h3>Learning Center</h3>
				<p>Educational videos & courses</p>
			</div>
		</a>

		<a href="/dashboard/day-trading-room/resources" class="access-card">
			<div class="card-icon">
				<IconBook size={28} />
			</div>
			<div class="card-content">
				<h3>Resources</h3>
				<p>Trading guides & tools</p>
			</div>
		</a>
	</div>

	<!-- Features Section -->
	<section class="features-section">
		<h2 class="section-title">What's Included</h2>
		<div class="features-grid">
			<div class="feature-item">
				<div class="feature-icon">
					<IconChartCandle size={24} />
				</div>
				<div class="feature-content">
					<h3>Live Day Trading Sessions</h3>
					<p>Join our expert traders during market hours for real-time analysis and trade execution</p>
				</div>
			</div>

			<div class="feature-item">
				<div class="feature-icon">
					<IconBell size={24} />
				</div>
				<div class="feature-content">
					<h3>Real-Time Trade Alerts</h3>
					<p>Receive instant notifications for entry points, exit strategies, and stop loss levels</p>
				</div>
			</div>

			<div class="feature-item">
				<div class="feature-icon">
					<IconVideo size={24} />
				</div>
				<div class="feature-content">
					<h3>Daily Market Analysis</h3>
					<p>Pre-market and post-market video analysis to prepare you for each trading day</p>
				</div>
			</div>

			<div class="feature-item">
				<div class="feature-icon">
					<IconBook size={24} />
				</div>
				<div class="feature-content">
					<h3>Educational Resources</h3>
					<p>Access our comprehensive library of trading strategies, guides, and tutorials</p>
				</div>
			</div>
		</div>
	</section>

	<!-- Schedule Section -->
	<section class="schedule-section">
		<h2 class="section-title">Trading Room Schedule</h2>
		<div class="schedule-card">
			<div class="schedule-item">
				<span class="schedule-label">Pre-Market Analysis</span>
				<span class="schedule-time">8:30 AM - 9:00 AM EST</span>
			</div>
			<div class="schedule-item">
				<span class="schedule-label">Live Trading Session</span>
				<span class="schedule-time">9:30 AM - 12:00 PM EST</span>
			</div>
			<div class="schedule-item">
				<span class="schedule-label">Afternoon Session</span>
				<span class="schedule-time">1:00 PM - 3:00 PM EST</span>
			</div>
			<div class="schedule-item">
				<span class="schedule-label">Post-Market Review</span>
				<span class="schedule-time">4:00 PM - 4:30 PM EST</span>
			</div>
		</div>
	</section>
</div>

<style>
	.trading-room-dashboard {
		padding: 30px;
		max-width: 1200px;
		margin: 0 auto;
	}

	.back-link {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		color: #1e73be;
		text-decoration: none;
		font-size: 14px;
		font-weight: 500;
		margin-bottom: 24px;
	}

	.back-link:hover {
		text-decoration: underline;
	}

	/* Header */
	.room-header {
		display: flex;
		align-items: center;
		gap: 24px;
		margin-bottom: 32px;
		padding: 24px;
		background: #fff;
		border-radius: 12px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
	}

	.room-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 80px;
		height: 80px;
		background: linear-gradient(135deg, #0984ae 0%, #076787 100%);
		border-radius: 16px;
		color: #fff;
		flex-shrink: 0;
	}

	.room-info {
		flex: 1;
	}

	.room-title {
		font-size: 28px;
		font-weight: 600;
		color: #333;
		margin: 0 0 8px;
	}

	.room-subtitle {
		font-size: 14px;
		color: #666;
		margin: 0;
		line-height: 1.5;
	}

	/* Quick Access Grid */
	.quick-access-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
		gap: 20px;
		margin-bottom: 48px;
	}

	.access-card {
		display: flex;
		align-items: center;
		gap: 16px;
		padding: 24px;
		background: #fff;
		border-radius: 12px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
		text-decoration: none;
		color: inherit;
		transition: all 0.15s;
		border: 2px solid transparent;
	}

	.access-card:hover {
		box-shadow: 0 4px 16px rgba(0, 0, 0, 0.12);
		transform: translateY(-2px);
		border-color: #0984ae;
	}

	.access-card.primary {
		background: linear-gradient(135deg, #0984ae 0%, #076787 100%);
		color: #fff;
	}

	.access-card.primary:hover {
		border-color: #fff;
	}

	.card-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 56px;
		height: 56px;
		background: rgba(255, 255, 255, 0.15);
		border-radius: 12px;
		flex-shrink: 0;
	}

	.access-card:not(.primary) .card-icon {
		background: linear-gradient(135deg, #0984ae 0%, #076787 100%);
		color: #fff;
	}

	.card-content h3 {
		font-size: 16px;
		font-weight: 600;
		margin: 0 0 4px;
	}

	.card-content p {
		font-size: 13px;
		margin: 0;
		opacity: 0.9;
	}

	/* Features Section */
	.features-section,
	.schedule-section {
		margin-bottom: 48px;
	}

	.section-title {
		font-size: 22px;
		font-weight: 600;
		color: #333;
		margin: 0 0 24px;
	}

	.features-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
		gap: 24px;
	}

	.feature-item {
		display: flex;
		gap: 16px;
		padding: 24px;
		background: #fff;
		border-radius: 8px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
	}

	.feature-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 48px;
		height: 48px;
		background: linear-gradient(135deg, #0984ae 0%, #076787 100%);
		border-radius: 12px;
		color: #fff;
		flex-shrink: 0;
	}

	.feature-content h3 {
		font-size: 16px;
		font-weight: 600;
		color: #333;
		margin: 0 0 8px;
	}

	.feature-content p {
		font-size: 14px;
		color: #666;
		margin: 0;
		line-height: 1.5;
	}

	/* Schedule Section */
	.schedule-card {
		background: #fff;
		border-radius: 8px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
		padding: 24px;
	}

	.schedule-item {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 16px 0;
		border-bottom: 1px solid #e5e7eb;
	}

	.schedule-item:last-child {
		border-bottom: none;
	}

	.schedule-label {
		font-size: 15px;
		font-weight: 500;
		color: #333;
	}

	.schedule-time {
		font-size: 14px;
		color: #0984ae;
		font-weight: 600;
	}

	@media (max-width: 768px) {
		.trading-room-dashboard {
			padding: 20px;
		}

		.room-header {
			flex-direction: column;
			text-align: center;
		}

		.quick-access-grid {
			grid-template-columns: 1fr;
		}

		.features-grid {
			grid-template-columns: 1fr;
		}

		.schedule-item {
			flex-direction: column;
			gap: 8px;
			text-align: center;
		}
	}
</style>

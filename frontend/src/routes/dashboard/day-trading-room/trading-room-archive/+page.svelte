<script lang="ts">
	/**
	 * Day Trading Room - Trading Room Archives
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Historical recordings of live trading room sessions
	 *
	 * @version 1.0.0
	 */
	import IconArrowLeft from '@tabler/icons-svelte/icons/arrow-left';
	import IconArchive from '@tabler/icons-svelte/icons/archive';
	import IconPlayerPlay from '@tabler/icons-svelte/icons/player-play';
	import IconCalendar from '@tabler/icons-svelte/icons/calendar';
	import IconClock from '@tabler/icons-svelte/icons/clock';

	// Mock archive data
	const archives = [
		{ id: 1, date: 'December 23, 2025', duration: '3:45:20', trader: 'HG' },
		{ id: 2, date: 'December 22, 2025', duration: '4:12:15', trader: 'Expert' },
		{ id: 3, date: 'December 19, 2025', duration: '3:58:40', trader: 'Expert' },
		{ id: 4, date: 'December 18, 2025', duration: '4:05:30', trader: 'HG' },
		{ id: 5, date: 'December 17, 2025', duration: '3:52:10', trader: 'Expert' },
		{ id: 6, date: 'December 16, 2025', duration: '4:18:25', trader: 'HG' },
		{ id: 7, date: 'December 15, 2025', duration: '3:45:55', trader: 'Expert' },
		{ id: 8, date: 'December 12, 2025', duration: '4:02:40', trader: 'Expert' },
		{ id: 9, date: 'December 11, 2025', duration: '3:55:15', trader: 'HG' },
		{ id: 10, date: 'December 10, 2025', duration: '4:10:30', trader: 'Expert' }
	];
</script>

<svelte:head>
	<title>Trading Room Archives - Day Trading Room | Revolution Trading Pros</title>
</svelte:head>

<div class="archives">
	<a href="/dashboard/day-trading-room" class="back-link">
		<IconArrowLeft size={18} />
		Back to Day Trading Room
	</a>

	<header class="page-header">
		<div class="header-icon">
			<IconArchive size={32} />
		</div>
		<div class="header-content">
			<h1 class="page-title">Trading Room Archives</h1>
			<p class="page-subtitle">Watch past live trading room sessions at your convenience</p>
		</div>
	</header>

	<div class="archives-list">
		{#each archives as archive}
			<article class="archive-card">
				<div class="archive-thumbnail">
					<div class="play-icon">
						<IconPlayerPlay size={32} />
					</div>
				</div>
				<div class="archive-info">
					<div class="archive-meta">
						<span class="archive-date">
							<IconCalendar size={16} />
							{archive.date}
						</span>
						<span class="archive-duration">
							<IconClock size={16} />
							{archive.duration}
						</span>
					</div>
					<h3 class="archive-title">Day Trading Room Session</h3>
					<p class="archive-trader">with {archive.trader}</p>
				</div>
				<a href="/chatroom-archive/day-trading-room/{archive.id}" class="watch-btn">
					Watch Recording
				</a>
			</article>
		{/each}
	</div>
</div>

<style>
	.archives {
		padding: 30px;
		max-width: 1200px;
		margin: 0 auto;
	}

	.back-link {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		color: #0984ae;
		text-decoration: none;
		font-size: 14px;
		font-weight: 500;
		margin-bottom: 24px;
	}

	.back-link:hover {
		text-decoration: underline;
	}

	.page-header {
		display: flex;
		align-items: center;
		gap: 20px;
		margin-bottom: 40px;
	}

	.header-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 64px;
		height: 64px;
		background: linear-gradient(135deg, #0984ae 0%, #076787 100%);
		border-radius: 16px;
		color: #fff;
	}

	.page-title {
		font-size: 28px;
		font-weight: 700;
		color: #333;
		margin: 0 0 4px;
		font-family: 'Open Sans', sans-serif;
	}

	.page-subtitle {
		font-size: 14px;
		color: #666;
		margin: 0;
	}

	.archives-list {
		display: flex;
		flex-direction: column;
		gap: 16px;
	}

	.archive-card {
		display: flex;
		align-items: center;
		gap: 20px;
		background: #fff;
		border-radius: 8px;
		padding: 16px 20px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
		transition: all 0.2s;
	}

	.archive-card:hover {
		box-shadow: 0 4px 16px rgba(0, 0, 0, 0.12);
	}

	.archive-thumbnail {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 80px;
		height: 60px;
		background: linear-gradient(135deg, #0984ae 0%, #076787 100%);
		border-radius: 6px;
		flex-shrink: 0;
	}

	.play-icon {
		color: #fff;
		opacity: 0.9;
	}

	.archive-info {
		flex: 1;
		min-width: 0;
	}

	.archive-meta {
		display: flex;
		align-items: center;
		gap: 16px;
		margin-bottom: 8px;
		font-size: 13px;
		color: #666;
	}

	.archive-date,
	.archive-duration {
		display: flex;
		align-items: center;
		gap: 6px;
	}

	.archive-title {
		font-size: 16px;
		font-weight: 700;
		color: #333;
		margin: 0;
		font-family: 'Open Sans', sans-serif;
	}

	.archive-trader {
		font-size: 14px;
		color: #0984ae;
		margin: 4px 0 0;
	}

	.watch-btn {
		display: inline-block;
		padding: 10px 20px;
		background: #f5f5f5;
		color: #333;
		text-decoration: none;
		border-radius: 5px;
		font-size: 14px;
		font-weight: 600;
		border: 1px solid #ddd;
		transition: all 0.2s;
		white-space: nowrap;
	}

	.watch-btn:hover {
		background: #e8e8e8;
		border-color: #ccc;
	}

	@media (max-width: 768px) {
		.archives {
			padding: 20px;
		}

		.archive-card {
			flex-direction: column;
			align-items: flex-start;
			gap: 12px;
		}

		.watch-btn {
			width: 100%;
			text-align: center;
		}
	}
</style>

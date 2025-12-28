<script lang="ts">
	/**
	 * Options Day Trading - Premium Daily Videos
	 * ═══════════════════════════════════════════════════════════════════════════
	 */
	import IconArrowLeft from '@tabler/icons-svelte/icons/arrow-left';
	import IconPlayerPlay from '@tabler/icons-svelte/icons/player-play';
	import IconCalendar from '@tabler/icons-svelte/icons/calendar';
	import IconClock from '@tabler/icons-svelte/icons/clock';

	const videos = [
		{
			id: 1,
			title: 'Options Market Analysis & Strategies',
			date: 'December 23, 2025',
			trader: 'Expert',
			duration: '20:15',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			description: 'Today we analyze key options setups for SPY, QQQ, and individual stock opportunities.'
		},
		{
			id: 2,
			title: 'Options Flow & Unusual Activity',
			date: 'December 22, 2025',
			trader: 'Expert',
			duration: '18:30',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134911/SimplerCentral_DShay.jpg',
			description: 'Breaking down the unusual options activity and what it means for our trades.'
		},
		{
			id: 3,
			title: 'Weekly Options Wrap-Up',
			date: 'December 20, 2025',
			trader: 'Expert',
			duration: '25:45',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134553/SimplerCentral_SS.jpg',
			description: 'Reviewing this week\'s options plays and setting up for next week.'
		}
	];
</script>

<svelte:head>
	<title>Premium Daily Videos - Options Day Trading | Revolution Trading Pros</title>
</svelte:head>

<div class="daily-videos">
	<a href="/dashboard/options-day-trading" class="back-link">
		<IconArrowLeft size={18} />
		Back to Options Day Trading
	</a>

	<header class="page-header">
		<div class="header-icon">
			<IconPlayerPlay size={32} />
		</div>
		<div class="header-content">
			<h1 class="page-title">Premium Daily Videos</h1>
			<p class="page-subtitle">Daily options analysis and trading insights</p>
		</div>
	</header>

	<div class="videos-grid">
		{#each videos as video}
			<article class="video-card">
				<div class="video-thumbnail" style="background-image: url({video.thumbnail})">
					<div class="play-overlay">
						<IconPlayerPlay size={48} />
					</div>
					<span class="video-duration">
						<IconClock size={14} />
						{video.duration}
					</span>
				</div>
				<div class="video-info">
					<div class="video-meta">
						<span class="video-date">
							<IconCalendar size={14} />
							{video.date}
						</span>
						<span class="video-trader">with {video.trader}</span>
					</div>
					<h3 class="video-title">{video.title}</h3>
					<p class="video-description">{video.description}</p>
					<a href="/daily/options-day-trading/{video.id}" class="watch-btn">Watch Now</a>
				</div>
			</article>
		{/each}
	</div>
</div>

<style>
	.daily-videos { padding: 30px; max-width: 1400px; margin: 0 auto; }
	.back-link { display: inline-flex; align-items: center; gap: 8px; color: #0984ae; text-decoration: none; font-size: 14px; font-weight: 500; margin-bottom: 24px; }
	.back-link:hover { text-decoration: underline; }
	.page-header { display: flex; align-items: center; gap: 20px; margin-bottom: 40px; }
	.header-icon { display: flex; align-items: center; justify-content: center; width: 64px; height: 64px; background: linear-gradient(135deg, #0984ae 0%, #076787 100%); border-radius: 16px; color: #fff; }
	.page-title { font-size: 28px; font-weight: 700; color: #333; margin: 0 0 4px; font-family: 'Open Sans', sans-serif; }
	.page-subtitle { font-size: 14px; color: #666; margin: 0; }
	.videos-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(380px, 1fr)); gap: 24px; }
	.video-card { background: #fff; border-radius: 8px; overflow: hidden; box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1); transition: all 0.2s; }
	.video-card:hover { box-shadow: 0 5px 30px rgba(0, 0, 0, 0.15); transform: translateY(-2px); }
	.video-thumbnail { position: relative; width: 100%; padding-top: 56.25%; background-size: cover; background-position: center; background-color: #0984ae; }
	.play-overlay { position: absolute; top: 0; left: 0; right: 0; bottom: 0; display: flex; align-items: center; justify-content: center; background: rgba(0, 0, 0, 0.3); color: #fff; opacity: 0; transition: opacity 0.2s; cursor: pointer; }
	.video-card:hover .play-overlay { opacity: 1; }
	.video-duration { position: absolute; bottom: 8px; right: 8px; display: flex; align-items: center; gap: 4px; padding: 4px 8px; background: rgba(0, 0, 0, 0.8); color: #fff; border-radius: 4px; font-size: 12px; font-weight: 500; }
	.video-info { padding: 20px; }
	.video-meta { display: flex; align-items: center; gap: 12px; margin-bottom: 12px; font-size: 13px; color: #666; }
	.video-date { display: flex; align-items: center; gap: 4px; }
	.video-trader { color: #0984ae; font-weight: 500; }
	.video-title { font-size: 18px; font-weight: 700; color: #333; margin: 0 0 8px; line-height: 1.4; font-family: 'Open Sans', sans-serif; }
	.video-description { font-size: 14px; color: #666; margin: 0 0 16px; line-height: 1.5; }
	.watch-btn { display: inline-block; padding: 10px 20px; background: #0984ae; color: #fff; text-decoration: none; border-radius: 4px; font-size: 14px; font-weight: 600; transition: background 0.2s; }
	.watch-btn:hover { background: #076787; }
	@media (max-width: 768px) { .daily-videos { padding: 20px; } .videos-grid { grid-template-columns: 1fr; } }
</style>

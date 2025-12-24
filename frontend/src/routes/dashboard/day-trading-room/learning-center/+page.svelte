<script lang="ts">
	/**
	 * Day Trading Room - Learning Center
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Educational videos and course content
	 *
	 * @version 1.0.0
	 */
	import IconArrowLeft from '@tabler/icons-svelte/icons/arrow-left';
	import IconVideo from '@tabler/icons-svelte/icons/video';
	import IconPlayerPlay from '@tabler/icons-svelte/icons/player-play';
	import IconClock from '@tabler/icons-svelte/icons/clock';

	// Mock video data
	const videoCategories = [
		{
			title: 'Getting Started',
			videos: [
				{ id: 1, title: 'Welcome to Day Trading Room', duration: '12:45', thumbnail: '' },
				{ id: 2, title: 'Platform Overview', duration: '18:30', thumbnail: '' },
				{ id: 3, title: 'Setting Up Your Workspace', duration: '15:20', thumbnail: '' }
			]
		},
		{
			title: 'Trading Strategies',
			videos: [
				{ id: 4, title: 'Momentum Trading Basics', duration: '25:15', thumbnail: '' },
				{ id: 5, title: 'Reading Price Action', duration: '32:40', thumbnail: '' },
				{ id: 6, title: 'Risk Management Essentials', duration: '28:10', thumbnail: '' }
			]
		},
		{
			title: 'Technical Analysis',
			videos: [
				{ id: 7, title: 'Support & Resistance', duration: '22:35', thumbnail: '' },
				{ id: 8, title: 'Chart Patterns', duration: '30:20', thumbnail: '' },
				{ id: 9, title: 'Indicators Deep Dive', duration: '35:45', thumbnail: '' }
			]
		}
	];
</script>

<svelte:head>
	<title>Learning Center - Day Trading Room | Revolution Trading Pros</title>
</svelte:head>

<div class="learning-center">
	<a href="/dashboard/day-trading-room" class="back-link">
		<IconArrowLeft size={18} />
		Back to Day Trading Room
	</a>

	<header class="page-header">
		<div class="header-icon">
			<IconVideo size={32} />
		</div>
		<div class="header-content">
			<h1 class="page-title">Learning Center</h1>
			<p class="page-subtitle">Master day trading with our comprehensive video library</p>
		</div>
	</header>

	<div class="content-wrapper">
		{#each videoCategories as category}
			<section class="video-category">
				<h2 class="category-title">{category.title}</h2>
				<div class="videos-grid">
					{#each category.videos as video}
						<article class="video-card">
							<div class="video-thumbnail">
								<div class="play-overlay">
									<IconPlayerPlay size={48} />
								</div>
								<span class="video-duration">
									<IconClock size={14} />
									{video.duration}
								</span>
							</div>
							<div class="video-info">
								<h3 class="video-title">{video.title}</h3>
							</div>
						</article>
					{/each}
				</div>
			</section>
		{/each}
	</div>
</div>

<style>
	.learning-center {
		padding: 30px;
		max-width: 1400px;
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
		font-weight: 600;
		color: #333;
		margin: 0 0 4px;
	}

	.page-subtitle {
		font-size: 14px;
		color: #666;
		margin: 0;
	}

	.video-category {
		margin-bottom: 48px;
	}

	.category-title {
		font-size: 20px;
		font-weight: 600;
		color: #333;
		margin: 0 0 20px;
	}

	.videos-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		gap: 24px;
	}

	.video-card {
		background: #fff;
		border-radius: 8px;
		overflow: hidden;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
		cursor: pointer;
		transition: all 0.15s;
	}

	.video-card:hover {
		box-shadow: 0 4px 16px rgba(0, 0, 0, 0.12);
		transform: translateY(-2px);
	}

	.video-thumbnail {
		position: relative;
		width: 100%;
		padding-top: 56.25%;
		background: linear-gradient(135deg, #0984ae 0%, #076787 100%);
	}

	.play-overlay {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #fff;
		opacity: 0.8;
		transition: opacity 0.15s;
	}

	.video-card:hover .play-overlay {
		opacity: 1;
	}

	.video-duration {
		position: absolute;
		bottom: 8px;
		right: 8px;
		display: flex;
		align-items: center;
		gap: 4px;
		padding: 4px 8px;
		background: rgba(0, 0, 0, 0.8);
		color: #fff;
		border-radius: 4px;
		font-size: 12px;
		font-weight: 500;
	}

	.video-info {
		padding: 16px;
	}

	.video-title {
		font-size: 15px;
		font-weight: 600;
		color: #333;
		margin: 0;
		line-height: 1.4;
	}

	@media (max-width: 768px) {
		.learning-center {
			padding: 20px;
		}

		.videos-grid {
			grid-template-columns: 1fr;
		}
	}
</style>

<script lang="ts">
	/**
	 * Day Trading Room - Meet the Traders
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Profiles of expert traders
	 *
	 * @version 1.0.0
	 */
	import IconArrowLeft from '@tabler/icons-svelte/icons/arrow-left';
	import IconUsers from '@tabler/icons-svelte/icons/users';
	import IconBrandTwitter from '@tabler/icons-svelte/icons/brand-twitter';
	import IconMail from '@tabler/icons-svelte/icons/mail';

	// Mock trader data
	const traders = [
		{
			id: 1,
			name: 'Henry Gambell',
			nickname: 'HG',
			title: 'Lead Day Trader',
			bio: 'With over 15 years of trading experience, HG specializes in momentum trading and technical analysis. He leads the Day Trading Room and provides daily market analysis.',
			image: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			specialties: ['Momentum Trading', 'Technical Analysis', 'Options']
		},
		{
			id: 2,
			name: 'David Shay',
			nickname: 'DShay',
			title: 'Senior Options Trader',
			bio: 'David brings a wealth of knowledge in options strategies and risk management. His methodical approach helps traders understand complex options concepts.',
			image: 'https://cdn.simplertrading.com/2025/05/07134911/SimplerCentral_DShay.jpg',
			specialties: ['Options Strategies', 'Risk Management', 'Swing Trading']
		},
		{
			id: 3,
			name: 'Sam Seiden',
			nickname: 'SS',
			title: 'Supply & Demand Expert',
			bio: 'Sam is known for his expertise in supply and demand zones. He teaches traders how to identify high-probability setups using institutional trading concepts.',
			image: 'https://cdn.simplertrading.com/2025/05/07134553/SimplerCentral_SS.jpg',
			specialties: ['Supply & Demand', 'Price Action', 'Futures Trading']
		}
	];
</script>

<svelte:head>
	<title>Meet the Traders - Day Trading Room | Revolution Trading Pros</title>
</svelte:head>

<div class="traders">
	<a href="/dashboard/day-trading-room" class="back-link">
		<IconArrowLeft size={18} />
		Back to Day Trading Room
	</a>

	<header class="page-header">
		<div class="header-icon">
			<IconUsers size={32} />
		</div>
		<div class="header-content">
			<h1 class="page-title">Meet the Traders</h1>
			<p class="page-subtitle">Learn from our team of expert traders with decades of combined experience</p>
		</div>
	</header>

	<div class="traders-grid">
		{#each traders as trader}
			<article class="trader-card">
				<div class="trader-image" style="background-image: url({trader.image})"></div>
				<div class="trader-info">
					<h3 class="trader-name">{trader.name}</h3>
					<p class="trader-title">{trader.title}</p>
					<p class="trader-bio">{trader.bio}</p>
					<div class="trader-specialties">
						{#each trader.specialties as specialty}
							<span class="specialty-tag">{specialty}</span>
						{/each}
					</div>
					<div class="trader-actions">
						<a href="/traders/{trader.id}" class="view-profile-btn">View Profile</a>
						<div class="social-links">
							<button type="button" class="social-link" aria-label="Twitter">
								<IconBrandTwitter size={18} />
							</button>
							<button type="button" class="social-link" aria-label="Email">
								<IconMail size={18} />
							</button>
						</div>
					</div>
				</div>
			</article>
		{/each}
	</div>
</div>

<style>
	.traders {
		padding: 30px;
		max-width: 1400px;
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

	.traders-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(380px, 1fr));
		gap: 24px;
	}

	.trader-card {
		background: #fff;
		border-radius: 12px;
		overflow: hidden;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
		transition: all 0.2s;
	}

	.trader-card:hover {
		box-shadow: 0 4px 16px rgba(0, 0, 0, 0.12);
		transform: translateY(-2px);
	}

	.trader-image {
		width: 100%;
		height: 200px;
		background-size: cover;
		background-position: center top;
		background-color: #0984ae;
	}

	.trader-info {
		padding: 24px;
	}

	.trader-name {
		font-size: 22px;
		font-weight: 700;
		color: #333;
		margin: 0 0 4px;
		font-family: 'Open Sans', sans-serif;
	}

	.trader-title {
		font-size: 14px;
		color: #0984ae;
		margin: 0 0 16px;
		font-weight: 600;
	}

	.trader-bio {
		font-size: 14px;
		color: #666;
		line-height: 1.6;
		margin: 0 0 16px;
	}

	.trader-specialties {
		display: flex;
		flex-wrap: wrap;
		gap: 8px;
		margin-bottom: 20px;
	}

	.specialty-tag {
		display: inline-block;
		padding: 4px 12px;
		background: #f0f7fa;
		color: #0984ae;
		border-radius: 20px;
		font-size: 12px;
		font-weight: 600;
	}

	.trader-actions {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.view-profile-btn {
		display: inline-block;
		padding: 10px 20px;
		background: #0984ae;
		color: #fff;
		text-decoration: none;
		border-radius: 5px;
		font-size: 14px;
		font-weight: 600;
		transition: background 0.2s;
	}

	.view-profile-btn:hover {
		background: #076787;
	}

	.social-links {
		display: flex;
		gap: 12px;
	}

	.social-link {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		background: #f5f5f5;
		color: #666;
		border-radius: 50%;
		transition: all 0.2s;
	}

	.social-link:hover {
		background: #0984ae;
		color: #fff;
	}

	@media (max-width: 768px) {
		.traders {
			padding: 20px;
		}

		.traders-grid {
			grid-template-columns: 1fr;
		}
	}
</style>

<script lang="ts">
	/**
	 * Member Dashboard Page
	 * Reference: frontend/Do's/wwclickememberdashboard
	 */
	const membershipRooms = [
		{
			id: 'mastering-the-trade',
			name: 'Mastering the Trade',
			description: 'Comprehensive options and stock trading education.',
			thumbnail: 'https://cdn.simplertrading.com/2025/09/29170752/MTT-TG.jpg',
			features: ['Daily Videos', 'Learning Center', 'Trading Room', 'Alerts']
		},
		{
			id: 'day-trading-room',
			name: 'Day Trading Room',
			description: 'Live day trading sessions and strategies.',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134553/SimplerCentral_SS.jpg',
			features: ['Live Trading', 'Daily Recap', 'Trade Alerts', 'Education']
		},
		{
			id: 'options-day-trading',
			name: 'Options Day Trading',
			description: 'Options-focused day trading strategies.',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134911/SimplerCentral_DShay.jpg',
			features: ['Options Training', 'Live Sessions', 'Trade Ideas', 'Mentorship']
		}
	];

	const quickLinks = [
		{ name: 'My Indicators', href: '/dashboard/indicators', icon: 'chart' },
		{ name: 'My Classes', href: '/dashboard/classes', icon: 'graduation' },
		{ name: 'My Courses', href: '/dashboard/courses', icon: 'book' },
		{ name: 'Account Settings', href: '/dashboard/account', icon: 'settings' }
	];

	const recentActivity = [
		{ type: 'video', title: 'Signal & Noise', trader: 'Sam', date: 'December 26, 2025' },
		{ type: 'alert', title: 'TSLA Trade Alert', trader: 'TG', date: 'December 26, 2025' },
		{ type: 'video', title: 'A Strong End Of Year', trader: 'TG', date: 'December 24, 2025' },
		{ type: 'course', title: 'Options Fundamentals', trader: 'John Carter', date: 'December 23, 2025' }
	];
</script>

<svelte:head>
	<title>Member Dashboard | Revolution Trading Pros</title>
</svelte:head>

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">Member Dashboard</h1>
		<p class="dashboard__subtitle">Welcome back! Here's your trading overview.</p>
	</div>
</header>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<!-- Quick Links Section -->
		<section class="dashboard__content-section quick-links-section">
			<h2 class="section-title">Quick Links</h2>
			<div class="quick-links-grid">
				{#each quickLinks as link (link.name)}
					<a href={link.href} class="quick-link-card">
						<div class="quick-link-icon">
							{#if link.icon === 'chart'}
								<svg viewBox="0 0 24 24" fill="currentColor" width="32" height="32">
									<path d="M3 13h2v-2H3v2zm0 4h2v-2H3v2zm0-8h2V7H3v2zm4 4h14v-2H7v2zm0 4h14v-2H7v2zM7 7v2h14V7H7z" />
								</svg>
							{:else if link.icon === 'graduation'}
								<svg viewBox="0 0 24 24" fill="currentColor" width="32" height="32">
									<path d="M5 13.18v4L12 21l7-3.82v-4L12 17l-7-3.82zM12 3L1 9l11 6 9-4.91V17h2V9L12 3z" />
								</svg>
							{:else if link.icon === 'book'}
								<svg viewBox="0 0 24 24" fill="currentColor" width="32" height="32">
									<path d="M18 2H6c-1.1 0-2 .9-2 2v16c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zM6 4h5v8l-2.5-1.5L6 12V4z" />
								</svg>
							{:else}
								<svg viewBox="0 0 24 24" fill="currentColor" width="32" height="32">
									<path d="M19.14 12.94c.04-.31.06-.63.06-.94 0-.31-.02-.63-.06-.94l2.03-1.58c.18-.14.23-.41.12-.61l-1.92-3.32c-.12-.22-.37-.29-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94l-.36-2.54c-.04-.24-.24-.41-.48-.41h-3.84c-.24 0-.43.17-.47.41l-.36 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96c-.22-.08-.47 0-.59.22L2.74 8.87c-.12.21-.08.47.12.61l2.03 1.58c-.04.31-.06.63-.06.94s.02.63.06.94l-2.03 1.58c-.18.14-.23.41-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.36 2.54c.05.24.24.41.48.41h3.84c.24 0 .44-.17.47-.41l.36-2.54c.59-.24 1.13-.56 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58zM12 15.6c-1.98 0-3.6-1.62-3.6-3.6s1.62-3.6 3.6-3.6 3.6 1.62 3.6 3.6-1.62 3.6-3.6 3.6z" />
								</svg>
							{/if}
						</div>
						<span class="quick-link-name">{link.name}</span>
					</a>
				{/each}
			</div>
		</section>

		<!-- My Memberships Section -->
		<section class="dashboard__content-section">
			<h2 class="section-title">My Memberships</h2>
			<div class="memberships-grid">
				{#each membershipRooms as room (room.id)}
					<article class="membership-card">
						<div class="membership-thumbnail" style="background-image: url({room.thumbnail});">
							<div class="membership-overlay"></div>
						</div>
						<div class="membership-content">
							<h3 class="membership-title">{room.name}</h3>
							<p class="membership-description">{room.description}</p>
							<div class="membership-features">
								{#each room.features as feature}
									<span class="feature-tag">{feature}</span>
								{/each}
							</div>
							<a href="/dashboard/{room.id}" class="btn btn-primary">Enter Room</a>
						</div>
					</article>
				{/each}
			</div>
		</section>

		<!-- Recent Activity Section -->
		<section class="dashboard__content-section">
			<h2 class="section-title">Recent Activity</h2>
			<div class="activity-list">
				{#each recentActivity as activity (activity.title)}
					<div class="activity-item">
						<div class="activity-icon">
							{#if activity.type === 'video'}
								<svg viewBox="0 0 24 24" fill="currentColor" width="20" height="20">
									<path d="M8 5v14l11-7z" />
								</svg>
							{:else if activity.type === 'alert'}
								<svg viewBox="0 0 24 24" fill="currentColor" width="20" height="20">
									<path d="M12 22c1.1 0 2-.9 2-2h-4c0 1.1.89 2 2 2zm6-6v-5c0-3.07-1.64-5.64-4.5-6.32V4c0-.83-.67-1.5-1.5-1.5s-1.5.67-1.5 1.5v.68C7.63 5.36 6 7.92 6 11v5l-2 2v1h16v-1l-2-2z" />
								</svg>
							{:else}
								<svg viewBox="0 0 24 24" fill="currentColor" width="20" height="20">
									<path d="M18 2H6c-1.1 0-2 .9-2 2v16c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zM6 4h5v8l-2.5-1.5L6 12V4z" />
								</svg>
							{/if}
						</div>
						<div class="activity-info">
							<span class="activity-title">{activity.title}</span>
							<span class="activity-meta">with {activity.trader} &bull; {activity.date}</span>
						</div>
					</div>
				{/each}
			</div>
		</section>
	</div>
</div>

<style>
	.dashboard__header {
		background-color: #fff;
		border-bottom: 1px solid #dbdbdb;
		padding: 30px 40px;
	}

	h1.dashboard__page-title {
		margin: 0;
		color: #333;
		font-size: 36px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
	}

	.dashboard__subtitle {
		margin: 10px 0 0;
		color: #666;
		font-size: 16px;
	}

	.dashboard__content {
		display: flex;
	}

	.dashboard__content-main {
		flex: 1;
		background-color: #efefef;
	}

	.dashboard__content-section {
		padding: 30px 40px;
		background-color: #fff;
		margin-bottom: 20px;
	}

	.section-title {
		color: #333;
		font-size: 24px;
		font-weight: 700;
		margin: 0 0 25px;
		font-family: 'Open Sans', sans-serif;
	}

	/* Quick Links */
	.quick-links-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
		gap: 20px;
	}

	.quick-link-card {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 25px 20px;
		background: #f8f9fa;
		border-radius: 8px;
		text-decoration: none;
		transition: all 0.2s;
		border: 1px solid #e9ecef;
	}

	.quick-link-card:hover {
		background: #e9ecef;
		transform: translateY(-2px);
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.15);
	}

	.quick-link-icon {
		color: #0984ae;
		margin-bottom: 12px;
	}

	.quick-link-name {
		color: #333;
		font-size: 14px;
		font-weight: 600;
	}

	/* Memberships Grid */
	.memberships-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
		gap: 25px;
	}

	.membership-card {
		background: #fff;
		border-radius: 8px;
		overflow: hidden;
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1);
		transition: all 0.2s;
	}

	.membership-card:hover {
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.15);
		transform: translateY(-3px);
	}

	.membership-thumbnail {
		height: 150px;
		background-size: cover;
		background-position: center;
		position: relative;
	}

	.membership-overlay {
		position: absolute;
		inset: 0;
		background: linear-gradient(to bottom, transparent 0%, rgba(0, 0, 0, 0.3) 100%);
	}

	.membership-content {
		padding: 20px;
	}

	.membership-title {
		margin: 0 0 10px;
		font-size: 20px;
		font-weight: 700;
		color: #333;
	}

	.membership-description {
		color: #666;
		font-size: 14px;
		margin: 0 0 15px;
		line-height: 1.5;
	}

	.membership-features {
		display: flex;
		flex-wrap: wrap;
		gap: 8px;
		margin-bottom: 20px;
	}

	.feature-tag {
		background: #e3f2fd;
		color: #0984ae;
		padding: 4px 10px;
		border-radius: 20px;
		font-size: 12px;
		font-weight: 500;
	}

	.btn {
		display: inline-block;
		padding: 10px 20px;
		border-radius: 4px;
		text-decoration: none;
		font-size: 14px;
		font-weight: 600;
		transition: all 0.2s;
	}

	.btn-primary {
		background: #0984ae;
		color: #fff;
		border: none;
	}

	.btn-primary:hover {
		background: #077494;
	}

	/* Activity List */
	.activity-list {
		display: flex;
		flex-direction: column;
		gap: 15px;
	}

	.activity-item {
		display: flex;
		align-items: center;
		padding: 15px;
		background: #f8f9fa;
		border-radius: 8px;
		border: 1px solid #e9ecef;
	}

	.activity-icon {
		width: 40px;
		height: 40px;
		background: #e3f2fd;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #0984ae;
		margin-right: 15px;
	}

	.activity-info {
		display: flex;
		flex-direction: column;
	}

	.activity-title {
		font-weight: 600;
		color: #333;
		font-size: 14px;
	}

	.activity-meta {
		color: #999;
		font-size: 12px;
		margin-top: 3px;
	}
</style>

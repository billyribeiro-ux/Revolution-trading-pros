<script lang="ts">
	/**
	 * Member Dashboard Page
	 * Reference: frontend/Do's/wwclickememberdashboard
	 * Updated: Svelte 5 runes + GSAP 3.14+ (Dec 2025)
	 */
	import { browser } from '$app/environment';

	// Svelte 5 reactive state
	let sectionsContainer = $state<HTMLElement | null>(null);
	let gsapContext = $state<any>(null);
	let isAnimated = $state(false);

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

	// GSAP ScrollTrigger animations - Svelte 5 $effect with GSAP 3.14+ syntax
	$effect(() => {
		if (!browser || !sectionsContainer || isAnimated) return;

		// Dynamic import for SSR safety (GSAP 3.14+ best practice)
		const initAnimations = async () => {
			const gsapModule = await import('gsap');
			const { ScrollTrigger } = await import('gsap/ScrollTrigger');

			const gsap = gsapModule.gsap || gsapModule.default;
			gsap.registerPlugin(ScrollTrigger);

			// GSAP 3.14+ context for automatic cleanup
			gsapContext = gsap.context(() => {
				// Animate sections on scroll
				const sections = sectionsContainer?.querySelectorAll('.dashboard__content-section');

				sections?.forEach((section, index) => {
					gsap.fromTo(
						section,
						{
							opacity: 0,
							y: 30
						},
						{
							opacity: 1,
							y: 0,
							duration: 0.6,
							ease: 'power2.out',
							scrollTrigger: {
								trigger: section,
								start: 'top 85%',
								toggleActions: 'play none none none'
							},
							delay: index * 0.1
						}
					);

					// Stagger children elements within each section
					const cards = section.querySelectorAll('.membership-card, .quick-link-card, .activity-item');
					if (cards.length) {
						gsap.fromTo(
							cards,
							{
								opacity: 0,
								y: 20,
								scale: 0.98
							},
							{
								opacity: 1,
								y: 0,
								scale: 1,
								duration: 0.5,
								ease: 'power2.out',
								stagger: 0.08,
								scrollTrigger: {
									trigger: section,
									start: 'top 80%',
									toggleActions: 'play none none none'
								}
							}
						);
					}
				});
			}, sectionsContainer);

			isAnimated = true;
		};

		initAnimations();

		// Cleanup function (Svelte 5 $effect cleanup pattern)
		return () => {
			if (gsapContext) {
				gsapContext.revert();
			}
		};
	});
</script>

<svelte:head>
	<title>Member Dashboard | Revolution Trading Pros</title>
	<link rel="preconnect" href="https://fonts.googleapis.com" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
	<link href="https://fonts.googleapis.com/css2?family=Open+Sans+Condensed:wght@700&family=Open+Sans:wght@400;600;700;800&display=swap" rel="stylesheet" />
</svelte:head>

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">Member Dashboard</h1>
		<p class="dashboard__subtitle">Welcome back! Here's your trading overview.</p>
	</div>
	<div class="dashboard__header-right">
		<a href="/dashboard" class="btn btn-primary">Enter a Trading Room</a>
	</div>
</header>

<div class="dashboard__content">
	<div class="dashboard__content-main" bind:this={sectionsContainer}>
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
							<div class="membership-actions">
								<a href="/dashboard/{room.id}" class="btn btn-primary">Dashboard</a>
								<a href="/dashboard/{room.id}/trading-room" class="btn btn-secondary" target="_blank">Trading Room</a>
							</div>
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
	/* ==========================================================================
	   HEADER - Reference: justify-content: space-between
	   ========================================================================== */
	.dashboard__header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		background-color: #fff;
		border-bottom: 1px solid #dbdbdb;
		padding: 30px 40px;
	}

	.dashboard__header-left {
		flex: 1;
	}

	.dashboard__header-right {
		flex-shrink: 0;
	}

	/* H1 - Reference: Open Sans Condensed, 42px, bold */
	h1.dashboard__page-title {
		margin: 0;
		color: #333;
		font-size: 42px;
		font-weight: 700;
		font-family: 'Open Sans Condensed', sans-serif;
		line-height: 1.1em;
	}

	.dashboard__subtitle {
		margin: 10px 0 0;
		color: #666;
		font-size: 16px;
		font-family: 'Open Sans', sans-serif;
	}

	/* ==========================================================================
	   CONTENT LAYOUT
	   ========================================================================== */
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
		/* Initial state for GSAP animation */
		opacity: 0;
	}

	/* Section Title - Reference: 32px */
	.section-title {
		color: #333;
		font-size: 32px;
		font-weight: 700;
		margin: 0 0 25px;
		font-family: 'Open Sans', sans-serif;
	}

	/* ==========================================================================
	   BUTTONS - Reference: #F69532, uppercase, 800 weight, letter-spacing
	   ========================================================================== */
	.btn {
		display: inline-block;
		padding: 10px 20px;
		border-radius: 4px;
		text-decoration: none;
		font-size: 14px;
		font-weight: 800;
		text-transform: uppercase;
		letter-spacing: 1.125px;
		transition: all 0.2s ease-in-out;
		box-shadow: 0 2px 5px rgba(0, 0, 0, 0.16);
	}

	.btn-primary {
		background-color: #F69532;
		color: #fff;
		border: none;
	}

	.btn-primary:hover {
		background-color: #dc7309;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
	}

	.btn-secondary {
		background-color: #dc2626;
		color: #fff;
		border: none;
	}

	.btn-secondary:hover {
		background-color: #b91c1c;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
	}

	/* ==========================================================================
	   QUICK LINKS
	   ========================================================================== */
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
		border-radius: 4px;
		text-decoration: none;
		transition: all 0.2s ease-in-out;
		border: 1px solid #e9ecef;
		box-shadow: 0 2px 5px rgba(0, 0, 0, 0.08);
	}

	.quick-link-card:hover {
		background: #e9ecef;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.12);
	}

	.quick-link-icon {
		color: #0984ae;
		margin-bottom: 12px;
	}

	.quick-link-name {
		color: #333;
		font-size: 14px;
		font-weight: 600;
		font-family: 'Open Sans', sans-serif;
	}

	/* ==========================================================================
	   MEMBERSHIP CARDS - Reference: subtle shadow, proper transitions
	   ========================================================================== */
	.memberships-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
		gap: 25px;
	}

	.membership-card {
		background: #fff;
		border-radius: 4px;
		overflow: hidden;
		box-shadow: 0 2px 5px rgba(0, 0, 0, 0.16);
		transition: all 0.2s ease-in-out;
	}

	.membership-card:hover {
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
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
		font-family: 'Open Sans', sans-serif;
	}

	.membership-description {
		color: #666;
		font-size: 14px;
		margin: 0 0 15px;
		line-height: 1.5;
		font-family: 'Open Sans', sans-serif;
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
		font-family: 'Open Sans', sans-serif;
	}

	/* Membership Actions - Two buttons like reference */
	.membership-actions {
		display: flex;
		gap: 10px;
	}

	.membership-actions .btn {
		flex: 1;
		text-align: center;
		padding: 12px 16px;
	}

	/* ==========================================================================
	   ACTIVITY LIST
	   ========================================================================== */
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
		border-radius: 4px;
		border: 1px solid #e9ecef;
		transition: all 0.2s ease-in-out;
	}

	.activity-item:hover {
		background: #f0f0f0;
		box-shadow: 0 2px 5px rgba(0, 0, 0, 0.08);
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
		flex-shrink: 0;
	}

	.activity-info {
		display: flex;
		flex-direction: column;
	}

	.activity-title {
		font-weight: 600;
		color: #333;
		font-size: 14px;
		font-family: 'Open Sans', sans-serif;
	}

	.activity-meta {
		color: #999;
		font-size: 12px;
		margin-top: 3px;
		font-family: 'Open Sans', sans-serif;
	}

	/* ==========================================================================
	   RESPONSIVE - Reference breakpoints
	   ========================================================================== */
	@media (max-width: 768px) {
		.dashboard__header {
			flex-direction: column;
			align-items: flex-start;
			gap: 20px;
			padding: 20px;
		}

		.dashboard__header-right {
			width: 100%;
		}

		.dashboard__header-right .btn {
			width: 100%;
			text-align: center;
		}

		h1.dashboard__page-title {
			font-size: 30px;
		}

		.section-title {
			font-size: 25px;
		}

		.dashboard__content-section {
			padding: 20px;
		}

		.memberships-grid {
			grid-template-columns: 1fr;
		}

		.membership-actions {
			flex-direction: column;
		}
	}

	/* ==========================================================================
	   REDUCED MOTION - Accessibility
	   ========================================================================== */
	@media (prefers-reduced-motion: reduce) {
		.dashboard__content-section {
			opacity: 1;
		}

		.btn,
		.quick-link-card,
		.membership-card,
		.activity-item {
			transition: none;
		}
	}
</style>

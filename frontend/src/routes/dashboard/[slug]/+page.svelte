<script lang="ts">
	/**
	 * Membership Dashboard Page - EXACT Match to Screenshot
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Layout:
	 * - Header with title, "New? Start Here", "Enter a Trading Room" button
	 * - Horizontal secondary nav tabs
	 * - Main content with Quick Access, Recent Videos, Resources
	 * - Right sidebar with Schedule and Quick Links
	 *
	 * @version 3.0.0 (December 2025)
	 */

	import { page } from '$app/stores';

	// Tabler Icons
	import IconHome from '@tabler/icons-svelte/icons/home';
	import IconVideo from '@tabler/icons-svelte/icons/video';
	import IconBook from '@tabler/icons-svelte/icons/book';
	import IconArchive from '@tabler/icons-svelte/icons/archive';
	import IconExternalLink from '@tabler/icons-svelte/icons/external-link';
	import IconPlayerPlay from '@tabler/icons-svelte/icons/player-play';
	import IconDownload from '@tabler/icons-svelte/icons/download';
	import IconMessage from '@tabler/icons-svelte/icons/message';
	import IconChevronRight from '@tabler/icons-svelte/icons/chevron-right';
	import IconCalendar from '@tabler/icons-svelte/icons/calendar';

	// ═══════════════════════════════════════════════════════════════════════════
	// ROUTE PARAMS
	// ═══════════════════════════════════════════════════════════════════════════

	const slug = $derived($page.params.slug);

	// ═══════════════════════════════════════════════════════════════════════════
	// MEMBERSHIP DATA
	// ═══════════════════════════════════════════════════════════════════════════

	interface MembershipDetail {
		name: string;
		slug: string;
		tradingRoomUrl: string;
		description: string;
	}

	const membershipData = $derived.by((): MembershipDetail => {
		const memberships: Record<string, MembershipDetail> = {
			'mastering-the-trade': {
				name: 'Mastering the Trade',
				slug: 'mastering-the-trade',
				tradingRoomUrl: 'https://chat.protradingroom.com',
				description: 'Live trading room with professional traders sharing real-time analysis and trade ideas.'
			},
			'simpler-showcase': {
				name: 'Simpler Showcase',
				slug: 'simpler-showcase',
				tradingRoomUrl: 'https://chat.protradingroom.com/simpler-showcase',
				description: 'Watch our traders showcase their strategies and trade setups in real-time.'
			}
		};

		return memberships[slug] || {
			name: slug.split('-').map(w => w.charAt(0).toUpperCase() + w.slice(1)).join(' '),
			slug,
			tradingRoomUrl: `/trading-room/${slug}`,
			description: 'Access your membership dashboard and resources.'
		};
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// SECONDARY NAV TABS
	// ═══════════════════════════════════════════════════════════════════════════

	const currentPath = $derived($page.url.pathname);

	const navTabs = $derived([
		{
			href: `/dashboard/${slug}`,
			label: `${membershipData.name} Dashboard`,
			icon: IconHome,
			isActive: currentPath === `/dashboard/${slug}`
		},
		{
			href: `/dashboard/${slug}/daily-videos`,
			label: 'Premium Daily Videos',
			icon: IconVideo,
			isActive: currentPath.includes('/daily-videos')
		},
		{
			href: `/dashboard/${slug}/learning-center`,
			label: 'Learning Center',
			icon: IconBook,
			isActive: currentPath.includes('/learning-center')
		},
		{
			href: `/dashboard/${slug}/trading-room-archive`,
			label: 'Trading Room Archives',
			icon: IconArchive,
			isActive: currentPath.includes('/trading-room-archive')
		},
		{
			href: membershipData.tradingRoomUrl,
			label: 'Trading Room',
			icon: IconExternalLink,
			isExternal: true,
			isActive: false
		}
	]);

	// ═══════════════════════════════════════════════════════════════════════════
	// QUICK ACCESS CARDS
	// ═══════════════════════════════════════════════════════════════════════════

	const quickAccessCards = $derived([
		{
			title: 'Enter Trading Room',
			subtitle: 'Join the live room',
			href: membershipData.tradingRoomUrl,
			icon: IconPlayerPlay,
			color: 'teal',
			isExternal: true
		},
		{
			title: 'Learning Center',
			subtitle: 'Courses & tutorials',
			href: `/dashboard/${slug}/learning-center`,
			icon: IconBook,
			color: 'default'
		},
		{
			title: 'Video Archive',
			subtitle: 'Past recordings',
			href: `/dashboard/${slug}/trading-room-archive`,
			icon: IconArchive,
			color: 'default'
		},
		{
			title: 'Community',
			subtitle: 'Join Discord',
			href: 'https://discord.gg/simplertrading',
			icon: IconMessage,
			color: 'default',
			isExternal: true
		}
	]);

	// ═══════════════════════════════════════════════════════════════════════════
	// RECENT VIDEOS
	// ═══════════════════════════════════════════════════════════════════════════

	const recentVideos = [
		{
			id: 1,
			title: 'Mastering the Trade - Morning Analysis',
			date: 'Dec 6, 2025',
			duration: '1:23:45',
			thumbnail: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg'
		},
		{
			id: 2,
			title: 'Mastering the Trade - Trade Breakdown',
			date: 'Dec 5, 2025',
			duration: '45:30',
			thumbnail: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg'
		},
		{
			id: 3,
			title: 'Mastering the Trade - Weekly Recap',
			date: 'Dec 4, 2025',
			duration: '58:12',
			thumbnail: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg'
		}
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// RESOURCES
	// ═══════════════════════════════════════════════════════════════════════════

	const resources = [
		{
			title: 'Trading Room Rules',
			href: 'https://cdn.simplertrading.com/2024/02/07192341/Simpler-Tradings-Rules-of-the-Room.pdf',
			icon: IconBook
		},
		{
			title: 'Getting Started Guide',
			href: `/dashboard/${slug}/getting-started`,
			icon: IconChevronRight
		},
		{
			title: 'Platform Setup',
			href: '/tutorials/platform-setup',
			icon: IconDownload
		},
		{
			title: 'Community Discord',
			href: 'https://discord.gg/simplertrading',
			icon: IconMessage
		}
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// TRADING ROOM SCHEDULE
	// ═══════════════════════════════════════════════════════════════════════════

	const schedule = [
		{ traderName: 'Taylor Horton', dateTime: 'Dec 22, 2025, 9:20 AM EST' },
		{ traderName: 'Sam Shames', dateTime: 'Dec 22, 2025, 10:30 AM EST' },
		{ traderName: 'Neil Yeager', dateTime: 'Dec 22, 2025, 11:30 AM EST' },
		{ traderName: 'Bruce Marshall', dateTime: 'Dec 22, 2025, 2:00 PM EST' },
		{ traderName: 'Henry Gambell', dateTime: 'Dec 22, 2025, 3:00 PM EST' },
		{ traderName: 'Henry Gambell', dateTime: 'Dec 23, 2025, 9:15 AM EST' },
		{ traderName: 'Raghee Horner', dateTime: 'Dec 23, 2025, 10:30 AM EST' },
		{ traderName: 'David Starr', dateTime: 'Dec 23, 2025, 11:30 AM EST' }
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// QUICK LINKS
	// ═══════════════════════════════════════════════════════════════════════════

	const quickLinks = [
		{ title: 'Support', href: 'https://intercom.help/simpler-trading/en/' },
		{ title: 'Platform Tutorials', href: '/tutorials' },
		{ title: 'Simpler Blog', href: '/blog' }
	];
</script>

<svelte:head>
	<title>{membershipData.name} Dashboard | Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<!-- ═══════════════════════════════════════════════════════════════════════════
     HEADER
     ═══════════════════════════════════════════════════════════════════════════ -->

<header class="membership-header">
	<div class="header-left">
		<h1 class="page-title">{membershipData.name} Dashboard</h1>
	</div>
	<div class="header-right">
		<a href="/dashboard/{slug}/getting-started" class="new-link">New? Start Here</a>
		<a href={membershipData.tradingRoomUrl} class="btn-trading-room" target="_blank" rel="nofollow">
			Enter a Trading Room
		</a>
	</div>
</header>

<div class="trading-rules-bar">
	<a href="https://cdn.simplertrading.com/2024/02/07192341/Simpler-Tradings-Rules-of-the-Room.pdf" target="_blank" class="trading-rules-link">
		Trading Room Rules
	</a>
	<span class="trading-rules-text">
		By logging into any of our Live Trading Rooms, You are agreeing to our Rules of the Room.
	</span>
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════
     SECONDARY NAV TABS (Horizontal)
     ═══════════════════════════════════════════════════════════════════════════ -->

<nav class="secondary-nav-tabs">
	{#each navTabs as tab (tab.href)}
		<a
			href={tab.href}
			class="nav-tab"
			class:is-active={tab.isActive}
			target={tab.isExternal ? '_blank' : undefined}
			rel={tab.isExternal ? 'nofollow' : undefined}
		>
			<svelte:component this={tab.icon} size={18} />
			<span>{tab.label}</span>
			{#if tab.isExternal}
				<IconExternalLink size={14} class="external-icon" />
			{/if}
		</a>
	{/each}
</nav>

<!-- ═══════════════════════════════════════════════════════════════════════════
     MAIN CONTENT WITH SIDEBAR
     ═══════════════════════════════════════════════════════════════════════════ -->

<div class="content-wrapper">
	<main class="main-content">
		<!-- Description -->
		<p class="membership-description">{membershipData.description}</p>

		<!-- Quick Access -->
		<section class="section">
			<h2 class="section-title">Quick Access</h2>
			<div class="quick-access-grid">
				{#each quickAccessCards as card (card.title)}
					<a
						href={card.href}
						class="quick-access-card"
						class:is-teal={card.color === 'teal'}
						target={card.isExternal ? '_blank' : undefined}
						rel={card.isExternal ? 'nofollow' : undefined}
					>
						<div class="card-icon">
							<svelte:component this={card.icon} size={32} />
						</div>
						<div class="card-content">
							<h3 class="card-title">{card.title}</h3>
							<p class="card-subtitle">{card.subtitle}</p>
						</div>
					</a>
				{/each}
			</div>
		</section>

		<!-- Recent Videos -->
		<section class="section">
			<h2 class="section-title">Recent Videos</h2>
			<div class="videos-grid">
				{#each recentVideos as video (video.id)}
					<a href="/dashboard/{slug}/daily-videos/{video.id}" class="video-card">
						<div class="video-thumbnail" style="background-image: url({video.thumbnail});">
							<div class="play-button">
								<IconPlayerPlay size={24} />
							</div>
							<span class="video-duration">{video.duration}</span>
						</div>
						<h4 class="video-title">{video.title}</h4>
						<p class="video-date">
							<IconCalendar size={14} />
							{video.date}
						</p>
					</a>
				{/each}
			</div>
			<a href="/dashboard/{slug}/daily-videos" class="view-all-link">
				View All Videos <IconChevronRight size={16} />
			</a>
		</section>

		<!-- Resources -->
		<section class="section resources-section">
			<h2 class="section-title">Resources</h2>
			<ul class="resources-list">
				{#each resources as resource (resource.title)}
					<li>
						<a href={resource.href} class="resource-link" target={resource.href.startsWith('http') ? '_blank' : undefined}>
							<svelte:component this={resource.icon} size={18} />
							<span>{resource.title}</span>
							<IconChevronRight size={16} class="chevron" />
						</a>
					</li>
				{/each}
			</ul>
		</section>
	</main>

	<!-- Right Sidebar -->
	<aside class="right-sidebar">
		<!-- Schedule -->
		<section class="sidebar-section">
			<h3 class="sidebar-title">{membershipData.name.toUpperCase()} SCHEDULE</h3>
			<p class="schedule-note">Schedule is subject to change.</p>
			<ul class="schedule-list">
				{#each schedule as item (item.traderName + item.dateTime)}
					<li class="schedule-item">
						<a href="/traders/{item.traderName.toLowerCase().replace(/ /g, '-')}" class="trader-name">
							{item.traderName}
						</a>
						<span class="schedule-time">{item.dateTime}</span>
					</li>
				{/each}
			</ul>
		</section>

		<!-- Quick Links -->
		<section class="sidebar-section">
			<h3 class="sidebar-title">QUICK LINKS</h3>
			<ul class="quick-links-list">
				{#each quickLinks as link (link.title)}
					<li>
						<a href={link.href} target="_blank">
							<IconChevronRight size={14} />
							{link.title}
						</a>
					</li>
				{/each}
			</ul>
		</section>
	</aside>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   HEADER
	   ═══════════════════════════════════════════════════════════════════════════ */

	.membership-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 20px 30px;
		background: #fff;
		border-bottom: 1px solid #e5e7eb;
	}

	.page-title {
		font-size: 24px;
		font-weight: 600;
		color: #1f2937;
		margin: 0;
	}

	.header-right {
		display: flex;
		align-items: center;
		gap: 20px;
	}

	.new-link {
		color: #0984ae;
		font-size: 14px;
		font-weight: 500;
		text-decoration: none;
	}

	.new-link:hover {
		text-decoration: underline;
	}

	.btn-trading-room {
		display: inline-block;
		padding: 10px 20px;
		background: #f99e31;
		color: #fff;
		font-size: 14px;
		font-weight: 600;
		text-decoration: none;
		border-radius: 4px;
		transition: background 0.15s ease;
	}

	.btn-trading-room:hover {
		background: #e8890f;
	}

	/* Trading Rules Bar */
	.trading-rules-bar {
		display: flex;
		justify-content: flex-end;
		align-items: center;
		gap: 10px;
		padding: 8px 30px;
		background: #fff;
		border-bottom: 1px solid #e5e7eb;
		font-size: 11px;
	}

	.trading-rules-link {
		color: #0984ae;
		font-weight: 600;
		text-decoration: none;
	}

	.trading-rules-text {
		color: #6b7280;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SECONDARY NAV TABS
	   ═══════════════════════════════════════════════════════════════════════════ */

	.secondary-nav-tabs {
		display: flex;
		gap: 0;
		padding: 0 30px;
		background: #fff;
		border-bottom: 1px solid #e5e7eb;
		overflow-x: auto;
	}

	.nav-tab {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 16px 20px;
		color: #6b7280;
		font-size: 14px;
		font-weight: 500;
		text-decoration: none;
		border-bottom: 2px solid transparent;
		white-space: nowrap;
		transition: all 0.15s ease;
	}

	.nav-tab:hover {
		color: #0984ae;
	}

	.nav-tab.is-active {
		color: #0984ae;
		border-bottom-color: #0984ae;
	}

	.nav-tab :global(.external-icon) {
		opacity: 0.5;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CONTENT WRAPPER
	   ═══════════════════════════════════════════════════════════════════════════ */

	.content-wrapper {
		display: flex;
		gap: 30px;
		padding: 30px;
		background: #f4f4f4;
		min-height: calc(100vh - 200px);
	}

	.main-content {
		flex: 1;
		min-width: 0;
	}

	.membership-description {
		color: #4b5563;
		font-size: 14px;
		line-height: 1.6;
		margin: 0 0 30px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SECTIONS
	   ═══════════════════════════════════════════════════════════════════════════ */

	.section {
		background: #fff;
		border-radius: 8px;
		padding: 24px;
		margin-bottom: 24px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.section-title {
		font-size: 18px;
		font-weight: 600;
		color: #1f2937;
		margin: 0 0 20px;
		padding-bottom: 12px;
		border-bottom: 2px solid #f99e31;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   QUICK ACCESS CARDS
	   ═══════════════════════════════════════════════════════════════════════════ */

	.quick-access-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 16px;
	}

	.quick-access-card {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 24px 16px;
		background: #f9fafb;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		text-decoration: none;
		text-align: center;
		transition: all 0.15s ease;
	}

	.quick-access-card:hover {
		border-color: #0984ae;
		box-shadow: 0 4px 12px rgba(9, 132, 174, 0.15);
	}

	.quick-access-card.is-teal {
		background: linear-gradient(135deg, #0984ae 0%, #076787 100%);
		border-color: #0984ae;
		color: #fff;
	}

	.quick-access-card.is-teal:hover {
		box-shadow: 0 4px 12px rgba(9, 132, 174, 0.4);
	}

	.card-icon {
		margin-bottom: 12px;
		color: #0984ae;
	}

	.quick-access-card.is-teal .card-icon {
		color: #fff;
	}

	.card-title {
		font-size: 14px;
		font-weight: 600;
		color: #1f2937;
		margin: 0 0 4px;
	}

	.quick-access-card.is-teal .card-title {
		color: #fff;
	}

	.card-subtitle {
		font-size: 12px;
		color: #6b7280;
		margin: 0;
	}

	.quick-access-card.is-teal .card-subtitle {
		color: rgba(255, 255, 255, 0.8);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   VIDEOS GRID
	   ═══════════════════════════════════════════════════════════════════════════ */

	.videos-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 20px;
		margin-bottom: 20px;
	}

	.video-card {
		text-decoration: none;
		transition: transform 0.15s ease;
	}

	.video-card:hover {
		transform: translateY(-4px);
	}

	.video-thumbnail {
		position: relative;
		aspect-ratio: 16 / 9;
		background-color: #1f2937;
		background-size: cover;
		background-position: center;
		border-radius: 8px;
		overflow: hidden;
		margin-bottom: 12px;
	}

	.play-button {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 48px;
		height: 48px;
		background: rgba(9, 132, 174, 0.9);
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #fff;
		opacity: 0;
		transition: opacity 0.15s ease;
	}

	.video-card:hover .play-button {
		opacity: 1;
	}

	.video-duration {
		position: absolute;
		bottom: 8px;
		right: 8px;
		background: rgba(0, 0, 0, 0.8);
		color: #fff;
		font-size: 12px;
		font-weight: 500;
		padding: 2px 6px;
		border-radius: 4px;
	}

	.video-title {
		font-size: 14px;
		font-weight: 600;
		color: #1f2937;
		margin: 0 0 6px;
		line-height: 1.4;
	}

	.video-date {
		display: flex;
		align-items: center;
		gap: 6px;
		font-size: 12px;
		color: #6b7280;
		margin: 0;
	}

	.view-all-link {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		color: #0984ae;
		font-size: 14px;
		font-weight: 500;
		text-decoration: none;
	}

	.view-all-link:hover {
		text-decoration: underline;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESOURCES
	   ═══════════════════════════════════════════════════════════════════════════ */

	.resources-list {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.resources-list li {
		border-bottom: 1px solid #e5e7eb;
	}

	.resources-list li:last-child {
		border-bottom: none;
	}

	.resource-link {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 16px 0;
		color: #1f2937;
		font-size: 14px;
		text-decoration: none;
		transition: color 0.15s ease;
	}

	.resource-link:hover {
		color: #0984ae;
	}

	.resource-link .chevron {
		margin-left: auto;
		color: #9ca3af;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RIGHT SIDEBAR
	   ═══════════════════════════════════════════════════════════════════════════ */

	.right-sidebar {
		width: 280px;
		flex-shrink: 0;
	}

	.sidebar-section {
		background: #fff;
		border-radius: 8px;
		padding: 20px;
		margin-bottom: 20px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.sidebar-title {
		font-size: 12px;
		font-weight: 700;
		color: #1f2937;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		margin: 0 0 16px;
	}

	.schedule-note {
		font-size: 11px;
		color: #6b7280;
		margin: 0 0 16px;
	}

	.schedule-list {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.schedule-item {
		margin-bottom: 16px;
	}

	.schedule-item:last-child {
		margin-bottom: 0;
	}

	.trader-name {
		display: block;
		color: #0984ae;
		font-size: 14px;
		font-weight: 500;
		text-decoration: none;
		margin-bottom: 2px;
	}

	.trader-name:hover {
		text-decoration: underline;
	}

	.schedule-time {
		font-size: 12px;
		color: #6b7280;
	}

	.quick-links-list {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.quick-links-list li {
		margin-bottom: 12px;
	}

	.quick-links-list li:last-child {
		margin-bottom: 0;
	}

	.quick-links-list a {
		display: flex;
		align-items: center;
		gap: 6px;
		color: #0984ae;
		font-size: 14px;
		text-decoration: none;
	}

	.quick-links-list a:hover {
		text-decoration: underline;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media screen and (max-width: 1200px) {
		.quick-access-grid {
			grid-template-columns: repeat(2, 1fr);
		}

		.videos-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media screen and (max-width: 1024px) {
		.content-wrapper {
			flex-direction: column;
		}

		.right-sidebar {
			width: 100%;
			display: grid;
			grid-template-columns: repeat(2, 1fr);
			gap: 20px;
		}

		.sidebar-section {
			margin-bottom: 0;
		}
	}

	@media screen and (max-width: 768px) {
		.membership-header {
			flex-direction: column;
			align-items: flex-start;
			gap: 16px;
		}

		.header-right {
			width: 100%;
			justify-content: space-between;
		}

		.quick-access-grid {
			grid-template-columns: 1fr;
		}

		.videos-grid {
			grid-template-columns: 1fr;
		}

		.right-sidebar {
			grid-template-columns: 1fr;
		}

		.secondary-nav-tabs {
			padding: 0 16px;
		}

		.nav-tab {
			padding: 12px 16px;
			font-size: 13px;
		}
	}
</style>

<script lang="ts">
	/**
	 * Individual Membership Dashboard Page - WordPress Simpler Trading Exact
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * This is the page shown when clicking "Dashboard" on a membership card.
	 * URL: /dashboard/mastering-the-trade, /dashboard/moxie, etc.
	 *
	 * WordPress Structure:
	 * - dashboard__header with page title and "Enter Trading Room" button
	 * - dashboard__nav-secondary with subpage links
	 * - dashboard__content with sections for videos, resources, etc.
	 *
	 * @version 1.0.0 (December 2025)
	 */

	import { page } from '$app/stores';
	import { browser } from '$app/environment';
	import {
		IconPlayerPlay,
		IconBook,
		IconInbox,
		IconExternalLink,
		IconChevronRight,
		IconClock,
		IconCalendar,
		IconVideo,
		IconDownload,
		IconMessageCircle
	} from '@tabler/icons-svelte';
	import '$lib/styles/st-icons.css';

	// ═══════════════════════════════════════════════════════════════════════════
	// ROUTE PARAMS
	// ═══════════════════════════════════════════════════════════════════════════

	const slug = $derived($page.params.slug);

	// ═══════════════════════════════════════════════════════════════════════════
	// MEMBERSHIP DATA (Mock - will be fetched from API)
	// ═══════════════════════════════════════════════════════════════════════════

	interface MembershipDetail {
		name: string;
		slug: string;
		type: 'trading-room' | 'course' | 'indicator' | 'alert' | 'mastery';
		description: string;
		icon: string;
		tradingRoomUrl?: string;
		hasLearningCenter: boolean;
		hasArchive: boolean;
		hasDiscord: boolean;
	}

	// Mock membership details based on slug
	const membershipData = $derived.by((): MembershipDetail => {
		const memberships: Record<string, MembershipDetail> = {
			'mastering-the-trade': {
				name: 'Mastering the Trade',
				slug: 'mastering-the-trade',
				type: 'trading-room',
				description: 'Live trading room with professional traders sharing real-time analysis and trade ideas.',
				icon: 'st-icon-mastering-the-trade',
				tradingRoomUrl: '/trading-room/mastering-the-trade',
				hasLearningCenter: true,
				hasArchive: true,
				hasDiscord: true
			},
			'simpler-showcase': {
				name: 'Revolution Showcase',
				slug: 'simpler-showcase',
				type: 'trading-room',
				description: 'Exclusive showcase of trading strategies from our top educators.',
				icon: 'st-icon-simpler-showcase',
				tradingRoomUrl: '/trading-room/simpler-showcase',
				hasLearningCenter: true,
				hasArchive: true,
				hasDiscord: true
			},
			'mm': {
				name: 'Moxie Indicator™ Mastery',
				slug: 'mm',
				type: 'mastery',
				description: 'Master the Moxie Indicator with comprehensive training and live sessions.',
				icon: 'st-icon-moxie',
				tradingRoomUrl: '/trading-room/moxie-mastery',
				hasLearningCenter: true,
				hasArchive: true,
				hasDiscord: false
			},
			'ww': {
				name: 'Weekly Watchlist',
				slug: 'ww',
				type: 'alert',
				description: 'Weekly curated stock watchlist with detailed analysis and entry points.',
				icon: 'st-icon-trade-of-the-week',
				hasLearningCenter: false,
				hasArchive: true,
				hasDiscord: false
			},
			'day-trading': {
				name: 'Day Trading Room',
				slug: 'day-trading',
				type: 'trading-room',
				description: 'Real-time day trading with focus on intraday opportunities.',
				icon: 'st-icon-day-trading',
				tradingRoomUrl: '/trading-room/day-trading',
				hasLearningCenter: true,
				hasArchive: true,
				hasDiscord: true
			},
			'swing-trading': {
				name: 'Swing Trading Room',
				slug: 'swing-trading',
				type: 'trading-room',
				description: 'Multi-day swing trade setups and analysis.',
				icon: 'st-icon-swing-trading',
				tradingRoomUrl: '/trading-room/swing-trading',
				hasLearningCenter: true,
				hasArchive: true,
				hasDiscord: true
			}
		};

		return memberships[slug] || {
			name: slug.split('-').map(w => w.charAt(0).toUpperCase() + w.slice(1)).join(' '),
			slug,
			type: 'trading-room',
			description: 'Access your membership content and resources.',
			icon: 'st-icon-options',
			tradingRoomUrl: `/trading-room/${slug}`,
			hasLearningCenter: true,
			hasArchive: true,
			hasDiscord: false
		};
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// RECENT VIDEOS (Mock data)
	// ═══════════════════════════════════════════════════════════════════════════

	const recentVideos = [
		{
			id: 1,
			title: 'Morning Market Analysis - December 6, 2025',
			duration: '1:23:45',
			date: 'Dec 6, 2025',
			thumbnail: '/images/videos/thumb1.jpg'
		},
		{
			id: 2,
			title: 'SPY Options Trade Breakdown',
			duration: '45:30',
			date: 'Dec 5, 2025',
			thumbnail: '/images/videos/thumb2.jpg'
		},
		{
			id: 3,
			title: 'Weekly Recap & Planning Session',
			duration: '58:12',
			date: 'Dec 4, 2025',
			thumbnail: '/images/videos/thumb3.jpg'
		}
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// QUICK RESOURCES (Mock data)
	// ═══════════════════════════════════════════════════════════════════════════

	const quickResources = [
		{ title: 'Trading Room Rules', icon: IconBook, href: '#rules' },
		{ title: 'Getting Started Guide', icon: IconChevronRight, href: '#getting-started' },
		{ title: 'Platform Setup', icon: IconDownload, href: '#platform' },
		{ title: 'Community Discord', icon: IconMessageCircle, href: '#discord' }
	];
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     HEAD
     ═══════════════════════════════════════════════════════════════════════════ -->

<svelte:head>
	<title>{membershipData.name} | Dashboard | Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<!-- ═══════════════════════════════════════════════════════════════════════════
     DASHBOARD HEADER - WordPress: .dashboard__header
     ═══════════════════════════════════════════════════════════════════════════ -->

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<!-- Breadcrumb -->
		<nav class="dashboard__breadcrumb" aria-label="Breadcrumb">
			<a href="/dashboard">Dashboard</a>
			<span class="separator">/</span>
			<span class="current">{membershipData.name}</span>
		</nav>
		<h1 class="dashboard__page-title">
			<span class="icon icon--lg {membershipData.icon}"></span>
			{membershipData.name}
		</h1>
	</div>
	<div class="dashboard__header-right">
		{#if membershipData.tradingRoomUrl}
			<a href={membershipData.tradingRoomUrl} class="btn btn-orange btn-tradingroom" target="_blank" rel="nofollow">
				<IconPlayerPlay size={18} />
				<strong>Enter Trading Room</strong>
			</a>
		{/if}
	</div>
</header>

<!-- ═══════════════════════════════════════════════════════════════════════════
     SECONDARY NAVIGATION - WordPress: .dashboard__nav-secondary
     ═══════════════════════════════════════════════════════════════════════════ -->

<nav class="dashboard__nav-secondary">
	<ul class="nav-menu">
		<li class="nav-item is-active">
			<a href="/dashboard/{slug}">
				<span class="st-icon-dashboard nav-icon"></span>
				<span class="nav-text">Dashboard</span>
			</a>
		</li>
		{#if membershipData.hasLearningCenter}
			<li class="nav-item">
				<a href="/dashboard/{slug}/learning-center">
					<span class="st-icon-learning-center nav-icon"></span>
					<span class="nav-text">Learning Center</span>
				</a>
			</li>
		{/if}
		{#if membershipData.hasArchive}
			<li class="nav-item">
				<a href="/dashboard/{slug}/archive">
					<span class="st-icon-chatroom-archive nav-icon"></span>
					<span class="nav-text">Video Archive</span>
				</a>
			</li>
		{/if}
		{#if membershipData.tradingRoomUrl}
			<li class="nav-item">
				<a href={membershipData.tradingRoomUrl} target="_blank" rel="nofollow">
					<span class="st-icon-training-room nav-icon"></span>
					<span class="nav-text">Trading Room</span>
					<IconExternalLink size={14} class="external-icon" />
				</a>
			</li>
		{/if}
	</ul>
</nav>

<!-- ═══════════════════════════════════════════════════════════════════════════
     DASHBOARD CONTENT - WordPress: .dashboard__content
     ═══════════════════════════════════════════════════════════════════════════ -->

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<!-- Description Section -->
		<section class="dashboard__content-section">
			<div class="membership-intro">
				<p>{membershipData.description}</p>
			</div>
		</section>

		<!-- Quick Actions Section -->
		<section class="dashboard__content-section">
			<h2 class="section-title">Quick Access</h2>
			<div class="quick-actions row">
				{#if membershipData.tradingRoomUrl}
					<div class="col-sm-6 col-lg-3">
						<a href={membershipData.tradingRoomUrl} class="action-card action-card--primary" target="_blank">
							<div class="action-card__icon">
								<IconPlayerPlay size={28} />
							</div>
							<div class="action-card__content">
								<h3>Enter Trading Room</h3>
								<p>Join the live session</p>
							</div>
						</a>
					</div>
				{/if}
				{#if membershipData.hasLearningCenter}
					<div class="col-sm-6 col-lg-3">
						<a href="/dashboard/{slug}/learning-center" class="action-card">
							<div class="action-card__icon">
								<IconBook size={28} />
							</div>
							<div class="action-card__content">
								<h3>Learning Center</h3>
								<p>Courses & tutorials</p>
							</div>
						</a>
					</div>
				{/if}
				{#if membershipData.hasArchive}
					<div class="col-sm-6 col-lg-3">
						<a href="/dashboard/{slug}/archive" class="action-card">
							<div class="action-card__icon">
								<IconInbox size={28} />
							</div>
							<div class="action-card__content">
								<h3>Video Archive</h3>
								<p>Past recordings</p>
							</div>
						</a>
					</div>
				{/if}
				{#if membershipData.hasDiscord}
					<div class="col-sm-6 col-lg-3">
						<a href="#discord" class="action-card">
							<div class="action-card__icon">
								<IconMessageCircle size={28} />
							</div>
							<div class="action-card__content">
								<h3>Community</h3>
								<p>Join Discord</p>
							</div>
						</a>
					</div>
				{/if}
			</div>
		</section>

		<!-- Recent Videos Section -->
		<section class="dashboard__content-section">
			<h2 class="section-title">Recent Videos</h2>
			<div class="videos-grid row">
				{#each recentVideos as video (video.id)}
					<div class="col-sm-6 col-xl-4">
						<a href="/dashboard/{slug}/archive/{video.id}" class="video-card">
							<div class="video-card__thumbnail">
								<div class="video-card__placeholder">
									<IconVideo size={48} />
								</div>
								<span class="video-card__duration">{video.duration}</span>
								<div class="video-card__play">
									<IconPlayerPlay size={32} />
								</div>
							</div>
							<div class="video-card__content">
								<h3 class="video-card__title">{video.title}</h3>
								<div class="video-card__meta">
									<IconCalendar size={14} />
									{video.date}
								</div>
							</div>
						</a>
					</div>
				{/each}
			</div>
			<div class="section-footer">
				<a href="/dashboard/{slug}/archive" class="btn btn-link">
					View All Videos
					<IconChevronRight size={16} />
				</a>
			</div>
		</section>

		<!-- Resources Section -->
		<section class="dashboard__content-section">
			<h2 class="section-title">Resources</h2>
			<div class="resources-list">
				{#each quickResources as resource}
					<a href={resource.href} class="resource-item">
						<svelte:component this={resource.icon} size={20} />
						<span>{resource.title}</span>
						<IconChevronRight size={16} class="chevron" />
					</a>
				{/each}
			</div>
		</section>
	</div>

	<!-- Sidebar -->
	<aside class="dashboard__content-sidebar">
		<section class="sidebar-section">
			<h3 class="sidebar-title">Membership Status</h3>
			<div class="status-card">
				<div class="status-row">
					<span class="status-label">Status</span>
					<span class="status-value status-active">Active</span>
				</div>
				<div class="status-row">
					<span class="status-label">Member Since</span>
					<span class="status-value">Jan 15, 2025</span>
				</div>
				<div class="status-row">
					<span class="status-label">Next Billing</span>
					<span class="status-value">Feb 15, 2025</span>
				</div>
			</div>
		</section>

		<section class="sidebar-section">
			<h3 class="sidebar-title">Trading Schedule</h3>
			<div class="schedule-list">
				<div class="schedule-item">
					<IconClock size={16} />
					<div>
						<strong>Morning Session</strong>
						<span>9:00 AM - 11:30 AM ET</span>
					</div>
				</div>
				<div class="schedule-item">
					<IconClock size={16} />
					<div>
						<strong>Afternoon Session</strong>
						<span>1:00 PM - 4:00 PM ET</span>
					</div>
				</div>
			</div>
		</section>

		<section class="sidebar-section">
			<h3 class="sidebar-title">Need Help?</h3>
			<a href="/support" class="btn btn-outline btn-block">Contact Support</a>
		</section>
	</aside>
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD HEADER - WordPress: .dashboard__header
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__header {
		background-color: #fff;
		border-bottom: 1px solid var(--st-border-color, #dbdbdb);
		padding: 20px 30px;
		display: flex;
		flex-wrap: wrap;
		justify-content: space-between;
		align-items: center;
		gap: 16px;
	}

	.dashboard__header-left,
	.dashboard__header-right {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.dashboard__header-left {
		flex-direction: column;
		align-items: flex-start;
		gap: 8px;
	}

	.dashboard__breadcrumb {
		font-size: 13px;
		color: var(--st-text-muted, #64748b);
	}

	.dashboard__breadcrumb a {
		color: var(--st-link-color, #1e73be);
		text-decoration: none;
	}

	.dashboard__breadcrumb a:hover {
		text-decoration: underline;
	}

	.dashboard__breadcrumb .separator {
		margin: 0 8px;
		color: #999;
	}

	.dashboard__breadcrumb .current {
		color: var(--st-text-color, #333);
	}

	.dashboard__page-title {
		display: flex;
		align-items: center;
		gap: 12px;
		color: var(--st-text-color, #333);
		font-family: 'Open Sans Condensed', sans-serif;
		font-size: 32px;
		font-weight: 700;
		margin: 0;
		line-height: 1.2;
	}

	.dashboard__page-title .icon {
		font-size: 36px;
		color: var(--st-primary, #0984ae);
	}

	.btn-tradingroom {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		background: var(--st-orange, #f99e31);
		color: #fff;
		font-size: 14px;
		padding: 12px 20px;
		border-radius: 5px;
		text-decoration: none;
		transition: all 0.15s ease-in-out;
		border: none;
		cursor: pointer;
	}

	.btn-tradingroom:hover {
		background: var(--st-orange-hover, #dc7309);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SECONDARY NAVIGATION - WordPress: .dashboard__nav-secondary
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-secondary {
		background: #fff;
		border-bottom: 1px solid var(--st-border-color, #dbdbdb);
		padding: 0 30px;
	}

	.nav-menu {
		display: flex;
		list-style: none;
		margin: 0;
		padding: 0;
		gap: 0;
		overflow-x: auto;
	}

	.nav-item a {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 16px 20px;
		color: var(--st-text-muted, #64748b);
		text-decoration: none;
		font-size: 14px;
		font-weight: 500;
		border-bottom: 3px solid transparent;
		transition: all 0.15s ease-in-out;
		white-space: nowrap;
	}

	.nav-item a:hover {
		color: var(--st-primary, #0984ae);
		background: rgba(9, 132, 174, 0.05);
	}

	.nav-item.is-active a {
		color: var(--st-primary, #0984ae);
		border-bottom-color: var(--st-primary, #0984ae);
	}

	.nav-icon {
		font-size: 18px;
	}

	:global(.external-icon) {
		opacity: 0.5;
		margin-left: 4px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD CONTENT - WordPress: .dashboard__content
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content {
		display: flex;
		gap: 30px;
		padding: 30px;
		background: #f8f9fa;
		min-height: calc(100vh - 200px);
	}

	.dashboard__content-main {
		flex: 1;
		min-width: 0;
	}

	.dashboard__content-sidebar {
		width: 300px;
		flex-shrink: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CONTENT SECTIONS - WordPress: .dashboard__content-section
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content-section {
		background: #fff;
		border-radius: 8px;
		padding: 24px;
		margin-bottom: 24px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.08);
	}

	.section-title {
		color: var(--st-text-color, #333);
		font-weight: 700;
		font-size: 20px;
		font-family: 'Open Sans Condensed', sans-serif;
		margin: 0 0 20px 0;
		padding-bottom: 10px;
		border-bottom: 2px solid var(--st-primary, #0984ae);
	}

	.section-footer {
		text-align: center;
		padding-top: 16px;
		border-top: 1px solid #eee;
		margin-top: 20px;
	}

	.btn-link {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		color: var(--st-link-color, #1e73be);
		text-decoration: none;
		font-weight: 600;
		font-size: 14px;
	}

	.btn-link:hover {
		color: var(--st-primary, #0984ae);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MEMBERSHIP INTRO
	   ═══════════════════════════════════════════════════════════════════════════ */

	.membership-intro p {
		font-size: 16px;
		color: var(--st-text-muted, #64748b);
		line-height: 1.6;
		margin: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   QUICK ACTIONS - Bootstrap Grid
	   ═══════════════════════════════════════════════════════════════════════════ */

	.quick-actions {
		display: flex;
		flex-wrap: wrap;
		margin: -10px;
	}

	.quick-actions > [class*="col-"] {
		padding: 10px;
	}

	.action-card {
		display: flex;
		align-items: center;
		gap: 16px;
		padding: 20px;
		background: #f8f9fa;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		text-decoration: none;
		transition: all 0.2s ease;
		height: 100%;
	}

	.action-card:hover {
		border-color: var(--st-primary, #0984ae);
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
	}

	.action-card--primary {
		background: var(--st-primary, #0984ae);
		border-color: var(--st-primary, #0984ae);
		color: #fff;
	}

	.action-card--primary:hover {
		background: #076787;
		border-color: #076787;
	}

	.action-card--primary .action-card__icon {
		background: rgba(255, 255, 255, 0.2);
		color: #fff;
	}

	.action-card--primary h3,
	.action-card--primary p {
		color: #fff;
	}

	.action-card__icon {
		width: 56px;
		height: 56px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #e0f2fe;
		border-radius: 12px;
		color: var(--st-primary, #0984ae);
		flex-shrink: 0;
	}

	.action-card__content h3 {
		font-size: 16px;
		font-weight: 600;
		color: var(--st-text-color, #333);
		margin: 0 0 4px;
	}

	.action-card__content p {
		font-size: 13px;
		color: var(--st-text-muted, #64748b);
		margin: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   VIDEOS GRID - Bootstrap Grid
	   ═══════════════════════════════════════════════════════════════════════════ */

	.videos-grid {
		display: flex;
		flex-wrap: wrap;
		margin: -10px;
	}

	.videos-grid > [class*="col-"] {
		padding: 10px;
	}

	.video-card {
		display: block;
		background: #fff;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		overflow: hidden;
		text-decoration: none;
		transition: all 0.2s ease;
	}

	.video-card:hover {
		border-color: var(--st-primary, #0984ae);
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
	}

	.video-card:hover .video-card__play {
		opacity: 1;
		transform: translate(-50%, -50%) scale(1);
	}

	.video-card__thumbnail {
		position: relative;
		height: 140px;
		background: linear-gradient(135deg, #1e293b, #334155);
	}

	.video-card__placeholder {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 100%;
		color: #64748b;
	}

	.video-card__duration {
		position: absolute;
		bottom: 8px;
		right: 8px;
		padding: 4px 8px;
		background: rgba(0, 0, 0, 0.8);
		border-radius: 4px;
		font-size: 12px;
		font-weight: 600;
		color: #fff;
	}

	.video-card__play {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%) scale(0.9);
		width: 56px;
		height: 56px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--st-primary, #0984ae);
		border-radius: 50%;
		color: #fff;
		opacity: 0.8;
		transition: all 0.2s ease;
	}

	.video-card__content {
		padding: 16px;
	}

	.video-card__title {
		font-size: 14px;
		font-weight: 600;
		color: var(--st-text-color, #333);
		margin: 0 0 8px;
		line-height: 1.4;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.video-card__meta {
		display: flex;
		align-items: center;
		gap: 6px;
		font-size: 12px;
		color: var(--st-text-muted, #64748b);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESOURCES LIST
	   ═══════════════════════════════════════════════════════════════════════════ */

	.resources-list {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.resource-item {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 12px 16px;
		background: #f8f9fa;
		border-radius: 8px;
		color: var(--st-text-color, #333);
		text-decoration: none;
		font-weight: 500;
		transition: all 0.15s ease;
	}

	.resource-item:hover {
		background: #e0f2fe;
		color: var(--st-primary, #0984ae);
	}

	.resource-item span {
		flex: 1;
	}

	:global(.resource-item .chevron) {
		color: #999;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SIDEBAR
	   ═══════════════════════════════════════════════════════════════════════════ */

	.sidebar-section {
		background: #fff;
		border-radius: 8px;
		padding: 20px;
		margin-bottom: 20px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.08);
	}

	.sidebar-title {
		font-size: 14px;
		font-weight: 700;
		color: var(--st-text-color, #333);
		margin: 0 0 16px;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.status-card {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.status-row {
		display: flex;
		justify-content: space-between;
		font-size: 14px;
	}

	.status-label {
		color: var(--st-text-muted, #64748b);
	}

	.status-value {
		font-weight: 600;
		color: var(--st-text-color, #333);
	}

	.status-active {
		color: #10b981;
	}

	.schedule-list {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.schedule-item {
		display: flex;
		gap: 12px;
		align-items: flex-start;
		color: var(--st-text-muted, #64748b);
	}

	.schedule-item div {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.schedule-item strong {
		font-size: 14px;
		color: var(--st-text-color, #333);
	}

	.schedule-item span {
		font-size: 13px;
	}

	.btn-outline {
		display: block;
		width: 100%;
		padding: 12px;
		border: 1px solid var(--st-border-color, #dbdbdb);
		border-radius: 8px;
		background: #fff;
		color: var(--st-text-color, #333);
		font-weight: 600;
		font-size: 14px;
		text-align: center;
		text-decoration: none;
		transition: all 0.15s ease;
	}

	.btn-outline:hover {
		border-color: var(--st-primary, #0984ae);
		color: var(--st-primary, #0984ae);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   BOOTSTRAP GRID UTILITIES
	   ═══════════════════════════════════════════════════════════════════════════ */

	.row {
		display: flex;
		flex-wrap: wrap;
	}

	[class*="col-"] {
		width: 100%;
	}

	@media (min-width: 576px) {
		.col-sm-6 {
			width: 50%;
		}
	}

	@media (min-width: 992px) {
		.col-lg-3 {
			width: 25%;
		}
	}

	@media (min-width: 1200px) {
		.col-xl-4 {
			width: 33.333333%;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 992px) {
		.dashboard__content {
			flex-direction: column;
		}

		.dashboard__content-sidebar {
			width: 100%;
		}
	}

	@media (max-width: 768px) {
		.dashboard__header {
			padding: 16px;
		}

		.dashboard__page-title {
			font-size: 24px;
		}

		.dashboard__nav-secondary {
			padding: 0 16px;
		}

		.dashboard__content {
			padding: 16px;
		}

		.dashboard__content-section {
			padding: 16px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   REDUCED MOTION
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.action-card,
		.video-card,
		.resource-item,
		.btn-tradingroom,
		.nav-item a {
			transition: none;
		}

		.video-card__play {
			opacity: 1;
			transform: translate(-50%, -50%);
		}
	}
</style>

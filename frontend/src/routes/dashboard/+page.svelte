<script lang="ts">
	/**
	 * Member Dashboard - Svelte 5 Component-Based Architecture
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Refactored to use modular Svelte 5 components with scoped CSS.
	 * All memberships, icons, and content come from the API/database.
	 *
	 * @version 3.0.0 - Svelte 5 with component-based architecture
	 */
	import { onMount } from 'svelte';
	import { getUserMemberships, type UserMembership, type UserMembershipsResponse } from '$lib/api/user-memberships';

	// Import Svelte 5 Dashboard Components
	import {
		DashboardHeader,
		MembershipCard,
		ArticleCard,
		WeeklyWatchlistSection,
		SectionTitle
	} from '$lib/components/dashboard';

	// Tabler Icons for membership cards
	import IconChartLine from '@tabler/icons-svelte/icons/chart-line';
	import IconChartCandle from '@tabler/icons-svelte/icons/chart-candle';
	import IconTrendingUp from '@tabler/icons-svelte/icons/trending-up';
	import IconActivity from '@tabler/icons-svelte/icons/activity';
	import IconRocket from '@tabler/icons-svelte/icons/rocket';
	import IconStar from '@tabler/icons-svelte/icons/star';
	import IconBolt from '@tabler/icons-svelte/icons/bolt';
	import IconBook from '@tabler/icons-svelte/icons/book';
	import IconSchool from '@tabler/icons-svelte/icons/school';
	import IconCalendarWeek from '@tabler/icons-svelte/icons/calendar-week';
	import IconWallet from '@tabler/icons-svelte/icons/wallet';
	import IconBell from '@tabler/icons-svelte/icons/bell';
	import IconReportAnalytics from '@tabler/icons-svelte/icons/report-analytics';

	// Icon registry - maps membership slugs to Tabler icon components
	type IconComponent = typeof IconChartLine;
	const membershipIconRegistry: Record<string, IconComponent> = {
		'mastering-the-trade': IconChartCandle,
		'options-day-trading': IconChartCandle,
		'simpler-showcase': IconStar,
		'revolution-showcase': IconStar,
		'trade-of-the-week': IconCalendarWeek,
		'weekly-watchlist': IconCalendarWeek,
		'ww': IconCalendarWeek,
		'day-trading-room': IconChartCandle,
		'explosive-swings': IconBolt,
		'swing-trading': IconTrendingUp,
		'small-accounts': IconWallet,
		'day-trading': IconActivity,
		'moxie': IconRocket,
		'foundation': IconBook,
		'courses': IconSchool,
		'indicators': IconReportAnalytics,
		'spx-profit-pulse': IconBell,
		'default': IconChartLine
	};

	function getMembershipIcon(slug: string): IconComponent {
		return membershipIconRegistry[slug] || membershipIconRegistry['default'];
	}

	// State
	let membershipsData = $state<UserMembershipsResponse | null>(null);
	let loading = $state(true);
	let error = $state<string | null>(null);

	// Fetch memberships on mount
	onMount(async () => {
		console.log('[Dashboard] Initializing dashboard page...');
		try {
			membershipsData = await getUserMemberships();
			console.log('[Dashboard] Memberships loaded:', {
				total: membershipsData?.memberships?.length || 0,
				tradingRooms: membershipsData?.tradingRooms?.length || 0
			});
		} catch (err) {
			console.error('[Dashboard] Failed to load memberships:', err);
			error = 'Failed to load memberships. Please try again.';
		} finally {
			loading = false;
		}
	});

	// URL generation helpers
	function getDashboardUrl(membership: UserMembership): string {
		if (membership.type === 'trading-room') {
			const roomSlug = membership.roomSlug || membership.slug;
			return `/dashboard/${roomSlug}`;
		}
		return membership.accessUrl || `/dashboard/${membership.slug}`;
	}

	function getAccessUrl(membership: UserMembership): string {
		if (membership.type === 'trading-room') {
			const roomSlug = membership.roomSlug || membership.slug;
			return `/dashboard/${roomSlug}`;
		}
		return `/dashboard/${membership.slug}/alerts`;
	}

	function getActionLabel(membership: UserMembership): string {
		if (membership.roomLabel) {
			return membership.roomLabel.includes('Room') ? 'Trading Room' : membership.roomLabel;
		}
		switch (membership.type) {
			case 'trading-room': return 'Trading Room';
			case 'alert-service': return 'View Alerts';
			case 'course': return 'View Course';
			case 'indicator': return 'Download';
			default: return 'Access';
		}
	}

	function shouldOpenNewTab(membership: UserMembership): boolean {
		return membership.type === 'trading-room';
	}

	// Sample articles for latest updates
	const sampleArticles = [
		{
			title: 'Welcome to Revolution Trading Pros',
			href: '/blog',
			image: 'https://images.unsplash.com/photo-1611974789855-9c2a0a7236a3?w=800&h=600&fit=crop',
			label: 'Daily Video',
			meta: 'Latest market insights and trading education',
			restricted: true
		},
		{
			title: 'Market Analysis & Trading Strategies',
			href: '/blog',
			image: 'https://images.unsplash.com/photo-1590283603385-17ffb3a7f29f?w=800&h=600&fit=crop',
			label: 'Daily Video',
			meta: 'Expert insights from our trading team',
			restricted: true
		},
		{
			title: 'Learn Advanced Trading Techniques',
			href: '/blog',
			image: 'https://images.unsplash.com/photo-1642790106117-e829e14a795f?w=800&h=600&fit=crop',
			label: 'Daily Video',
			meta: 'Professional trading education',
			excerpt: 'Get access to exclusive trading education, live trading rooms, and expert analysis.'
		}
	];
</script>

<!-- DASHBOARD HEADER -->
<DashboardHeader
	title="Member Dashboard"
	tradingRooms={membershipsData?.tradingRooms || []}
/>

<!-- DASHBOARD CONTENT -->
<div class="dashboard__content">
	<div class="dashboard__content-main">

		<!-- MEMBERSHIPS SECTION -->
		{#if loading}
			<section class="dashboard__content-section">
				<SectionTitle title="Memberships" />
				<div class="loading-state">Loading memberships...</div>
			</section>
		{:else if error}
			<section class="dashboard__content-section">
				<SectionTitle title="Memberships" />
				<div class="error-state">
					<p>{error}</p>
					<button class="btn btn-primary" onclick={() => location.reload()}>Retry</button>
				</div>
			</section>
		{:else if membershipsData?.memberships && membershipsData.memberships.length > 0}
			<section class="dashboard__content-section">
				<SectionTitle title="Memberships" />
				<div class="membership-cards">
					{#each membershipsData.memberships as membership (membership.id)}
						{@const MembershipIcon = getMembershipIcon(membership.slug)}
						<div class="membership-cards__item">
							<MembershipCard
								name={membership.name}
								slug={membership.slug}
								type={membership.type}
								dashboardUrl={getDashboardUrl(membership)}
								actionUrl={getAccessUrl(membership)}
								actionLabel={getActionLabel(membership)}
								actionNewTab={shouldOpenNewTab(membership)}
								variant={membership.slug === 'simpler-showcase' ? 'simpler-showcase' : 'default'}
							>
								{#snippet icon()}
									<MembershipIcon size={24} />
								{/snippet}
							</MembershipCard>
						</div>
					{/each}
				</div>
			</section>
		{/if}

		<!-- LATEST UPDATES SECTION - Show when no memberships -->
		{#if !loading && !error && (!membershipsData?.memberships || membershipsData.memberships.length === 0)}
			<section class="dashboard__content-section">
				<SectionTitle title="Latest Updates" />
				<div class="article-cards">
					{#each sampleArticles as article}
						<div class="article-cards__item">
							<ArticleCard
								title={article.title}
								href={article.href}
								image={article.image}
								label={article.label}
								meta={article.meta}
								excerpt={article.excerpt}
								restricted={article.restricted}
								buttonText="Watch Now"
							/>
						</div>
					{/each}
				</div>
			</section>
		{/if}

		<!-- TOOLS SECTION -->
		<section class="dashboard__content-section">
			<SectionTitle title="Tools" />
			<div class="membership-cards">
				<div class="membership-cards__item">
					<MembershipCard
						name="Weekly Watchlist"
						slug="ww"
						type="tool"
						actionLabel="Dashboard"
					>
						{#snippet icon()}
							<IconCalendarWeek size={24} />
						{/snippet}
					</MembershipCard>
				</div>
			</div>
		</section>

		<!-- WEEKLY WATCHLIST FEATURED SECTION -->
		<WeeklyWatchlistSection
			title="Weekly Watchlist"
			featuredTitle="Weekly Watchlist with TG Watkins"
			description="Week of December 22, 2025."
			href="/watchlist/latest"
		/>

	</div>

	<!-- SECONDARY SIDEBAR (empty on main dashboard) -->
	<aside class="dashboard__content-sidebar">
		<section class="content-sidebar__section"></section>
	</aside>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD CONTENT LAYOUT
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard__content {
		display: flex;
		flex-flow: row nowrap;
	}

	.dashboard__content-main {
		border-right: 1px solid #dbdbdb;
		flex: 1 1 auto;
		background-color: #efefef;
		min-width: 0;
	}

	.dashboard__content-section {
		padding: 30px 40px;
		background-color: #fff;
		margin-bottom: 20px;
	}

	@media (min-width: 1280px) {
		.dashboard__content-section {
			padding: 30px;
		}
	}

	@media (min-width: 1440px) {
		.dashboard__content-section {
			padding: 40px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MEMBERSHIP CARDS GRID
	   ═══════════════════════════════════════════════════════════════════════════ */
	.membership-cards {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 30px;
	}

	.membership-cards__item {
		min-width: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ARTICLE CARDS GRID
	   ═══════════════════════════════════════════════════════════════════════════ */
	.article-cards {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 20px;
	}

	.article-cards__item {
		display: flex;
		min-width: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   LOADING & ERROR STATES
	   ═══════════════════════════════════════════════════════════════════════════ */
	.loading-state,
	.error-state {
		padding: 40px;
		text-align: center;
		color: #666;
		font-size: 14px;
	}

	.error-state p {
		margin: 0 0 20px;
		color: #dc3545;
	}

	.btn {
		display: inline-block;
		padding: 10px 20px;
		background: #F69532;
		color: #fff;
		text-decoration: none;
		border: none;
		border-radius: 4px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		cursor: pointer;
		transition: background-color 0.2s ease-in-out;
	}

	.btn:hover {
		background: #dc7309;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SECONDARY SIDEBAR (hidden on main dashboard)
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard__content-sidebar {
		display: none;
		width: 260px;
		flex: 0 0 auto;
		background: #fff;
		border-right: 1px solid #dbdbdb;
	}

	.content-sidebar__section {
		padding: 20px;
	}
</style>

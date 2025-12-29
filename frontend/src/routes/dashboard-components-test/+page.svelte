<script lang="ts">
	/**
	 * Dashboard Components Test Page
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * This page demonstrates the proper Svelte 5 component architecture.
	 * All UI is composed from reusable components with scoped styles.
	 *
	 * URL: /dashboard-components-test
	 *
	 * @version 1.0.0 - Svelte 5 with $props()
	 */

	// Import dashboard components
	import {
		DashboardSidebar,
		DashboardHeader,
		DashboardBreadcrumbs,
		MembershipCard,
		ArticleCard,
		WeeklyWatchlistSection,
		MobileToggle,
		SectionTitle
	} from '$lib/components/dashboard';

	// Import NavBar and Footer
	import { NavBar } from '$lib/components/nav';
	import Footer from '$lib/components/sections/Footer.svelte';

	// Sample data for testing
	const user = {
		name: 'John Trader',
		avatar: 'https://www.gravatar.com/avatar/?d=mp&s=88'
	};

	const memberships = [
		{ id: 1, name: 'Mastering the Trade', slug: 'mastering-the-trade', type: 'trading-room' },
		{ id: 2, name: 'Options Day Trading Room', slug: 'options-day-trading', type: 'trading-room' },
		{ id: 3, name: 'Simpler Showcase', slug: 'simpler-showcase', type: 'showcase' }
	];

	const tradingRooms = [
		{ id: 1, name: 'Day Trading Room', slug: 'day-trading-room', roomLabel: 'Day Trading Room' },
		{ id: 2, name: 'Options Room', slug: 'options-day-trading', roomLabel: 'Options Day Trading Room' },
		{ id: 3, name: 'Swing Trading Room', slug: 'swing-trading-room', roomLabel: 'Swing Trading Room' }
	];

	const articles = [
		{
			id: 1,
			title: 'Welcome to Revolution Trading Pros',
			href: '/blog',
			image: 'https://images.unsplash.com/photo-1611974789855-9c2a0a7236a3?w=800&h=600&fit=crop',
			label: 'Daily Video',
			meta: 'Latest market insights and trading education',
			restricted: true
		},
		{
			id: 2,
			title: 'Market Analysis & Trading Strategies',
			href: '/blog',
			image: 'https://images.unsplash.com/photo-1590283603385-17ffb3a7f29f?w=800&h=600&fit=crop',
			label: 'Daily Video',
			meta: 'Expert insights from our trading team',
			restricted: true
		},
		{
			id: 3,
			title: 'Learn Advanced Trading Techniques',
			href: '/blog',
			image: 'https://images.unsplash.com/photo-1642790106117-e829e14a795f?w=800&h=600&fit=crop',
			label: 'Daily Video',
			meta: 'Professional trading education',
			excerpt: 'Get access to exclusive trading education, live trading rooms, and expert analysis.'
		}
	];

	// Sidebar navigation categories
	const sidebarCategories = [
		{
			title: '',
			items: [
				{ label: 'Member Dashboard', href: '/dashboard/', bold: false },
				{ label: 'My Classes', href: '/dashboard/classes/', bold: true },
				{ label: 'My Indicators', href: '/dashboard/indicators/', bold: true }
			]
		},
		{
			title: 'memberships',
			items: [
				{ label: 'Day Trading Room', href: '/dashboard/day-trading-room/' },
				{ label: 'Options Day Trading', href: '/dashboard/options-day-trading/' }
			]
		},
		{
			title: 'tools',
			items: [
				{ label: 'Weekly Watchlist', href: '/dashboard/ww/' },
				{ label: 'Support', href: 'https://intercom.help/simpler-trading/en/' }
			]
		},
		{
			title: 'account',
			items: [
				{ label: 'My Account', href: '/dashboard/account/' }
			]
		}
	];

	// Mobile menu state
	let mobileMenuOpen = $state(false);

	function toggleMobileMenu() {
		mobileMenuOpen = !mobileMenuOpen;
	}
</script>

<svelte:head>
	<title>Dashboard Components Test | Revolution Trading Pros</title>
	<link href="https://fonts.googleapis.com/css2?family=Open+Sans:wght@300;400;600;700;800&family=Open+Sans+Condensed:wght@700&display=swap" rel="stylesheet">
</svelte:head>

<!-- NAV BAR -->
<NavBar />

<!-- BREADCRUMBS -->
<DashboardBreadcrumbs />

<!-- DASHBOARD LAYOUT -->
<div class="dashboard-layout" class:menu-open={mobileMenuOpen}>

	<!-- SIDEBAR (using component) -->
	<DashboardSidebar
		userName={user.name}
		userAvatar={user.avatar}
		categories={sidebarCategories}
	/>

	<!-- MAIN CONTENT -->
	<main class="dashboard-main">

		<!-- HEADER (using component) -->
		<DashboardHeader
			title="Member Dashboard"
			tradingRooms={tradingRooms}
		/>

		<!-- CONTENT -->
		<div class="dashboard-content">

			<!-- MEMBERSHIPS SECTION -->
			<section class="dashboard-section">
				<SectionTitle title="Memberships" />
				<div class="cards-grid">
					{#each memberships as membership (membership.id)}
						<MembershipCard
							name={membership.name}
							slug={membership.slug}
							type={membership.type}
							variant={membership.slug === 'simpler-showcase' ? 'simpler-showcase' : 'default'}
							actionNewTab={membership.type === 'trading-room'}
						/>
					{/each}
				</div>
			</section>

			<!-- TOOLS SECTION -->
			<section class="dashboard-section">
				<SectionTitle title="Tools" />
				<div class="cards-grid">
					<MembershipCard
						name="Weekly Watchlist"
						slug="ww"
						type="tool"
						actionLabel="Dashboard"
					/>
				</div>
			</section>

			<!-- LATEST UPDATES SECTION -->
			<section class="dashboard-section">
				<SectionTitle title="Latest Updates" />
				<div class="articles-grid">
					{#each articles as article (article.id)}
						<ArticleCard
							title={article.title}
							href={article.href}
							image={article.image}
							label={article.label}
							meta={article.meta}
							excerpt={article.excerpt}
							restricted={article.restricted}
						/>
					{/each}
				</div>
			</section>

			<!-- WEEKLY WATCHLIST FEATURED -->
			<WeeklyWatchlistSection
				description="Week of December 29, 2025."
			/>

		</div>
	</main>
</div>

<!-- MOBILE TOGGLE -->
<MobileToggle
	isOpen={mobileMenuOpen}
	onToggle={toggleMobileMenu}
/>

<!-- FOOTER -->
<Footer />

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD LAYOUT - Clean, minimal styles
	   Component styles are scoped within each component
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard-layout {
		display: flex;
		flex-flow: row nowrap;
		min-height: calc(100vh - 100px);
		background-color: #efefef;
	}

	.dashboard-main {
		flex: 1 1 auto;
		min-width: 0;
		display: flex;
		flex-direction: column;
		background-color: #fff;
	}

	.dashboard-content {
		flex: 1;
		background-color: #efefef;
	}

	/* Sections */
	.dashboard-section {
		padding: 30px 40px;
		background-color: #fff;
		margin-bottom: 0;
	}

	.dashboard-section + .dashboard-section {
		border-top: 1px solid #dbdbdb;
	}

	/* Card Grids */
	.cards-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		gap: 20px;
	}

	.articles-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 20px;
	}

	/* Responsive */
	@media (max-width: 1279px) {
		.dashboard-layout {
			flex-direction: column;
			padding-bottom: 50px;
		}
	}

	@media (max-width: 767px) {
		.dashboard-section {
			padding: 20px;
		}

		.cards-grid,
		.articles-grid {
			grid-template-columns: 1fr;
		}
	}
</style>

<!--
	URL: /dashboard/day-trading-room/traders/lead-trader
	
	Lead Trader Profile Page
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation
	
	Profile and bio for the Day Trading Room lead trader.
	
	@version 1.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	import SecondaryDashboardSidebar from '$lib/components/dashboard/SecondaryDashboardSidebar.svelte';

	const secondaryNavItems = [
		{ href: '/dashboard/day-trading-room', icon: 'layout-dashboard', text: 'Day Trading Room Dashboard' },
		{ href: '/dashboard/day-trading-room/daily-videos', icon: 'video', text: 'Premium Daily Videos' },
		{ href: '/dashboard/day-trading-room/learning-center', icon: 'school', text: 'Learning Center' },
		{ href: '/dashboard/day-trading-room/trading-room-archive', icon: 'archive', text: 'Trading Room Archives' },
		{
			href: '#',
			icon: 'users',
			text: 'Meet the Traders',
			submenu: [
				{ href: '/dashboard/day-trading-room/traders/lead-trader', icon: '', text: 'Lead Trader' },
				{ href: '/dashboard/day-trading-room/traders/senior-analyst', icon: '', text: 'Senior Analyst' },
				{ href: '/dashboard/day-trading-room/traders/head-moderator', icon: '', text: 'Head Moderator' }
			]
		},
		{
			href: '#',
			icon: 'shopping-cart',
			text: 'Trader Store',
			submenu: [
				{ href: '/dashboard/day-trading-room/store/indicators', icon: '', text: 'Indicators' },
				{ href: '/dashboard/day-trading-room/store/courses', icon: '', text: 'Advanced Courses' },
				{ href: '/dashboard/day-trading-room/store/tools', icon: '', text: 'Trading Tools' }
			]
		}
	];
</script>

<svelte:head>
	<title>Lead Trader | Day Trading Room | Revolution Trading Pros</title>
	<meta name="description" content="Meet our lead trader and learn about their trading approach and expertise." />
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<SecondaryDashboardSidebar courseName="Day Trading Room" menuItems={secondaryNavItems} />

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">Lead Trader</h1>
	</div>
</header>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<h2 class="section-title">Meet Our Lead Trader</h2>
			<p>Learn about our lead trader's background, trading philosophy, and approach to day trading.</p>
		</section>
	</div>
</div>

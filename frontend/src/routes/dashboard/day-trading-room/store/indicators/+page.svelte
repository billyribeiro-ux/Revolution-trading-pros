<!--
	URL: /dashboard/day-trading-room/store/indicators
	
	Indicators Store Page
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation
	
	Browse and purchase trading indicators and tools.
	
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
	<title>Trading Indicators | Day Trading Room | Revolution Trading Pros</title>
	<meta name="description" content="Browse and purchase professional trading indicators and technical analysis tools." />
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">Trading Indicators</h1>
	</div>
</header>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<h2 class="section-title">Professional Trading Indicators</h2>
			<p>Enhance your trading with our proprietary indicators and technical analysis tools.</p>
		</section>
	</div>
</div>

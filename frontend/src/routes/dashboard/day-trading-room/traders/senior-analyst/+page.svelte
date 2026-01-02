<!--
	URL: /dashboard/day-trading-room/traders/senior-analyst
	
	Senior Analyst Profile Page
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation
	
	Profile and bio for the Day Trading Room senior analyst.
	
	@version 1.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	import CourseSecondaryNav from '$lib/components/dashboard/CourseSecondaryNav.svelte';

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
	<title>Senior Analyst | Day Trading Room | Revolution Trading Pros</title>
	<meta name="description" content="Meet our senior analyst and learn about their market analysis expertise." />
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<CourseSecondaryNav courseName="Day Trading Room" menuItems={secondaryNavItems} />

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">Senior Analyst</h1>
	</div>
</header>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<h2 class="section-title">Meet Our Senior Analyst</h2>
			<p>Learn about our senior analyst's background, market analysis approach, and technical expertise.</p>
		</section>
	</div>
</div>

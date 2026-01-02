<!--
	URL: /dashboard/day-trading-room/daily-videos
	
	Premium Daily Videos Page
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation
	
	Daily video analysis and market commentary for Day Trading Room members.
	
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
	<title>Premium Daily Videos | Day Trading Room | Revolution Trading Pros</title>
	<meta name="description" content="Access daily video analysis and market commentary from our expert traders." />
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<CourseSecondaryNav courseName="Day Trading Room" menuItems={secondaryNavItems} />

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">Premium Daily Videos</h1>
	</div>
</header>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<h2 class="section-title">Daily Market Analysis</h2>
			<p>Access our library of premium daily videos with expert market analysis, trade setups, and educational content.</p>
		</section>
	</div>
</div>

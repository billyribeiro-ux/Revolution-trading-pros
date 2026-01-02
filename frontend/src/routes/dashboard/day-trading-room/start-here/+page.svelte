<!--
	URL: /dashboard/day-trading-room/start-here
	
	Start Here - Getting Started Guide
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation
	
	Onboarding and getting started guide for new Day Trading Room members.
	
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
	<title>Start Here | Day Trading Room | Revolution Trading Pros</title>
	<meta name="description" content="New to the Day Trading Room? Start here to get oriented and learn how to make the most of your membership." />
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<CourseSecondaryNav courseName="Day Trading Room" menuItems={secondaryNavItems} />

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">New? Start Here</h1>
	</div>
</header>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<h2 class="section-title">Welcome to the Day Trading Room!</h2>
			<p>This guide will help you get started and make the most of your membership.</p>
			
			<div class="getting-started-content">
				<h3>Step 1: Watch the Welcome Video</h3>
				<p>Start by watching our comprehensive welcome video that introduces you to the Day Trading Room, our trading philosophy, and how to navigate the platform.</p>
				
				<h3>Step 2: Review Trading Room Rules</h3>
				<p>Familiarize yourself with our <a href="/trading-room-rules.pdf" target="_blank">Trading Room Rules</a> to ensure a positive experience for all members.</p>
				
				<h3>Step 3: Join the Live Trading Sessions</h3>
				<p>Check the <a href="/dashboard/day-trading-room">Trading Room Schedule</a> and join us for live market analysis and real-time trading.</p>
				
				<h3>Step 4: Access Educational Resources</h3>
				<p>Visit the <a href="/dashboard/day-trading-room/learning-center">Learning Center</a> to access courses, tutorials, and training materials.</p>
				
				<h3>Step 5: Review Daily Videos</h3>
				<p>Watch our <a href="/dashboard/day-trading-room/daily-videos">Premium Daily Videos</a> for market analysis and trade setups.</p>
			</div>
		</section>
	</div>
</div>

<style>
	.getting-started-content {
		max-width: 800px;
		margin-top: 30px;
	}

	.getting-started-content h3 {
		font-size: 20px;
		font-weight: 700;
		color: #191717;
		margin: 30px 0 12px;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
	}

	.getting-started-content p {
		color: #555;
		line-height: 1.6;
		margin-bottom: 16px;
	}

	.getting-started-content a {
		color: #0984ae;
		text-decoration: none;
		font-weight: 600;
	}

	.getting-started-content a:hover {
		text-decoration: underline;
	}
</style>

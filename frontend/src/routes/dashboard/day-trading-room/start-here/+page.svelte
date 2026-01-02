<!--
	URL: /dashboard/day-trading-room/start-here
	
	Getting Started with Day Trading Room
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation
	Based on MasterDash Start Here page structure
	
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
	<title>Getting Started with Day Trading Room | Revolution Trading Pros</title>
	<meta name="description" content="Welcome to your Revolution Trading Pros Day Trading Room Membership. Get started on the right path with our comprehensive getting started checklist." />
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<h2 class="start-here-title">Welcome to your Revolution Trading Pros Day Trading Room Membership</h2>
			
			<div class="start-here-intro">
				<p>On this "Start Here" page, we've outlined what we believe are the key steps to get you started on a good path with us. Please take some time to go through this page carefully and "check all of the boxes" to ensure you're set-up for success.</p>
				<p>The <strong>Getting Started Checklist</strong> below is essential. Regardless of if you're new to trading or are a seasoned veteran, we want to help you navigate your way through the Revolution Trading environment in the most efficient way possible.</p>
			</div>

			<div class="contact-support">
				<p>As always, should you have questions or need assistance, please contact our dedicated support team at <a href="mailto:support@revolutiontradingpros.com">support@revolutiontradingpros.com</a> or call (512) 266-8659.</p>
			</div>

			<div class="getting-started-checklist">
				<h3>Getting Started Checklist</h3>
				
				<div class="checklist-section">
					<h4>📋 1. Review Your Trading Plan</h4>
					<p>First off, do you have one? If you do, great! It's a great time to re-examine this and remind yourself of your goals, your rules, and what you most hope to get out of working with Revolution Trading. Be sure to update your plan accordingly and document anything new that pops up.</p>
					<p>If you don't have a trading plan (or if you do and you'd like to change it up a bit), we have tools and resources to help you craft a plan that will help your focus and determination with trading.</p>
				</div>

				<div class="checklist-section">
					<h4>💻 2. Set Up Your Charts, Broker & Tools</h4>
					<p>There's a wide-range here and it can become more to manage than what's really needed. However, there are some basic considerations for every trader to ensure you're stepping into the world of trading appropriately equipped.</p>
					<p>Are you set-up with a broker and have negotiated a rate for your commissions that's in line with the industry standard and/or that you're happy with?</p>
					<p><strong>Recommended Brokers:</strong></p>
					<ul>
						<li><strong>ThinkorSwim by TD Ameritrade/Schwab</strong> - Robust charting and execution platform</li>
						<li><strong>tastytrade</strong> - Best pricing with zero commissions and low fees</li>
						<li><strong>TradeStation</strong> - Excellent charting features and execution</li>
					</ul>
				</div>

				<div class="checklist-section">
					<h4>👥 3. Get to Know Our Trading Team</h4>
					<p>If you haven't already watched the welcome video that walks through everything you get to enjoy as part of this service, please make time to do so.</p>
					<p>Beyond what's in the service, you'll find links to each of our Traders Bio Pages. Inside their pages you'll find everything from their Trading Plans, Favorite Strategies & Setups, what's on their charts, recommended courses and videos, and more.</p>
				</div>

				<div class="checklist-section">
					<h4>📚 4. Explore the Learning Center</h4>
					<p>Visit the <a href="/dashboard/day-trading-room/learning-center">Learning Center</a> to access our comprehensive library of courses, tutorials, and training materials designed to help you master day trading.</p>
				</div>

				<div class="checklist-section">
					<h4>📺 5. Watch Premium Daily Videos</h4>
					<p>Access our <a href="/dashboard/day-trading-room/daily-videos">Premium Daily Videos</a> for expert market analysis, trade setups, and educational content updated daily.</p>
				</div>

				<div class="checklist-section">
					<h4>🎯 6. Join Live Trading Sessions</h4>
					<p>Check the Trading Room Schedule on the <a href="/dashboard/day-trading-room">main dashboard</a> and join us for live market analysis and real-time trading sessions.</p>
				</div>

				<div class="checklist-section">
					<h4>📖 7. Review Trading Room Rules</h4>
					<p>Familiarize yourself with our <a href="/trading-room-rules.pdf" target="_blank">Trading Room Rules</a> to ensure a positive and productive experience for all members.</p>
				</div>
			</div>
		</section>
	</div>
</div>

<style>
	.start-here-title {
		font-size: 2.5rem;
		font-weight: 700;
		color: #666666;
		text-align: center;
		margin-bottom: 30px;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.2;
	}

	.start-here-intro {
		font-size: 1.3rem;
		line-height: 1.8;
		color: #676767;
		margin-bottom: 40px;
	}

	.start-here-intro p {
		margin-bottom: 20px;
	}

	.contact-support {
		background: #f8f9fa;
		border-left: 4px solid #143E59;
		padding: 20px;
		margin: 30px 0;
		border-radius: 4px;
	}

	.contact-support p {
		margin: 0;
		color: #555;
		line-height: 1.6;
	}

	.contact-support a {
		color: #3879F3;
		text-decoration: none;
		font-weight: 600;
	}

	.contact-support a:hover {
		text-decoration: underline;
	}

	.getting-started-checklist {
		margin-top: 50px;
	}

	.getting-started-checklist h3 {
		font-size: 2rem;
		font-weight: 700;
		color: #191717;
		margin-bottom: 30px;
		font-family: 'Open Sans', sans-serif;
	}

	.checklist-section {
		background: #ffffff;
		border: 1px solid #e5e5e5;
		border-radius: 8px;
		padding: 30px;
		margin-bottom: 20px;
		box-shadow: 0px 2px 4px rgba(0, 0, 0, 0.05);
	}

	.checklist-section h4 {
		font-size: 1.5rem;
		font-weight: 700;
		color: #143E59;
		margin-bottom: 15px;
		font-family: 'Open Sans', sans-serif;
	}

	.checklist-section p {
		color: #555;
		line-height: 1.8;
		margin-bottom: 15px;
		font-size: 1.1rem;
	}

	.checklist-section ul {
		margin: 15px 0;
		padding-left: 25px;
	}

	.checklist-section li {
		color: #555;
		line-height: 1.8;
		margin-bottom: 10px;
		font-size: 1.1rem;
	}

	.checklist-section a {
		color: #143E59;
		text-decoration: none;
		font-weight: 600;
	}

	.checklist-section a:hover {
		text-decoration: underline;
	}

	@media (max-width: 768px) {
		.start-here-title {
			font-size: 1.9rem;
		}

		.start-here-intro {
			font-size: 1.1rem;
		}

		.checklist-section {
			padding: 20px;
		}

		.checklist-section h4 {
			font-size: 1.3rem;
		}
	}
</style>

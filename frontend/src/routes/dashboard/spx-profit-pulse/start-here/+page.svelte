<!--
	URL: /dashboard/spx-profit-pulse/start-here
	
	Getting Started with SPX Profit Pulse - 3 Card Layout (2-1 Grid)
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation - Svelte 5
	Responsive card layout: 2 cards top row, 1 centered bottom
	
	@version 2.0.0 - Card-based layout
	@author Revolution Trading Pros
-->
<script lang="ts">
	// No state needed for static card layout
</script>

<svelte:head>
	<title>Getting Started with SPX Profit Pulse | Revolution Trading Pros</title>
	<meta
		name="description"
		content="Welcome to your Revolution Trading Pros SPX Profit Pulse Membership. Get started on the right path with our comprehensive getting started checklist."
	/>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<!-- Welcome Heading -->
			<h2 class="page-heading">
				<span class="page-heading__text"
					>Welcome to your Revolution Trading Pros SPX Profit Pulse Membership</span
				>
			</h2>

			<!-- Intro Text -->
			<div class="intro-content">
				<p>
					On this "Start Here" page, we've outlined what we believe are the key steps to get you
					started on a good path with us. Please take some time to go through this page carefully
					and "check all of the boxes" to ensure you're set-up for success.
				</p>
				<p>
					The <strong>Getting Started Checklist</strong> below is essential. Regardless of if you're new
					to trading or are a seasoned veteran, we want to help you navigate your way through the Revolution
					Trading environment in the most efficient way possible.
				</p>
			</div>

			<!-- Contact Support Section -->
			<div class="contact-support--container">
				<div class="img">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="159.102"
						height="100"
						viewBox="0 0 159.102 146.793"
					>
						<g transform="translate(-28.444 -251.482)">
							<path
								d="M131.267,370.888l-21.211-27.476a1.148,1.148,0,0,0-1.606-.207L97.582,351.6a1.144,1.144,0,0,0-.207,1.606l.195.252c-7.58-.663-21.246-10.162-29.3-20.427-7.893-10.393-13.624-26.02-12.346-33.519l.195.252a1.148,1.148,0,0,0,1.606.207l10.868-8.392a1.144,1.144,0,0,0,.207-1.606L47.583,262.494a1.143,1.143,0,0,0-1.6-.207l-10.877,8.4a1.13,1.13,0,0,0-.207,1.587l.007.007c-5.759,5.988-7.708,12.945-5.681,22.805,1.012,4.925,3.574,24.651,23.188,50.155C72.118,370.674,90.578,378.178,95.09,380.4c9.027,4.457,16.247,4.336,23.5.278l.007.007a1.13,1.13,0,0,0,1.59.2l10.877-8.4A1.144,1.144,0,0,0,131.267,370.888Z"
								transform="translate(0 14.54)"
								fill="#505050"
							/>
							<path
								d="M97.6,272.17c20.115,0,36.42,13,36.42,29.028a24.069,24.069,0,0,1-3.255,11.992c3.84,11.089,11.728,14.575,11.728,14.575-13.227.737-18.272-3.184-20.037-5.363A41.7,41.7,0,0,1,97.6,330.224c-20.118,0-36.422-13-36.422-29.026S77.486,272.17,97.6,272.17Z"
								transform="translate(45.049 28.467)"
								fill="#ffb412"
							/>
							<path
								d="M101.176,251.482c-25.473,0-46.126,16.459-46.126,36.76a30.482,30.482,0,0,0,4.125,15.19c-4.864,14.04-14.853,18.457-14.853,18.457,16.751.934,23.138-4.03,25.376-6.791C77.94,321.237,89,325,101.176,325c25.476,0,46.126-16.456,46.126-36.76S126.651,251.482,101.176,251.482Z"
								transform="translate(21.849 0)"
								fill="#53c2c9"
							/>
							<g transform="translate(106.231 264.591)">
								<path
									d="M93.494,269.459c0,8.307-10.445,10.752-10.445,13.99v1.281H72.6v-3.053c0-4.46,7.941-7.024,7.941-11.973a4.008,4.008,0,0,0-4.032-4.275c-3.3,0-3.97,2.564-3.97,5.13a14.7,14.7,0,0,0,.062,1.649H61.365c-.062-.8-.183-1.525-.183-2.321,0-7.941,7.085-12.888,16.675-12.888C87.2,257,93.494,261.887,93.494,269.459Zm-9.528,30.3h-12.1v-11.3h12.1Z"
									transform="translate(-61.182 -256.999)"
									fill="#fff"
								/>
							</g>
							<g transform="translate(129.697 338.818)">
								<circle cx="3.528" cy="3.528" r="3.528" transform="translate(0 0)" fill="#fff" />
								<circle
									cx="3.528"
									cy="3.528"
									r="3.528"
									transform="translate(6.118 0)"
									fill="#fff"
								/>
								<circle
									cx="3.528"
									cy="3.528"
									r="3.528"
									transform="translate(12.236 0)"
									fill="#fff"
								/>
							</g>
						</g>
					</svg>
				</div>
				<p class="support-copy">
					As always, should you have questions or need assistance, please contact our dedicated
					support team at <a href="mailto:support@revolutiontradingpros.com"
						>support@revolutiontradingpros.com</a
					> or call 512-266-8659.
				</p>
			</div>

			<!-- 3 Card Grid Layout: 2 cards top, 1 centered bottom -->
			<div class="start-here-cards">
				<!-- Card 1: Charts & Tools -->
				<div class="start-card">
					<div class="start-card__header">
						<h3 class="start-card__title">Charts, Brokers, & Tools</h3>
					</div>
					<div class="start-card__body">
						<p>
							Part of being set up for success with trading is having the right tools for the job.
							There's a wide range here and it can become more to manage than what's really needed.
							However, there are some basic considerations for every trader to ensure you're
							stepping into the world of SPX Profit Pulse appropriately equipped.
						</p>
						<ul>
							<li>Options trading approval (Level 2 or higher)</li>
							<li>Access to SPX index options</li>
							<li>Real-time market data</li>
							<li>Fast order execution platform</li>
						</ul>
					</div>
					<div class="start-card__footer">
						<a href="/dashboard/spx-profit-pulse/learning-center" class="start-card__link"
							>Learn More →</a
						>
					</div>
				</div>

				<!-- Card 2: Alerts & Notifications -->
				<div class="start-card">
					<div class="start-card__header">
						<h3 class="start-card__title">Setting Up Your Alerts</h3>
					</div>
					<div class="start-card__body">
						<p>
							To ensure you receive all trade alerts in real-time, configure your notification
							settings properly. We recommend enabling both SMS and email alerts to ensure you never
							miss a trade opportunity.
						</p>
						<ul>
							<li>Enable SMS alerts for instant notifications</li>
							<li>Configure email notifications</li>
							<li>Add alerts@revolutiontradingpros.com to contacts</li>
							<li>Test your notification settings</li>
						</ul>
					</div>
					<div class="start-card__footer">
						<a href="/dashboard/account" class="start-card__link">Configure Alerts →</a>
					</div>
				</div>

				<!-- Card 3: Understanding 0DTE (Centered Bottom) -->
				<div class="start-card start-card--centered">
					<div class="start-card__header">
						<h3 class="start-card__title">Understanding 0DTE Trading</h3>
					</div>
					<div class="start-card__body">
						<p>
							0DTE options trading involves buying and selling options contracts that expire on the
							same day. This strategy offers unique opportunities but requires careful risk
							management and understanding of key concepts.
						</p>
						<ul>
							<li>
								<strong>Time Decay:</strong> Options lose value rapidly as expiration approaches
							</li>
							<li><strong>High Leverage:</strong> Small price movements = significant results</li>
							<li><strong>Defined Risk:</strong> Maximum loss limited to premium paid</li>
							<li><strong>Quick Decisions:</strong> Trades last minutes to hours</li>
						</ul>
					</div>
					<div class="start-card__footer">
						<a href="/dashboard/spx-profit-pulse/learning-center" class="start-card__link"
							>View Resources →</a
						>
					</div>
				</div>
			</div>
		</section>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * START HERE CARDS - 2-1 Grid Layout (Responsive)
	 * 2 cards on top row, 1 centered card on bottom
	 * Svelte 5 + SvelteKit Best Practices (Nov/Dec 2025)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.start-here-cards {
		display: grid;
		grid-template-columns: 1fr;
		gap: 30px;
		margin: 40px 0;
	}

	/* Tablet: 2 columns */
	@media (min-width: 768px) {
		.start-here-cards {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	/* Desktop: 2-1 layout */
	@media (min-width: 1024px) {
		.start-here-cards {
			gap: 40px;
		}
	}

	/* Card Styles */
	.start-card {
		background: #ffffff;
		border-radius: 12px;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
		display: flex;
		flex-direction: column;
		overflow: hidden;
		border: 1px solid #e0e0e0;
	}

	.start-card:hover {
		box-shadow: 0 8px 24px rgba(20, 62, 89, 0.15);
		transform: translateY(-4px);
		border-color: #143e59;
	}

	/* Centered bottom card spans 2 columns on desktop */
	@media (min-width: 768px) {
		.start-card--centered {
			grid-column: 1 / -1;
			max-width: 600px;
			margin: 0 auto;
			width: 100%;
		}
	}

	.start-card__header {
		background: linear-gradient(135deg, #143e59 0%, #0f2d41 100%);
		padding: 30px;
		text-align: center;
	}

	.start-card__title {
		color: #ffffff;
		font-size: 1.75rem;
		font-weight: 700;
		margin: 0;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.3;
	}

	@media (min-width: 768px) {
		.start-card__title {
			font-size: 2rem;
		}
	}

	.start-card__body {
		padding: 30px;
		flex: 1;
	}

	.start-card__body p {
		font-size: 1.05rem;
		line-height: 1.7;
		color: #555;
		margin-bottom: 20px;
	}

	.start-card__body ul {
		list-style: none;
		padding: 0;
		margin: 20px 0 0 0;
	}

	.start-card__body li {
		position: relative;
		padding-left: 28px;
		margin-bottom: 12px;
		font-size: 1rem;
		line-height: 1.6;
		color: #666;
	}

	.start-card__body li::before {
		content: '✓';
		position: absolute;
		left: 0;
		color: #143e59;
		font-weight: 700;
		font-size: 1.1rem;
	}

	.start-card__body li strong {
		color: #333;
		font-weight: 600;
	}

	.start-card__footer {
		padding: 0 30px 30px;
		text-align: center;
	}

	.start-card__link {
		display: inline-block;
		background: #143e59;
		color: #ffffff !important;
		padding: 10px 16px;
		border-radius: 4px;
		text-decoration: none;
		font-weight: 600;
		font-size: 13px;
		font-family: 'Open Sans', sans-serif;
		transition: all 0.2s ease;
		border: 1px solid #143e59;
		min-height: 44px;
		line-height: 1.5;
	}

	.start-card__link:hover {
		background: #0c2638;
		border-color: #0c2638;
		color: #ffffff !important;
	}

	/* Welcome Heading */
	.page-heading {
		text-align: center;
		margin-bottom: 30px;
	}

	.page-heading__text {
		font-size: 2rem;
		font-weight: 400;
		color: #666666;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.3;
	}

	@media (min-width: 768px) {
		.page-heading__text {
			font-size: 2.5rem;
		}
	}

	/* Intro Content */
	.intro-content {
		margin-bottom: 30px;
	}

	.intro-content p {
		font-size: 1rem;
		line-height: 1.8;
		color: #676767;
		margin-bottom: 15px;
	}

	/* Contact Support Section */
	.contact-support--container {
		display: flex;
		align-items: center;
		gap: 20px;
		background: #f8f9fa;
		border-radius: 8px;
		padding: 25px 30px;
		margin: 30px 0;
	}

	.contact-support--container .img {
		flex-shrink: 0;
	}

	.contact-support--container .img svg {
		width: 100px;
		height: auto;
	}

	.support-copy {
		margin: 0;
		font-size: 1rem;
		line-height: 1.6;
		color: #555;
	}

	.support-copy a {
		color: #3879f3;
		text-decoration: none;
		font-weight: 600;
	}

	.support-copy a:hover {
		text-decoration: underline;
	}

	@media (max-width: 640px) {
		.contact-support--container {
			flex-direction: column;
			text-align: center;
		}
	}

	/* Dashboard Content */
	.dashboard__content {
		display: flex;
		flex-flow: row nowrap;
	}

	.dashboard__content-main {
		border-right: 1px solid #dbdbdb;
		flex: 1 1 auto;
		min-width: 0;
		background: #fff;
	}

	.dashboard__content-section {
		padding: 30px 20px;
	}

	@media screen and (min-width: 1024px) {
		.dashboard__content-section {
			padding: 30px;
		}
	}

	@media screen and (min-width: 1440px) {
		.dashboard__content-section {
			padding: 40px;
		}
	}
</style>

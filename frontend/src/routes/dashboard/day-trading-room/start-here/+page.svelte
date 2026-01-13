<!--
	URL: /dashboard/day-trading-room/start-here
	
	Getting Started with Day Trading Room - Pixel Perfect Match
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation
	Based on NewStartHere reference - Svelte 5
	
	@version 2.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	import { IconChevronRight } from '$lib/icons';
	import WeeklyWatchlist from '$lib/components/dashboard/WeeklyWatchlist.svelte';
	import type { PageData } from './$types';

	// Svelte 5 props with proper typing
	interface Props {
		data: PageData;
	}

	let { data }: Props = $props();

	// Accordion state management - Svelte 5 runes
	let openAccordions = $state<Set<number>>(new Set());

	// Toggle accordion with Set reassignment for reactivity
	function toggleAccordion(index: number): void {
		const updated = new Set(openAccordions);
		if (updated.has(index)) {
			updated.delete(index);
		} else {
			updated.add(index);
		}
		openAccordions = updated;
	}

	// Check if accordion is open
	function isOpen(index: number): boolean {
		return openAccordions.has(index);
	}

	// Trader team data - matches reference HTML
	const traders = [
		{ name: 'Billy Ribeiro', title: 'Founder of Revolution Trading Pros', slug: 'billy-ribeiro', image: 'https://s3.amazonaws.com/simpler-cdn/dev/wp-content/uploads/2020/06/23145036/John-Carter-1-300x300.jpg' },
		{ name: 'Freddie Ferber', title: 'Senior Managing Director of Options Trading', slug: 'freddie-ferber', image: 'https://s3.amazonaws.com/simpler-cdn/dev/wp-content/uploads/2020/06/23145751/Henry-Gambell-300x300.jpg' },
		{ name: 'Shao Wan', title: 'Director of Options and Income Trading', slug: 'shao-wan', image: 'https://s3.amazonaws.com/simpler-cdn/dev/wp-content/uploads/2020/06/23145841/Bruce-Marhsall-300x300.jpg' }
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
			<!-- Welcome Heading -->
			<h2 class="fl-heading">
				<span class="fl-heading-text">Welcome to your Revolution Trading Pros Day Trading Room Membership</span>
			</h2>

			<!-- Intro Text -->
			<div class="fl-rich-text intro-text">
				<p>On this "Start Here" page, we've outlined what we believe are the key steps to get you started on a good path with us. Please take some time to go through this page carefully and "check all of the boxes" to ensure you're set-up for success.</p>
				<p>The <strong>Getting Started Checklist</strong> below is essential. Regardless of if you're new to trading or are a seasoned veteran, we want to help you navigate your way through the Revolution Trading environment in the most efficient way possible.</p>
			</div>

			<!-- Contact Support Section -->
			<div class="contact-support--container">
				<div class="img">
					<svg xmlns="http://www.w3.org/2000/svg" width="159.102" height="100" viewBox="0 0 159.102 146.793">
						<g transform="translate(-28.444 -251.482)">
							<path d="M131.267,370.888l-21.211-27.476a1.148,1.148,0,0,0-1.606-.207L97.582,351.6a1.144,1.144,0,0,0-.207,1.606l.195.252c-7.58-.663-21.246-10.162-29.3-20.427-7.893-10.393-13.624-26.02-12.346-33.519l.195.252a1.148,1.148,0,0,0,1.606.207l10.868-8.392a1.144,1.144,0,0,0,.207-1.606L47.583,262.494a1.143,1.143,0,0,0-1.6-.207l-10.877,8.4a1.13,1.13,0,0,0-.207,1.587l.007.007c-5.759,5.988-7.708,12.945-5.681,22.805,1.012,4.925,3.574,24.651,23.188,50.155C72.118,370.674,90.578,378.178,95.09,380.4c9.027,4.457,16.247,4.336,23.5.278l.007.007a1.13,1.13,0,0,0,1.59.2l10.877-8.4A1.144,1.144,0,0,0,131.267,370.888Z" transform="translate(0 14.54)" fill="#505050"/>
							<path d="M97.6,272.17c20.115,0,36.42,13,36.42,29.028a24.069,24.069,0,0,1-3.255,11.992c3.84,11.089,11.728,14.575,11.728,14.575-13.227.737-18.272-3.184-20.037-5.363A41.7,41.7,0,0,1,97.6,330.224c-20.118,0-36.422-13-36.422-29.026S77.486,272.17,97.6,272.17Z" transform="translate(45.049 28.467)" fill="#ffb412"/>
							<path d="M101.176,251.482c-25.473,0-46.126,16.459-46.126,36.76a30.482,30.482,0,0,0,4.125,15.19c-4.864,14.04-14.853,18.457-14.853,18.457,16.751.934,23.138-4.03,25.376-6.791C77.94,321.237,89,325,101.176,325c25.476,0,46.126-16.456,46.126-36.76S126.651,251.482,101.176,251.482Z" transform="translate(21.849 0)" fill="#53c2c9"/>
							<g transform="translate(106.231 264.591)">
								<path d="M93.494,269.459c0,8.307-10.445,10.752-10.445,13.99v1.281H72.6v-3.053c0-4.46,7.941-7.024,7.941-11.973a4.008,4.008,0,0,0-4.032-4.275c-3.3,0-3.97,2.564-3.97,5.13a14.7,14.7,0,0,0,.062,1.649H61.365c-.062-.8-.183-1.525-.183-2.321,0-7.941,7.085-12.888,16.675-12.888C87.2,257,93.494,261.887,93.494,269.459Zm-9.528,30.3h-12.1v-11.3h12.1Z" transform="translate(-61.182 -256.999)" fill="#fff"/>
							</g>
							<g transform="translate(129.697 338.818)">
								<circle cx="3.528" cy="3.528" r="3.528" transform="translate(0 0)" fill="#fff"/>
								<circle cx="3.528" cy="3.528" r="3.528" transform="translate(6.118 0)" fill="#fff"/>
								<circle cx="3.528" cy="3.528" r="3.528" transform="translate(12.236 0)" fill="#fff"/>
							</g>
						</g>
					</svg>
				</div>
				<p class="support-copy">As always, should you have questions or need assistance, please contact our dedicated support team at <a href="mailto:support@revolutiontradingpros.com">support@revolutiontradingpros.com</a> or call 512-266-8659.</p>
			</div>

			<!-- Click + Intro -->
			<div class="intro-container">
				Click <svg xmlns="http://www.w3.org/2000/svg" width="40" height="40" viewBox="0 0 123 123">
					<g transform="translate(-1632 -1126)">
						<circle cx="61.5" cy="61.5" r="61.5" transform="translate(1632 1126)" fill="#3879f3"/>
						<g transform="translate(1665.677 1159.678)">
							<path d="M132.94,149.955H112.434V138.347H132.94V117.688h11.6v20.659H165.05v11.608H144.544v20.4h-11.6Z" transform="translate(-112.434 -117.688)" fill="#fff"/>
						</g>
					</g>
				</svg>
				below to learn more about the features included in your Day Trading Room Membership:
			</div>

			<!-- Accordion Sections -->
			<div id="accordion_options" class="fl-accordion fl-accordion-large">
				<!-- Accordion 1: Trading Plan -->
				<div class="fl-accordion-item" class:is-open={isOpen(0)}>
					<button 
						class="fl-accordion-button"
						onclick={() => toggleAccordion(0)}
						aria-expanded={isOpen(0)}
					>
						<span class="fl-accordion-button-label">
							Trading Plan 
							<span class="accordion-subtitle"><em>As you prepare to dive into this service and trade with us, one of the most crucial steps to getting started is taking time to look at your trading plan.</em></span>
						</span>
						<span class="fl-accordion-button-icon fl-accordion-button-icon-right">
							{#if isOpen(0)}
								<i class="fas fa-times-circle"></i>
							{:else}
								<i class="fas fa-plus-circle"></i>
							{/if}
						</span>
					</button>
					{#if isOpen(0)}
						<div class="fl-accordion-content">
							<p>First off, do you have one? If you do, great! It's a great time to re-examine this and remind yourself of your goals, your rules, and what you most hope to get out of working with Revolution Trading. Be sure to update your plan accordingly and document anything new that pops up.</p>
							<p>If you don't have a trading plan (or if you do and you'd like to change it up a bit), we have a few tools and resources to help you craft a plan that will help your focus and determination with trading.</p>
							<p><strong>Below is an example of one of our Trader's Plans:</strong></p>
							<hr />
							<p><em><strong>Why am I trading?</strong></em></p>
							<ul>
								<li>To generate long term wealth</li>
								<li>To gain technical competence in the market</li>
								<li>To have a better understanding of how financial markets work</li>
								<li>To create a consistent source of income through trading</li>
							</ul>
							<p><em><strong>What is my approach?</strong></em></p>
							<ul>
								<li><strong>Basic Swing Trades</strong> - I'll have a technical approach, establishing bullish positions in symbols with a daily Squeeze near the mean (21 EMA Daily) attempting to maintain a theta positive portfolio.</li>
							</ul>
							<p><em><strong>What are my goals?</strong></em></p>
							<ul>
								<li>Weekly, static wire of $987 ($51,324 annually)</li>
								<li>Continued compound growth</li>
								<li>Withdraw 1/2 my remaining trading profits on DEC 31</li>
							</ul>
							<p><a class="download-link" href="/trading-plan-template.docx" target="_blank">CLICK HERE to Download a blank trading plan to fill out for yourself</a></p>
						</div>
					{/if}
				</div>

				<!-- Accordion 2: Charts, Brokers & Tools -->
				<div class="fl-accordion-item" class:is-open={isOpen(1)}>
					<button 
						class="fl-accordion-button"
						onclick={() => toggleAccordion(1)}
						aria-expanded={isOpen(1)}
					>
						<span class="fl-accordion-button-label">
							Charts, Brokers, & Tools 
							<span class="accordion-subtitle"><em>Part of being set-up for success with trading is having the right tools for the job. This section has some helpful tips to get started.</em></span>
						</span>
						<span class="fl-accordion-button-icon fl-accordion-button-icon-right">
							{#if isOpen(1)}
								<i class="fas fa-times-circle"></i>
							{:else}
								<i class="fas fa-plus-circle"></i>
							{/if}
						</span>
					</button>
					{#if isOpen(1)}
						<div class="fl-accordion-content">
							<p>There's a wide-range here and it can become more to manage than what's really needed. However, there are some basic considerations for every trader to ensure you're stepping into the world of trading appropriately equipped.</p>
							<p>Are you set-up with a broker and have negotiated a rate for your commissions that's in line with the industry standard and/or that you're happy with?</p>
							<p>Here's what we use and recommend:</p>
							
							<div class="charts-columns">
								<div class="image-column">
									<img src="https://s3.amazonaws.com/simpler-cdn/dev/wp-content/uploads/2020/06/15142240/thinkorswim.png" alt="ThinkorSwim" width="184" height="71" />
								</div>
								<div class="copy-column">
									<strong>ThinkorSwim by TD Ameritrade/Schwab</strong><br />
									The majority of our team uses TOS and of all the charting softwares/platforms that you'll see here, you'll likely see this used the most. TOS provides a charting software and brokerage platform in one. It's very robust, sophisticated, and considered one of the best solutions for traders of all kinds.
								</div>
							</div>
							<hr />
							<div class="charts-columns">
								<div class="image-column">
									<img src="https://cdn.simplertrading.com/2023/05/02114224/tastytrade2023-300x55.png" alt="tastytrade" width="200" height="37" />
								</div>
								<div class="copy-column">
									<strong>tastytrade</strong><br />
									From the creators of TOS and the popular Financial Network. Provides users with the best when it comes to pricing, technology, and inspiration. It's fast, it's reliable, and they pioneered the zero commissions and low fees movement.
								</div>
							</div>
							<hr />
							<div class="charts-columns">
								<div class="image-column">
									<img src="https://s3.amazonaws.com/simpler-cdn/dev/wp-content/uploads/2020/07/09154047/tradestation.png" alt="TradeStation" width="184" height="71" />
								</div>
								<div class="copy-column">
									<strong>TradeStation</strong><br />
									This platform offers some of, if not the best solution when it comes to its charting features. You're also able to execute trades here.
								</div>
							</div>
						</div>
					{/if}
				</div>

				<!-- Accordion 3: Get to Know Our Trading Team -->
				<div class="fl-accordion-item" class:is-open={isOpen(2)}>
					<button 
						class="fl-accordion-button"
						onclick={() => toggleAccordion(2)}
						aria-expanded={isOpen(2)}
					>
						<span class="fl-accordion-button-label">
							Get to Know Revolution Trading & Our Trading Team 
							<span class="accordion-subtitle"><em>Here you'll find everything from the Traders' trading plans, favorite strategies & setups, what's on their charts, their recommended courses and videos, and more.</em></span>
						</span>
						<span class="fl-accordion-button-icon fl-accordion-button-icon-right">
							{#if isOpen(2)}
								<i class="fas fa-times-circle"></i>
							{:else}
								<i class="fas fa-plus-circle"></i>
							{/if}
						</span>
					</button>
					{#if isOpen(2)}
						<div class="fl-accordion-content">
							<p>If you haven't already watched the welcome video that walks through everything you get to enjoy as part of this service, please make time to do so.</p>
							<p>Below you'll find links to each of our Traders Bio Pages. Inside their pages you'll find everything from their Trading Plans, Favorite Strategies & Setups, what's on their charts, recommended courses and videos, and more.</p>
							
							<div class="trader-team--container">
								{#each traders as trader}
									<div class="team_member">
										<a href="/traders/{trader.slug}">
											<img src={trader.image} alt={trader.name} />
										</a>
										<div class="trader-info">
											<h3>
												<a href="/traders/{trader.slug}">
													{trader.name} <IconChevronRight size={16} />
												</a>
											</h3>
											<p>{trader.title}</p>
										</div>
									</div>
								{/each}
							</div>
						</div>
					{/if}
				</div>

				<!-- Accordion 4: Trade Alerts -->
				<div class="fl-accordion-item" class:is-open={isOpen(3)}>
					<button 
						class="fl-accordion-button"
						onclick={() => toggleAccordion(3)}
						aria-expanded={isOpen(3)}
					>
						<span class="fl-accordion-button-label">
							Make Sure You're Signed Up for Trade Alerts 
							<span class="accordion-subtitle"><em>Trade Alerts are sent out by various members of the Trading Team during their Live Trading Sessions. You'll want to be sure to sign up for these powerful alerts.</em></span>
						</span>
						<span class="fl-accordion-button-icon fl-accordion-button-icon-right">
							{#if isOpen(3)}
								<i class="fas fa-times-circle"></i>
							{:else}
								<i class="fas fa-plus-circle"></i>
							{/if}
						</span>
					</button>
					{#if isOpen(3)}
						<div class="fl-accordion-content">
							<div class="trader-alerts--container">
								<div class="alerts-copy">
									<p>The trades are posted in the room and simultaneously delivered through our App via push notifications. Our <a href="https://intercom.help/simpler-trading/en/?q=mobile+app" target="_blank" rel="noopener">Support Section</a> has some great articles about the app.</p>
									<p><strong>Follow these steps to download our app:</strong></p>
									<ol class="app-steps">
										<li>Navigate to your App Store (<a href="https://apps.apple.com/us/developer/simpler-options/id663042245" target="_blank" rel="noopener">Apple</a> or <a href="https://play.google.com/store/apps/details?id=com.bellesoft.stprotradingroom&hl=en" target="_blank" rel="noopener">Android</a>) Search for "Simpler Trading Mobile" and then download the app.
											<div class="mobile-app-buttons">
												<a href="https://apps.apple.com/us/developer/simpler-options/id663042245" target="_blank" rel="noopener">
													<img src="https://s3.amazonaws.com/simpler-cdn/dev/wp-content/uploads/2020/06/25152618/apple-store.png" alt="Apple App Store" width="149" height="46" />
												</a>
												<a href="https://play.google.com/store/apps/details?id=com.bellesoft.stprotradingroom&hl=en" target="_blank" rel="noopener">
													<img src="https://s3.amazonaws.com/simpler-cdn/dev/wp-content/uploads/2020/06/25152633/google-play.png" alt="Google Play Store" width="149" height="47" />
												</a>
											</div>
										</li>
										<li>Once downloaded, login using the same username (email) and password as you have for the site.</li>
										<li>Next, you'll login to the Trading Room you have access to, and from there, you'll be enabled for push notifications with the trade alerts.</li>
									</ol>
								</div>
								<div class="alerts-image">
									<img src="https://s3.amazonaws.com/simpler-cdn/dev/wp-content/uploads/2020/06/17115251/trade-alerts-phone.png" alt="Trade Alerts Phone" width="240" height="297" />
									<div class="app-images">
										<a href="https://apps.apple.com/us/developer/simpler-options/id663042245" target="_blank" rel="noopener">
											<img src="https://s3.amazonaws.com/simpler-cdn/dev/wp-content/uploads/2020/06/25152618/apple-store.png" alt="Apple App Store" width="149" height="46" />
										</a>
										<a href="https://play.google.com/store/apps/details?id=com.bellesoft.stprotradingroom&hl=en" target="_blank" rel="noopener">
											<img src="https://s3.amazonaws.com/simpler-cdn/dev/wp-content/uploads/2020/06/25152633/google-play.png" alt="Google Play Store" width="149" height="47" />
										</a>
									</div>
								</div>
							</div>
							<p class="alerts-note">If you need help interpreting the trade alerts, please check out this <a href="https://www.simplertrading.com/entering-trades-using-trade-alerts-henry-gambell/" target="_blank" rel="noopener">blog post</a>. Keep in mind our traders take trades appropriate for their personal accounts. It is not intended for you to blindly copy trades. You must decide for yourself which trades are appropriate for your trading plan and risk appetite.</p>
						</div>
					{/if}
				</div>

				<!-- Accordion 5: Explore the Live Trading Room -->
				<div class="fl-accordion-item" class:is-open={isOpen(4)}>
					<button 
						class="fl-accordion-button"
						onclick={() => toggleAccordion(4)}
						aria-expanded={isOpen(4)}
					>
						<span class="fl-accordion-button-label">
							Explore the Live Trading Room 
							<span class="accordion-subtitle"><em>The Live Trading Room provides an excellent platform to connect, collaborate, and build consistency in a live trading environment.</em></span>
						</span>
						<span class="fl-accordion-button-icon fl-accordion-button-icon-right">
							{#if isOpen(4)}
								<i class="fas fa-times-circle"></i>
							{:else}
								<i class="fas fa-plus-circle"></i>
							{/if}
						</span>
					</button>
					{#if isOpen(4)}
						<div class="fl-accordion-content">
							<p>Check out the schedule for each day/week which is posted just below the login to the room on your Day Trading Room Member Dashboard:</p>
							<p>
								<img src="https://cdn.simplertrading.com/2025/05/01151648/SCR-20250501-mrti.png" alt="Trading Room Schedule" class="schedule-screenshot" />
							</p>
							<p>In the Live Trading Room you'll find a unique community of traders that offer support & guidance. It also provides the opportunity to look over the shoulder of seasoned traders who are trading real accounts in live market conditions. It allows you to better understand the thought process behind the setups and in doing so, better apply these trading ideas into your trading plan.</p>
							<p>And, if you aren't able to be in the room throughout the day or happen to miss a session that you really wanted to see, we've got you covered. You can find a complete recording of the trading room each day in our <a href="/dashboard/day-trading-room/trading-room-archive">Trading Room Archive section HERE</a>.</p>
						</div>
					{/if}
				</div>

				<!-- Accordion 6: Additional Member Resources -->
				<div class="fl-accordion-item" class:is-open={isOpen(5)}>
					<button 
						class="fl-accordion-button"
						onclick={() => toggleAccordion(5)}
						aria-expanded={isOpen(5)}
					>
						<span class="fl-accordion-button-label">
							Additional Member Resources 
							<span class="accordion-subtitle"><em>These links are helpful when trying to learn some of the internal Simpler Language as well as find articles on a wide array of technical information.</em></span>
						</span>
						<span class="fl-accordion-button-icon fl-accordion-button-icon-right">
							{#if isOpen(5)}
								<i class="fas fa-times-circle"></i>
							{:else}
								<i class="fas fa-plus-circle"></i>
							{/if}
						</span>
					</button>
					{#if isOpen(5)}
						<div class="fl-accordion-content">
							<p><strong>Simpler Terms</strong><br />
							A big part of learning how to trade or acclimating to any new service is understanding the jargon used. This page will help you understand the terminology used by our Traders.</p>
							<p><a class="resource-link" href="https://www.simplertrading.com/simpler-terms/" target="_blank" rel="noopener"><strong>Check out the Simpler Terms here</strong></a></p>
							
							<p><strong>Support Knowledge Base</strong><br />
							For questions and definitions on everything from Indicators to Chart Patterns, Studies and more, please make sure to check out the Support Knowledge Base.</p>
							<p><a class="resource-link" href="https://intercom.help/simpler-trading/en/" target="_blank" rel="noopener"><strong>Visit the Support Knowledge base</strong></a></p>
						</div>
					{/if}
				</div>
			</div>
		</section>
	</div>
</div>

<!-- Weekly Watchlist Section - SSR pre-fetched for 0ms loading -->
<div class="dashboard__content-section u--background-color-white">
	<WeeklyWatchlist data={(data as { watchlist?: any }).watchlist} />
</div>

<style>
	/* Welcome Heading */
	.fl-heading {
		text-align: center;
		margin-bottom: 30px;
	}

	.fl-heading-text {
		font-size: 2rem;
		font-weight: 400;
		color: #666666;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.3;
	}

	@media (min-width: 768px) {
		.fl-heading-text {
			font-size: 2.5rem;
		}
	}

	/* Intro Text */
	.intro-text {
		margin-bottom: 30px;
	}

	.intro-text p {
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
		color: #3879F3;
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

	/* Click + Intro */
	.intro-container {
		display: flex;
		align-items: center;
		gap: 10px;
		font-size: 1.1rem;
		color: #676767;
		margin: 30px 0;
		flex-wrap: wrap;
	}

	.intro-container svg {
		flex-shrink: 0;
	}

	/* Accordion Styles */
	.fl-accordion {
		margin-top: 20px;
	}

	.fl-accordion-item {
		background: #ffffff;
		border: 1px solid #e0e0e0;
		border-radius: 8px;
		margin-bottom: 15px;
		overflow: hidden;
	}

	.fl-accordion-item.is-open {
		border-color: #3879F3;
	}

	.fl-accordion-button {
		width: 100%;
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		padding: 20px 25px;
		background: #f8f9fa;
		border: none;
		cursor: pointer;
		text-align: left;
		transition: background 0.2s ease;
	}

	.fl-accordion-button:hover {
		background: #f0f0f0;
	}

	.fl-accordion-item.is-open .fl-accordion-button {
		background: #e8f4fd;
	}

	.fl-accordion-button-label {
		flex: 1;
		font-size: 1.25rem;
		font-weight: 600;
		color: #333;
		font-family: 'Open Sans', sans-serif;
	}

	.accordion-subtitle {
		display: block;
		font-size: 1rem;
		font-weight: 400;
		color: #666;
		margin-top: 8px;
	}

	.fl-accordion-button-icon {
		flex-shrink: 0;
		margin-left: 15px;
		margin-top: 3px;
	}

	.fl-accordion-button-icon-right {
		color: #fdac3b;
		font-size: 1.5rem;
	}

	.fl-accordion-button-icon-right i {
		color: #fdac3b;
	}

	.fl-accordion-content {
		padding: 25px;
		border-top: 1px solid #e0e0e0;
	}

	.fl-accordion-content p {
		font-size: 1rem;
		line-height: 1.8;
		color: #555;
		margin-bottom: 15px;
	}

	.fl-accordion-content ul,
	.fl-accordion-content ol {
		margin: 15px 0;
		padding-left: 25px;
	}

	.fl-accordion-content li {
		font-size: 1rem;
		line-height: 1.8;
		color: #555;
		margin-bottom: 8px;
	}

	.fl-accordion-content hr {
		border: none;
		border-top: 1px solid #e0e0e0;
		margin: 20px 0;
	}

	.fl-accordion-content a {
		color: #3879F3;
		text-decoration: none;
	}

	.fl-accordion-content a:hover {
		text-decoration: underline;
	}

	.download-link,
	.resource-link {
		color: #ffb412 !important;
		text-decoration: underline !important;
		font-weight: 600;
	}

	/* Charts Columns */
	.charts-columns {
		display: flex;
		align-items: flex-start;
		gap: 20px;
		margin: 20px 0;
	}

	.image-column {
		flex-shrink: 0;
		width: 200px;
	}

	.image-column img {
		max-width: 100%;
		height: auto;
	}

	.copy-column {
		flex: 1;
		font-size: 1rem;
		line-height: 1.6;
		color: #676767;
	}

	.copy-column strong {
		color: #555;
	}

	@media (max-width: 640px) {
		.charts-columns {
			flex-direction: column;
		}

		.image-column {
			width: 100%;
			text-align: center;
		}
	}

	/* Trader Team Grid */
	.trader-team--container {
		display: grid;
		grid-template-columns: repeat(1, 1fr);
		gap: 20px;
		margin-top: 30px;
	}

	@media (min-width: 640px) {
		.trader-team--container {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (min-width: 1024px) {
		.trader-team--container {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	.team_member {
		position: relative;
		overflow: hidden;
		border-radius: 8px;
	}

	.team_member img {
		width: 100%;
		height: auto;
		display: block;
		filter: grayscale(100%);
		transition: filter 0.3s ease;
	}

	.team_member:hover img {
		filter: grayscale(0%);
	}

	.trader-info {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		background: linear-gradient(transparent, rgba(0,0,0,0.8));
		padding: 20px;
		color: #fff;
	}

	.trader-info h3 {
		margin: 0 0 5px;
		font-size: 1.25rem;
		font-weight: 700;
	}

	.trader-info h3 a {
		color: #fff;
		text-decoration: none;
		display: flex;
		align-items: center;
		gap: 5px;
	}

	.trader-info p {
		margin: 0;
		font-size: 0.875rem;
		opacity: 0.9;
	}

	/* Trade Alerts Section */
	.trader-alerts--container {
		display: flex;
		gap: 30px;
		align-items: flex-start;
		flex-wrap: wrap;
	}

	.alerts-copy {
		flex: 1;
		min-width: 300px;
	}

	.alerts-image {
		flex-shrink: 0;
		text-align: center;
	}

	.alerts-image img {
		max-width: 240px;
		height: auto;
	}

	.app-images {
		margin-top: 15px;
		display: flex;
		gap: 10px;
		justify-content: center;
	}

	.app-images img {
		height: 46px;
		width: auto;
	}

	.mobile-app-buttons {
		display: flex;
		gap: 10px;
		margin-top: 15px;
		margin-bottom: 15px;
	}

	.mobile-app-buttons img {
		height: 46px;
		width: auto;
	}

	.app-steps {
		list-style-type: decimal;
	}

	.app-steps li {
		margin: 20px 0;
		text-align: left;
	}

	.alerts-note {
		margin-top: 20px;
		padding-top: 15px;
		border-top: 1px solid #e0e0e0;
	}

	@media (max-width: 768px) {
		.trader-alerts--container {
			flex-direction: column;
		}

		.alerts-image {
			width: 100%;
		}
	}

	/* Schedule Screenshot */
	.schedule-screenshot {
		max-width: 100%;
		height: auto;
		border-radius: 8px;
		box-shadow: 0 2px 8px rgba(0,0,0,0.1);
	}

	/* Weekly Watchlist Section */
	.u--background-color-white {
		background: #fff;
		padding: 40px 20px;
		margin-top: 30px;
		border-radius: 8px;
	}
</style>

<!--
	URL: /dashboard/spx-profit-pulse/start-here
	
	Getting Started with SPX Profit Pulse - Pixel Perfect Match
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation
	Based on Day Trading Room Start Here pattern - Svelte 5
	
	@version 1.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	import { IconPlus, IconMinus, IconChevronRight } from '$lib/icons';

	// Accordion state management - Svelte 5 runes
	let openAccordions = $state<Set<number>>(new Set());

	function toggleAccordion(index: number) {
		if (openAccordions.has(index)) {
			openAccordions.delete(index);
			openAccordions = new Set(openAccordions);
		} else {
			openAccordions.add(index);
			openAccordions = new Set(openAccordions);
		}
	}

	function isOpen(index: number): boolean {
		return openAccordions.has(index);
	}
</script>

<svelte:head>
	<title>Getting Started with SPX Profit Pulse | Revolution Trading Pros</title>
	<meta name="description" content="Welcome to your Revolution Trading Pros SPX Profit Pulse Membership. Get started on the right path with our comprehensive getting started checklist." />
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<!-- Welcome Heading -->
			<h2 class="fl-heading">
				<span class="fl-heading-text">Welcome to your Revolution Trading Pros SPX Profit Pulse Membership</span>
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
				below to learn more about the features included in your SPX Profit Pulse Membership:
			</div>

			<!-- Accordion Sections -->
			<div id="accordion_options" class="fl-accordion fl-accordion-large">
				<!-- Accordion 1: What is SPX Profit Pulse -->
				<div class="fl-accordion-item" class:is-open={isOpen(0)}>
					<button 
						class="fl-accordion-button"
						onclick={() => toggleAccordion(0)}
						aria-expanded={isOpen(0)}
					>
						<span class="fl-accordion-button-label">
							What is SPX Profit Pulse? 
							<span class="accordion-subtitle"><em>Learn about our 0DTE SPX options trading alert service and how it works.</em></span>
						</span>
						<span class="fl-accordion-button-icon">
							{#if isOpen(0)}
								<IconMinus size={20} />
							{:else}
								<IconPlus size={20} />
							{/if}
						</span>
					</button>
					{#if isOpen(0)}
						<div class="fl-accordion-content">
							<p>SPX Profit Pulse is our premium alert service focused exclusively on 0DTE (Zero Days to Expiration) options trading on the S&P 500 Index (SPX).</p>
							<p><strong>What You Get:</strong></p>
							<ul>
								<li>Real-time trade alerts sent directly to your phone and email</li>
								<li>Detailed entry and exit instructions for each trade</li>
								<li>Clear risk management guidelines</li>
								<li>Technical analysis and market commentary</li>
								<li>Performance tracking and trade history</li>
							</ul>
							<p><strong>Trading Schedule:</strong></p>
							<ul>
								<li>Monday - Friday during market hours (9:30 AM - 4:00 PM ET)</li>
								<li>Alerts sent in real-time as opportunities arise</li>
								<li>Pre-market analysis available by 8:30 AM ET</li>
							</ul>
						</div>
					{/if}
				</div>

				<!-- Accordion 2: Setting Up Alerts -->
				<div class="fl-accordion-item" class:is-open={isOpen(1)}>
					<button 
						class="fl-accordion-button"
						onclick={() => toggleAccordion(1)}
						aria-expanded={isOpen(1)}
					>
						<span class="fl-accordion-button-label">
							Setting Up Your Alerts 
							<span class="accordion-subtitle"><em>Configure your notification preferences to receive trade alerts instantly.</em></span>
						</span>
						<span class="fl-accordion-button-icon">
							{#if isOpen(1)}
								<IconMinus size={20} />
							{:else}
								<IconPlus size={20} />
							{/if}
						</span>
					</button>
					{#if isOpen(1)}
						<div class="fl-accordion-content">
							<p>To ensure you receive all trade alerts in real-time, please configure your notification settings:</p>
							<p><strong>Step 1: Enable SMS Alerts</strong></p>
							<ol>
								<li>Go to your <a href="/dashboard/account">Account Settings</a></li>
								<li>Navigate to the "Notifications" section</li>
								<li>Enter and verify your mobile phone number</li>
								<li>Enable SMS alerts for "SPX Profit Pulse"</li>
							</ol>
							<p><strong>Step 2: Enable Email Alerts</strong></p>
							<ol>
								<li>Verify your email address is correct in your account settings</li>
								<li>Enable email notifications for "SPX Profit Pulse"</li>
								<li>Add <strong>alerts@revolutiontradingpros.com</strong> to your email contacts to prevent spam filtering</li>
							</ol>
							<p><strong>Important:</strong> We recommend enabling both SMS and email alerts to ensure you never miss a trade opportunity.</p>
						</div>
					{/if}
				</div>

				<!-- Accordion 3: Understanding 0DTE Trading -->
				<div class="fl-accordion-item" class:is-open={isOpen(2)}>
					<button 
						class="fl-accordion-button"
						onclick={() => toggleAccordion(2)}
						aria-expanded={isOpen(2)}
					>
						<span class="fl-accordion-button-label">
							Understanding 0DTE Trading 
							<span class="accordion-subtitle"><em>Learn the basics of zero days to expiration options trading and risk management.</em></span>
						</span>
						<span class="fl-accordion-button-icon">
							{#if isOpen(2)}
								<IconMinus size={20} />
							{:else}
								<IconPlus size={20} />
							{/if}
						</span>
					</button>
					{#if isOpen(2)}
						<div class="fl-accordion-content">
							<p>0DTE options trading involves buying and selling options contracts that expire on the same day. This strategy offers unique opportunities but requires careful risk management.</p>
							<p><strong>Key Characteristics:</strong></p>
							<ul>
								<li><strong>Time Decay:</strong> Options lose value rapidly as expiration approaches</li>
								<li><strong>High Leverage:</strong> Small price movements can result in significant gains or losses</li>
								<li><strong>Defined Risk:</strong> Maximum loss is limited to the premium paid (for buyers)</li>
								<li><strong>Quick Decisions:</strong> Trades typically last minutes to hours, not days</li>
							</ul>
							<p><strong>Risk Management Rules:</strong></p>
							<ul>
								<li>Never risk more than 1-2% of your account on a single trade</li>
								<li>Always use stop losses as specified in alerts</li>
								<li>Don't hold positions into the final hour of trading unless instructed</li>
								<li>Start with smaller position sizes until you're comfortable</li>
							</ul>
							<p><strong>Recommended Resources:</strong></p>
							<ul>
								<li><a href="/learning-center/0dte-basics">0DTE Trading Basics Course</a></li>
								<li><a href="/learning-center/risk-management">Risk Management Guide</a></li>
								<li><a href="/learning-center/options-greeks">Understanding Options Greeks</a></li>
							</ul>
						</div>
					{/if}
				</div>

				<!-- Accordion 4: Broker Requirements -->
				<div class="fl-accordion-item" class:is-open={isOpen(3)}>
					<button 
						class="fl-accordion-button"
						onclick={() => toggleAccordion(3)}
						aria-expanded={isOpen(3)}
					>
						<span class="fl-accordion-button-label">
							Broker Requirements & Setup 
							<span class="accordion-subtitle"><em>Ensure your brokerage account is properly configured for 0DTE SPX options trading.</em></span>
						</span>
						<span class="fl-accordion-button-icon">
							{#if isOpen(3)}
								<IconMinus size={20} />
							{:else}
								<IconPlus size={20} />
							{/if}
						</span>
					</button>
					{#if isOpen(3)}
						<div class="fl-accordion-content">
							<p>To trade 0DTE SPX options, you'll need a brokerage account with specific capabilities:</p>
							<p><strong>Required Account Features:</strong></p>
							<ul>
								<li>Options trading approval (Level 2 or higher)</li>
								<li>Access to SPX index options</li>
								<li>Real-time market data</li>
								<li>Fast order execution platform</li>
							</ul>
							<p><strong>Recommended Brokers:</strong></p>
							<div class="charts-columns">
								<div class="image-column">
									<img src="https://s3.amazonaws.com/simpler-cdn/dev/wp-content/uploads/2020/06/15142240/thinkorswim.png" alt="ThinkorSwim" width="184" height="71" />
								</div>
								<div class="copy-column">
									<strong>ThinkorSwim by TD Ameritrade/Schwab</strong><br />
									Excellent platform for options trading with advanced charting and fast execution. Commission-free options trading.
								</div>
							</div>
							<hr />
							<div class="charts-columns">
								<div class="image-column">
									<img src="https://cdn.simplertrading.com/2023/05/02114224/tastytrade2023-300x55.png" alt="tastytrade" width="200" height="37" />
								</div>
								<div class="copy-column">
									<strong>tastytrade</strong><br />
									Built specifically for options traders. Low fees, fast execution, and excellent mobile app for on-the-go trading.
								</div>
							</div>
							<p><strong>Important:</strong> Make sure you have sufficient buying power in your account. We recommend having at least $5,000-$10,000 available for 0DTE trading.</p>
						</div>
					{/if}
				</div>

				<!-- Accordion 5: Additional Resources -->
				<div class="fl-accordion-item" class:is-open={isOpen(4)}>
					<button 
						class="fl-accordion-button"
						onclick={() => toggleAccordion(4)}
						aria-expanded={isOpen(4)}
					>
						<span class="fl-accordion-button-label">
							Additional Member Resources 
							<span class="accordion-subtitle"><em>Access our library of educational content, tools, and support resources.</em></span>
						</span>
						<span class="fl-accordion-button-icon">
							{#if isOpen(4)}
								<IconMinus size={20} />
							{:else}
								<IconPlus size={20} />
							{/if}
						</span>
					</button>
					{#if isOpen(4)}
						<div class="fl-accordion-content">
							<p><strong>Trading Terms Glossary</strong><br />
							A comprehensive guide to options trading terminology and jargon used in our alerts.</p>
							<p><a class="resource-link" href="/trading-terms">View Trading Terms Glossary</a></p>
							
							<p><strong>Support Knowledge Base</strong><br />
							Find answers to common questions about the platform, alerts, and trading strategies.</p>
							<p><a class="resource-link" href="/support">Visit Support Knowledge Base</a></p>

							<p><strong>Performance History</strong><br />
							Review our complete trade history and performance metrics.</p>
							<p><a class="resource-link" href="/dashboard/spx-profit-pulse/alerts">View Alert Archive</a></p>

							<p><strong>Community Forum</strong><br />
							Connect with other SPX Profit Pulse members to share insights and strategies.</p>
							<p><a class="resource-link" href="/community">Join the Community</a></p>
						</div>
					{/if}
				</div>
			</div>
		</section>
	</div>
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
		color: #3879F3;
		margin-left: 15px;
		margin-top: 3px;
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

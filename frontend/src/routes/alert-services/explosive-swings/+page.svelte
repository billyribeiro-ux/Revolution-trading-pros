<script lang="ts">
	let selectedPlan = 'monthly';
</script>

<svelte:head>
	<title>Explosive Swings Alerts | Multi-Day Swing Trading | Revolution Trading Pros</title>
	<meta
		name="description"
		content="Get explosive swing trading alerts for multi-day opportunities. Expert analysis, precise entries/exits, and proven strategies for swing traders."
	/>
</svelte:head>

<!-- Hero Section -->
<section
	class="relative bg-gradient-to-br from-rtp-emerald via-rtp-blue to-rtp-primary text-white py-20 md:py-32"
>
	<div class="absolute inset-0 bg-black/20"></div>
	<div class="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
		<div class="text-center">
			<div
				class="inline-flex items-center gap-2 bg-white/10 backdrop-blur-sm px-4 py-2 rounded-full mb-6"
			>
				<svg class="w-4 h-4 text-emerald-400" fill="currentColor" viewBox="0 0 20 20">
					<path d="M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z" />
					<path d="M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z" />
				</svg>
				<span class="text-sm font-medium">Premium Swing Trading Alerts</span>
			</div>
			<h1 class="text-4xl md:text-6xl font-heading font-bold mb-6">Explosive Swings</h1>
			<p class="text-xl md:text-2xl text-white/90 mb-8 max-w-3xl mx-auto">
				Catch multi-day swing trades with our premium alert service. Get precise entry/exit signals
				for high-probability setups.
			</p>
			<div class="flex flex-wrap gap-4 justify-center">
				<a
					href="#pricing"
					class="bg-white text-rtp-emerald px-8 py-3 rounded-lg font-semibold hover:bg-rtp-bg transition-all hover:scale-105 shadow-lg"
				>
					Subscribe Now
				</a>
				<a
					href="#pricing"
					class="bg-white/10 backdrop-blur-sm text-white border-2 border-white/20 px-8 py-3 rounded-lg font-semibold hover:bg-white/20 transition-all"
				>
					View Pricing
				</a>
			</div>
		</div>
	</div>
</section>

<!-- Stats Bar -->
<section class="bg-rtp-surface border-b border-rtp-border py-12">
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
		<div class="grid grid-cols-2 md:grid-cols-4 gap-8">
			<div class="text-center">
				<div class="text-3xl md:text-4xl font-bold text-rtp-emerald mb-2">82%</div>
				<div class="text-rtp-muted font-medium">Win Rate</div>
			</div>
			<div class="text-center">
				<div class="text-3xl md:text-4xl font-bold text-rtp-primary mb-2">3-7</div>
				<div class="text-rtp-muted font-medium">Days Hold Time</div>
			</div>
			<div class="text-center">
				<div class="text-3xl md:text-4xl font-bold text-rtp-indigo mb-2">4:1</div>
				<div class="text-rtp-muted font-medium">Avg Risk/Reward</div>
			</div>
			<div class="text-center">
				<div class="text-3xl md:text-4xl font-bold text-rtp-blue mb-2">2-4</div>
				<div class="text-rtp-muted font-medium">Alerts Per Week</div>
			</div>
		</div>
	</div>
</section>

<!-- What You Get -->
<section class="py-20 bg-rtp-bg">
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
		<div class="text-center mb-16">
			<h2 class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-4">
				What's Included
			</h2>
			<p class="text-xl text-rtp-muted max-w-3xl mx-auto">
				Everything you need to execute profitable swing trades
			</p>
		</div>

		<div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
			<div class="bg-rtp-surface p-8 rounded-2xl shadow-lg hover:shadow-xl transition-all">
				<div class="w-12 h-12 bg-rtp-emerald/10 rounded-xl flex items-center justify-center mb-6">
					<svg
						class="w-6 h-6 text-rtp-emerald"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M13 10V3L4 14h7v7l9-11h-7z"
						/>
					</svg>
				</div>
				<h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Precise Entry Alerts</h3>
				<p class="text-rtp-muted leading-relaxed">
					Get exact entry prices, position sizes, and stop losses for each swing trade setup.
				</p>
			</div>

			<div class="bg-rtp-surface p-8 rounded-2xl shadow-lg hover:shadow-xl transition-all">
				<div class="w-12 h-12 bg-rtp-primary/10 rounded-xl flex items-center justify-center mb-6">
					<svg
						class="w-6 h-6 text-rtp-primary"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
						/>
					</svg>
				</div>
				<h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Technical Analysis</h3>
				<p class="text-rtp-muted leading-relaxed">
					Detailed chart analysis explaining the setup, key levels, and expected price action.
				</p>
			</div>

			<div class="bg-rtp-surface p-8 rounded-2xl shadow-lg hover:shadow-xl transition-all">
				<div class="w-12 h-12 bg-rtp-indigo/10 rounded-xl flex items-center justify-center mb-6">
					<svg
						class="w-6 h-6 text-rtp-indigo"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
						/>
					</svg>
				</div>
				<h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Exit Management</h3>
				<p class="text-rtp-muted leading-relaxed">
					Real-time updates on profit targets, trailing stops, and when to exit positions.
				</p>
			</div>

			<div class="bg-rtp-surface p-8 rounded-2xl shadow-lg hover:shadow-xl transition-all">
				<div class="w-12 h-12 bg-rtp-blue/10 rounded-xl flex items-center justify-center mb-6">
					<svg class="w-6 h-6 text-rtp-blue" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"
						/>
					</svg>
				</div>
				<h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Risk Management</h3>
				<p class="text-rtp-muted leading-relaxed">
					Clear risk parameters with calculated position sizing based on account size.
				</p>
			</div>

			<div class="bg-rtp-surface p-8 rounded-2xl shadow-lg hover:shadow-xl transition-all">
				<div class="w-12 h-12 bg-rtp-emerald/10 rounded-xl flex items-center justify-center mb-6">
					<svg
						class="w-6 h-6 text-rtp-emerald"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z"
						/>
					</svg>
				</div>
				<h3 class="text-xl font-heading font-bold text-rtp-text mb-3">SMS & Email Alerts</h3>
				<p class="text-rtp-muted leading-relaxed">
					Receive instant notifications via SMS, email, and Discord for all new alerts.
				</p>
			</div>

			<div class="bg-rtp-surface p-8 rounded-2xl shadow-lg hover:shadow-xl transition-all">
				<div class="w-12 h-12 bg-rtp-primary/10 rounded-xl flex items-center justify-center mb-6">
					<svg
						class="w-6 h-6 text-rtp-primary"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"
						/>
					</svg>
				</div>
				<h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Trading Education</h3>
				<p class="text-rtp-muted leading-relaxed">
					Learn the strategies behind each trade with detailed explanations and market context.
				</p>
			</div>
		</div>
	</div>
</section>

<!-- Trading Strategy -->
<section class="py-20 bg-rtp-surface">
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
		<div class="grid md:grid-cols-2 gap-12 items-center">
			<div>
				<h2 class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-6">
					Our Swing Trading Approach
				</h2>
				<p class="text-lg text-rtp-muted mb-8">
					We focus on high-probability swing setups with clear technical levels and momentum
					confirmation. Each trade is carefully analyzed for optimal risk/reward ratios.
				</p>
				<div class="space-y-4">
					<div class="flex items-start gap-4">
						<div
							class="flex-shrink-0 w-8 h-8 bg-rtp-emerald rounded-full flex items-center justify-center"
						>
							<svg class="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M5 13l4 4L19 7"
								/>
							</svg>
						</div>
						<div>
							<h3 class="font-semibold text-rtp-text mb-1">Technical Confirmation</h3>
							<p class="text-rtp-muted">
								Multiple timeframe analysis with clear support/resistance levels
							</p>
						</div>
					</div>
					<div class="flex items-start gap-4">
						<div
							class="flex-shrink-0 w-8 h-8 bg-rtp-emerald rounded-full flex items-center justify-center"
						>
							<svg class="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M5 13l4 4L19 7"
								/>
							</svg>
						</div>
						<div>
							<h3 class="font-semibold text-rtp-text mb-1">Momentum Analysis</h3>
							<p class="text-rtp-muted">
								Volume confirmation and momentum indicators for entry timing
							</p>
						</div>
					</div>
					<div class="flex items-start gap-4">
						<div
							class="flex-shrink-0 w-8 h-8 bg-rtp-emerald rounded-full flex items-center justify-center"
						>
							<svg class="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M5 13l4 4L19 7"
								/>
							</svg>
						</div>
						<div>
							<h3 class="font-semibold text-rtp-text mb-1">Risk Management</h3>
							<p class="text-rtp-muted">
								Defined stop losses and profit targets with 3:1 minimum R/R
							</p>
						</div>
					</div>
				</div>
			</div>
			<div class="bg-rtp-bg p-8 rounded-2xl">
				<h3 class="text-2xl font-heading font-bold text-rtp-text mb-6">Typical Trade Profile</h3>
				<div class="space-y-6">
					<div>
						<div class="flex justify-between mb-2">
							<span class="text-rtp-muted">Hold Time</span>
							<span class="font-semibold text-rtp-text">3-7 Days</span>
						</div>
						<div class="w-full bg-rtp-border rounded-full h-2">
							<div class="bg-rtp-emerald h-2 rounded-full" style="width: 75%"></div>
						</div>
					</div>
					<div>
						<div class="flex justify-between mb-2">
							<span class="text-rtp-muted">Win Rate</span>
							<span class="font-semibold text-rtp-text">82%</span>
						</div>
						<div class="w-full bg-rtp-border rounded-full h-2">
							<div class="bg-rtp-primary h-2 rounded-full" style="width: 82%"></div>
						</div>
					</div>
					<div>
						<div class="flex justify-between mb-2">
							<span class="text-rtp-muted">Avg Profit Target</span>
							<span class="font-semibold text-rtp-text">15-30%</span>
						</div>
						<div class="w-full bg-rtp-border rounded-full h-2">
							<div class="bg-rtp-indigo h-2 rounded-full" style="width: 65%"></div>
						</div>
					</div>
					<div>
						<div class="flex justify-between mb-2">
							<span class="text-rtp-muted">Max Risk Per Trade</span>
							<span class="font-semibold text-rtp-text">2-3%</span>
						</div>
						<div class="w-full bg-rtp-border rounded-full h-2">
							<div class="bg-rtp-blue h-2 rounded-full" style="width: 25%"></div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</section>

<!-- Pricing Section -->
<section id="pricing" class="py-20 bg-rtp-bg">
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
		<div class="text-center mb-16">
			<h2 class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-4">Simple Pricing</h2>
			<p class="text-xl text-rtp-muted max-w-3xl mx-auto">
				Choose your plan and start receiving explosive swing trade alerts today
			</p>
		</div>

		<div class="flex justify-center mb-12">
			<div class="bg-rtp-surface p-1 rounded-lg shadow-md inline-flex gap-1">
				<button
					on:click={() => (selectedPlan = 'monthly')}
					class="px-6 py-2 rounded-md font-semibold transition-all {selectedPlan === 'monthly'
						? 'bg-rtp-emerald text-white'
						: 'text-rtp-muted hover:text-rtp-text'}"
				>
					Monthly
				</button>
				<button
					on:click={() => (selectedPlan = 'annual')}
					class="px-6 py-2 rounded-md font-semibold transition-all {selectedPlan === 'annual'
						? 'bg-rtp-emerald text-white'
						: 'text-rtp-muted hover:text-rtp-text'}"
				>
					Annual
				</button>
			</div>
		</div>

		<div class="max-w-md mx-auto">
			{#if selectedPlan === 'monthly'}
				<div class="bg-rtp-surface p-8 rounded-2xl shadow-lg border-2 border-rtp-emerald">
					<div class="text-center mb-8">
						<h3 class="text-2xl font-heading font-bold text-rtp-text mb-2">Monthly</h3>
						<div class="text-5xl font-bold text-rtp-emerald mb-2">$97</div>
						<div class="text-rtp-muted">per month</div>
					</div>
					<ul class="space-y-4 mb-8">
						<li class="flex items-start gap-3">
							<svg
								class="w-6 h-6 text-rtp-emerald flex-shrink-0"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M5 13l4 4L19 7"
								/>
							</svg>
							<span class="text-rtp-text">2-4 swing alerts per week</span>
						</li>
						<li class="flex items-start gap-3">
							<svg
								class="w-6 h-6 text-rtp-emerald flex-shrink-0"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M5 13l4 4L19 7"
								/>
							</svg>
							<span class="text-rtp-text">SMS & Email notifications</span>
						</li>
						<li class="flex items-start gap-3">
							<svg
								class="w-6 h-6 text-rtp-emerald flex-shrink-0"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M5 13l4 4L19 7"
								/>
							</svg>
							<span class="text-rtp-text">Full technical analysis</span>
						</li>
						<li class="flex items-start gap-3">
							<svg
								class="w-6 h-6 text-rtp-emerald flex-shrink-0"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M5 13l4 4L19 7"
								/>
							</svg>
							<span class="text-rtp-text">Real-time exit updates</span>
						</li>
						<li class="flex items-start gap-3">
							<svg
								class="w-6 h-6 text-rtp-emerald flex-shrink-0"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M5 13l4 4L19 7"
								/>
							</svg>
							<span class="text-rtp-text">Discord community access</span>
						</li>
					</ul>
					<button
						class="w-full bg-rtp-emerald text-white py-3 rounded-lg font-semibold hover:bg-emerald-600 transition-all hover:scale-105"
					>
						Subscribe Now
					</button>
				</div>
			{:else}
				<div class="bg-rtp-surface p-8 rounded-2xl shadow-lg border-2 border-rtp-primary">
					<div
						class="inline-block bg-rtp-emerald text-white px-3 py-1 rounded-full text-sm font-semibold mb-4"
					>
						Save 20%
					</div>
					<div class="text-center mb-8">
						<h3 class="text-2xl font-heading font-bold text-rtp-text mb-2">Annual</h3>
						<div class="text-5xl font-bold text-rtp-primary mb-2">$927</div>
						<div class="text-rtp-muted">per year ($77/month)</div>
					</div>
					<ul class="space-y-4 mb-8">
						<li class="flex items-start gap-3">
							<svg
								class="w-6 h-6 text-rtp-emerald flex-shrink-0"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M5 13l4 4L19 7"
								/>
							</svg>
							<span class="text-rtp-text">Everything in Monthly</span>
						</li>
						<li class="flex items-start gap-3">
							<svg
								class="w-6 h-6 text-rtp-emerald flex-shrink-0"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M5 13l4 4L19 7"
								/>
							</svg>
							<span class="text-rtp-text font-semibold">Save $237 per year</span>
						</li>
						<li class="flex items-start gap-3">
							<svg
								class="w-6 h-6 text-rtp-emerald flex-shrink-0"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M5 13l4 4L19 7"
								/>
							</svg>
							<span class="text-rtp-text">Priority support</span>
						</li>
						<li class="flex items-start gap-3">
							<svg
								class="w-6 h-6 text-rtp-emerald flex-shrink-0"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M5 13l4 4L19 7"
								/>
							</svg>
							<span class="text-rtp-text">Exclusive indicators</span>
						</li>
					</ul>
					<button
						class="w-full bg-rtp-primary text-white py-3 rounded-lg font-semibold hover:bg-rtp-blue transition-all hover:scale-105"
					>
						Subscribe Now
					</button>
				</div>
			{/if}
		</div>
	</div>
</section>

<!-- Final CTA -->
<section class="py-20 bg-gradient-to-br from-rtp-emerald to-rtp-primary text-white">
	<div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
		<h2 class="text-3xl md:text-5xl font-heading font-bold mb-6">Start Trading Explosive Swings</h2>
		<p class="text-xl text-white/90 mb-8 max-w-2xl mx-auto">
			Join hundreds of profitable swing traders. 30-day money-back guarantee.
		</p>
		<a
			href="#pricing"
			class="inline-block bg-white text-rtp-emerald px-8 py-4 rounded-lg font-semibold hover:bg-rtp-bg transition-all hover:scale-105 shadow-lg"
		>
			Get Started Today
		</a>
	</div>
</section>

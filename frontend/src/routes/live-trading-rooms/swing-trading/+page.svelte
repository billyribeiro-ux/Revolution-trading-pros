<script lang="ts">
	let selectedPlan = 'monthly';
</script>

<svelte:head>
	<title>Swing Trading Room | Multi-Day SPX Trading | Revolution Trading Pros</title>
	<meta
		name="description"
		content="Join our swing trading room for multi-day SPX positions. Professional setups, risk management, and community support for swing traders."
	/>
	<link rel="canonical" href="https://revolutiontradingpros.com/live-trading-rooms/swing-trading" />
</svelte:head>

<!-- Hero Section -->
<section
	class="relative bg-gradient-to-br from-rtp-indigo via-rtp-primary to-rtp-blue text-white py-20 md:py-32"
>
	<div class="absolute inset-0 bg-black/20"></div>
	<div class="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
		<div class="text-center">
			<div
				class="inline-flex items-center gap-2 bg-white/10 backdrop-blur-sm px-4 py-2 rounded-full mb-6"
			>
				<div class="w-2 h-2 bg-rtp-emerald rounded-full animate-pulse"></div>
				<span class="text-sm font-medium">Multi-Day Positions</span>
			</div>
			<h1 class="text-4xl md:text-6xl font-heading font-bold mb-6">Swing Trading Room</h1>
			<p class="text-xl md:text-2xl text-white/90 mb-8 max-w-3xl mx-auto">
				Capture larger market moves with professionally analyzed swing trades. Hold positions for
				days to weeks with clear entry and exit strategies.
			</p>
			<div class="flex flex-wrap gap-4 justify-center">
				<a
					href="#pricing"
					class="bg-white text-rtp-indigo px-8 py-3 rounded-lg font-semibold hover:bg-rtp-bg transition-all hover:scale-105 shadow-lg"
				>
					View Pricing
				</a>
				<a
					href="#features"
					class="bg-white/10 backdrop-blur-sm text-white border-2 border-white/20 px-8 py-3 rounded-lg font-semibold hover:bg-white/20 transition-all"
				>
					Learn More
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
				<div class="text-3xl md:text-4xl font-bold text-rtp-indigo mb-2">65%</div>
				<div class="text-rtp-muted font-medium">Win Rate</div>
			</div>
			<div class="text-center">
				<div class="text-3xl md:text-4xl font-bold text-rtp-emerald mb-2">3.2:1</div>
				<div class="text-rtp-muted font-medium">Avg Risk/Reward</div>
			</div>
			<div class="text-center">
				<div class="text-3xl md:text-4xl font-bold text-rtp-primary mb-2">2-10</div>
				<div class="text-rtp-muted font-medium">Days Per Trade</div>
			</div>
			<div class="text-center">
				<div class="text-3xl md:text-4xl font-bold text-rtp-blue mb-2">300+</div>
				<div class="text-rtp-muted font-medium">Active Members</div>
			</div>
		</div>
	</div>
</section>

<!-- Features Section -->
<section id="features" class="py-20 bg-rtp-bg">
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
		<div class="text-center mb-16">
			<h2 class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-4">
				Why Swing Trade With Us
			</h2>
			<p class="text-xl text-rtp-muted max-w-3xl mx-auto">
				Professional swing trading strategies for traders who can't watch the market all day
			</p>
		</div>

		<div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
			<!-- Feature 1 -->
			<div
				class="bg-rtp-surface p-8 rounded-2xl shadow-lg hover:shadow-xl transition-all hover:-translate-y-1"
			>
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
							d="M7 12l3-3 3 3 4-4M8 21l4-4 4 4M3 4h18M4 4h16v12a1 1 0 01-1 1H5a1 1 0 01-1-1V4z"
						/>
					</svg>
				</div>
				<h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Multi-Day Setups</h3>
				<p class="text-rtp-muted leading-relaxed">
					Carefully selected swing trades with 2-10 day holding periods targeting larger price
					movements.
				</p>
			</div>

			<!-- Feature 2 -->
			<div
				class="bg-rtp-surface p-8 rounded-2xl shadow-lg hover:shadow-xl transition-all hover:-translate-y-1"
			>
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
					In-depth chart analysis identifying key support, resistance, and trend continuation
					patterns.
				</p>
			</div>

			<!-- Feature 3 -->
			<div
				class="bg-rtp-surface p-8 rounded-2xl shadow-lg hover:shadow-xl transition-all hover:-translate-y-1"
			>
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
							d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"
						/>
					</svg>
				</div>
				<h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Position Updates</h3>
				<p class="text-rtp-muted leading-relaxed">
					Daily updates on active positions with adjustment recommendations and profit-taking
					strategies.
				</p>
			</div>

			<!-- Feature 4 -->
			<div
				class="bg-rtp-surface p-8 rounded-2xl shadow-lg hover:shadow-xl transition-all hover:-translate-y-1"
			>
				<div class="w-12 h-12 bg-rtp-blue/10 rounded-xl flex items-center justify-center mb-6">
					<svg class="w-6 h-6 text-rtp-blue" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4"
						/>
					</svg>
				</div>
				<h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Risk Management</h3>
				<p class="text-rtp-muted leading-relaxed">
					Clear stop losses, position sizing guidelines, and risk/reward ratios for every trade
					setup.
				</p>
			</div>

			<!-- Feature 5 -->
			<div
				class="bg-rtp-surface p-8 rounded-2xl shadow-lg hover:shadow-xl transition-all hover:-translate-y-1"
			>
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
							d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"
						/>
					</svg>
				</div>
				<h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Community Trading</h3>
				<p class="text-rtp-muted leading-relaxed">
					Active Discord community for discussing setups, sharing ideas, and supporting each other.
				</p>
			</div>

			<!-- Feature 6 -->
			<div
				class="bg-rtp-surface p-8 rounded-2xl shadow-lg hover:shadow-xl transition-all hover:-translate-y-1"
			>
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
							d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
						/>
					</svg>
				</div>
				<h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Flexible Schedule</h3>
				<p class="text-rtp-muted leading-relaxed">
					Perfect for traders with full-time jobs. No need to watch the market all day long.
				</p>
			</div>
		</div>
	</div>
</section>

<!-- Trading Strategy Section -->
<section class="py-20 bg-rtp-surface">
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
		<div class="text-center mb-16">
			<h2 class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-4">
				Our Swing Trading Approach
			</h2>
			<p class="text-xl text-rtp-muted">Systematic strategies for consistent results</p>
		</div>

		<div class="grid md:grid-cols-2 gap-12 items-center">
			<div class="space-y-8">
				<div class="flex gap-4">
					<div
						class="flex-shrink-0 w-12 h-12 bg-rtp-primary rounded-lg flex items-center justify-center text-white font-bold text-xl"
					>
						1
					</div>
					<div>
						<h3 class="text-xl font-heading font-bold text-rtp-text mb-2">Market Analysis</h3>
						<p class="text-rtp-muted leading-relaxed">
							Weekly market outlook identifying key levels, trends, and upcoming catalysts that
							could impact swing positions.
						</p>
					</div>
				</div>

				<div class="flex gap-4">
					<div
						class="flex-shrink-0 w-12 h-12 bg-rtp-indigo rounded-lg flex items-center justify-center text-white font-bold text-xl"
					>
						2
					</div>
					<div>
						<h3 class="text-xl font-heading font-bold text-rtp-text mb-2">Setup Identification</h3>
						<p class="text-rtp-muted leading-relaxed">
							Scan for high-probability setups using technical analysis, volume patterns, and
							momentum indicators.
						</p>
					</div>
				</div>

				<div class="flex gap-4">
					<div
						class="flex-shrink-0 w-12 h-12 bg-rtp-emerald rounded-lg flex items-center justify-center text-white font-bold text-xl"
					>
						3
					</div>
					<div>
						<h3 class="text-xl font-heading font-bold text-rtp-text mb-2">Entry & Alerts</h3>
						<p class="text-rtp-muted leading-relaxed">
							Clear entry signals with specific price levels, position sizing, and risk parameters
							for each trade.
						</p>
					</div>
				</div>

				<div class="flex gap-4">
					<div
						class="flex-shrink-0 w-12 h-12 bg-rtp-blue rounded-lg flex items-center justify-center text-white font-bold text-xl"
					>
						4
					</div>
					<div>
						<h3 class="text-xl font-heading font-bold text-rtp-text mb-2">Trade Management</h3>
						<p class="text-rtp-muted leading-relaxed">
							Daily updates on position management including profit targets, trailing stops, and
							adjustment strategies.
						</p>
					</div>
				</div>
			</div>

			<div class="bg-gradient-to-br from-rtp-primary/10 to-rtp-indigo/10 p-8 rounded-2xl">
				<div class="bg-rtp-surface p-6 rounded-xl shadow-lg mb-4">
					<div class="flex items-center justify-between mb-2">
						<span class="text-sm font-semibold text-rtp-muted">SPX Call Spread</span>
						<span class="text-sm font-semibold text-rtp-emerald">+45%</span>
					</div>
					<div class="text-xs text-rtp-muted">Held 5 days | Target reached</div>
				</div>

				<div class="bg-rtp-surface p-6 rounded-xl shadow-lg mb-4">
					<div class="flex items-center justify-between mb-2">
						<span class="text-sm font-semibold text-rtp-muted">SPX Put Spread</span>
						<span class="text-sm font-semibold text-rtp-emerald">+32%</span>
					</div>
					<div class="text-xs text-rtp-muted">Held 3 days | Target reached</div>
				</div>

				<div class="bg-rtp-surface p-6 rounded-xl shadow-lg mb-4">
					<div class="flex items-center justify-between mb-2">
						<span class="text-sm font-semibold text-rtp-muted">SPX Iron Condor</span>
						<span class="text-sm font-semibold text-rtp-emerald">+28%</span>
					</div>
					<div class="text-xs text-rtp-muted">Held 7 days | Target reached</div>
				</div>

				<div class="bg-rtp-indigo text-white p-6 rounded-xl text-center">
					<div class="text-3xl font-bold mb-1">35%</div>
					<div class="text-sm">Average Gain Per Trade</div>
				</div>
			</div>
		</div>
	</div>
</section>

<!-- Pricing Section -->
<section id="pricing" class="py-20 bg-rtp-bg">
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
		<div class="text-center mb-16">
			<h2 class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-4">
				Membership Pricing
			</h2>
			<p class="text-xl text-rtp-muted max-w-3xl mx-auto">Join our swing trading community today</p>
		</div>

		<div class="flex justify-center mb-12">
			<div class="bg-rtp-surface p-1 rounded-lg shadow-md inline-flex gap-1">
				<button
					on:click={() => (selectedPlan = 'monthly')}
					class="px-6 py-2 rounded-md font-semibold transition-all {selectedPlan === 'monthly'
						? 'bg-rtp-indigo text-white'
						: 'text-rtp-muted hover:text-rtp-text'}"
				>
					Monthly
				</button>
				<button
					on:click={() => (selectedPlan = 'quarterly')}
					class="px-6 py-2 rounded-md font-semibold transition-all {selectedPlan === 'quarterly'
						? 'bg-rtp-indigo text-white'
						: 'text-rtp-muted hover:text-rtp-text'}"
				>
					Quarterly
				</button>
				<button
					on:click={() => (selectedPlan = 'annual')}
					class="px-6 py-2 rounded-md font-semibold transition-all {selectedPlan === 'annual'
						? 'bg-rtp-indigo text-white'
						: 'text-rtp-muted hover:text-rtp-text'}"
				>
					Annual
				</button>
			</div>
		</div>

		<div class="grid md:grid-cols-3 gap-8 max-w-5xl mx-auto">
			{#if selectedPlan === 'monthly'}
				<div
					class="bg-rtp-surface p-8 rounded-2xl shadow-lg border-2 border-rtp-border hover:border-rtp-indigo transition-all md:col-start-2"
				>
					<div class="text-center mb-8">
						<h3 class="text-2xl font-heading font-bold text-rtp-text mb-2">Monthly</h3>
						<div class="text-5xl font-bold text-rtp-indigo mb-2">$147</div>
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
							<span class="text-rtp-text">Weekly swing trade setups</span>
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
							<span class="text-rtp-text">Daily position updates</span>
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
							<span class="text-rtp-text">Trade analysis & education</span>
						</li>
					</ul>
					<button
						class="w-full bg-rtp-indigo text-white py-3 rounded-lg font-semibold hover:bg-purple-800 transition-all hover:scale-105"
					>
						Get Started
					</button>
				</div>
			{:else if selectedPlan === 'quarterly'}
				<div
					class="bg-rtp-surface p-8 rounded-2xl shadow-lg border-2 border-rtp-indigo md:col-start-2"
				>
					<div
						class="inline-block bg-rtp-emerald text-white px-3 py-1 rounded-full text-sm font-semibold mb-4"
					>
						Save 20%
					</div>
					<div class="text-center mb-8">
						<h3 class="text-2xl font-heading font-bold text-rtp-text mb-2">Quarterly</h3>
						<div class="text-5xl font-bold text-rtp-indigo mb-2">$353</div>
						<div class="text-rtp-muted">per quarter ($118/month)</div>
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
							<span class="text-rtp-text font-semibold">Save $88</span>
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
							<span class="text-rtp-text">Bonus swing strategies</span>
						</li>
					</ul>
					<button
						class="w-full bg-rtp-indigo text-white py-3 rounded-lg font-semibold hover:bg-purple-800 transition-all hover:scale-105"
					>
						Get Started
					</button>
				</div>
			{:else}
				<div
					class="bg-rtp-surface p-8 rounded-2xl shadow-lg border-2 border-rtp-emerald md:col-start-2"
				>
					<div
						class="inline-block bg-rtp-emerald text-white px-3 py-1 rounded-full text-sm font-semibold mb-4"
					>
						Save 35%
					</div>
					<div class="text-center mb-8">
						<h3 class="text-2xl font-heading font-bold text-rtp-text mb-2">Annual</h3>
						<div class="text-5xl font-bold text-rtp-indigo mb-2">$1,147</div>
						<div class="text-rtp-muted">per year ($96/month)</div>
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
							<span class="text-rtp-text">Everything in Quarterly</span>
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
							<span class="text-rtp-text font-semibold">Save $617</span>
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
							<span class="text-rtp-text">1-on-1 strategy session</span>
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
						class="w-full bg-rtp-emerald text-white py-3 rounded-lg font-semibold hover:bg-emerald-600 transition-all hover:scale-105"
					>
						Get Started
					</button>
				</div>
			{/if}
		</div>
	</div>
</section>

<!-- Testimonials Section -->
<section class="py-20 bg-rtp-surface">
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
		<div class="text-center mb-16">
			<h2 class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-4">
				Member Success Stories
			</h2>
			<p class="text-xl text-rtp-muted">Real results from swing traders in our community</p>
		</div>

		<div class="grid md:grid-cols-3 gap-8">
			<div class="bg-rtp-bg p-8 rounded-2xl border border-rtp-border">
				<div class="flex items-center gap-1 mb-4">
					{#each Array(5) as _}
						<svg class="w-5 h-5 text-yellow-400 fill-current" viewBox="0 0 20 20">
							<path
								d="M10 15l-5.878 3.09 1.123-6.545L.489 6.91l6.572-.955L10 0l2.939 5.955 6.572.955-4.756 4.635 1.123 6.545z"
							/>
						</svg>
					{/each}
				</div>
				<p class="text-rtp-text mb-6 italic">
					"Perfect for my schedule. I check the room once in the morning and evening. The swing
					setups have been incredibly profitable."
				</p>
				<div class="flex items-center gap-3">
					<div
						class="w-10 h-10 bg-rtp-indigo rounded-full flex items-center justify-center text-white font-bold"
					>
						TC
					</div>
					<div>
						<div class="font-semibold text-rtp-text">Tom C.</div>
						<div class="text-sm text-rtp-muted">Full-time professional</div>
					</div>
				</div>
			</div>

			<div class="bg-rtp-bg p-8 rounded-2xl border border-rtp-border">
				<div class="flex items-center gap-1 mb-4">
					{#each Array(5) as _}
						<svg class="w-5 h-5 text-yellow-400 fill-current" viewBox="0 0 20 20">
							<path
								d="M10 15l-5.878 3.09 1.123-6.545L.489 6.91l6.572-.955L10 0l2.939 5.955 6.572.955-4.756 4.635 1.123 6.545z"
							/>
						</svg>
					{/each}
				</div>
				<p class="text-rtp-text mb-6 italic">
					"The risk management and position sizing guidance has been game-changing. I'm finally
					trading with discipline and seeing consistent results."
				</p>
				<div class="flex items-center gap-3">
					<div
						class="w-10 h-10 bg-rtp-primary rounded-full flex items-center justify-center text-white font-bold"
					>
						LH
					</div>
					<div>
						<div class="font-semibold text-rtp-text">Lisa H.</div>
						<div class="text-sm text-rtp-muted">Swing trader</div>
					</div>
				</div>
			</div>

			<div class="bg-rtp-bg p-8 rounded-2xl border border-rtp-border">
				<div class="flex items-center gap-1 mb-4">
					{#each Array(5) as _}
						<svg class="w-5 h-5 text-yellow-400 fill-current" viewBox="0 0 20 20">
							<path
								d="M10 15l-5.878 3.09 1.123-6.545L.489 6.91l6.572-.955L10 0l2.939 5.955 6.572.955-4.756 4.635 1.123 6.545z"
							/>
						</svg>
					{/each}
				</div>
				<p class="text-rtp-text mb-6 italic">
					"Great community and excellent trade setups. The daily updates keep me informed without
					overwhelming me with constant notifications."
				</p>
				<div class="flex items-center gap-3">
					<div
						class="w-10 h-10 bg-rtp-emerald rounded-full flex items-center justify-center text-white font-bold"
					>
						RK
					</div>
					<div>
						<div class="font-semibold text-rtp-text">Robert K.</div>
						<div class="text-sm text-rtp-muted">Part-time trader</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</section>

<!-- Final CTA -->
<section class="py-20 bg-gradient-to-br from-rtp-indigo to-rtp-primary text-white">
	<div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
		<h2 class="text-3xl md:text-5xl font-heading font-bold mb-6">Start Swing Trading Today</h2>
		<p class="text-xl text-white/90 mb-8 max-w-2xl mx-auto">
			Join our community and start receiving professional swing trade setups. 30-day money-back
			guarantee.
		</p>
		<div class="flex flex-wrap gap-4 justify-center">
			<a
				href="#pricing"
				class="bg-white text-rtp-indigo px-8 py-4 rounded-lg font-semibold hover:bg-rtp-bg transition-all hover:scale-105 shadow-lg"
			>
				Choose Your Plan
			</a>
			<a
				href="/resources"
				class="bg-white/10 backdrop-blur-sm text-white border-2 border-white/20 px-8 py-4 rounded-lg font-semibold hover:bg-white/20 transition-all"
			>
				Learn More
			</a>
		</div>
	</div>
</section>

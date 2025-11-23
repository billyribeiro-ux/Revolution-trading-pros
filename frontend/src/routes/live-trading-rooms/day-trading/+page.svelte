<script lang="ts">
	let selectedPlan = 'monthly';
</script>

<svelte:head>
	<title>Day Trading Room | Live SPX 0DTE Trading | Revolution Trading Pros</title>
	<meta
		name="description"
		content="Join our live day trading room for SPX 0DTE options. Real-time trade alerts, professional analysis, and community support from experienced traders."
	/>
	<link rel="canonical" href="https://revolutiontradingpros.com/live-trading-rooms/day-trading" />
</svelte:head>

<!-- Hero Section -->
<section
	class="relative bg-gradient-to-br from-rtp-primary via-rtp-blue to-rtp-indigo text-white py-20 md:py-32"
>
	<div class="absolute inset-0 bg-black/20"></div>
	<div class="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
		<div class="text-center">
			<div
				class="inline-flex items-center gap-2 bg-white/10 backdrop-blur-sm px-4 py-2 rounded-full mb-6"
			>
				<div class="w-2 h-2 bg-emerald-400 rounded-full animate-pulse"></div>
				<span class="text-sm font-medium">Live Trading Daily</span>
			</div>
			<h1 class="text-4xl md:text-6xl font-heading font-bold mb-6">Day Trading Room</h1>
			<p class="text-xl md:text-2xl text-white/90 mb-8 max-w-3xl mx-auto">
				Trade SPX 0DTE options with professional traders in our live room. Get real-time alerts,
				professional analysis, and expert guidance.
			</p>
			<div class="flex flex-wrap gap-4 justify-center">
				<a
					href="#pricing"
					class="bg-white text-rtp-primary px-8 py-3 rounded-lg font-semibold hover:bg-rtp-bg transition-all hover:scale-105 shadow-lg"
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
				<div class="text-3xl md:text-4xl font-bold text-rtp-primary mb-2">500+</div>
				<div class="text-rtp-muted font-medium">Active Members</div>
			</div>
			<div class="text-center">
				<div class="text-3xl md:text-4xl font-bold text-rtp-emerald mb-2">5+</div>
				<div class="text-rtp-muted font-medium">Years Trading</div>
			</div>
			<div class="text-center">
				<div class="text-3xl md:text-4xl font-bold text-rtp-indigo mb-2">1000+</div>
				<div class="text-rtp-muted font-medium">Trades Shared</div>
			</div>
			<div class="text-center">
				<div class="text-3xl md:text-4xl font-bold text-rtp-blue mb-2">24/7</div>
				<div class="text-rtp-muted font-medium">Community Support</div>
			</div>
		</div>
	</div>
</section>

<!-- Features Section -->
<section id="features" class="py-20 bg-rtp-bg">
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
		<div class="text-center mb-16">
			<h2 class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-4">What You Get</h2>
			<p class="text-xl text-rtp-muted max-w-3xl mx-auto">
				Everything you need to succeed in day trading SPX 0DTE options
			</p>
		</div>

		<div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
			<!-- Feature 1 -->
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
							d="M13 10V3L4 14h7v7l9-11h-7z"
						/>
					</svg>
				</div>
				<h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Real-Time Alerts</h3>
				<p class="text-rtp-muted leading-relaxed">
					Get instant notifications for entries, exits, and adjustments as they happen in the live
					market.
				</p>
			</div>

			<!-- Feature 2 -->
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
							d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
						/>
					</svg>
				</div>
				<h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Live Analysis</h3>
				<p class="text-rtp-muted leading-relaxed">
					Watch professional traders analyze charts, identify setups, and explain their
					decision-making process.
				</p>
			</div>

			<!-- Feature 3 -->
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
				<h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Active Community</h3>
				<p class="text-rtp-muted leading-relaxed">
					Connect with hundreds of traders, share ideas, and learn from experienced professionals.
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
							d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"
						/>
					</svg>
				</div>
				<h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Educational Content</h3>
				<p class="text-rtp-muted leading-relaxed">
					Access exclusive training videos, strategy guides, and market analysis to improve your
					skills.
				</p>
			</div>

			<!-- Feature 5 -->
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
				<h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Market Hours Coverage</h3>
				<p class="text-rtp-muted leading-relaxed">
					Active room from market open to close, covering all major trading sessions and
					opportunities.
				</p>
			</div>

			<!-- Feature 6 -->
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
							d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"
						/>
					</svg>
				</div>
				<h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Risk Management</h3>
				<p class="text-rtp-muted leading-relaxed">
					Learn proper position sizing, stop losses, and risk/reward ratios from experienced
					professionals.
				</p>
			</div>
		</div>
	</div>
</section>

<!-- Schedule Section -->
<section class="py-20 bg-rtp-surface">
	<div class="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8">
		<div class="text-center mb-16">
			<h2 class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-4">Daily Schedule</h2>
			<p class="text-xl text-rtp-muted">Structured trading sessions throughout the day</p>
		</div>

		<div class="space-y-4">
			<div
				class="bg-rtp-bg p-6 rounded-xl border border-rtp-border hover:border-rtp-primary transition-colors"
			>
				<div class="flex items-start gap-4">
					<div class="flex-shrink-0 w-24 text-rtp-primary font-bold">8:30 AM</div>
					<div class="flex-1">
						<h3 class="font-heading font-bold text-rtp-text mb-2">Pre-Market Analysis</h3>
						<p class="text-rtp-muted">
							Review overnight action, key levels, and economic calendar for the day ahead.
						</p>
					</div>
				</div>
			</div>

			<div
				class="bg-rtp-bg p-6 rounded-xl border border-rtp-border hover:border-rtp-primary transition-colors"
			>
				<div class="flex items-start gap-4">
					<div class="flex-shrink-0 w-24 text-rtp-emerald font-bold">9:30 AM</div>
					<div class="flex-1">
						<h3 class="font-heading font-bold text-rtp-text mb-2">Market Open Trading</h3>
						<p class="text-rtp-muted">
							Active trading and real-time alerts during the volatile opening session.
						</p>
					</div>
				</div>
			</div>

			<div
				class="bg-rtp-bg p-6 rounded-xl border border-rtp-border hover:border-rtp-primary transition-colors"
			>
				<div class="flex items-start gap-4">
					<div class="flex-shrink-0 w-24 text-rtp-indigo font-bold">11:00 AM</div>
					<div class="flex-1">
						<h3 class="font-heading font-bold text-rtp-text mb-2">Mid-Day Review</h3>
						<p class="text-rtp-muted">
							Assess morning positions and identify new opportunities in the mid-day session.
						</p>
					</div>
				</div>
			</div>

			<div
				class="bg-rtp-bg p-6 rounded-xl border border-rtp-border hover:border-rtp-primary transition-colors"
			>
				<div class="flex items-start gap-4">
					<div class="flex-shrink-0 w-24 text-rtp-blue font-bold">3:00 PM</div>
					<div class="flex-1">
						<h3 class="font-heading font-bold text-rtp-text mb-2">Power Hour & Close</h3>
						<p class="text-rtp-muted">
							Trade the final hour volatility and manage positions into market close.
						</p>
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
			<h2 class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-4">
				Membership Pricing
			</h2>
			<p class="text-xl text-rtp-muted max-w-3xl mx-auto">
				Choose the plan that works best for you. All plans include full access to the live trading
				room.
			</p>
		</div>

		<div class="flex justify-center mb-12">
			<div class="bg-rtp-surface p-1 rounded-lg shadow-md inline-flex gap-1">
				<button
					on:click={() => (selectedPlan = 'monthly')}
					class="px-6 py-2 rounded-md font-semibold transition-all {selectedPlan === 'monthly'
						? 'bg-rtp-primary text-white'
						: 'text-rtp-muted hover:text-rtp-text'}"
				>
					Monthly
				</button>
				<button
					on:click={() => (selectedPlan = 'quarterly')}
					class="px-6 py-2 rounded-md font-semibold transition-all {selectedPlan === 'quarterly'
						? 'bg-rtp-primary text-white'
						: 'text-rtp-muted hover:text-rtp-text'}"
				>
					Quarterly
				</button>
				<button
					on:click={() => (selectedPlan = 'annual')}
					class="px-6 py-2 rounded-md font-semibold transition-all {selectedPlan === 'annual'
						? 'bg-rtp-primary text-white'
						: 'text-rtp-muted hover:text-rtp-text'}"
				>
					Annual
				</button>
			</div>
		</div>

		<div class="grid md:grid-cols-3 gap-8 max-w-5xl mx-auto">
			<!-- Monthly Plan -->
			{#if selectedPlan === 'monthly'}
				<div
					class="bg-rtp-surface p-8 rounded-2xl shadow-lg border-2 border-rtp-border hover:border-rtp-primary transition-all"
				>
					<div class="text-center mb-8">
						<h3 class="text-2xl font-heading font-bold text-rtp-text mb-2">Monthly</h3>
						<div class="text-5xl font-bold text-rtp-primary mb-2">$197</div>
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
							<span class="text-rtp-text">Full live room access</span>
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
							<span class="text-rtp-text">Real-time trade alerts</span>
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
							<span class="text-rtp-text">Community support</span>
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
							<span class="text-rtp-text">Educational content</span>
						</li>
					</ul>
					<button
						class="w-full bg-rtp-primary text-white py-3 rounded-lg font-semibold hover:bg-rtp-blue transition-all hover:scale-105"
					>
						Get Started
					</button>
				</div>
			{/if}

			<!-- Quarterly Plan -->
			{#if selectedPlan === 'quarterly'}
				<div class="bg-rtp-surface p-8 rounded-2xl shadow-lg border-2 border-rtp-primary">
					<div
						class="inline-block bg-rtp-emerald text-white px-3 py-1 rounded-full text-sm font-semibold mb-4"
					>
						Save 15%
					</div>
					<div class="text-center mb-8">
						<h3 class="text-2xl font-heading font-bold text-rtp-text mb-2">Quarterly</h3>
						<div class="text-5xl font-bold text-rtp-primary mb-2">$497</div>
						<div class="text-rtp-muted">per quarter ($166/month)</div>
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
							<span class="text-rtp-text font-semibold">Save $94</span>
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
							<span class="text-rtp-text">Quarterly strategy sessions</span>
						</li>
					</ul>
					<button
						class="w-full bg-rtp-primary text-white py-3 rounded-lg font-semibold hover:bg-rtp-blue transition-all hover:scale-105"
					>
						Get Started
					</button>
				</div>
			{/if}

			<!-- Annual Plan -->
			{#if selectedPlan === 'annual'}
				<div class="bg-rtp-surface p-8 rounded-2xl shadow-lg border-2 border-rtp-emerald">
					<div
						class="inline-block bg-rtp-emerald text-white px-3 py-1 rounded-full text-sm font-semibold mb-4"
					>
						Save 30%
					</div>
					<div class="text-center mb-8">
						<h3 class="text-2xl font-heading font-bold text-rtp-text mb-2">Annual</h3>
						<div class="text-5xl font-bold text-rtp-primary mb-2">$1,647</div>
						<div class="text-rtp-muted">per year ($137/month)</div>
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
							<span class="text-rtp-text font-semibold">Save $717</span>
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
							<span class="text-rtp-text">1-on-1 coaching call</span>
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
				What Members Say
			</h2>
			<p class="text-xl text-rtp-muted">Real feedback from our trading community</p>
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
					"The day trading room completely changed my approach to 0DTE options. The real-time alerts
					and analysis are invaluable."
				</p>
				<div class="flex items-center gap-3">
					<div
						class="w-10 h-10 bg-rtp-primary rounded-full flex items-center justify-center text-white font-bold"
					>
						JD
					</div>
					<div>
						<div class="font-semibold text-rtp-text">John D.</div>
						<div class="text-sm text-rtp-muted">Member since 2023</div>
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
					"Best trading community I've been part of. The education and support have helped me become
					a consistently profitable trader."
				</p>
				<div class="flex items-center gap-3">
					<div
						class="w-10 h-10 bg-rtp-emerald rounded-full flex items-center justify-center text-white font-bold"
					>
						SM
					</div>
					<div>
						<div class="font-semibold text-rtp-text">Sarah M.</div>
						<div class="text-sm text-rtp-muted">Member since 2022</div>
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
					"The professional analysis and trade management strategies have taken my trading to the
					next level. Worth every penny."
				</p>
				<div class="flex items-center gap-3">
					<div
						class="w-10 h-10 bg-rtp-indigo rounded-full flex items-center justify-center text-white font-bold"
					>
						MR
					</div>
					<div>
						<div class="font-semibold text-rtp-text">Mike R.</div>
						<div class="text-sm text-rtp-muted">Member since 2024</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</section>

<!-- Final CTA -->
<section class="py-20 bg-gradient-to-br from-rtp-primary to-rtp-indigo text-white">
	<div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
		<h2 class="text-3xl md:text-5xl font-heading font-bold mb-6">Ready to Join the Room?</h2>
		<p class="text-xl text-white/90 mb-8 max-w-2xl mx-auto">
			Start trading with professional guidance today. 30-day money-back guarantee.
		</p>
		<div class="flex flex-wrap gap-4 justify-center">
			<a
				href="#pricing"
				class="bg-white text-rtp-primary px-8 py-4 rounded-lg font-semibold hover:bg-rtp-bg transition-all hover:scale-105 shadow-lg"
			>
				Choose Your Plan
			</a>
			<a
				href="/resources"
				class="bg-white/10 backdrop-blur-sm text-white border-2 border-white/20 px-8 py-4 rounded-lg font-semibold hover:bg-white/20 transition-all"
			>
				View Resources
			</a>
		</div>
	</div>
</section>

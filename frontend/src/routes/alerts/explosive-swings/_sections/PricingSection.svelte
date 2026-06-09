<script lang="ts">
	/**
	 * Explosive Swings — Pricing Section
	 * Three-tier pricing with monthly/quarterly/annual toggle.
	 * Extracted from /alerts/explosive-swings/+page.svelte (Cascade audit, May 2026).
	 */

	let selectedPlan = $state<'monthly' | 'quarterly' | 'annual'>('quarterly');

	const pricing = {
		monthly: {
			price: '97',
			period: '/mo',
			btnText: 'Select Monthly',
			link: '/checkout/monthly-swings',
			tagline: 'Flexibility to cancel anytime'
		},
		quarterly: {
			price: '247',
			period: '/qtr',
			btnText: 'Join Quarterly',
			link: '/checkout/quarterly-swings',
			tagline: 'Save 15% ($8.20 / trading day)'
		},
		annual: {
			price: '927',
			period: '/yr',
			btnText: 'Select Annual',
			link: '/checkout/annual-swings',
			tagline: 'Like getting 2.5 months FREE'
		}
	};

	function getPlanButtonClass(plan: 'monthly' | 'quarterly' | 'annual'): string[] {
		return [
			'relative z-10 px-6 py-2 rounded-lg font-bold text-sm transition-colors duration-200',
			selectedPlan === plan ? 'text-white' : 'text-slate-400 hover:text-white'
		];
	}

	function getSelectorLeft(): string {
		if (selectedPlan === 'monthly') return '0.375rem';
		if (selectedPlan === 'quarterly') return 'calc(33.33% + 0.2rem)';
		return 'calc(66.66% + 0.1rem)';
	}

	function getSidePlanClass(plan: 'monthly' | 'annual'): string[] {
		const selectedClass =
			plan === 'monthly'
				? 'border-emerald-500 opacity-100 scale-105 shadow-xl shadow-emerald-500/10'
				: 'border-emerald-500 opacity-100 scale-105';

		return [
			'bg-slate-900 p-8 rounded-2xl border transition-all',
			selectedPlan === plan ? selectedClass : 'border-slate-800 opacity-70 hover:opacity-90'
		];
	}

	function getQuarterlyPlanClass(): string[] {
		return [
			'bg-slate-950 p-10 rounded-3xl border-2 shadow-2xl transform relative z-10 transition-all',
			selectedPlan === 'quarterly'
				? 'border-emerald-500 shadow-emerald-500/20 md:scale-110 opacity-100'
				: 'border-slate-800 shadow-slate-800/10 md:scale-100 opacity-70 hover:opacity-90'
		];
	}
</script>

<section id="pricing" class="py-24 bg-slate-900 border-t border-slate-800 overflow-hidden">
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
		<div class="text-center mb-16">
			<h2 class="text-3xl md:text-5xl font-heading font-bold text-white mb-6">Simple Pricing</h2>
			<p class="text-xl text-slate-400 max-w-3xl mx-auto">
				Pay for the subscription with your first successful swing trade.
			</p>
		</div>

		<div class="flex justify-center mb-16">
			<div
				class="bg-rtp-surface bg-slate-950 p-1.5 rounded-xl border border-slate-700/50 inline-flex relative"
			>
				<button
					type="button"
					onclick={() => (selectedPlan = 'monthly')}
					class={getPlanButtonClass('monthly')}>Monthly</button
				>
				<button
					type="button"
					onclick={() => (selectedPlan = 'quarterly')}
					class={getPlanButtonClass('quarterly')}>Quarterly</button
				>
				<button
					type="button"
					onclick={() => (selectedPlan = 'annual')}
					class={getPlanButtonClass('annual')}>Annual</button
				>

				<div
					class="absolute top-1.5 bottom-1.5 bg-emerald-600 rounded-lg shadow-md transition-all duration-300 ease-out"
					style:left={getSelectorLeft()}
					style:width="calc(33.33% - 0.4rem)"
				></div>
			</div>
		</div>

		<div class="grid md:grid-cols-3 gap-8 max-w-6xl mx-auto items-center overflow-visible">
			<div class={getSidePlanClass('monthly')}>
				<h3 class="text-xl font-bold text-white mb-4">Monthly</h3>
				<div class="flex items-baseline gap-1 mb-6">
					<span class="text-4xl font-bold text-white">${pricing.monthly.price}</span>
					<span class="text-slate-500">{pricing.monthly.period}</span>
				</div>
				<div class="text-xs font-mono text-slate-500 bg-slate-950 p-2 rounded mb-6 text-center">
					{pricing.monthly.tagline}
				</div>
				<ul class="space-y-4 mb-8 text-sm text-slate-400">
					<li class="flex gap-3">
						<span class="text-emerald-500">✓</span> 2-4 Premium Swings / Week
					</li>
					<li class="flex gap-3">
						<span class="text-emerald-500">✓</span> Instant SMS & Email Alerts
					</li>
					<li class="flex gap-3">
						<span class="text-emerald-500">✓</span> Private Discord Community
					</li>
					<li class="flex gap-3"><span class="text-emerald-500">✓</span> Entry & Exit Zones</li>
				</ul>
				<a
					href={pricing.monthly.link}
					class="block w-full py-3 bg-slate-950 border border-slate-800 text-white font-bold rounded-lg text-center hover:bg-white hover:text-black transition-colors"
					>{pricing.monthly.btnText}</a
				>
			</div>

			<div class={getQuarterlyPlanClass()}>
				<div
					class="absolute top-0 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-emerald-500 text-slate-900 px-4 py-1 rounded-full text-xs font-bold uppercase tracking-wide"
				>
					Most Popular
				</div>
				<h3 class="text-2xl font-bold text-white mb-4">Quarterly</h3>
				<div class="flex items-baseline gap-1 mb-6">
					<span class="text-5xl font-extrabold text-white">${pricing.quarterly.price}</span>
					<span class="text-slate-500">{pricing.quarterly.period}</span>
				</div>
				<div
					class="text-xs font-mono text-emerald-400 bg-emerald-500/10 p-2 rounded mb-6 text-center border border-emerald-500/30"
				>
					{pricing.quarterly.tagline}
				</div>
				<ul class="space-y-4 mb-8 text-sm text-white">
					<li class="flex gap-3">
						<span class="text-emerald-500 font-bold">✓</span>
						<span class="font-bold">Priority Support</span>
					</li>
					<li class="flex gap-3">
						<span class="text-emerald-500 font-bold">✓</span> 2-4 Premium Swings / Week
					</li>
					<li class="flex gap-3">
						<span class="text-emerald-500 font-bold">✓</span> Instant SMS & Email Alerts
					</li>
					<li class="flex gap-3">
						<span class="text-emerald-500 font-bold">✓</span> Private Discord Community
					</li>
					<li class="flex gap-3">
						<span class="text-emerald-500 font-bold">✓</span> Full Trade Management
					</li>
				</ul>
				<a
					href={pricing.quarterly.link}
					class="block w-full py-4 bg-emerald-500 text-slate-900 font-bold rounded-xl text-center hover:bg-emerald-400 transition-colors shadow-lg"
					>{pricing.quarterly.btnText}</a
				>
			</div>

			<div class={getSidePlanClass('annual')}>
				<h3 class="text-xl font-bold text-white mb-4">Annual</h3>
				<div class="flex items-baseline gap-1 mb-6">
					<span class="text-4xl font-bold text-white">${pricing.annual.price}</span>
					<span class="text-slate-500">{pricing.annual.period}</span>
				</div>
				<div class="text-xs font-mono text-emerald-400 bg-slate-950 p-2 rounded mb-6 text-center">
					{pricing.annual.tagline}
				</div>
				<ul class="space-y-4 mb-8 text-sm text-slate-400">
					<li class="flex gap-3">
						<span class="text-emerald-500">✓</span>
						<span class="font-bold">Bonus: Strategy Video Library</span>
					</li>
					<li class="flex gap-3">
						<span class="text-emerald-500">✓</span> 2-4 Premium Swings / Week
					</li>
					<li class="flex gap-3">
						<span class="text-emerald-500">✓</span> Instant SMS & Email Alerts
					</li>
					<li class="flex gap-3">
						<span class="text-emerald-500">✓</span> Private Discord Community
					</li>
				</ul>
				<a
					href={pricing.annual.link}
					class="block w-full py-3 bg-slate-950 border border-emerald-500 text-emerald-500 font-bold rounded-lg text-center hover:bg-emerald-500 hover:text-slate-900 transition-colors"
					>{pricing.annual.btnText}</a
				>
			</div>
		</div>
		<div class="mt-12 text-center">
			<p class="text-slate-500 text-sm">Secure checkout powered by Stripe. Cancel anytime.</p>
		</div>

		<p class="text-center text-slate-500 text-sm mt-6 flex items-center justify-center gap-2">
			<svg aria-hidden="true" class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"
				><path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
				/></svg
			>
			30-Day Money Back Guarantee.
		</p>
	</div>
</section>

<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { cubicOut } from 'svelte/easing';
	import IconQuote from '@tabler/icons-svelte-runes/icons/quote';
	import IconCircleCheckFilled from '@tabler/icons-svelte-runes/icons/circle-check-filled';
	import IconTrendingUp from '@tabler/icons-svelte-runes/icons/trending-up';
	import IconUserCircle from '@tabler/icons-svelte-runes/icons/user-circle';
	import IconChartDots from '@tabler/icons-svelte-runes/icons/chart-dots';
	import IconShieldCheck from '@tabler/icons-svelte-runes/icons/shield-check';
	import IconActivity from '@tabler/icons-svelte-runes/icons/activity';

	// --- Institutional Client Data ---
	const reviews = [
		{
			id: 'CL-8821',
			name: 'Marcus T.',
			role: 'Prop Desk Manager',
			location: 'London, UK',
			quote:
				"The risk protocols alone saved our desk six figures during the last CPI volatility. This isn't a course; it's an operating system for capital preservation.",
			metric: 'Drawdown Reduced',
			value: '-18.5%',
			trend: 'positive'
		},
		{
			id: 'CL-4920',
			name: 'Sarah J.',
			role: 'Independent Fund',
			location: 'New York, NY',
			quote:
				"I spent 3 years in retail 'discord' hell. One month in the institutional audit reset my entire view of liquidity. My execution drag is now virtually zero.",
			metric: 'Execution Alpha',
			value: '+12bps',
			trend: 'positive'
		},
		{
			id: 'CL-1049',
			name: 'David K.',
			role: 'Swing Trader',
			location: 'Singapore',
			quote:
				'The dark pool logic is frighteningly accurate. We are front-running moves that I used to chase. The mental clarity of having a structured invalidated level is priceless.',
			metric: 'Win Rate',
			value: '68.2%',
			trend: 'positive'
		},
		{
			id: 'CL-3392',
			name: 'Elena R.',
			role: 'Futures Scalper',
			location: 'Chicago, IL',
			quote:
				'Strictly business. No hype, no noise. Just level 2 analysis and probability mapping. If you treat trading as a career, this is the only room that matters.',
			metric: 'Profit Factor',
			value: '2.45',
			trend: 'positive'
		},
		{
			id: 'CL-5591',
			name: 'James H.',
			role: 'Options Desk',
			location: 'Toronto, CA',
			quote:
				"The 0DTE structure is surgical. We aren't gambling on gamma squeezes anymore; we are capturing defined volatility expansion. Professional grade.",
			metric: 'Sharpe Ratio',
			value: '1.92',
			trend: 'positive'
		},
		{
			id: 'CL-9920',
			name: 'Robert B.',
			role: 'Capital Growth',
			location: 'Sydney, AU',
			quote:
				"Scaled my account from mid-five to six figures in Q3 using the 'Compound Protocol.' The psychological sizing models removed the fear of pulling the trigger.",
			metric: 'YTD Return',
			value: '+41.8%',
			trend: 'positive'
		}
	];

	// --- Animation Logic ---
	let containerRef = $state<HTMLElement | null>(null);
	let mouse = $state({ x: 0, y: 0 });
	// ICT11+ Fix: Start false, set true in onMount to trigger in: transitions
	let isVisible = $state(false);

	const handleMouseMove = (e: MouseEvent) => {
		if (!containerRef) return;
		const rect = containerRef.getBoundingClientRect();
		mouse.x = e.clientX - rect.left;
		mouse.y = e.clientY - rect.top;
	};

	function heavySlide(_node: Element, { delay = 0, duration = 1000 }) {
		return {
			delay,
			duration,
			css: (t: number) => {
				const eased = cubicOut(t);
				return `opacity: ${eased}; transform: translateY(${(1 - eased) * 30}px);`;
			}
		};
	}

	// Trigger entrance animations when section scrolls into viewport
	onMount(() => {
		if (!browser) {
			isVisible = true;
			return;
		}

		queueMicrotask(() => {
			if (!containerRef) {
				isVisible = true;
				return;
			}

			const visibilityObserver = new IntersectionObserver(
				(entries) => {
					if (entries[0]?.isIntersecting) {
						isVisible = true;
						visibilityObserver.disconnect();
					}
				},
				{ threshold: 0.1, rootMargin: '50px' }
			);

			visibilityObserver.observe(containerRef);
		});
	});

	// Ticker Tape Data
	const tickerItems = Array(20)
		.fill(0)
		.map((_, i) => ({
			symbol: ['ES_F', 'NQ_F', 'SPX', 'VIX', 'AAPL', 'NVDA'][i % 6],
			price: (4000 + Math.random() * 1000).toFixed(2),
			change: (Math.random() * 2 - 0.5).toFixed(2) + '%'
		}));
</script>

<section
	bind:this={containerRef}
	onmousemove={handleMouseMove}
	class="relative py-32 px-6 bg-[#020202] overflow-hidden border-b border-white/5"
	aria-label="Client Performance Ledger"
>
	<div
		class="absolute inset-0 pointer-events-none opacity-[0.04] overflow-hidden flex flex-col justify-center gap-24 transform -rotate-6 scale-110"
	>
		<div class="flex gap-12 whitespace-nowrap animate-marquee-left text-xs font-mono text-white">
			{#each [...tickerItems, ...tickerItems, ...tickerItems] as item}
				<div class="flex gap-4">
					<span class="font-bold">{item.symbol}</span>
					<span>{item.price}</span>
					<span class={item.change.startsWith('-') ? 'text-red-500' : 'text-emerald-500'}
						>{item.change}</span
					>
				</div>
			{/each}
		</div>
		<div class="flex gap-12 whitespace-nowrap animate-marquee-right text-xs font-mono text-white">
			{#each [...tickerItems, ...tickerItems, ...tickerItems] as item}
				<div class="flex gap-4">
					<span class="font-bold">{item.symbol}</span>
					<span>{item.price}</span>
					<span class={item.change.startsWith('-') ? 'text-red-500' : 'text-emerald-500'}
						>{item.change}</span
					>
				</div>
			{/each}
		</div>
	</div>

	<div
		class="absolute inset-0 pointer-events-none bg-gradient-to-b from-[#020202] via-transparent to-[#020202]"
	></div>
	<div
		class="absolute inset-0 pointer-events-none opacity-40 transition-opacity duration-300"
		style="background: radial-gradient(1000px circle at var(--x) var(--y), rgba(16, 185, 129, 0.03), transparent 50%);"
	></div>

	<div class="relative max-w-[1600px] mx-auto z-10">
		<div class="max-w-4xl mx-auto text-center mb-24">
			{#if isVisible}
				<div
					in:heavySlide={{ delay: 0, duration: 1000 }}
					class="inline-flex items-center gap-3 px-4 py-1.5 border border-emerald-900/30 bg-emerald-950/10 text-emerald-500 text-[10px] font-bold tracking-[0.3em] uppercase mb-8 rounded-sm"
				>
					<IconShieldCheck size={14} />
					Verified Performance
				</div>

				<h2
					in:heavySlide={{ delay: 100 }}
					class="text-5xl md:text-7xl font-serif text-white mb-8 tracking-tight"
				>
					Performance <span class="text-slate-700">Attribution.</span>
				</h2>

				<p
					in:heavySlide={{ delay: 200 }}
					class="text-lg text-slate-400 font-light leading-relaxed max-w-2xl mx-auto"
				>
					We don't rely on marketing claims. We rely on the PnL of our desk. Verified feedback from
					funded traders, prop managers, and institutional clients.
				</p>
			{/if}
		</div>

		<div class="columns-1 md:columns-2 lg:columns-3 gap-6 space-y-6">
			{#each reviews as review, i}
				{#if isVisible}
					<div
						in:heavySlide={{ delay: 300 + i * 100 }}
						class="break-inside-avoid relative group bg-[#050505] border border-white/10 p-8 hover:bg-[#080808] hover:border-emerald-500/30 transition-all duration-500"
					>
						<div class="flex justify-between items-start mb-6 pb-6 border-b border-white/5">
							<div class="flex items-center gap-3">
								<div
									class="w-8 h-8 rounded-full bg-white/5 flex items-center justify-center text-slate-500"
								>
									<IconUserCircle size={20} />
								</div>
								<div>
									<div class="text-sm font-medium text-white">{review.name}</div>
									<div class="text-[10px] font-mono uppercase text-slate-500 tracking-wide">
										{review.role}
									</div>
								</div>
							</div>
							<div
								class="text-[10px] font-mono text-emerald-500/80 border border-emerald-900/30 px-2 py-1 bg-emerald-950/10 rounded"
							>
								{review.id}
							</div>
						</div>

						<div class="relative mb-8">
							<IconQuote
								size={24}
								class="absolute -top-2 -left-2 text-white/5 transform -scale-x-100"
							/>
							<p class="relative z-10 text-slate-300 font-light leading-relaxed text-sm">
								"{review.quote}"
							</p>
						</div>

						<div
							class="flex items-center justify-between bg-white/[0.02] -mx-8 -mb-8 px-8 py-4 border-t border-white/5 group-hover:bg-emerald-950/[0.05] transition-colors duration-500"
						>
							<div class="flex flex-col">
								<span class="text-[10px] font-mono uppercase text-slate-500 tracking-widest mb-1">
									{review.metric}
								</span>
								<div class="flex items-center gap-2">
									<IconTrendingUp size={14} class="text-emerald-500" />
									<span class="text-lg font-serif text-white">{review.value}</span>
								</div>
							</div>
							<div
								class="opacity-0 group-hover:opacity-100 transition-opacity duration-500 flex items-center gap-1.5 text-[10px] font-bold text-emerald-500 uppercase tracking-widest"
							>
								<IconCircleCheckFilled size={14} />
								Verified
							</div>
						</div>

						<div class="absolute top-0 right-0 w-8 h-8 pointer-events-none overflow-hidden">
							<div
								class="absolute top-0 right-0 w-2 h-2 bg-emerald-500/0 group-hover:bg-emerald-500/50 transition-colors duration-500"
							></div>
							<svg
								class="absolute top-0 right-0 w-full h-full text-emerald-500/20 opacity-0 group-hover:opacity-100 transition-opacity duration-500"
								viewBox="0 0 24 24"
							>
								<path d="M24 0 L0 0" stroke="currentColor" stroke-width="1" />
								<path d="M24 0 L24 24" stroke="currentColor" stroke-width="1" />
							</svg>
						</div>
					</div>
				{/if}
			{/each}
		</div>

		{#if isVisible}
			<div
				in:heavySlide={{ delay: 600 }}
				class="mt-20 pt-10 border-t border-white/5 flex flex-wrap justify-center gap-12 lg:gap-24 opacity-50 grayscale hover:grayscale-0 transition-all duration-500"
			>
				<div class="flex items-center gap-3 text-white/60">
					<IconChartDots size={24} />
					<span class="text-xs font-mono uppercase tracking-widest">Audited by NinjaTrader</span>
				</div>
				<div class="flex items-center gap-3 text-white/60">
					<IconShieldCheck size={24} />
					<span class="text-xs font-mono uppercase tracking-widest">MyFxBook Verified</span>
				</div>
				<div class="flex items-center gap-3 text-white/60">
					<IconActivity size={24} />
					<span class="text-xs font-mono uppercase tracking-widest">Institutional Data Feed</span>
				</div>
			</div>
		{/if}
	</div>
</section>

<style>
	/* Marquee Animations for Ticker Tape */
	@keyframes marquee-left {
		0% {
			transform: translateX(0);
		}
		100% {
			transform: translateX(-50%);
		}
	}
	@keyframes marquee-right {
		0% {
			transform: translateX(-50%);
		}
		100% {
			transform: translateX(0);
		}
	}

	.animate-marquee-left {
		animation: marquee-left 40s linear infinite;
	}
	.animate-marquee-right {
		animation: marquee-right 40s linear infinite;
	}
</style>

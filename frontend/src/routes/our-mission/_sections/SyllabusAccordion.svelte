<script lang="ts">
	/**
	 * Syllabus Section — From Subjective to Objective
	 * Accordion of curriculum modules (M1..M6).
	 * Extracted from /our-mission/+page.svelte (Cascade audit, May 2026).
	 */
	import { slide } from 'svelte/transition';
	import IconBook from '@tabler/icons-svelte-runes/icons/book';
	import IconCheck from '@tabler/icons-svelte-runes/icons/check';
	import IconChevronDown from '@tabler/icons-svelte-runes/icons/chevron-down';

	const syllabus = [
		{
			title: 'Module 1: Market Microstructure',
			desc: 'Understanding the Auction Process, Bid/Ask mechanics, and how liquidity moves price. We deconstruct the Limit Order Book (LOB) and aggressive vs passive order flow.'
		},
		{
			title: 'Module 2: Volume Profiling',
			desc: "Identifying value areas, POC (Point of Control), and composite profiles. Learning to spot 'trapped' traders and liquidity voids where price accelerates."
		},
		{
			title: 'Module 3: Institutional Order Flow',
			desc: 'Reading the DOM (Depth of Market), Delta Divergence, and Footprint charts. Identifying iceberg orders and absorption at key levels.'
		},
		{
			title: 'Module 4: Advanced Risk Modeling',
			desc: 'Building a personalized risk management plan based on Kelly Criterion, Sharpe Ratio optimization, and drawdown recovery protocols.'
		},
		{
			title: 'Module 5: The Psychology of Variance',
			desc: "Cognitive Behavioral Therapy (CBT) techniques for traders. Dealing with 'Recency Bias', 'Tilt', and the physiological response to risk."
		},
		{
			title: 'Module 6: System Architecture',
			desc: 'Building your Playbook. Defining A+ setups versus B setups. Creating a trade execution checklist that removes discretionary errors.'
		}
	];

	let openIndex = $state<number | null>(0);
	const toggle = (idx: number) => (openIndex = openIndex === idx ? null : idx);
</script>

<section class="py-32 bg-[#020202] border-t border-white/5">
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
		<div class="grid lg:grid-cols-12 gap-20">
			<div data-gsap class="lg:col-span-5 sticky top-32 h-fit">
				<div class="flex items-center gap-3 mb-6">
					<span class="p-2 bg-blue-500/10 rounded-lg text-blue-500 border border-blue-500/20"
						><IconBook size={20} stroke={1.5} /></span
					>
					<span class="font-bold tracking-widest uppercase text-sm text-blue-400">The Syllabus</span
					>
				</div>
				<h2 class="text-4xl md:text-5xl font-heading font-bold text-white mb-8">
					From Subjective to <br />
					<span class="text-white border-b-4 border-rtp-primary">Objective.</span>
				</h2>
				<p class="text-lg text-slate-400 mb-8 leading-relaxed">
					Most traders look at a chart and see "patterns." We look at a chart and see <strong
						>Auctions</strong
					>.
				</p>
				<p class="text-lg text-slate-400 mb-12 leading-relaxed">
					We teach you to read the raw data of the market: Volume, Liquidity, and Time. This allows
					you to identify where the "Smart Money" is transacting, not just where price has been.
				</p>

				<div class="p-1 bg-white/5 rounded-xl border border-white/10">
					<div
						class="aspect-video bg-[#0f172a] rounded-lg flex items-center justify-center relative overflow-hidden group"
					>
						<div
							class="absolute inset-0 bg-[linear-gradient(0deg,rgba(255,255,255,0.03)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.03)_1px,transparent_1px)] bg-size-[20px_20px]"
						></div>
						<div class="absolute left-0 bottom-20 h-4 w-32 bg-blue-500/20 rounded-r"></div>
						<div class="absolute left-0 bottom-24 h-4 w-48 bg-blue-500/40 rounded-r"></div>
						<div class="absolute left-0 bottom-28 h-4 w-64 bg-blue-500/60 rounded-r"></div>
						<div class="absolute left-0 bottom-32 h-4 w-40 bg-blue-500/40 rounded-r"></div>
						<div class="absolute left-0 bottom-36 h-4 w-24 bg-blue-500/20 rounded-r"></div>

						<div class="absolute right-10 top-10 text-right">
							<div class="text-xs text-slate-500 font-mono">POC: 4450.25</div>
							<div class="text-xs text-slate-500 font-mono">VAH: 4462.00</div>
							<div class="text-xs text-slate-500 font-mono">VAL: 4438.50</div>
						</div>
						<p
							class="text-center text-xs text-slate-500 mt-24 font-mono uppercase tracking-widest z-10"
						>
							Figure 1.1: Auction Market Theory
						</p>
					</div>
				</div>
			</div>

			<div class="lg:col-span-7 space-y-4 pt-8">
				{#each syllabus as module, i (module.title)}
					<div
						class="border border-white/10 rounded-xl bg-[#0a0a0a] overflow-hidden group hover:border-rtp-primary/50 transition-all duration-300"
					>
						<button
							class="w-full flex justify-between items-center p-6 text-left"
							onclick={() => toggle(i)}
						>
							<div class="flex items-center gap-6">
								<span
									class="text-rtp-primary font-mono text-sm bg-rtp-primary/10 px-3 py-1 rounded border border-rtp-primary/20"
									>MOD {i + 1}</span
								>
								<span
									class="font-bold text-white text-lg group-hover:text-rtp-primary transition-colors"
									>{module.title}</span
								>
							</div>
							<div
								class="w-8 h-8 rounded-full bg-white/5 flex items-center justify-center text-slate-500 transform transition-transform duration-300 {openIndex ===
								i
									? 'rotate-180 bg-white/10 text-white'
									: ''}"
							>
								<IconChevronDown size={20} stroke={1.5} />
							</div>
						</button>
						{#if openIndex === i}
							<div
								transition:slide
								class="px-6 pb-8 pl-22 text-slate-400 leading-relaxed border-t border-white/5 pt-6 text-sm"
							>
								{module.desc}
								<div class="mt-4 flex gap-4">
									<span class="text-xs font-mono text-emerald-400 flex items-center gap-1"
										><IconCheck size={12} stroke={2} /> Video</span
									>
									<span class="text-xs font-mono text-emerald-400 flex items-center gap-1"
										><IconCheck size={12} stroke={2} /> PDF</span
									>
									<span class="text-xs font-mono text-emerald-400 flex items-center gap-1"
										><IconCheck size={12} stroke={2} /> Quiz</span
									>
								</div>
							</div>
						{/if}
					</div>
				{/each}
			</div>
		</div>
	</div>
</section>

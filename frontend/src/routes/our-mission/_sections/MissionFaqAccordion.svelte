<script lang="ts">
	/**
	 * Common Questions / FAQ Accordion
	 * Extracted from /our-mission/+page.svelte (Cascade audit, May 2026).
	 */
	import { slide } from 'svelte/transition';
	import IconChevronDown from '@tabler/icons-svelte-runes/icons/chevron-down';

	const faqs = [
		{
			q: 'Is this a "Get Rich Quick" scheme?',
			a: 'Absolutely not. If you are looking for fast money, please look elsewhere. We teach a profession that takes years to master. We promise hard work, not Lamborghinis.'
		},
		{
			q: 'Do I need a large account to start?',
			a: 'No. In fact, we recommend starting with a small account or a simulator. Institutional risk management concepts (Position Sizing, R-Multiples) apply whether you are trading $1,000 or $1,000,000.'
		},
		{
			q: 'What trading style do you teach?',
			a: 'We are primarily Day Traders and Swing Traders. We focus on Index Futures (ES, NQ) and Large Cap Tech Options. Our methodology is based on Price Action, Volume Profile, and Order Flow.'
		}
	];

	let openIndex = $state<number | null>(null);
	const toggle = (idx: number) => (openIndex = openIndex === idx ? null : idx);
</script>

<section class="py-24 bg-white/2 border-t border-white/5">
	<div class="max-w-3xl mx-auto px-4">
		<div class="text-center mb-12">
			<h2 class="text-3xl font-bold text-white mb-4">Common Questions</h2>
			<p class="text-slate-400">Understanding our philosophy before you join.</p>
		</div>

		<div class="space-y-4">
			{#each faqs as faq, i (faq.q)}
				<div class="border border-white/10 rounded-lg bg-[#0a0a0a] overflow-hidden">
					<button
						class="w-full flex justify-between items-center p-6 text-left hover:bg-white/5 transition-colors"
						onclick={() => toggle(i)}
						aria-expanded={openIndex === i}
						aria-controls="mission-faq-{i}"
					>
						<span class="font-bold text-white">{faq.q}</span>
						<IconChevronDown
							size={20}
							stroke={1.5}
							class={`text-slate-500 transition-transform duration-300 ${openIndex === i ? 'rotate-180' : ''}`}
						/>
					</button>
					{#if openIndex === i}
						<div
							id="mission-faq-{i}"
							role="region"
							transition:slide
							class="px-6 pb-6 text-slate-400 leading-relaxed border-t border-white/5 pt-4"
						>
							{faq.a}
						</div>
					{/if}
				</div>
			{/each}
		</div>
	</div>
</section>

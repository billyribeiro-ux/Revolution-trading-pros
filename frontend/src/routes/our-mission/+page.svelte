<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import IconTrendingUp from '@tabler/icons-svelte-runes/icons/trending-up';
	import IconShield from '@tabler/icons-svelte-runes/icons/shield';
	import IconUsers from '@tabler/icons-svelte-runes/icons/users';
	import IconScale from '@tabler/icons-svelte-runes/icons/scale';
	import IconBrain from '@tabler/icons-svelte-runes/icons/brain';
	import IconArrowRight from '@tabler/icons-svelte-runes/icons/arrow-right';
	import IconCheck from '@tabler/icons-svelte-runes/icons/check';
	import IconQuote from '@tabler/icons-svelte-runes/icons/quote';
	import IconSearch from '@tabler/icons-svelte-runes/icons/search';
	import MathOfEdgeCalculator from './_sections/MathOfEdgeCalculator.svelte';
	import SyllabusAccordion from './_sections/SyllabusAccordion.svelte';
	import MissionFaqAccordion from './_sections/MissionFaqAccordion.svelte';

	onMount(() => {
		if (!browser) return;

		let ctx: ReturnType<typeof import('gsap').gsap.context> | null = null;

		(async () => {
			const { gsap } = await import('gsap');
			const { ScrollTrigger } = await import('gsap/ScrollTrigger');
			gsap.registerPlugin(ScrollTrigger);

			const prefersReducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
			if (prefersReducedMotion) {
				gsap.set('[data-gsap]', { opacity: 1, y: 0 });
				return;
			}

			ctx = gsap.context(() => {
				const all = document.querySelectorAll('[data-gsap]');
				gsap.set(all, { opacity: 0, y: 30 });

				const aboveFold: Element[] = [];
				const belowFold: Element[] = [];
				all.forEach((el) => {
					const top = el.getBoundingClientRect().top;
					if (top < window.innerHeight * 0.85) aboveFold.push(el);
					else belowFold.push(el);
				});

				if (aboveFold.length > 0) {
					gsap.to(aboveFold, { opacity: 1, y: 0, duration: 0.9, ease: 'power3.out', stagger: 0.1 });
				}

				if (belowFold.length > 0) {
					ScrollTrigger.batch(belowFold, {
						onEnter: (batch) => {
							gsap.to(batch, {
								opacity: 1,
								y: 0,
								duration: 0.8,
								ease: 'power3.out',
								stagger: 0.1,
								overwrite: true
							});
						},
						start: 'top 85%',
						once: true
					});
				}

				ScrollTrigger.refresh();
			});
		})();

		return () => ctx?.revert();
	});

	let glossarySearch = $state('');

	const pillars = [
		{
			icon: IconShield,
			title: 'Radical Transparency',
			desc: 'We operate in a glass house. We publish raw track records, including losses and commissions. In an industry of smoke and mirrors, truth is the only currency.',
			variant: 'emerald' as const
		},
		{
			icon: IconUsers,
			title: 'Collective Intelligence',
			desc: 'Trading is an isolation sport. We replace that with a hive-mind of disciplined professionals sharing real-time order flow, risk assessments, and sentiment. We win together.',
			variant: 'blue' as const
		},
		{
			icon: IconScale,
			title: 'Probabilistic Thinking',
			desc: 'We reject gambling. We teach the mathematics of "Edge"—thinking in Expected Value (EV), Standard Deviation, and R-Multiples. We do not predict; we react to probability.',
			variant: 'purple' as const
		}
	];

	const glossary = [
		{
			term: 'Expectancy',
			def: 'The average amount you can expect to win (or lose) per dollar at risk over N trades.'
		},
		{
			term: 'R-Multiple',
			def: 'A standardized unit of risk. We measure success in R units, not dollars.'
		},
		{
			term: 'Auction Theory',
			def: 'The market is a two-way auction process seeking to facilitate trade and find fair value.'
		},
		{
			term: 'Alpha Decay',
			def: 'The reduction in edge effectiveness as more participants exploit the same inefficiency.'
		},
		{
			term: 'Liquidity',
			def: 'The ability to enter or exit a position without significant price impact (slippage).'
		},
		{
			term: 'Variance',
			def: "The statistical deviation from expected results. The 'luck' factor in the short term."
		},
		{
			term: 'Absorption',
			def: 'When aggressive buying/selling is met with passive limit orders, halting price movement.'
		},
		{
			term: 'Delta',
			def: 'The difference between buying volume and selling volume at a specific price node.'
		},
		{
			term: 'Gamma',
			def: 'The rate of change of Delta. Crucial for understanding options dealer hedging flows.'
		},
		{ term: 'Vanna', def: 'The sensitivity of Delta to changes in Implied Volatility.' },
		{
			term: 'Dark Pools',
			def: 'Private exchanges for trading securities that are not accessible by the investing public.'
		},
		{
			term: 'Drawdown',
			def: 'The peak-to-trough decline during a specific record period of an investment.'
		}
	];

	let filteredGlossary = $derived(
		glossary.filter((g) => g.term.toLowerCase().includes(glossarySearch.toLowerCase()))
	);

	const axioms = [
		'Risk comes first.',
		"Don't predict, react.",
		'Cash is a position.',
		'Silence the noise.',
		'Trade the chart, not the P&L.',
		'Patience pays.',
		'Routine equals results.'
	];

	const manifesto = [
		'Capital preservation is the primary objective.',
		"We never sell 'signals' without explaining the logic.",
		'Trading is a business of risk, not a casino.',
		'Consistency is a result of routine, not luck.',
		'Losses are tuition fees for the market.',
		'We are fiduciaries to our own education.'
	];
</script>

<div class="mission">
	<div class="mission__atmosphere" aria-hidden="true">
		<div class="mission__halo mission__halo--top"></div>
		<div class="mission__noise"></div>
		<div class="mission__halo mission__halo--bottom"></div>
	</div>

	<div class="mission__content">
		<div class="ticker">
			<div class="ticker__track">
				{#each [...axioms, ...axioms] as axiom, _ai (_ai)}
					<div class="ticker__item">
						<span class="ticker__dot" aria-hidden="true">●</span>
						{axiom}
					</div>
				{/each}
			</div>
		</div>

		<section class="hero">
			<div data-gsap class="hero__badge">
				<span class="hero__badge-dot" aria-hidden="true"></span>
				<span class="hero__badge-label">Mission Control</span>
			</div>

			<h1 data-gsap data-speakable class="hero__title">
				We Don't Sell Dreams.<br />
				We Build <span class="hero__title-accent">Careers.</span>
			</h1>

			<div data-gsap class="hero__lede">
				<p data-speakable>
					Our mission is to dismantle the "Retail Trader" stereotype and rebuild it with
					<span class="hero__lede-emph">Institutional DNA</span>. We exist to transition you from a
					gambler seeking action to a risk manager executing an edge.
				</p>
			</div>

			<div data-gsap class="hero__stats">
				<div class="stat-card">
					<div class="stat-card__value">90%</div>
					<div class="stat-card__label">Retail Failure Rate</div>
				</div>
				<div class="stat-card">
					<div class="stat-card__value stat-card__value--primary">10+</div>
					<div class="stat-card__label">Years Experience</div>
				</div>
				<div class="stat-card">
					<div class="stat-card__value stat-card__value--emerald">100%</div>
					<div class="stat-card__label">Verified Audits</div>
				</div>
				<div class="stat-card">
					<div class="stat-card__value">$0</div>
					<div class="stat-card__label">Hidden Fees</div>
				</div>
			</div>
		</section>

		<section class="reality">
			<div class="reality__shell">
				<div class="reality__grid">
					<div class="reality__copy" data-gsap>
						<div class="reality__chip">
							<span class="reality__chip-icon"><IconTrendingUp size={20} stroke={1.5} /></span>
							<span class="reality__chip-label">The Reality Check</span>
						</div>

						<h2 class="reality__title">The "90/90/90" Rule is Real.</h2>

						<div class="reality__prose">
							<p>
								It is a known statistic in the brokerage industry: <strong
									>90% of retail traders lose 90% of their money in their first 90 days.</strong
								>
							</p>
							<p>
								Why? Because they enter a battlefield against supercomputers, HFT algorithms, and
								hedge funds armed only with a smartphone and "hope." They lack the mathematical
								framework to survive variance.
							</p>
							<div class="reality__callout">
								<h4 class="reality__callout-title">
									<IconBrain size={16} stroke={1.5} /> The Cognitive Gap
								</h4>
								<p class="reality__callout-body">
									Retail traders seek dopamine hits (action). Institutional traders seek boredom
									(execution). The market is designed to transfer wealth from the impatient to the
									patient.
								</p>
							</div>
						</div>
					</div>

					<div class="reality__table-wrap" data-gsap>
						<div class="compare-table">
							<div class="compare-table__rule" aria-hidden="true"></div>
							<div class="compare-table__scroll">
								<table>
									<thead>
										<tr>
											<th class="compare-table__th compare-table__th--label">Metric</th>
											<th class="compare-table__th compare-table__th--amateur">The Amateur</th>
											<th class="compare-table__th compare-table__th--pro">The Professional</th>
										</tr>
									</thead>
									<tbody>
										<tr>
											<td class="compare-table__td compare-table__td--label">Focus</td>
											<td class="compare-table__td compare-table__td--amateur">Profit (P&amp;L)</td>
											<td class="compare-table__td compare-table__td--pro"
												>Process &amp; Execution</td
											>
										</tr>
										<tr>
											<td class="compare-table__td compare-table__td--label">Risk Mgmt</td>
											<td class="compare-table__td compare-table__td--amateur"
												>Arbitrary / Emotion</td
											>
											<td class="compare-table__td compare-table__td--pro"
												>Fixed % (Kelly Criterion)</td
											>
										</tr>
										<tr>
											<td class="compare-table__td compare-table__td--label">Data Source</td>
											<td class="compare-table__td compare-table__td--amateur"
												>Lagging Indicators (RSI)</td
											>
											<td class="compare-table__td compare-table__td--pro"
												>Order Flow &amp; Volume</td
											>
										</tr>
										<tr>
											<td class="compare-table__td compare-table__td--label">Timeframe</td>
											<td class="compare-table__td compare-table__td--amateur"
												>Immediate Gratification</td
											>
											<td class="compare-table__td compare-table__td--pro">Multi-Quarter Growth</td>
										</tr>
										<tr>
											<td class="compare-table__td compare-table__td--label">Losses</td>
											<td class="compare-table__td compare-table__td--amateur">Source of Anger</td>
											<td class="compare-table__td compare-table__td--pro">Cost of Business</td>
										</tr>
									</tbody>
								</table>
							</div>
						</div>
					</div>
				</div>
			</div>
		</section>

		<MathOfEdgeCalculator />

		<section class="ecosystem">
			<div class="ecosystem__halo" aria-hidden="true"></div>

			<div class="ecosystem__shell">
				<div class="ecosystem__head">
					<div data-gsap class="ecosystem__eyebrow-wrap">
						<span class="ecosystem__eyebrow">The Ecosystem</span>
					</div>
					<h2 data-gsap class="ecosystem__title">
						Institutional Grade.<br />Retail Accessible.
					</h2>
					<p data-gsap class="ecosystem__lede">
						We built the environment we wished existed when we started. A sanctuary of data,
						discipline, and truth.
					</p>
				</div>

				<div class="pillars">
					{#each pillars as pillar, _i (pillar.title)}
						{@const PillarIcon = pillar.icon}
						<div data-gsap class={['pillar', `pillar--${pillar.variant}`]}>
							<div class="pillar__shadow" aria-hidden="true"></div>
							<div class="pillar__card">
								<div class="pillar__icon">
									<PillarIcon size={28} stroke={1.5} />
								</div>
								<h3 class="pillar__title">{pillar.title}</h3>
								<p class="pillar__desc">{pillar.desc}</p>
								<div class="pillar__cta">
									Learn More <IconArrowRight size={16} stroke={2} />
								</div>
							</div>
						</div>
					{/each}
				</div>
			</div>
		</section>

		<section class="note">
			<div class="note__shell">
				<div class="note__sigil-wrap">
					<div class="note__sigil">R</div>
				</div>
				<div class="note__prose">
					<h3 class="note__title">A Note from the Desk</h3>
					<p>
						I started Revolution Trading Pros because I was tired of seeing good people lose money
						to bad advice. The internet is flooded with "gurus" renting Lamborghinis, selling the
						dream of easy money.
					</p>
					<p>
						<strong>Trading is not easy.</strong> It is the hardest way to make an easy living.
					</p>
					<p>
						When I worked on the institutional side, I saw how the sausage was made. I saw the
						algorithms designed to hunt retail stop losses. I saw the order flow that moves markets
						before the news even hits your feed. I realized that the retail trader is playing a game
						they don't even understand.
					</p>
					<blockquote class="note__quote">
						"The gap between 'Retail' and 'Pro' isn't intelligence. It's information and
						discipline."
					</blockquote>
					<p>
						We built this platform to bridge that gap. To give you the tools (Bookmap, Gamma
						Exposure), the data, and the community you need to survive the learning curve and thrive
						in the volatility.
					</p>
					<p>
						We don't promise you'll get rich quick. We promise we will tell you the truth about what
						it takes. We promise to treat you like a professional from day one.
					</p>
					<div class="note__sig">
						<div class="note__avatar">B</div>
						<div>
							<p class="note__sig-name">Billy Ribeiro</p>
							<p class="note__sig-role">Founder &amp; Head Trader</p>
						</div>
					</div>
				</div>
			</div>
		</section>

		<SyllabusAccordion />

		<section class="lexicon">
			<div class="lexicon__shell">
				<div class="lexicon__head">
					<div>
						<h2 class="lexicon__title">The Institutional Lexicon</h2>
						<p class="lexicon__lede">We speak the language of the market. You will too.</p>
					</div>
					<div class="lexicon__search">
						<span class="lexicon__search-icon" aria-hidden="true">
							<IconSearch size={16} stroke={1.5} />
						</span>
						<input
							id="page-glossarysearch"
							name="page-glossarysearch"
							type="text"
							bind:value={glossarySearch}
							placeholder="Search terms..."
							class="lexicon__input"
							autocomplete="off"
						/>
					</div>
				</div>

				<div class="lexicon__grid">
					{#each filteredGlossary as item (item.term)}
						<div class="lex-card">
							<h4 class="lex-card__term">{item.term}</h4>
							<p class="lex-card__def">{item.def}</p>
						</div>
					{/each}
				</div>
			</div>
		</section>

		<section class="manifesto">
			<div class="manifesto__shell">
				<div data-gsap class="manifesto__card">
					<div class="manifesto__quote-bg" aria-hidden="true">
						<IconQuote size={256} />
					</div>

					<div class="manifesto__inner">
						<div class="manifesto__head">
							<h2 class="manifesto__title">The Operator's Manifesto</h2>
							<div class="manifesto__rule" aria-hidden="true"></div>
						</div>

						<div class="manifesto__grid">
							{#each manifesto as item (item)}
								<div class="manifesto__item">
									<div class="manifesto__check" aria-hidden="true">
										<IconCheck size={16} stroke={2} />
									</div>
									<span class="manifesto__text">{item}</span>
								</div>
							{/each}
						</div>

						<div class="manifesto__footer">
							<p>Est. 2023 // New York • London • Tokyo</p>
						</div>
					</div>
				</div>
			</div>
		</section>

		<MissionFaqAccordion />

		<section class="cta-final">
			<div class="cta-final__halo" aria-hidden="true"></div>

			<div data-gsap class="cta-final__shell">
				<div class="cta-final__pulse">
					<IconBrain size={48} stroke={1.5} />
				</div>

				<h2 class="cta-final__title">
					Stop Gambling. <br />
					Start <span class="cta-final__accent">Operating.</span>
				</h2>

				<p class="cta-final__lede">
					Join the only trading environment dedicated to the professional development of the retail
					trader. Your career starts here.
				</p>

				<div class="cta-final__actions">
					<a href="/pricing" class="btn btn--primary">
						<span>Join the Professional Tier</span>
						<span class="btn__arrow" aria-hidden="true">
							<IconArrowRight size={20} stroke={2} />
						</span>
					</a>
					<a href="/methodology" class="btn btn--ghost">Explore Our Methodology</a>
				</div>

				<p class="cta-final__fine-print">30-Day Money Back Guarantee • Cancel Anytime</p>
			</div>
		</section>
	</div>
</div>

<style>
	/* ─────────────────────────────────────────────────────────────────
	   Pre-hide data-gsap elements so the GSAP entrance animation has a
	   from-state without a flash-of-visible-content during the dynamic
	   import. transition: none prevents Tailwind's transition-all from
	   interpolating against GSAP's per-frame opacity ticks. Reduced-motion
	   users (and no-JS) bypass these rules.
	   ───────────────────────────────────────────────────────────────── */
	:global([data-gsap]) {
		opacity: 0;
		transform: translateY(30px);
		transition: none;
	}
	@media (prefers-reduced-motion: reduce) {
		:global([data-gsap]) {
			opacity: 1;
			transform: none;
			transition: revert;
		}
	}

	/* ─────────────────────────────────────────────────────────────────
	   Page root
	   ───────────────────────────────────────────────────────────────── */
	.mission {
		--m-primary-mix-30: color-mix(in oklab, var(--rtp-primary) 30%, transparent);
		--m-primary-mix-20: color-mix(in oklab, var(--rtp-primary) 20%, transparent);
		--m-primary-mix-10: color-mix(in oklab, var(--rtp-primary) 10%, transparent);
		--m-primary-mix-5: color-mix(in oklab, var(--rtp-primary) 5%, transparent);
		--m-primary-glow: rgba(59, 130, 246, 0.3);
		--m-primary-glow-soft: rgba(59, 130, 246, 0.1);
		--m-bg-card: var(--rtp-bg);
		--m-bg-card-elev: #0f172a;
		--m-emerald-bright: var(--rtp-emerald-bright);
		--m-blue-bright: var(--rtp-blue-bright);
		--m-purple-bright: #a78bfa;
		--m-red-bright: #f87171;

		background: var(--rtp-bg-deep);
		color: var(--rtp-text-soft);
		font-family: var(--rtp-font-sans);
		position: relative;
		overflow-x: clip;
	}
	.mission ::selection {
		background: var(--m-primary-mix-30);
		color: #fff;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Atmosphere (fixed background halos + noise)
	   ───────────────────────────────────────────────────────────────── */
	.mission__atmosphere {
		position: fixed;
		inset: 0;
		z-index: 0;
		pointer-events: none;
	}
	.mission__halo {
		position: absolute;
		border-radius: 50%;
		mix-blend-mode: screen;
	}
	.mission__halo--top {
		inset-block-start: -10%;
		inset-inline-start: 50%;
		translate: -50% 0;
		width: 1200px;
		height: 800px;
		background: var(--m-primary-mix-5);
		filter: blur(120px);
	}
	.mission__halo--bottom {
		inset-block-end: 0;
		inset-inline-end: 0;
		width: 800px;
		height: 600px;
		background: color-mix(in oklab, var(--rtp-indigo) 5%, transparent);
		filter: blur(100px);
	}
	.mission__noise {
		position: absolute;
		inset: 0;
		opacity: 0.2;
		background-image: url('data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iNjAiIGhlaWdodD0iNjAiIHZpZXdCb3g9IjAgMCA2MCA2MCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj48cGF0aCBkPSJNNTkuNSA2MEg2MHYtMC41SDU5LjVWNTBINjB2LTAuNUg1OS41VjQwSDYwdi0wLjVINTkuNVYzMEg2MHYtMC41SDU5LjVWMjBINjB2LTAuNUg1OS41VjEwSDYwdi0wLjVINTkuNVYwSDYwdjYwaC0wLjV6IiBmaWxsPSIjZmZmIiBmaWxsLW9wYWNpdHk9IjAuMDIiLz48L3N2Zz4=');
	}
	.mission__content {
		position: relative;
		z-index: 10;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Ticker
	   ───────────────────────────────────────────────────────────────── */
	.ticker {
		width: 100%;
		background: var(--m-primary-mix-10);
		border-block-end: 1px solid var(--m-primary-mix-20);
		overflow: hidden;
		padding-block: 0.5rem;
	}
	.ticker__track {
		display: flex;
		white-space: nowrap;
		animation: ticker 30s linear infinite;
	}
	.ticker__item {
		display: flex;
		align-items: center;
		margin-inline: 2rem;
		font-size: 0.75rem;
		font-family: var(--rtp-font-mono);
		font-weight: 700;
		color: var(--rtp-primary);
		text-transform: uppercase;
		letter-spacing: 0.2em;
	}
	.ticker__dot {
		margin-inline-end: 0.5rem;
	}

	@keyframes ticker {
		0% {
			transform: translateX(0);
		}
		100% {
			transform: translateX(-50%);
		}
	}

	/* ─────────────────────────────────────────────────────────────────
	   Hero
	   ───────────────────────────────────────────────────────────────── */
	.hero {
		position: relative;
		padding-block: 8rem;
		padding-inline: 1rem;
		max-width: var(--rtp-content-max);
		margin-inline: auto;
		text-align: center;
	}
	@media (min-width: 640px) {
		.hero {
			padding-inline: 1.5rem;
		}
	}
	@media (min-width: 1024px) {
		.hero {
			padding-inline: 2rem;
		}
	}

	.hero__badge {
		display: inline-flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.5rem 1rem;
		border-radius: var(--rtp-radius-pill);
		background: color-mix(in oklab, #fff 5%, transparent);
		border: 1px solid var(--rtp-border);
		backdrop-filter: blur(8px);
		-webkit-backdrop-filter: blur(8px);
		margin-block-end: 3rem;
		box-shadow: 0 10px 15px -3px var(--m-primary-mix-5);
		transition: border-color 300ms var(--rtp-ease-out);
		cursor: default;
	}
	.hero__badge:hover {
		border-color: color-mix(in oklab, var(--rtp-primary) 30%, transparent);
	}
	.hero__badge-dot {
		width: 0.5rem;
		height: 0.5rem;
		border-radius: 50%;
		background: var(--rtp-primary);
		animation: mission-pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
	}
	.hero__badge-label {
		font-size: 0.75rem;
		font-weight: 700;
		letter-spacing: 0.2em;
		text-transform: uppercase;
		color: var(--rtp-text-soft);
		transition: color var(--rtp-dur-fast) var(--rtp-ease-out);
	}
	.hero__badge:hover .hero__badge-label {
		color: #fff;
	}

	@keyframes mission-pulse {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.5;
		}
	}

	.hero__title {
		font-family: var(--rtp-font-display);
		letter-spacing: -0.03em;
		font-weight: 800;
		color: #fff;
		font-size: 3.75rem; /* 6xl */
		margin-block-end: 2.5rem;
		line-height: 0.95;
	}
	@media (min-width: 768px) {
		.hero__title {
			font-size: 6rem;
		}
	}
	@media (min-width: 1024px) {
		.hero__title {
			font-size: 8rem;
		}
	}
	.hero__title-accent {
		background: linear-gradient(
			to right,
			var(--rtp-primary),
			var(--m-blue-bright),
			color-mix(in oklab, var(--rtp-indigo), white 30%)
		);
		-webkit-background-clip: text;
		background-clip: text;
		color: transparent;
	}

	.hero__lede {
		max-width: 48rem;
		margin-inline: auto;
	}
	.hero__lede p {
		font-size: 1.25rem;
		color: var(--rtp-text-muted);
		line-height: 1.625;
		font-weight: 300;
	}
	@media (min-width: 768px) {
		.hero__lede p {
			font-size: 1.5rem;
		}
	}
	.hero__lede-emph {
		color: #fff;
		font-weight: 500;
		border-block-end: 1px solid color-mix(in oklab, var(--rtp-primary) 50%, transparent);
	}

	.hero__stats {
		margin-block-start: 4rem;
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 1rem;
		max-width: 56rem; /* max-w-4xl */
		margin-inline: auto;
	}
	@media (min-width: 768px) {
		.hero__stats {
			grid-template-columns: repeat(4, minmax(0, 1fr));
		}
	}

	.stat-card {
		background: color-mix(in oklab, #fff 2%, transparent);
		border: 1px solid var(--rtp-border);
		padding: 1.5rem;
		border-radius: 0.75rem;
		backdrop-filter: blur(4px);
		-webkit-backdrop-filter: blur(4px);
	}
	.stat-card__value {
		font-size: 1.875rem;
		font-weight: 700;
		color: #fff;
		font-family: var(--rtp-font-mono);
	}
	.stat-card__value--primary {
		color: var(--rtp-primary);
	}
	.stat-card__value--emerald {
		color: var(--m-emerald-bright);
	}
	.stat-card__label {
		font-size: 0.625rem;
		text-transform: uppercase;
		letter-spacing: 0.2em;
		color: var(--rtp-text-subtle);
		margin-block-start: 0.5rem;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Reality (90/90/90)
	   ───────────────────────────────────────────────────────────────── */
	.reality {
		padding-block: 8rem;
		border-block: 1px solid var(--rtp-border-soft);
		background: color-mix(in oklab, #fff 1%, transparent);
	}
	.reality__shell {
		max-width: var(--rtp-content-max);
		margin-inline: auto;
		padding-inline: 1rem;
	}
	@media (min-width: 640px) {
		.reality__shell {
			padding-inline: 1.5rem;
		}
	}
	@media (min-width: 1024px) {
		.reality__shell {
			padding-inline: 2rem;
		}
	}
	.reality__grid {
		display: grid;
		gap: 4rem;
		align-items: center;
	}
	@media (min-width: 1024px) {
		.reality__grid {
			grid-template-columns: repeat(12, minmax(0, 1fr));
		}
		.reality__copy {
			grid-column: span 5 / span 5;
		}
		.reality__table-wrap {
			grid-column: span 7 / span 7;
		}
	}

	.reality__chip {
		display: inline-flex;
		align-items: center;
		gap: 0.75rem;
		margin-block-end: 2rem;
	}
	.reality__chip-icon {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: 0.5rem;
		background: color-mix(in oklab, var(--rtp-red) 10%, transparent);
		color: var(--rtp-red);
		border: 1px solid color-mix(in oklab, var(--rtp-red) 20%, transparent);
		border-radius: var(--rtp-radius-md);
	}
	.reality__chip-label {
		font-weight: 700;
		letter-spacing: 0.2em;
		text-transform: uppercase;
		font-size: 0.875rem;
		color: var(--m-red-bright);
	}

	.reality__title {
		font-family: var(--rtp-font-display);
		letter-spacing: -0.03em;
		font-size: 2.25rem;
		font-weight: 700;
		color: #fff;
		margin-block-end: 2rem;
		line-height: 1.1;
	}
	@media (min-width: 768px) {
		.reality__title {
			font-size: 3rem;
		}
	}

	.reality__prose {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
		font-size: 1.125rem;
		color: var(--rtp-text-muted);
		line-height: 1.625;
	}

	.reality__callout {
		padding: 1.5rem;
		border: 1px solid color-mix(in oklab, var(--rtp-red) 20%, transparent);
		background: color-mix(in oklab, #7f1d1d 5%, transparent);
		border-radius: 0.75rem;
	}
	.reality__callout-title {
		color: var(--m-red-bright);
		font-weight: 700;
		margin-block-end: 0.5rem;
		font-size: 0.875rem;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}
	.reality__callout-body {
		font-size: 0.875rem;
		color: color-mix(in oklab, #fecaca 60%, transparent);
	}

	/* ─────────────────────────────────────────────────────────────────
	   Compare table
	   ───────────────────────────────────────────────────────────────── */
	.compare-table {
		position: relative;
		background: var(--rtp-bg);
		border: 1px solid var(--rtp-border);
		border-radius: 1rem;
		overflow: hidden;
		box-shadow:
			0 25px 50px -12px rgba(0, 0, 0, 0.4),
			0 20px 25px -5px rgba(0, 0, 0, 0.1);
	}
	.compare-table__rule {
		position: absolute;
		inset-block-start: 0;
		inset-inline-start: 0;
		width: 100%;
		height: 4px;
		background: linear-gradient(to right, var(--rtp-red), var(--rtp-emerald));
	}
	.compare-table__scroll {
		overflow-x: auto;
	}
	.compare-table table {
		width: 100%;
		text-align: start;
		border-collapse: collapse;
	}
	.compare-table thead tr {
		font-size: 0.75rem;
		text-transform: uppercase;
		letter-spacing: 0.2em;
		border-block-end: 1px solid var(--rtp-border);
		color: var(--rtp-text-subtle);
	}
	.compare-table__th {
		padding: 1.5rem;
		font-weight: 500;
	}
	.compare-table__th--label {
		background: color-mix(in oklab, #fff 2%, transparent);
	}
	.compare-table__th--amateur {
		color: var(--m-red-bright);
		background: color-mix(in oklab, #7f1d1d 10%, transparent);
		border-inline-start: 1px solid var(--rtp-border-soft);
	}
	.compare-table__th--pro {
		color: var(--m-emerald-bright);
		background: color-mix(in oklab, #064e3b 10%, transparent);
		border-inline-start: 1px solid var(--rtp-border-soft);
	}
	.compare-table tbody {
		font-size: 0.875rem;
		font-family: var(--rtp-font-mono);
	}
	.compare-table tbody tr + tr {
		border-block-start: 1px solid var(--rtp-border-soft);
	}
	.compare-table__td {
		padding: 1.25rem;
	}
	.compare-table__td--label {
		font-weight: 700;
		color: #fff;
	}
	.compare-table__td--amateur {
		color: var(--rtp-text-muted);
		border-inline-start: 1px solid var(--rtp-border-soft);
	}
	.compare-table__td--pro {
		color: #fff;
		border-inline-start: 1px solid var(--rtp-border-soft);
	}

	/* ─────────────────────────────────────────────────────────────────
	   Ecosystem (3 pillars)
	   ───────────────────────────────────────────────────────────────── */
	.ecosystem {
		padding-block: 8rem;
		position: relative;
		overflow: hidden;
	}
	.ecosystem__halo {
		position: absolute;
		inset-block-start: 50%;
		inset-inline-start: 50%;
		translate: -50% -50%;
		width: 1000px;
		height: 1000px;
		background: color-mix(in oklab, var(--rtp-indigo) 5%, transparent);
		filter: blur(150px);
		border-radius: 50%;
		pointer-events: none;
	}
	.ecosystem__shell {
		max-width: var(--rtp-content-max);
		margin-inline: auto;
		padding-inline: 1rem;
		position: relative;
		z-index: 10;
	}
	@media (min-width: 640px) {
		.ecosystem__shell {
			padding-inline: 1.5rem;
		}
	}
	@media (min-width: 1024px) {
		.ecosystem__shell {
			padding-inline: 2rem;
		}
	}
	.ecosystem__head {
		text-align: center;
		margin-block-end: 5rem;
	}
	.ecosystem__eyebrow-wrap {
		display: inline-block;
		margin-block-end: 1rem;
	}
	.ecosystem__eyebrow {
		color: var(--rtp-primary);
		font-weight: 700;
		letter-spacing: 0.2em;
		text-transform: uppercase;
		font-size: 0.875rem;
	}
	.ecosystem__title {
		font-family: var(--rtp-font-display);
		font-size: 2.25rem;
		font-weight: 700;
		color: #fff;
		margin-block-end: 1.5rem;
		letter-spacing: -0.03em;
	}
	@media (min-width: 768px) {
		.ecosystem__title {
			font-size: 3rem;
		}
	}
	.ecosystem__lede {
		color: var(--rtp-text-muted);
		max-width: 42rem;
		margin-inline: auto;
		font-size: 1.125rem;
	}

	.pillars {
		display: grid;
		gap: 2rem;
	}
	@media (min-width: 768px) {
		.pillars {
			grid-template-columns: repeat(3, minmax(0, 1fr));
		}
	}

	.pillar {
		--pillar-accent: var(--m-emerald-bright);
		position: relative;
		height: 100%;
	}
	.pillar--emerald {
		--pillar-accent: var(--m-emerald-bright);
	}
	.pillar--blue {
		--pillar-accent: var(--m-blue-bright);
	}
	.pillar--purple {
		--pillar-accent: var(--m-purple-bright);
	}
	.pillar__shadow {
		position: absolute;
		inset: 0;
		background: var(--m-bg-card-elev);
		border-radius: 1rem;
		transition: transform 300ms var(--rtp-ease-out);
	}
	.pillar:hover .pillar__shadow {
		transform: scale(1.02);
	}
	.pillar__card {
		position: relative;
		height: 100%;
		padding: 2rem;
		background: var(--m-bg-card);
		border: 1px solid var(--rtp-border);
		border-radius: 1rem;
		display: flex;
		flex-direction: column;
		transition: border-color 300ms var(--rtp-ease-out);
	}
	.pillar:hover .pillar__card {
		border-color: var(--rtp-border-strong);
	}
	.pillar__icon {
		width: 3.5rem;
		height: 3.5rem;
		border-radius: 0.75rem;
		display: flex;
		align-items: center;
		justify-content: center;
		margin-block-end: 2rem;
		background: color-mix(in oklab, var(--pillar-accent) 10%, transparent);
		color: var(--pillar-accent);
		border: 1px solid color-mix(in oklab, var(--pillar-accent) 20%, transparent);
	}
	.pillar__title {
		font-size: 1.5rem;
		font-weight: 700;
		color: #fff;
		margin-block-end: 1rem;
	}
	.pillar__desc {
		color: var(--rtp-text-muted);
		line-height: 1.625;
		flex-grow: 1;
	}
	.pillar__cta {
		margin-block-start: 2rem;
		padding-block-start: 1.5rem;
		border-block-start: 1px solid var(--rtp-border-soft);
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.875rem;
		font-weight: 700;
		color: color-mix(in oklab, #fff 60%, transparent);
		transition: color var(--rtp-dur-fast) var(--rtp-ease-out);
	}
	.pillar:hover .pillar__cta {
		color: #fff;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Note (founder letter)
	   ───────────────────────────────────────────────────────────────── */
	.note {
		padding-block: 8rem;
		background: linear-gradient(to bottom, var(--rtp-bg-deep), var(--rtp-bg));
		border-block: 1px solid var(--rtp-border-soft);
	}
	.note__shell {
		max-width: 48rem;
		margin-inline: auto;
		padding-inline: 1.5rem;
	}
	.note__sigil-wrap {
		display: flex;
		justify-content: center;
		margin-block-end: 3rem;
	}
	.note__sigil {
		width: 5rem;
		height: 5rem;
		border-radius: 50%;
		background: color-mix(in oklab, #fff 5%, transparent);
		border: 1px solid var(--rtp-border);
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 1.875rem;
		font-family: var(--rtp-font-display);
		font-weight: 700;
		color: var(--rtp-primary);
		box-shadow: 0 0 30px var(--m-primary-glow-soft);
	}
	.note__prose {
		font-weight: 300;
		font-size: 1.125rem;
		line-height: 1.7;
		color: var(--rtp-text-soft);
	}
	.note__prose > * + * {
		margin-block-start: 1.25rem;
	}
	.note__prose strong {
		color: #fff;
		font-weight: 600;
	}
	.note__title {
		text-align: center;
		font-family: var(--rtp-font-display);
		font-size: 1.875rem;
		color: #fff;
		margin-block-end: 2.5rem;
		letter-spacing: -0.03em;
		font-weight: 700;
	}
	.note__quote {
		border-inline-start: 4px solid var(--rtp-primary);
		padding-inline-start: 1.5rem;
		font-style: italic;
		color: #fff;
		font-size: 1.25rem;
		margin-block: 2rem;
	}
	.note__sig {
		margin-block-start: 3rem;
		padding-block-start: 2rem;
		border-block-start: 1px solid var(--rtp-border);
		display: flex;
		align-items: center;
		gap: 1rem;
	}
	.note__avatar {
		width: 3rem;
		height: 3rem;
		border-radius: 50%;
		background: color-mix(in oklab, #fff 10%, transparent);
		display: flex;
		align-items: center;
		justify-content: center;
		font-weight: 700;
		color: #fff;
		font-size: 1.125rem;
	}
	.note__sig-name {
		color: #fff;
		font-weight: 700;
		font-family: var(--rtp-font-display);
	}
	.note__sig-role {
		font-size: 0.875rem;
		color: var(--rtp-text-subtle);
	}

	/* ─────────────────────────────────────────────────────────────────
	   Lexicon (glossary)
	   ───────────────────────────────────────────────────────────────── */
	.lexicon {
		padding-block: 6rem;
		background: color-mix(in oklab, #fff 1%, transparent);
		border-block: 1px solid var(--rtp-border-soft);
	}
	.lexicon__shell {
		max-width: var(--rtp-content-max);
		margin-inline: auto;
		padding-inline: 1rem;
	}
	@media (min-width: 640px) {
		.lexicon__shell {
			padding-inline: 1.5rem;
		}
	}
	@media (min-width: 1024px) {
		.lexicon__shell {
			padding-inline: 2rem;
		}
	}
	.lexicon__head {
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		align-items: flex-end;
		margin-block-end: 3rem;
		gap: 1.5rem;
	}
	@media (min-width: 768px) {
		.lexicon__head {
			flex-direction: row;
		}
	}
	.lexicon__title {
		font-size: 1.875rem;
		font-family: var(--rtp-font-display);
		font-weight: 700;
		color: #fff;
		margin-block-end: 0.5rem;
		letter-spacing: -0.03em;
	}
	.lexicon__lede {
		color: var(--rtp-text-muted);
	}
	.lexicon__search {
		position: relative;
	}
	.lexicon__search-icon {
		position: absolute;
		inset-inline-start: 0.75rem;
		inset-block-start: 50%;
		translate: 0 -50%;
		color: var(--rtp-text-subtle);
		width: 1rem;
		height: 1rem;
	}
	.lexicon__input {
		background: var(--rtp-bg);
		border: 1px solid var(--rtp-border);
		border-radius: var(--rtp-radius-md);
		padding-inline: 2.5rem 1rem;
		padding-block: 0.5rem;
		font-size: 0.875rem;
		color: #fff;
		outline: none;
		width: 16rem;
		transition: border-color var(--rtp-dur-fast) var(--rtp-ease-out);
	}
	.lexicon__input:focus {
		border-color: var(--rtp-primary);
	}

	.lexicon__grid {
		display: grid;
		gap: 1.5rem;
	}
	@media (min-width: 768px) {
		.lexicon__grid {
			grid-template-columns: repeat(2, minmax(0, 1fr));
		}
	}
	@media (min-width: 1024px) {
		.lexicon__grid {
			grid-template-columns: repeat(3, minmax(0, 1fr));
		}
	}
	.lex-card {
		padding: 1.5rem;
		background: var(--rtp-bg);
		border: 1px solid var(--rtp-border);
		border-radius: 0.75rem;
		transition: border-color var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.lex-card:hover {
		border-color: color-mix(in oklab, #fff 30%, transparent);
	}
	.lex-card__term {
		color: var(--rtp-primary);
		font-weight: 700;
		margin-block-end: 0.5rem;
		font-family: var(--rtp-font-mono);
		text-transform: uppercase;
		letter-spacing: 0.025em;
		font-size: 0.875rem;
		transition: color var(--rtp-dur-fast) var(--rtp-ease-out);
	}
	.lex-card:hover .lex-card__term {
		color: #fff;
	}
	.lex-card__def {
		font-size: 0.875rem;
		color: var(--rtp-text-muted);
		line-height: 1.625;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Manifesto
	   ───────────────────────────────────────────────────────────────── */
	.manifesto {
		padding-block: 8rem;
		padding-inline: 1rem;
	}
	@media (min-width: 640px) {
		.manifesto {
			padding-inline: 1.5rem;
		}
	}
	@media (min-width: 1024px) {
		.manifesto {
			padding-inline: 2rem;
		}
	}
	.manifesto__shell {
		max-width: 72rem;
		margin-inline: auto;
	}
	.manifesto__card {
		position: relative;
		background: linear-gradient(to bottom right, var(--m-bg-card-elev), #020617);
		border: 1px solid var(--rtp-border);
		border-radius: 1.5rem;
		padding: 2rem;
		overflow: hidden;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.4);
	}
	@media (min-width: 768px) {
		.manifesto__card {
			padding: 4rem;
		}
	}
	.manifesto__quote-bg {
		position: absolute;
		inset-block-start: -2rem;
		inset-inline-end: -2rem;
		color: color-mix(in oklab, #fff 5%, transparent);
		rotate: 12deg;
	}
	.manifesto__inner {
		position: relative;
		z-index: 10;
	}
	.manifesto__head {
		text-align: center;
		margin-block-end: 3rem;
	}
	.manifesto__title {
		font-family: var(--rtp-font-display);
		font-size: 1.875rem;
		font-weight: 700;
		color: #fff;
		margin-block-end: 1rem;
		letter-spacing: -0.03em;
	}
	@media (min-width: 768px) {
		.manifesto__title {
			font-size: 2.25rem;
		}
	}
	.manifesto__rule {
		height: 0.25rem;
		width: 5rem;
		background: var(--rtp-primary);
		margin-inline: auto;
		border-radius: var(--rtp-radius-pill);
	}
	.manifesto__grid {
		display: grid;
		gap: 2rem 4rem;
	}
	@media (min-width: 768px) {
		.manifesto__grid {
			grid-template-columns: repeat(2, minmax(0, 1fr));
		}
	}
	.manifesto__item {
		display: flex;
		align-items: flex-start;
		gap: 1rem;
	}
	.manifesto__check {
		margin-block-start: 0.25rem;
		color: var(--rtp-emerald);
		background: color-mix(in oklab, var(--rtp-emerald) 10%, transparent);
		padding: 0.25rem;
		border-radius: 0.375rem;
		transition:
			background var(--rtp-dur-fast) var(--rtp-ease-out),
			color var(--rtp-dur-fast) var(--rtp-ease-out);
	}
	.manifesto__item:hover .manifesto__check {
		background: var(--rtp-emerald);
		color: #fff;
	}
	.manifesto__text {
		color: var(--rtp-text-soft);
		font-weight: 500;
		font-size: 1.125rem;
		transition: color var(--rtp-dur-fast) var(--rtp-ease-out);
	}
	.manifesto__item:hover .manifesto__text {
		color: #fff;
	}
	.manifesto__footer {
		margin-block-start: 4rem;
		text-align: center;
	}
	.manifesto__footer p {
		color: var(--rtp-text-subtle);
		font-size: 0.875rem;
		font-family: var(--rtp-font-mono);
		text-transform: uppercase;
		letter-spacing: 0.2em;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Final CTA
	   ───────────────────────────────────────────────────────────────── */
	.cta-final {
		padding-block: 8rem;
		text-align: center;
		position: relative;
		overflow: hidden;
	}
	.cta-final__halo {
		position: absolute;
		inset-block-start: 0;
		inset-inline-start: 50%;
		translate: -50% 0;
		width: 100%;
		height: 100%;
		background: linear-gradient(to bottom, var(--m-primary-mix-5), transparent);
		pointer-events: none;
	}
	.cta-final__shell {
		max-width: 56rem;
		margin-inline: auto;
		padding-inline: 1rem;
		position: relative;
		z-index: 10;
	}
	.cta-final__pulse {
		display: inline-block;
		padding: 1rem;
		border-radius: 50%;
		background: color-mix(in oklab, #fff 5%, transparent);
		border: 1px solid var(--rtp-border);
		margin-block-end: 2rem;
		color: var(--rtp-primary);
		animation: cta-bounce 1s infinite;
	}
	@keyframes cta-bounce {
		0%,
		100% {
			translate: 0 -25%;
			animation-timing-function: cubic-bezier(0.8, 0, 1, 1);
		}
		50% {
			translate: 0 0;
			animation-timing-function: cubic-bezier(0, 0, 0.2, 1);
		}
	}

	.cta-final__title {
		font-family: var(--rtp-font-display);
		font-size: 3rem;
		font-weight: 800;
		color: #fff;
		margin-block-end: 2rem;
		letter-spacing: -0.025em;
	}
	@media (min-width: 768px) {
		.cta-final__title {
			font-size: 3.75rem;
		}
	}
	.cta-final__accent {
		color: var(--rtp-primary);
	}

	.cta-final__lede {
		font-size: 1.25rem;
		color: var(--rtp-text-muted);
		margin-block-end: 3rem;
		max-width: 42rem;
		margin-inline: auto;
	}

	.cta-final__actions {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 1.5rem;
	}
	@media (min-width: 640px) {
		.cta-final__actions {
			flex-direction: row;
		}
	}

	.btn {
		display: inline-flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1.25rem 2.5rem;
		border-radius: 0.75rem;
		font-weight: 700;
		font-size: 1.125rem;
		text-decoration: none;
		transition:
			background var(--rtp-dur-base) var(--rtp-ease-out),
			translate var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.btn--primary {
		background: var(--rtp-primary);
		color: #fff;
		box-shadow: 0 0 30px var(--m-primary-glow);
	}
	.btn--primary:hover {
		background: #2563eb;
		translate: 0 -0.25rem;
	}
	.btn__arrow {
		display: inline-flex;
		transition: translate var(--rtp-dur-fast) var(--rtp-ease-out);
	}
	.btn--primary:hover .btn__arrow {
		translate: 0.25rem 0;
	}
	.btn--ghost {
		color: var(--rtp-text-soft);
		border: 1px solid var(--rtp-border);
		background: transparent;
	}
	.btn--ghost:hover {
		background: color-mix(in oklab, #fff 5%, transparent);
	}

	.cta-final__fine-print {
		margin-block-start: 2rem;
		font-size: 0.875rem;
		color: var(--rtp-text-faint);
		font-family: var(--rtp-font-mono);
	}

	/* ─────────────────────────────────────────────────────────────────
	   Scrollbar (page-level customization — :global because document scroll)
	   ───────────────────────────────────────────────────────────────── */
	:global(::-webkit-scrollbar) {
		width: 8px;
	}
	:global(::-webkit-scrollbar-track) {
		background: var(--rtp-bg-darker);
	}
	:global(::-webkit-scrollbar-thumb) {
		background: #1e293b;
		border-radius: 4px;
	}
	:global(::-webkit-scrollbar-thumb:hover) {
		background: #334155;
	}
</style>

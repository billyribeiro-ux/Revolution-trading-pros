<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { slide } from 'svelte/transition';
	import IconLock from '@tabler/icons-svelte-runes/icons/lock';
	import IconGlobe from '@tabler/icons-svelte-runes/icons/globe';
	import IconShield from '@tabler/icons-svelte-runes/icons/shield';
	import IconChevronDown from '@tabler/icons-svelte-runes/icons/chevron-down';
	import IconActivity from '@tabler/icons-svelte-runes/icons/activity';
	import IconTerminal from '@tabler/icons-svelte-runes/icons/terminal';
	import IconBrain from '@tabler/icons-svelte-runes/icons/brain';
	import IconCheck from '@tabler/icons-svelte-runes/icons/check';
	import IconFileText from '@tabler/icons-svelte-runes/icons/file-text';

	let openAccordion = $state<number | null>(0);
	const toggleAccordion = (idx: number) => (openAccordion = openAccordion === idx ? null : idx);
	const accordionChevronClass = (idx: number) =>
		openAccordion === idx ? 'accordion__chevron accordion__chevron--open' : 'accordion__chevron';

	onMount(async () => {
		if (!browser) return;
		const { gsap } = await import('gsap');

		const tl = gsap.timeline({ defaults: { ease: 'power4.out' } });
		const badge = document.querySelector<HTMLElement>('[data-gsap-hero="badge"]');
		const title = document.querySelector<HTMLElement>('[data-gsap-hero="title"]');
		const desc = document.querySelector<HTMLElement>('[data-gsap-hero="desc"]');
		const metrics = document.querySelector<HTMLElement>('[data-gsap-hero="metrics"]');
		const graphic = document.querySelector<HTMLElement>('[data-gsap-hero="graphic"]');

		if (badge)
			tl.fromTo(badge, { y: 20, opacity: 0 }, { y: 0, opacity: 1, duration: 1, delay: 0.2 });
		if (title)
			tl.fromTo(
				title,
				{ y: 40, opacity: 0 },
				{ y: 0, opacity: 1, duration: 1.2, stagger: 0.1 },
				'-=0.8'
			);
		if (desc) tl.fromTo(desc, { y: 20, opacity: 0 }, { y: 0, opacity: 1, duration: 1 }, '-=0.8');
		if (metrics)
			tl.fromTo(
				metrics,
				{ opacity: 0, scale: 0.98 },
				{ opacity: 1, scale: 1, duration: 1.2 },
				'-=0.6'
			);
		if (graphic)
			tl.fromTo(graphic, { x: 40, opacity: 0 }, { x: 0, opacity: 1, duration: 1.5 }, '-=1.0');

		const observer = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) {
						const target = entry.target as HTMLElement;
						const children = target.querySelectorAll('.gsap-reveal-item');
						if (children.length > 0) {
							gsap.fromTo(
								children,
								{ y: 30, opacity: 0 },
								{
									y: 0,
									opacity: 1,
									duration: 0.8,
									stagger: 0.1,
									ease: 'power2.out',
									overwrite: true
								}
							);
						} else {
							gsap.fromTo(
								target,
								{ y: 30, opacity: 0 },
								{ y: 0, opacity: 1, duration: 0.8, ease: 'power2.out', overwrite: true }
							);
						}
						observer.unobserve(target);
					}
				});
			},
			{ threshold: 0.15 }
		);
		document.querySelectorAll('.gsap-section').forEach((el) => observer.observe(el));
	});

	const sessionBreakdown = [
		{
			time: '00:00 - 00:30',
			phase: 'Phase I: Microstructure Audit',
			desc: "Forensic analysis of your trade logs (last 1,000 executions). We measure your slippage against ADV, analyze your fill quality across venues, and identify 'Alpha Decay' where execution drag is eroding edge."
		},
		{
			time: '00:30 - 01:00',
			phase: 'Phase II: Risk Parameterization',
			desc: 'Stress-testing your current risk model against 6-sigma events. We reconstruct your position sizing logic using Kelly Criterion modified for fat-tail distribution to optimize Geometric Mean Return.'
		},
		{
			time: '01:00 - 01:45',
			phase: 'Phase III: Edge Calibration',
			desc: "Strategic realignment. We overlay your discretionary edge with institutional data sets (Dark Pool prints, GEX levels, Vanna Flows) to create a 'Confluence Filter' that filters out B-grade setups."
		},
		{
			time: '01:45 - 02:00',
			phase: 'Phase IV: Neural Mapping',
			desc: "Addressing the 'Psychological Ceiling.' We identify the specific cognitive biases (Loss Aversion, Recency Bias) preventing you from scaling size and implement a 'Circuit Breaker' protocol."
		}
	];
</script>

<div class="mentorship">
	<div class="mentorship__grid-overlay" aria-hidden="true"></div>

	<!-- ICT11+ Fix: Changed from <main> to <div> - root layout provides <main> -->
	<div class="mentorship__content">
		<nav class="m-topbar" aria-label="Institutional context">
			<div class="m-topbar__container">
				<div class="m-topbar__brand">
					Revolution <span class="m-topbar__slash">//</span> Institutional
				</div>
				<div class="m-topbar__meta">
					<div class="m-topbar__status">
						<span class="m-topbar__pulse" aria-hidden="true"></span>
						<span class="m-topbar__status-label">Secure Uplink</span>
					</div>
					<span class="m-topbar__divider" aria-hidden="true"></span>
					<span class="m-topbar__client">Client ID: Guest</span>
				</div>
			</div>
		</nav>

		<section class="hero">
			<div class="hero__grid">
				<div class="hero__copy">
					<div data-gsap-hero="badge" class="hero__badge">
						<IconLock size={12} />
						Restricted Access
					</div>

					<h1 data-gsap-hero="title" class="hero__title">
						Strategic<br />
						<span class="hero__title-mute">Alpha</span> Audit.
					</h1>

					<div data-gsap-hero="desc" class="hero__lede">
						<p>
							A high-velocity, forensic deconstruction of your trading business. Designed strictly
							for <span class="hero__lede-emph">Portfolio Managers</span> and
							<span class="hero__lede-emph">Proprietary Traders</span> deploying 7-8 figure capital.
						</p>
					</div>

					<div data-gsap-hero="metrics" class="hero__metrics">
						<div class="hero__metric">
							<div class="hero__metric-label">Consultation Fee</div>
							<div class="hero__metric-value">
								$25,000 <span class="hero__metric-unit">USD</span>
							</div>
						</div>
						<div class="hero__metric">
							<div class="hero__metric-label">Duration</div>
							<div class="hero__metric-value">
								120 <span class="hero__metric-unit">MINUTES</span>
							</div>
						</div>
						<div class="hero__metric">
							<div class="hero__metric-label">Availability</div>
							<div class="hero__metric-value">
								Q4: <span class="hero__metric-accent">2 SLOTS</span>
							</div>
						</div>
					</div>
				</div>

				<div data-gsap-hero="graphic" class="hero__scope">
					<div class="scope-card">
						<div class="scope-card__icon" aria-hidden="true">
							<IconGlobe size={48} stroke={1.2} />
						</div>
						<h3 class="scope-card__heading">Scope of Engagement</h3>
						<ul class="scope-card__list">
							<li class="scope-card__row">
								<span>&gt; Execution Audit</span>
								<div class="scope-card__status">
									<span class="scope-card__dot" aria-hidden="true"></span>
									<span class="scope-card__status-label">Active</span>
								</div>
							</li>
							<li class="scope-card__row">
								<span>&gt; Risk Parameterization</span>
								<div class="scope-card__status">
									<span class="scope-card__dot" aria-hidden="true"></span>
									<span class="scope-card__status-label">Active</span>
								</div>
							</li>
							<li class="scope-card__row">
								<span>&gt; Psychological Mapping</span>
								<div class="scope-card__status">
									<span class="scope-card__dot" aria-hidden="true"></span>
									<span class="scope-card__status-label">Active</span>
								</div>
							</li>
							<li class="scope-card__row">
								<span>&gt; Infrastructure Review</span>
								<div class="scope-card__status">
									<span class="scope-card__dot" aria-hidden="true"></span>
									<span class="scope-card__status-label">Active</span>
								</div>
							</li>
						</ul>
						<div class="scope-card__footer">
							<p>
								<span class="scope-card__warning">WARNING:</span> This service is not educational. It
								is advisory. We assume the client possesses sophisticated knowledge of derivatives, margin
								mechanics, and market microstructure.
							</p>
						</div>
					</div>
				</div>
			</div>
		</section>

		<section class="ceiling gsap-section">
			<div class="ceiling__inner">
				<div class="ceiling__grid">
					<div class="ceiling__copy gsap-reveal-item">
						<h2 class="section-title">The Liquidity Ceiling.</h2>
						<div class="ceiling__body">
							<p>
								Scaling a portfolio from $100,000 to $1,000,000 is a math problem. Scaling from
								$10,000,000 to $100,000,000 is a liquidity problem.
							</p>
							<p>
								At institutional size, your entry <em>is</em> the market. Alpha decays the moment you
								execute. Without optimizing your participation rate, dark pool routing, and variance drag,
								you are simply paying a "Size Tax" to HFT firms.
							</p>
							<p class="ceiling__pullquote">
								Revolution Trading Pros intervenes at this inflection point. We do not teach you <em
									>how</em
								> to trade. We engineer your business to scale without breaking.
							</p>
						</div>
					</div>

					<div class="ceiling__metrics-grid">
						<div class="metric-tile gsap-reveal-item">
							<div class="metric-tile__icon"><IconActivity size={32} stroke={1.2} /></div>
							<div>
								<div class="metric-tile__value">
									-18<span class="metric-tile__unit">%</span>
								</div>
								<div class="metric-tile__label">Avg. Execution Drag</div>
							</div>
						</div>
						<div class="metric-tile gsap-reveal-item">
							<div class="metric-tile__icon"><IconBrain size={32} stroke={1.2} /></div>
							<div>
								<div class="metric-tile__value">Bias</div>
								<div class="metric-tile__label">Primary Bottleneck</div>
							</div>
						</div>
						<div class="metric-tile gsap-reveal-item">
							<div class="metric-tile__icon"><IconTerminal size={32} stroke={1.2} /></div>
							<div>
								<div class="metric-tile__value">Zero</div>
								<div class="metric-tile__label">Latency Tolerance</div>
							</div>
						</div>
						<div class="metric-tile gsap-reveal-item">
							<div class="metric-tile__icon"><IconShield size={32} stroke={1.2} /></div>
							<div>
								<div class="metric-tile__value">MNDA</div>
								<div class="metric-tile__label">Legal Protection</div>
							</div>
						</div>
					</div>
				</div>
			</div>
		</section>

		<section class="protocol gsap-section">
			<div class="protocol__inner">
				<div class="protocol__header gsap-reveal-item">
					<div>
						<h2 class="section-title section-title--small">The 120-Minute Protocol</h2>
						<p class="protocol__subhead">Session Agenda // Confidential</p>
					</div>
					<div class="protocol__regions">
						<div class="protocol__regions-primary">NYC / LON / SIN</div>
						<div class="protocol__regions-caption">Global Availability</div>
					</div>
				</div>

				<div class="protocol__list">
					{#each sessionBreakdown as item (item.time)}
						<div class="protocol__item gsap-reveal-item">
							<div class="protocol__row">
								<div class="protocol__time">
									<span>{item.time}</span>
								</div>
								<div>
									<h3 class="protocol__phase">{item.phase}</h3>
									<p class="protocol__desc">{item.desc}</p>
								</div>
							</div>
						</div>
					{/each}
				</div>
			</div>
		</section>

		<section class="engagement gsap-section">
			<div class="engagement__grid">
				<div class="engagement__col gsap-reveal-item">
					<div class="engagement__heading-row">
						<div class="engagement__heading-icon">
							<IconFileText size={24} stroke={1.2} />
						</div>
						<h3 class="engagement__heading">Technical Prerequisites</h3>
					</div>
					<div class="prereq">
						<p class="prereq__intro">
							To ensure maximum utility of the 120-minute window, we require the following data
							points to be uploaded to our encrypted portal 48 hours prior to the session.
						</p>
						<ul class="prereq__list">
							<li class="prereq__item">
								<span class="prereq__bullet">&gt;&gt;</span>
								Past 12 Months of Brokerage Statements (Redacted PII)
							</li>
							<li class="prereq__item">
								<span class="prereq__bullet">&gt;&gt;</span>
								Current Risk Management Policy Document
							</li>
							<li class="prereq__item">
								<span class="prereq__bullet">&gt;&gt;</span>
								List of Execution Venues / Prime Brokerage Relationships
							</li>
							<li class="prereq__item">
								<span class="prereq__bullet">&gt;&gt;</span>
								Signed Mutual Non-Disclosure Agreement (MNDA)
							</li>
						</ul>
					</div>
				</div>

				<div class="engagement__col gsap-reveal-item">
					<div class="engagement__heading-row">
						<div class="engagement__heading-icon">
							<IconShield size={24} stroke={1.2} />
						</div>
						<h3 class="engagement__heading">Engagement Protocols</h3>
					</div>

					<div class="accordion">
						<div class="accordion__item">
							<button
								class="accordion__trigger"
								aria-expanded={openAccordion === 0}
								onclick={() => toggleAccordion(0)}
							>
								<span class="accordion__label">CONFIDENTIALITY &amp; DATA SOVEREIGNTY</span>
								<IconChevronDown size={16} class={accordionChevronClass(0)} />
							</button>
							{#if openAccordion === 0}
								<div transition:slide class="accordion__panel">
									Revolution Trading Pros operates under a strict code of silence. We execute a
									Mutual Non-Disclosure Agreement (MNDA) before receiving any trade logs. We do not
									store client data on cloud servers; all analysis is performed on air-gapped local
									machines and wiped post-session.
								</div>
							{/if}
						</div>

						<div class="accordion__item">
							<button
								class="accordion__trigger"
								aria-expanded={openAccordion === 1}
								onclick={() => toggleAccordion(1)}
							>
								<span class="accordion__label">CONFLICT OF INTEREST</span>
								<IconChevronDown size={16} class={accordionChevronClass(1)} />
							</button>
							{#if openAccordion === 1}
								<div transition:slide class="accordion__panel">
									We maintain a rigorous "Chinese Wall." We do not front-run client order flow, nor
									do we take opposing positions in instruments discussed during the consultation.
									Our role is strictly advisory; we do not take custody of funds or execute trades
									on your behalf.
								</div>
							{/if}
						</div>

						<div class="accordion__item">
							<button
								class="accordion__trigger"
								aria-expanded={openAccordion === 2}
								onclick={() => toggleAccordion(2)}
							>
								<span class="accordion__label">LOGISTICS &amp; EXPENSING</span>
								<IconChevronDown size={16} class={accordionChevronClass(2)} />
							</button>
							{#if openAccordion === 2}
								<div transition:slide class="accordion__panel">
									Sessions are conducted via encrypted video link (Zoom Enterprise/Teams) or
									in-person in NYC (subject to travel retainer). We provide itemized corporate
									invoicing suitable for fund administration expenses under "Professional
									Consultation."
								</div>
							{/if}
						</div>
					</div>
				</div>
			</div>
		</section>

		<section class="cta gsap-section">
			<div class="cta__vline" aria-hidden="true"></div>
			<div class="cta__hline" aria-hidden="true"></div>

			<div class="cta__shell gsap-reveal-item">
				<div class="cta__inner">
					<div class="cta__halo" aria-hidden="true"></div>

					<div class="cta__head">
						<div class="cta__icon"><IconCheck size={48} stroke={1.5} /></div>
						<h2 class="cta__title">Initiate Application</h2>
						<p class="cta__ref">Reference: Q4-INST-AUDIT</p>
					</div>

					<div class="cta__terms">
						<div class="cta__term">
							<div class="cta__term-label">Wire Amount</div>
							<div class="cta__term-value">$25,000.00</div>
						</div>
						<div class="cta__term">
							<div class="cta__term-label">Payment Terms</div>
							<div class="cta__term-value">Net 0</div>
						</div>
					</div>

					<a
						href="mailto:institutional@revolutiontradingpros.com?subject=Institutional%20Audit%20Application"
						class="cta__button"
					>
						Submit Request
					</a>

					<p class="cta__fine-print">
						Due to high demand, applications are reviewed weekly. <br class="cta__br" /> We reserve the
						right to decline engagements based on fit.
					</p>
				</div>
			</div>
		</section>
	</div>
</div>

<style>
	/* ─────────────────────────────────────────────────────────────────
	   Page-local tokens (one-offs for this page only).
	   Reusable color/spacing values come from --rtp-* (marketing.css).
	   ───────────────────────────────────────────────────────────────── */
	.mentorship {
		/* Amber metallic palette — used heavily on this page and on /about,
		   but each page authors its own shade scale because the about palette
		   leans richer and this page leans more reserved. */
		--m-amber-500: #f59e0b;
		--m-amber-600: #d97706;
		--m-amber-700: #b45309;
		--m-amber-warn: color-mix(in oklab, var(--m-amber-600) 30%, transparent);
		--m-amber-pill-bg: color-mix(in oklab, #78350f 10%, transparent);
		--m-amber-pill-border: color-mix(in oklab, #78350f 30%, transparent);
		--m-amber-cta-halo: color-mix(in oklab, #78350f 10%, transparent);
		--m-amber-border-divider: color-mix(in oklab, var(--m-amber-700) 50%, transparent);

		background: var(--rtp-bg-darker);
		color: var(--rtp-text-muted);
		font-family: var(--rtp-font-sans);
		position: relative;
		overflow-x: clip;
	}
	.mentorship::selection {
		background: #fff;
		color: #000;
	}

	.mentorship__grid-overlay {
		position: fixed;
		inset: 0;
		z-index: 0;
		pointer-events: none;
		background-image:
			linear-gradient(rgba(255, 255, 255, 0.03) 1px, transparent 1px),
			linear-gradient(90deg, rgba(255, 255, 255, 0.03) 1px, transparent 1px);
		background-size: 60px 60px;
	}

	.mentorship__content {
		position: relative;
		z-index: 10;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Top context bar — fixed strip above the navbar's hero area
	   ───────────────────────────────────────────────────────────────── */
	.m-topbar {
		position: fixed;
		inset-block-start: 0;
		inset-inline: 0;
		z-index: 50;
		background: color-mix(in oklab, var(--rtp-bg-darker) 80%, transparent);
		border-block-end: 1px solid var(--rtp-border);
		backdrop-filter: blur(4px);
		-webkit-backdrop-filter: blur(4px);
	}
	.m-topbar__container {
		max-width: var(--rtp-content-wide);
		margin-inline: auto;
		padding-inline: 1.5rem;
		height: 4rem;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}
	.m-topbar__brand {
		font-size: 0.75rem;
		font-family: var(--rtp-font-mono);
		text-transform: uppercase;
		letter-spacing: 0.2em;
		color: #fff;
		transition: color var(--rtp-dur-fast) var(--rtp-ease-out);
		cursor: default;
	}
	.m-topbar__brand:hover {
		color: var(--m-amber-500);
	}
	.m-topbar__slash {
		color: var(--rtp-text-faint);
	}
	.m-topbar__meta {
		display: flex;
		align-items: center;
		gap: 1.5rem;
	}
	.m-topbar__status {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}
	.m-topbar__pulse {
		width: 0.375rem;
		height: 0.375rem;
		border-radius: 50%;
		background: var(--rtp-emerald);
		animation: m-pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
	}
	.m-topbar__status-label,
	.m-topbar__client {
		font-size: 0.625rem;
		font-family: var(--rtp-font-mono);
		text-transform: uppercase;
		letter-spacing: 0.2em;
	}
	.m-topbar__status-label {
		color: var(--rtp-text-muted);
	}
	.m-topbar__client {
		color: var(--rtp-text-subtle);
	}
	.m-topbar__divider {
		width: 1px;
		height: 1rem;
		background: var(--rtp-border);
	}

	@keyframes m-pulse {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.5;
		}
	}

	/* ─────────────────────────────────────────────────────────────────
	   Section primitives — shared title / inner wrappers
	   ───────────────────────────────────────────────────────────────── */
	.section-title {
		font-family: var(--rtp-font-serif);
		font-size: clamp(2rem, 4vw, 2.5rem);
		color: var(--rtp-text);
		margin-block-end: 2rem;
	}
	.section-title--small {
		margin-block-end: 0.5rem;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Hero
	   ───────────────────────────────────────────────────────────────── */
	.hero {
		padding-block: 12rem 8rem;
		padding-inline: 1.5rem;
		border-block-end: 1px solid var(--rtp-border);
	}
	.hero__grid {
		max-width: var(--rtp-content-wide);
		margin-inline: auto;
		display: grid;
		gap: 4rem;
	}
	@media (min-width: 1024px) {
		.hero__grid {
			grid-template-columns: repeat(12, minmax(0, 1fr));
		}
		.hero__copy {
			grid-column: span 8 / span 8;
		}
		.hero__scope {
			grid-column: span 4 / span 4;
			display: flex;
			flex-direction: column;
			justify-content: flex-end;
		}
	}

	.hero__badge {
		display: inline-flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.25rem 0.75rem;
		border: 1px solid var(--m-amber-pill-border);
		background: var(--m-amber-pill-bg);
		color: var(--m-amber-500);
		font-size: 0.625rem;
		font-weight: 700;
		letter-spacing: 0.3em;
		text-transform: uppercase;
		margin-block-end: 3rem;
	}

	.hero__title {
		font-family: var(--rtp-font-serif);
		font-size: 3.75rem; /* 6xl */
		color: var(--rtp-text);
		margin-block-end: 3rem;
		letter-spacing: -0.025em;
		line-height: 0.9;
		transform-origin: left;
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
	.hero__title-mute {
		color: var(--rtp-text-faint);
	}

	.hero__lede {
		max-width: 42rem;
		border-inline-start: 2px solid var(--m-amber-700);
		padding-inline-start: 2rem;
		padding-block: 0.5rem;
	}
	.hero__lede p {
		font-size: 1.25rem;
		color: var(--rtp-text-soft);
		font-weight: 300;
		line-height: 1.625;
	}
	@media (min-width: 768px) {
		.hero__lede p {
			font-size: 1.5rem;
		}
	}
	.hero__lede-emph {
		color: var(--rtp-text);
		font-weight: 500;
	}

	.hero__metrics {
		margin-block-start: 4rem;
		display: flex;
		flex-wrap: wrap;
		gap: 3rem;
	}
	.hero__metric-label {
		font-size: 0.625rem;
		font-family: var(--rtp-font-mono);
		text-transform: uppercase;
		letter-spacing: 0.2em;
		color: var(--rtp-text-faint);
		margin-block-end: 0.5rem;
	}
	.hero__metric-value {
		font-size: 1.875rem;
		font-family: var(--rtp-font-serif);
		color: var(--rtp-text);
	}
	.hero__metric-unit {
		font-size: 0.875rem;
		color: var(--rtp-text-faint);
		font-family: var(--rtp-font-sans);
		vertical-align: middle;
	}
	.hero__metric-accent {
		color: var(--m-amber-500);
	}

	/* ─────────────────────────────────────────────────────────────────
	   Scope card (hero right-side panel)
	   ───────────────────────────────────────────────────────────────── */
	.scope-card {
		background: var(--rtp-bg-deeper);
		border: 1px solid var(--rtp-border);
		padding: 2rem;
		position: relative;
		overflow: hidden;
	}
	.scope-card__icon {
		position: absolute;
		inset-block-start: 0;
		inset-inline-end: 0;
		padding: 1rem;
		opacity: 0.2;
		width: 3rem;
		height: 3rem;
		color: var(--rtp-text-subtle);
	}
	.scope-card__heading {
		font-family: var(--rtp-font-mono);
		font-size: 0.75rem;
		text-transform: uppercase;
		letter-spacing: 0.2em;
		color: var(--rtp-text);
		margin-block-end: 1.5rem;
		padding-block-end: 1rem;
		border-block-end: 1px solid var(--rtp-border);
	}
	.scope-card__list {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		flex-direction: column;
		gap: 1rem;
		font-family: var(--rtp-font-mono);
		font-size: 0.875rem;
		color: var(--rtp-text-muted);
	}
	.scope-card__row {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}
	.scope-card__status {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}
	.scope-card__dot {
		width: 0.375rem;
		height: 0.375rem;
		background: var(--rtp-emerald);
		border-radius: 50%;
	}
	.scope-card__status-label {
		color: var(--rtp-emerald);
		font-size: 0.625rem;
		text-transform: uppercase;
	}
	.scope-card__footer {
		margin-block-start: 2rem;
		padding-block-start: 1.5rem;
		border-block-start: 1px solid var(--rtp-border);
	}
	.scope-card__footer p {
		font-size: 0.75rem;
		color: var(--rtp-text-faint);
		line-height: 1.625;
	}
	.scope-card__warning {
		color: var(--m-amber-600);
		font-weight: 700;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Ceiling section (the liquidity ceiling)
	   ───────────────────────────────────────────────────────────────── */
	.ceiling {
		padding-block: 8rem;
		padding-inline: 1.5rem;
		background: var(--rtp-bg-deep);
		border-block-end: 1px solid var(--rtp-border-soft);
	}
	.ceiling__inner {
		max-width: var(--rtp-content-wide);
		margin-inline: auto;
	}
	.ceiling__grid {
		display: grid;
		gap: 6rem;
	}
	@media (min-width: 1024px) {
		.ceiling__grid {
			grid-template-columns: 1fr 1fr;
		}
	}
	.ceiling__body {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
		font-size: 1.125rem;
		font-weight: 300;
		line-height: 1.625;
		color: var(--rtp-text-muted);
	}
	.ceiling__pullquote {
		color: var(--rtp-text);
		border-inline-start: 2px solid var(--rtp-border-strong);
		padding-inline-start: 1.5rem;
		padding-block: 0.5rem;
	}

	.ceiling__metrics-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1px;
		background: var(--rtp-border);
		border: 1px solid var(--rtp-border);
	}

	.metric-tile {
		background: var(--rtp-bg-deeper);
		padding: 2.5rem;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		height: 16rem;
	}
	.metric-tile__icon {
		color: var(--m-amber-600);
	}
	.metric-tile__value {
		font-size: 1.875rem;
		font-family: var(--rtp-font-serif);
		color: var(--rtp-text);
		margin-block-end: 0.5rem;
	}
	.metric-tile__unit {
		font-size: 1.125rem;
	}
	.metric-tile__label {
		font-size: 0.625rem;
		font-family: var(--rtp-font-mono);
		text-transform: uppercase;
		letter-spacing: 0.2em;
		color: var(--rtp-text-subtle);
	}

	/* ─────────────────────────────────────────────────────────────────
	   Protocol (120-minute agenda)
	   ───────────────────────────────────────────────────────────────── */
	.protocol {
		padding-block: 8rem;
		padding-inline: 1.5rem;
		border-block-end: 1px solid var(--rtp-border-soft);
	}
	.protocol__inner {
		max-width: 64rem; /* max-w-5xl */
		margin-inline: auto;
	}
	.protocol__header {
		display: flex;
		align-items: flex-end;
		justify-content: space-between;
		margin-block-end: 5rem;
		border-block-end: 1px solid var(--rtp-border);
		padding-block-end: 2rem;
	}
	.protocol__subhead {
		font-size: 0.875rem;
		font-family: var(--rtp-font-mono);
		color: var(--rtp-text-subtle);
		text-transform: uppercase;
		letter-spacing: 0.2em;
	}
	.protocol__regions {
		display: none;
		text-align: end;
	}
	@media (min-width: 768px) {
		.protocol__regions {
			display: block;
		}
	}
	.protocol__regions-primary {
		color: var(--m-amber-600);
		font-family: var(--rtp-font-mono);
		font-size: 0.875rem;
	}
	.protocol__regions-caption {
		font-size: 0.75rem;
		color: var(--rtp-text-faint);
		font-family: var(--rtp-font-mono);
		text-transform: uppercase;
	}

	.protocol__list {
		display: flex;
		flex-direction: column;
		gap: 2rem;
	}
	.protocol__row {
		display: flex;
		flex-direction: column;
		gap: 2rem;
		padding: 2rem;
		border-inline-start: 2px solid var(--rtp-border);
		transition:
			background var(--rtp-dur-fast) var(--rtp-ease-out),
			border-color var(--rtp-dur-fast) var(--rtp-ease-out);
	}
	.protocol__row:hover {
		background: color-mix(in oklab, #fff 2%, transparent);
		border-inline-start-color: var(--m-amber-600);
	}
	@media (min-width: 768px) {
		.protocol__row {
			flex-direction: row;
			gap: 4rem;
		}
	}
	.protocol__time {
		width: 8rem;
		flex-shrink: 0;
		padding-block-start: 0.25rem;
	}
	.protocol__time span {
		font-family: var(--rtp-font-mono);
		color: var(--m-amber-600);
		font-size: 0.875rem;
	}
	.protocol__phase {
		font-size: 1.25rem;
		font-family: var(--rtp-font-serif);
		color: var(--rtp-text);
		margin-block-end: 0.75rem;
	}
	.protocol__desc {
		color: var(--rtp-text-muted);
		font-weight: 300;
		line-height: 1.625;
		font-size: 0.875rem;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Engagement (prereqs + accordion)
	   ───────────────────────────────────────────────────────────────── */
	.engagement {
		padding-block: 8rem;
		padding-inline: 1.5rem;
		background: var(--rtp-bg-deeper);
		border-block-end: 1px solid var(--rtp-border-soft);
	}
	.engagement__grid {
		max-width: var(--rtp-content-wide);
		margin-inline: auto;
		display: grid;
		gap: 6rem;
	}
	@media (min-width: 1024px) {
		.engagement__grid {
			grid-template-columns: 1fr 1fr;
		}
	}
	.engagement__heading-row {
		display: flex;
		align-items: center;
		gap: 1rem;
		margin-block-end: 2.5rem;
	}
	.engagement__heading-icon {
		width: 2.5rem;
		height: 2.5rem;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: var(--rtp-radius-sm);
		background: color-mix(in oklab, #fff 5%, transparent);
		color: var(--rtp-text);
	}
	.engagement__heading {
		font-size: 1.5rem;
		font-family: var(--rtp-font-serif);
		color: var(--rtp-text);
	}

	.prereq {
		background: var(--rtp-bg-darker);
		border: 1px solid var(--rtp-border);
		padding: 2.5rem;
	}
	.prereq__intro {
		font-size: 0.875rem;
		color: var(--rtp-text-subtle);
		margin-block-end: 2rem;
		line-height: 1.625;
	}
	.prereq__list {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}
	.prereq__item {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		font-size: 0.875rem;
		font-family: var(--rtp-font-mono);
		color: var(--rtp-text-soft);
	}
	.prereq__bullet {
		color: var(--m-amber-600);
		flex-shrink: 0;
	}

	.accordion {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}
	.accordion__item {
		border: 1px solid var(--rtp-border);
		background: var(--rtp-bg-darker);
	}
	.accordion__trigger {
		width: 100%;
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem;
		text-align: start;
		background: transparent;
		border: 0;
		color: inherit;
		font: inherit;
		cursor: pointer;
		transition: background var(--rtp-dur-fast) var(--rtp-ease-out);
	}
	.accordion__trigger:hover {
		background: color-mix(in oklab, #fff 5%, transparent);
	}
	.accordion__trigger:focus-visible {
		outline: 2px solid var(--m-amber-500);
		outline-offset: -2px;
	}
	.accordion__label {
		font-weight: 700;
		color: var(--rtp-text);
		font-size: 0.875rem;
		letter-spacing: 0.025em;
	}
	.accordion :global(.accordion__chevron) {
		color: var(--rtp-text-subtle);
		transition: transform var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.accordion :global(.accordion__chevron--open) {
		transform: rotate(180deg);
	}
	.accordion__panel {
		padding: 1rem 1.5rem 1.5rem;
		color: var(--rtp-text-muted);
		font-size: 0.875rem;
		font-weight: 300;
		line-height: 1.625;
		border-block-start: 1px solid var(--rtp-border-soft);
	}

	/* ─────────────────────────────────────────────────────────────────
	   CTA (application card)
	   ───────────────────────────────────────────────────────────────── */
	.cta {
		padding-block: 10rem;
		padding-inline: 1.5rem;
		position: relative;
		overflow: hidden;
		background: var(--rtp-bg-darker);
	}
	.cta__vline,
	.cta__hline {
		position: absolute;
		background: var(--rtp-border-soft);
	}
	.cta__vline {
		inset-block-start: 0;
		inset-inline-start: 50%;
		translate: -50% 0;
		width: 1px;
		height: 100%;
	}
	.cta__hline {
		inset-block-start: 50%;
		inset-inline-start: 0;
		width: 100%;
		height: 1px;
	}

	.cta__shell {
		max-width: 56rem; /* max-w-4xl */
		margin-inline: auto;
		position: relative;
		z-index: 10;
		background: var(--rtp-bg-darker);
		border: 1px solid var(--rtp-border);
		padding: 0.25rem;
	}
	.cta__inner {
		border: 1px solid var(--rtp-border-soft);
		padding: 3rem;
		text-align: center;
		position: relative;
		overflow: hidden;
	}
	@media (min-width: 768px) {
		.cta__inner {
			padding: 5rem;
		}
	}
	.cta__halo {
		position: absolute;
		inset: 0;
		background: var(--m-amber-cta-halo);
		opacity: 0;
		transition: opacity 1s var(--rtp-ease-out);
		pointer-events: none;
	}
	.cta__inner:hover .cta__halo {
		opacity: 1;
	}

	.cta__head {
		margin-block-end: 2.5rem;
	}
	.cta__icon {
		color: var(--m-amber-600);
		margin-inline: auto;
		margin-block-end: 1.5rem;
	}
	.cta__title {
		font-size: 2.25rem; /* 4xl */
		font-family: var(--rtp-font-serif);
		color: var(--rtp-text);
		margin-block-end: 1rem;
	}
	@media (min-width: 768px) {
		.cta__title {
			font-size: 3rem; /* 5xl */
		}
	}
	.cta__ref {
		color: var(--rtp-text-subtle);
		font-family: var(--rtp-font-mono);
		font-size: 0.75rem;
		text-transform: uppercase;
		letter-spacing: 0.2em;
	}

	.cta__terms {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 2rem;
		max-width: 32rem;
		margin-inline: auto;
		margin-block-end: 3rem;
		text-align: start;
	}
	.cta__term {
		border-inline-start: 1px solid var(--m-amber-border-divider);
		padding-inline-start: 1rem;
	}
	.cta__term-label {
		font-size: 0.625rem;
		font-family: var(--rtp-font-mono);
		text-transform: uppercase;
		color: var(--rtp-text-subtle);
		margin-block-end: 0.25rem;
	}
	.cta__term-value {
		font-size: 1.25rem;
		color: var(--rtp-text);
		font-family: var(--rtp-font-serif);
	}

	.cta__button {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 100%;
		padding: 1.25rem 3rem;
		background: #fff;
		color: #000;
		font-weight: 700;
		font-size: 0.875rem;
		text-transform: uppercase;
		letter-spacing: 0.25em;
		text-decoration: none;
		transition: background 300ms var(--rtp-ease-out);
	}
	.cta__button:hover {
		background: var(--m-amber-500);
	}
	.cta__button:focus-visible {
		outline: 2px solid var(--m-amber-500);
		outline-offset: 4px;
	}
	@media (min-width: 768px) {
		.cta__button {
			width: auto;
		}
	}

	.cta__fine-print {
		margin-block-start: 2rem;
		font-size: 0.625rem;
		color: var(--rtp-text-faint);
		font-family: var(--rtp-font-mono);
		text-transform: uppercase;
	}
	.cta__br {
		display: none;
	}
	@media (min-width: 768px) {
		.cta__br {
			display: inline;
		}
	}
</style>

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
	const chevronClass = (idx: number) => ['chevron-disc', openIndex === idx && 'chevron-disc--open'];
</script>

<section class="syllabus-section">
	<div class="syllabus-container">
		<div class="syllabus-layout">
			<div data-gsap class="syllabus-intro">
				<div class="section-kicker">
					<span class="kicker-icon"><IconBook size={20} stroke={1.5} /></span>
					<span class="kicker-text">The Syllabus</span>
				</div>
				<h2 class="section-title">
					From Subjective to <br />
					<span>Objective.</span>
				</h2>
				<p class="section-copy">
					Most traders look at a chart and see "patterns." We look at a chart and see <strong
						>Auctions</strong
					>.
				</p>
				<p class="section-copy section-copy--last">
					We teach you to read the raw data of the market: Volume, Liquidity, and Time. This allows
					you to identify where the "Smart Money" is transacting, not just where price has been.
				</p>

				<div class="figure-frame">
					<div class="auction-figure">
						<div class="auction-grid"></div>
						<div class="volume-bar volume-bar--1"></div>
						<div class="volume-bar volume-bar--2"></div>
						<div class="volume-bar volume-bar--3"></div>
						<div class="volume-bar volume-bar--4"></div>
						<div class="volume-bar volume-bar--5"></div>

						<div class="figure-stats">
							<div>POC: 4450.25</div>
							<div>VAH: 4462.00</div>
							<div>VAL: 4438.50</div>
						</div>
						<p class="figure-caption">Figure 1.1: Auction Market Theory</p>
					</div>
				</div>
			</div>

			<div class="modules-list">
				{#each syllabus as module, i (module.title)}
					<div class="module-card">
						<button class="module-trigger" onclick={() => toggle(i)}>
							<div class="module-heading">
								<span class="module-number">MOD {i + 1}</span>
								<span class="module-title">{module.title}</span>
							</div>
							<div class={chevronClass(i)}>
								<IconChevronDown size={20} stroke={1.5} />
							</div>
						</button>
						{#if openIndex === i}
							<div transition:slide class="module-body">
								{module.desc}
								<div class="module-resources">
									<span><IconCheck size={12} stroke={2} /> Video</span>
									<span><IconCheck size={12} stroke={2} /> PDF</span>
									<span><IconCheck size={12} stroke={2} /> Quiz</span>
								</div>
							</div>
						{/if}
					</div>
				{/each}
			</div>
		</div>
	</div>
</section>

<style>
	.syllabus-section {
		border-top: 1px solid rgb(255 255 255 / 0.05);
		background: #020202;
		padding-block: 8rem;
	}

	.syllabus-container {
		width: min(100% - 2rem, 80rem);
		margin-inline: auto;
	}

	.syllabus-layout {
		display: grid;
		gap: 5rem;
	}

	.syllabus-intro {
		height: fit-content;
	}

	.section-kicker,
	.module-heading,
	.module-trigger,
	.module-resources,
	.module-resources span,
	.chevron-disc {
		display: flex;
		align-items: center;
	}

	.section-kicker {
		gap: 0.75rem;
		margin-bottom: 1.5rem;
	}

	.kicker-icon {
		display: inline-flex;
		border: 1px solid rgb(59 130 246 / 0.2);
		border-radius: 0.5rem;
		background: rgb(59 130 246 / 0.1);
		color: rgb(59 130 246);
		padding: 0.5rem;
	}

	.kicker-text {
		color: rgb(96 165 250);
		font-size: 0.875rem;
		font-weight: 700;
		letter-spacing: 0.14em;
		line-height: 1.2;
		text-transform: uppercase;
	}

	.section-title {
		margin: 0 0 2rem;
		color: #fff;
		font-family: var(--font-heading);
		font-size: clamp(2.25rem, 5vw, 3rem);
		font-weight: 700;
		line-height: 1.08;
	}

	.section-title span {
		border-bottom: 4px solid var(--rtp-primary);
		color: #fff;
	}

	.section-copy {
		margin: 0 0 2rem;
		color: rgb(148 163 184);
		font-size: 1.125rem;
		line-height: 1.65;
	}

	.section-copy--last {
		margin-bottom: 3rem;
	}

	.figure-frame {
		border: 1px solid rgb(255 255 255 / 0.1);
		border-radius: 0.75rem;
		background: rgb(255 255 255 / 0.05);
		padding: 0.25rem;
	}

	.auction-figure {
		position: relative;
		display: flex;
		aspect-ratio: 16 / 9;
		align-items: center;
		justify-content: center;
		overflow: hidden;
		border-radius: 0.5rem;
		background: #0f172a;
	}

	.auction-grid {
		position: absolute;
		inset: 0;
		background-image:
			linear-gradient(0deg, rgb(255 255 255 / 0.03) 1px, transparent 1px),
			linear-gradient(90deg, rgb(255 255 255 / 0.03) 1px, transparent 1px);
		background-size: 20px 20px;
	}

	.volume-bar {
		position: absolute;
		left: 0;
		height: 1rem;
		border-radius: 0 0.25rem 0.25rem 0;
		background: rgb(59 130 246 / 0.2);
	}

	.volume-bar--1 {
		bottom: 5rem;
		width: 8rem;
	}

	.volume-bar--2 {
		bottom: 6rem;
		width: 12rem;
		background: rgb(59 130 246 / 0.4);
	}

	.volume-bar--3 {
		bottom: 7rem;
		width: 16rem;
		background: rgb(59 130 246 / 0.6);
	}

	.volume-bar--4 {
		bottom: 8rem;
		width: 10rem;
		background: rgb(59 130 246 / 0.4);
	}

	.volume-bar--5 {
		bottom: 9rem;
		width: 6rem;
	}

	.figure-stats {
		position: absolute;
		top: 2.5rem;
		right: 2.5rem;
		color: rgb(100 116 139);
		font-family:
			ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New',
			monospace;
		font-size: 0.75rem;
		line-height: 1.5;
		text-align: right;
	}

	.figure-caption {
		position: relative;
		z-index: 1;
		margin: 6rem 0 0;
		color: rgb(100 116 139);
		font-family:
			ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New',
			monospace;
		font-size: 0.75rem;
		letter-spacing: 0.14em;
		line-height: 1.2;
		text-align: center;
		text-transform: uppercase;
	}

	.modules-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		padding-top: 2rem;
	}

	.module-card {
		overflow: hidden;
		border: 1px solid rgb(255 255 255 / 0.1);
		border-radius: 0.75rem;
		background: #0a0a0a;
		transition: border-color 300ms ease;
	}

	.module-card:hover {
		border-color: color-mix(in oklab, var(--rtp-primary) 50%, transparent);
	}

	.module-trigger {
		width: 100%;
		justify-content: space-between;
		gap: 1rem;
		border: 0;
		background: transparent;
		color: inherit;
		cursor: pointer;
		font: inherit;
		padding: 1.5rem;
		text-align: left;
	}

	.module-heading {
		gap: 1.5rem;
		min-width: 0;
	}

	.module-number {
		border: 1px solid color-mix(in oklab, var(--rtp-primary) 20%, transparent);
		border-radius: 0.25rem;
		background: color-mix(in oklab, var(--rtp-primary) 10%, transparent);
		color: var(--rtp-primary);
		font-family:
			ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New',
			monospace;
		font-size: 0.875rem;
		line-height: 1.2;
		padding: 0.25rem 0.75rem;
		white-space: nowrap;
	}

	.module-title {
		color: #fff;
		font-size: 1.125rem;
		font-weight: 700;
		line-height: 1.35;
		transition: color 160ms ease;
	}

	.module-card:hover .module-title {
		color: var(--rtp-primary);
	}

	.chevron-disc {
		width: 2rem;
		height: 2rem;
		flex: 0 0 auto;
		justify-content: center;
		border-radius: 999px;
		background: rgb(255 255 255 / 0.05);
		color: rgb(100 116 139);
		transition:
			background-color 300ms ease,
			color 300ms ease,
			transform 300ms ease;
	}

	.chevron-disc--open {
		background: rgb(255 255 255 / 0.1);
		color: #fff;
		transform: rotate(180deg);
	}

	.module-body {
		border-top: 1px solid rgb(255 255 255 / 0.05);
		color: rgb(148 163 184);
		font-size: 0.875rem;
		line-height: 1.65;
		padding: 1.5rem 1.5rem 2rem 5.5rem;
	}

	.module-resources {
		gap: 1rem;
		margin-top: 1rem;
	}

	.module-resources span {
		gap: 0.25rem;
		color: rgb(52 211 153);
		font-family:
			ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New',
			monospace;
		font-size: 0.75rem;
		line-height: 1.2;
	}

	@media (min-width: 640px) {
		.syllabus-container {
			width: min(100% - 3rem, 80rem);
		}
	}

	@media (min-width: 1024px) {
		.syllabus-layout {
			grid-template-columns: repeat(12, minmax(0, 1fr));
		}

		.syllabus-intro {
			position: sticky;
			top: 8rem;
			grid-column: span 5;
		}

		.modules-list {
			grid-column: span 7;
		}
	}

	@media (max-width: 640px) {
		.syllabus-section {
			padding-block: 5rem;
		}

		.module-trigger {
			align-items: flex-start;
		}

		.module-heading {
			align-items: flex-start;
			flex-direction: column;
			gap: 0.75rem;
		}

		.module-body {
			padding-inline: 1.25rem;
		}

		.module-resources {
			flex-wrap: wrap;
		}

		.figure-stats {
			right: 1rem;
		}
	}
</style>

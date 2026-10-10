<script lang="ts">
	import gsap from 'gsap';
	import { untrack } from 'svelte';
	import { prefersReducedMotion } from 'svelte/motion';
	import type { Attachment } from 'svelte/attachments';
	import type { ScrollTrigger } from 'gsap/ScrollTrigger';
	import { createCalculatorState } from '../state/calculator.svelte.js';
	import { createMarketDataService } from '../data/market-data-service.svelte.js';
	import InputPanel from './InputPanel.svelte';
	import ResultsBar from './ResultsBar.svelte';
	import GreeksPanel from './GreeksPanel.svelte';
	import VisualizationTabs from './VisualizationTabs.svelte';
	import StrategyBuilder from './StrategyBuilder.svelte';
	import ScenarioEngine from './ScenarioEngine.svelte';
	import TimeMachine from './TimeMachine.svelte';
	import TickerSearch from './TickerSearch.svelte';
	import LiveDataToggle from './LiveDataToggle.svelte';
	import DataSourceBadge from './DataSourceBadge.svelte';
	import MispricingAlert from './MispricingAlert.svelte';
	import GlassCard from './ui/GlassCard.svelte';
	import ThemeToggle from './ui/ThemeToggle.svelte';
	import Toast from './ui/Toast.svelte';
	import EducationOverlay from './education/EducationOverlay.svelte';
	import ExportMenu from './export/ExportMenu.svelte';
	import ExportPNG from './export/ExportPNG.svelte';
	import ExportCSV from './export/ExportCSV.svelte';
	import ShareLink from './export/ShareLink.svelte';
	import EmbedCodeGenerator from './export/EmbedCodeGenerator.svelte';
	import SaveConfigModal from './saved/SaveConfigModal.svelte';
	import SavedConfigs from './saved/SavedConfigs.svelte';
	import CommandPalette from './power-user/CommandPalette.svelte';
	import KeyboardShortcuts from './power-user/KeyboardShortcuts.svelte';
	import CTABanner from './growth/CTABanner.svelte';
	import LeadCaptureModal from './growth/LeadCaptureModal.svelte';
	import IconCalculator from '@tabler/icons-svelte-runes/icons/calculator';
	import IconBolt from '@tabler/icons-svelte-runes/icons/bolt';
	import IconFolderOpen from '@tabler/icons-svelte-runes/icons/folder-open';
	import type { MarketSnapshot } from '../data/types.js';

	interface Props {
		/** Level of the calculator's title; 2 when the page already has an <h1> (the hero). */
		headingLevel?: 1 | 2;
	}

	let { headingLevel = 1 }: Props = $props();

	const calc = createCalculatorState();
	const marketData = createMarketDataService();

	let exportCSVRef: ExportCSV | undefined = $state();
	let calculatorEl: HTMLDivElement | undefined = $state();

	function handleSnapshot(snapshot: MarketSnapshot) {
		calc.applyMarketSnapshot(snapshot);
	}

	let themeApplied = false;

	$effect(() => {
		const t = calc.theme;
		if (typeof document !== 'undefined') {
			document.documentElement.setAttribute('data-calc-theme', t);
		}
		// Soften the swap with a brief fade on toggles (not on the initial apply).
		if (!themeApplied) {
			themeApplied = true;
			return;
		}
		untrack(() => {
			if (calculatorEl && !prefersReducedMotion.current) {
				calculatorEl.animate([{ opacity: 0.85 }, { opacity: 1 }], {
					duration: 400,
					easing: 'ease-out'
				});
			}
		});
	});

	/** Fixed bar along the top edge tracking how far through the calculator the reader is. */
	const scrollProgress: Attachment<HTMLDivElement> = (bar) => {
		const trigger = calculatorEl;
		if (!trigger) return;
		let disposed = false;
		let progressTrigger: ScrollTrigger | undefined;
		void import('gsap/ScrollTrigger').then(({ ScrollTrigger: plugin }) => {
			if (disposed) return;
			gsap.registerPlugin(plugin);
			progressTrigger = plugin.create({
				trigger,
				start: 'top top',
				end: 'bottom bottom',
				onUpdate: (self) => {
					bar.style.transform = `scaleX(${self.progress})`;
				}
			});
		});
		return () => {
			disposed = true;
			progressTrigger?.kill();
		};
	};

	/** Hover ripple: centres the button's highlight on the pointer. */
	function handleRippleMove(event: PointerEvent & { currentTarget: HTMLButtonElement }) {
		const target = event.currentTarget;
		const rect = target.getBoundingClientRect();
		target.style.setProperty('--ripple-x', `${((event.clientX - rect.left) / rect.width) * 100}%`);
		target.style.setProperty('--ripple-y', `${((event.clientY - rect.top) / rect.height) * 100}%`);
	}

	function handleExportCSV(): void {
		exportCSVRef?.exportCSV();
	}

	function handleSavedConfigsRefresh(): void {
		// Placeholder for any refresh logic after saving
	}
</script>

<div class="calc-scroll-progress" {@attach scrollProgress} aria-hidden="true"></div>

<div class="calc-layout" bind:this={calculatorEl}>
	<!-- Header -->
	<div class="col-span-full flex items-center justify-between px-1">
		<div class="flex items-center gap-3">
			<div
				class="flex items-center justify-center w-9 h-9 rounded-xl"
				style="background: var(--calc-accent-glow); color: var(--calc-accent);"
			>
				<IconCalculator size={20} />
			</div>
			<div>
				<svelte:element
					this={headingLevel === 1 ? 'h1' : 'h2'}
					class="text-lg font-bold tracking-tight"
					style="color: var(--calc-text); font-family: var(--calc-font-display);"
				>
					Black-Scholes Calculator
				</svelte:element>
				<p class="text-xs" style="color: var(--calc-text-muted);">
					Institutional-grade options pricing
				</p>
			</div>
		</div>
		<div class="flex items-center gap-2 flex-wrap">
			<!-- Live Data Toggle -->
			<LiveDataToggle {marketData} />

			<!-- Ticker Search (visible in live mode) -->
			{#if marketData.dataMode === 'live'}
				<TickerSearch {marketData} onSnapshot={handleSnapshot} />
			{/if}

			<!-- Mode Toggle -->
			<div class="flex rounded-lg overflow-hidden" style="border: 1px solid var(--calc-border);">
				<button
					onclick={() => (calc.calculatorMode = 'single')}
					onpointermove={handleRippleMove}
					class="calc-ripple text-[10px] font-medium px-2.5 py-1 transition-colors cursor-pointer"
					style={calc.calculatorMode === 'single'
						? 'background: var(--calc-accent-glow); color: var(--calc-accent);'
						: 'color: var(--calc-text-muted);'}>Single</button
				>
				<button
					onclick={() => (calc.calculatorMode = 'strategy')}
					onpointermove={handleRippleMove}
					class="calc-ripple text-[10px] font-medium px-2.5 py-1 transition-colors cursor-pointer"
					style={calc.calculatorMode === 'strategy'
						? 'background: var(--calc-accent-glow); color: var(--calc-accent);'
						: 'color: var(--calc-text-muted);'}>Strategy</button
				>
			</div>

			<!-- Scenario Toggle -->
			<button
				onclick={() => (calc.isScenarioPanelOpen = !calc.isScenarioPanelOpen)}
				onpointermove={handleRippleMove}
				class="calc-ripple flex items-center gap-1 text-[10px] font-medium px-2 py-1 rounded-lg transition-colors cursor-pointer"
				style={calc.isScenarioPanelOpen
					? 'background: var(--calc-warning); color: white;'
					: 'background: var(--calc-surface); color: var(--calc-text-muted); border: 1px solid var(--calc-border);'}
			>
				<IconBolt size={10} />
				Scenarios
			</button>

			<!-- Data Source Badge -->
			<DataSourceBadge {marketData} />

			<!-- Phase 4: Education Toggle -->
			<EducationOverlay {calc} />

			<!-- Phase 4: Export Menu -->
			<ExportMenu
				onExportPNG={() => (calc.showExportPNG = true)}
				onExportCSV={handleExportCSV}
				onShareLink={() => (calc.showShareLink = true)}
				onEmbedCode={() => (calc.showEmbedCode = true)}
			/>

			<!-- Phase 4: Saved Configs Button -->
			<button
				onclick={() => (calc.showSavedConfigs = true)}
				class="flex items-center gap-1 text-[10px] font-medium px-2.5 py-1.5 rounded-lg transition-colors cursor-pointer"
				style="background: var(--calc-surface); color: var(--calc-text-muted); border: 1px solid var(--calc-border);"
				title="Saved configurations"
			>
				<IconFolderOpen size={11} />
				<span class="hidden sm:inline">Saved</span>
			</button>

			<ThemeToggle theme={calc.theme} onToggle={() => calc.toggleTheme()} />
		</div>
	</div>

	<!-- Left Panel: Inputs -->
	<aside class="hidden lg:block row-span-1 lg:row-span-2">
		<GlassCard class="h-full" animate={true} delay={0.1}>
			<InputPanel {calc} />
		</GlassCard>
	</aside>

	<!-- Mobile Input Toggle -->
	<div class="lg:hidden col-span-full">
		<button
			onclick={() => (calc.isInputPanelOpen = !calc.isInputPanelOpen)}
			class="w-full text-sm font-medium py-2.5 rounded-xl transition-colors cursor-pointer"
			style="background: var(--calc-surface); color: var(--calc-text-secondary); border: 1px solid var(--calc-border);"
		>
			{calc.isInputPanelOpen ? 'Hide' : 'Show'} Parameters
		</button>
		{#if calc.isInputPanelOpen}
			<div class="mt-3">
				<GlassCard animate={true}>
					<InputPanel {calc} />
				</GlassCard>
			</div>
		{/if}
	</div>

	<!-- Main Content -->
	<main class="flex flex-col gap-4 min-w-0">
		<!-- Results -->
		<GlassCard animate={true} delay={0.2}>
			<ResultsBar {calc} />
		</GlassCard>

		<!-- Strategy Builder (visible in strategy mode) -->
		{#if calc.calculatorMode === 'strategy'}
			<GlassCard animate={true} delay={0.25}>
				<StrategyBuilder {calc} />
			</GlassCard>
		{/if}

		<!-- Scenario Engine (toggle panel) -->
		{#if calc.isScenarioPanelOpen}
			<GlassCard animate={true} delay={0.25}>
				<ScenarioEngine {calc} />
			</GlassCard>
		{/if}

		<!-- Greeks -->
		<GlassCard animate={true} delay={0.3}>
			<GreeksPanel {calc} />
		</GlassCard>

		<!-- Mispricing Alerts -->
		{#if marketData.currentChain}
			<GlassCard animate={true} delay={0.35}>
				<MispricingAlert {marketData} {calc} />
			</GlassCard>
		{/if}

		<!-- Visualizations -->
		<GlassCard animate={true} delay={0.4}>
			<VisualizationTabs {calc} {marketData} />
		</GlassCard>

		<!-- Time Machine -->
		<GlassCard animate={true} delay={0.45}>
			<TimeMachine {calc} />
		</GlassCard>

		<!-- Phase 4: CTA Banner -->
		<CTABanner />
	</main>
</div>

<!-- Phase 4: Global Components (always mounted) -->
<KeyboardShortcuts {calc} onExportCSV={handleExportCSV} />
<CommandPalette {calc} />
<Toast {calc} />
<ExportCSV bind:this={exportCSVRef} {calc} />
<ExportPNG {calc} captureElement={calculatorEl} />
<ShareLink {calc} />
<EmbedCodeGenerator {calc} />
<SaveConfigModal {calc} onSaved={handleSavedConfigsRefresh} />
<SavedConfigs {calc} />
<LeadCaptureModal />

<style>
	.calc-scroll-progress {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		height: 3px;
		/* Above the sticky site navbar (z-index 1000). */
		z-index: 1001;
		background: var(--calc-accent);
		transform: scaleX(0);
		transform-origin: left;
		border-radius: 0 2px 2px 0;
		pointer-events: none;
	}

	.calc-ripple {
		position: relative;
		overflow: hidden;
	}

	.calc-ripple::after {
		content: '';
		position: absolute;
		inset: 0;
		background: radial-gradient(
			circle at var(--ripple-x, 50%) var(--ripple-y, 50%),
			rgba(255, 255, 255, 0.15) 0%,
			transparent 60%
		);
		opacity: 0;
		transition: opacity 0.3s;
		pointer-events: none;
	}

	.calc-ripple:hover::after {
		opacity: 1;
	}
</style>

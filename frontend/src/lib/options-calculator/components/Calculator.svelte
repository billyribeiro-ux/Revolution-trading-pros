<script lang="ts">
	import { untrack } from 'svelte';
	import { page } from '$app/state';
	import { prefersReducedMotion } from 'svelte/motion';
	import { innerHeight, scrollY } from 'svelte/reactivity/window';
	import type { Attachment } from 'svelte/attachments';
	import { CalculatorState } from '../state/calculator.svelte.js';
	import { MarketDataService } from '../data/market-data-service.svelte.js';
	import { setCalculator, setMarketData } from '../state/context.js';
	import { decodeState } from '../utils/share-utils.js';
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
	import type { CalculatorMode } from '../engine/types.js';

	interface Props {
		/** Level of the calculator's title; 2 when the page already has an <h1> (the hero). */
		headingLevel?: 1 | 2;
	}

	let { headingLevel = 1 }: Props = $props();

	// This component owns the calculator's state; every descendant reads it from context.
	const calc = setCalculator(new CalculatorState());
	const marketData = setMarketData(new MarketDataService());

	const inputPanelId = $props.id();

	/**
	 * The query string of the page, as one string, so hash-only navigations (the hero's
	 * "jump to calculator" link) do not count as a change.
	 */
	const sharedQuery = $derived(page.url.search);

	// A share link (ShareLink.svelte) carries the setup in the query string. It is applied
	// after hydration: the main page is prerendered, so its query is only known in the
	// browser. It is applied again only if the query itself changes (another shared link).
	$effect(() => {
		const shared = decodeState(new URLSearchParams(sharedQuery));
		untrack(() => calc.applyShareableState(shared));
	});

	const CALCULATOR_MODES: { id: CalculatorMode; label: string }[] = [
		{ id: 'single', label: 'Single' },
		{ id: 'strategy', label: 'Strategy' }
	];

	/** Only used from event handlers, so it needs no reactivity. */
	let csvExporter: ExportCSV | undefined;
	let calculatorEl = $state<HTMLDivElement>();
	let calculatorHeight = $state(0);

	// The theme tokens in calculator-theme.css key off <html data-calc-theme>, so overlays
	// rendered outside .calc-layout (toasts, modals, the command palette) follow the theme too.
	$effect(() => {
		const root = document.documentElement;
		root.setAttribute('data-calc-theme', calc.theme);
		return () => root.removeAttribute('data-calc-theme');
	});

	/** Softens theme toggles with a brief fade (never on the initial theme). */
	const fadeOnThemeChange: Attachment<HTMLElement> = (node) => {
		let shownTheme = untrack(() => calc.theme);
		$effect(() => {
			if (calc.theme === shownTheme) return;
			shownTheme = calc.theme;
			if (prefersReducedMotion.current) return;
			node.animate([{ opacity: 0.85 }, { opacity: 1 }], { duration: 400, easing: 'ease-out' });
		});
	};

	/**
	 * Reading progress through the calculator for the bar along the top edge: 0 while its top
	 * is below the viewport top, 1 once its bottom reaches the viewport bottom. Recomputed on
	 * scroll, on viewport resize and whenever the calculator's own height changes (strategy
	 * mode, scenario panel, second-order Greeks, mispricing alert, taller or shorter charts).
	 */
	const scrollProgress = $derived.by(() => {
		const scrolled = scrollY.current;
		const viewportHeight = innerHeight.current;
		// Nothing to track on the server, before mount, or while the calculator has no height.
		if (!calculatorEl || !calculatorHeight) return 0;
		if (scrolled === undefined || viewportHeight === undefined) return 0;
		// Document offset of the calculator's top edge.
		const start = calculatorEl.getBoundingClientRect().top + scrolled;
		const distance = calculatorHeight - viewportHeight;
		if (distance <= 0) return scrolled >= start ? 1 : 0;
		return Math.min(Math.max((scrolled - start) / distance, 0), 1);
	});

	/** Hover ripple: centres the button's highlight on the pointer. */
	function handleRippleMove(event: PointerEvent & { currentTarget: HTMLButtonElement }) {
		const target = event.currentTarget;
		const rect = target.getBoundingClientRect();
		target.style.setProperty('--ripple-x', `${((event.clientX - rect.left) / rect.width) * 100}%`);
		target.style.setProperty('--ripple-y', `${((event.clientY - rect.top) / rect.height) * 100}%`);
	}

	function handleExportCSV(): void {
		csvExporter?.exportCSV();
	}

	function handleConfigSaved(): void {
		// Nothing to refresh: SavedConfigs re-reads storage every time it opens.
	}
</script>

<div
	class="calc-scroll-progress"
	style:transform="scaleX({scrollProgress})"
	aria-hidden="true"
></div>

<div
	class="calc-layout"
	bind:this={calculatorEl}
	bind:offsetHeight={calculatorHeight}
	{@attach fadeOnThemeChange}
>
	<!-- Header -->
	<div class="col-span-full flex items-center justify-between px-1">
		<div class="flex items-center gap-3">
			<div
				class="flex items-center justify-center w-9 h-9 rounded-xl"
				style="background: var(--calc-accent-glow); color: var(--calc-accent);"
			>
				<IconCalculator size={20} aria-hidden="true" />
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
			<LiveDataToggle />

			<!-- Ticker Search (visible in live mode) -->
			{#if marketData.dataMode === 'live'}
				<TickerSearch onSnapshot={calc.applyMarketSnapshot} />
			{/if}

			<!-- Mode Toggle -->
			<div
				class="flex rounded-lg overflow-hidden"
				style="border: 1px solid var(--calc-border);"
				role="group"
				aria-label="Calculator mode"
			>
				{#each CALCULATOR_MODES as mode (mode.id)}
					{const active = $derived(calc.calculatorMode === mode.id)}
					<button
						type="button"
						onclick={() => (calc.calculatorMode = mode.id)}
						onpointermove={handleRippleMove}
						aria-pressed={active}
						class="calc-ripple text-[10px] font-medium px-2.5 py-1 transition-colors motion-reduce:transition-none cursor-pointer"
						style:background={active ? 'var(--calc-accent-glow)' : undefined}
						style:color={active ? 'var(--calc-accent)' : 'var(--calc-text-muted)'}
					>
						{mode.label}
					</button>
				{/each}
			</div>

			<!-- Scenario Toggle -->
			<button
				type="button"
				onclick={() => (calc.isScenarioPanelOpen = !calc.isScenarioPanelOpen)}
				onpointermove={handleRippleMove}
				aria-pressed={calc.isScenarioPanelOpen}
				class="calc-ripple flex items-center gap-1 text-[10px] font-medium px-2 py-1 rounded-lg transition-colors motion-reduce:transition-none cursor-pointer"
				style:background={calc.isScenarioPanelOpen ? 'var(--calc-warning)' : 'var(--calc-surface)'}
				style:color={calc.isScenarioPanelOpen ? 'white' : 'var(--calc-text-muted)'}
				style:border={calc.isScenarioPanelOpen ? undefined : '1px solid var(--calc-border)'}
			>
				<IconBolt size={10} aria-hidden="true" />
				Scenarios
			</button>

			<!-- Data Source Badge -->
			<DataSourceBadge />

			<!-- Phase 4: Education Toggle -->
			<EducationOverlay />

			<!-- Phase 4: Export Menu -->
			<ExportMenu
				onExportPNG={() => (calc.showExportPNG = true)}
				onExportCSV={handleExportCSV}
				onShareLink={() => (calc.showShareLink = true)}
				onEmbedCode={() => (calc.showEmbedCode = true)}
			/>

			<!-- Phase 4: Saved Configs Button -->
			<button
				type="button"
				onclick={() => (calc.showSavedConfigs = true)}
				class="flex items-center gap-1 text-[10px] font-medium px-2.5 py-1.5 rounded-lg transition-colors motion-reduce:transition-none cursor-pointer"
				style="background: var(--calc-surface); color: var(--calc-text-muted); border: 1px solid var(--calc-border);"
				title="Saved configurations"
				aria-label="Saved configurations"
			>
				<IconFolderOpen size={11} aria-hidden="true" />
				<span class="hidden sm:inline">Saved</span>
			</button>

			<ThemeToggle theme={calc.theme} onToggle={calc.toggleTheme} />
		</div>
	</div>

	<!-- Left Panel: Inputs -->
	<aside class="hidden lg:block row-span-1 lg:row-span-2">
		<GlassCard class="h-full" animate={true} delay={0.1}>
			<InputPanel />
		</GlassCard>
	</aside>

	<!-- Mobile Input Toggle -->
	<div class="lg:hidden col-span-full">
		<button
			type="button"
			onclick={() => (calc.isInputPanelOpen = !calc.isInputPanelOpen)}
			aria-expanded={calc.isInputPanelOpen}
			aria-controls={inputPanelId}
			class="w-full text-sm font-medium py-2.5 rounded-xl transition-colors motion-reduce:transition-none cursor-pointer"
			style="background: var(--calc-surface); color: var(--calc-text-secondary); border: 1px solid var(--calc-border);"
		>
			{calc.isInputPanelOpen ? 'Hide' : 'Show'} Parameters
		</button>
		<div id={inputPanelId}>
			{#if calc.isInputPanelOpen}
				<div class="mt-3">
					<GlassCard animate={true}>
						<InputPanel />
					</GlassCard>
				</div>
			{/if}
		</div>
	</div>

	<!-- Main Content (the page layout already provides the <main> landmark) -->
	<!-- data-capture-region marks what the screenshot zones select (ExportPNG). -->
	<div class="flex flex-col gap-4 min-w-0" data-capture-region="main">
		<!-- Results -->
		<GlassCard animate={true} delay={0.2} data-capture-region="results">
			<ResultsBar />
		</GlassCard>

		<!-- Strategy Builder (visible in strategy mode) -->
		{#if calc.calculatorMode === 'strategy'}
			<GlassCard animate={true} delay={0.25}>
				<StrategyBuilder />
			</GlassCard>
		{/if}

		<!-- Scenario Engine (toggle panel) -->
		{#if calc.isScenarioPanelOpen}
			<GlassCard animate={true} delay={0.25}>
				<ScenarioEngine />
			</GlassCard>
		{/if}

		<!-- Greeks -->
		<GlassCard animate={true} delay={0.3}>
			<GreeksPanel />
		</GlassCard>

		<!-- Mispricing Alerts -->
		{#if marketData.currentChain}
			<GlassCard animate={true} delay={0.35}>
				<MispricingAlert />
			</GlassCard>
		{/if}

		<!-- Visualizations -->
		<GlassCard animate={true} delay={0.4} data-capture-region="chart">
			<VisualizationTabs />
		</GlassCard>

		<!-- Time Machine -->
		<GlassCard animate={true} delay={0.45}>
			<TimeMachine />
		</GlassCard>

		<!-- Phase 4: CTA Banner -->
		<CTABanner />
	</div>
</div>

<!-- Phase 4: Global Components (always mounted) -->
<KeyboardShortcuts onExportCSV={handleExportCSV} />
<CommandPalette />
<Toast />
<ExportCSV bind:this={csvExporter} />
<ExportPNG captureElement={calculatorEl} />
<ShareLink />
<EmbedCodeGenerator />
<SaveConfigModal onSaved={handleConfigSaved} />
<SavedConfigs />
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

	@media (prefers-reduced-motion: reduce) {
		.calc-ripple::after {
			transition: none;
		}
	}
</style>

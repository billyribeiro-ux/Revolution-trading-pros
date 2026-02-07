<script lang="ts">
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
	import { Calculator as CalculatorIcon, Zap, FolderOpen } from '@lucide/svelte';
	import type { MarketSnapshot } from '../data/types.js';

	const calc = createCalculatorState();
	const marketData = createMarketDataService();

	let exportCSVRef: ExportCSV | undefined = $state();
	let calculatorEl: HTMLDivElement | undefined = $state();

	function handleSnapshot(snapshot: MarketSnapshot) {
		calc.applyMarketSnapshot(snapshot);
	}

	$effect(() => {
		const t = calc.theme;
		if (typeof document !== 'undefined') {
			document.documentElement.setAttribute('data-calc-theme', t);
		}
	});

	function handleExportCSV(): void {
		exportCSVRef?.exportCSV();
	}

	function handleSavedConfigsRefresh(): void {
		// Placeholder for any refresh logic after saving
	}
</script>

<div class="calc-layout">
	<!-- Header -->
	<div class="col-span-full flex items-center justify-between px-1">
		<div class="flex items-center gap-3">
			<div
				class="flex items-center justify-center w-9 h-9 rounded-xl"
				style="background: var(--calc-accent-glow); color: var(--calc-accent);"
			>
				<CalculatorIcon size={20} />
			</div>
			<div>
				<h1
					class="text-lg font-bold tracking-tight"
					style="color: var(--calc-text); font-family: var(--calc-font-display);"
				>
					Black-Scholes Calculator
				</h1>
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
					class="text-[10px] font-medium px-2.5 py-1 transition-colors cursor-pointer"
					style={calc.calculatorMode === 'single'
						? 'background: var(--calc-accent-glow); color: var(--calc-accent);'
						: 'color: var(--calc-text-muted);'}
				>Single</button>
				<button
					onclick={() => (calc.calculatorMode = 'strategy')}
					class="text-[10px] font-medium px-2.5 py-1 transition-colors cursor-pointer"
					style={calc.calculatorMode === 'strategy'
						? 'background: var(--calc-accent-glow); color: var(--calc-accent);'
						: 'color: var(--calc-text-muted);'}
				>Strategy</button>
			</div>

			<!-- Scenario Toggle -->
			<button
				onclick={() => (calc.isScenarioPanelOpen = !calc.isScenarioPanelOpen)}
				class="flex items-center gap-1 text-[10px] font-medium px-2 py-1 rounded-lg transition-colors cursor-pointer"
				style={calc.isScenarioPanelOpen
					? 'background: var(--calc-warning); color: white;'
					: 'background: var(--calc-surface); color: var(--calc-text-muted); border: 1px solid var(--calc-border);'}
			>
				<Zap size={10} />
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
				<FolderOpen size={11} />
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

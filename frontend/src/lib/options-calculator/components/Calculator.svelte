<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
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
	import LazySection from '$lib/components/LazySection.svelte';
	import { Calculator as CalculatorIcon, Lightning as Zap, FolderOpen } from 'phosphor-svelte';
	import type { MarketSnapshot } from '../data/types.js';

	const calc = createCalculatorState();
	const marketData = createMarketDataService();

	let exportCSVRef: ExportCSV | undefined = $state();
	let calculatorEl: HTMLDivElement | undefined = $state();
	let progressBarEl: HTMLDivElement | undefined = $state();

	function handleSnapshot(snapshot: MarketSnapshot) {
		calc.applyMarketSnapshot(snapshot);
	}

	$effect(() => {
		const t = calc.theme;
		if (typeof document !== 'undefined') {
			document.documentElement.setAttribute('data-calc-theme', t);
			// Subtle flash transition on theme change
			if (browser && calculatorEl) {
				import('gsap').then(({ default: gsap }) => {
					if (!calculatorEl) return;
					gsap.fromTo(
						calculatorEl,
						{ opacity: 0.85 },
						{ opacity: 1, duration: 0.4, ease: 'power2.out' }
					);
				});
			}
		}
	});

	function handleExportCSV(): void {
		exportCSVRef?.exportCSV();
	}

	function handleSavedConfigsRefresh(): void {
		// Placeholder for any refresh logic after saving
	}

	// --- GSAP ScrollTrigger stagger entrance + scroll progress indicator ---
	onMount(async () => {
		if (!browser) return;

		// Fetch admin-configured active provider
		marketData.refreshActiveProvider();
		const prefersReducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
		if (prefersReducedMotion) return;

		const gsap = (await import('gsap')).default;
		const { ScrollTrigger } = await import('gsap/ScrollTrigger');
		gsap.registerPlugin(ScrollTrigger);

		// Stagger entrance for all GlassCard sections
		const sections = calculatorEl?.querySelectorAll('.glass-card');
		if (sections?.length) {
			gsap.set(sections, { opacity: 0, y: 30 });
			sections.forEach((section, i) => {
				gsap.to(section, {
					opacity: 1,
					y: 0,
					duration: 0.7,
					delay: i * 0.12,
					ease: 'power2.out',
					scrollTrigger: {
						trigger: section,
						start: 'top 85%',
						toggleActions: 'play none none none'
					}
				});
			});
		}

		// Scroll progress bar
		if (progressBarEl && calculatorEl) {
			gsap.to(progressBarEl, {
				scaleX: 1,
				ease: 'none',
				scrollTrigger: {
					trigger: calculatorEl,
					start: 'top top',
					end: 'bottom bottom',
					scrub: true
				}
			});
		}
	});

	// --- Micro-interaction: ripple coordinate tracking ---
	function handleRippleMove(e: PointerEvent) {
		const target = e.currentTarget as HTMLElement;
		const rect = target.getBoundingClientRect();
		const x = ((e.clientX - rect.left) / rect.width) * 100;
		const y = ((e.clientY - rect.top) / rect.height) * 100;
		target.style.setProperty('--ripple-x', `${x}%`);
		target.style.setProperty('--ripple-y', `${y}%`);
	}
</script>

<div class="calc-layout" bind:this={calculatorEl}>
	<!-- Scroll progress indicator -->
	<div class="calc-scroll-progress col-span-full" bind:this={progressBarEl}></div>

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
		<LazySection rootMargin="300px">
			<GlassCard animate={true} delay={0.4}>
				<VisualizationTabs {calc} {marketData} />
			</GlassCard>
		</LazySection>

		<!-- Time Machine -->
		<LazySection rootMargin="300px">
			<GlassCard animate={true} delay={0.45}>
				<TimeMachine {calc} />
			</GlassCard>
		</LazySection>

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
	/* Scroll progress indicator */
	.calc-scroll-progress {
		position: sticky;
		top: 0;
		height: 3px;
		background: var(--calc-accent);
		z-index: 50;
		transform-origin: left;
		transform: scaleX(0);
		border-radius: 0 2px 2px 0;
	}

	/* Micro-interaction: button hover ripple */
	:global(.calc-ripple) {
		position: relative;
		overflow: hidden;
	}
	:global(.calc-ripple)::after {
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
	:global(.calc-ripple):hover::after {
		opacity: 1;
	}
</style>

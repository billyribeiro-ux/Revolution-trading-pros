<script lang="ts">
	import { X, Download, Upload, Trash2, FolderOpen } from '@lucide/svelte';
	import gsap from 'gsap';
	import ConfigCard from './ConfigCard.svelte';
	import {
		getSavedConfigs,
		deleteConfig,
		exportConfigs,
		importConfigs,
		clearAllConfigs,
	} from '../../utils/saved-configs.js';
	import type { SavedConfig } from '../../engine/types.js';
	import type { CalculatorState } from '../../state/calculator.svelte.js';

	interface Props {
		calc: CalculatorState;
	}

	let { calc }: Props = $props();

	let panelEl: HTMLDivElement | undefined = $state();
	let configs = $state<SavedConfig[]>([]);
	let searchQuery = $state('');
	let fileInputEl: HTMLInputElement | undefined = $state();

	let filtered = $derived(
		searchQuery.trim()
			? configs.filter(
					(c) =>
						c.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
						c.ticker?.toLowerCase().includes(searchQuery.toLowerCase()) ||
						c.tags?.some((t) => t.toLowerCase().includes(searchQuery.toLowerCase())),
				)
			: configs,
	);

	$effect(() => {
		if (calc.showSavedConfigs) {
			configs = getSavedConfigs();
			searchQuery = '';
			requestAnimationFrame(() => {
				if (panelEl) {
					gsap.fromTo(
						panelEl,
						{ x: 320, opacity: 0 },
						{ x: 0, opacity: 1, duration: 0.3, ease: 'power2.out' },
					);
				}
			});
		}
	});

	function refreshConfigs(): void {
		configs = getSavedConfigs();
	}

	function handleLoad(config: SavedConfig): void {
		calc.spotPrice = config.inputs.spotPrice;
		calc.strikePrice = config.inputs.strikePrice;
		calc.volatility = config.inputs.volatility;
		calc.timeToExpiry = config.inputs.timeToExpiry;
		calc.riskFreeRate = config.inputs.riskFreeRate;
		calc.dividendYield = config.inputs.dividendYield;
		calc.optionType = config.optionType;
		calc.calculatorMode = config.mode;
		if (config.ticker) calc.activeTicker = config.ticker;
		calc.addToast('success', `Loaded "${config.name}"`);
		calc.showSavedConfigs = false;
	}

	function handleDelete(id: string): void {
		deleteConfig(id);
		refreshConfigs();
		calc.addToast('info', 'Configuration deleted');
	}

	function handleExport(): void {
		const json = exportConfigs();
		const blob = new Blob([json], { type: 'application/json' });
		const url = URL.createObjectURL(blob);
		const a = document.createElement('a');
		a.href = url;
		a.download = 'rtp-calculator-configs.json';
		a.click();
		URL.revokeObjectURL(url);
		calc.addToast('success', 'Configs exported!');
	}

	function handleImportClick(): void {
		fileInputEl?.click();
	}

	function handleImportFile(e: Event): void {
		const input = e.target as HTMLInputElement;
		const file = input.files?.[0];
		if (!file) return;
		const reader = new FileReader();
		reader.onload = () => {
			const added = importConfigs(reader.result as string);
			refreshConfigs();
			calc.addToast('success', `Imported ${added} configuration${added !== 1 ? 's' : ''}`);
		};
		reader.readAsText(file);
		input.value = '';
	}

	function handleClearAll(): void {
		clearAllConfigs();
		configs = [];
		calc.addToast('info', 'All configurations cleared');
	}

	function handleClose(): void {
		calc.showSavedConfigs = false;
	}

	function handleKeydown(e: KeyboardEvent): void {
		if (!calc.showSavedConfigs) return;
		if (e.key === 'Escape') handleClose();
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if calc.showSavedConfigs}
	<div
		class="fixed inset-0 z-[9994]"
		role="dialog"
		aria-modal="true"
		aria-label="Saved Configurations"
	>
		<!-- Backdrop -->
		<button
			class="absolute inset-0 cursor-default"
			style="background: rgba(0,0,0,0.4);"
			onclick={handleClose}
			aria-label="Close"
			tabindex={-1}
		></button>

		<!-- Slide-out panel -->
		<div
			bind:this={panelEl}
			class="absolute right-0 top-0 bottom-0 w-full max-w-sm flex flex-col"
			style="
				background: var(--calc-bg);
				border-left: 1px solid var(--calc-border);
				box-shadow: -8px 0 40px rgba(0,0,0,0.3);
			"
		>
			<!-- Header -->
			<div
				class="flex items-center justify-between px-4 py-3"
				style="border-bottom: 1px solid var(--calc-border);"
			>
				<div class="flex items-center gap-2">
					<FolderOpen size={16} style="color: var(--calc-accent);" />
					<h3
						class="text-sm font-semibold"
						style="color: var(--calc-text); font-family: var(--calc-font-display);"
					>Saved Configs</h3>
					<span
						class="text-[9px] px-1.5 py-0.5 rounded"
						style="background: var(--calc-surface); color: var(--calc-text-muted);"
					>{configs.length}</span>
				</div>
				<button
					onclick={handleClose}
					class="cursor-pointer p-1 rounded-md"
					style="color: var(--calc-text-muted);"
					aria-label="Close"
				>
					<X size={16} />
				</button>
			</div>

			<!-- Search -->
			<div class="px-4 py-2">
				<input
					type="text"
					bind:value={searchQuery}
					placeholder="Search configs..."
					class="w-full text-xs px-3 py-2 rounded-lg outline-none"
					style="background: var(--calc-surface); color: var(--calc-text); border: 1px solid var(--calc-border);"
				/>
			</div>

			<!-- Config List -->
			<div class="flex-1 overflow-y-auto px-4 py-2 flex flex-col gap-2 calc-scrollbar">
				{#if filtered.length === 0}
					<div class="flex flex-col items-center justify-center py-12 gap-3">
						<FolderOpen size={32} style="color: var(--calc-text-muted); opacity: 0.3;" />
						<p class="text-xs text-center" style="color: var(--calc-text-muted);">
							{configs.length === 0 ? 'No saved configurations yet.' : 'No matching configs.'}
						</p>
						{#if configs.length === 0}
							<p class="text-[10px] text-center" style="color: var(--calc-text-muted);">
								Press
								<kbd
									class="px-1 py-0.5 rounded text-[9px]"
									style="background: var(--calc-surface); border: 1px solid var(--calc-border);"
								>\u2318S</kbd>
								to save your first one.
							</p>
						{/if}
					</div>
				{:else}
					{#each filtered as config (config.id)}
						<ConfigCard {config} onLoad={handleLoad} onDelete={handleDelete} />
					{/each}
				{/if}
			</div>

			<!-- Footer Actions -->
			<div
				class="flex items-center gap-2 px-4 py-3"
				style="border-top: 1px solid var(--calc-border);"
			>
				<button
					onclick={handleExport}
					class="flex items-center gap-1 text-[10px] px-2 py-1.5 rounded-lg cursor-pointer"
					style="color: var(--calc-text-muted); border: 1px solid var(--calc-border);"
					title="Export all configs as JSON"
				>
					<Download size={10} /> Export
				</button>
				<button
					onclick={handleImportClick}
					class="flex items-center gap-1 text-[10px] px-2 py-1.5 rounded-lg cursor-pointer"
					style="color: var(--calc-text-muted); border: 1px solid var(--calc-border);"
					title="Import configs from JSON"
				>
					<Upload size={10} /> Import
				</button>
				{#if configs.length > 0}
					<button
						onclick={handleClearAll}
						class="flex items-center gap-1 text-[10px] px-2 py-1.5 rounded-lg cursor-pointer ml-auto"
						style="color: var(--calc-put); border: 1px solid var(--calc-border);"
						title="Clear all saved configs"
					>
						<Trash2 size={10} /> Clear All
					</button>
				{/if}
			</div>

			<!-- Hidden file input for import -->
			<input
				bind:this={fileInputEl}
				type="file"
				accept=".json"
				class="hidden"
				onchange={handleImportFile}
			/>
		</div>
	</div>
{/if}

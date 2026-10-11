<script lang="ts">
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconDownload from '@tabler/icons-svelte-runes/icons/download';
	import IconUpload from '@tabler/icons-svelte-runes/icons/upload';
	import IconTrash from '@tabler/icons-svelte-runes/icons/trash';
	import IconFolderOpen from '@tabler/icons-svelte-runes/icons/folder-open';
	import { quadOut } from 'svelte/easing';
	import { prefersReducedMotion } from 'svelte/motion';
	import { fly } from 'svelte/transition';
	import type { Attachment } from 'svelte/attachments';
	import ConfigCard from './ConfigCard.svelte';
	import {
		getSavedConfigs,
		deleteConfig,
		exportConfigs,
		importConfigs,
		clearAllConfigs
	} from '../../utils/saved-configs.js';
	import { downloadBlob } from '../../utils/export-utils.js';
	import { getCalculator } from '../../state/context.js';
	import type { SavedConfig } from '../../engine/types.js';

	const calc = getCalculator();
	const uid = $props.id();

	/** Replaced wholesale after every storage change, never mutated in place. */
	let configs = $state.raw<SavedConfig[]>([]);

	function matchesQuery(config: SavedConfig, query: string): boolean {
		const needle = query.trim().toLowerCase();
		if (!needle) return true;
		return (
			config.name.toLowerCase().includes(needle) ||
			(config.ticker?.toLowerCase().includes(needle) ?? false) ||
			(config.tags?.some((tag) => tag.toLowerCase().includes(needle)) ?? false)
		);
	}

	/**
	 * Runs each time the panel opens: reads storage (I/O, so after mount and never during
	 * render), moves focus into the panel, and hands focus back to the opener on close.
	 */
	const openPanel: Attachment<HTMLElement> = (panel) => {
		configs = getSavedConfigs();
		const opener = document.activeElement;
		panel.focus();
		return () => {
			if (opener instanceof HTMLElement && opener.isConnected) opener.focus();
		};
	};

	/** Storage can be full or blocked (private mode): report a failed write instead of throwing. */
	function tryWrite(write: () => void, failure: string): boolean {
		try {
			write();
			return true;
		} catch {
			calc.addToast('error', failure);
			return false;
		}
	}

	function close(): void {
		calc.showSavedConfigs = false;
	}

	function handleLoad(config: SavedConfig): void {
		// The save dialog stores strategy legs; they come back with the rest of the setup.
		calc.applyShareableState({
			inputs: config.inputs,
			optionType: config.optionType,
			mode: config.mode,
			ticker: config.ticker,
			strategyLegs: config.strategyLegs
		});
		calc.addToast('success', `Loaded "${config.name}"`);
		close();
	}

	function handleDelete(id: string): void {
		if (!tryWrite(() => deleteConfig(id), 'Could not delete the configuration.')) return;
		configs = getSavedConfigs();
		calc.addToast('info', 'Configuration deleted');
	}

	function handleExport(): void {
		downloadBlob(
			new Blob([exportConfigs()], { type: 'application/json' }),
			'rtp-calculator-configs.json'
		);
		calc.addToast('success', 'Configs exported!');
	}

	async function importFile(file: File): Promise<void> {
		let json: string;
		try {
			json = await file.text();
		} catch {
			calc.addToast('error', 'Could not read that file.');
			return;
		}
		const added = importConfigs(json);
		configs = getSavedConfigs();
		calc.addToast('success', `Imported ${added} configuration${added === 1 ? '' : 's'}`);
	}

	function handleImportFile(event: Event & { currentTarget: HTMLInputElement }): void {
		const input = event.currentTarget;
		const file = input.files?.[0];
		// Reset so choosing the same file again still fires `change`.
		input.value = '';
		if (file) void importFile(file);
	}

	function handleClearAll(): void {
		if (!tryWrite(clearAllConfigs, 'Could not clear saved configurations.')) return;
		configs = [];
		calc.addToast('info', 'All configurations cleared');
	}

	function handleKeydown(event: KeyboardEvent): void {
		if (calc.showSavedConfigs && event.key === 'Escape') close();
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if calc.showSavedConfigs}
	<!-- The search lives exactly as long as the panel, so every opening starts unfiltered. -->
	{let searchQuery = $state('')}
	{const filtered = $derived(configs.filter((config) => matchesQuery(config, searchQuery)))}

	<div class="fixed inset-0 z-[9994]" role="dialog" aria-modal="true" aria-labelledby="{uid}-title">
		<!-- Backdrop -->
		<button
			type="button"
			class="absolute inset-0 cursor-default"
			style="background: rgba(0,0,0,0.4);"
			onclick={close}
			aria-label="Close"
			tabindex={-1}
		></button>

		<!-- Slide-out panel -->
		<div
			{@attach openPanel}
			in:fly={{ x: 320, duration: prefersReducedMotion.current ? 0 : 300, easing: quadOut }}
			tabindex="-1"
			class="absolute right-0 top-0 bottom-0 w-full max-w-sm flex flex-col outline-none"
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
					<IconFolderOpen size={16} style="color: var(--calc-accent);" aria-hidden="true" />
					<h3
						id="{uid}-title"
						class="text-sm font-semibold"
						style="color: var(--calc-text); font-family: var(--calc-font-display);"
					>
						Saved Configs
					</h3>
					<span
						class="text-[9px] px-1.5 py-0.5 rounded"
						style="background: var(--calc-surface); color: var(--calc-text-muted);"
						>{configs.length}</span
					>
				</div>
				<button
					type="button"
					onclick={close}
					class="cursor-pointer p-1 rounded-md"
					style="color: var(--calc-text-muted);"
					aria-label="Close"
				>
					<IconX size={16} aria-hidden="true" />
				</button>
			</div>

			<!-- Search -->
			<div class="px-4 py-2">
				<input
					type="text"
					bind:value={searchQuery}
					aria-label="Search saved configurations"
					placeholder="Search configs..."
					class="w-full text-xs px-3 py-2 rounded-lg outline-none"
					style="background: var(--calc-surface); color: var(--calc-text); border: 1px solid var(--calc-border);"
				/>
			</div>

			<!-- Config List -->
			<div class="flex-1 overflow-y-auto px-4 py-2 flex flex-col gap-2 calc-scrollbar">
				{#if filtered.length === 0}
					<div class="flex flex-col items-center justify-center py-12 gap-3">
						<IconFolderOpen
							size={32}
							style="color: var(--calc-text-muted); opacity: 0.3;"
							aria-hidden="true"
						/>
						<p class="text-xs text-center" style="color: var(--calc-text-muted);">
							{configs.length === 0 ? 'No saved configurations yet.' : 'No matching configs.'}
						</p>
						{#if configs.length === 0}
							<p class="text-[10px] text-center" style="color: var(--calc-text-muted);">
								Press
								<kbd
									class="px-1 py-0.5 rounded text-[9px]"
									style="background: var(--calc-surface); border: 1px solid var(--calc-border);"
									>⌘S</kbd
								>
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
					type="button"
					onclick={handleExport}
					class="flex items-center gap-1 text-[10px] px-2 py-1.5 rounded-lg cursor-pointer"
					style="color: var(--calc-text-muted); border: 1px solid var(--calc-border);"
					title="Export all configs as JSON"
				>
					<IconDownload size={10} aria-hidden="true" /> Export
				</button>
				<!-- A label opens its file input natively; the input stays focusable (sr-only, not hidden). -->
				<label
					class="flex items-center gap-1 text-[10px] px-2 py-1.5 rounded-lg cursor-pointer has-focus-visible:outline-2 has-focus-visible:outline-offset-2"
					style="color: var(--calc-text-muted); border: 1px solid var(--calc-border);"
					title="Import configs from JSON"
				>
					<input
						type="file"
						accept=".json,application/json"
						class="sr-only"
						onchange={handleImportFile}
					/>
					<IconUpload size={10} aria-hidden="true" /> Import
				</label>
				{#if configs.length > 0}
					<button
						type="button"
						onclick={handleClearAll}
						class="flex items-center gap-1 text-[10px] px-2 py-1.5 rounded-lg cursor-pointer ml-auto"
						style="color: var(--calc-put); border: 1px solid var(--calc-border);"
						title="Clear all saved configs"
					>
						<IconTrash size={10} aria-hidden="true" /> Clear All
					</button>
				{/if}
			</div>
		</div>
	</div>
{/if}

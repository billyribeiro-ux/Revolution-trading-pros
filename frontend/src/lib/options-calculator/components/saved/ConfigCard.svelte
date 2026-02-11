<script lang="ts">
	import { Trash2, Play } from '@lucide/svelte';
	import { formatCurrency } from '../../utils/formatters.js';
	import type { SavedConfig } from '../../engine/types.js';

	interface Props {
		config: SavedConfig;
		onLoad: (config: SavedConfig) => void;
		onDelete: (id: string) => void;
	}

	let { config, onLoad, onDelete }: Props = $props();

	let dateStr = $derived(
		new Date(config.createdAt).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric'
		})
	);

	let typeColor = $derived(
		config.optionType === 'call'
			? { bg: 'var(--calc-call-bg)', text: 'var(--calc-call)' }
			: { bg: 'var(--calc-put-bg)', text: 'var(--calc-put)' }
	);
</script>

<div
	class="rounded-xl p-3 flex flex-col gap-2 transition-colors"
	style="background: var(--calc-surface); border: 1px solid var(--calc-border);"
>
	<!-- Header: name + date -->
	<div class="flex items-center justify-between gap-2">
		<span class="text-xs font-semibold truncate" style="color: var(--calc-text);"
			>{config.name}</span
		>
		<span
			class="text-[9px] px-1.5 py-0.5 rounded flex-shrink-0"
			style="background: var(--calc-surface-hover); color: var(--calc-text-muted);">{dateStr}</span
		>
	</div>

	<!-- Badges: ticker, type, key inputs -->
	<div class="flex items-center gap-1.5 flex-wrap">
		{#if config.ticker}
			<span
				class="text-[9px] font-bold px-1.5 py-0.5 rounded"
				style="background: var(--calc-accent-glow); color: var(--calc-accent);"
				>{config.ticker}</span
			>
		{/if}
		<span
			class="text-[9px] font-medium px-1.5 py-0.5 rounded"
			style="background: {typeColor.bg}; color: {typeColor.text};"
			>{config.optionType.toUpperCase()}</span
		>
		<span
			class="text-[9px]"
			style="color: var(--calc-text-muted); font-family: var(--calc-font-mono);"
		>
			S={formatCurrency(config.inputs.spotPrice)} K={formatCurrency(config.inputs.strikePrice)} \u03c3={(
				config.inputs.volatility * 100
			).toFixed(0)}%
		</span>
	</div>

	<!-- Description (if any) -->
	{#if config.description}
		<p class="text-[10px] truncate" style="color: var(--calc-text-muted);">
			{config.description}
		</p>
	{/if}

	<!-- Tags -->
	{#if config.tags && config.tags.length > 0}
		<div class="flex items-center gap-1 flex-wrap">
			{#each config.tags as tag (tag)}
				<span
					class="text-[8px] px-1 py-0.5 rounded"
					style="background: var(--calc-surface-hover); color: var(--calc-text-muted); border: 1px solid var(--calc-border);"
					>{tag}</span
				>
			{/each}
		</div>
	{/if}

	<!-- Actions -->
	<div class="flex items-center gap-2 mt-1">
		<button
			onclick={() => onLoad(config)}
			class="flex-1 flex items-center justify-center gap-1 text-[10px] font-semibold py-1.5 rounded-lg cursor-pointer transition-all duration-150"
			style="background: var(--calc-accent); color: white;"
		>
			<Play size={10} />
			Load
		</button>
		<button
			onclick={() => onDelete(config.id)}
			class="flex items-center justify-center p-1.5 rounded-lg cursor-pointer transition-colors"
			style="color: var(--calc-text-muted); border: 1px solid var(--calc-border);"
			aria-label="Delete configuration"
		>
			<Trash2 size={11} />
		</button>
	</div>
</div>

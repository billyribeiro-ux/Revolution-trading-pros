<script lang="ts">
	import IconTrash from '@tabler/icons-svelte-runes/icons/trash';
	import IconPlayerPlay from '@tabler/icons-svelte-runes/icons/player-play';
	import { formatCurrency } from '../../utils/formatters.js';
	import type { SavedConfig } from '../../engine/types.js';

	interface Props {
		config: SavedConfig;
		onLoad: (config: SavedConfig) => void;
		onDelete: (id: string) => void;
	}

	let { config, onLoad, onDelete }: Props = $props();

	const uid = $props.id();

	const dateStr = $derived(
		new Date(config.createdAt).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric'
		})
	);

	const isCall = $derived(config.optionType === 'call');

	/** Imported or hand-edited configs may repeat a tag, and each tag keys its badge. */
	const tags = $derived([...new Set(config.tags ?? [])]);
</script>

<div
	class="rounded-xl p-3 flex flex-col gap-2 transition-colors motion-reduce:transition-none"
	style="background: var(--calc-surface); border: 1px solid var(--calc-border);"
>
	<!-- Header: name + date -->
	<div class="flex items-center justify-between gap-2">
		<span id="{uid}-name" class="text-xs font-semibold truncate" style="color: var(--calc-text);"
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
			style:background={isCall ? 'var(--calc-call-bg)' : 'var(--calc-put-bg)'}
			style:color={isCall ? 'var(--calc-call)' : 'var(--calc-put)'}
			>{config.optionType.toUpperCase()}</span
		>
		<span
			class="text-[9px]"
			style="color: var(--calc-text-muted); font-family: var(--calc-font-mono);"
		>
			S={formatCurrency(config.inputs.spotPrice)} K={formatCurrency(config.inputs.strikePrice)} σ={(
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
	{#if tags.length > 0}
		<div class="flex items-center gap-1 flex-wrap">
			{#each tags as tag (tag)}
				<span
					class="text-[8px] px-1 py-0.5 rounded"
					style="background: var(--calc-surface-hover); color: var(--calc-text-muted); border: 1px solid var(--calc-border);"
					>{tag}</span
				>
			{/each}
		</div>
	{/if}

	<!-- Actions (described by the config name, so "Load" says which one) -->
	<div class="flex items-center gap-2 mt-1">
		<button
			type="button"
			onclick={() => onLoad(config)}
			aria-describedby="{uid}-name"
			class="flex-1 flex items-center justify-center gap-1 text-[10px] font-semibold py-1.5 rounded-lg cursor-pointer transition-all motion-reduce:transition-none duration-150"
			style="background: var(--calc-accent); color: white;"
		>
			<IconPlayerPlay size={10} aria-hidden="true" />
			Load
		</button>
		<button
			type="button"
			onclick={() => onDelete(config.id)}
			aria-label="Delete configuration"
			aria-describedby="{uid}-name"
			class="flex items-center justify-center p-1.5 rounded-lg cursor-pointer transition-colors motion-reduce:transition-none"
			style="color: var(--calc-text-muted); border: 1px solid var(--calc-border);"
		>
			<IconTrash size={11} aria-hidden="true" />
		</button>
	</div>
</div>

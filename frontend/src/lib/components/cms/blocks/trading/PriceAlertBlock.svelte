<!--
/**
 * Price Alert Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Trading price alert with target levels and R:R visualization
 */
-->

<script lang="ts">
	import { IconBell, IconTrendingUp, IconTrendingDown, IconTarget } from '$lib/icons';
	import type { Block, BlockContent } from '../types';
	import type { BlockId } from '$lib/stores/blockState.svelte';

	interface Props {
		block: Block;
		blockId: BlockId;
		isSelected: boolean;
		isEditing: boolean;
		onUpdate: (updates: Partial<Block>) => void;
		onError?: (error: Error) => void;
	}

	let props: Props = $props();

	let symbol = $derived(props.block.content.alertSymbol || 'SPY');
	let direction = $derived((props.block.content.alertDirection as 'above' | 'below') || 'above');
	let targetPrice = $derived(props.block.content.alertTarget || 480);
	let entryPrice = $derived(props.block.content.alertEntry || 475);
	let stopLoss = $derived(props.block.content.alertStop || 470);
	let note = $derived(props.block.content.alertNote || '');

	let isBullish = $derived(direction === 'above');
	let riskReward = $derived.by(() => {
		const risk = Math.abs(entryPrice - stopLoss);
		const reward = Math.abs(targetPrice - entryPrice);
		return risk > 0 ? (reward / risk).toFixed(2) : '0.00';
	});

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	function handlePaste(e: ClipboardEvent): void {
		e.preventDefault();
		document.execCommand('insertText', false, e.clipboardData?.getData('text/plain') || '');
	}
</script>

<div
	class="price-alert-block"
	class:bullish={isBullish}
	class:bearish={!isBullish}
	role="article"
	aria-label="Price alert for {symbol}"
>
	<div class="alert-header">
		<div class="alert-icon">
			<IconBell size={20} aria-hidden="true" />
		</div>
		<div class="alert-title">
			{#if props.isEditing}
				<input
					type="text"
					class="symbol-input"
					value={symbol}
					oninput={(e) => updateContent({ alertSymbol: (e.target as HTMLInputElement).value.toUpperCase() })}
					aria-label="Symbol"
				/>
			{:else}
				<span class="symbol">{symbol}</span>
			{/if}
			<span class="alert-type">Price Alert</span>
		</div>
		<div class="direction-badge">
			{#if isBullish}
				<IconTrendingUp size={16} aria-hidden="true" />
				<span>Bullish</span>
			{:else}
				<IconTrendingDown size={16} aria-hidden="true" />
				<span>Bearish</span>
			{/if}
		</div>
	</div>

	<div class="alert-body">
		<div class="price-levels">
			<div class="level target">
				<span class="level-label"><IconTarget size={14} /> Target</span>
				{#if props.isEditing}
					<input type="number" step="0.01" value={targetPrice} oninput={(e) => updateContent({ alertTarget: parseFloat((e.target as HTMLInputElement).value) })} />
				{:else}
					<span class="level-value">${targetPrice.toFixed(2)}</span>
				{/if}
			</div>
			<div class="level entry">
				<span class="level-label">Entry</span>
				{#if props.isEditing}
					<input type="number" step="0.01" value={entryPrice} oninput={(e) => updateContent({ alertEntry: parseFloat((e.target as HTMLInputElement).value) })} />
				{:else}
					<span class="level-value">${entryPrice.toFixed(2)}</span>
				{/if}
			</div>
			<div class="level stop">
				<span class="level-label">Stop Loss</span>
				{#if props.isEditing}
					<input type="number" step="0.01" value={stopLoss} oninput={(e) => updateContent({ alertStop: parseFloat((e.target as HTMLInputElement).value) })} />
				{:else}
					<span class="level-value">${stopLoss.toFixed(2)}</span>
				{/if}
			</div>
		</div>

		<div class="rr-display">
			<span class="rr-label">Risk/Reward</span>
			<span class="rr-value">{riskReward}:1</span>
		</div>

		{#if props.isEditing}
			<div class="direction-selector">
				<label>
					<input type="radio" name="direction-{props.blockId}" value="above" checked={direction === 'above'} onchange={() => updateContent({ alertDirection: 'above' })} />
					<span>Bullish (Above)</span>
				</label>
				<label>
					<input type="radio" name="direction-{props.blockId}" value="below" checked={direction === 'below'} onchange={() => updateContent({ alertDirection: 'below' })} />
					<span>Bearish (Below)</span>
				</label>
			</div>
		{/if}

		{#if note || props.isEditing}
			<div class="alert-note">
				{#if props.isEditing}
					<textarea
						placeholder="Add notes about this alert..."
						value={note}
						oninput={(e) => updateContent({ alertNote: (e.target as HTMLTextAreaElement).value })}
						aria-label="Alert notes"
					></textarea>
				{:else}
					<p>{note}</p>
				{/if}
			</div>
		{/if}
	</div>
</div>

<style>
	.price-alert-block {
		border-radius: 12px;
		overflow: hidden;
		border: 1px solid #e5e7eb;
	}
	.price-alert-block.bullish { background: linear-gradient(135deg, #f0fdf4 0%, #dcfce7 100%); border-color: #86efac; }
	.price-alert-block.bearish { background: linear-gradient(135deg, #fef2f2 0%, #fee2e2 100%); border-color: #fca5a5; }

	.alert-header {
		display: flex; align-items: center; gap: 0.75rem;
		padding: 1rem 1.25rem;
		border-bottom: 1px solid rgba(0,0,0,0.05);
	}
	.alert-icon {
		width: 40px; height: 40px;
		display: flex; align-items: center; justify-content: center;
		border-radius: 10px;
	}
	.bullish .alert-icon { background: #22c55e; color: white; }
	.bearish .alert-icon { background: #ef4444; color: white; }

	.alert-title { flex: 1; }
	.symbol { font-size: 1.25rem; font-weight: 700; color: #0f172a; display: block; }
	.symbol-input {
		font-size: 1.25rem; font-weight: 700; padding: 0.25rem 0.5rem;
		border: 1px solid #d1d5db; border-radius: 4px; width: 100px;
		text-transform: uppercase;
	}
	.alert-type { font-size: 0.8125rem; color: #64748b; }

	.direction-badge {
		display: flex; align-items: center; gap: 0.375rem;
		padding: 0.375rem 0.75rem;
		border-radius: 20px; font-size: 0.8125rem; font-weight: 600;
	}
	.bullish .direction-badge { background: #22c55e; color: white; }
	.bearish .direction-badge { background: #ef4444; color: white; }

	.alert-body { padding: 1.25rem; }
	.price-levels { display: flex; flex-direction: column; gap: 0.75rem; margin-bottom: 1rem; }
	.level {
		display: flex; justify-content: space-between; align-items: center;
		padding: 0.75rem 1rem; border-radius: 8px; background: rgba(255,255,255,0.7);
	}
	.level-label { display: flex; align-items: center; gap: 0.375rem; font-size: 0.875rem; font-weight: 500; color: #475569; }
	.level-value { font-size: 1.125rem; font-weight: 700; color: #0f172a; }
	.level input { width: 100px; padding: 0.375rem; font-size: 1rem; font-weight: 600; text-align: right; border: 1px solid #d1d5db; border-radius: 4px; }
	.level.target .level-label { color: #16a34a; }
	.level.stop .level-label { color: #dc2626; }

	.rr-display {
		display: flex; justify-content: space-between; align-items: center;
		padding: 0.75rem 1rem; border-radius: 8px;
		background: rgba(0,0,0,0.05); margin-bottom: 1rem;
	}
	.rr-label { font-size: 0.875rem; font-weight: 500; color: #475569; }
	.rr-value { font-size: 1.25rem; font-weight: 700; }
	.bullish .rr-value { color: #16a34a; }
	.bearish .rr-value { color: #dc2626; }

	.direction-selector {
		display: flex; gap: 1rem; margin-bottom: 1rem;
	}
	.direction-selector label {
		display: flex; align-items: center; gap: 0.375rem;
		font-size: 0.875rem; cursor: pointer;
	}

	.alert-note { margin-top: 1rem; }
	.alert-note textarea {
		width: 100%; min-height: 80px; padding: 0.75rem;
		border: 1px solid #d1d5db; border-radius: 8px;
		font-size: 0.875rem; resize: vertical;
	}
	.alert-note p { margin: 0; font-size: 0.875rem; color: #475569; line-height: 1.6; }

	:global(.dark) .price-alert-block.bullish { background: linear-gradient(135deg, #052e16 0%, #14532d 100%); border-color: #166534; }
	:global(.dark) .price-alert-block.bearish { background: linear-gradient(135deg, #450a0a 0%, #7f1d1d 100%); border-color: #991b1b; }
	:global(.dark) .symbol { color: #f8fafc; }
	:global(.dark) .alert-type { color: #94a3b8; }
	:global(.dark) .level { background: rgba(0,0,0,0.3); }
	:global(.dark) .level-label { color: #94a3b8; }
	:global(.dark) .level-value { color: #f1f5f9; }
	:global(.dark) .rr-display { background: rgba(0,0,0,0.3); }
	:global(.dark) .rr-label { color: #94a3b8; }
	:global(.dark) .alert-note p { color: #94a3b8; }
	:global(.dark) .symbol-input, :global(.dark) .level input, :global(.dark) .alert-note textarea {
		background: #0f172a; border-color: #475569; color: #e2e8f0;
	}
</style>

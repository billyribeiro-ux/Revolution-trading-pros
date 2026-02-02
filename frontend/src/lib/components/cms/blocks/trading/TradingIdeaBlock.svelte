<!--
/**
 * Trading Idea Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Complete trade setup with entry, stops, targets, and analysis
 */
-->

<script lang="ts">
	import { IconChartCandle, IconTrendingUp, IconTrendingDown, IconTarget, IconShieldCheck } from '$lib/icons';
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

	const props: Props = $props();

	let symbol = $derived(props.block.content.tradeSymbol || 'AAPL');
	let direction = $derived((props.block.content.tradeDirection as 'long' | 'short') || 'long');
	let entry = $derived(props.block.content.tradeEntry || 185.00);
	let stopLoss = $derived(props.block.content.tradeStop || 180.00);
	let target1 = $derived(props.block.content.tradeTarget1 || 195.00);
	let target2 = $derived(props.block.content.tradeTarget2 || 205.00);
	let confidence = $derived(props.block.content.tradeConfidence || 75);
	let thesis = $derived(props.block.content.tradeThesis || '');
	let timeframe = $derived(props.block.content.tradeTimeframe || 'Swing (1-2 weeks)');

	let isLong = $derived(direction === 'long');
	let risk = $derived(Math.abs(entry - stopLoss));
	let reward1 = $derived(Math.abs(target1 - entry));
	let reward2 = $derived(Math.abs(target2 - entry));
	let rr1 = $derived(risk > 0 ? (reward1 / risk).toFixed(1) : '0');
	let rr2 = $derived(risk > 0 ? (reward2 / risk).toFixed(1) : '0');

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}
</script>

<div class="trade-idea-block" class:long={isLong} class:short={!isLong} role="article" aria-label="Trade idea for {symbol}">
	<div class="trade-header">
		<div class="trade-icon">
			<IconChartCandle size={24} aria-hidden="true" />
		</div>
		<div class="trade-info">
			{#if props.isEditing}
				<input type="text" class="symbol-input" value={symbol} oninput={(e) => updateContent({ tradeSymbol: (e.target as HTMLInputElement).value.toUpperCase() })} />
			{:else}
				<span class="trade-symbol">{symbol}</span>
			{/if}
			<span class="trade-label">Trade Idea</span>
		</div>
		<div class="direction-badge">
			{#if isLong}
				<IconTrendingUp size={18} aria-hidden="true" />
				<span>LONG</span>
			{:else}
				<IconTrendingDown size={18} aria-hidden="true" />
				<span>SHORT</span>
			{/if}
		</div>
	</div>

	<div class="trade-body">
		<div class="levels-section">
			<div class="level-row entry-level">
				<span class="level-label">Entry</span>
				{#if props.isEditing}
					<input type="number" step="0.01" value={entry} oninput={(e) => updateContent({ tradeEntry: parseFloat((e.target as HTMLInputElement).value) })} />
				{:else}
					<span class="level-value">${entry.toFixed(2)}</span>
				{/if}
			</div>
			<div class="level-row stop-level">
				<span class="level-label"><IconShieldCheck size={14} /> Stop Loss</span>
				{#if props.isEditing}
					<input type="number" step="0.01" value={stopLoss} oninput={(e) => updateContent({ tradeStop: parseFloat((e.target as HTMLInputElement).value) })} />
				{:else}
					<span class="level-value">${stopLoss.toFixed(2)}</span>
				{/if}
				<span class="level-risk">Risk: ${risk.toFixed(2)}</span>
			</div>
			<div class="level-row target-level">
				<span class="level-label"><IconTarget size={14} /> Target 1</span>
				{#if props.isEditing}
					<input type="number" step="0.01" value={target1} oninput={(e) => updateContent({ tradeTarget1: parseFloat((e.target as HTMLInputElement).value) })} />
				{:else}
					<span class="level-value">${target1.toFixed(2)}</span>
				{/if}
				<span class="level-rr">{rr1}:1 R:R</span>
			</div>
			<div class="level-row target-level">
				<span class="level-label"><IconTarget size={14} /> Target 2</span>
				{#if props.isEditing}
					<input type="number" step="0.01" value={target2} oninput={(e) => updateContent({ tradeTarget2: parseFloat((e.target as HTMLInputElement).value) })} />
				{:else}
					<span class="level-value">${target2.toFixed(2)}</span>
				{/if}
				<span class="level-rr">{rr2}:1 R:R</span>
			</div>
		</div>

		<div class="meta-section">
			<div class="confidence-meter">
				<span class="meta-label">Confidence</span>
				{#if props.isEditing}
					<input type="range" min="0" max="100" value={confidence} oninput={(e) => updateContent({ tradeConfidence: parseInt((e.target as HTMLInputElement).value) })} />
				{/if}
				<div class="confidence-bar">
					<div class="confidence-fill" style="width: {confidence}%"></div>
				</div>
				<span class="confidence-value">{confidence}%</span>
			</div>
			<div class="timeframe">
				<span class="meta-label">Timeframe</span>
				{#if props.isEditing}
					<select value={timeframe} onchange={(e) => updateContent({ tradeTimeframe: (e.target as HTMLSelectElement).value })}>
						<option>Scalp (minutes)</option>
						<option>Day Trade</option>
						<option>Swing (1-2 weeks)</option>
						<option>Position (weeks-months)</option>
					</select>
				{:else}
					<span class="timeframe-value">{timeframe}</span>
				{/if}
			</div>
		</div>

		{#if props.isEditing}
			<div class="direction-toggle">
				<label><input type="radio" name="dir-{props.blockId}" value="long" checked={isLong} onchange={() => updateContent({ tradeDirection: 'long' })} /> Long</label>
				<label><input type="radio" name="dir-{props.blockId}" value="short" checked={!isLong} onchange={() => updateContent({ tradeDirection: 'short' })} /> Short</label>
			</div>
		{/if}

		<div class="thesis-section">
			<span class="meta-label">Trade Thesis</span>
			{#if props.isEditing}
				<textarea placeholder="Explain your trade reasoning..." value={thesis} oninput={(e) => updateContent({ tradeThesis: (e.target as HTMLTextAreaElement).value })}></textarea>
			{:else if thesis}
				<p class="thesis-text">{thesis}</p>
			{/if}
		</div>

		<div class="chart-placeholder">
			<IconChartCandle size={48} aria-hidden="true" />
			<span>Chart View</span>
		</div>
	</div>
</div>

<style>
	.trade-idea-block { border-radius: 16px; overflow: hidden; border: 1px solid #e5e7eb; }
	.trade-idea-block.long { background: linear-gradient(180deg, #f0fdf4 0%, #ffffff 30%); }
	.trade-idea-block.short { background: linear-gradient(180deg, #fef2f2 0%, #ffffff 30%); }

	.trade-header { display: flex; align-items: center; gap: 1rem; padding: 1.25rem; border-bottom: 1px solid rgba(0,0,0,0.05); }
	.trade-icon { width: 48px; height: 48px; display: flex; align-items: center; justify-content: center; border-radius: 12px; }
	.long .trade-icon { background: linear-gradient(135deg, #22c55e, #16a34a); color: white; }
	.short .trade-icon { background: linear-gradient(135deg, #ef4444, #dc2626); color: white; }

	.trade-info { flex: 1; }
	.trade-symbol { display: block; font-size: 1.5rem; font-weight: 800; color: #0f172a; }
	.symbol-input { font-size: 1.5rem; font-weight: 800; padding: 0.25rem 0.5rem; border: 1px solid #d1d5db; border-radius: 6px; width: 120px; text-transform: uppercase; }
	.trade-label { font-size: 0.8125rem; color: #64748b; }

	.direction-badge { display: flex; align-items: center; gap: 0.5rem; padding: 0.5rem 1rem; border-radius: 24px; font-weight: 700; font-size: 0.875rem; }
	.long .direction-badge { background: #22c55e; color: white; }
	.short .direction-badge { background: #ef4444; color: white; }

	.trade-body { padding: 1.25rem; }
	.levels-section { display: flex; flex-direction: column; gap: 0.75rem; margin-bottom: 1.5rem; }
	.level-row { display: flex; align-items: center; gap: 1rem; padding: 0.875rem 1rem; border-radius: 10px; background: #f8fafc; }
	.level-label { display: flex; align-items: center; gap: 0.375rem; font-size: 0.875rem; font-weight: 500; color: #475569; min-width: 100px; }
	.level-value { font-size: 1.125rem; font-weight: 700; color: #0f172a; flex: 1; }
	.level-row input { width: 100px; padding: 0.375rem; font-size: 1rem; font-weight: 600; border: 1px solid #d1d5db; border-radius: 4px; }
	.level-risk, .level-rr { font-size: 0.75rem; font-weight: 600; padding: 0.25rem 0.5rem; border-radius: 4px; }
	.level-risk { background: #fef2f2; color: #dc2626; }
	.level-rr { background: #f0fdf4; color: #16a34a; }
	.stop-level { border-left: 3px solid #ef4444; }
	.target-level { border-left: 3px solid #22c55e; }
	.entry-level { border-left: 3px solid #3b82f6; }

	.meta-section { display: flex; gap: 1.5rem; flex-wrap: wrap; margin-bottom: 1.5rem; }
	.meta-label { display: block; font-size: 0.75rem; font-weight: 600; color: #64748b; text-transform: uppercase; margin-bottom: 0.5rem; }
	.confidence-meter { flex: 1; min-width: 200px; }
	.confidence-meter input[type="range"] { width: 100%; margin-bottom: 0.5rem; }
	.confidence-bar { height: 8px; background: #e5e7eb; border-radius: 4px; overflow: hidden; }
	.confidence-fill { height: 100%; border-radius: 4px; transition: width 0.3s; }
	.long .confidence-fill { background: linear-gradient(90deg, #22c55e, #16a34a); }
	.short .confidence-fill { background: linear-gradient(90deg, #ef4444, #dc2626); }
	.confidence-value { font-size: 1.25rem; font-weight: 700; color: #0f172a; }
	.timeframe select { padding: 0.5rem; border: 1px solid #d1d5db; border-radius: 6px; font-size: 0.875rem; }
	.timeframe-value { font-size: 0.9375rem; font-weight: 500; color: #0f172a; }

	.direction-toggle { display: flex; gap: 1rem; margin-bottom: 1rem; }
	.direction-toggle label { display: flex; align-items: center; gap: 0.375rem; font-size: 0.875rem; cursor: pointer; }

	.thesis-section { margin-bottom: 1.5rem; }
	.thesis-section textarea { width: 100%; min-height: 100px; padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 8px; font-size: 0.875rem; resize: vertical; }
	.thesis-text { margin: 0; font-size: 0.9375rem; color: #374151; line-height: 1.7; }

	.chart-placeholder { display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 0.5rem; padding: 3rem; background: #f1f5f9; border-radius: 12px; color: #94a3b8; }

	:global(.dark) .trade-idea-block.long { background: linear-gradient(180deg, #052e16 0%, #0f172a 30%); border-color: #166534; }
	:global(.dark) .trade-idea-block.short { background: linear-gradient(180deg, #450a0a 0%, #0f172a 30%); border-color: #991b1b; }
	:global(.dark) .trade-symbol { color: #f8fafc; }
	:global(.dark) .trade-label { color: #94a3b8; }
	:global(.dark) .level-row { background: #1e293b; }
	:global(.dark) .level-label { color: #94a3b8; }
	:global(.dark) .level-value { color: #f1f5f9; }
	:global(.dark) .confidence-value { color: #f1f5f9; }
	:global(.dark) .confidence-bar { background: #334155; }
	:global(.dark) .timeframe-value { color: #f1f5f9; }
	:global(.dark) .thesis-text { color: #e2e8f0; }
	:global(.dark) .chart-placeholder { background: #1e293b; color: #64748b; }
	:global(.dark) .symbol-input, :global(.dark) .level-row input, :global(.dark) .thesis-section textarea, :global(.dark) .timeframe select { background: #0f172a; border-color: #475569; color: #e2e8f0; }
</style>

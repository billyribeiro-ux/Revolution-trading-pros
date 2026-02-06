<!--
/**
 * Ticker Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Real-time stock tickers display (simulated data)
 */
-->

<script lang="ts">
	import { IconTrendingUp, IconTrendingDown, IconPlus, IconX, IconChartLine } from '$lib/icons';
	import type { Block, BlockContent } from '../types';
	import type { BlockId } from '$lib/stores/blockState.svelte';
	import { onMount } from 'svelte';

	interface Props {
		block: Block;
		blockId: BlockId;
		isSelected: boolean;
		isEditing: boolean;
		onUpdate: (updates: Partial<Block>) => void;
		onError?: (error: Error) => void;
	}

	let props: Props = $props();

	interface TickerItem {
		id: string;
		symbol: string;
		price: number;
		change: number;
		changePercent: number;
	}

	let tickers = $state<TickerItem[]>(
		props.block.content.tickerItems || [
			{ id: 't1', symbol: 'SPY', price: 478.52, change: 2.34, changePercent: 0.49 },
			{ id: 't2', symbol: 'QQQ', price: 412.18, change: -1.23, changePercent: -0.3 },
			{ id: 't3', symbol: 'AAPL', price: 185.92, change: 3.45, changePercent: 1.89 }
		]
	);

	let layout = $derived((props.block.settings.tickerLayout as 'grid' | 'list') || 'grid');

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	function updateTicker(index: number, updates: Partial<TickerItem>): void {
		const newTickers = tickers.map((t, i) => (i === index ? { ...t, ...updates } : t));
		tickers = newTickers;
		updateContent({ tickerItems: newTickers });
	}

	function addTicker(): void {
		const newTickers = [
			...tickers,
			{ id: `t${Date.now()}`, symbol: 'NEW', price: 100, change: 0, changePercent: 0 }
		];
		tickers = newTickers;
		updateContent({ tickerItems: newTickers });
	}

	function removeTicker(index: number): void {
		if (tickers.length > 1) {
			const newTickers = tickers.filter((_, i) => i !== index);
			tickers = newTickers;
			updateContent({ tickerItems: newTickers });
		}
	}

	function simulatePriceUpdate(): void {
		tickers = tickers.map((t) => {
			const changeAmount = (Math.random() - 0.5) * 2;
			const newPrice = Math.max(0.01, t.price + changeAmount);
			const newChange = t.change + changeAmount;
			const newChangePercent = (newChange / (newPrice - newChange)) * 100;
			return { ...t, price: newPrice, change: newChange, changePercent: newChangePercent };
		});
	}

	onMount(() => {
		if (!props.isEditing) {
			const interval = setInterval(simulatePriceUpdate, 3000);
			return () => clearInterval(interval);
		}
		return () => {};
	});

	function formatPrice(price: number): string {
		return price.toFixed(2);
	}

	function formatChange(change: number): string {
		const sign = change >= 0 ? '+' : '';
		return `${sign}${change.toFixed(2)}`;
	}

	function formatPercent(percent: number): string {
		const sign = percent >= 0 ? '+' : '';
		return `${sign}${percent.toFixed(2)}%`;
	}
</script>

<div class="ticker-block" role="region" aria-label="Stock tickers">
	<div class="ticker-container layout-{layout}">
		{#each tickers as ticker, index (ticker.id)}
			{@const isPositive = ticker.change >= 0}
			<div class="ticker-card" class:positive={isPositive} class:negative={!isPositive}>
				{#if props.isEditing}
					<input
						type="text"
						class="ticker-symbol-input"
						value={ticker.symbol}
						oninput={(e) =>
							updateTicker(index, { symbol: (e.target as HTMLInputElement).value.toUpperCase() })}
						aria-label="Ticker symbol"
					/>
					<div class="ticker-edit-fields">
						<input
							type="number"
							step="0.01"
							value={ticker.price}
							oninput={(e) =>
								updateTicker(index, {
									price: parseFloat((e.target as HTMLInputElement).value) || 0
								})}
							aria-label="Price"
						/>
						<input
							type="number"
							step="0.01"
							value={ticker.change}
							oninput={(e) => {
								const change = parseFloat((e.target as HTMLInputElement).value) || 0;
								const percent = (change / ticker.price) * 100;
								updateTicker(index, { change, changePercent: percent });
							}}
							aria-label="Change"
						/>
					</div>
					{#if tickers.length > 1}
						<button
							type="button"
							class="remove-ticker"
							onclick={() => removeTicker(index)}
							aria-label="Remove ticker"
						>
							<IconX size={14} />
						</button>
					{/if}
				{:else}
					<div class="ticker-header">
						<span class="ticker-symbol">{ticker.symbol}</span>
						<span class="ticker-icon" aria-hidden="true">
							{#if isPositive}
								<IconTrendingUp size={16} />
							{:else}
								<IconTrendingDown size={16} />
							{/if}
						</span>
					</div>
					<div class="ticker-price">${formatPrice(ticker.price)}</div>
					<div class="ticker-change">
						<span class="change-value">{formatChange(ticker.change)}</span>
						<span class="change-percent">({formatPercent(ticker.changePercent)})</span>
					</div>
					<div class="ticker-sparkline" aria-hidden="true">
						<IconChartLine size={40} />
					</div>
				{/if}
			</div>
		{/each}

		{#if props.isEditing}
			<button type="button" class="add-ticker" onclick={addTicker} aria-label="Add ticker">
				<IconPlus size={20} />
				<span>Add Ticker</span>
			</button>
		{/if}
	</div>

	{#if props.isEditing && props.isSelected}
		<div class="ticker-settings">
			<label class="setting-field">
				<span>Layout:</span>
				<select
					value={layout}
					onchange={(e) =>
						props.onUpdate({
							settings: {
								...props.block.settings,
								tickerLayout: (e.target as HTMLSelectElement).value as 'grid' | 'list'
							}
						})}
				>
					<option value="grid">Grid</option>
					<option value="list">List</option>
				</select>
			</label>
		</div>
	{/if}
</div>

<style>
	.ticker-block {
		width: 100%;
	}
	.ticker-container {
		display: grid;
		gap: 1rem;
	}
	.ticker-container.layout-grid {
		grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
	}
	.ticker-container.layout-list {
		grid-template-columns: 1fr;
	}

	.ticker-card {
		position: relative;
		padding: 1rem;
		background: linear-gradient(135deg, #f8fafc 0%, #f1f5f9 100%);
		border: 1px solid #e2e8f0;
		border-radius: 12px;
		transition: all 0.2s;
	}
	.ticker-card.positive {
		border-left: 3px solid #22c55e;
	}
	.ticker-card.negative {
		border-left: 3px solid #ef4444;
	}
	.ticker-card:hover {
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
	}

	.ticker-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.5rem;
	}
	.ticker-symbol {
		font-size: 1.125rem;
		font-weight: 700;
		color: #1e293b;
	}
	.ticker-icon {
		display: flex;
	}
	.ticker-card.positive .ticker-icon {
		color: #22c55e;
	}
	.ticker-card.negative .ticker-icon {
		color: #ef4444;
	}

	.ticker-price {
		font-size: 1.5rem;
		font-weight: 600;
		color: #0f172a;
		margin-bottom: 0.25rem;
	}
	.ticker-change {
		display: flex;
		gap: 0.5rem;
		font-size: 0.875rem;
		font-weight: 500;
	}
	.ticker-card.positive .ticker-change {
		color: #16a34a;
	}
	.ticker-card.negative .ticker-change {
		color: #dc2626;
	}

	.ticker-sparkline {
		position: absolute;
		bottom: 0.5rem;
		right: 0.5rem;
		opacity: 0.15;
	}
	.ticker-card.positive .ticker-sparkline {
		color: #22c55e;
	}
	.ticker-card.negative .ticker-sparkline {
		color: #ef4444;
	}

	.ticker-symbol-input {
		width: 100%;
		padding: 0.5rem;
		margin-bottom: 0.5rem;
		font-size: 1rem;
		font-weight: 700;
		text-transform: uppercase;
		border: 1px solid #d1d5db;
		border-radius: 6px;
		background: white;
	}
	.ticker-edit-fields {
		display: flex;
		gap: 0.5rem;
	}
	.ticker-edit-fields input {
		flex: 1;
		padding: 0.375rem 0.5rem;
		font-size: 0.875rem;
		border: 1px solid #d1d5db;
		border-radius: 4px;
	}
	.remove-ticker {
		position: absolute;
		top: 0.5rem;
		right: 0.5rem;
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #fee2e2;
		border: none;
		border-radius: 50%;
		color: #dc2626;
		cursor: pointer;
	}

	.add-ticker {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 2rem;
		min-height: 120px;
		background: transparent;
		border: 2px dashed #d1d5db;
		border-radius: 12px;
		color: #9ca3af;
		cursor: pointer;
		transition: all 0.15s;
	}
	.add-ticker:hover {
		border-color: #3b82f6;
		color: #3b82f6;
	}

	.ticker-settings {
		margin-top: 1rem;
		padding: 1rem;
		background: #f9fafb;
		border-radius: 8px;
	}
	.setting-field {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		font-size: 0.875rem;
	}
	.setting-field select {
		padding: 0.5rem;
		border: 1px solid #d1d5db;
		border-radius: 6px;
	}

	:global(.dark) .ticker-card {
		background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%);
		border-color: #334155;
	}
	:global(.dark) .ticker-symbol {
		color: #f1f5f9;
	}
	:global(.dark) .ticker-price {
		color: #f8fafc;
	}
	:global(.dark) .ticker-symbol-input,
	:global(.dark) .ticker-edit-fields input {
		background: #0f172a;
		border-color: #475569;
		color: #e2e8f0;
	}
	:global(.dark) .ticker-settings {
		background: #1e293b;
	}
	:global(.dark) .setting-field select {
		background: #0f172a;
		border-color: #475569;
		color: #e2e8f0;
	}
</style>

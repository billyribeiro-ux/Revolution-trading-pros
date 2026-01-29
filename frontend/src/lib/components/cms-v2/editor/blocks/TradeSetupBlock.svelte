<!--
	Trade Setup Block - Trading entry with price levels
	═══════════════════════════════════════════════════════════════════════════════

	Trading-specific block for documenting trade setups.

	@version 1.0.0
	@author Revolution Trading Pros
	@since January 2026
-->

<script lang="ts">
	import type { ContentBlock } from '$lib/stores/editor.svelte';
	import IconTrendingUp from '@tabler/icons-svelte/icons/trending-up';
	import IconTrendingDown from '@tabler/icons-svelte/icons/trending-down';

	interface Props {
		block: ContentBlock;
		readonly?: boolean;
		onUpdate?: (data: Record<string, unknown>) => void;
	}

	let { block, readonly = false, onUpdate }: Props = $props();

	let data = $derived(
		block.data as {
			ticker?: string;
			direction?: 'long' | 'short';
			entry?: string;
			target?: string;
			stop?: string;
			notes?: string;
		}
	);

	let isLong = $derived(data.direction !== 'short');

	function handleFieldChange(field: string, value: string) {
		onUpdate?.({ ...data, [field]: value });
	}

	function handleDirectionChange(direction: 'long' | 'short') {
		onUpdate?.({ ...data, direction });
	}
</script>

<div class="trade-setup-block">
	<div class="trade-header">
		<div class="ticker-section">
			{#if readonly}
				<span class="ticker">{data.ticker || 'TICKER'}</span>
			{:else}
				<input
					type="text"
					class="ticker-input"
					value={data.ticker ?? ''}
					oninput={(e) => handleFieldChange('ticker', (e.target as HTMLInputElement).value)}
					placeholder="TICKER"
				/>
			{/if}
		</div>

		<div class="direction-section">
			{#if readonly}
				<span class="direction" class:long={isLong} class:short={!isLong}>
					{#if isLong}
						<IconTrendingUp size={16} />
						LONG
					{:else}
						<IconTrendingDown size={16} />
						SHORT
					{/if}
				</span>
			{:else}
				<button
					type="button"
					class="direction-btn long"
					class:active={isLong}
					onclick={() => handleDirectionChange('long')}
				>
					<IconTrendingUp size={14} />
					Long
				</button>
				<button
					type="button"
					class="direction-btn short"
					class:active={!isLong}
					onclick={() => handleDirectionChange('short')}
				>
					<IconTrendingDown size={14} />
					Short
				</button>
			{/if}
		</div>
	</div>

	<div class="trade-levels">
		<div class="level-row">
			<span class="level-label">Entry</span>
			{#if readonly}
				<span class="level-value">${data.entry || '—'}</span>
			{:else}
				<input
					type="text"
					class="level-input"
					value={data.entry ?? ''}
					oninput={(e) => handleFieldChange('entry', (e.target as HTMLInputElement).value)}
					placeholder="0.00"
				/>
			{/if}
		</div>

		<div class="level-row target">
			<span class="level-label">Target</span>
			{#if readonly}
				<span class="level-value profit">${data.target || '—'}</span>
			{:else}
				<input
					type="text"
					class="level-input"
					value={data.target ?? ''}
					oninput={(e) => handleFieldChange('target', (e.target as HTMLInputElement).value)}
					placeholder="0.00"
				/>
			{/if}
		</div>

		<div class="level-row stop">
			<span class="level-label">Stop</span>
			{#if readonly}
				<span class="level-value loss">${data.stop || '—'}</span>
			{:else}
				<input
					type="text"
					class="level-input"
					value={data.stop ?? ''}
					oninput={(e) => handleFieldChange('stop', (e.target as HTMLInputElement).value)}
					placeholder="0.00"
				/>
			{/if}
		</div>
	</div>

	{#if data.notes || !readonly}
		<div class="trade-notes">
			{#if readonly}
				<p>{data.notes}</p>
			{:else}
				<textarea
					class="notes-input"
					value={data.notes ?? ''}
					oninput={(e) => handleFieldChange('notes', (e.target as HTMLTextAreaElement).value)}
					placeholder="Trade notes..."
					rows="2"
				></textarea>
			{/if}
		</div>
	{/if}
</div>

<style>
	.trade-setup-block {
		padding: 1rem;
		background: linear-gradient(135deg, rgba(30, 41, 59, 0.8), rgba(15, 23, 42, 0.8));
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.75rem;
	}

	.trade-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
		padding-bottom: 0.75rem;
		border-bottom: 1px solid rgba(51, 65, 85, 0.5);
	}

	.ticker {
		font-size: 1.25rem;
		font-weight: 700;
		color: #f1f5f9;
		letter-spacing: 0.02em;
	}

	.ticker-input {
		width: 100px;
		padding: 0.375rem 0.5rem;
		background: rgba(0, 0, 0, 0.2);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.375rem;
		color: #f1f5f9;
		font-size: 1.125rem;
		font-weight: 700;
		text-transform: uppercase;
	}

	.ticker-input:focus {
		outline: none;
		border-color: rgba(99, 102, 241, 0.5);
	}

	.direction-section {
		display: flex;
		gap: 0.25rem;
	}

	.direction {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.75rem;
		border-radius: 0.375rem;
		font-size: 0.75rem;
		font-weight: 600;
	}

	.direction.long {
		background: rgba(16, 185, 129, 0.15);
		color: #10b981;
	}

	.direction.short {
		background: rgba(239, 68, 68, 0.15);
		color: #ef4444;
	}

	.direction-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.375rem 0.625rem;
		background: rgba(51, 65, 85, 0.3);
		border: 1px solid transparent;
		border-radius: 0.375rem;
		color: #64748b;
		font-size: 0.75rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s;
	}

	.direction-btn.long:hover,
	.direction-btn.long.active {
		background: rgba(16, 185, 129, 0.15);
		border-color: rgba(16, 185, 129, 0.3);
		color: #10b981;
	}

	.direction-btn.short:hover,
	.direction-btn.short.active {
		background: rgba(239, 68, 68, 0.15);
		border-color: rgba(239, 68, 68, 0.3);
		color: #ef4444;
	}

	.trade-levels {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.level-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.5rem 0.75rem;
		background: rgba(0, 0, 0, 0.15);
		border-radius: 0.375rem;
	}

	.level-label {
		font-size: 0.8125rem;
		font-weight: 500;
		color: #94a3b8;
	}

	.level-value {
		font-size: 0.9375rem;
		font-weight: 600;
		color: #f1f5f9;
		font-family: 'Fira Code', monospace;
	}

	.level-value.profit {
		color: #10b981;
	}

	.level-value.loss {
		color: #ef4444;
	}

	.level-input {
		width: 100px;
		padding: 0.25rem 0.5rem;
		background: rgba(0, 0, 0, 0.2);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.25rem;
		color: #f1f5f9;
		font-size: 0.875rem;
		font-family: 'Fira Code', monospace;
		text-align: right;
	}

	.level-input:focus {
		outline: none;
		border-color: rgba(99, 102, 241, 0.5);
	}

	.trade-notes {
		margin-top: 0.75rem;
		padding-top: 0.75rem;
		border-top: 1px solid rgba(51, 65, 85, 0.5);
	}

	.trade-notes p {
		margin: 0;
		font-size: 0.8125rem;
		color: #94a3b8;
		line-height: 1.5;
	}

	.notes-input {
		width: 100%;
		padding: 0.5rem;
		background: rgba(0, 0, 0, 0.15);
		border: 1px solid rgba(51, 65, 85, 0.3);
		border-radius: 0.375rem;
		color: #94a3b8;
		font-size: 0.8125rem;
		line-height: 1.5;
		resize: vertical;
	}

	.notes-input:focus {
		outline: none;
		border-color: rgba(99, 102, 241, 0.5);
	}
</style>

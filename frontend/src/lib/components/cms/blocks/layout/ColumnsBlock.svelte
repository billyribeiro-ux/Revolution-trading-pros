<!--
/**
 * Columns Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Multi-column layout with presets, gap control, and mobile stacking
 * Supports 1-4 columns with nested block content
 */
-->

<script lang="ts">
	import type { Block, BlockContent, BlockSettings } from '../types';
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

	// Column presets
	type ColumnPreset = '50/50' | '33/33/33' | '66/34' | '34/66' | '25/25/25/25' | 'custom';

	const PRESET_LAYOUTS: Record<ColumnPreset, string[]> = {
		'50/50': ['1fr', '1fr'],
		'33/33/33': ['1fr', '1fr', '1fr'],
		'66/34': ['2fr', '1fr'],
		'34/66': ['1fr', '2fr'],
		'25/25/25/25': ['1fr', '1fr', '1fr', '1fr'],
		custom: []
	};

	const GAP_OPTIONS = [
		{ value: '0', label: 'None' },
		{ value: '0.5rem', label: 'XS' },
		{ value: '1rem', label: 'S' },
		{ value: '1.5rem', label: 'M' },
		{ value: '2rem', label: 'L' },
		{ value: '3rem', label: 'XL' }
	];

	// Derived state
	const columnCount = $derived((props.block.content.columnCount || 2) as number);
	const columnLayout = $derived((props.block.content.columnLayout || '50/50') as ColumnPreset);
	const gap = $derived((props.block.settings.gap || '2rem') as string);
	const children = $derived((props.block.content.children || []) as Block[]);

	// Compute grid template from preset
	const gridTemplate = $derived(() => {
		const preset = PRESET_LAYOUTS[columnLayout];
		if (preset && preset.length > 0) {
			return preset.join(' ');
		}
		// Custom: equal columns
		return Array(columnCount).fill('1fr').join(' ');
	});

	// Generate column array for rendering
	const columns = $derived(() => {
		const count = PRESET_LAYOUTS[columnLayout]?.length || columnCount;
		return Array.from({ length: count }, (_, i) => ({
			index: i,
			content: children[i] || null
		}));
	});

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	function updateSettings(updates: Partial<BlockSettings>): void {
		props.onUpdate({ settings: { ...props.block.settings, ...updates } });
	}

	function setPreset(preset: ColumnPreset): void {
		const count = PRESET_LAYOUTS[preset]?.length || 2;
		updateContent({
			columnLayout: preset,
			columnCount: count
		});
	}

	function setGap(newGap: string): void {
		updateSettings({ gap: newGap });
	}

	function setCustomColumnCount(count: number): void {
		const validCount = Math.min(4, Math.max(1, count));
		updateContent({
			columnLayout: 'custom',
			columnCount: validCount
		});
	}
</script>

<div
	class="columns-block"
	class:editing={props.isEditing}
	class:selected={props.isSelected}
	role="group"
	aria-label="Column layout with {columns().length} columns"
>
	{#if props.isEditing && props.isSelected}
		<div class="columns-toolbar" role="toolbar" aria-label="Column settings">
			<div class="toolbar-group">
				<span class="toolbar-label">Layout:</span>
				<div class="preset-buttons">
					{#each Object.keys(PRESET_LAYOUTS) as preset}
						{#if preset !== 'custom'}
							<button
								type="button"
								class="preset-btn"
								class:active={columnLayout === preset}
								onclick={() => setPreset(preset as ColumnPreset)}
								title="Set layout to {preset}"
								aria-pressed={columnLayout === preset}
							>
								{preset}
							</button>
						{/if}
					{/each}
				</div>
			</div>

			<div class="toolbar-group">
				<label class="toolbar-label" for="custom-columns">Custom:</label>
				<input
					type="number"
					id="custom-columns"
					class="column-count-input"
					min="1"
					max="4"
					value={columnCount}
					oninput={(e) => setCustomColumnCount(parseInt((e.target as HTMLInputElement).value) || 2)}
					aria-label="Number of columns"
				/>
			</div>

			<div class="toolbar-group">
				<label class="toolbar-label" for="gap-select">Gap:</label>
				<select
					id="gap-select"
					class="gap-select"
					value={gap}
					onchange={(e) => setGap((e.target as HTMLSelectElement).value)}
					aria-label="Column gap"
				>
					{#each GAP_OPTIONS as option}
						<option value={option.value}>{option.label}</option>
					{/each}
				</select>
			</div>
		</div>
	{/if}

	<div
		class="columns-container"
		style:grid-template-columns={gridTemplate()}
		style:gap={gap}
	>
		{#each columns() as column (column.index)}
			<div
				class="column"
				class:empty={!column.content}
				role="region"
				aria-label="Column {column.index + 1}"
			>
				<div class="column-content">
					{#if props.isEditing}
						<div class="nested-placeholder">
							<span class="placeholder-text">Column {column.index + 1} content</span>
							<span class="placeholder-hint">Drop blocks here</span>
						</div>
					{:else}
						<!-- In production, use BlockRenderer to render nested blocks -->
						{#if column.content}
							<div class="nested-content">
								<!-- BlockRenderer component would go here -->
							</div>
						{/if}
					{/if}
				</div>
			</div>
		{/each}
	</div>
</div>

<style>
	.columns-block {
		position: relative;
		width: 100%;
	}

	.columns-block.editing {
		padding: 0.5rem;
		border: 2px dashed transparent;
		border-radius: 8px;
		transition: border-color 0.15s;
	}

	.columns-block.editing.selected {
		border-color: #3b82f6;
	}

	.columns-toolbar {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 1rem;
		margin-bottom: 1rem;
		padding: 0.75rem;
		background: #f8fafc;
		border-radius: 8px;
		border: 1px solid #e2e8f0;
	}

	.toolbar-group {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.toolbar-label {
		font-size: 0.8125rem;
		font-weight: 500;
		color: #64748b;
		white-space: nowrap;
	}

	.preset-buttons {
		display: flex;
		gap: 0.25rem;
	}

	.preset-btn {
		padding: 0.375rem 0.625rem;
		background: white;
		border: 1px solid #e2e8f0;
		border-radius: 4px;
		font-size: 0.75rem;
		font-weight: 500;
		color: #475569;
		cursor: pointer;
		transition: all 0.15s;
	}

	.preset-btn:hover {
		background: #f1f5f9;
		border-color: #cbd5e1;
	}

	.preset-btn:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	.preset-btn.active {
		background: #3b82f6;
		border-color: #3b82f6;
		color: white;
	}

	.column-count-input {
		width: 3.5rem;
		padding: 0.375rem 0.5rem;
		border: 1px solid #e2e8f0;
		border-radius: 4px;
		font-size: 0.875rem;
		text-align: center;
	}

	.column-count-input:focus {
		outline: 2px solid #3b82f6;
		outline-offset: -1px;
		border-color: #3b82f6;
	}

	.gap-select {
		padding: 0.375rem 0.625rem;
		border: 1px solid #e2e8f0;
		border-radius: 4px;
		font-size: 0.875rem;
		background: white;
		cursor: pointer;
	}

	.gap-select:focus {
		outline: 2px solid #3b82f6;
		outline-offset: -1px;
		border-color: #3b82f6;
	}

	.columns-container {
		display: grid;
		min-height: 100px;
	}

	/* Mobile stacking */
	@media (max-width: 767px) {
		.columns-container {
			grid-template-columns: 1fr !important;
			gap: 1rem !important;
		}
	}

	.column {
		min-height: 80px;
		border-radius: 6px;
		transition: background-color 0.15s;
	}

	.editing .column {
		background: #f8fafc;
		border: 1px dashed #cbd5e1;
	}

	.editing .column:hover {
		background: #f1f5f9;
		border-color: #94a3b8;
	}

	.column-content {
		height: 100%;
		padding: 1rem;
	}

	.nested-placeholder {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 100%;
		min-height: 60px;
		text-align: center;
		color: #94a3b8;
	}

	.placeholder-text {
		font-size: 0.875rem;
		font-weight: 500;
		color: #64748b;
	}

	.placeholder-hint {
		font-size: 0.75rem;
		margin-top: 0.25rem;
	}

	.nested-content {
		height: 100%;
	}

	/* Dark mode */
	:global(.dark) .columns-toolbar {
		background: #1e293b;
		border-color: #334155;
	}

	:global(.dark) .toolbar-label {
		color: #94a3b8;
	}

	:global(.dark) .preset-btn {
		background: #334155;
		border-color: #475569;
		color: #cbd5e1;
	}

	:global(.dark) .preset-btn:hover {
		background: #475569;
		border-color: #64748b;
	}

	:global(.dark) .preset-btn.active {
		background: #3b82f6;
		border-color: #3b82f6;
		color: white;
	}

	:global(.dark) .column-count-input,
	:global(.dark) .gap-select {
		background: #334155;
		border-color: #475569;
		color: #e2e8f0;
	}

	:global(.dark) .editing .column {
		background: #0f172a;
		border-color: #334155;
	}

	:global(.dark) .editing .column:hover {
		background: #1e293b;
		border-color: #475569;
	}

	:global(.dark) .placeholder-text {
		color: #94a3b8;
	}

	:global(.dark) .nested-placeholder {
		color: #64748b;
	}

	:global(.dark) .columns-block.editing.selected {
		border-color: #3b82f6;
	}
</style>

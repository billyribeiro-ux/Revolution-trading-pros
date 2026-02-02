<!--
/**
 * Divider Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Horizontal separator line with configurable style, width, color, and spacing
 * Accessible with proper ARIA role="separator"
 */
-->

<script lang="ts">
	import type { Block, BlockSettings } from '../types';
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

	// Style options
	type DividerStyle = 'solid' | 'dashed' | 'dotted' | 'double';
	const STYLE_OPTIONS: { value: DividerStyle; label: string }[] = [
		{ value: 'solid', label: 'Solid' },
		{ value: 'dashed', label: 'Dashed' },
		{ value: 'dotted', label: 'Dotted' },
		{ value: 'double', label: 'Double' }
	];

	// Width options
	type DividerWidth = '25%' | '50%' | '75%' | '100%';
	const WIDTH_OPTIONS: { value: DividerWidth; label: string }[] = [
		{ value: '25%', label: '25%' },
		{ value: '50%', label: '50%' },
		{ value: '75%', label: '75%' },
		{ value: '100%', label: '100%' }
	];

	// Spacing options
	type SpacingSize = 'small' | 'medium' | 'large';
	const SPACING_VALUES: Record<SpacingSize, string> = {
		small: '1rem',
		medium: '2rem',
		large: '3rem'
	};

	// Thickness options
	const THICKNESS_OPTIONS = [
		{ value: '1px', label: 'Thin' },
		{ value: '2px', label: 'Medium' },
		{ value: '3px', label: 'Thick' },
		{ value: '4px', label: 'Bold' }
	];

	// Derived state
	const dividerStyle = $derived((props.block.settings.borderStyle || 'solid') as DividerStyle);
	const dividerWidth = $derived((props.block.settings.width || '100%') as DividerWidth);
	const dividerColor = $derived((props.block.settings.borderColor || '#e2e8f0') as string);
	const spacing = $derived((props.block.settings.margin || 'medium') as SpacingSize);
	const thickness = $derived((props.block.settings.borderWidth || '1px') as string);

	// Computed values
	const marginValue = $derived(SPACING_VALUES[spacing] || spacing);

	function updateSettings(updates: Partial<BlockSettings>): void {
		props.onUpdate({ settings: { ...props.block.settings, ...updates } });
	}

	function setStyle(style: DividerStyle): void {
		updateSettings({ borderStyle: style });
	}

	function setWidth(width: DividerWidth): void {
		updateSettings({ width });
	}

	function setColor(color: string): void {
		updateSettings({ borderColor: color });
	}

	function setSpacing(size: SpacingSize): void {
		updateSettings({ margin: size });
	}

	function setThickness(value: string): void {
		updateSettings({ borderWidth: value });
	}
</script>

<div
	class="divider-block"
	class:editing={props.isEditing}
	class:selected={props.isSelected}
>
	{#if props.isEditing && props.isSelected}
		<div class="divider-toolbar" role="toolbar" aria-label="Divider settings">
			<div class="toolbar-row">
				<div class="toolbar-group">
					<span class="toolbar-label">Style:</span>
					<div class="button-group">
						{#each STYLE_OPTIONS as option}
							<button
								type="button"
								class="style-btn"
								class:active={dividerStyle === option.value}
								onclick={() => setStyle(option.value)}
								aria-pressed={dividerStyle === option.value}
								title="{option.label} line"
							>
								<span
									class="style-preview"
									style:border-top-style={option.value}
								></span>
							</button>
						{/each}
					</div>
				</div>

				<div class="toolbar-group">
					<span class="toolbar-label">Width:</span>
					<div class="button-group">
						{#each WIDTH_OPTIONS as option}
							<button
								type="button"
								class="width-btn"
								class:active={dividerWidth === option.value}
								onclick={() => setWidth(option.value)}
								aria-pressed={dividerWidth === option.value}
							>
								{option.label}
							</button>
						{/each}
					</div>
				</div>
			</div>

			<div class="toolbar-row">
				<div class="toolbar-group">
					<label class="toolbar-label" for="divider-color">Color:</label>
					<input
						type="color"
						id="divider-color"
						class="color-picker"
						value={dividerColor}
						oninput={(e) => setColor((e.target as HTMLInputElement).value)}
						aria-label="Divider color"
					/>
					<span class="color-value">{dividerColor}</span>
				</div>

				<div class="toolbar-group">
					<label class="toolbar-label" for="thickness-select">Thickness:</label>
					<select
						id="thickness-select"
						class="select-input"
						value={thickness}
						onchange={(e) => setThickness((e.target as HTMLSelectElement).value)}
						aria-label="Line thickness"
					>
						{#each THICKNESS_OPTIONS as option}
							<option value={option.value}>{option.label}</option>
						{/each}
					</select>
				</div>

				<div class="toolbar-group">
					<span class="toolbar-label">Spacing:</span>
					<div class="button-group">
						{#each ['small', 'medium', 'large'] as size}
							<button
								type="button"
								class="spacing-btn"
								class:active={spacing === size}
								onclick={() => setSpacing(size as SpacingSize)}
								aria-pressed={spacing === size}
							>
								{size.charAt(0).toUpperCase()}
							</button>
						{/each}
					</div>
				</div>
			</div>
		</div>
	{/if}

	<div
		class="divider-wrapper"
		style:margin-top={marginValue}
		style:margin-bottom={marginValue}
	>
		<hr
			class="divider-line"
			style:width={dividerWidth}
			style:border-top-style={dividerStyle}
			style:border-top-color={dividerColor}
			style:border-top-width={dividerStyle === 'double' ? `${parseInt(thickness) * 2}px` : thickness}
			role="separator"
			aria-orientation="horizontal"
		/>
	</div>
</div>

<style>
	.divider-block {
		position: relative;
		width: 100%;
	}

	.divider-block.editing {
		padding: 0.5rem;
		border-radius: 8px;
		transition: background-color 0.15s;
	}

	.divider-block.editing:hover {
		background: #f8fafc;
	}

	.divider-block.editing.selected {
		background: #eff6ff;
		outline: 2px dashed #3b82f6;
		outline-offset: 2px;
	}

	.divider-toolbar {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		margin-bottom: 1rem;
		padding: 0.75rem;
		background: #f8fafc;
		border-radius: 8px;
		border: 1px solid #e2e8f0;
	}

	.toolbar-row {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 1.25rem;
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

	.button-group {
		display: flex;
		gap: 0.25rem;
	}

	.style-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 28px;
		padding: 0 0.5rem;
		background: white;
		border: 1px solid #e2e8f0;
		border-radius: 4px;
		cursor: pointer;
		transition: all 0.15s;
	}

	.style-btn:hover {
		background: #f1f5f9;
		border-color: #cbd5e1;
	}

	.style-btn:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	.style-btn.active {
		background: #3b82f6;
		border-color: #3b82f6;
	}

	.style-btn.active .style-preview {
		border-color: white;
	}

	.style-preview {
		width: 24px;
		height: 0;
		border-top-width: 2px;
		border-top-color: #475569;
	}

	.width-btn,
	.spacing-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		min-width: 36px;
		height: 28px;
		padding: 0 0.5rem;
		background: white;
		border: 1px solid #e2e8f0;
		border-radius: 4px;
		font-size: 0.75rem;
		font-weight: 500;
		color: #475569;
		cursor: pointer;
		transition: all 0.15s;
	}

	.width-btn:hover,
	.spacing-btn:hover {
		background: #f1f5f9;
		border-color: #cbd5e1;
	}

	.width-btn:focus-visible,
	.spacing-btn:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	.width-btn.active,
	.spacing-btn.active {
		background: #3b82f6;
		border-color: #3b82f6;
		color: white;
	}

	.color-picker {
		width: 32px;
		height: 28px;
		padding: 2px;
		border: 1px solid #e2e8f0;
		border-radius: 4px;
		cursor: pointer;
		background: white;
	}

	.color-picker::-webkit-color-swatch-wrapper {
		padding: 0;
	}

	.color-picker::-webkit-color-swatch {
		border: none;
		border-radius: 2px;
	}

	.color-value {
		font-size: 0.75rem;
		font-family: monospace;
		color: #64748b;
		text-transform: uppercase;
	}

	.select-input {
		padding: 0.375rem 0.625rem;
		border: 1px solid #e2e8f0;
		border-radius: 4px;
		font-size: 0.8125rem;
		background: white;
		cursor: pointer;
	}

	.select-input:focus {
		outline: 2px solid #3b82f6;
		outline-offset: -1px;
		border-color: #3b82f6;
	}

	.divider-wrapper {
		display: flex;
		justify-content: center;
	}

	.divider-line {
		margin: 0;
		border: none;
		border-top-style: solid;
	}

	/* Dark mode */
	:global(.dark) .divider-block.editing:hover {
		background: #1e293b;
	}

	:global(.dark) .divider-block.editing.selected {
		background: #1e3a5f;
		outline-color: #3b82f6;
	}

	:global(.dark) .divider-toolbar {
		background: #1e293b;
		border-color: #334155;
	}

	:global(.dark) .toolbar-label {
		color: #94a3b8;
	}

	:global(.dark) .style-btn {
		background: #334155;
		border-color: #475569;
	}

	:global(.dark) .style-btn:hover {
		background: #475569;
		border-color: #64748b;
	}

	:global(.dark) .style-btn.active {
		background: #3b82f6;
		border-color: #3b82f6;
	}

	:global(.dark) .style-preview {
		border-color: #94a3b8;
	}

	:global(.dark) .width-btn,
	:global(.dark) .spacing-btn {
		background: #334155;
		border-color: #475569;
		color: #cbd5e1;
	}

	:global(.dark) .width-btn:hover,
	:global(.dark) .spacing-btn:hover {
		background: #475569;
		border-color: #64748b;
	}

	:global(.dark) .width-btn.active,
	:global(.dark) .spacing-btn.active {
		background: #3b82f6;
		border-color: #3b82f6;
		color: white;
	}

	:global(.dark) .color-picker {
		background: #334155;
		border-color: #475569;
	}

	:global(.dark) .color-value {
		color: #94a3b8;
	}

	:global(.dark) .select-input {
		background: #334155;
		border-color: #475569;
		color: #e2e8f0;
	}
</style>

<!--
/**
 * Group Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Container for grouping nested blocks with styling options
 * Supports background color, padding, border radius, max width, and alignment
 */
-->

<script lang="ts">
	import type { Block, BlockSettings } from '../types';
	import Icon from '$lib/components/Icon.svelte';
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

	// Padding presets
	type PaddingSize = 'none' | 'small' | 'medium' | 'large';
	const PADDING_VALUES: Record<PaddingSize, string> = {
		none: '0',
		small: '1rem',
		medium: '2rem',
		large: '3rem'
	};

	// Max width presets
	type MaxWidthSize = 'full' | 'large' | 'medium' | 'small';
	const MAX_WIDTH_VALUES: Record<MaxWidthSize, string> = {
		full: '100%',
		large: '1200px',
		medium: '960px',
		small: '640px'
	};

	// Border radius presets
	const BORDER_RADIUS_OPTIONS = [
		{ value: '0', label: 'None' },
		{ value: '4px', label: 'S' },
		{ value: '8px', label: 'M' },
		{ value: '12px', label: 'L' },
		{ value: '16px', label: 'XL' },
		{ value: '24px', label: '2XL' }
	];

	// Alignment options
	type Alignment = 'left' | 'center' | 'right';

	// Derived state
	const backgroundColor = $derived((props.block.settings.backgroundColor || '') as string);
	const paddingSize = $derived((props.block.settings.padding || 'medium') as PaddingSize);
	const borderRadius = $derived((props.block.settings.borderRadius || '8px') as string);
	const maxWidthSize = $derived((props.block.settings.maxWidth || 'full') as MaxWidthSize);
	const alignment = $derived((props.block.settings.textAlign || 'left') as Alignment);
	const children = $derived((props.block.content.children || []) as Block[]);

	// Computed styles
	const paddingValue = $derived(PADDING_VALUES[paddingSize] || paddingSize);
	const maxWidthValue = $derived(MAX_WIDTH_VALUES[maxWidthSize] || maxWidthSize);

	// Compute margin for alignment
	const marginStyle = $derived.by(() => {
		switch (alignment) {
			case 'center':
				return '0 auto';
			case 'right':
				return '0 0 0 auto';
			default:
				return '0';
		}
	});

	function updateSettings(updates: Partial<BlockSettings>): void {
		props.onUpdate({ settings: { ...props.block.settings, ...updates } });
	}

	function setBackgroundColor(color: string): void {
		updateSettings({ backgroundColor: color });
	}

	function setPadding(size: PaddingSize): void {
		updateSettings({ padding: size });
	}

	function setBorderRadius(radius: string): void {
		updateSettings({ borderRadius: radius });
	}

	function setMaxWidth(size: MaxWidthSize): void {
		updateSettings({ maxWidth: size });
	}

	function setAlignment(align: Alignment): void {
		updateSettings({ textAlign: align });
	}

	function clearBackground(): void {
		updateSettings({ backgroundColor: '' });
	}
</script>

<div class={['group-block-wrapper', { editing: props.isEditing, selected: props.isSelected }]}>
	{#if props.isEditing && props.isSelected}
		<div class="group-toolbar" role="toolbar" aria-label="Group settings">
			<div class="toolbar-row">
				<div class="toolbar-group">
					<label class="toolbar-label" for="bg-color">Background:</label>
					<div class="color-picker-wrapper">
						<input
							type="color"
							id="bg-color"
							class="color-picker"
							value={backgroundColor || '#ffffff'}
							oninput={(e) => setBackgroundColor((e.target as HTMLInputElement).value)}
							aria-label="Background color"
						/>
						{#if backgroundColor}
							<button
								type="button"
								class="clear-btn"
								onclick={clearBackground}
								title="Clear background"
								aria-label="Clear background color"
							>
								<Icon name="IconX" size={14} />
							</button>
						{/if}
					</div>
				</div>

				<div class="toolbar-group">
					<span class="toolbar-label">Padding:</span>
					<div class="button-group">
						{#each ['none', 'small', 'medium', 'large'] as size (size)}
							<button
								type="button"
								class={['size-btn', { active: paddingSize === size }]}
								onclick={() => setPadding(size as PaddingSize)}
								aria-pressed={paddingSize === size}
							>
								{size.charAt(0).toUpperCase()}
							</button>
						{/each}
					</div>
				</div>

				<div class="toolbar-group">
					<label class="toolbar-label" for="border-radius">Radius:</label>
					<select
						id="border-radius"
						class="select-input"
						value={borderRadius}
						onchange={(e) => setBorderRadius((e.target as HTMLSelectElement).value)}
						aria-label="Border radius"
					>
						{#each BORDER_RADIUS_OPTIONS as option (option.value)}
							<option value={option.value}>{option.label}</option>
						{/each}
					</select>
				</div>
			</div>

			<div class="toolbar-row">
				<div class="toolbar-group">
					<span class="toolbar-label">Max Width:</span>
					<div class="button-group">
						{#each ['full', 'large', 'medium', 'small'] as size (size)}
							<button
								type="button"
								class={['size-btn', { active: maxWidthSize === size }]}
								onclick={() => setMaxWidth(size as MaxWidthSize)}
								aria-pressed={maxWidthSize === size}
							>
								{size.charAt(0).toUpperCase()}
							</button>
						{/each}
					</div>
				</div>

				<div class="toolbar-group">
					<span class="toolbar-label">Align:</span>
					<div class="button-group">
						<button
							type="button"
							class={['align-btn', { active: alignment === 'left' }]}
							onclick={() => setAlignment('left')}
							title="Align left"
							aria-pressed={alignment === 'left'}
							aria-label="Align left"
						>
							<Icon name="IconAlignLeft" size={16} />
						</button>
						<button
							type="button"
							class={['align-btn', { active: alignment === 'center' }]}
							onclick={() => setAlignment('center')}
							title="Align center"
							aria-pressed={alignment === 'center'}
							aria-label="Align center"
						>
							<Icon name="IconAlignCenter" size={16} />
						</button>
						<button
							type="button"
							class={['align-btn', { active: alignment === 'right' }]}
							onclick={() => setAlignment('right')}
							title="Align right"
							aria-pressed={alignment === 'right'}
							aria-label="Align right"
						>
							<Icon name="IconAlignRight" size={16} />
						</button>
					</div>
				</div>
			</div>
		</div>
	{/if}

	<div
		class="group-container"
		style:background-color={backgroundColor || undefined}
		style:padding={paddingValue}
		style:border-radius={borderRadius}
		style:max-width={maxWidthValue}
		style:margin={marginStyle}
		role="group"
		aria-label="Content group"
	>
		<div class="group-content">
			{#if props.isEditing}
				<div class="nested-placeholder">
					{#if children.length === 0}
						<div class="placeholder-icon">
							<Icon name="IconSquareRoundedPlus" size={24} stroke={1.5} />
						</div>
						<span class="placeholder-text">Group content</span>
						<span class="placeholder-hint">Drop blocks here to group them</span>
					{:else}
						<span class="placeholder-text"
							>{children.length} nested block{children.length !== 1 ? 's' : ''}</span
						>
					{/if}
				</div>
			{:else}
				<!-- In production, use BlockRenderer to render nested blocks -->
				{#if children.length > 0}
					<div class="nested-content">
						<!-- BlockRenderer component would go here -->
					</div>
				{/if}
			{/if}
		</div>
	</div>
</div>

<style>
	.group-block-wrapper {
		position: relative;
		width: 100%;
	}

	.group-block-wrapper.editing {
		padding: 0.25rem;
	}

	.group-block-wrapper.editing.selected {
		outline: 2px dashed #3b82f6;
		outline-offset: 4px;
		border-radius: 4px;
	}

	.group-toolbar {
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

	.color-picker-wrapper {
		display: flex;
		align-items: center;
		gap: 0.375rem;
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

	.clear-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		padding: 0;
		background: #fef2f2;
		border: 1px solid #fecaca;
		border-radius: 4px;
		color: #dc2626;
		cursor: pointer;
		transition: all 0.15s;
	}

	.clear-btn:hover {
		background: #fee2e2;
	}

	.button-group {
		display: flex;
		gap: 0.25rem;
	}

	.size-btn,
	.align-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		min-width: 28px;
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

	.size-btn:hover,
	.align-btn:hover {
		background: #f1f5f9;
		border-color: #cbd5e1;
	}

	.size-btn:focus-visible,
	.align-btn:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	.size-btn.active,
	.align-btn.active {
		background: #3b82f6;
		border-color: #3b82f6;
		color: white;
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

	.group-container {
		min-height: 60px;
		transition: all 0.2s ease;
	}

	.editing .group-container {
		border: 1px dashed #cbd5e1;
	}

	.group-content {
		height: 100%;
	}

	.nested-placeholder {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 80px;
		text-align: center;
		color: #94a3b8;
	}

	.placeholder-icon {
		margin-bottom: 0.5rem;
		color: #cbd5e1;
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
	:global(.dark) .group-toolbar {
		background: #1e293b;
		border-color: #334155;
	}

	:global(.dark) .toolbar-label {
		color: #94a3b8;
	}

	:global(.dark) .color-picker {
		background: #334155;
		border-color: #475569;
	}

	:global(.dark) .clear-btn {
		background: #450a0a;
		border-color: #7f1d1d;
		color: #f87171;
	}

	:global(.dark) .clear-btn:hover {
		background: #7f1d1d;
	}

	:global(.dark) .size-btn,
	:global(.dark) .align-btn {
		background: #334155;
		border-color: #475569;
		color: #cbd5e1;
	}

	:global(.dark) .size-btn:hover,
	:global(.dark) .align-btn:hover {
		background: #475569;
		border-color: #64748b;
	}

	:global(.dark) .size-btn.active,
	:global(.dark) .align-btn.active {
		background: #3b82f6;
		border-color: #3b82f6;
		color: white;
	}

	:global(.dark) .select-input {
		background: #334155;
		border-color: #475569;
		color: #e2e8f0;
	}

	:global(.dark) .editing .group-container {
		border-color: #475569;
	}

	:global(.dark) .placeholder-icon {
		color: #475569;
	}

	:global(.dark) .placeholder-text {
		color: #94a3b8;
	}

	:global(.dark) .nested-placeholder {
		color: #64748b;
	}

	:global(.dark) .group-block-wrapper.editing.selected {
		outline-color: #3b82f6;
	}
</style>

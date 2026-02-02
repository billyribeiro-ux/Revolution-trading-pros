<!--
/**
 * Spacer Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Vertical spacing component with adjustable height
 * Shows drag handle in edit mode, invisible in view mode
 * Supports presets and custom heights from 1rem to 10rem
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

	// Height presets
	const HEIGHT_PRESETS = [
		{ value: '1rem', label: 'XS' },
		{ value: '2rem', label: 'S' },
		{ value: '3rem', label: 'M' },
		{ value: '4rem', label: 'L' },
		{ value: '6rem', label: 'XL' },
		{ value: '8rem', label: '2XL' },
		{ value: '10rem', label: '3XL' }
	];

	// Internal state for drag handling
	let isDragging = $state(false);
	let dragStartY = $state(0);
	let dragStartHeight = $state(0);

	// Derived state
	const height = $derived((props.block.settings.height || '2.5rem') as string);

	// Parse height value for slider and display
	const heightInRem = $derived(() => {
		const match = height.match(/^([\d.]+)rem$/);
		return match ? parseFloat(match[1]) : 2.5;
	});

	// Computed pixel height for display
	const heightInPx = $derived(() => {
		return Math.round(heightInRem() * 16);
	});

	function updateSettings(updates: Partial<BlockSettings>): void {
		props.onUpdate({ settings: { ...props.block.settings, ...updates } });
	}

	function setHeight(value: string): void {
		updateSettings({ height: value });
	}

	function setHeightFromRem(rem: number): void {
		const clampedRem = Math.min(10, Math.max(1, rem));
		setHeight(`${clampedRem.toFixed(1)}rem`);
	}

	function handleSliderInput(e: Event): void {
		const value = parseFloat((e.target as HTMLInputElement).value);
		setHeightFromRem(value);
	}

	function handleDragStart(e: MouseEvent): void {
		if (!props.isEditing) return;

		e.preventDefault();
		isDragging = true;
		dragStartY = e.clientY;
		dragStartHeight = heightInRem();

		// Add event listeners to document
		document.addEventListener('mousemove', handleDragMove);
		document.addEventListener('mouseup', handleDragEnd);
	}

	function handleDragMove(e: MouseEvent): void {
		if (!isDragging) return;

		const deltaY = e.clientY - dragStartY;
		const deltaRem = deltaY / 16; // Convert pixels to rem
		const newHeight = dragStartHeight + deltaRem;

		setHeightFromRem(newHeight);
	}

	function handleDragEnd(): void {
		isDragging = false;
		document.removeEventListener('mousemove', handleDragMove);
		document.removeEventListener('mouseup', handleDragEnd);
	}

	// Keyboard support for drag handle
	function handleKeyDown(e: KeyboardEvent): void {
		if (!props.isEditing) return;

		const step = e.shiftKey ? 1 : 0.25;

		switch (e.key) {
			case 'ArrowUp':
				e.preventDefault();
				setHeightFromRem(heightInRem() + step);
				break;
			case 'ArrowDown':
				e.preventDefault();
				setHeightFromRem(heightInRem() - step);
				break;
		}
	}
</script>

{#if props.isEditing}
	<div
		class="spacer-block editing"
		class:selected={props.isSelected}
		class:dragging={isDragging}
	>
		{#if props.isSelected}
			<div class="spacer-toolbar" role="toolbar" aria-label="Spacer settings">
				<div class="toolbar-row">
					<div class="toolbar-group">
						<span class="toolbar-label">Height:</span>
						<div class="preset-buttons">
							{#each HEIGHT_PRESETS as preset}
								<button
									type="button"
									class="preset-btn"
									class:active={height === preset.value}
									onclick={() => setHeight(preset.value)}
									aria-pressed={height === preset.value}
									title="{preset.value}"
								>
									{preset.label}
								</button>
							{/each}
						</div>
					</div>
				</div>

				<div class="toolbar-row">
					<div class="toolbar-group slider-group">
						<label class="toolbar-label" for="height-slider">Custom:</label>
						<input
							type="range"
							id="height-slider"
							class="height-slider"
							min="1"
							max="10"
							step="0.25"
							value={heightInRem()}
							oninput={handleSliderInput}
							aria-label="Spacer height"
							aria-valuemin={1}
							aria-valuemax={10}
							aria-valuenow={heightInRem()}
							aria-valuetext="{heightInRem()} rem"
						/>
						<span class="height-value">{heightInRem().toFixed(1)}rem ({heightInPx()}px)</span>
					</div>
				</div>
			</div>
		{/if}

		<div
			class="spacer-visual"
			style:height={height}
			role="presentation"
		>
			<div class="spacer-outline">
				<span class="spacer-label">{heightInPx()}px</span>
			</div>

			<!-- Drag handle -->
			<div
				class="drag-handle"
				role="slider"
				tabindex="0"
				aria-label="Resize spacer height"
				aria-valuemin={16}
				aria-valuemax={160}
				aria-valuenow={heightInPx()}
				aria-valuetext="{heightInPx()} pixels"
				onmousedown={handleDragStart}
				onkeydown={handleKeyDown}
			>
				<div class="drag-handle-icon">
					<svg width="20" height="8" viewBox="0 0 20 8" fill="currentColor">
						<rect x="0" y="0" width="20" height="2" rx="1" />
						<rect x="0" y="6" width="20" height="2" rx="1" />
					</svg>
				</div>
				<span class="drag-hint">Drag to resize</span>
			</div>
		</div>
	</div>
{:else}
	<!-- View mode: invisible spacer, just creates vertical space -->
	<div
		class="spacer-block"
		style:height={height}
		role="separator"
		aria-label="Vertical spacing"
	></div>
{/if}

<style>
	.spacer-block {
		position: relative;
		width: 100%;
		transition: height 0.2s ease-out;
	}

	.spacer-block.editing {
		padding: 0.5rem;
		border-radius: 8px;
		transition: background-color 0.15s;
	}

	.spacer-block.editing:hover {
		background: #f8fafc;
	}

	.spacer-block.editing.selected {
		background: #eff6ff;
		outline: 2px dashed #3b82f6;
		outline-offset: 2px;
	}

	.spacer-block.editing.dragging {
		background: #dbeafe;
	}

	.spacer-toolbar {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		margin-bottom: 0.75rem;
		padding: 0.75rem;
		background: #f8fafc;
		border-radius: 8px;
		border: 1px solid #e2e8f0;
	}

	.toolbar-row {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 1rem;
	}

	.toolbar-group {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.slider-group {
		flex: 1;
		min-width: 200px;
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
		display: flex;
		align-items: center;
		justify-content: center;
		min-width: 32px;
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

	.height-slider {
		flex: 1;
		min-width: 120px;
		height: 4px;
		background: #e2e8f0;
		border-radius: 2px;
		outline: none;
		cursor: pointer;
		-webkit-appearance: none;
		appearance: none;
	}

	.height-slider::-webkit-slider-thumb {
		-webkit-appearance: none;
		appearance: none;
		width: 16px;
		height: 16px;
		background: #3b82f6;
		border-radius: 50%;
		cursor: grab;
		transition: transform 0.15s;
	}

	.height-slider::-webkit-slider-thumb:hover {
		transform: scale(1.1);
	}

	.height-slider::-webkit-slider-thumb:active {
		cursor: grabbing;
		transform: scale(1.15);
	}

	.height-slider::-moz-range-thumb {
		width: 16px;
		height: 16px;
		background: #3b82f6;
		border: none;
		border-radius: 50%;
		cursor: grab;
	}

	.height-slider:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	.height-value {
		font-size: 0.75rem;
		font-family: monospace;
		color: #64748b;
		min-width: 100px;
	}

	.spacer-visual {
		position: relative;
		display: flex;
		align-items: center;
		justify-content: center;
		min-height: 16px;
	}

	.spacer-outline {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		border: 2px dashed #cbd5e1;
		border-radius: 4px;
		background: repeating-linear-gradient(
			45deg,
			transparent,
			transparent 10px,
			rgba(203, 213, 225, 0.1) 10px,
			rgba(203, 213, 225, 0.1) 20px
		);
	}

	.spacer-label {
		padding: 0.25rem 0.5rem;
		background: white;
		border-radius: 4px;
		font-size: 0.75rem;
		font-weight: 500;
		color: #64748b;
		box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
	}

	.drag-handle {
		position: absolute;
		bottom: -12px;
		left: 50%;
		transform: translateX(-50%);
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.25rem;
		padding: 0.375rem 0.75rem;
		background: #3b82f6;
		border-radius: 4px;
		color: white;
		cursor: ns-resize;
		z-index: 10;
		transition: all 0.15s;
		box-shadow: 0 2px 4px rgba(59, 130, 246, 0.3);
	}

	.drag-handle:hover {
		background: #2563eb;
		transform: translateX(-50%) scale(1.05);
	}

	.drag-handle:focus-visible {
		outline: 2px solid white;
		outline-offset: 2px;
	}

	.dragging .drag-handle {
		background: #1d4ed8;
		transform: translateX(-50%) scale(1.1);
	}

	.drag-handle-icon {
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.drag-hint {
		font-size: 0.625rem;
		text-transform: uppercase;
		letter-spacing: 0.025em;
		opacity: 0.9;
	}

	/* Dark mode */
	:global(.dark) .spacer-block.editing:hover {
		background: #1e293b;
	}

	:global(.dark) .spacer-block.editing.selected {
		background: #1e3a5f;
		outline-color: #3b82f6;
	}

	:global(.dark) .spacer-block.editing.dragging {
		background: #1e3a5f;
	}

	:global(.dark) .spacer-toolbar {
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

	:global(.dark) .height-slider {
		background: #475569;
	}

	:global(.dark) .height-value {
		color: #94a3b8;
	}

	:global(.dark) .spacer-outline {
		border-color: #475569;
		background: repeating-linear-gradient(
			45deg,
			transparent,
			transparent 10px,
			rgba(71, 85, 105, 0.2) 10px,
			rgba(71, 85, 105, 0.2) 20px
		);
	}

	:global(.dark) .spacer-label {
		background: #334155;
		color: #94a3b8;
	}
</style>

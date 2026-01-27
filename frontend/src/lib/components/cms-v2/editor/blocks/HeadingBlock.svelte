<!--
	Heading Block - Standalone heading element
	═══════════════════════════════════════════════════════════════════════════════

	Renders H1-H4 headings with inline editing.

	@version 1.0.0
	@author Revolution Trading Pros
	@since January 2026
-->

<script lang="ts">
	import { browser } from '$app/environment';
	import type { ContentBlock } from '$lib/stores/editor.svelte';

	// ==========================================================================
	// Props
	// ==========================================================================

	interface Props {
		block: ContentBlock;
		readonly?: boolean;
		onUpdate?: (data: Record<string, unknown>) => void;
	}

	let { block, readonly = false, onUpdate }: Props = $props();

	// ==========================================================================
	// State
	// ==========================================================================

	let inputRef = $state<HTMLElement | null>(null);

	// ==========================================================================
	// Derived
	// ==========================================================================

	let level = $derived((block.data as { level?: number })?.level ?? 2);
	let text = $derived((block.data as { text?: string })?.text ?? '');

	// ==========================================================================
	// Handlers
	// ==========================================================================

	function handleInput(e: Event) {
		const target = e.target as HTMLElement;
		onUpdate?.({ level, text: target.textContent ?? '' });
	}

	function handleKeyDown(e: KeyboardEvent) {
		if (e.key === 'Enter' && !e.shiftKey) {
			e.preventDefault();
			// Blur to exit editing
			(e.target as HTMLElement).blur();
		}
	}

	function handleLevelChange(newLevel: number) {
		onUpdate?.({ level: newLevel, text });
	}

	// Initialize content
	$effect(() => {
		if (browser && inputRef && text !== inputRef.textContent) {
			inputRef.textContent = text;
		}
	});
</script>

<div class="heading-block">
	{#if !readonly}
		<div class="level-selector">
			{#each [1, 2, 3, 4] as lvl}
				<button
					type="button"
					class="level-btn"
					class:active={level === lvl}
					onclick={() => handleLevelChange(lvl)}
					aria-label="Heading level {lvl}"
				>
					H{lvl}
				</button>
			{/each}
		</div>
	{/if}

	<div
		bind:this={inputRef}
		class="heading-input heading-{level}"
		contenteditable={!readonly}
		tabindex={readonly ? -1 : 0}
		oninput={handleInput}
		onkeydown={handleKeyDown}
		data-placeholder="Heading {level}"
		role="textbox"
		aria-label="Heading text"
	></div>
</div>

<style>
	.heading-block {
		position: relative;
	}

	.level-selector {
		display: flex;
		gap: 0.25rem;
		margin-bottom: 0.5rem;
	}

	.level-btn {
		padding: 0.25rem 0.5rem;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid transparent;
		border-radius: 0.25rem;
		color: #64748b;
		font-size: 0.6875rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.15s;
	}

	.level-btn:hover {
		background: rgba(99, 102, 241, 0.2);
		color: #94a3b8;
	}

	.level-btn.active {
		background: rgba(99, 102, 241, 0.2);
		border-color: rgba(99, 102, 241, 0.3);
		color: #818cf8;
	}

	.heading-input {
		width: 100%;
		outline: none;
		color: #f1f5f9;
		font-weight: 700;
		line-height: 1.3;
	}

	.heading-input:empty::before {
		content: attr(data-placeholder);
		color: #475569;
		pointer-events: none;
	}

	.heading-1 {
		font-size: 2.25rem;
	}

	.heading-2 {
		font-size: 1.75rem;
	}

	.heading-3 {
		font-size: 1.375rem;
	}

	.heading-4 {
		font-size: 1.125rem;
	}
</style>

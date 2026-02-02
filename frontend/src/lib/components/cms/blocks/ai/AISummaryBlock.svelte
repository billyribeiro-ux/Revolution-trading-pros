<!--
/**
 * AI Summary Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Collapsible TL;DR summary with length options
 */
-->

<script lang="ts">
	import { IconSparkles, IconChevronDown, IconRefresh, IconLoader2 } from '$lib/icons';
	import { getBlockStateManager, type BlockId } from '$lib/stores/blockState.svelte';
	import type { Block, BlockContent } from '../types';

	interface Props {
		block: Block;
		blockId: BlockId;
		isSelected: boolean;
		isEditing: boolean;
		onUpdate: (updates: Partial<Block>) => void;
		onError?: (error: Error) => void;
	}

	const props: Props = $props();
	const stateManager = getBlockStateManager();

	let aiState = $derived(stateManager.getAISummaryState(props.blockId));
	let isExpanded = $state(true);

	let sourceContent = $derived(props.block.content.summarySource || '');
	let summaryLength = $derived((props.block.content.summaryLength as 'short' | 'medium' | 'long') || 'medium');
	let summaryOutput = $derived(props.block.content.summaryOutput || aiState.output || '');

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	async function generateSummary(): Promise<void> {
		if (!sourceContent.trim()) {
			props.onError?.(new Error('Please provide content to summarize'));
			return;
		}

		stateManager.setAISummaryState(props.blockId, { loading: true, error: null });

		try {
			await new Promise(resolve => setTimeout(resolve, 1500));
			
			const lengths = { short: 50, medium: 100, long: 200 };
			const targetLength = lengths[summaryLength];
			const words = sourceContent.split(/\s+/).slice(0, targetLength).join(' ');
			const simulatedSummary = `TL;DR: ${words}${sourceContent.split(/\s+/).length > targetLength ? '...' : ''}`;
			
			stateManager.setAISummaryState(props.blockId, { loading: false, output: simulatedSummary, lastGenerated: Date.now() });
			updateContent({ summaryOutput: simulatedSummary });
		} catch (error) {
			const errorMessage = error instanceof Error ? error.message : 'Failed to generate summary';
			stateManager.setAISummaryState(props.blockId, { loading: false, error: errorMessage });
			props.onError?.(new Error(errorMessage));
		}
	}

	function handlePaste(e: ClipboardEvent): void {
		e.preventDefault();
		document.execCommand('insertText', false, e.clipboardData?.getData('text/plain') || '');
	}
</script>

<div class="ai-summary-block" role="region" aria-label="AI summary">
	<button
		type="button"
		class="summary-header"
		onclick={() => isExpanded = !isExpanded}
		aria-expanded={isExpanded}
	>
		<div class="header-left">
			<IconSparkles size={18} aria-hidden="true" />
			<span class="header-title">TL;DR</span>
		</div>
		<span class="chevron" class:rotated={isExpanded}><IconChevronDown size={18} aria-hidden="true" /></span>
	</button>

	{#if isExpanded}
		<div class="summary-content">
			{#if props.isEditing}
				<div class="source-section">
					<label>
						<span class="label-text">Content to Summarize</span>
						<textarea
							placeholder="Paste or type the content you want to summarize..."
							value={sourceContent}
							oninput={(e) => updateContent({ summarySource: (e.target as HTMLTextAreaElement).value })}
							rows="4"
						></textarea>
					</label>

					<div class="summary-controls">
						<label class="length-select">
							<span>Length:</span>
							<select value={summaryLength} onchange={(e) => updateContent({ summaryLength: (e.target as HTMLSelectElement).value as 'short' | 'medium' | 'long' })}>
								<option value="short">Short (~1 sentence)</option>
								<option value="medium">Medium (~2-3 sentences)</option>
								<option value="long">Long (~paragraph)</option>
							</select>
						</label>

						<button
							type="button"
							class="generate-btn"
							onclick={generateSummary}
							disabled={aiState.loading || !sourceContent.trim()}
						>
							{#if aiState.loading}
								<IconLoader2 size={16} class="spinning" aria-hidden="true" />
								<span>Summarizing...</span>
							{:else}
								<IconSparkles size={16} aria-hidden="true" />
								<span>Summarize</span>
							{/if}
						</button>
					</div>
				</div>
			{/if}

			{#if aiState.error}
				<div class="summary-error" role="alert">{aiState.error}</div>
			{/if}

			{#if summaryOutput}
				<div class="output-section">
					{#if props.isEditing}
						<div class="output-header">
							<span>Summary</span>
							<button type="button" class="regen-btn" onclick={generateSummary} disabled={aiState.loading}>
								<IconRefresh size={14} aria-hidden="true" />
							</button>
						</div>
					{/if}
					<div
						class="summary-text"
						contenteditable={props.isEditing}
						oninput={(e) => updateContent({ summaryOutput: (e.target as HTMLElement).textContent || '' })}
						onpaste={handlePaste}
					>
						{summaryOutput}
					</div>
				</div>
			{:else if !props.isEditing}
				<div class="no-summary">No summary available</div>
			{/if}
		</div>
	{/if}
</div>

<style>
	.ai-summary-block { border: 1px solid #e9d5ff; border-radius: 12px; overflow: hidden; background: linear-gradient(135deg, #faf5ff 0%, #f3e8ff 100%); }

	.summary-header {
		display: flex; justify-content: space-between; align-items: center;
		width: 100%; padding: 0.875rem 1rem;
		background: transparent; border: none;
		cursor: pointer; transition: background 0.15s;
	}
	.summary-header:hover { background: rgba(139, 92, 246, 0.05); }
	.header-left { display: flex; align-items: center; gap: 0.5rem; color: #7c3aed; }
	.header-title { font-weight: 600; font-size: 0.9375rem; }
	.chevron { color: #7c3aed; transition: transform 0.2s; }
	.chevron.rotated { transform: rotate(180deg); }

	.summary-content { padding: 0 1rem 1rem; animation: fadeIn 0.2s; }
	@keyframes fadeIn { from { opacity: 0; } to { opacity: 1; } }

	.source-section { margin-bottom: 1rem; }
	.label-text { display: block; font-size: 0.75rem; font-weight: 600; color: #6b21a8; text-transform: uppercase; margin-bottom: 0.5rem; }
	.source-section textarea {
		width: 100%; padding: 0.75rem;
		border: 1px solid #d8b4fe; border-radius: 8px;
		font-size: 0.875rem; resize: vertical; background: white;
	}
	.source-section textarea:focus { outline: none; border-color: #8b5cf6; }

	.summary-controls { display: flex; justify-content: space-between; align-items: center; gap: 1rem; margin-top: 0.75rem; flex-wrap: wrap; }
	.length-select { display: flex; align-items: center; gap: 0.5rem; font-size: 0.875rem; color: #6b21a8; }
	.length-select select { padding: 0.375rem 0.5rem; border: 1px solid #d8b4fe; border-radius: 6px; font-size: 0.8125rem; background: white; }

	.generate-btn {
		display: inline-flex; align-items: center; gap: 0.375rem;
		padding: 0.5rem 1rem;
		background: #8b5cf6; border: none; border-radius: 6px;
		color: white; font-size: 0.875rem; font-weight: 500;
		cursor: pointer; transition: background 0.15s;
	}
	.generate-btn:hover:not(:disabled) { background: #7c3aed; }
	.generate-btn:disabled { opacity: 0.6; cursor: not-allowed; }
	.generate-btn :global(.spinning) { animation: spin 1s linear infinite; }
	@keyframes spin { to { transform: rotate(360deg); } }

	.summary-error { padding: 0.5rem 0.75rem; background: #fef2f2; color: #dc2626; font-size: 0.8125rem; border-radius: 6px; margin-bottom: 0.75rem; }

	.output-section { }
	.output-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.5rem; }
	.output-header span { font-size: 0.75rem; font-weight: 600; color: #6b21a8; text-transform: uppercase; }
	.regen-btn {
		width: 28px; height: 28px;
		display: flex; align-items: center; justify-content: center;
		background: #ede9fe; border: none; border-radius: 6px;
		color: #7c3aed; cursor: pointer;
	}
	.regen-btn:hover { background: #ddd6fe; }
	.regen-btn:disabled { opacity: 0.5; }

	.summary-text {
		padding: 0.875rem;
		background: white; border: 1px solid #e9d5ff; border-radius: 8px;
		font-size: 0.9375rem; line-height: 1.6; color: #374151;
		outline: none;
	}
	.summary-text:focus { border-color: #8b5cf6; }

	.no-summary { padding: 1rem; text-align: center; color: #9ca3af; font-size: 0.875rem; }

	:global(.dark) .ai-summary-block { background: linear-gradient(135deg, #1e1b4b 0%, #312e81 100%); border-color: #4c1d95; }
	:global(.dark) .header-left { color: #c4b5fd; }
	:global(.dark) .chevron { color: #c4b5fd; }
	:global(.dark) .label-text { color: #c4b5fd; }
	:global(.dark) .source-section textarea { background: #0f172a; border-color: #4c1d95; color: #e2e8f0; }
	:global(.dark) .length-select { color: #c4b5fd; }
	:global(.dark) .length-select select { background: #0f172a; border-color: #4c1d95; color: #e2e8f0; }
	:global(.dark) .output-header span { color: #c4b5fd; }
	:global(.dark) .regen-btn { background: #312e81; color: #c4b5fd; }
	:global(.dark) .summary-text { background: #0f172a; border-color: #4c1d95; color: #e2e8f0; }
</style>

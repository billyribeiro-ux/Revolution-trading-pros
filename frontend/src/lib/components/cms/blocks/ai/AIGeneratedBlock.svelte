<!--
/**
 * AI Generated Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * AI content generation with prompt input and model selection
 */
-->

<script lang="ts">
	import { IconSparkles, IconRefresh, IconLoader2, IconCopy, IconCheck } from '$lib/icons';
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

	let aiState = $derived(stateManager.getAIGeneratedState(props.blockId));
	let copied = $state(false);

	let prompt = $derived(props.block.content.aiPrompt || '');
	let model = $derived(props.block.content.aiModel || 'gpt-4');
	let output = $derived(props.block.content.aiOutput || aiState.output || '');

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	async function generateContent(): Promise<void> {
		if (!prompt.trim()) {
			props.onError?.(new Error('Please enter a prompt'));
			return;
		}

		stateManager.setAIGeneratedState(props.blockId, { loading: true, error: null });

		try {
			// Simulated API call - replace with actual AI service integration
			await new Promise(resolve => setTimeout(resolve, 2000));
			
			const simulatedOutput = `AI-generated content based on your prompt: "${prompt}"\n\nThis is a placeholder response. In production, this would connect to your AI service (OpenAI, Claude, etc.) to generate real content based on the prompt and selected model (${model}).`;
			
			stateManager.setAIGeneratedState(props.blockId, { 
				loading: false, 
				output: simulatedOutput,
				lastGenerated: Date.now()
			});
			updateContent({ aiOutput: simulatedOutput });
		} catch (error) {
			const errorMessage = error instanceof Error ? error.message : 'Failed to generate content';
			stateManager.setAIGeneratedState(props.blockId, { loading: false, error: errorMessage });
			props.onError?.(new Error(errorMessage));
		}
	}

	async function copyOutput(): Promise<void> {
		if (output) {
			await navigator.clipboard.writeText(output);
			copied = true;
			setTimeout(() => { copied = false; }, 2000);
		}
	}

	function handleOutputEdit(e: Event): void {
		const target = e.target as HTMLElement;
		updateContent({ aiOutput: target.textContent || '' });
	}

	function handlePaste(e: ClipboardEvent): void {
		e.preventDefault();
		document.execCommand('insertText', false, e.clipboardData?.getData('text/plain') || '');
	}
</script>

<div class="ai-generated-block" role="region" aria-label="AI content generator">
	<div class="ai-header">
		<div class="ai-icon">
			<IconSparkles size={20} aria-hidden="true" />
		</div>
		<span class="ai-title">AI Content Generator</span>
	</div>

	{#if props.isEditing}
		<div class="ai-input-section">
			<label class="prompt-label">
				<span>Prompt</span>
				<textarea
					placeholder="Describe what content you want the AI to generate..."
					value={prompt}
					oninput={(e) => updateContent({ aiPrompt: (e.target as HTMLTextAreaElement).value })}
					aria-label="AI prompt"
					rows="3"
				></textarea>
			</label>

			<div class="ai-controls">
				<label class="model-select">
					<span>Model:</span>
					<select value={model} onchange={(e) => updateContent({ aiModel: (e.target as HTMLSelectElement).value })}>
						<option value="gpt-4">GPT-4</option>
						<option value="gpt-3.5-turbo">GPT-3.5 Turbo</option>
						<option value="claude-3">Claude 3</option>
						<option value="claude-2">Claude 2</option>
					</select>
				</label>

				<button
					type="button"
					class="generate-btn"
					onclick={generateContent}
					disabled={aiState.loading || !prompt.trim()}
					aria-label={aiState.loading ? 'Generating...' : 'Generate content'}
				>
					{#if aiState.loading}
						<IconLoader2 size={18} class="spinning" aria-hidden="true" />
						<span>Generating...</span>
					{:else}
						<IconSparkles size={18} aria-hidden="true" />
						<span>Generate</span>
					{/if}
				</button>
			</div>
		</div>
	{/if}

	{#if aiState.error}
		<div class="ai-error" role="alert">
			<span>{aiState.error}</span>
		</div>
	{/if}

	{#if output}
		<div class="ai-output-section">
			<div class="output-header">
				<span class="output-label">Generated Content</span>
				<div class="output-actions">
					{#if props.isEditing}
						<button type="button" class="action-btn" onclick={generateContent} disabled={aiState.loading} aria-label="Regenerate">
							<IconRefresh size={16} aria-hidden="true" />
						</button>
					{/if}
					<button type="button" class="action-btn" onclick={copyOutput} aria-label={copied ? 'Copied!' : 'Copy'}>
						{#if copied}
							<IconCheck size={16} aria-hidden="true" />
						{:else}
							<IconCopy size={16} aria-hidden="true" />
						{/if}
					</button>
				</div>
			</div>
			<div
				class="output-content"
				contenteditable={props.isEditing}
				role={props.isEditing ? 'textbox' : undefined}
				aria-label={props.isEditing ? 'Edit generated content' : undefined}
				oninput={handleOutputEdit}
				onpaste={handlePaste}
			>
				{output}
			</div>
		</div>
	{:else if !props.isEditing}
		<div class="ai-empty">
			<IconSparkles size={32} aria-hidden="true" />
			<span>No AI content generated yet</span>
		</div>
	{/if}
</div>

<style>
	.ai-generated-block { border: 1px solid #e5e7eb; border-radius: 12px; overflow: hidden; }

	.ai-header {
		display: flex; align-items: center; gap: 0.75rem;
		padding: 1rem 1.25rem;
		background: linear-gradient(135deg, #8b5cf6 0%, #6366f1 100%);
		color: white;
	}
	.ai-icon { display: flex; }
	.ai-title { font-weight: 600; font-size: 0.9375rem; }

	.ai-input-section { padding: 1.25rem; background: #faf5ff; border-bottom: 1px solid #e9d5ff; }
	.prompt-label { display: block; }
	.prompt-label span { display: block; font-size: 0.8125rem; font-weight: 600; color: #6b21a8; margin-bottom: 0.5rem; }
	.prompt-label textarea {
		width: 100%; padding: 0.75rem;
		border: 1px solid #d8b4fe; border-radius: 8px;
		font-size: 0.9375rem; resize: vertical;
		background: white;
	}
	.prompt-label textarea:focus { outline: none; border-color: #8b5cf6; box-shadow: 0 0 0 3px rgba(139, 92, 246, 0.1); }

	.ai-controls { display: flex; align-items: center; justify-content: space-between; gap: 1rem; margin-top: 1rem; flex-wrap: wrap; }
	.model-select { display: flex; align-items: center; gap: 0.5rem; font-size: 0.875rem; color: #6b21a8; }
	.model-select select { padding: 0.5rem 0.75rem; border: 1px solid #d8b4fe; border-radius: 6px; font-size: 0.875rem; background: white; }

	.generate-btn {
		display: inline-flex; align-items: center; gap: 0.5rem;
		padding: 0.625rem 1.25rem;
		background: linear-gradient(135deg, #8b5cf6, #6366f1);
		border: none; border-radius: 8px;
		color: white; font-size: 0.9375rem; font-weight: 600;
		cursor: pointer; transition: all 0.15s;
	}
	.generate-btn:hover:not(:disabled) { transform: translateY(-1px); box-shadow: 0 4px 12px rgba(139, 92, 246, 0.3); }
	.generate-btn:disabled { opacity: 0.6; cursor: not-allowed; }
	.generate-btn :global(.spinning) { animation: spin 1s linear infinite; }
	@keyframes spin { to { transform: rotate(360deg); } }

	.ai-error { padding: 0.75rem 1.25rem; background: #fef2f2; color: #dc2626; font-size: 0.875rem; }

	.ai-output-section { padding: 1.25rem; }
	.output-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.75rem; }
	.output-label { font-size: 0.8125rem; font-weight: 600; color: #6b7280; }
	.output-actions { display: flex; gap: 0.5rem; }
	.action-btn {
		width: 32px; height: 32px;
		display: flex; align-items: center; justify-content: center;
		background: #f3f4f6; border: none; border-radius: 6px;
		color: #6b7280; cursor: pointer; transition: all 0.15s;
	}
	.action-btn:hover { background: #e5e7eb; color: #374151; }
	.action-btn:disabled { opacity: 0.5; cursor: not-allowed; }

	.output-content {
		padding: 1rem;
		background: #f9fafb; border: 1px solid #e5e7eb; border-radius: 8px;
		font-size: 0.9375rem; line-height: 1.7; color: #374151;
		white-space: pre-wrap; outline: none;
		min-height: 100px;
	}
	.output-content:focus { border-color: #8b5cf6; box-shadow: 0 0 0 3px rgba(139, 92, 246, 0.1); }

	.ai-empty {
		display: flex; flex-direction: column; align-items: center; gap: 0.75rem;
		padding: 3rem; color: #9ca3af;
	}

	:global(.dark) .ai-generated-block { border-color: #374151; }
	:global(.dark) .ai-input-section { background: #1e1b4b; border-color: #4c1d95; }
	:global(.dark) .prompt-label span { color: #c4b5fd; }
	:global(.dark) .prompt-label textarea { background: #0f172a; border-color: #4c1d95; color: #e2e8f0; }
	:global(.dark) .model-select { color: #c4b5fd; }
	:global(.dark) .model-select select { background: #0f172a; border-color: #4c1d95; color: #e2e8f0; }
	:global(.dark) .ai-error { background: #450a0a; color: #fca5a5; }
	:global(.dark) .output-content { background: #1e293b; border-color: #334155; color: #e2e8f0; }
	:global(.dark) .action-btn { background: #1e293b; color: #94a3b8; }
	:global(.dark) .action-btn:hover { background: #334155; color: #e2e8f0; }
</style>

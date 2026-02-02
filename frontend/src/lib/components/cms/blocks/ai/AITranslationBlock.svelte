<!--
/**
 * AI Translation Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Language translation with side-by-side or stacked view
 */
-->

<script lang="ts">
	import { IconLanguage, IconArrowsLeftRight, IconCopy, IconCheck, IconLoader2 } from '$lib/icons';
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

	let props: Props = $props();
	const stateManager = getBlockStateManager();

	let aiState = $derived(stateManager.getAITranslationState(props.blockId));
	let copied = $state(false);

	let sourceText = $derived(props.block.content.translationSource || '');
	let sourceLang = $derived(props.block.content.translationSourceLang || 'en');
	let targetLang = $derived(props.block.content.translationTargetLang || 'es');
	let translatedText = $derived(props.block.content.translationOutput || aiState.output || '');
	let viewMode = $derived((props.block.content.translationView as 'stacked' | 'side-by-side') || 'stacked');

	const languages = [
		{ code: 'en', name: 'English' },
		{ code: 'es', name: 'Spanish' },
		{ code: 'fr', name: 'French' },
		{ code: 'de', name: 'German' },
		{ code: 'it', name: 'Italian' },
		{ code: 'pt', name: 'Portuguese' },
		{ code: 'zh', name: 'Chinese' },
		{ code: 'ja', name: 'Japanese' },
		{ code: 'ko', name: 'Korean' },
		{ code: 'ar', name: 'Arabic' },
		{ code: 'ru', name: 'Russian' }
	];

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	async function translate(): Promise<void> {
		if (!sourceText.trim()) {
			props.onError?.(new Error('Please enter text to translate'));
			return;
		}

		stateManager.setAITranslationState(props.blockId, { loading: true, error: null });

		try {
			await new Promise(resolve => setTimeout(resolve, 1500));
			const targetName = languages.find(l => l.code === targetLang)?.name || targetLang;
			const simulated = `[${targetName} translation of]: ${sourceText}`;
			
			stateManager.setAITranslationState(props.blockId, { loading: false, output: simulated, lastGenerated: Date.now() });
			updateContent({ translationOutput: simulated });
		} catch (error) {
			const msg = error instanceof Error ? error.message : 'Translation failed';
			stateManager.setAITranslationState(props.blockId, { loading: false, error: msg });
			props.onError?.(new Error(msg));
		}
	}

	function swapLanguages(): void {
		updateContent({
			translationSourceLang: targetLang,
			translationTargetLang: sourceLang,
			translationSource: translatedText,
			translationOutput: sourceText
		});
	}

	async function copyTranslation(): Promise<void> {
		if (translatedText) {
			await navigator.clipboard.writeText(translatedText);
			copied = true;
			setTimeout(() => { copied = false; }, 2000);
		}
	}
</script>

<div class="translation-block" class:side-by-side={viewMode === 'side-by-side'} role="region" aria-label="Translation">
	<div class="translation-header">
		<IconLanguage size={20} aria-hidden="true" />
		<span>Translation</span>
	</div>

	<div class="language-bar">
		<select value={sourceLang} onchange={(e) => updateContent({ translationSourceLang: (e.target as HTMLSelectElement).value })} aria-label="Source language">
			{#each languages as lang (lang.code)}
				<option value={lang.code}>{lang.name}</option>
			{/each}
		</select>

		<button type="button" class="swap-btn" onclick={swapLanguages} aria-label="Swap languages">
			<IconArrowsLeftRight size={18} />
		</button>

		<select value={targetLang} onchange={(e) => updateContent({ translationTargetLang: (e.target as HTMLSelectElement).value })} aria-label="Target language">
			{#each languages as lang (lang.code)}
				<option value={lang.code}>{lang.name}</option>
			{/each}
		</select>
	</div>

	<div class="translation-panels">
		<div class="panel source-panel">
			<div class="panel-header">
				<span>{languages.find(l => l.code === sourceLang)?.name || 'Source'}</span>
			</div>
			{#if props.isEditing}
				<textarea
					class="panel-content"
					placeholder="Enter text to translate..."
					value={sourceText}
					oninput={(e) => updateContent({ translationSource: (e.target as HTMLTextAreaElement).value })}
				></textarea>
			{:else}
				<div class="panel-content readonly">{sourceText || 'No source text'}</div>
			{/if}
		</div>

		<div class="panel target-panel">
			<div class="panel-header">
				<span>{languages.find(l => l.code === targetLang)?.name || 'Target'}</span>
				{#if translatedText}
					<button type="button" class="copy-btn" onclick={copyTranslation} aria-label="Copy translation">
						{#if copied}<IconCheck size={14} />{:else}<IconCopy size={14} />{/if}
					</button>
				{/if}
			</div>
			<div class="panel-content readonly">
				{#if aiState.loading}
					<div class="loading"><IconLoader2 size={20} class="spinning" /> Translating...</div>
				{:else if translatedText}
					{translatedText}
				{:else}
					<span class="placeholder">Translation will appear here</span>
				{/if}
			</div>
		</div>
	</div>

	{#if aiState.error}
		<div class="translation-error" role="alert">{aiState.error}</div>
	{/if}

	{#if props.isEditing}
		<div class="translation-actions">
			<button type="button" class="translate-btn" onclick={translate} disabled={aiState.loading || !sourceText.trim()}>
				{#if aiState.loading}
					<IconLoader2 size={16} class="spinning" /> Translating...
				{:else}
					<IconLanguage size={16} /> Translate
				{/if}
			</button>

			<div class="view-toggle">
				<label>
					<input type="radio" name="view-{props.blockId}" value="stacked" checked={viewMode === 'stacked'} onchange={() => updateContent({ translationView: 'stacked' })} />
					Stacked
				</label>
				<label>
					<input type="radio" name="view-{props.blockId}" value="side-by-side" checked={viewMode === 'side-by-side'} onchange={() => updateContent({ translationView: 'side-by-side' })} />
					Side by Side
				</label>
			</div>
		</div>
	{/if}
</div>

<style>
	.translation-block { border: 1px solid #e5e7eb; border-radius: 12px; overflow: hidden; }

	.translation-header {
		display: flex; align-items: center; gap: 0.5rem;
		padding: 0.875rem 1rem;
		background: linear-gradient(135deg, #0ea5e9 0%, #0284c7 100%);
		color: white; font-weight: 600;
	}

	.language-bar {
		display: flex; align-items: center; gap: 0.75rem;
		padding: 0.75rem 1rem;
		background: #f0f9ff; border-bottom: 1px solid #e0f2fe;
	}
	.language-bar select {
		flex: 1; padding: 0.5rem 0.75rem;
		border: 1px solid #bae6fd; border-radius: 6px;
		font-size: 0.875rem; background: white;
	}
	.swap-btn {
		width: 36px; height: 36px;
		display: flex; align-items: center; justify-content: center;
		background: white; border: 1px solid #bae6fd; border-radius: 8px;
		color: #0284c7; cursor: pointer; transition: all 0.15s;
	}
	.swap-btn:hover { background: #e0f2fe; }

	.translation-panels { display: flex; flex-direction: column; }
	.side-by-side .translation-panels { flex-direction: row; }

	.panel { flex: 1; display: flex; flex-direction: column; }
	.side-by-side .panel { min-width: 0; }
	.source-panel { border-bottom: 1px solid #e5e7eb; }
	.side-by-side .source-panel { border-bottom: none; border-right: 1px solid #e5e7eb; }

	.panel-header {
		display: flex; justify-content: space-between; align-items: center;
		padding: 0.5rem 1rem;
		background: #f9fafb; font-size: 0.75rem; font-weight: 600; color: #6b7280; text-transform: uppercase;
	}
	.copy-btn {
		width: 28px; height: 28px;
		display: flex; align-items: center; justify-content: center;
		background: #e5e7eb; border: none; border-radius: 4px;
		color: #6b7280; cursor: pointer;
	}
	.copy-btn:hover { background: #d1d5db; }

	.panel-content {
		flex: 1; min-height: 120px; padding: 1rem;
		font-size: 0.9375rem; line-height: 1.6;
		border: none; resize: none; outline: none;
	}
	textarea.panel-content { background: white; }
	.panel-content.readonly { background: #f9fafb; color: #374151; }
	.panel-content .placeholder { color: #9ca3af; }
	.panel-content .loading { display: flex; align-items: center; gap: 0.5rem; color: #6b7280; }
	.panel-content :global(.spinning) { animation: spin 1s linear infinite; }
	@keyframes spin { to { transform: rotate(360deg); } }

	.translation-error { padding: 0.75rem 1rem; background: #fef2f2; color: #dc2626; font-size: 0.875rem; }

	.translation-actions {
		display: flex; justify-content: space-between; align-items: center; gap: 1rem;
		padding: 1rem; background: #f9fafb; border-top: 1px solid #e5e7eb; flex-wrap: wrap;
	}
	.translate-btn {
		display: inline-flex; align-items: center; gap: 0.5rem;
		padding: 0.625rem 1.25rem;
		background: #0ea5e9; border: none; border-radius: 8px;
		color: white; font-weight: 600; cursor: pointer; transition: background 0.15s;
	}
	.translate-btn:hover:not(:disabled) { background: #0284c7; }
	.translate-btn:disabled { opacity: 0.6; cursor: not-allowed; }

	.view-toggle { display: flex; gap: 1rem; }
	.view-toggle label { display: flex; align-items: center; gap: 0.375rem; font-size: 0.875rem; cursor: pointer; }

	@media (max-width: 640px) {
		.side-by-side .translation-panels { flex-direction: column; }
		.side-by-side .source-panel { border-right: none; border-bottom: 1px solid #e5e7eb; }
	}

	:global(.dark) .translation-block { border-color: #374151; }
	:global(.dark) .language-bar { background: #0c4a6e; border-color: #075985; }
	:global(.dark) .language-bar select { background: #0f172a; border-color: #075985; color: #e2e8f0; }
	:global(.dark) .swap-btn { background: #0f172a; border-color: #075985; color: #38bdf8; }
	:global(.dark) .panel-header { background: #1e293b; color: #94a3b8; }
	:global(.dark) textarea.panel-content { background: #0f172a; color: #e2e8f0; }
	:global(.dark) .panel-content.readonly { background: #1e293b; color: #e2e8f0; }
	:global(.dark) .translation-actions { background: #1e293b; border-color: #374151; }
	:global(.dark) .copy-btn { background: #334155; color: #94a3b8; }
</style>

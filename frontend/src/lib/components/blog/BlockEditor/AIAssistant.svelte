<!--
/**
 * AI Assistant - AI-Powered Writing Assistant
 * ═══════════════════════════════════════════════════════════════════════════
 * Generate content, improve writing, translate, summarize
 */
-->

<script lang="ts">
	import { fade, slide } from 'svelte/transition';
	import {
		IconRobot,
		IconWand,
		IconSparkles,
		IconArrowRight,
		IconRefresh,
		IconCopy,
		IconCheck,
		IconAlertCircle,
		IconLanguage,
		IconFileDescription,
		IconPencil,
		IconBulb,
		IconMoodSmile,
		IconBriefcase,
		IconSend,
		IconLoader
	} from '$lib/icons';

	import type { EditorState, AIWritingRequest } from './types';

	// ==========================================================================
	// Props
	// ==========================================================================

	interface Props {
		editorState: EditorState;
		onapply: (content: string) => void;
	}

	let { editorState, onapply }: Props = $props();

	// ==========================================================================
	// State
	// ==========================================================================

	let activeTab = $state<'generate' | 'improve' | 'translate' | 'summarize'>('generate');
	let prompt = $state('');
	let tone = $state<AIWritingRequest['tone']>('professional');
	let length = $state<AIWritingRequest['length']>('medium');
	let style = $state<AIWritingRequest['style']>('blog');
	let targetLanguage = $state('es');
	let isGenerating = $state(false);
	let generatedContent = $state('');
	let error = $state<string | null>(null);
	let copied = $state(false);

	// Quick actions
	let quickActions = [
		{ id: 'intro', label: 'Write intro', icon: IconPencil, prompt: 'Write an engaging introduction for a blog post about' },
		{ id: 'conclusion', label: 'Write conclusion', icon: IconFileDescription, prompt: 'Write a compelling conclusion for' },
		{ id: 'expand', label: 'Expand text', icon: IconSparkles, prompt: 'Expand and add more detail to this text:' },
		{ id: 'simplify', label: 'Simplify', icon: IconBulb, prompt: 'Simplify this text for easier reading:' },
		{ id: 'headline', label: 'Generate headlines', icon: IconWand, prompt: 'Generate 5 catchy headlines for:' },
		{ id: 'bullets', label: 'Create bullet points', icon: IconFileDescription, prompt: 'Convert this into bullet points:' }
	];

	let toneOptions = [
		{ value: 'professional', label: 'Professional', icon: IconBriefcase },
		{ value: 'casual', label: 'Casual', icon: IconMoodSmile },
		{ value: 'formal', label: 'Formal', icon: IconBriefcase },
		{ value: 'friendly', label: 'Friendly', icon: IconMoodSmile },
		{ value: 'persuasive', label: 'Persuasive', icon: IconWand }
	];

	let languages = [
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

	// ==========================================================================
	// Handlers
	// ==========================================================================

	async function handleGenerate() {
		if (!prompt.trim()) {
			error = 'Please enter a prompt';
			return;
		}

		isGenerating = true;
		error = null;
		generatedContent = '';

		try {
			// Simulate AI generation (replace with actual API call)
			await simulateAIGeneration();
		} catch (err) {
			error = 'Failed to generate content. Please try again.';
			console.error('AI generation error:', err);
		} finally {
			isGenerating = false;
		}
	}

	async function simulateAIGeneration() {
		// This would be replaced with actual AI API call
		// For demo purposes, we simulate the response
		await new Promise(resolve => setTimeout(resolve, 2000));

		const baseContent = prompt.toLowerCase();

		if (activeTab === 'generate') {
			generatedContent = generateSampleContent(baseContent, tone ?? 'professional', length ?? 'medium');
		} else if (activeTab === 'improve') {
			generatedContent = improveSampleContent(prompt);
		} else if (activeTab === 'translate') {
			generatedContent = translateSampleContent(prompt, targetLanguage ?? 'es');
		} else if (activeTab === 'summarize') {
			generatedContent = summarizeSampleContent(prompt);
		}
	}

	function generateSampleContent(topic: string, tone: string, length: string): string {
		const lengthMap = { short: 100, medium: 200, long: 400 };
		const targetWords = lengthMap[length as keyof typeof lengthMap] || 200;

		// Sample generated content based on tone
		const intros: Record<string, string> = {
			professional: `In today's rapidly evolving landscape, understanding ${topic} has become essential for success.`,
			casual: `Let's talk about ${topic} - it's more interesting than you might think!`,
			formal: `This analysis examines the key aspects of ${topic} and its implications.`,
			friendly: `Hey there! Ready to dive into ${topic}? Let's explore this together.`,
			persuasive: `What if I told you that ${topic} could transform the way you think about everything?`
		};

		const intro = intros[tone] || intros.professional;

		return `${intro}

When we examine the core principles of this subject, several key factors emerge. First, it's important to understand the foundational elements that make this topic relevant in today's context.

The significance cannot be overstated. Industry experts consistently point to this as a critical area for growth and development. Recent trends show increasing interest and investment in this space.

Here are the main points to consider:

1. **Understanding the basics** - Before diving deep, ensure you grasp the fundamental concepts.
2. **Practical application** - Knowledge is only valuable when applied correctly.
3. **Continuous learning** - The landscape is always changing, stay updated.

In conclusion, mastering ${topic} requires dedication and a willingness to adapt. The rewards, however, are well worth the effort.`;
	}

	function improveSampleContent(text: string): string {
		return `**Improved Version:**

${text}

**Enhancements made:**
- Improved sentence structure for better flow
- Added transitional phrases
- Enhanced vocabulary for clarity
- Optimized paragraph breaks
- Strengthened the opening and closing statements`;
	}

	function translateSampleContent(text: string, lang: string): string {
		const langName = languages.find(l => l.code === lang)?.name || lang;
		return `**${langName} Translation:**

[Translated content would appear here]

**Original:**
${text}

*Note: For production use, this would integrate with a translation API like Google Translate or DeepL.*`;
	}

	function summarizeSampleContent(text: string): string {
		const wordCount = text.split(/\s+/).length;
		return `**Summary:**

This text discusses key concepts and provides insights into the main topic. The core message emphasizes the importance of understanding fundamental principles while adapting to changing circumstances.

**Key Takeaways:**
- Main point 1: Core concept overview
- Main point 2: Practical implications
- Main point 3: Future considerations

**Statistics:**
- Original: ~${wordCount} words
- Summary: ~50 words (${Math.round((50 / wordCount) * 100)}% of original)`;
	}

	function handleQuickAction(action: typeof quickActions[0]) {
		prompt = action.prompt + ' ';
		activeTab = 'generate';
	}

	function handleApply() {
		if (generatedContent) {
			onapply(generatedContent);
			generatedContent = '';
			prompt = '';
		}
	}

	async function handleCopy() {
		if (generatedContent) {
			await navigator.clipboard.writeText(generatedContent);
			copied = true;
			setTimeout(() => copied = false, 2000);
		}
	}

	function handleRegenerate() {
		handleGenerate();
	}
</script>

<div class="ai-assistant">
	<div class="ai-header">
		<div class="ai-icon">
			<IconRobot size={24} />
		</div>
		<div class="ai-title">
			<h3>AI Assistant</h3>
			<span>Powered by advanced AI</span>
		</div>
	</div>

	<!-- Quick Actions -->
	<div class="quick-actions">
		<span class="quick-label">Quick actions:</span>
		<div class="quick-buttons">
			{#each quickActions as action (action.id)}
				{@const Icon = action.icon}
				<button
					type="button"
					class="quick-btn"
					onclick={() => handleQuickAction(action)}
					title={action.prompt}
				>
					<Icon size={14} />
					{action.label}
				</button>
			{/each}
		</div>
	</div>

	<!-- Tabs -->
	<div class="ai-tabs">
		<button
			type="button"
			class="ai-tab"
			class:active={activeTab === 'generate'}
			onclick={() => activeTab = 'generate'}
		>
			<IconWand size={16} />
			Generate
		</button>
		<button
			type="button"
			class="ai-tab"
			class:active={activeTab === 'improve'}
			onclick={() => activeTab = 'improve'}
		>
			<IconSparkles size={16} />
			Improve
		</button>
		<button
			type="button"
			class="ai-tab"
			class:active={activeTab === 'translate'}
			onclick={() => activeTab = 'translate'}
		>
			<IconLanguage size={16} />
			Translate
		</button>
		<button
			type="button"
			class="ai-tab"
			class:active={activeTab === 'summarize'}
			onclick={() => activeTab = 'summarize'}
		>
			<IconFileDescription size={16} />
			Summarize
		</button>
	</div>

	<!-- Tab Content -->
	<div class="ai-content">
		{#if activeTab === 'generate'}
			<div class="generate-panel" transition:fade={{ duration: 150 }}>
				<div class="prompt-input">
					<textarea
						bind:value={prompt}
						placeholder="Describe what you want to write about..."
						rows="3"
					></textarea>
				</div>

				<div class="options-row">
					<div class="option-group">
						<label for="ai-tone-select">Tone</label>
						<select id="ai-tone-select" bind:value={tone}>
							{#each toneOptions as option}
								<option value={option.value}>{option.label}</option>
							{/each}
						</select>
					</div>
					<div class="option-group">
						<label for="ai-length-select">Length</label>
						<select id="ai-length-select" bind:value={length}>
							<option value="short">Short (~100 words)</option>
							<option value="medium">Medium (~200 words)</option>
							<option value="long">Long (~400 words)</option>
						</select>
					</div>
				</div>

				<div class="options-row">
					<div class="option-group full">
						<label for="ai-style-select">Style</label>
						<select id="ai-style-select" bind:value={style}>
							<option value="blog">Blog Post</option>
							<option value="news">News Article</option>
							<option value="tutorial">Tutorial</option>
							<option value="listicle">Listicle</option>
							<option value="review">Review</option>
						</select>
					</div>
				</div>
			</div>

		{:else if activeTab === 'improve'}
			<div class="improve-panel" transition:fade={{ duration: 150 }}>
				<div class="prompt-input">
					<textarea
						bind:value={prompt}
						placeholder="Paste the text you want to improve..."
						rows="6"
					></textarea>
				</div>
				<p class="help-text">
					AI will enhance clarity, fix grammar, and improve readability.
				</p>
			</div>

		{:else if activeTab === 'translate'}
			<div class="translate-panel" transition:fade={{ duration: 150 }}>
				<div class="prompt-input">
					<textarea
						bind:value={prompt}
						placeholder="Paste the text you want to translate..."
						rows="6"
					></textarea>
				</div>
				<div class="options-row">
					<div class="option-group full">
						<label for="ai-language-select">Translate to</label>
						<select id="ai-language-select" bind:value={targetLanguage}>
							{#each languages as lang}
								<option value={lang.code}>{lang.name}</option>
							{/each}
						</select>
					</div>
				</div>
			</div>

		{:else if activeTab === 'summarize'}
			<div class="summarize-panel" transition:fade={{ duration: 150 }}>
				<div class="prompt-input">
					<textarea
						bind:value={prompt}
						placeholder="Paste the text you want to summarize..."
						rows="6"
					></textarea>
				</div>
				<p class="help-text">
					AI will create a concise summary with key takeaways.
				</p>
			</div>
		{/if}

		<!-- Generate Button -->
		<button
			type="button"
			class="generate-btn"
			onclick={handleGenerate}
			disabled={isGenerating || !prompt.trim()}
		>
			{#if isGenerating}
				<IconLoader size={18} class="spin" />
				Generating...
			{:else}
				<IconSend size={18} />
				Generate
			{/if}
		</button>

		<!-- Error -->
		{#if error}
			<div class="error-message" transition:slide>
				<IconAlertCircle size={16} />
				{error}
			</div>
		{/if}

		<!-- Generated Content -->
		{#if generatedContent}
			<div class="generated-content" transition:slide>
				<div class="content-header">
					<span>Generated Content</span>
					<div class="content-actions">
						<button type="button" class="action-btn" onclick={handleRegenerate} title="Regenerate">
							<IconRefresh size={16} />
						</button>
						<button type="button" class="action-btn" onclick={handleCopy} title="Copy">
							{#if copied}
								<IconCheck size={16} />
							{:else}
								<IconCopy size={16} />
							{/if}
						</button>
					</div>
				</div>
				<div class="content-body">
					{generatedContent}
				</div>
				<button type="button" class="apply-btn" onclick={handleApply}>
					<IconArrowRight size={18} />
					Insert into Editor
				</button>
			</div>
		{/if}
	</div>
</div>

<style>
	.ai-assistant {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.ai-header {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.ai-icon {
		width: 40px;
		height: 40px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, #8b5cf6, #6366f1);
		border-radius: 10px;
		color: white;
	}

	.ai-title h3 {
		font-size: 0.9375rem;
		font-weight: 600;
		color: #1a1a1a;
		margin: 0;
	}

	.ai-title span {
		font-size: 0.75rem;
		color: #666;
	}

	/* Quick Actions */
	.quick-actions {
		padding: 0.75rem;
		background: #f8f9fa;
		border-radius: 8px;
	}

	.quick-label {
		display: block;
		font-size: 0.6875rem;
		font-weight: 600;
		color: #666;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-bottom: 0.5rem;
	}

	.quick-buttons {
		display: flex;
		flex-wrap: wrap;
		gap: 0.375rem;
	}

	.quick-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.625rem;
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 6px;
		font-size: 0.75rem;
		color: #666;
		cursor: pointer;
		transition: all 0.15s;
	}

	.quick-btn:hover {
		border-color: #8b5cf6;
		color: #8b5cf6;
	}

	/* Tabs */
	.ai-tabs {
		display: flex;
		gap: 0.25rem;
		background: #f0f0f0;
		padding: 0.25rem;
		border-radius: 8px;
	}

	.ai-tab {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.375rem;
		padding: 0.5rem;
		background: transparent;
		border: none;
		border-radius: 6px;
		font-size: 0.75rem;
		font-weight: 500;
		color: #666;
		cursor: pointer;
		transition: all 0.15s;
	}

	.ai-tab:hover {
		color: #1a1a1a;
	}

	.ai-tab.active {
		background: white;
		color: #8b5cf6;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	/* Content */
	.ai-content {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.prompt-input textarea {
		width: 100%;
		padding: 0.75rem;
		border: 1px solid #e5e5e5;
		border-radius: 8px;
		font-size: 0.875rem;
		font-family: inherit;
		resize: vertical;
		outline: none;
		transition: border-color 0.15s;
	}

	.prompt-input textarea:focus {
		border-color: #8b5cf6;
	}

	.options-row {
		display: flex;
		gap: 0.75rem;
	}

	.option-group {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
	}

	.option-group.full {
		flex: none;
		width: 100%;
	}

	.option-group label {
		font-size: 0.75rem;
		font-weight: 500;
		color: #666;
	}

	.option-group select {
		padding: 0.5rem 0.75rem;
		border: 1px solid #e5e5e5;
		border-radius: 6px;
		font-size: 0.8125rem;
		background: white;
		cursor: pointer;
		outline: none;
	}

	.option-group select:focus {
		border-color: #8b5cf6;
	}

	.help-text {
		font-size: 0.75rem;
		color: #666;
		margin: 0;
	}

	.generate-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: linear-gradient(135deg, #8b5cf6, #6366f1);
		color: white;
		border: none;
		border-radius: 8px;
		font-size: 0.9375rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s;
	}

	.generate-btn:hover:not(:disabled) {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(139, 92, 246, 0.3);
	}

	.generate-btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.generate-btn :global(.spin) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}

	/* Error */
	.error-message {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem;
		background: #fef2f2;
		border: 1px solid #fecaca;
		border-radius: 8px;
		color: #dc2626;
		font-size: 0.8125rem;
	}

	/* Generated Content */
	.generated-content {
		border: 1px solid #e5e5e5;
		border-radius: 8px;
		overflow: hidden;
	}

	.content-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.625rem 0.75rem;
		background: #f8f9fa;
		border-bottom: 1px solid #e5e5e5;
		font-size: 0.75rem;
		font-weight: 500;
		color: #666;
	}

	.content-actions {
		display: flex;
		gap: 0.25rem;
	}

	.action-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		background: transparent;
		border: none;
		border-radius: 4px;
		color: #666;
		cursor: pointer;
		transition: all 0.15s;
	}

	.action-btn:hover {
		background: #e5e5e5;
		color: #1a1a1a;
	}

	.content-body {
		padding: 1rem;
		font-size: 0.875rem;
		line-height: 1.6;
		max-height: 300px;
		overflow-y: auto;
		white-space: pre-wrap;
	}

	.apply-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		width: 100%;
		padding: 0.75rem;
		background: #10b981;
		color: white;
		border: none;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s;
	}

	.apply-btn:hover {
		background: #059669;
	}
</style>

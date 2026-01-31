<!--
/**
 * AI Assistant - AI-Powered Writing Assistant with Real Backend API
 * ═══════════════════════════════════════════════════════════════════════════
 * Generate content, improve writing, translate, summarize
 * Connected to backend API with SSE streaming support
 *
 * Backend endpoints:
 * - POST /api/cms/ai/assist - Single response
 * - POST /api/cms/ai/assist/stream - SSE streaming response
 * - GET /api/cms/ai/history - Get AI usage history
 *
 * @version 2.0.0 - Real API Integration
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
		IconLoader,
		IconX,
		IconClock
	} from '$lib/icons';

	import { API_BASE_URL } from '$lib/api/config';
	import { getAuthToken } from '$lib/stores/auth.svelte';
	import type { EditorState, AIWritingRequest } from './types';

	// ==========================================================================
	// Types
	// ==========================================================================

	type AIAction = 'generate' | 'improve' | 'translate' | 'summarize';

	interface AIAssistRequest {
		action: AIAction;
		input_text: string;
		block_id?: string;
		content_id?: string;
		options?: {
			tone?: string;
			length?: string;
			style?: string;
			target_language?: string;
		};
	}

	interface AIAssistResponse {
		success: boolean;
		content?: string;
		error?: string;
		usage?: {
			tokens_used: number;
			requests_remaining: number;
			rate_limit_reset?: string;
		};
	}

	interface SSEEvent {
		event: string;
		data: string;
	}

	interface RateLimitState {
		remaining: number;
		resetTime: Date | null;
		isLimited: boolean;
	}

	// ==========================================================================
	// Props
	// ==========================================================================

	interface Props {
		editorState: EditorState;
		blockId?: string;
		contentId?: string;
		onapply: (content: string) => void;
	}

	let { editorState, blockId, contentId, onapply }: Props = $props();

	// ==========================================================================
	// State - Svelte 5 Runes
	// ==========================================================================

	let activeTab = $state<AIAction>('generate');
	let prompt = $state('');
	let tone = $state<AIWritingRequest['tone']>('professional');
	let length = $state<AIWritingRequest['length']>('medium');
	let style = $state<AIWritingRequest['style']>('blog');
	let targetLanguage = $state('es');
	let isGenerating = $state(false);
	let generatedContent = $state('');
	let error = $state<string | null>(null);
	let copied = $state(false);

	// Streaming state
	let streamController = $state<AbortController | null>(null);
	let isStreaming = $state(false);

	// Rate limiting state
	let rateLimit = $state<RateLimitState>({
		remaining: 10,
		resetTime: null,
		isLimited: false
	});

	// Retry state
	let retryCount = $state(0);
	const MAX_RETRIES = 3;
	const BASE_RETRY_DELAY = 1000; // 1 second

	// ==========================================================================
	// Derived values
	// ==========================================================================

	let rateLimitMessage = $derived(() => {
		if (!rateLimit.isLimited || !rateLimit.resetTime) return '';
		const now = new Date();
		const diff = Math.ceil((rateLimit.resetTime.getTime() - now.getTime()) / 1000);
		if (diff <= 0) return '';
		const minutes = Math.floor(diff / 60);
		const seconds = diff % 60;
		return `Rate limit reached. Try again in ${minutes}m ${seconds}s`;
	});

	let canGenerate = $derived(!isGenerating && prompt.trim().length > 0 && !rateLimit.isLimited);

	// ==========================================================================
	// Constants
	// ==========================================================================

	const quickActions = [
		{
			id: 'intro',
			label: 'Write intro',
			icon: IconPencil,
			prompt: 'Write an engaging introduction for a blog post about'
		},
		{
			id: 'conclusion',
			label: 'Write conclusion',
			icon: IconFileDescription,
			prompt: 'Write a compelling conclusion for'
		},
		{
			id: 'expand',
			label: 'Expand text',
			icon: IconSparkles,
			prompt: 'Expand and add more detail to this text:'
		},
		{
			id: 'simplify',
			label: 'Simplify',
			icon: IconBulb,
			prompt: 'Simplify this text for easier reading:'
		},
		{
			id: 'headline',
			label: 'Generate headlines',
			icon: IconWand,
			prompt: 'Generate 5 catchy headlines for:'
		},
		{
			id: 'bullets',
			label: 'Create bullet points',
			icon: IconFileDescription,
			prompt: 'Convert this into bullet points:'
		}
	];

	const toneOptions = [
		{ value: 'professional', label: 'Professional', icon: IconBriefcase },
		{ value: 'casual', label: 'Casual', icon: IconMoodSmile },
		{ value: 'formal', label: 'Formal', icon: IconBriefcase },
		{ value: 'friendly', label: 'Friendly', icon: IconMoodSmile },
		{ value: 'persuasive', label: 'Persuasive', icon: IconWand }
	];

	const languages = [
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
	// Rate limit timer effect
	// ==========================================================================

	$effect(() => {
		if (!rateLimit.isLimited || !rateLimit.resetTime) return;

		const checkRateLimit = () => {
			const now = new Date();
			if (rateLimit.resetTime && now >= rateLimit.resetTime) {
				rateLimit = {
					remaining: 10,
					resetTime: null,
					isLimited: false
				};
			}
		};

		const interval = setInterval(checkRateLimit, 1000);
		return () => clearInterval(interval);
	});

	// ==========================================================================
	// API Helpers
	// ==========================================================================

	function buildRequestBody(): AIAssistRequest {
		const options: AIAssistRequest['options'] = {};

		if (activeTab === 'generate') {
			options.tone = tone;
			options.length = length;
			options.style = style;
		} else if (activeTab === 'translate') {
			options.target_language = targetLanguage;
		}

		return {
			action: activeTab,
			input_text: prompt,
			...(blockId && { block_id: blockId }),
			...(contentId && { content_id: contentId }),
			options
		};
	}

	function getAuthHeaders(): HeadersInit {
		const token = getAuthToken();
		const headers: HeadersInit = {
			'Content-Type': 'application/json',
			Accept: 'text/event-stream'
		};

		if (token) {
			headers['Authorization'] = `Bearer ${token}`;
		}

		return headers;
	}

	async function delay(ms: number): Promise<void> {
		return new Promise((resolve) => setTimeout(resolve, ms));
	}

	function parseSSELine(line: string): SSEEvent | null {
		if (!line.trim() || line.startsWith(':')) return null;

		if (line.startsWith('data:')) {
			return {
				event: 'data',
				data: line.slice(5).trim()
			};
		}

		if (line.startsWith('event:')) {
			return {
				event: line.slice(6).trim(),
				data: ''
			};
		}

		return null;
	}

	function handleRateLimitResponse(response: Response): void {
		const remaining = response.headers.get('X-RateLimit-Remaining');
		const reset = response.headers.get('X-RateLimit-Reset');

		if (remaining !== null) {
			const remainingNum = parseInt(remaining, 10);
			rateLimit = {
				...rateLimit,
				remaining: remainingNum,
				isLimited: remainingNum <= 0
			};
		}

		if (reset) {
			rateLimit = {
				...rateLimit,
				resetTime: new Date(reset)
			};
		}

		if (response.status === 429) {
			const retryAfter = response.headers.get('Retry-After');
			const resetTime = retryAfter
				? new Date(Date.now() + parseInt(retryAfter, 10) * 1000)
				: new Date(Date.now() + 60000); // Default 1 minute

			rateLimit = {
				remaining: 0,
				resetTime,
				isLimited: true
			};
		}
	}

	// ==========================================================================
	// Streaming API Call
	// ==========================================================================

	async function streamGenerate(): Promise<void> {
		const controller = new AbortController();
		streamController = controller;
		isStreaming = true;

		try {
			const response = await fetch(`${API_BASE_URL}/api/cms/ai/assist/stream`, {
				method: 'POST',
				headers: getAuthHeaders(),
				body: JSON.stringify(buildRequestBody()),
				signal: controller.signal,
				credentials: 'include'
			});

			handleRateLimitResponse(response);

			if (!response.ok) {
				if (response.status === 429) {
					throw new Error('Rate limit exceeded. Please wait before trying again.');
				}
				if (response.status === 401) {
					throw new Error('Authentication required. Please log in again.');
				}
				if (response.status === 403) {
					throw new Error('You do not have permission to use the AI assistant.');
				}

				const errorData = await response.json().catch(() => ({}));
				throw new Error(errorData.error || `Server error: ${response.status}`);
			}

			if (!response.body) {
				throw new Error('No response body received');
			}

			const reader = response.body.getReader();
			const decoder = new TextDecoder();
			let buffer = '';

			while (true) {
				const { done, value } = await reader.read();

				if (done) break;

				buffer += decoder.decode(value, { stream: true });
				const lines = buffer.split('\n');

				// Keep the last incomplete line in buffer
				buffer = lines.pop() || '';

				for (const line of lines) {
					const event = parseSSELine(line);

					if (!event) continue;

					if (event.event === 'data' && event.data) {
						try {
							// Handle [DONE] signal
							if (event.data === '[DONE]') {
								break;
							}

							const parsed = JSON.parse(event.data);

							if (parsed.content) {
								generatedContent += parsed.content;
							}

							if (parsed.error) {
								throw new Error(parsed.error);
							}

							if (parsed.usage) {
								rateLimit = {
									...rateLimit,
									remaining: parsed.usage.requests_remaining ?? rateLimit.remaining
								};
							}
						} catch (parseError) {
							// If it's not JSON, treat it as raw text content
							if (event.data !== '[DONE]') {
								generatedContent += event.data;
							}
						}
					}

					if (event.event === 'error') {
						throw new Error(event.data || 'Stream error occurred');
					}
				}
			}

			// Process any remaining buffer
			if (buffer.trim()) {
				const event = parseSSELine(buffer);
				if (event?.event === 'data' && event.data && event.data !== '[DONE]') {
					try {
						const parsed = JSON.parse(event.data);
						if (parsed.content) {
							generatedContent += parsed.content;
						}
					} catch {
						generatedContent += event.data;
					}
				}
			}

			retryCount = 0; // Reset retry count on success
		} catch (err) {
			if (err instanceof Error && err.name === 'AbortError') {
				// User cancelled - don't show error
				return;
			}
			throw err;
		} finally {
			isStreaming = false;
			streamController = null;
		}
	}

	// ==========================================================================
	// Fallback Non-Streaming API Call
	// ==========================================================================

	async function nonStreamingGenerate(): Promise<void> {
		const response = await fetch(`${API_BASE_URL}/api/cms/ai/assist`, {
			method: 'POST',
			headers: {
				...getAuthHeaders(),
				Accept: 'application/json'
			},
			body: JSON.stringify(buildRequestBody()),
			credentials: 'include'
		});

		handleRateLimitResponse(response);

		if (!response.ok) {
			if (response.status === 429) {
				throw new Error('Rate limit exceeded. Please wait before trying again.');
			}
			if (response.status === 401) {
				throw new Error('Authentication required. Please log in again.');
			}

			const errorData = await response.json().catch(() => ({}));
			throw new Error(errorData.error || `Server error: ${response.status}`);
		}

		const data: AIAssistResponse = await response.json();

		if (!data.success) {
			throw new Error(data.error || 'Failed to generate content');
		}

		generatedContent = data.content || '';

		if (data.usage) {
			rateLimit = {
				...rateLimit,
				remaining: data.usage.requests_remaining
			};
		}

		retryCount = 0;
	}

	// ==========================================================================
	// Main Generate Handler with Retry Logic
	// ==========================================================================

	async function handleGenerate(): Promise<void> {
		if (!prompt.trim()) {
			error = 'Please enter a prompt';
			return;
		}

		if (rateLimit.isLimited) {
			error = rateLimitMessage();
			return;
		}

		isGenerating = true;
		error = null;
		generatedContent = '';

		const attemptGenerate = async (): Promise<void> => {
			try {
				// Try streaming first
				await streamGenerate();
			} catch (streamError) {
				// If streaming fails, try non-streaming as fallback
				console.warn('Streaming failed, trying non-streaming:', streamError);
				try {
					await nonStreamingGenerate();
				} catch (fallbackError) {
					throw fallbackError;
				}
			}
		};

		try {
			await attemptGenerate();
		} catch (err) {
			const errorMessage = err instanceof Error ? err.message : 'An unexpected error occurred';

			// Check if we should retry
			const isRetryable =
				errorMessage.includes('network') ||
				errorMessage.includes('timeout') ||
				errorMessage.includes('Server error: 5');

			if (isRetryable && retryCount < MAX_RETRIES) {
				retryCount++;
				const retryDelay = BASE_RETRY_DELAY * Math.pow(2, retryCount - 1);

				error = `Request failed. Retrying in ${retryDelay / 1000}s... (Attempt ${retryCount}/${MAX_RETRIES})`;

				await delay(retryDelay);

				// Clear error and retry
				error = null;

				try {
					await attemptGenerate();
				} catch (retryErr) {
					error = retryErr instanceof Error ? retryErr.message : 'Failed after retries';
					console.error('AI generation error after retry:', retryErr);
				}
			} else {
				error = errorMessage;
				console.error('AI generation error:', err);
			}
		} finally {
			isGenerating = false;
		}
	}

	// ==========================================================================
	// Cancel Streaming
	// ==========================================================================

	function handleCancel(): void {
		if (streamController) {
			streamController.abort();
			streamController = null;
		}
		isGenerating = false;
		isStreaming = false;
	}

	// ==========================================================================
	// Other Handlers
	// ==========================================================================

	function handleQuickAction(action: (typeof quickActions)[0]): void {
		prompt = action.prompt + ' ';
		activeTab = 'generate';
	}

	function handleApply(): void {
		if (generatedContent) {
			onapply(generatedContent);
			generatedContent = '';
			prompt = '';
		}
	}

	async function handleCopy(): Promise<void> {
		if (generatedContent) {
			await navigator.clipboard.writeText(generatedContent);
			copied = true;
			setTimeout(() => (copied = false), 2000);
		}
	}

	function handleRegenerate(): void {
		handleGenerate();
	}

	function handleClearError(): void {
		error = null;
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
		{#if rateLimit.remaining < 10}
			<div class="rate-limit-badge" class:warning={rateLimit.remaining <= 3}>
				<IconClock size={14} />
				{rateLimit.remaining} left
			</div>
		{/if}
	</div>

	<!-- Rate Limit Warning -->
	{#if rateLimit.isLimited}
		<div class="rate-limit-warning" transition:slide>
			<IconClock size={16} />
			<span>{rateLimitMessage()}</span>
		</div>
	{/if}

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
					disabled={isGenerating}
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
			onclick={() => (activeTab = 'generate')}
			disabled={isGenerating}
		>
			<IconWand size={16} />
			Generate
		</button>
		<button
			type="button"
			class="ai-tab"
			class:active={activeTab === 'improve'}
			onclick={() => (activeTab = 'improve')}
			disabled={isGenerating}
		>
			<IconSparkles size={16} />
			Improve
		</button>
		<button
			type="button"
			class="ai-tab"
			class:active={activeTab === 'translate'}
			onclick={() => (activeTab = 'translate')}
			disabled={isGenerating}
		>
			<IconLanguage size={16} />
			Translate
		</button>
		<button
			type="button"
			class="ai-tab"
			class:active={activeTab === 'summarize'}
			onclick={() => (activeTab = 'summarize')}
			disabled={isGenerating}
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
						disabled={isGenerating}
					></textarea>
				</div>

				<div class="options-row">
					<div class="option-group">
						<label for="ai-tone-select">Tone</label>
						<select id="ai-tone-select" bind:value={tone} disabled={isGenerating}>
							{#each toneOptions as option}
								<option value={option.value}>{option.label}</option>
							{/each}
						</select>
					</div>
					<div class="option-group">
						<label for="ai-length-select">Length</label>
						<select id="ai-length-select" bind:value={length} disabled={isGenerating}>
							<option value="short">Short (~100 words)</option>
							<option value="medium">Medium (~200 words)</option>
							<option value="long">Long (~400 words)</option>
						</select>
					</div>
				</div>

				<div class="options-row">
					<div class="option-group full">
						<label for="ai-style-select">Style</label>
						<select id="ai-style-select" bind:value={style} disabled={isGenerating}>
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
						disabled={isGenerating}
					></textarea>
				</div>
				<p class="help-text">AI will enhance clarity, fix grammar, and improve readability.</p>
			</div>
		{:else if activeTab === 'translate'}
			<div class="translate-panel" transition:fade={{ duration: 150 }}>
				<div class="prompt-input">
					<textarea
						bind:value={prompt}
						placeholder="Paste the text you want to translate..."
						rows="6"
						disabled={isGenerating}
					></textarea>
				</div>
				<div class="options-row">
					<div class="option-group full">
						<label for="ai-language-select">Translate to</label>
						<select id="ai-language-select" bind:value={targetLanguage} disabled={isGenerating}>
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
						disabled={isGenerating}
					></textarea>
				</div>
				<p class="help-text">AI will create a concise summary with key takeaways.</p>
			</div>
		{/if}

		<!-- Generate Button -->
		<div class="button-row">
			<button
				type="button"
				class="generate-btn"
				onclick={handleGenerate}
				disabled={!canGenerate}
			>
				{#if isGenerating}
					<IconLoader size={18} class="spin" />
					{isStreaming ? 'Streaming...' : 'Generating...'}
				{:else}
					<IconSend size={18} />
					Generate
				{/if}
			</button>

			{#if isGenerating}
				<button type="button" class="cancel-btn" onclick={handleCancel} title="Cancel generation">
					<IconX size={18} />
					Cancel
				</button>
			{/if}
		</div>

		<!-- Error -->
		{#if error}
			<div class="error-message" transition:slide>
				<IconAlertCircle size={16} />
				<span>{error}</span>
				<button type="button" class="error-dismiss" onclick={handleClearError} title="Dismiss">
					<IconX size={14} />
				</button>
			</div>
		{/if}

		<!-- Generated Content -->
		{#if generatedContent}
			<div class="generated-content" transition:slide>
				<div class="content-header">
					<span>Generated Content</span>
					{#if isStreaming}
						<span class="streaming-indicator">
							<IconLoader size={14} class="spin" />
							Streaming...
						</span>
					{/if}
					<div class="content-actions">
						<button
							type="button"
							class="action-btn"
							onclick={handleRegenerate}
							title="Regenerate"
							disabled={isGenerating}
						>
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
					{#if isStreaming}
						<span class="cursor">|</span>
					{/if}
				</div>
				<button type="button" class="apply-btn" onclick={handleApply} disabled={isStreaming}>
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

	.ai-title {
		flex: 1;
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

	.rate-limit-badge {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.25rem 0.5rem;
		background: #f0f9ff;
		border: 1px solid #bae6fd;
		border-radius: 12px;
		font-size: 0.6875rem;
		color: #0369a1;
	}

	.rate-limit-badge.warning {
		background: #fef3c7;
		border-color: #fcd34d;
		color: #92400e;
	}

	.rate-limit-warning {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem;
		background: #fef3c7;
		border: 1px solid #fcd34d;
		border-radius: 8px;
		color: #92400e;
		font-size: 0.8125rem;
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

	.quick-btn:hover:not(:disabled) {
		border-color: #8b5cf6;
		color: #8b5cf6;
	}

	.quick-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
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

	.ai-tab:hover:not(:disabled) {
		color: #1a1a1a;
	}

	.ai-tab:disabled {
		opacity: 0.5;
		cursor: not-allowed;
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

	.prompt-input textarea:disabled {
		background: #f9fafb;
		cursor: not-allowed;
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

	.option-group select:disabled {
		background: #f9fafb;
		cursor: not-allowed;
	}

	.help-text {
		font-size: 0.75rem;
		color: #666;
		margin: 0;
	}

	.button-row {
		display: flex;
		gap: 0.5rem;
	}

	.generate-btn {
		flex: 1;
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

	.cancel-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.375rem;
		padding: 0.75rem 1rem;
		background: #f1f5f9;
		color: #64748b;
		border: 1px solid #e2e8f0;
		border-radius: 8px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s;
	}

	.cancel-btn:hover {
		background: #e2e8f0;
		color: #475569;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
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

	.error-message span {
		flex: 1;
	}

	.error-dismiss {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 20px;
		height: 20px;
		padding: 0;
		background: transparent;
		border: none;
		border-radius: 4px;
		color: #dc2626;
		cursor: pointer;
		opacity: 0.7;
		transition: opacity 0.15s;
	}

	.error-dismiss:hover {
		opacity: 1;
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

	.streaming-indicator {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		color: #8b5cf6;
	}

	.streaming-indicator :global(.spin) {
		animation: spin 1s linear infinite;
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

	.action-btn:hover:not(:disabled) {
		background: #e5e5e5;
		color: #1a1a1a;
	}

	.action-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.content-body {
		padding: 1rem;
		font-size: 0.875rem;
		line-height: 1.6;
		max-height: 300px;
		overflow-y: auto;
		white-space: pre-wrap;
	}

	.cursor {
		animation: blink 1s step-end infinite;
		color: #8b5cf6;
	}

	@keyframes blink {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0;
		}
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

	.apply-btn:hover:not(:disabled) {
		background: #059669;
	}

	.apply-btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}
</style>

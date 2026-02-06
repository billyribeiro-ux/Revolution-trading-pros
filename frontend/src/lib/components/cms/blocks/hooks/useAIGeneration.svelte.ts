/**
 * AI Generation Hook
 * ═══════════════════════════════════════════════════════════════════════════
 * Composable logic for AI content generation, summarization, and translation
 */

import {
	getBlockStateManager,
	type BlockId,
	type AIBlockState
} from '$lib/stores/blockState.svelte';

// ============================================================================
// Types
// ============================================================================

export type AIGenerationType = 'generate' | 'summary' | 'translation';

export interface AIGenerationOptions {
	blockId: BlockId;
	type: AIGenerationType;
	onSuccess?: (output: string) => void;
	onError?: (error: Error) => void;
}

export interface AIGenerationConfig {
	model?: string;
	temperature?: number;
	maxTokens?: number;
	tone?: 'professional' | 'casual' | 'formal' | 'friendly' | 'persuasive';
	length?: 'short' | 'medium' | 'long';
}

export interface TranslationConfig extends AIGenerationConfig {
	sourceLanguage?: string;
	targetLanguage: string;
}

interface AIAPIRequest {
	prompt: string;
	model: string;
	type: AIGenerationType;
	config?: AIGenerationConfig | TranslationConfig;
}

interface AIAPIResponse {
	success: boolean;
	output?: string;
	error?: string;
	usage?: {
		promptTokens: number;
		completionTokens: number;
		totalTokens: number;
	};
}

// ============================================================================
// Constants
// ============================================================================

const AI_API_ENDPOINT = '/api/ai/generate';
const DEFAULT_MODEL = 'gpt-4';
const DEFAULT_TIMEOUT = 60000; // 60 seconds
const MAX_RETRIES = 3;
const RETRY_DELAY = 1000;

// ============================================================================
// Hook Implementation
// ============================================================================

export function useAIGeneration(options: AIGenerationOptions) {
	const stateManager = getBlockStateManager();
	const { blockId, type, onSuccess, onError } = options;

	// Get reactive state from manager based on type
	let state = $derived.by((): AIBlockState => {
		switch (type) {
			case 'generate':
				return stateManager.getAIGeneratedState(blockId);
			case 'summary':
				return stateManager.getAISummaryState(blockId);
			case 'translation':
				return stateManager.getAITranslationState(blockId);
			default:
				return stateManager.getAIGeneratedState(blockId);
		}
	});

	// Abort controller for cancellation
	let abortController = $state<AbortController | null>(null);

	// ========================================================================
	// State Setters
	// ========================================================================

	function setState(updates: Partial<AIBlockState>): void {
		switch (type) {
			case 'generate':
				stateManager.setAIGeneratedState(blockId, updates);
				break;
			case 'summary':
				stateManager.setAISummaryState(blockId, updates);
				break;
			case 'translation':
				stateManager.setAITranslationState(blockId, updates);
				break;
		}
	}

	// ========================================================================
	// API Call with Retry Logic
	// ========================================================================

	async function callAPI(
		request: AIAPIRequest,
		signal: AbortSignal,
		retryCount = 0
	): Promise<AIAPIResponse> {
		try {
			const response = await fetch(AI_API_ENDPOINT, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(request),
				signal
			});

			if (!response.ok) {
				// Handle rate limiting
				if (response.status === 429 && retryCount < MAX_RETRIES) {
					const retryAfter = response.headers.get('Retry-After');
					const delay = retryAfter
						? parseInt(retryAfter, 10) * 1000
						: RETRY_DELAY * (retryCount + 1);
					await sleep(delay);
					return callAPI(request, signal, retryCount + 1);
				}

				const errorData = await response.json().catch(() => ({}));
				throw new Error(errorData.error || `API error: ${response.status}`);
			}

			return await response.json();
		} catch (error) {
			if (error instanceof Error && error.name === 'AbortError') {
				throw error;
			}

			// Retry on network errors
			if (retryCount < MAX_RETRIES) {
				await sleep(RETRY_DELAY * (retryCount + 1));
				return callAPI(request, signal, retryCount + 1);
			}

			throw error;
		}
	}

	// ========================================================================
	// Generation Methods
	// ========================================================================

	async function generate(prompt: string, config: AIGenerationConfig = {}): Promise<void> {
		if (!prompt.trim()) {
			const error = new Error('Prompt cannot be empty');
			onError?.(error);
			return;
		}

		// Cancel any existing request
		if (abortController) {
			abortController.abort();
		}

		abortController = new AbortController();
		const timeoutId = setTimeout(() => abortController?.abort(), DEFAULT_TIMEOUT);

		setState({ loading: true, error: null });

		try {
			const request: AIAPIRequest = {
				prompt: prompt.trim(),
				model: config.model || DEFAULT_MODEL,
				type,
				config
			};

			const response = await callAPI(request, abortController.signal);

			if (response.success && response.output) {
				setState({
					loading: false,
					output: response.output,
					error: null,
					lastGenerated: Date.now()
				});
				onSuccess?.(response.output);
			} else {
				throw new Error(response.error || 'Failed to generate content');
			}
		} catch (error) {
			if (error instanceof Error && error.name === 'AbortError') {
				setState({ loading: false, error: 'Request cancelled' });
				return;
			}

			const errorMessage = error instanceof Error ? error.message : 'An unexpected error occurred';
			setState({ loading: false, error: errorMessage });
			onError?.(error instanceof Error ? error : new Error(errorMessage));
		} finally {
			clearTimeout(timeoutId);
			abortController = null;
		}
	}

	async function summarize(
		sourceText: string,
		length: 'short' | 'medium' | 'long' = 'medium'
	): Promise<void> {
		if (!sourceText.trim()) {
			const error = new Error('Source text cannot be empty');
			onError?.(error);
			return;
		}

		const lengthPrompts = {
			short: 'Provide a brief 1-2 sentence summary.',
			medium: 'Provide a concise summary in 3-4 sentences.',
			long: 'Provide a detailed summary in 5-7 sentences covering all key points.'
		};

		const prompt = `Summarize the following text. ${lengthPrompts[length]}\n\nText:\n${sourceText}`;

		await generate(prompt, { length });
	}

	async function translate(sourceText: string, config: TranslationConfig): Promise<void> {
		if (!sourceText.trim()) {
			const error = new Error('Source text cannot be empty');
			onError?.(error);
			return;
		}

		if (!config.targetLanguage) {
			const error = new Error('Target language is required');
			onError?.(error);
			return;
		}

		// Update translation-specific state
		if (type === 'translation') {
			stateManager.setAITranslationState(blockId, {
				sourceText,
				targetLanguage: config.targetLanguage
			});
		}

		const sourceLanguageClause = config.sourceLanguage ? `from ${config.sourceLanguage} ` : '';

		const prompt = `Translate the following text ${sourceLanguageClause}to ${config.targetLanguage}. Preserve the original formatting and tone.\n\nText:\n${sourceText}`;

		await generate(prompt, config);
	}

	// ========================================================================
	// Control Methods
	// ========================================================================

	function cancel(): void {
		if (abortController) {
			abortController.abort();
			abortController = null;
		}
		setState({ loading: false });
	}

	function clear(): void {
		cancel();
		setState({
			loading: false,
			error: null,
			output: null,
			lastGenerated: null
		});

		if (type === 'translation') {
			stateManager.setAITranslationState(blockId, {
				sourceText: '',
				targetLanguage: 'es',
				view: 'stacked'
			});
		}
	}

	function retry(prompt: string, config?: AIGenerationConfig): void {
		if (state.error) {
			generate(prompt, config);
		}
	}

	// ========================================================================
	// Utility Methods
	// ========================================================================

	function setTranslationView(view: 'stacked' | 'side-by-side'): void {
		if (type === 'translation') {
			stateManager.setAITranslationState(blockId, { view });
		}
	}

	function copyToClipboard(): Promise<boolean> {
		if (!state.output) {
			return Promise.resolve(false);
		}

		return navigator.clipboard
			.writeText(state.output)
			.then(() => true)
			.catch(() => false);
	}

	// ========================================================================
	// Cleanup
	// ========================================================================

	stateManager.registerCleanup(blockId, () => {
		if (abortController) {
			abortController.abort();
		}
	});

	// ========================================================================
	// Return API
	// ========================================================================

	return {
		// Reactive state (via getters)
		get loading() {
			return state?.loading ?? false;
		},
		get error() {
			return state?.error ?? null;
		},
		get output() {
			return state?.output ?? null;
		},
		get lastGenerated() {
			return state?.lastGenerated ?? null;
		},

		// Translation-specific state
		get translationView() {
			if (type === 'translation') {
				const translationState = stateManager.getAITranslationState(blockId);
				return translationState.view;
			}
			return 'stacked';
		},
		get sourceText() {
			if (type === 'translation') {
				const translationState = stateManager.getAITranslationState(blockId);
				return translationState.sourceText;
			}
			return '';
		},
		get targetLanguage() {
			if (type === 'translation') {
				const translationState = stateManager.getAITranslationState(blockId);
				return translationState.targetLanguage;
			}
			return 'es';
		},

		// Generation methods
		generate,
		summarize,
		translate,

		// Control methods
		cancel,
		clear,
		retry,

		// Utility methods
		setTranslationView,
		copyToClipboard
	};
}

// ============================================================================
// Helper Functions
// ============================================================================

function sleep(ms: number): Promise<void> {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

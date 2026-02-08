/**
 * useAIGeneration Hook - Comprehensive Test Suite
 * ===============================================================================
 * Unit tests for AI content generation, summarization, and translation
 *
 * @version 2.0.0
 */

// Import setup first to initialize mocks
import './setup';

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';

// ============================================================================
// Mock Dependencies
// ============================================================================

// Create a mock state manager
const mockStateManager = {
	getAIGeneratedState: vi.fn(() => ({
		loading: false,
		error: null,
		output: null,
		lastGenerated: null
	})),
	setAIGeneratedState: vi.fn(),
	getAISummaryState: vi.fn(() => ({
		loading: false,
		error: null,
		output: null,
		lastGenerated: null
	})),
	setAISummaryState: vi.fn(),
	getAITranslationState: vi.fn(() => ({
		loading: false,
		error: null,
		output: null,
		lastGenerated: null,
		sourceText: '',
		targetLanguage: 'es',
		view: 'stacked' as const
	})),
	setAITranslationState: vi.fn(),
	registerCleanup: vi.fn()
};

// Mock the blockState module
vi.mock('$lib/stores/blockState.svelte', () => ({
	getBlockStateManager: () => mockStateManager,
	createBlockId: (id: string) => id
}));

// Import the hook after mocking
import { useAIGeneration, type AIGenerationType } from '../useAIGeneration.svelte';

// ============================================================================
// Test Setup
// ============================================================================

describe('useAIGeneration', () => {
	const mockBlockId = 'test-block-123' as unknown as Parameters<
		typeof useAIGeneration
	>[0]['blockId'];

	beforeEach(() => {
		vi.clearAllMocks();
		vi.useFakeTimers();

		// Reset mock state
		mockStateManager.getAIGeneratedState.mockReturnValue({
			loading: false,
			error: null,
			output: null,
			lastGenerated: null
		});
		mockStateManager.getAISummaryState.mockReturnValue({
			loading: false,
			error: null,
			output: null,
			lastGenerated: null
		});
		mockStateManager.getAITranslationState.mockReturnValue({
			loading: false,
			error: null,
			output: null,
			lastGenerated: null,
			sourceText: '',
			targetLanguage: 'es',
			view: 'stacked' as const
		});

		// Mock fetch
		global.fetch = vi.fn();
	});

	afterEach(() => {
		vi.useRealTimers();
		vi.restoreAllMocks();
	});

	// ========================================================================
	// Initialization Tests
	// ========================================================================

	describe('Initialization', () => {
		it('initializes with default state', () => {
			const onSuccess = vi.fn();
			const hook = useAIGeneration({
				blockId: mockBlockId,
				type: 'generate',
				onSuccess
			});

			expect(hook.loading).toBe(false);
			expect(hook.error).toBeNull();
			expect(hook.output).toBeNull();
			expect(hook.lastGenerated).toBeNull();
		});

		it('registers cleanup callback on creation', () => {
			void useAIGeneration({
				blockId: mockBlockId,
				type: 'generate',
				onSuccess: vi.fn()
			});

			expect(mockStateManager.registerCleanup).toHaveBeenCalledWith(
				mockBlockId,
				expect.any(Function)
			);
		});

		it('provides all expected methods', () => {
			const hook = useAIGeneration({
				blockId: mockBlockId,
				type: 'generate',
				onSuccess: vi.fn()
			});

			expect(typeof hook.generate).toBe('function');
			expect(typeof hook.summarize).toBe('function');
			expect(typeof hook.translate).toBe('function');
			expect(typeof hook.cancel).toBe('function');
			expect(typeof hook.clear).toBe('function');
			expect(typeof hook.retry).toBe('function');
			expect(typeof hook.copyToClipboard).toBe('function');
		});
	});

	// ========================================================================
	// Loading State Tests
	// ========================================================================

	describe('Loading State', () => {
		it('sets loading during generation', async () => {
			// Mock fetch to delay response
			(global.fetch as ReturnType<typeof vi.fn>).mockImplementation(
				() =>
					new Promise((resolve) =>
						setTimeout(
							() =>
								resolve({
									ok: true,
									json: () => Promise.resolve({ success: true, output: 'Generated text' })
								}),
							100
						)
					)
			);

			const hook = useAIGeneration({
				blockId: mockBlockId,
				type: 'generate',
				onSuccess: vi.fn()
			});

			const generatePromise = hook.generate('Test prompt');

			// Should set loading to true
			expect(mockStateManager.setAIGeneratedState).toHaveBeenCalledWith(
				mockBlockId,
				expect.objectContaining({ loading: true, error: null })
			);

			// Advance timers and wait
			await vi.advanceTimersByTimeAsync(150);
			await generatePromise;
		});

		it('clears loading on success', async () => {
			(global.fetch as ReturnType<typeof vi.fn>).mockResolvedValue({
				ok: true,
				json: () => Promise.resolve({ success: true, output: 'Generated text' })
			});

			const hook = useAIGeneration({
				blockId: mockBlockId,
				type: 'generate',
				onSuccess: vi.fn()
			});

			await hook.generate('Test prompt');

			// Should have been called with loading: false
			expect(mockStateManager.setAIGeneratedState).toHaveBeenCalledWith(
				mockBlockId,
				expect.objectContaining({ loading: false, output: 'Generated text' })
			);
		});

		it('clears loading on error', async () => {
			(global.fetch as ReturnType<typeof vi.fn>).mockResolvedValue({
				ok: false,
				status: 500,
				json: () => Promise.resolve({ error: 'Server error' })
			});

			const hook = useAIGeneration({
				blockId: mockBlockId,
				type: 'generate',
				onSuccess: vi.fn(),
				onError: vi.fn()
			});

			// Start generate without awaiting — retry sleep() uses setTimeout
			const promise = hook.generate('Test prompt');
			await vi.runAllTimersAsync();
			await promise;

			// Should have been called with loading: false
			expect(mockStateManager.setAIGeneratedState).toHaveBeenCalledWith(
				mockBlockId,
				expect.objectContaining({ loading: false })
			);
		});
	});

	// ========================================================================
	// Success Handling Tests
	// ========================================================================

	describe('Successful Generation', () => {
		it('handles successful generation', async () => {
			const generatedOutput = 'This is AI generated content';
			(global.fetch as ReturnType<typeof vi.fn>).mockResolvedValue({
				ok: true,
				json: () => Promise.resolve({ success: true, output: generatedOutput })
			});

			const hook = useAIGeneration({
				blockId: mockBlockId,
				type: 'generate',
				onSuccess: vi.fn()
			});

			await hook.generate('Test prompt');

			expect(mockStateManager.setAIGeneratedState).toHaveBeenCalledWith(
				mockBlockId,
				expect.objectContaining({
					loading: false,
					output: generatedOutput,
					error: null,
					lastGenerated: expect.any(Number)
				})
			);
		});

		it('calls onSuccess callback with output', async () => {
			const generatedOutput = 'This is AI generated content';
			(global.fetch as ReturnType<typeof vi.fn>).mockResolvedValue({
				ok: true,
				json: () => Promise.resolve({ success: true, output: generatedOutput })
			});

			const onSuccess = vi.fn();
			const hook = useAIGeneration({
				blockId: mockBlockId,
				type: 'generate',
				onSuccess
			});

			await hook.generate('Test prompt');

			expect(onSuccess).toHaveBeenCalledWith(generatedOutput);
		});

		it('stores lastGenerated timestamp', async () => {
			const now = Date.now();
			vi.setSystemTime(now);

			(global.fetch as ReturnType<typeof vi.fn>).mockResolvedValue({
				ok: true,
				json: () => Promise.resolve({ success: true, output: 'Content' })
			});

			const hook = useAIGeneration({
				blockId: mockBlockId,
				type: 'generate',
				onSuccess: vi.fn()
			});

			await hook.generate('Test prompt');

			expect(mockStateManager.setAIGeneratedState).toHaveBeenCalledWith(
				mockBlockId,
				expect.objectContaining({ lastGenerated: now })
			);
		});
	});

	// ========================================================================
	// Error Handling Tests
	// ========================================================================

	describe('Error Handling', () => {
		it('handles API errors', async () => {
			const errorMessage = 'Rate limit exceeded';
			(global.fetch as ReturnType<typeof vi.fn>).mockResolvedValue({
				ok: false,
				status: 429,
				headers: new Headers(),
				json: () => Promise.resolve({ error: errorMessage })
			});

			const onError = vi.fn();
			const hook = useAIGeneration({
				blockId: mockBlockId,
				type: 'generate',
				onSuccess: vi.fn(),
				onError
			});

			// Start generate without awaiting — retry sleep() uses setTimeout
			const promise = hook.generate('Test prompt');

			// After 3 retries (with delays), should eventually fail
			await vi.runAllTimersAsync();
			await promise;

			expect(mockStateManager.setAIGeneratedState).toHaveBeenCalledWith(
				mockBlockId,
				expect.objectContaining({ loading: false })
			);
		});

		it('handles network errors', async () => {
			(global.fetch as ReturnType<typeof vi.fn>).mockRejectedValue(new Error('Network error'));

			const onError = vi.fn();
			const hook = useAIGeneration({
				blockId: mockBlockId,
				type: 'generate',
				onSuccess: vi.fn(),
				onError
			});

			const promise = hook.generate('Test prompt');
			await vi.runAllTimersAsync();
			await promise;

			expect(onError).toHaveBeenCalled();
		});

		it('calls onError callback with error', async () => {
			(global.fetch as ReturnType<typeof vi.fn>).mockResolvedValue({
				ok: false,
				status: 500,
				json: () => Promise.resolve({})
			});

			const onError = vi.fn();
			const hook = useAIGeneration({
				blockId: mockBlockId,
				type: 'generate',
				onSuccess: vi.fn(),
				onError
			});

			const promise = hook.generate('Test prompt');
			await vi.runAllTimersAsync();
			await promise;

			expect(onError).toHaveBeenCalledWith(expect.any(Error));
		});

		it('handles empty prompt error', async () => {
			const onError = vi.fn();
			const hook = useAIGeneration({
				blockId: mockBlockId,
				type: 'generate',
				onSuccess: vi.fn(),
				onError
			});

			await hook.generate('   ');

			expect(onError).toHaveBeenCalledWith(expect.any(Error));
			expect(global.fetch).not.toHaveBeenCalled();
		});
	});

	// ========================================================================
	// Clear State Tests
	// ========================================================================

	describe('Clear State', () => {
		it('clears state on clear()', () => {
			const hook = useAIGeneration({
				blockId: mockBlockId,
				type: 'generate',
				onSuccess: vi.fn()
			});

			hook.clear();

			expect(mockStateManager.setAIGeneratedState).toHaveBeenCalledWith(
				mockBlockId,
				expect.objectContaining({
					loading: false,
					error: null,
					output: null,
					lastGenerated: null
				})
			);
		});

		it('cancels pending request on clear()', async () => {
			// Create a long-running request
			(global.fetch as ReturnType<typeof vi.fn>).mockImplementation(
				() =>
					new Promise((resolve) =>
						setTimeout(
							() =>
								resolve({
									ok: true,
									json: () => Promise.resolve({ success: true, output: 'Content' })
								}),
							5000
						)
					)
			);

			const hook = useAIGeneration({
				blockId: mockBlockId,
				type: 'generate',
				onSuccess: vi.fn()
			});

			hook.generate('Test prompt');
			hook.clear();

			// Verify state was cleared
			expect(mockStateManager.setAIGeneratedState).toHaveBeenCalledWith(
				mockBlockId,
				expect.objectContaining({ loading: false })
			);
		});
	});

	// ========================================================================
	// Generation Type Tests
	// ========================================================================

	describe('Generation Types', () => {
		it('supports generate type', () => {
			void useAIGeneration({
				blockId: mockBlockId,
				type: 'generate',
				onSuccess: vi.fn()
			});

			expect(mockStateManager.getAIGeneratedState).toHaveBeenCalledWith(mockBlockId);
		});

		it('supports summary type', () => {
			void useAIGeneration({
				blockId: mockBlockId,
				type: 'summary',
				onSuccess: vi.fn()
			});

			expect(mockStateManager.getAISummaryState).toHaveBeenCalledWith(mockBlockId);
		});

		it('supports translation type', () => {
			void useAIGeneration({
				blockId: mockBlockId,
				type: 'translation',
				onSuccess: vi.fn()
			});

			expect(mockStateManager.getAITranslationState).toHaveBeenCalledWith(mockBlockId);
		});

		it('uses correct state setter for generate type', async () => {
			(global.fetch as ReturnType<typeof vi.fn>).mockResolvedValue({
				ok: true,
				json: () => Promise.resolve({ success: true, output: 'Content' })
			});

			const hook = useAIGeneration({
				blockId: mockBlockId,
				type: 'generate',
				onSuccess: vi.fn()
			});

			await hook.generate('Test prompt');

			expect(mockStateManager.setAIGeneratedState).toHaveBeenCalled();
			expect(mockStateManager.setAISummaryState).not.toHaveBeenCalled();
		});

		it('uses correct state setter for summary type', async () => {
			(global.fetch as ReturnType<typeof vi.fn>).mockResolvedValue({
				ok: true,
				json: () => Promise.resolve({ success: true, output: 'Summary' })
			});

			const hook = useAIGeneration({
				blockId: mockBlockId,
				type: 'summary',
				onSuccess: vi.fn()
			});

			await hook.summarize('Long text to summarize');

			expect(mockStateManager.setAISummaryState).toHaveBeenCalled();
		});

		it('uses correct state setter for translation type', async () => {
			(global.fetch as ReturnType<typeof vi.fn>).mockResolvedValue({
				ok: true,
				json: () => Promise.resolve({ success: true, output: 'Translated' })
			});

			const hook = useAIGeneration({
				blockId: mockBlockId,
				type: 'translation',
				onSuccess: vi.fn()
			});

			await hook.translate('Text to translate', { targetLanguage: 'es' });

			expect(mockStateManager.setAITranslationState).toHaveBeenCalled();
		});
	});

	// ========================================================================
	// Summarize Tests
	// ========================================================================

	describe('Summarize', () => {
		it('generates summary with short length', async () => {
			(global.fetch as ReturnType<typeof vi.fn>).mockResolvedValue({
				ok: true,
				json: () => Promise.resolve({ success: true, output: 'Short summary' })
			});

			const hook = useAIGeneration({
				blockId: mockBlockId,
				type: 'summary',
				onSuccess: vi.fn()
			});

			await hook.summarize('Long text content here', 'short');

			expect(global.fetch).toHaveBeenCalledWith(
				'/api/ai/generate',
				expect.objectContaining({
					method: 'POST',
					body: expect.stringContaining('1-2 sentence')
				})
			);
		});

		it('handles empty source text', async () => {
			const onError = vi.fn();
			const hook = useAIGeneration({
				blockId: mockBlockId,
				type: 'summary',
				onSuccess: vi.fn(),
				onError
			});

			await hook.summarize('');

			expect(onError).toHaveBeenCalledWith(expect.any(Error));
			expect(global.fetch).not.toHaveBeenCalled();
		});
	});

	// ========================================================================
	// Translation Tests
	// ========================================================================

	describe('Translation', () => {
		it('translates with target language', async () => {
			(global.fetch as ReturnType<typeof vi.fn>).mockResolvedValue({
				ok: true,
				json: () => Promise.resolve({ success: true, output: 'Texto traducido' })
			});

			const hook = useAIGeneration({
				blockId: mockBlockId,
				type: 'translation',
				onSuccess: vi.fn()
			});

			await hook.translate('Text to translate', { targetLanguage: 'Spanish' });

			expect(global.fetch).toHaveBeenCalledWith(
				'/api/ai/generate',
				expect.objectContaining({
					method: 'POST',
					body: expect.stringContaining('Spanish')
				})
			);
		});

		it('translates with source and target language', async () => {
			(global.fetch as ReturnType<typeof vi.fn>).mockResolvedValue({
				ok: true,
				json: () => Promise.resolve({ success: true, output: 'Translated' })
			});

			const hook = useAIGeneration({
				blockId: mockBlockId,
				type: 'translation',
				onSuccess: vi.fn()
			});

			await hook.translate('Hello', { sourceLanguage: 'English', targetLanguage: 'French' });

			expect(global.fetch).toHaveBeenCalledWith(
				'/api/ai/generate',
				expect.objectContaining({
					body: expect.stringContaining('from English')
				})
			);
		});

		it('updates translation-specific state', async () => {
			(global.fetch as ReturnType<typeof vi.fn>).mockResolvedValue({
				ok: true,
				json: () => Promise.resolve({ success: true, output: 'Translated' })
			});

			const hook = useAIGeneration({
				blockId: mockBlockId,
				type: 'translation',
				onSuccess: vi.fn()
			});

			await hook.translate('Hello world', { targetLanguage: 'Spanish' });

			expect(mockStateManager.setAITranslationState).toHaveBeenCalledWith(
				mockBlockId,
				expect.objectContaining({
					sourceText: 'Hello world',
					targetLanguage: 'Spanish'
				})
			);
		});

		it('handles missing target language', async () => {
			const onError = vi.fn();
			const hook = useAIGeneration({
				blockId: mockBlockId,
				type: 'translation',
				onSuccess: vi.fn(),
				onError
			});

			await hook.translate('Text', { targetLanguage: '' });

			expect(onError).toHaveBeenCalledWith(expect.any(Error));
		});

		it('provides translation view getter', () => {
			const hook = useAIGeneration({
				blockId: mockBlockId,
				type: 'translation',
				onSuccess: vi.fn()
			});

			expect(hook.translationView).toBe('stacked');
		});

		it('can set translation view', () => {
			const hook = useAIGeneration({
				blockId: mockBlockId,
				type: 'translation',
				onSuccess: vi.fn()
			});

			hook.setTranslationView('side-by-side');

			expect(mockStateManager.setAITranslationState).toHaveBeenCalledWith(mockBlockId, {
				view: 'side-by-side'
			});
		});
	});

	// ========================================================================
	// Cancel Tests
	// ========================================================================

	describe('Cancel', () => {
		it('cancels pending request', async () => {
			let abortSignal: AbortSignal | undefined;

			(global.fetch as ReturnType<typeof vi.fn>).mockImplementation((_url, options) => {
				abortSignal = options?.signal;
				return new Promise((resolve, reject) => {
					options?.signal?.addEventListener('abort', () => {
						reject(new DOMException('Aborted', 'AbortError'));
					});
					setTimeout(
						() =>
							resolve({
								ok: true,
								json: () => Promise.resolve({ success: true, output: 'Content' })
							}),
						5000
					);
				});
			});

			const hook = useAIGeneration({
				blockId: mockBlockId,
				type: 'generate',
				onSuccess: vi.fn()
			});

			hook.generate('Test prompt');
			hook.cancel();

			expect(abortSignal?.aborted).toBe(true);
		});

		it('sets loading to false on cancel', () => {
			const hook = useAIGeneration({
				blockId: mockBlockId,
				type: 'generate',
				onSuccess: vi.fn()
			});

			hook.cancel();

			expect(mockStateManager.setAIGeneratedState).toHaveBeenCalledWith(
				mockBlockId,
				expect.objectContaining({ loading: false })
			);
		});
	});

	// ========================================================================
	// Retry Tests
	// ========================================================================

	describe('Retry', () => {
		it('retries generation when in error state', async () => {
			// First set up an error state
			mockStateManager.getAIGeneratedState.mockReturnValue({
				loading: false,
				error: 'Previous error',
				output: null,
				lastGenerated: null
			} as any);

			(global.fetch as ReturnType<typeof vi.fn>).mockResolvedValue({
				ok: true,
				json: () => Promise.resolve({ success: true, output: 'Retry success' })
			});

			const hook = useAIGeneration({
				blockId: mockBlockId,
				type: 'generate',
				onSuccess: vi.fn()
			});

			hook.retry('New prompt');

			expect(global.fetch).toHaveBeenCalled();
		});

		it('does not retry when not in error state', async () => {
			mockStateManager.getAIGeneratedState.mockReturnValue({
				loading: false,
				error: null,
				output: 'Previous output',
				lastGenerated: Date.now()
			} as any);

			const hook = useAIGeneration({
				blockId: mockBlockId,
				type: 'generate',
				onSuccess: vi.fn()
			});

			hook.retry('New prompt');

			expect(global.fetch).not.toHaveBeenCalled();
		});
	});

	// ========================================================================
	// Clipboard Tests
	// ========================================================================

	describe('Copy to Clipboard', () => {
		it('copies output to clipboard', async () => {
			// Re-mock clipboard since mockReset: true resets it between tests
			Object.assign(navigator, {
				clipboard: {
					writeText: vi.fn().mockResolvedValue(undefined),
					readText: vi.fn().mockResolvedValue('')
				}
			});

			mockStateManager.getAIGeneratedState.mockReturnValue({
				loading: false,
				error: null,
				output: 'Content to copy',
				lastGenerated: Date.now()
			} as any);

			const hook = useAIGeneration({
				blockId: mockBlockId,
				type: 'generate',
				onSuccess: vi.fn()
			});

			const result = await hook.copyToClipboard();

			expect(navigator.clipboard.writeText).toHaveBeenCalledWith('Content to copy');
			expect(result).toBe(true);
		});

		it('returns false when no output', async () => {
			mockStateManager.getAIGeneratedState.mockReturnValue({
				loading: false,
				error: null,
				output: null,
				lastGenerated: null
			});

			const hook = useAIGeneration({
				blockId: mockBlockId,
				type: 'generate',
				onSuccess: vi.fn()
			});

			const result = await hook.copyToClipboard();

			expect(result).toBe(false);
		});
	});

	// ========================================================================
	// API Request Tests
	// ========================================================================

	describe('API Request', () => {
		it('sends correct request format', async () => {
			(global.fetch as ReturnType<typeof vi.fn>).mockResolvedValue({
				ok: true,
				json: () => Promise.resolve({ success: true, output: 'Content' })
			});

			const hook = useAIGeneration({
				blockId: mockBlockId,
				type: 'generate',
				onSuccess: vi.fn()
			});

			await hook.generate('Test prompt', { model: 'gpt-4', temperature: 0.7 });

			expect(global.fetch).toHaveBeenCalledWith(
				'/api/ai/generate',
				expect.objectContaining({
					method: 'POST',
					headers: { 'Content-Type': 'application/json' }
				})
			);

			const body = JSON.parse((global.fetch as ReturnType<typeof vi.fn>).mock.calls[0][1].body);
			expect(body.prompt).toBe('Test prompt');
			expect(body.model).toBe('gpt-4');
			expect(body.type).toBe('generate');
		});

		it('uses default model when not specified', async () => {
			(global.fetch as ReturnType<typeof vi.fn>).mockResolvedValue({
				ok: true,
				json: () => Promise.resolve({ success: true, output: 'Content' })
			});

			const hook = useAIGeneration({
				blockId: mockBlockId,
				type: 'generate',
				onSuccess: vi.fn()
			});

			await hook.generate('Test prompt');

			const body = JSON.parse((global.fetch as ReturnType<typeof vi.fn>).mock.calls[0][1].body);
			expect(body.model).toBe('gpt-4');
		});
	});
});

// ============================================================================
// Type Tests
// ============================================================================

describe('AIGenerationType', () => {
	it('accepts generate type', () => {
		const type: AIGenerationType = 'generate';
		expect(type).toBe('generate');
	});

	it('accepts summary type', () => {
		const type: AIGenerationType = 'summary';
		expect(type).toBe('summary');
	});

	it('accepts translation type', () => {
		const type: AIGenerationType = 'translation';
		expect(type).toBe('translation');
	});
});

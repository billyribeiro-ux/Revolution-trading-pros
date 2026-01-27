<!--
/**
 * AI Assist Floating Menu - CMS Editor Enhancement
 * ═══════════════════════════════════════════════════════════════════════════
 * Floating menu that appears on text selection with AI-powered actions
 *
 * FEATURES:
 * - Floating menu positioned near text selection
 * - Multiple AI actions (Improve, Shorten, Expand, Fix Grammar, Summarize, FAQ)
 * - Tone change dropdown (professional, casual, friendly, authoritative, educational)
 * - Loading state with spinner during AI processing
 * - Streaming support for longer content (SSE)
 * - Result preview with Apply/Discard buttons
 * - Character count change display (+X or -Y)
 * - Keyboard shortcut hints on hover
 * - Glassmorphism design with smooth animations
 * - Full accessibility support (ARIA)
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */
-->

<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { fade, fly, scale } from 'svelte/transition';
	import { quintOut, cubicOut } from 'svelte/easing';
	import { browser } from '$app/environment';
	import {
		IconSparkles,
		IconMinimize,
		IconMaximize,
		IconCheck,
		IconMoodSmile,
		IconFileText,
		IconHelp,
		IconX,
		IconLoader2,
		IconAlertCircle,
		IconArrowRight,
		IconChevronDown,
		IconCopy,
		IconRefresh
	} from '$lib/icons';

	// ==========================================================================
	// Types
	// ==========================================================================

	type AIAction =
		| 'improve'
		| 'shorten'
		| 'expand'
		| 'fix_grammar'
		| 'change_tone'
		| 'summarize'
		| 'generate_faq';

	type ToneOption = 'professional' | 'casual' | 'friendly' | 'authoritative' | 'educational';

	interface AIActionConfig {
		id: AIAction;
		label: string;
		icon: typeof IconSparkles;
		shortcut?: string;
		description: string;
	}

	// ==========================================================================
	// Props
	// ==========================================================================

	interface Props {
		/** Position of the menu relative to selection */
		position: { x: number; y: number };
		/** The selected text to process */
		selectedText: string;
		/** Callback when AI result is applied */
		onApply: (result: string) => void;
		/** Callback when menu is closed */
		onClose: () => void;
		/** Whether the menu is visible */
		visible: boolean;
	}

	let { position, selectedText, onApply, onClose, visible }: Props = $props();

	// ==========================================================================
	// State
	// ==========================================================================

	let isLoading = $state(false);
	let result = $state<string | null>(null);
	let error = $state<string | null>(null);
	let selectedAction = $state<AIAction | null>(null);
	let streamedText = $state('');
	let showToneDropdown = $state(false);
	let selectedTone = $state<ToneOption>('professional');
	let menuRef = $state<HTMLDivElement | null>(null);
	let copied = $state(false);
	let adjustedPosition = $state({ x: 0, y: 0, placement: 'below' as 'above' | 'below' });

	// ==========================================================================
	// Constants
	// ==========================================================================

	const AI_ACTIONS: AIActionConfig[] = [
		{
			id: 'improve',
			label: 'Improve',
			icon: IconSparkles,
			shortcut: 'Alt+I',
			description: 'Enhance clarity and readability'
		},
		{
			id: 'shorten',
			label: 'Shorten',
			icon: IconMinimize,
			shortcut: 'Alt+S',
			description: 'Make text more concise'
		},
		{
			id: 'expand',
			label: 'Expand',
			icon: IconMaximize,
			shortcut: 'Alt+E',
			description: 'Add more detail and context'
		},
		{
			id: 'fix_grammar',
			label: 'Fix Grammar',
			icon: IconCheck,
			shortcut: 'Alt+G',
			description: 'Correct grammar and spelling'
		},
		{
			id: 'summarize',
			label: 'Summarize',
			icon: IconFileText,
			shortcut: 'Alt+U',
			description: 'Create a brief summary'
		},
		{
			id: 'generate_faq',
			label: 'Generate FAQ',
			icon: IconHelp,
			shortcut: 'Alt+F',
			description: 'Create FAQ from content'
		}
	];

	const TONE_OPTIONS: { value: ToneOption; label: string }[] = [
		{ value: 'professional', label: 'Professional' },
		{ value: 'casual', label: 'Casual' },
		{ value: 'friendly', label: 'Friendly' },
		{ value: 'authoritative', label: 'Authoritative' },
		{ value: 'educational', label: 'Educational' }
	];

	// ==========================================================================
	// Computed Values
	// ==========================================================================

	/** Calculate character count difference */
	let charDiff = $derived.by(() => {
		if (!result && !streamedText) return 0;
		const newLength = (result || streamedText).length;
		const originalLength = selectedText.length;
		return newLength - originalLength;
	});

	/** Format character difference for display */
	let charDiffDisplay = $derived.by(() => {
		if (charDiff === 0) return '0';
		return charDiff > 0 ? `+${charDiff}` : `${charDiff}`;
	});

	/** Whether we have a result to show */
	let hasResult = $derived(result !== null || streamedText.length > 0);

	/** Display text (result or streamed) */
	let displayText = $derived(result || streamedText);

	// ==========================================================================
	// Effects
	// ==========================================================================

	// Calculate adjusted position based on viewport
	$effect(() => {
		if (visible && browser) {
			const viewportHeight = window.innerHeight;
			const viewportWidth = window.innerWidth;
			const menuHeight = 400; // Estimated max menu height
			const menuWidth = 320; // Menu width

			let x = position.x;
			let y = position.y;
			let placement: 'above' | 'below' = 'below';

			// Check if menu would overflow bottom
			if (y + menuHeight > viewportHeight - 20) {
				// Position above the selection
				y = position.y - menuHeight - 10;
				placement = 'above';
			} else {
				// Position below the selection
				y = position.y + 10;
				placement = 'below';
			}

			// Check horizontal overflow
			if (x + menuWidth > viewportWidth - 20) {
				x = viewportWidth - menuWidth - 20;
			}
			if (x < 20) {
				x = 20;
			}

			// Ensure y is not negative
			if (y < 20) {
				y = 20;
			}

			adjustedPosition = { x, y, placement };
		}
	});

	// Reset state when menu becomes visible
	$effect(() => {
		if (visible) {
			isLoading = false;
			result = null;
			error = null;
			selectedAction = null;
			streamedText = '';
			showToneDropdown = false;
			copied = false;
		}
	});

	// Handle click outside to close menu
	$effect(() => {
		if (!visible || !browser) return;

		const handleClickOutside = (e: MouseEvent) => {
			if (menuRef && !menuRef.contains(e.target as Node)) {
				onClose();
			}
		};

		// Delay to prevent immediate close
		const timeoutId = setTimeout(() => {
			document.addEventListener('mousedown', handleClickOutside);
		}, 100);

		return () => {
			clearTimeout(timeoutId);
			document.removeEventListener('mousedown', handleClickOutside);
		};
	});

	// ==========================================================================
	// Keyboard Shortcuts
	// ==========================================================================

	function handleKeydown(e: KeyboardEvent) {
		if (!visible) return;

		// Close on Escape
		if (e.key === 'Escape') {
			e.preventDefault();
			if (hasResult) {
				resetState();
			} else {
				onClose();
			}
			return;
		}

		// Apply on Enter when result is shown
		if (e.key === 'Enter' && hasResult && !isLoading) {
			e.preventDefault();
			handleApply();
			return;
		}

		// Alt + key shortcuts for actions
		if (e.altKey && !isLoading && !hasResult) {
			const keyMap: Record<string, AIAction> = {
				i: 'improve',
				s: 'shorten',
				e: 'expand',
				g: 'fix_grammar',
				u: 'summarize',
				f: 'generate_faq'
			};

			const action = keyMap[e.key.toLowerCase()];
			if (action) {
				e.preventDefault();
				executeAction(action);
			}
		}
	}

	onMount(() => {
		if (browser) {
			window.addEventListener('keydown', handleKeydown);
		}
	});

	onDestroy(() => {
		if (browser) {
			window.removeEventListener('keydown', handleKeydown);
		}
	});

	// ==========================================================================
	// Actions
	// ==========================================================================

	async function executeAction(action: AIAction, tone?: ToneOption) {
		if (isLoading) return;

		selectedAction = action;
		isLoading = true;
		error = null;
		result = null;
		streamedText = '';
		showToneDropdown = false;

		try {
			// Determine if we should use streaming based on content length
			const useStreaming = selectedText.length > 500;

			if (useStreaming) {
				await executeStreamingAction(action, tone);
			} else {
				await executeStandardAction(action, tone);
			}
		} catch (err) {
			console.error('AI assist error:', err);
			error = err instanceof Error ? err.message : 'An error occurred while processing your request.';
		} finally {
			isLoading = false;
		}
	}

	async function executeStandardAction(action: AIAction, tone?: ToneOption) {
		const response = await fetch('/api/cms/ai/assist', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				action,
				content: selectedText,
				...(action === 'change_tone' && tone ? { tone } : {})
			})
		});

		if (!response.ok) {
			const errorData = await response.json().catch(() => ({}));
			throw new Error(errorData.message || `Request failed with status ${response.status}`);
		}

		const data = await response.json();
		result = data.result || data.content || '';
	}

	async function executeStreamingAction(action: AIAction, tone?: ToneOption) {
		const response = await fetch('/api/cms/ai/assist/stream', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				action,
				content: selectedText,
				...(action === 'change_tone' && tone ? { tone } : {})
			})
		});

		if (!response.ok) {
			const errorData = await response.json().catch(() => ({}));
			throw new Error(errorData.message || `Request failed with status ${response.status}`);
		}

		const reader = response.body?.getReader();
		if (!reader) {
			throw new Error('Streaming not supported');
		}

		const decoder = new TextDecoder();
		let buffer = '';

		try {
			while (true) {
				const { done, value } = await reader.read();

				if (done) break;

				buffer += decoder.decode(value, { stream: true });

				// Process SSE events
				const lines = buffer.split('\n');
				buffer = lines.pop() || '';

				for (const line of lines) {
					if (line.startsWith('data: ')) {
						const data = line.slice(6);

						if (data === '[DONE]') {
							result = streamedText;
							return;
						}

						try {
							const parsed = JSON.parse(data);
							if (parsed.content) {
								streamedText += parsed.content;
							}
							if (parsed.error) {
								throw new Error(parsed.error);
							}
						} catch (parseErr) {
							// If not JSON, append as text
							if (!data.startsWith('{')) {
								streamedText += data;
							}
						}
					}
				}
			}

			// Finalize result
			if (streamedText) {
				result = streamedText;
			}
		} finally {
			reader.releaseLock();
		}
	}

	function handleToneSelect(tone: ToneOption) {
		selectedTone = tone;
		showToneDropdown = false;
		executeAction('change_tone', tone);
	}

	function handleApply() {
		if (displayText) {
			onApply(displayText);
			onClose();
		}
	}

	function resetState() {
		result = null;
		streamedText = '';
		error = null;
		selectedAction = null;
		copied = false;
	}

	async function handleCopy() {
		if (displayText) {
			try {
				await navigator.clipboard.writeText(displayText);
				copied = true;
				setTimeout(() => (copied = false), 2000);
			} catch (err) {
				console.error('Failed to copy:', err);
			}
		}
	}

	function handleRegenerate() {
		if (selectedAction) {
			const action = selectedAction;
			const tone = selectedAction === 'change_tone' ? selectedTone : undefined;
			resetState();
			executeAction(action, tone);
		}
	}
</script>

{#if visible}
	<div
		bind:this={menuRef}
		class="ai-assist-menu"
		class:has-result={hasResult}
		style:left="{adjustedPosition.x}px"
		style:top="{adjustedPosition.y}px"
		role="dialog"
		aria-modal="true"
		aria-label="AI Assist Menu"
		transition:scale={{ duration: 200, start: 0.9, easing: quintOut }}
	>
		<!-- Header -->
		<div class="menu-header">
			<div class="header-left">
				<IconSparkles size={16} class="header-icon" />
				<span class="header-title">AI Assist</span>
			</div>
			<button
				type="button"
				class="close-btn"
				onclick={onClose}
				aria-label="Close menu"
				title="Close (Esc)"
			>
				<IconX size={16} />
			</button>
		</div>

		<!-- Loading State -->
		{#if isLoading}
			<div class="loading-container" transition:fade={{ duration: 150 }}>
				<div class="loading-spinner">
					<IconLoader2 size={32} class="spinner-icon" />
				</div>
				<p class="loading-text">
					{#if selectedAction === 'improve'}
						Improving your text...
					{:else if selectedAction === 'shorten'}
						Making it more concise...
					{:else if selectedAction === 'expand'}
						Adding more detail...
					{:else if selectedAction === 'fix_grammar'}
						Fixing grammar...
					{:else if selectedAction === 'change_tone'}
						Changing tone to {selectedTone}...
					{:else if selectedAction === 'summarize'}
						Creating summary...
					{:else if selectedAction === 'generate_faq'}
						Generating FAQ...
					{:else}
						Processing...
					{/if}
				</p>

				<!-- Streaming Preview -->
				{#if streamedText}
					<div class="streaming-preview" transition:fade={{ duration: 100 }}>
						<div class="streaming-text">{streamedText}<span class="cursor">|</span></div>
					</div>
				{/if}
			</div>
		{:else if error}
			<!-- Error State -->
			<div class="error-container" transition:fade={{ duration: 150 }}>
				<div class="error-icon">
					<IconAlertCircle size={24} />
				</div>
				<p class="error-message">{error}</p>
				<button type="button" class="retry-btn" onclick={() => { error = null; selectedAction = null; }}>
					Try Again
				</button>
			</div>
		{:else if hasResult}
			<!-- Result State -->
			<div class="result-container" transition:fly={{ y: 10, duration: 200, easing: cubicOut }}>
				<div class="result-header">
					<span class="result-label">Result</span>
					<div class="result-meta">
						<span class="char-diff" class:positive={charDiff > 0} class:negative={charDiff < 0}>
							{charDiffDisplay} chars
						</span>
						<button
							type="button"
							class="action-icon-btn"
							onclick={handleCopy}
							aria-label={copied ? 'Copied' : 'Copy result'}
							title="Copy result"
						>
							{#if copied}
								<IconCheck size={14} />
							{:else}
								<IconCopy size={14} />
							{/if}
						</button>
						<button
							type="button"
							class="action-icon-btn"
							onclick={handleRegenerate}
							aria-label="Regenerate"
							title="Regenerate"
						>
							<IconRefresh size={14} />
						</button>
					</div>
				</div>
				<div class="result-content">
					{displayText}
				</div>
				<div class="result-actions">
					<button
						type="button"
						class="discard-btn"
						onclick={resetState}
					>
						Discard
					</button>
					<button
						type="button"
						class="apply-btn"
						onclick={handleApply}
					>
						<IconArrowRight size={16} />
						Apply
					</button>
				</div>
			</div>
		{:else}
			<!-- Action Buttons -->
			<div class="actions-container" transition:fade={{ duration: 150 }}>
				<div class="actions-grid">
					{#each AI_ACTIONS as action (action.id)}
						{@const Icon = action.icon}
						<button
							type="button"
							class="action-btn"
							onclick={() => executeAction(action.id)}
							aria-label={action.label}
							title="{action.description} ({action.shortcut})"
						>
							<Icon size={18} />
							<span class="action-label">{action.label}</span>
							{#if action.shortcut}
								<span class="shortcut-hint">{action.shortcut}</span>
							{/if}
						</button>
					{/each}
				</div>

				<!-- Tone Dropdown -->
				<div class="tone-section">
					<div class="tone-dropdown-container">
						<button
							type="button"
							class="tone-trigger"
							onclick={() => (showToneDropdown = !showToneDropdown)}
							aria-expanded={showToneDropdown}
							aria-haspopup="listbox"
						>
							<IconMoodSmile size={18} />
							<span>Change Tone</span>
							<IconChevronDown size={16} class={showToneDropdown ? 'chevron rotated' : 'chevron'} />
						</button>

						{#if showToneDropdown}
							<div
								class="tone-dropdown"
								role="listbox"
								aria-label="Select tone"
								transition:scale={{ duration: 150, start: 0.95, easing: quintOut }}
							>
								{#each TONE_OPTIONS as option (option.value)}
									<button
										type="button"
										class="tone-option"
										class:selected={selectedTone === option.value}
										role="option"
										aria-selected={selectedTone === option.value}
										onclick={() => handleToneSelect(option.value)}
									>
										{option.label}
										{#if selectedTone === option.value}
											<IconCheck size={14} />
										{/if}
									</button>
								{/each}
							</div>
						{/if}
					</div>
				</div>

				<!-- Selected Text Preview -->
				<div class="selection-preview">
					<span class="preview-label">Selected text:</span>
					<p class="preview-text">{selectedText.slice(0, 150)}{selectedText.length > 150 ? '...' : ''}</p>
					<span class="char-count">{selectedText.length} characters</span>
				</div>
			</div>
		{/if}

		<!-- Keyboard Hints Footer -->
		{#if !isLoading && !hasResult && !error}
			<div class="menu-footer">
				<span class="hint">
					<kbd>Esc</kbd> Close
				</span>
				<span class="hint">
					<kbd>Alt</kbd>+Key Quick action
				</span>
			</div>
		{/if}
	</div>
{/if}

<style>
	.ai-assist-menu {
		position: fixed;
		z-index: 9999;
		width: 320px;
		background: linear-gradient(
			135deg,
			rgba(255, 255, 255, 0.95) 0%,
			rgba(249, 250, 251, 0.95) 100%
		);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 16px;
		box-shadow:
			0 25px 50px -12px rgba(0, 0, 0, 0.25),
			0 0 0 1px rgba(255, 255, 255, 0.8) inset,
			0 0 40px rgba(99, 102, 241, 0.1);
		backdrop-filter: blur(20px);
		overflow: hidden;
		font-family: system-ui, -apple-system, sans-serif;
	}

	.ai-assist-menu.has-result {
		width: 380px;
	}

	/* Header */
	.menu-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.75rem 1rem;
		background: linear-gradient(135deg, rgba(99, 102, 241, 0.08) 0%, rgba(139, 92, 246, 0.08) 100%);
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
	}

	.header-left {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.header-left :global(.header-icon) {
		color: #6366f1;
	}

	.header-title {
		font-size: 0.875rem;
		font-weight: 600;
		color: #1f2937;
	}

	.close-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		background: transparent;
		border: none;
		border-radius: 6px;
		color: #6b7280;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.close-btn:hover {
		background: rgba(0, 0, 0, 0.05);
		color: #374151;
	}

	/* Actions Container */
	.actions-container {
		padding: 0.75rem;
	}

	.actions-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 0.5rem;
		margin-bottom: 0.75rem;
	}

	.action-btn {
		position: relative;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.375rem;
		padding: 0.75rem 0.5rem;
		background: white;
		border: 1px solid #e5e7eb;
		border-radius: 10px;
		color: #4b5563;
		font-size: 0.75rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.action-btn:hover {
		background: linear-gradient(135deg, rgba(99, 102, 241, 0.05) 0%, rgba(139, 92, 246, 0.05) 100%);
		border-color: rgba(99, 102, 241, 0.3);
		color: #6366f1;
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(99, 102, 241, 0.15);
	}

	.action-btn:active {
		transform: translateY(0);
	}

	.action-btn:focus-visible {
		outline: 2px solid #6366f1;
		outline-offset: 2px;
	}

	.action-label {
		white-space: nowrap;
	}

	.shortcut-hint {
		position: absolute;
		top: 4px;
		right: 4px;
		padding: 0.125rem 0.25rem;
		background: rgba(99, 102, 241, 0.1);
		border-radius: 4px;
		font-size: 0.5625rem;
		font-weight: 500;
		color: #6366f1;
		opacity: 0;
		transition: opacity 0.15s ease;
	}

	.action-btn:hover .shortcut-hint {
		opacity: 1;
	}

	/* Tone Section */
	.tone-section {
		margin-bottom: 0.75rem;
	}

	.tone-dropdown-container {
		position: relative;
	}

	.tone-trigger {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		width: 100%;
		padding: 0.625rem 0.75rem;
		background: white;
		border: 1px solid #e5e7eb;
		border-radius: 10px;
		color: #4b5563;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.tone-trigger:hover {
		border-color: rgba(99, 102, 241, 0.3);
		color: #6366f1;
	}

	/* Icon rotation handled via class prop on component */
	:global(.chevron) {
		margin-left: auto;
		transition: transform 0.2s ease;
	}

	:global(.chevron.rotated) {
		transform: rotate(180deg);
	}

	.tone-dropdown {
		position: absolute;
		top: calc(100% + 0.375rem);
		left: 0;
		right: 0;
		background: white;
		border: 1px solid #e5e7eb;
		border-radius: 10px;
		box-shadow: 0 10px 30px rgba(0, 0, 0, 0.15);
		z-index: 10;
		overflow: hidden;
	}

	.tone-option {
		display: flex;
		align-items: center;
		justify-content: space-between;
		width: 100%;
		padding: 0.625rem 0.75rem;
		background: transparent;
		border: none;
		color: #4b5563;
		font-size: 0.8125rem;
		text-align: left;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.tone-option:hover {
		background: rgba(99, 102, 241, 0.05);
		color: #6366f1;
	}

	.tone-option.selected {
		background: rgba(99, 102, 241, 0.1);
		color: #6366f1;
	}

	/* Selection Preview */
	.selection-preview {
		padding: 0.625rem 0.75rem;
		background: #f9fafb;
		border-radius: 8px;
	}

	.preview-label {
		display: block;
		font-size: 0.6875rem;
		font-weight: 600;
		color: #9ca3af;
		text-transform: uppercase;
		letter-spacing: 0.025em;
		margin-bottom: 0.375rem;
	}

	.preview-text {
		font-size: 0.8125rem;
		color: #4b5563;
		line-height: 1.5;
		margin: 0 0 0.375rem;
		overflow: hidden;
		text-overflow: ellipsis;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
	}

	.char-count {
		font-size: 0.6875rem;
		color: #9ca3af;
	}

	/* Loading State */
	.loading-container {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 2rem 1rem;
	}

	.loading-spinner {
		margin-bottom: 1rem;
	}

	.loading-spinner :global(.spinner-icon) {
		color: #6366f1;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	.loading-text {
		font-size: 0.875rem;
		color: #6b7280;
		margin: 0;
		text-align: center;
	}

	.streaming-preview {
		width: 100%;
		max-height: 150px;
		margin-top: 1rem;
		padding: 0.75rem;
		background: #f9fafb;
		border-radius: 8px;
		overflow-y: auto;
	}

	.streaming-text {
		font-size: 0.8125rem;
		color: #4b5563;
		line-height: 1.6;
		white-space: pre-wrap;
		word-break: break-word;
	}

	.cursor {
		animation: blink 1s step-end infinite;
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

	/* Error State */
	.error-container {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 1.5rem 1rem;
		text-align: center;
	}

	.error-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 48px;
		height: 48px;
		background: #fef2f2;
		border-radius: 50%;
		color: #ef4444;
		margin-bottom: 0.75rem;
	}

	.error-message {
		font-size: 0.875rem;
		color: #6b7280;
		margin: 0 0 1rem;
		line-height: 1.5;
	}

	.retry-btn {
		padding: 0.5rem 1rem;
		background: #f3f4f6;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		color: #374151;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.retry-btn:hover {
		background: #e5e7eb;
	}

	/* Result State */
	.result-container {
		padding: 0.75rem;
	}

	.result-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 0.5rem;
	}

	.result-label {
		font-size: 0.75rem;
		font-weight: 600;
		color: #6b7280;
		text-transform: uppercase;
		letter-spacing: 0.025em;
	}

	.result-meta {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.char-diff {
		padding: 0.125rem 0.375rem;
		background: #f3f4f6;
		border-radius: 4px;
		font-size: 0.6875rem;
		font-weight: 600;
		color: #6b7280;
	}

	.char-diff.positive {
		background: rgba(16, 185, 129, 0.1);
		color: #059669;
	}

	.char-diff.negative {
		background: rgba(239, 68, 68, 0.1);
		color: #dc2626;
	}

	.action-icon-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		background: transparent;
		border: none;
		border-radius: 4px;
		color: #9ca3af;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.action-icon-btn:hover {
		background: #f3f4f6;
		color: #6b7280;
	}

	.result-content {
		max-height: 200px;
		padding: 0.75rem;
		background: #f9fafb;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		font-size: 0.875rem;
		color: #374151;
		line-height: 1.6;
		overflow-y: auto;
		white-space: pre-wrap;
		word-break: break-word;
		margin-bottom: 0.75rem;
	}

	.result-actions {
		display: flex;
		gap: 0.5rem;
	}

	.discard-btn {
		flex: 1;
		padding: 0.625rem 1rem;
		background: white;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		color: #6b7280;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.discard-btn:hover {
		background: #f9fafb;
		border-color: #d1d5db;
	}

	.apply-btn {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.375rem;
		padding: 0.625rem 1rem;
		background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 100%);
		border: none;
		border-radius: 8px;
		color: white;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.apply-btn:hover {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(99, 102, 241, 0.3);
	}

	.apply-btn:active {
		transform: translateY(0);
	}

	/* Footer */
	.menu-footer {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 1.5rem;
		padding: 0.625rem 1rem;
		background: #f9fafb;
		border-top: 1px solid #e5e7eb;
	}

	.hint {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.6875rem;
		color: #9ca3af;
	}

	.hint kbd {
		padding: 0.125rem 0.375rem;
		background: white;
		border: 1px solid #e5e7eb;
		border-radius: 4px;
		font-size: 0.625rem;
		font-family: inherit;
		color: #6b7280;
	}

	/* Dark mode support */
	@media (prefers-color-scheme: dark) {
		.ai-assist-menu {
			background: linear-gradient(
				135deg,
				rgba(30, 41, 59, 0.98) 0%,
				rgba(15, 23, 42, 0.98) 100%
			);
			border-color: rgba(99, 102, 241, 0.3);
		}

		.menu-header {
			background: linear-gradient(
				135deg,
				rgba(99, 102, 241, 0.15) 0%,
				rgba(139, 92, 246, 0.15) 100%
			);
			border-color: rgba(99, 102, 241, 0.2);
		}

		.header-title {
			color: #f1f5f9;
		}

		.close-btn {
			color: #94a3b8;
		}

		.close-btn:hover {
			background: rgba(255, 255, 255, 0.1);
			color: #cbd5e1;
		}

		.action-btn {
			background: #1e293b;
			border-color: #334155;
			color: #cbd5e1;
		}

		.action-btn:hover {
			background: linear-gradient(
				135deg,
				rgba(99, 102, 241, 0.15) 0%,
				rgba(139, 92, 246, 0.15) 100%
			);
			border-color: rgba(99, 102, 241, 0.4);
			color: #a5b4fc;
		}

		.tone-trigger {
			background: #1e293b;
			border-color: #334155;
			color: #cbd5e1;
		}

		.tone-trigger:hover {
			border-color: rgba(99, 102, 241, 0.4);
			color: #a5b4fc;
		}

		.tone-dropdown {
			background: #1e293b;
			border-color: #334155;
		}

		.tone-option {
			color: #cbd5e1;
		}

		.tone-option:hover {
			background: rgba(99, 102, 241, 0.15);
			color: #a5b4fc;
		}

		.tone-option.selected {
			background: rgba(99, 102, 241, 0.2);
			color: #a5b4fc;
		}

		.selection-preview {
			background: #0f172a;
		}

		.preview-label {
			color: #64748b;
		}

		.preview-text {
			color: #cbd5e1;
		}

		.char-count {
			color: #64748b;
		}

		.loading-text {
			color: #94a3b8;
		}

		.streaming-preview {
			background: #0f172a;
		}

		.streaming-text {
			color: #cbd5e1;
		}

		.error-icon {
			background: rgba(239, 68, 68, 0.15);
		}

		.error-message {
			color: #94a3b8;
		}

		.retry-btn {
			background: #334155;
			border-color: #475569;
			color: #cbd5e1;
		}

		.retry-btn:hover {
			background: #475569;
		}

		.result-label {
			color: #94a3b8;
		}

		.char-diff {
			background: #334155;
			color: #94a3b8;
		}

		.action-icon-btn {
			color: #64748b;
		}

		.action-icon-btn:hover {
			background: #334155;
			color: #94a3b8;
		}

		.result-content {
			background: #0f172a;
			border-color: #334155;
			color: #cbd5e1;
		}

		.discard-btn {
			background: #1e293b;
			border-color: #334155;
			color: #94a3b8;
		}

		.discard-btn:hover {
			background: #334155;
			border-color: #475569;
		}

		.menu-footer {
			background: #0f172a;
			border-color: #334155;
		}

		.hint {
			color: #64748b;
		}

		.hint kbd {
			background: #1e293b;
			border-color: #334155;
			color: #94a3b8;
		}
	}

	/* Responsive */
	@media (max-width: 640px) {
		.ai-assist-menu {
			width: calc(100vw - 40px);
			max-width: 320px;
		}

		.ai-assist-menu.has-result {
			width: calc(100vw - 40px);
			max-width: 380px;
		}

		.shortcut-hint {
			display: none;
		}

		.menu-footer {
			display: none;
		}
	}
</style>

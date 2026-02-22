<!--
/**
 * Block Error Boundary - Enterprise Error Isolation
 * =============================================================================
 *
 * Wraps individual blocks to catch and handle errors gracefully without
 * crashing the entire editor. Features:
 *
 * - Error isolation per block
 * - User-friendly error messages with block type context
 * - Recovery options: Retry, Reset to Default, Delete
 * - Animated error state transitions
 * - Error logging to console and backend
 * - Block data preservation for recovery
 * - Nested error boundary support for columns/groups
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 * @since January 2026
 */
-->

<script lang="ts">
import { logger } from '$lib/utils/logger';
	import type { Snippet } from 'svelte';
	import { fade, fly, scale } from 'svelte/transition';
	import { cubicOut, elasticOut } from 'svelte/easing';
	import { IconAlertTriangle, IconRefresh, IconTrash, IconArrowBackUp } from '$lib/icons';

	import type { Block } from './types';
	import { BLOCK_DEFINITIONS } from './types';
	import {
		captureBlockError,
		recoverBlock,
		getErrorMessage,
		isRecoverable,
		classifyError,
		determineErrorSeverity,
		markErrorRecovered,
		clearBlockErrors,
		ErrorType,
		ErrorSeverity,
		type BlockErrorContext
	} from './error-handling';

	// ==========================================================================
	// Props
	// ==========================================================================

	interface Props {
		/** The block being wrapped */
		block: Block;
		/** Callback to retry rendering the block */
		onRetry: () => void;
		/** Callback to reset block to default state */
		onReset: () => void;
		/** Callback to delete the block */
		onDelete: () => void;
		/** Children content (the block component) */
		children: Snippet;
		/** Whether this is a nested boundary (inside columns/groups) */
		isNested?: boolean;
		/** Additional context for error logging */
		errorContext?: BlockErrorContext;
		/** Custom fallback content */
		fallback?: Snippet<
			[{ error: Error; block: Block; retry: () => void; reset: () => void; remove: () => void }]
		>;
	}

	let props: Props = $props();
	const block = $derived(props.block);
	const onRetry = $derived(props.onRetry);
	const onReset = $derived(props.onReset);
	const onDelete = $derived(props.onDelete);
	const children = $derived(props.children);
	const isNested = $derived(props.isNested ?? false);
	const errorContext = $derived(props.errorContext ?? {});
	const fallback = $derived(props.fallback);

	// ==========================================================================
	// State
	// ==========================================================================

	/** Whether an error has occurred */
	let hasError = $state(false);

	/** The captured error */
	let error = $state<Error | null>(null);

	/** Error record ID for tracking */
	let errorRecordId = $state<string | null>(null);

	/** Number of retry attempts */
	let retryCount = $state(0);

	/** Whether recovery is in progress */
	let isRecovering = $state(false);

	/** Whether showing confirmation dialog */
	let showDeleteConfirm = $state(false);

	/** Animation key for error state */
	let animationKey = $state(0);

	// ==========================================================================
	// Derived State
	// ==========================================================================

	/** Block definition for display info */
	const blockDefinition = $derived(BLOCK_DEFINITIONS[block.type]);

	/** User-friendly error message */
	const errorMessage = $derived(error ? getErrorMessage(error, block.type) : '');

	/** Error type classification */
	const errorType = $derived(error ? classifyError(error) : null);

	/** Error severity */
	const errorSeverity = $derived(
		error && errorType ? determineErrorSeverity(error, errorType) : null
	);

	/** Whether error appears recoverable */
	const canRecover = $derived(error ? isRecoverable(error) : false);

	/** Whether max retries exceeded */
	const maxRetriesExceeded = $derived(retryCount >= 3);

	/** Show technical details in dev mode */
	const showDetails = $derived(import.meta.env.DEV);

	// ==========================================================================
	// Error Handling
	// ==========================================================================

	/**
	 * Handle error caught by svelte:boundary
	 */
	function handleError(e: unknown) {
		const err = e instanceof Error ? e : new Error(String(e));
		error = err;
		hasError = true;
		animationKey++;

		// Capture and log the error
		const record = captureBlockError(err, block, {
			...errorContext,
			componentName: 'BlockErrorBoundary',
			action: 'render'
		});

		errorRecordId = record.id;

		logger.error(`[BlockErrorBoundary] Error in ${block.type} block:`, err);
	}

	// ==========================================================================
	// Actions
	// ==========================================================================

	/**
	 * Retry rendering the block
	 */
	async function handleRetry() {
		if (maxRetriesExceeded) return;

		isRecovering = true;
		retryCount++;

		// Brief delay for visual feedback
		await new Promise((resolve) => setTimeout(resolve, 300));

		// Clear error state
		hasError = false;
		error = null;

		// Mark previous error as recovered
		if (errorRecordId) {
			markErrorRecovered(errorRecordId);
			errorRecordId = null;
		}

		// Notify parent
		onRetry();

		isRecovering = false;
	}

	/**
	 * Reset block to default state
	 */
	async function handleReset() {
		isRecovering = true;

		// Brief delay for visual feedback
		await new Promise((resolve) => setTimeout(resolve, 300));

		// Attempt recovery
		const recoveredBlock = recoverBlock(block);

		// Log recovery attempt with ErrorType context for debugging
		// ErrorType values: RENDER_ERROR, VALIDATION_ERROR, SAVE_ERROR, NETWORK_ERROR, AI_ERROR, MEDIA_ERROR, UNKNOWN_ERROR
		const errorTypeLabel = errorType ?? ErrorType.UNKNOWN_ERROR;
		logger.debug(`[BlockErrorBoundary] Recovery attempted for block ${block.id}`, {
			errorType: errorTypeLabel,
			recoveredBlock: recoveredBlock ? recoveredBlock.id : null
		});

		// Clear error state
		hasError = false;
		error = null;
		retryCount = 0;

		// Clear errors for this block
		clearBlockErrors(block.id);

		// Notify parent with recovered block
		onReset();

		isRecovering = false;
	}

	/**
	 * Delete the block
	 */
	function handleDelete() {
		if (!showDeleteConfirm) {
			showDeleteConfirm = true;
			return;
		}

		// Clear errors for this block
		clearBlockErrors(block.id);

		// Notify parent
		onDelete();
	}

	/**
	 * Cancel delete confirmation
	 */
	function cancelDelete() {
		showDeleteConfirm = false;
	}

	// ==========================================================================
	// Keyboard Handling
	// ==========================================================================

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape' && showDeleteConfirm) {
			e.preventDefault();
			cancelDelete();
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} />

<svelte:boundary onerror={handleError}>
	{#if hasError && error}
		{#if fallback}
			{@render fallback({
				error,
				block,
				retry: handleRetry,
				reset: handleReset,
				remove: handleDelete
			})}
		{:else}
			{#key animationKey}
				<div
					class="block-error-boundary"
					class:nested={isNested}
					class:severity-critical={errorSeverity === ErrorSeverity.CRITICAL}
					class:severity-high={errorSeverity === ErrorSeverity.HIGH}
					class:severity-medium={errorSeverity === ErrorSeverity.MEDIUM}
					class:severity-low={errorSeverity === ErrorSeverity.LOW}
					in:fly={{ y: 10, duration: 300, easing: cubicOut }}
					role="alert"
					aria-live="polite"
				>
					<!-- Error Header -->
					<div class="error-header">
						<div class="error-icon" in:scale={{ duration: 400, easing: elasticOut }}>
							<IconAlertTriangle size={24} />
						</div>
						<div class="error-info">
							<h4 class="error-title">
								{blockDefinition?.name || block.type} Block Error
							</h4>
							<p class="error-message">{errorMessage}</p>
						</div>
					</div>

					<!-- Technical Details (Dev Mode) -->
					{#if showDetails}
						<details class="error-details" transition:fade={{ duration: 200 }}>
							<summary>Technical Details</summary>
							<div class="error-technical">
								<div class="detail-row">
									<span class="detail-label">Error Type:</span>
									<code class="detail-value">{errorType}</code>
								</div>
								<div class="detail-row">
									<span class="detail-label">Severity:</span>
									<code
										class="detail-value severity-badge"
										class:critical={errorSeverity === ErrorSeverity.CRITICAL}
									>
										{errorSeverity}
									</code>
								</div>
								<div class="detail-row">
									<span class="detail-label">Block ID:</span>
									<code class="detail-value">{block.id}</code>
								</div>
								<div class="detail-row">
									<span class="detail-label">Recoverable:</span>
									<code class="detail-value">{canRecover ? 'Yes' : 'No'}</code>
								</div>
								<div class="detail-row">
									<span class="detail-label">Retry Count:</span>
									<code class="detail-value">{retryCount}/3</code>
								</div>
								{#if error.message}
									<div class="detail-row full-width">
										<span class="detail-label">Message:</span>
										<code class="detail-value error-text">{error.message}</code>
									</div>
								{/if}
								{#if error.stack}
									<div class="detail-row full-width">
										<span class="detail-label">Stack:</span>
										<pre class="error-stack">{error.stack}</pre>
									</div>
								{/if}
							</div>
						</details>
					{/if}

					<!-- Action Buttons -->
					<div class="error-actions" class:confirm-mode={showDeleteConfirm}>
						{#if showDeleteConfirm}
							<div class="confirm-dialog" transition:fly={{ y: -10, duration: 200 }}>
								<span class="confirm-text">Delete this block?</span>
								<button
									type="button"
									class="btn-confirm-delete"
									onclick={handleDelete}
									aria-label="Confirm delete block"
								>
									Yes, Delete
								</button>
								<button
									type="button"
									class="btn-cancel"
									onclick={cancelDelete}
									aria-label="Cancel delete"
								>
									Cancel
								</button>
							</div>
						{:else}
							<!-- Retry Button -->
							<button
								type="button"
								class="btn-action btn-retry"
								onclick={handleRetry}
								disabled={isRecovering || maxRetriesExceeded}
								aria-label="Retry rendering this block"
								title={maxRetriesExceeded ? 'Maximum retries exceeded' : 'Retry rendering'}
							>
								<IconRefresh size={16} class={isRecovering ? 'spin' : ''} />
								<span>Retry</span>
								{#if retryCount > 0}
									<span class="retry-count">({retryCount}/3)</span>
								{/if}
							</button>

							<!-- Reset Button -->
							<button
								type="button"
								class="btn-action btn-reset"
								onclick={handleReset}
								disabled={isRecovering}
								aria-label="Reset block to default state"
								title="Reset block to default values"
							>
								<IconArrowBackUp size={16} />
								<span>Reset to Default</span>
							</button>

							<!-- Delete Button -->
							<button
								type="button"
								class="btn-action btn-delete"
								onclick={handleDelete}
								disabled={isRecovering}
								aria-label="Delete this block"
								title="Remove this block"
							>
								<IconTrash size={16} />
								<span>Delete Block</span>
							</button>
						{/if}
					</div>

					<!-- Recovery Progress Indicator -->
					{#if isRecovering}
						<div class="recovery-progress" transition:fade={{ duration: 150 }}>
							<div class="progress-bar"></div>
						</div>
					{/if}
				</div>
			{/key}
		{/if}
	{:else}
		<!-- Normal block rendering -->
		{@render children()}
	{/if}
</svelte:boundary>

<style>
	.block-error-boundary {
		position: relative;
		padding: 1.25rem;
		background: linear-gradient(135deg, #fef2f2 0%, #fee2e2 100%);
		border: 1px solid #fecaca;
		border-left: 4px solid #ef4444;
		border-radius: 8px;
		overflow: hidden;
	}

	.block-error-boundary.nested {
		padding: 1rem;
		border-radius: 6px;
	}

	/* Severity-based styling */
	.block-error-boundary.severity-critical {
		border-left-color: #dc2626;
		background: linear-gradient(135deg, #fef2f2 0%, #fecaca 100%);
	}

	.block-error-boundary.severity-high {
		border-left-color: #ef4444;
	}

	.block-error-boundary.severity-medium {
		border-left-color: #f97316;
		background: linear-gradient(135deg, #fffbeb 0%, #fef3c7 100%);
		border-color: #fed7aa;
	}

	.block-error-boundary.severity-low {
		border-left-color: #eab308;
		background: linear-gradient(135deg, #fefce8 0%, #fef9c3 100%);
		border-color: #fde68a;
	}

	/* Error Header */
	.error-header {
		display: flex;
		align-items: flex-start;
		gap: 1rem;
		margin-bottom: 1rem;
	}

	.error-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		background: #fee2e2;
		border-radius: 8px;
		color: #dc2626;
		flex-shrink: 0;
	}

	.severity-medium .error-icon {
		background: #fef3c7;
		color: #d97706;
	}

	.severity-low .error-icon {
		background: #fef9c3;
		color: #ca8a04;
	}

	.error-info {
		flex: 1;
		min-width: 0;
	}

	.error-title {
		margin: 0 0 0.25rem 0;
		font-size: 0.9375rem;
		font-weight: 600;
		color: #991b1b;
	}

	.severity-medium .error-title {
		color: #92400e;
	}

	.severity-low .error-title {
		color: #854d0e;
	}

	.error-message {
		margin: 0;
		font-size: 0.875rem;
		color: #7f1d1d;
		line-height: 1.5;
	}

	.severity-medium .error-message {
		color: #78350f;
	}

	.severity-low .error-message {
		color: #713f12;
	}

	/* Technical Details */
	.error-details {
		margin-bottom: 1rem;
		padding: 0.75rem;
		background: rgba(255, 255, 255, 0.5);
		border: 1px solid rgba(0, 0, 0, 0.05);
		border-radius: 6px;
		font-size: 0.8125rem;
	}

	.error-details summary {
		cursor: pointer;
		font-weight: 500;
		color: #6b7280;
		user-select: none;
		padding: 0.25rem 0;
	}

	.error-details summary:hover {
		color: #374151;
	}

	.error-details[open] summary {
		margin-bottom: 0.75rem;
		padding-bottom: 0.5rem;
		border-bottom: 1px solid rgba(0, 0, 0, 0.05);
	}

	.error-technical {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.detail-row {
		display: flex;
		align-items: flex-start;
		gap: 0.5rem;
	}

	.detail-row.full-width {
		flex-direction: column;
		gap: 0.25rem;
	}

	.detail-label {
		font-weight: 500;
		color: #6b7280;
		white-space: nowrap;
	}

	.detail-value {
		font-family: 'SF Mono', 'Fira Code', monospace;
		font-size: 0.75rem;
		padding: 0.125rem 0.375rem;
		background: rgba(0, 0, 0, 0.05);
		border-radius: 3px;
		color: #1f2937;
	}

	.severity-badge.critical {
		background: #fee2e2;
		color: #dc2626;
	}

	.error-text {
		word-break: break-word;
		white-space: pre-wrap;
	}

	.error-stack {
		margin: 0;
		padding: 0.5rem;
		background: rgba(0, 0, 0, 0.05);
		border-radius: 4px;
		font-size: 0.6875rem;
		overflow-x: auto;
		max-height: 150px;
		overflow-y: auto;
		white-space: pre;
		color: #6b7280;
	}

	/* Action Buttons */
	.error-actions {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.error-actions.confirm-mode {
		justify-content: center;
	}

	.btn-action {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 0.875rem;
		background: white;
		border: 1px solid #e5e7eb;
		border-radius: 6px;
		font-size: 0.8125rem;
		font-weight: 500;
		color: #374151;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.btn-action:hover:not(:disabled) {
		background: #f9fafb;
		border-color: #d1d5db;
	}

	.btn-action:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-retry {
		background: #eff6ff;
		border-color: #bfdbfe;
		color: #1d4ed8;
	}

	.btn-retry:hover:not(:disabled) {
		background: #dbeafe;
		border-color: #93c5fd;
	}

	.btn-reset {
		background: #ecfdf5;
		border-color: #a7f3d0;
		color: #047857;
	}

	.btn-reset:hover:not(:disabled) {
		background: #d1fae5;
		border-color: #6ee7b7;
	}

	.btn-delete {
		background: #fef2f2;
		border-color: #fecaca;
		color: #dc2626;
	}

	.btn-delete:hover:not(:disabled) {
		background: #fee2e2;
		border-color: #fca5a5;
	}

	.retry-count {
		font-size: 0.6875rem;
		opacity: 0.7;
	}

	/* Confirm Dialog */
	.confirm-dialog {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		background: #fef2f2;
		border: 1px solid #fecaca;
		border-radius: 8px;
	}

	.confirm-text {
		font-size: 0.875rem;
		font-weight: 500;
		color: #991b1b;
	}

	.btn-confirm-delete {
		padding: 0.5rem 1rem;
		background: #dc2626;
		border: none;
		border-radius: 6px;
		font-size: 0.8125rem;
		font-weight: 500;
		color: white;
		cursor: pointer;
		transition: background 0.15s;
	}

	.btn-confirm-delete:hover {
		background: #b91c1c;
	}

	.btn-cancel {
		padding: 0.5rem 1rem;
		background: white;
		border: 1px solid #d1d5db;
		border-radius: 6px;
		font-size: 0.8125rem;
		font-weight: 500;
		color: #6b7280;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-cancel:hover {
		background: #f3f4f6;
		border-color: #9ca3af;
	}

	/* Recovery Progress */
	.recovery-progress {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		height: 3px;
		background: rgba(0, 0, 0, 0.1);
		overflow: hidden;
	}

	.progress-bar {
		height: 100%;
		background: linear-gradient(90deg, #3b82f6, #8b5cf6, #3b82f6);
		background-size: 200% 100%;
		animation: progress 1s ease-in-out infinite;
	}

	@keyframes progress {
		0% {
			background-position: 100% 0;
		}
		100% {
			background-position: -100% 0;
		}
	}

	/* Spinning icon animation */
	:global(.spin) {
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	/* Dark mode support */
	@media (prefers-color-scheme: dark) {
		.block-error-boundary {
			background: linear-gradient(135deg, #450a0a 0%, #7f1d1d 100%);
			border-color: #991b1b;
		}

		.block-error-boundary.severity-medium {
			background: linear-gradient(135deg, #451a03 0%, #78350f 100%);
			border-color: #92400e;
		}

		.block-error-boundary.severity-low {
			background: linear-gradient(135deg, #422006 0%, #713f12 100%);
			border-color: #854d0e;
		}

		.error-icon {
			background: rgba(0, 0, 0, 0.2);
		}

		.error-title,
		.error-message {
			color: #fecaca;
		}

		.error-details {
			background: rgba(0, 0, 0, 0.2);
			border-color: rgba(255, 255, 255, 0.1);
		}

		.error-details summary {
			color: #d1d5db;
		}

		.detail-value {
			background: rgba(0, 0, 0, 0.3);
			color: #e5e7eb;
		}

		.error-stack {
			background: rgba(0, 0, 0, 0.3);
			color: #9ca3af;
		}

		.btn-action {
			background: rgba(0, 0, 0, 0.3);
			border-color: rgba(255, 255, 255, 0.1);
			color: #e5e7eb;
		}

		.btn-action:hover:not(:disabled) {
			background: rgba(0, 0, 0, 0.4);
		}

		.btn-retry {
			background: rgba(59, 130, 246, 0.2);
			border-color: rgba(59, 130, 246, 0.3);
			color: #93c5fd;
		}

		.btn-reset {
			background: rgba(16, 185, 129, 0.2);
			border-color: rgba(16, 185, 129, 0.3);
			color: #6ee7b7;
		}

		.btn-delete {
			background: rgba(239, 68, 68, 0.2);
			border-color: rgba(239, 68, 68, 0.3);
			color: #fca5a5;
		}

		.confirm-dialog {
			background: rgba(0, 0, 0, 0.3);
			border-color: rgba(239, 68, 68, 0.3);
		}

		.confirm-text {
			color: #fca5a5;
		}

		.btn-cancel {
			background: rgba(0, 0, 0, 0.3);
			border-color: rgba(255, 255, 255, 0.2);
			color: #d1d5db;
		}
	}

	/* Responsive adjustments */
	@media (max-width: 480px) {
		.block-error-boundary {
			padding: 1rem;
		}

		.error-header {
			flex-direction: column;
			text-align: center;
		}

		.error-icon {
			margin: 0 auto;
		}

		.error-actions {
			flex-direction: column;
		}

		.btn-action {
			justify-content: center;
		}

		.confirm-dialog {
			flex-direction: column;
			text-align: center;
		}
	}
</style>

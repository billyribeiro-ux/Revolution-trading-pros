<!--
/**
 * Newsletter Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Email signup form with validation, loading states, and customizable styling.
 * Uses BlockStateManager for centralized state management.
 */
-->

<script lang="ts">
	import { IconMail, IconLoader2, IconCheck, IconAlertCircle } from '$lib/icons';
	import { getBlockStateManager, type BlockId } from '$lib/stores/blockState.svelte';
	import { isValidEmail } from '$lib/utils/sanitization';
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

	// Get centralized state manager
	const stateManager = getBlockStateManager();

	// Get newsletter state from BlockStateManager
	let newsletterState = $derived(stateManager.getNewsletterState(props.blockId));

	// Content fields with defaults
	let placeholder = $derived(
		props.block.content.newsletterPlaceholder || 'Enter your email address'
	);
	let buttonText = $derived(props.block.content.newsletterButtonText || 'Subscribe');

	// Local state for input binding
	let emailInput = $state('');

	// Sync email input with state manager on mount
	$effect(() => {
		emailInput = newsletterState.email;
	});

	/**
	 * Update block content
	 */
	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	/**
	 * Handle email input changes
	 */
	function handleEmailInput(e: Event): void {
		const value = (e.target as HTMLInputElement).value;
		emailInput = value;
		stateManager.setNewsletterState(props.blockId, { email: value, error: null });
	}

	/**
	 * Validate and submit newsletter form
	 */
	async function handleSubmit(e: Event): Promise<void> {
		e.preventDefault();

		// Don't submit while editing
		if (props.isEditing) return;

		// Clear previous error
		stateManager.setNewsletterState(props.blockId, { error: null });

		// Validate email
		const email = emailInput.trim();
		if (!email) {
			stateManager.setNewsletterState(props.blockId, {
				error: 'Please enter your email address'
			});
			return;
		}

		if (!isValidEmail(email)) {
			stateManager.setNewsletterState(props.blockId, {
				error: 'Please enter a valid email address'
			});
			return;
		}

		// Start submission
		stateManager.setNewsletterState(props.blockId, { submitting: true });

		try {
			// Simulate API call - replace with actual newsletter service
			await new Promise((resolve) => setTimeout(resolve, 1500));

			// Success
			stateManager.setNewsletterState(props.blockId, {
				submitting: false,
				success: true,
				email: ''
			});
			emailInput = '';
		} catch (error) {
			const errorMessage =
				error instanceof Error ? error.message : 'Failed to subscribe. Please try again.';
			stateManager.setNewsletterState(props.blockId, {
				submitting: false,
				error: errorMessage
			});

			if (props.onError && error instanceof Error) {
				props.onError(error);
			}
		}
	}

	/**
	 * Reset success state to allow re-subscription
	 */
	function handleReset(): void {
		stateManager.setNewsletterState(props.blockId, {
			success: false,
			email: '',
			error: null
		});
		emailInput = '';
	}
</script>

<div
	class="newsletter-block"
	class:is-editing={props.isEditing}
	role="region"
	aria-label="Newsletter signup"
>
	<div class="newsletter-header">
		<div class="newsletter-icon">
			<IconMail size={24} aria-hidden="true" />
		</div>
		<h3 class="newsletter-title">Subscribe to Our Newsletter</h3>
		<p class="newsletter-description">Get the latest trading insights delivered to your inbox.</p>
	</div>

	{#if newsletterState.success}
		<!-- Success State -->
		<div class="newsletter-success" role="status" aria-live="polite">
			<div class="success-icon">
				<IconCheck size={28} aria-hidden="true" />
			</div>
			<p class="success-message">Thanks for subscribing!</p>
			<p class="success-subtext">Check your inbox for a confirmation email.</p>
			{#if props.isEditing}
				<button type="button" class="reset-button" onclick={handleReset}> Reset Form </button>
			{/if}
		</div>
	{:else}
		<!-- Form State -->
		<form class="newsletter-form" onsubmit={handleSubmit} novalidate>
			<div class="form-group">
				<input
					type="email"
					value={emailInput}
					oninput={handleEmailInput}
					{placeholder}
					disabled={newsletterState.submitting || props.isEditing}
					aria-label="Email address"
					aria-invalid={newsletterState.error ? 'true' : undefined}
					aria-describedby={newsletterState.error ? 'newsletter-error' : undefined}
					autocomplete="email"
				/>
				<button
					type="submit"
					disabled={newsletterState.submitting || props.isEditing}
					aria-busy={newsletterState.submitting}
				>
					{#if newsletterState.submitting}
						<IconLoader2 size={18} class="spinner" aria-hidden="true" />
						<span class="sr-only">Subscribing...</span>
					{:else}
						{buttonText}
					{/if}
				</button>
			</div>

			{#if newsletterState.error}
				<div class="form-error" id="newsletter-error" role="alert">
					<IconAlertCircle size={16} aria-hidden="true" />
					<span>{newsletterState.error}</span>
				</div>
			{/if}

			<p class="privacy-text">We respect your privacy. Unsubscribe at any time.</p>
		</form>
	{/if}

	<!-- Settings Panel (visible when editing and selected) -->
	{#if props.isEditing && props.isSelected}
		<div class="newsletter-settings">
			<h4 class="settings-title">Newsletter Settings</h4>
			<div class="settings-grid">
				<label class="setting-field">
					<span class="setting-label">Placeholder Text:</span>
					<input
						type="text"
						value={placeholder}
						oninput={(e) =>
							updateContent({ newsletterPlaceholder: (e.target as HTMLInputElement).value })}
						placeholder="Enter your email..."
					/>
				</label>
				<label class="setting-field">
					<span class="setting-label">Button Text:</span>
					<input
						type="text"
						value={buttonText}
						oninput={(e) =>
							updateContent({ newsletterButtonText: (e.target as HTMLInputElement).value })}
						placeholder="Subscribe"
					/>
				</label>
			</div>
		</div>
	{/if}
</div>

<style>
	.newsletter-block {
		padding: 2rem;
		background: linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%);
		border-radius: 16px;
		text-align: center;
		color: white;
	}

	.newsletter-block.is-editing {
		outline: 2px dashed rgba(255, 255, 255, 0.4);
		outline-offset: -2px;
	}

	.newsletter-header {
		margin-bottom: 1.5rem;
	}

	.newsletter-icon {
		width: 56px;
		height: 56px;
		margin: 0 auto 1rem;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(255, 255, 255, 0.2);
		border-radius: 14px;
	}

	.newsletter-title {
		margin: 0 0 0.5rem;
		font-size: 1.5rem;
		font-weight: 700;
	}

	.newsletter-description {
		margin: 0;
		font-size: 1rem;
		opacity: 0.9;
	}

	/* Form Styles */
	.newsletter-form {
		max-width: 420px;
		margin: 0 auto;
	}

	.form-group {
		display: flex;
		gap: 0.5rem;
		background: white;
		padding: 0.375rem;
		border-radius: 12px;
	}

	.form-group input {
		flex: 1;
		min-width: 0;
		padding: 0.75rem 1rem;
		border: none;
		background: transparent;
		font-size: 1rem;
		color: #1e293b;
	}

	.form-group input::placeholder {
		color: #94a3b8;
	}

	.form-group input:focus {
		outline: none;
	}

	.form-group input:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.form-group button {
		flex-shrink: 0;
		padding: 0.75rem 1.5rem;
		background: #1d4ed8;
		border: none;
		border-radius: 8px;
		color: white;
		font-size: 1rem;
		font-weight: 600;
		cursor: pointer;
		transition:
			background 0.15s,
			transform 0.1s;
		display: flex;
		align-items: center;
		justify-content: center;
		min-width: 120px;
	}

	.form-group button:hover:not(:disabled) {
		background: #1e40af;
	}

	.form-group button:active:not(:disabled) {
		transform: scale(0.98);
	}

	.form-group button:disabled {
		opacity: 0.7;
		cursor: not-allowed;
	}

	/* Spinner Animation */
	.form-group button :global(.spinner) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Error State */
	.form-error {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		margin-top: 0.75rem;
		padding: 0.5rem 0.75rem;
		background: rgba(254, 202, 202, 0.2);
		border-radius: 8px;
		font-size: 0.875rem;
		color: #fecaca;
	}

	/* Privacy Text */
	.privacy-text {
		margin: 1rem 0 0;
		font-size: 0.75rem;
		opacity: 0.8;
	}

	/* Success State */
	.newsletter-success {
		padding: 1.5rem;
		background: rgba(255, 255, 255, 0.15);
		border-radius: 12px;
		max-width: 420px;
		margin: 0 auto;
	}

	.success-icon {
		width: 56px;
		height: 56px;
		margin: 0 auto 1rem;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(34, 197, 94, 0.2);
		border-radius: 50%;
		color: #86efac;
	}

	.success-message {
		margin: 0 0 0.25rem;
		font-size: 1.25rem;
		font-weight: 600;
	}

	.success-subtext {
		margin: 0;
		font-size: 0.875rem;
		opacity: 0.8;
	}

	.reset-button {
		margin-top: 1rem;
		padding: 0.5rem 1rem;
		background: rgba(255, 255, 255, 0.2);
		border: 1px solid rgba(255, 255, 255, 0.3);
		border-radius: 6px;
		color: white;
		font-size: 0.875rem;
		cursor: pointer;
		transition: background 0.15s;
	}

	.reset-button:hover {
		background: rgba(255, 255, 255, 0.3);
	}

	/* Settings Panel */
	.newsletter-settings {
		margin-top: 1.5rem;
		padding-top: 1.5rem;
		border-top: 1px solid rgba(255, 255, 255, 0.2);
	}

	.settings-title {
		margin: 0 0 1rem;
		font-size: 0.875rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		opacity: 0.9;
	}

	.settings-grid {
		display: flex;
		gap: 1rem;
		flex-wrap: wrap;
	}

	.setting-field {
		flex: 1;
		min-width: 180px;
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
		text-align: left;
	}

	.setting-label {
		font-size: 0.75rem;
		font-weight: 600;
		opacity: 0.9;
	}

	.setting-field input {
		padding: 0.5rem 0.75rem;
		border: 1px solid rgba(255, 255, 255, 0.3);
		border-radius: 6px;
		background: rgba(255, 255, 255, 0.1);
		color: white;
		font-size: 0.875rem;
	}

	.setting-field input::placeholder {
		color: rgba(255, 255, 255, 0.5);
	}

	.setting-field input:focus {
		outline: none;
		border-color: rgba(255, 255, 255, 0.6);
		background: rgba(255, 255, 255, 0.15);
	}

	/* Screen Reader Only */
	.sr-only {
		position: absolute;
		width: 1px;
		height: 1px;
		padding: 0;
		margin: -1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		white-space: nowrap;
		border: 0;
	}

	/* Responsive: Stack on mobile */
	@media (max-width: 480px) {
		.newsletter-block {
			padding: 1.5rem;
		}

		.form-group {
			flex-direction: column;
			padding: 0.5rem;
		}

		.form-group input {
			padding: 0.875rem 1rem;
		}

		.form-group button {
			width: 100%;
			padding: 0.875rem 1.5rem;
		}

		.settings-grid {
			flex-direction: column;
		}

		.setting-field {
			min-width: 100%;
		}
	}

	/* Dark Mode Support */
	:global(.dark) .newsletter-block {
		background: linear-gradient(135deg, #1e3a8a 0%, #1e40af 100%);
	}

	:global(.dark) .form-group {
		background: #0f172a;
	}

	:global(.dark) .form-group input {
		color: #f1f5f9;
	}

	:global(.dark) .form-group input::placeholder {
		color: #64748b;
	}

	:global(.dark) .form-group button {
		background: #3b82f6;
	}

	:global(.dark) .form-group button:hover:not(:disabled) {
		background: #2563eb;
	}

	:global(.dark) .form-error {
		background: rgba(239, 68, 68, 0.2);
		color: #fca5a5;
	}

	:global(.dark) .newsletter-success {
		background: rgba(255, 255, 255, 0.1);
	}

	:global(.dark) .success-icon {
		background: rgba(34, 197, 94, 0.15);
		color: #4ade80;
	}

	:global(.dark) .setting-field input {
		background: rgba(0, 0, 0, 0.2);
		border-color: rgba(255, 255, 255, 0.2);
	}

	:global(.dark) .setting-field input:focus {
		border-color: rgba(255, 255, 255, 0.4);
		background: rgba(0, 0, 0, 0.3);
	}
</style>

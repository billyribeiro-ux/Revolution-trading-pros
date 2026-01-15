<script lang="ts">
	/**
	 * SocialLoginButtons - OAuth Social Sign-In
	 * Apple Principal Engineer ICT 11 Grade
	 *
	 * Google and Apple sign-in buttons with premium styling
	 * Uses SvelteKit goto() for proper SPA navigation & error feedback
	 *
	 * ICT 11+ Patterns:
	 * - Async error handling with user-visible feedback
	 * - Optional callback props for custom OAuth flows
	 * - SvelteKit navigation preserves SPA context
	 * - Accessible ARIA labels and loading states
	 *
	 * @version 2.0.0 - ICT 11 Grade
	 */
	import { goto } from '$app/navigation';
	import { fade } from 'svelte/transition';

	interface Props {
		disabled?: boolean;
		onGoogleClick?: () => void;
		onAppleClick?: () => void;
		onError?: (message: string) => void;
	}

	let { disabled = false, onGoogleClick, onAppleClick, onError }: Props = $props();

	let isGoogleLoading = $state(false);
	let isAppleLoading = $state(false);
	let error = $state('');

	async function handleGoogleLogin() {
		if (disabled || isGoogleLoading) return;
		isGoogleLoading = true;
		error = '';

		try {
			if (onGoogleClick) {
				onGoogleClick();
			} else {
				// ICT11+ Pattern: Use goto() for SPA navigation, preserves animation context
				await goto('/api/auth/google', { replaceState: false });
			}
		} catch (err) {
			const message = err instanceof Error ? err.message : 'Failed to sign in with Google';
			error = message;
			onError?.(message);
			console.error('[SocialLogin] Google login error:', err);
		} finally {
			isGoogleLoading = false;
		}
	}

	async function handleAppleLogin() {
		if (disabled || isAppleLoading) return;
		isAppleLoading = true;
		error = '';

		try {
			if (onAppleClick) {
				onAppleClick();
			} else {
				// ICT11+ Pattern: Use goto() for SPA navigation, preserves animation context
				await goto('/api/auth/apple', { replaceState: false });
			}
		} catch (err) {
			const message = err instanceof Error ? err.message : 'Failed to sign in with Apple';
			error = message;
			onError?.(message);
			console.error('[SocialLogin] Apple login error:', err);
		} finally {
			isAppleLoading = false;
		}
	}
</script>

<div class="social-login">
	<div class="divider">
		<span class="divider-line"></span>
		<span class="divider-text">or continue with</span>
		<span class="divider-line"></span>
	</div>

	<!-- ICT11+ Pattern: User-visible error feedback -->
	{#if error}
		<div class="social-error" transition:fade={{ duration: 200 }} role="alert">
			<span class="error-text">{error}</span>
		</div>
	{/if}

	<div class="social-buttons">
		<!-- Google Button -->
		<button
			type="button"
			class="social-btn google-btn"
			onclick={handleGoogleLogin}
			disabled={disabled || isGoogleLoading}
			aria-label="Sign in with Google"
		>
			<svg class="social-icon" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
				<path
					d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"
					fill="#4285F4"
				/>
				<path
					d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"
					fill="#34A853"
				/>
				<path
					d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"
					fill="#FBBC05"
				/>
				<path
					d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"
					fill="#EA4335"
				/>
			</svg>
			<span class="btn-text">
				{#if isGoogleLoading}
					<span class="spinner"></span>
				{:else}
					Google
				{/if}
			</span>
		</button>

		<!-- Apple Button -->
		<button
			type="button"
			class="social-btn apple-btn"
			onclick={handleAppleLogin}
			disabled={disabled || isAppleLoading}
			aria-label="Sign in with Apple"
		>
			<svg class="social-icon" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
				<path
					d="M17.05 20.28c-.98.95-2.05.8-3.08.35-1.09-.46-2.09-.48-3.24 0-1.44.62-2.2.44-3.06-.35C2.79 15.25 3.51 7.59 9.05 7.31c1.35.07 2.29.74 3.08.8 1.18-.24 2.31-.93 3.57-.84 1.51.12 2.65.72 3.4 1.8-3.12 1.87-2.38 5.98.48 7.13-.57 1.5-1.31 2.99-2.54 4.09l.01-.01zM12.03 7.25c-.15-2.23 1.66-4.07 3.74-4.25.29 2.58-2.34 4.5-3.74 4.25z"
					fill="currentColor"
				/>
			</svg>
			<span class="btn-text">
				{#if isAppleLoading}
					<span class="spinner"></span>
				{:else}
					Apple
				{/if}
			</span>
		</button>
	</div>
</div>

<style>
	.social-login {
		margin-top: 1.5rem;
	}

	/* ICT11+ Pattern: Error feedback styling */
	.social-error {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0.625rem 1rem;
		margin-bottom: 1rem;
		background: var(--auth-error-bg, rgba(239, 68, 68, 0.1));
		border: 1px solid var(--auth-error-border, rgba(239, 68, 68, 0.3));
		border-radius: 8px;
	}

	.error-text {
		font-size: 0.8125rem;
		font-weight: 500;
		color: var(--auth-error, #f87171);
		text-align: center;
	}

	/* Divider */
	.divider {
		display: flex;
		align-items: center;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.divider-line {
		flex: 1;
		height: 1px;
		background: var(--auth-card-border, rgba(230, 184, 0, 0.15));
	}

	.divider-text {
		font-family: var(--font-body);
		font-size: 0.8125rem;
		color: var(--auth-muted, #64748b);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		white-space: nowrap;
	}

	/* Social Buttons Container */
	.social-buttons {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 0.75rem;
	}

	/* Social Button Base */
	.social-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.625rem;
		padding: 0.875rem 1rem;
		border-radius: 12px;
		font-family: var(--font-heading);
		font-size: 0.9375rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
		border: 2px solid var(--auth-input-border, rgba(100, 116, 139, 0.3));
		background: var(--auth-input-bg, rgba(15, 23, 42, 0.5));
	}

	.social-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.social-btn:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 8px 25px rgba(0, 0, 0, 0.15);
	}

	.social-btn:active:not(:disabled) {
		transform: translateY(0);
	}

	/* Google Button */
	.google-btn {
		color: var(--auth-text, #e2e8f0);
	}

	.google-btn:hover:not(:disabled) {
		border-color: rgba(66, 133, 244, 0.5);
		background: rgba(66, 133, 244, 0.1);
	}

	/* Apple Button */
	.apple-btn {
		color: var(--auth-text, #e2e8f0);
	}

	.apple-btn:hover:not(:disabled) {
		border-color: var(--auth-text, #e2e8f0);
		background: rgba(255, 255, 255, 0.1);
	}

	/* Icon */
	.social-icon {
		width: 20px;
		height: 20px;
		flex-shrink: 0;
	}

	/* Button Text */
	.btn-text {
		display: flex;
		align-items: center;
		justify-content: center;
		min-width: 50px;
	}

	/* Spinner */
	.spinner {
		width: 16px;
		height: 16px;
		border: 2px solid transparent;
		border-top-color: currentColor;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	/* Light Theme */
	:global(html.light) .social-btn,
	:global(body.light) .social-btn {
		background: #ffffff;
		border-color: #d2d2d7;
		color: #1d1d1f;
	}

	:global(html.light) .google-btn:hover:not(:disabled),
	:global(body.light) .google-btn:hover:not(:disabled) {
		background: rgba(66, 133, 244, 0.08);
		border-color: rgba(66, 133, 244, 0.4);
	}

	:global(html.light) .apple-btn:hover:not(:disabled),
	:global(body.light) .apple-btn:hover:not(:disabled) {
		background: rgba(0, 0, 0, 0.04);
		border-color: #1d1d1f;
	}
</style>

<script lang="ts">
	/**
	 * Consent Banner Component - Svelte 5 Runes
	 *
	 * Displays a GDPR/CCPA-compliant cookie consent banner.
	 * Shows only when the user hasn't made a consent choice yet.
	 *
	 * Features:
	 * - Accept All / Reject All quick actions
	 * - Link to detailed preferences modal
	 * - Accessible and keyboard-navigable
	 * - Respects reduced motion preferences
	 * - Template system support for customizable designs
	 *
	 * @version 2.0.0 - November 2025 (Svelte 5 Runes)
	 * @component
	 */

	import { fly, fade } from 'svelte/transition';
	import { browser } from '$app/environment';
	import {
		consentStore,
		showConsentBanner,
		openPreferencesModal,
	} from '../store';
	import { activeTemplate, isPreviewMode } from '../templates/store';
	import BannerRenderer from '../templates/BannerRenderer.svelte';

	// Svelte 5 props interface
	interface Props {
		position?: 'bottom' | 'top';
		useTemplates?: boolean;
		class?: string;
	}

	let { position = 'bottom', useTemplates = true, class: className = '' }: Props = $props();

	// Handle accept all
	function handleAcceptAll(): void {
		consentStore.acceptAll();
	}

	// Handle reject all
	function handleRejectAll(): void {
		consentStore.rejectAll();
	}

	// Open preferences modal
	function handleManagePreferences(): void {
		openPreferencesModal();
	}

	// Svelte 5 derived runes
	let prefersReducedMotion = $derived(
		browser ? window.matchMedia('(prefers-reduced-motion: reduce)').matches : false
	);

	let transitionOptions = $derived(
		prefersReducedMotion
			? { duration: 0 }
			: { y: position === 'bottom' ? 100 : -100, duration: 300 }
	);
</script>

{#if useTemplates}
	<!-- Template-based rendering -->
	<BannerRenderer />
{:else}
	<!-- Legacy hardcoded banner -->
	{#if $showConsentBanner}
		<div
			class="consent-banner {position} {className}"
			role="dialog"
			aria-modal="false"
			aria-labelledby="consent-banner-title"
			aria-describedby="consent-banner-description"
			transition:fly={transitionOptions}
		>
			<div class="consent-banner-content">
				<div class="consent-banner-text">
					<h2 id="consent-banner-title" class="consent-banner-title">
						We value your privacy
					</h2>
					<p id="consent-banner-description" class="consent-banner-description">
						We use cookies and similar technologies to enhance your experience,
						analyze site traffic, and for marketing purposes. You can choose to
						accept all cookies, reject non-essential ones, or customize your
						preferences.
					</p>
				</div>

				<div class="consent-banner-actions">
					<button
						type="button"
						class="consent-btn consent-btn-secondary"
						on:click={handleManagePreferences}
					>
						Manage Preferences
					</button>
					<button
						type="button"
						class="consent-btn consent-btn-outline"
						on:click={handleRejectAll}
					>
						Reject All
					</button>
					<button
						type="button"
						class="consent-btn consent-btn-primary"
						on:click={handleAcceptAll}
					>
						Accept All
					</button>
				</div>
			</div>
		</div>
	{/if}
{/if}

<style>
	.consent-banner {
		position: fixed;
		left: 0;
		right: 0;
		z-index: 9999;
		padding: 1rem;
		background: linear-gradient(135deg, #1a1f2e 0%, #0d1117 100%);
		border-top: 1px solid rgba(255, 255, 255, 0.1);
		box-shadow: 0 -4px 20px rgba(0, 0, 0, 0.3);
	}

	.consent-banner.bottom {
		bottom: 0;
	}

	.consent-banner.top {
		top: 0;
		border-top: none;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
	}

	.consent-banner-content {
		max-width: 1200px;
		margin: 0 auto;
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
	}

	.consent-banner-text {
		flex: 1;
		min-width: 280px;
	}

	.consent-banner-title {
		margin: 0 0 0.5rem;
		font-size: 1.125rem;
		font-weight: 600;
		color: #ffffff;
	}

	.consent-banner-description {
		margin: 0;
		font-size: 0.875rem;
		color: rgba(255, 255, 255, 0.8);
		line-height: 1.5;
	}

	.consent-banner-actions {
		display: flex;
		flex-wrap: wrap;
		gap: 0.75rem;
	}

	.consent-btn {
		padding: 0.625rem 1.25rem;
		font-size: 0.875rem;
		font-weight: 500;
		border-radius: 0.5rem;
		cursor: pointer;
		transition: all 0.2s ease;
		white-space: nowrap;
		border: none;
	}

	.consent-btn:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	.consent-btn-primary {
		background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);
		color: #ffffff;
	}

	.consent-btn-primary:hover {
		background: linear-gradient(135deg, #2563eb 0%, #1d4ed8 100%);
		transform: translateY(-1px);
	}

	.consent-btn-secondary {
		background: transparent;
		color: #3b82f6;
		border: 1px solid transparent;
	}

	.consent-btn-secondary:hover {
		background: rgba(59, 130, 246, 0.1);
	}

	.consent-btn-outline {
		background: transparent;
		color: rgba(255, 255, 255, 0.9);
		border: 1px solid rgba(255, 255, 255, 0.3);
	}

	.consent-btn-outline:hover {
		background: rgba(255, 255, 255, 0.1);
		border-color: rgba(255, 255, 255, 0.5);
	}

	/* Responsive adjustments */
	@media (max-width: 768px) {
		.consent-banner-content {
			flex-direction: column;
			text-align: center;
		}

		.consent-banner-actions {
			width: 100%;
			justify-content: center;
		}

		.consent-btn {
			flex: 1;
			min-width: 100px;
		}
	}

	/* Reduced motion */
	@media (prefers-reduced-motion: reduce) {
		.consent-btn {
			transition: none;
		}

		.consent-btn-primary:hover {
			transform: none;
		}
	}
</style>

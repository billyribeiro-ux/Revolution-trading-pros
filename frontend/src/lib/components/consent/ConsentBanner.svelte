<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	/**
	 * ConsentBanner - GDPR/CCPA Compliant Cookie Consent
	 *
	 * Matches Consent Magic Pro features:
	 * - Multi-category consent (necessary, functional, analytics, marketing, personalization)
	 * - Granular preference management
	 * - GeoIP-based consent requirements
	 * - Customizable styling and positioning
	 * - Consent history tracking
	 * - Integration with backend consent service
	 */

	interface ConsentCategory {
		id: string;
		name: string;
		description: string;
		required: boolean;
		default: boolean;
	}

	interface Props {
		apiEndpoint?: string;
		position?: 'bottom' | 'top' | 'center';
		theme?: 'light' | 'dark';
		blurBackground?: boolean;
		showPreferencesLink?: boolean;
		privacyPolicyUrl?: string;
		cookiePolicyUrl?: string;
		onConsentChange?: (consents: Record<string, boolean>) => void;
	}

	let {
		apiEndpoint = '/api/consent',
		position = 'bottom',
		theme = 'light',
		blurBackground = true,
		showPreferencesLink = true,
		privacyPolicyUrl = '/privacy-policy',
		cookiePolicyUrl = '/cookie-policy',
		onConsentChange
	}: Props = $props();

	// State
	let showBanner = $state(false);
	let showPreferences = $state(false);
	let isLoading = $state(true);
	let isSaving = $state(false);
	let categories: ConsentCategory[] = $state([]);
	let consents: Record<string, boolean> = $state({});
	let consentId: string | null = $state(null);

	// Default categories if API doesn't provide them
	const defaultCategories: ConsentCategory[] = [
		{
			id: 'necessary',
			name: 'Strictly Necessary',
			description:
				'Essential cookies required for the website to function properly. These cannot be disabled.',
			required: true,
			default: true
		},
		{
			id: 'functional',
			name: 'Functional',
			description:
				'Cookies that enable enhanced functionality and personalization, such as remembering your preferences.',
			required: false,
			default: false
		},
		{
			id: 'analytics',
			name: 'Analytics',
			description:
				'Cookies that help us understand how visitors interact with our website to improve user experience.',
			required: false,
			default: false
		},
		{
			id: 'marketing',
			name: 'Marketing',
			description:
				'Cookies used to track visitors across websites to display relevant advertisements.',
			required: false,
			default: false
		},
		{
			id: 'personalization',
			name: 'Personalization',
			description:
				'Cookies that enable personalized content and recommendations based on your behavior.',
			required: false,
			default: false
		}
	];

	// Get or create consent ID
	function getConsentId(): string {
		if (!browser) return '';

		let id = localStorage.getItem('consent_id');
		if (!id) {
			id = crypto.randomUUID();
			localStorage.setItem('consent_id', id);
		}
		return id;
	}

	// Check if consent has been given
	function hasStoredConsent(): boolean {
		if (!browser) return true;
		return localStorage.getItem('consent_given') === 'true';
	}

	// Load consent configuration from API
	async function loadConsentConfig(): Promise<void> {
		try {
			const response = await fetch(`${apiEndpoint}/config`);
			if (response.ok) {
				const config = await response.json();
				categories = config.categories || defaultCategories;

				// Load stored consents or use defaults
				const stored = localStorage.getItem('consent_preferences');
				if (stored) {
					consents = JSON.parse(stored);
				} else {
					// Initialize with defaults
					categories.forEach((cat) => {
						consents[cat.id] = cat.required || cat.default;
					});
				}
			} else {
				categories = defaultCategories;
				initDefaultConsents();
			}
		} catch {
			categories = defaultCategories;
			initDefaultConsents();
		}
	}

	function initDefaultConsents(): void {
		categories.forEach((cat) => {
			consents[cat.id] = cat.required || cat.default;
		});
	}

	// Save consent preferences
	async function saveConsents(): Promise<void> {
		isSaving = true;

		try {
			// Save to local storage first
			localStorage.setItem('consent_preferences', JSON.stringify(consents));
			localStorage.setItem('consent_given', 'true');
			localStorage.setItem('consent_date', new Date().toISOString());

			// Send to backend
			await fetch(`${apiEndpoint}/record`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					identifier: consentId,
					consents,
					metadata: {
						source: 'consent_banner',
						user_agent: navigator.userAgent,
						timestamp: new Date().toISOString()
					}
				})
			});

			// Callback
			onConsentChange?.(consents);

			// Apply consent changes
			applyConsentChanges();

			// Hide banner
			showBanner = false;
			showPreferences = false;
		} catch (error) {
			console.error('Failed to save consent preferences:', error);
		} finally {
			isSaving = false;
		}
	}

	// Accept all cookies
	function acceptAll(): void {
		categories.forEach((cat) => {
			consents[cat.id] = true;
		});
		saveConsents();
	}

	// Reject all optional cookies
	function rejectAll(): void {
		categories.forEach((cat) => {
			consents[cat.id] = cat.required;
		});
		saveConsents();
	}

	// Apply consent changes to third-party scripts
	function applyConsentChanges(): void {
		if (!browser) return;

		// Dispatch custom event for other scripts to listen to
		window.dispatchEvent(
			new CustomEvent('consentChanged', {
				detail: consents
			})
		);

		// Handle analytics consent
		if (consents.analytics) {
			// Enable Google Analytics, etc.
			enableAnalytics();
		} else {
			disableAnalytics();
		}

		// Handle marketing consent
		if (consents.marketing) {
			enableMarketing();
		} else {
			disableMarketing();
		}
	}

	function enableAnalytics(): void {
		// Placeholder for enabling analytics scripts
		// Example: Initialize Google Analytics
		if (typeof window !== 'undefined' && (window as any).gtag) {
			(window as any).gtag('consent', 'update', {
				analytics_storage: 'granted'
			});
		}
	}

	function disableAnalytics(): void {
		// Placeholder for disabling analytics scripts
		if (typeof window !== 'undefined' && (window as any).gtag) {
			(window as any).gtag('consent', 'update', {
				analytics_storage: 'denied'
			});
		}
	}

	function enableMarketing(): void {
		// Placeholder for enabling marketing scripts
		if (typeof window !== 'undefined' && (window as any).gtag) {
			(window as any).gtag('consent', 'update', {
				ad_storage: 'granted',
				ad_user_data: 'granted',
				ad_personalization: 'granted'
			});
		}
	}

	function disableMarketing(): void {
		// Placeholder for disabling marketing scripts
		if (typeof window !== 'undefined' && (window as any).gtag) {
			(window as any).gtag('consent', 'update', {
				ad_storage: 'denied',
				ad_user_data: 'denied',
				ad_personalization: 'denied'
			});
		}
	}

	// Toggle category consent
	function toggleCategory(categoryId: string): void {
		const category = categories.find((c) => c.id === categoryId);
		if (category && !category.required) {
			consents[categoryId] = !consents[categoryId];
		}
	}

	// Open preferences modal
	function openPreferences(): void {
		showPreferences = true;
	}

	// Close preferences modal
	function closePreferences(): void {
		showPreferences = false;
	}

	// Initialize on mount
	onMount(async () => {
		if (!browser) return;

		consentId = getConsentId();

		await loadConsentConfig();

		// Show banner if consent hasn't been given
		if (!hasStoredConsent()) {
			showBanner = true;
		} else {
			// Apply stored consents
			applyConsentChanges();
		}

		isLoading = false;
	});

	// Derived classes
	let bannerClasses = $derived(
		`consent-banner consent-banner--${position} consent-banner--${theme} ${showBanner ? 'consent-banner--visible' : ''}`
	);

	let overlayClasses = $derived(
		`consent-overlay ${showBanner && blurBackground ? 'consent-overlay--visible' : ''}`
	);
</script>

{#if !isLoading}
	<!-- Overlay -->
	{#if blurBackground}
		<div class={overlayClasses}></div>
	{/if}

	<!-- Main Banner -->
	{#if showBanner && !showPreferences}
		<div class={bannerClasses} role="dialog" aria-labelledby="consent-title">
			<div class="consent-banner__content">
				<div class="consent-banner__text">
					<h2 id="consent-title" class="consent-banner__title">Cookie & Privacy Settings</h2>
					<p class="consent-banner__description">
						We use cookies and similar technologies to enhance your experience, analyze traffic, and
						for marketing purposes. Please choose your preferences below.
					</p>
					<div class="consent-banner__links">
						<a href={privacyPolicyUrl} target="_blank" rel="noopener">Privacy Policy</a>
						<a href={cookiePolicyUrl} target="_blank" rel="noopener">Cookie Policy</a>
					</div>
				</div>

				<div class="consent-banner__actions">
					<button type="button" class="consent-btn consent-btn--secondary" onclick={rejectAll}>
						Reject All
					</button>
					{#if showPreferencesLink}
						<button
							type="button"
							class="consent-btn consent-btn--outline"
							onclick={openPreferences}
						>
							Manage Preferences
						</button>
					{/if}
					<button type="button" class="consent-btn consent-btn--primary" onclick={acceptAll}>
						Accept All
					</button>
				</div>
			</div>
		</div>
	{/if}

	<!-- Preferences Modal -->
	{#if showPreferences}
		<div class="consent-modal-overlay" onclick={closePreferences}>
			<div
				class="consent-modal consent-modal--{theme}"
				role="dialog"
				aria-labelledby="preferences-title"
				onclick={(e) => e.stopPropagation()}
			>
				<div class="consent-modal__header">
					<h2 id="preferences-title" class="consent-modal__title">Cookie Preferences</h2>
					<button
						type="button"
						class="consent-modal__close"
						onclick={closePreferences}
						aria-label="Close"
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							width="24"
							height="24"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
						>
							<line x1="18" y1="6" x2="6" y2="18"></line>
							<line x1="6" y1="6" x2="18" y2="18"></line>
						</svg>
					</button>
				</div>

				<div class="consent-modal__body">
					<p class="consent-modal__description">
						Manage your cookie preferences. You can enable or disable different types of cookies
						below.
					</p>

					<div class="consent-categories">
						{#each categories as category (category.id)}
							<div class="consent-category">
								<div class="consent-category__header">
									<div class="consent-category__info">
										<h3 class="consent-category__name">{category.name}</h3>
										{#if category.required}
											<span class="consent-category__badge">Required</span>
										{/if}
									</div>
									<label class="consent-toggle">
										<input
											type="checkbox"
											checked={consents[category.id]}
											disabled={category.required}
											onchange={() => toggleCategory(category.id)}
										/>
										<span class="consent-toggle__slider"></span>
									</label>
								</div>
								<p class="consent-category__description">{category.description}</p>
							</div>
						{/each}
					</div>
				</div>

				<div class="consent-modal__footer">
					<button type="button" class="consent-btn consent-btn--secondary" onclick={rejectAll}>
						Reject All
					</button>
					<button type="button" class="consent-btn consent-btn--primary" onclick={saveConsents}>
						{isSaving ? 'Saving...' : 'Save Preferences'}
					</button>
				</div>
			</div>
		</div>
	{/if}

	<!-- Floating Preferences Button (when banner is hidden) -->
	{#if !showBanner && showPreferencesLink}
		<button
			type="button"
			class="consent-floating-btn consent-floating-btn--{theme}"
			onclick={() => {
				showBanner = true;
				showPreferences = true;
			}}
			aria-label="Cookie Settings"
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="20"
				height="20"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<circle cx="12" cy="12" r="3"></circle>
				<path
					d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"
				></path>
			</svg>
		</button>
	{/if}
{/if}

<style>
	/* Overlay */
	.consent-overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: rgba(0, 0, 0, 0.5);
		backdrop-filter: blur(4px);
		z-index: 9998;
		opacity: 0;
		visibility: hidden;
		transition: all 0.3s ease;
	}

	.consent-overlay--visible {
		opacity: 1;
		visibility: visible;
	}

	/* Main Banner */
	.consent-banner {
		position: fixed;
		left: 0;
		right: 0;
		z-index: 9999;
		padding: 1.5rem;
		transform: translateY(100%);
		transition: transform 0.3s ease;
	}

	.consent-banner--visible {
		transform: translateY(0);
	}

	.consent-banner--bottom {
		bottom: 0;
	}

	.consent-banner--top {
		top: 0;
		transform: translateY(-100%);
	}

	.consent-banner--top.consent-banner--visible {
		transform: translateY(0);
	}

	.consent-banner--center {
		top: 50%;
		left: 50%;
		right: auto;
		max-width: 600px;
		transform: translate(-50%, -50%) scale(0.9);
		opacity: 0;
		border-radius: 1rem;
	}

	.consent-banner--center.consent-banner--visible {
		transform: translate(-50%, -50%) scale(1);
		opacity: 1;
	}

	/* Light Theme */
	.consent-banner--light {
		background: white;
		color: #1a1a1a;
		box-shadow: 0 -4px 20px rgba(0, 0, 0, 0.15);
	}

	/* Dark Theme */
	.consent-banner--dark {
		background: #1a1a1a;
		color: #ffffff;
		box-shadow: 0 -4px 20px rgba(0, 0, 0, 0.3);
	}

	.consent-banner__content {
		max-width: 1200px;
		margin: 0 auto;
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 1.5rem;
	}

	.consent-banner__text {
		flex: 1;
		min-width: 300px;
	}

	.consent-banner__title {
		font-size: 1.125rem;
		font-weight: 600;
		margin: 0 0 0.5rem;
	}

	.consent-banner__description {
		font-size: 0.875rem;
		opacity: 0.9;
		margin: 0 0 0.5rem;
		line-height: 1.5;
	}

	.consent-banner__links {
		display: flex;
		gap: 1rem;
	}

	.consent-banner__links a {
		font-size: 0.75rem;
		opacity: 0.7;
		text-decoration: underline;
	}

	.consent-banner--light .consent-banner__links a {
		color: #2563eb;
	}

	.consent-banner--dark .consent-banner__links a {
		color: #60a5fa;
	}

	.consent-banner__actions {
		display: flex;
		gap: 0.75rem;
		flex-wrap: wrap;
	}

	/* Buttons */
	.consent-btn {
		padding: 0.625rem 1.25rem;
		border-radius: 0.5rem;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
	}

	.consent-btn--primary {
		background: #2563eb;
		color: white;
	}

	.consent-btn--primary:hover {
		background: #1d4ed8;
	}

	.consent-btn--secondary {
		background: #f3f4f6;
		color: #374151;
	}

	.consent-btn--secondary:hover {
		background: #e5e7eb;
	}

	.consent-banner--dark .consent-btn--secondary {
		background: #374151;
		color: #f3f4f6;
	}

	.consent-banner--dark .consent-btn--secondary:hover {
		background: #4b5563;
	}

	.consent-btn--outline {
		background: transparent;
		border: 1px solid currentColor;
		opacity: 0.8;
	}

	.consent-btn--outline:hover {
		opacity: 1;
	}

	/* Modal */
	.consent-modal-overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: rgba(0, 0, 0, 0.5);
		backdrop-filter: blur(4px);
		z-index: 10000;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 1rem;
	}

	.consent-modal {
		background: white;
		border-radius: 1rem;
		max-width: 600px;
		width: 100%;
		max-height: 90vh;
		overflow: hidden;
		display: flex;
		flex-direction: column;
	}

	.consent-modal--dark {
		background: #1a1a1a;
		color: white;
	}

	.consent-modal__header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1.25rem 1.5rem;
		border-bottom: 1px solid rgba(0, 0, 0, 0.1);
	}

	.consent-modal--dark .consent-modal__header {
		border-color: rgba(255, 255, 255, 0.1);
	}

	.consent-modal__title {
		font-size: 1.25rem;
		font-weight: 600;
		margin: 0;
	}

	.consent-modal__close {
		background: none;
		border: none;
		padding: 0.5rem;
		cursor: pointer;
		opacity: 0.7;
		transition: opacity 0.2s;
	}

	.consent-modal__close:hover {
		opacity: 1;
	}

	.consent-modal__body {
		padding: 1.5rem;
		overflow-y: auto;
		flex: 1;
	}

	.consent-modal__description {
		font-size: 0.875rem;
		opacity: 0.8;
		margin: 0 0 1.5rem;
	}

	.consent-categories {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.consent-category {
		padding: 1rem;
		border-radius: 0.5rem;
		background: rgba(0, 0, 0, 0.03);
	}

	.consent-modal--dark .consent-category {
		background: rgba(255, 255, 255, 0.05);
	}

	.consent-category__header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 0.5rem;
	}

	.consent-category__info {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.consent-category__name {
		font-size: 0.9375rem;
		font-weight: 600;
		margin: 0;
	}

	.consent-category__badge {
		font-size: 0.625rem;
		padding: 0.125rem 0.375rem;
		background: #2563eb;
		color: white;
		border-radius: 0.25rem;
		text-transform: uppercase;
		font-weight: 600;
	}

	.consent-category__description {
		font-size: 0.8125rem;
		opacity: 0.7;
		margin: 0;
		line-height: 1.5;
	}

	/* Toggle Switch */
	.consent-toggle {
		position: relative;
		display: inline-block;
		width: 48px;
		height: 26px;
	}

	.consent-toggle input {
		opacity: 0;
		width: 0;
		height: 0;
	}

	.consent-toggle__slider {
		position: absolute;
		cursor: pointer;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: #cbd5e1;
		transition: 0.3s;
		border-radius: 26px;
	}

	.consent-toggle__slider:before {
		position: absolute;
		content: '';
		height: 20px;
		width: 20px;
		left: 3px;
		bottom: 3px;
		background: white;
		transition: 0.3s;
		border-radius: 50%;
	}

	.consent-toggle input:checked + .consent-toggle__slider {
		background: #2563eb;
	}

	.consent-toggle input:checked + .consent-toggle__slider:before {
		transform: translateX(22px);
	}

	.consent-toggle input:disabled + .consent-toggle__slider {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.consent-modal__footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1.25rem 1.5rem;
		border-top: 1px solid rgba(0, 0, 0, 0.1);
	}

	.consent-modal--dark .consent-modal__footer {
		border-color: rgba(255, 255, 255, 0.1);
	}

	/* Floating Button */
	.consent-floating-btn {
		position: fixed;
		bottom: 1rem;
		left: 1rem;
		width: 48px;
		height: 48px;
		border-radius: 50%;
		border: none;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		box-shadow: 0 2px 10px rgba(0, 0, 0, 0.2);
		transition: all 0.2s;
		z-index: 9990;
	}

	.consent-floating-btn--light {
		background: white;
		color: #374151;
	}

	.consent-floating-btn--dark {
		background: #1a1a1a;
		color: white;
	}

	.consent-floating-btn:hover {
		transform: scale(1.1);
	}

	/* Responsive */
	@media (max-width: 640px) {
		.consent-banner__content {
			flex-direction: column;
			text-align: center;
		}

		.consent-banner__actions {
			width: 100%;
			justify-content: center;
		}

		.consent-btn {
			flex: 1;
		}
	}
</style>

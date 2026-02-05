<script lang="ts">
	/**
	 * CookieConsent - GDPR/CCPA Compliant Cookie Consent Banner
	 * ==========================================================
	 * Apple ICT 7+ Principal Engineer Grade - February 2026
	 *
	 * Features:
	 * - GDPR and CCPA compliant
	 * - Granular cookie category controls
	 * - Preference persistence
	 * - Accessible keyboard navigation
	 * - Smooth animations
	 * - Mobile-first responsive design
	 * - Analytics integration
	 *
	 * Cookie Categories:
	 * - Necessary: Required for site functionality (always enabled)
	 * - Analytics: Performance and usage tracking
	 * - Marketing: Advertising and personalization
	 * - Preferences: Remember user choices
	 *
	 * @accessibility WCAG 2.1 AA compliant
	 */
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { IconX, IconSettings, IconCheck } from '$lib/icons';

	// Props
	interface Props {
		/** Position of the banner */
		position?: 'bottom' | 'top' | 'bottom-left' | 'bottom-right';
		/** Show preferences panel immediately */
		showPreferences?: boolean;
		/** Company name for privacy policy link */
		companyName?: string;
		/** Privacy policy URL */
		privacyPolicyUrl?: string;
		/** Cookie policy URL */
		cookiePolicyUrl?: string;
		/** Callback when consent is given */
		onConsent?: (preferences: CookiePreferences) => void;
	}

	let {
		position = 'bottom',
		showPreferences = false,
		companyName = 'Revolution Trading Pros',
		privacyPolicyUrl = '/privacy',
		cookiePolicyUrl = '/cookies',
		onConsent
	}: Props = $props();

	// Types
	interface CookiePreferences {
		necessary: boolean; // Always true
		analytics: boolean;
		marketing: boolean;
		preferences: boolean;
		timestamp: string;
	}

	// State
	let isVisible = $state(false);
	let isPreferencesOpen = $state(false);
	let isExiting = $state(false);

	// Sync isPreferencesOpen from prop when it changes
	$effect(() => {
		isPreferencesOpen = showPreferences;
	});
	let preferences = $state<CookiePreferences>({
		necessary: true,
		analytics: false,
		marketing: false,
		preferences: false,
		timestamp: ''
	});

	// Cookie name
	const COOKIE_CONSENT_NAME = 'rtp_cookie_consent';
	const COOKIE_EXPIRY_DAYS = 365;

	onMount(() => {
		if (!browser) return;

		// Check for existing consent
		const existingConsent = getConsentCookie();

		if (!existingConsent) {
			// Show banner after short delay
			setTimeout(() => {
				isVisible = true;
			}, 1000);
		} else {
			// Apply existing preferences
			preferences = existingConsent;
			applyPreferences(existingConsent);
		}
	});

	function getConsentCookie(): CookiePreferences | null {
		if (!browser) return null;

		const value = document.cookie
			.split('; ')
			.find((row) => row.startsWith(`${COOKIE_CONSENT_NAME}=`))
			?.split('=')[1];

		if (value) {
			try {
				return JSON.parse(decodeURIComponent(value));
			} catch {
				return null;
			}
		}
		return null;
	}

	function setConsentCookie(prefs: CookiePreferences): void {
		if (!browser) return;

		const expires = new Date();
		expires.setTime(expires.getTime() + COOKIE_EXPIRY_DAYS * 24 * 60 * 60 * 1000);

		document.cookie = `${COOKIE_CONSENT_NAME}=${encodeURIComponent(JSON.stringify(prefs))}; expires=${expires.toUTCString()}; path=/; SameSite=Lax`;
	}

	function applyPreferences(prefs: CookiePreferences): void {
		if (!browser) return;

		// Apply analytics (Google Analytics, etc.)
		if (prefs.analytics) {
			enableAnalytics();
		} else {
			disableAnalytics();
		}

		// Apply marketing (Facebook Pixel, Google Ads, etc.)
		if (prefs.marketing) {
			enableMarketing();
		} else {
			disableMarketing();
		}

		// Apply preferences cookies
		if (!prefs.preferences) {
			// Clear preference-related cookies except consent
		}

		// Notify callback
		onConsent?.(prefs);
	}

	function enableAnalytics(): void {
		// Enable Google Analytics
		if ('gtag' in window) {
			(window as any).gtag('consent', 'update', {
				analytics_storage: 'granted'
			});
		}

		// Enable other analytics tools
		window.dispatchEvent(new CustomEvent('cookie-consent:analytics', { detail: true }));
	}

	function disableAnalytics(): void {
		if ('gtag' in window) {
			(window as any).gtag('consent', 'update', {
				analytics_storage: 'denied'
			});
		}

		window.dispatchEvent(new CustomEvent('cookie-consent:analytics', { detail: false }));
	}

	function enableMarketing(): void {
		if ('gtag' in window) {
			(window as any).gtag('consent', 'update', {
				ad_storage: 'granted',
				ad_user_data: 'granted',
				ad_personalization: 'granted'
			});
		}

		// Enable Facebook Pixel
		if ('fbq' in window) {
			(window as any).fbq('consent', 'grant');
		}

		window.dispatchEvent(new CustomEvent('cookie-consent:marketing', { detail: true }));
	}

	function disableMarketing(): void {
		if ('gtag' in window) {
			(window as any).gtag('consent', 'update', {
				ad_storage: 'denied',
				ad_user_data: 'denied',
				ad_personalization: 'denied'
			});
		}

		if ('fbq' in window) {
			(window as any).fbq('consent', 'revoke');
		}

		window.dispatchEvent(new CustomEvent('cookie-consent:marketing', { detail: false }));
	}

	async function handleAcceptAll(): Promise<void> {
		preferences = {
			necessary: true,
			analytics: true,
			marketing: true,
			preferences: true,
			timestamp: new Date().toISOString()
		};

		setConsentCookie(preferences);
		applyPreferences(preferences);
		await closeWithAnimation();
	}

	async function handleAcceptNecessary(): Promise<void> {
		preferences = {
			necessary: true,
			analytics: false,
			marketing: false,
			preferences: false,
			timestamp: new Date().toISOString()
		};

		setConsentCookie(preferences);
		applyPreferences(preferences);
		await closeWithAnimation();
	}

	async function handleSavePreferences(): Promise<void> {
		preferences.necessary = true; // Always required
		preferences.timestamp = new Date().toISOString();

		setConsentCookie(preferences);
		applyPreferences(preferences);
		await closeWithAnimation();
	}

	async function closeWithAnimation(): Promise<void> {
		isExiting = true;
		await new Promise((resolve) => setTimeout(resolve, 300));
		isVisible = false;
		isExiting = false;
	}

	function togglePreferences(): void {
		isPreferencesOpen = !isPreferencesOpen;
	}

	// Position classes
	function getPositionClass(): string {
		switch (position) {
			case 'top':
				return 'cookie-position-top';
			case 'bottom-left':
				return 'cookie-position-bottom-left';
			case 'bottom-right':
				return 'cookie-position-bottom-right';
			default:
				return 'cookie-position-bottom';
		}
	}
</script>

{#if isVisible}
	<div
		class="cookie-consent {getPositionClass()}"
		class:exiting={isExiting}
		role="dialog"
		aria-modal="false"
		aria-label="Cookie consent"
	>
		<div class="cookie-content">
			<!-- Header -->
			<div class="cookie-header">
				<div class="cookie-icon">
					<IconSettings size={24} />
				</div>
				<h2 class="cookie-title">Cookie Preferences</h2>
			</div>

			<!-- Description -->
			<p class="cookie-description">
				We use cookies to enhance your experience on {companyName}. Some cookies are necessary for
				the site to work, while others help us improve your experience and provide personalized
				content.
			</p>

			<!-- Preferences Panel -->
			{#if isPreferencesOpen}
				<div class="cookie-preferences">
					<!-- Necessary Cookies -->
					<div class="cookie-category">
						<div class="cookie-category-header">
							<label class="cookie-label">
								<input
									type="checkbox"
									checked={true}
									disabled
									class="cookie-checkbox"
									aria-describedby="necessary-desc"
								/>
								<span class="cookie-category-name">Necessary</span>
								<span class="cookie-badge required">Required</span>
							</label>
						</div>
						<p id="necessary-desc" class="cookie-category-desc">
							Essential for the website to function. Cannot be disabled.
						</p>
					</div>

					<!-- Analytics Cookies -->
					<div class="cookie-category">
						<div class="cookie-category-header">
							<label class="cookie-label">
								<input
									type="checkbox"
									bind:checked={preferences.analytics}
									class="cookie-checkbox"
									aria-describedby="analytics-desc"
								/>
								<span class="cookie-category-name">Analytics</span>
							</label>
						</div>
						<p id="analytics-desc" class="cookie-category-desc">
							Help us understand how visitors interact with our website by collecting anonymous usage
							data.
						</p>
					</div>

					<!-- Marketing Cookies -->
					<div class="cookie-category">
						<div class="cookie-category-header">
							<label class="cookie-label">
								<input
									type="checkbox"
									bind:checked={preferences.marketing}
									class="cookie-checkbox"
									aria-describedby="marketing-desc"
								/>
								<span class="cookie-category-name">Marketing</span>
							</label>
						</div>
						<p id="marketing-desc" class="cookie-category-desc">
							Used to deliver personalized advertisements and track ad campaign performance.
						</p>
					</div>

					<!-- Preferences Cookies -->
					<div class="cookie-category">
						<div class="cookie-category-header">
							<label class="cookie-label">
								<input
									type="checkbox"
									bind:checked={preferences.preferences}
									class="cookie-checkbox"
									aria-describedby="prefs-desc"
								/>
								<span class="cookie-category-name">Preferences</span>
							</label>
						</div>
						<p id="prefs-desc" class="cookie-category-desc">
							Remember your settings and preferences for a better experience.
						</p>
					</div>
				</div>
			{/if}

			<!-- Actions -->
			<div class="cookie-actions">
				{#if isPreferencesOpen}
					<button type="button" class="cookie-btn cookie-btn-secondary" onclick={togglePreferences}>
						Back
					</button>
					<button type="button" class="cookie-btn cookie-btn-primary" onclick={handleSavePreferences}>
						<IconCheck size={16} />
						Save Preferences
					</button>
				{:else}
					<button
						type="button"
						class="cookie-btn cookie-btn-outline"
						onclick={handleAcceptNecessary}
					>
						Necessary Only
					</button>
					<button type="button" class="cookie-btn cookie-btn-secondary" onclick={togglePreferences}>
						<IconSettings size={16} />
						Customize
					</button>
					<button type="button" class="cookie-btn cookie-btn-primary" onclick={handleAcceptAll}>
						<IconCheck size={16} />
						Accept All
					</button>
				{/if}
			</div>

			<!-- Links -->
			<div class="cookie-links">
				<a href={privacyPolicyUrl} class="cookie-link">Privacy Policy</a>
				<span class="cookie-link-separator" aria-hidden="true">|</span>
				<a href={cookiePolicyUrl} class="cookie-link">Cookie Policy</a>
			</div>
		</div>
	</div>
{/if}

<style>
	/* Base styles */
	.cookie-consent {
		position: fixed;
		z-index: 99999;
		background: #ffffff;
		border-radius: 16px;
		box-shadow:
			0 20px 25px -5px rgba(0, 0, 0, 0.1),
			0 10px 10px -5px rgba(0, 0, 0, 0.04);
		max-width: 480px;
		width: calc(100% - 2rem);
		animation: slideUp 0.3s ease-out;
	}

	.cookie-consent.exiting {
		animation: slideDown 0.3s ease-out forwards;
	}

	/* Position variants */
	.cookie-position-bottom {
		bottom: 1rem;
		left: 50%;
		transform: translateX(-50%);
	}

	.cookie-position-top {
		top: 1rem;
		left: 50%;
		transform: translateX(-50%);
	}

	.cookie-position-bottom-left {
		bottom: 1rem;
		left: 1rem;
	}

	.cookie-position-bottom-right {
		bottom: 1rem;
		right: 1rem;
	}

	/* Content */
	.cookie-content {
		padding: 1.5rem;
	}

	/* Header */
	.cookie-header {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 1rem;
	}

	.cookie-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
		border-radius: 10px;
		color: white;
	}

	.cookie-title {
		font-size: 1.125rem;
		font-weight: 700;
		color: #1f2937;
		margin: 0;
	}

	/* Description */
	.cookie-description {
		font-size: 0.875rem;
		color: #6b7280;
		line-height: 1.5;
		margin: 0 0 1.25rem 0;
	}

	/* Preferences panel */
	.cookie-preferences {
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		padding: 0.5rem;
		margin-bottom: 1.25rem;
	}

	.cookie-category {
		padding: 0.75rem;
		border-radius: 8px;
		transition: background-color 0.15s;
	}

	.cookie-category:hover {
		background: #f9fafb;
	}

	.cookie-category + .cookie-category {
		border-top: 1px solid #f3f4f6;
	}

	.cookie-category-header {
		display: flex;
		align-items: center;
	}

	.cookie-label {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		cursor: pointer;
		flex: 1;
	}

	.cookie-checkbox {
		width: 20px;
		height: 20px;
		accent-color: #3b82f6;
		cursor: pointer;
	}

	.cookie-checkbox:disabled {
		cursor: not-allowed;
		opacity: 0.7;
	}

	.cookie-category-name {
		font-size: 0.9375rem;
		font-weight: 600;
		color: #1f2937;
	}

	.cookie-badge {
		font-size: 0.6875rem;
		font-weight: 600;
		padding: 0.125rem 0.5rem;
		border-radius: 9999px;
		text-transform: uppercase;
		letter-spacing: 0.025em;
	}

	.cookie-badge.required {
		background: #dbeafe;
		color: #1d4ed8;
	}

	.cookie-category-desc {
		font-size: 0.8125rem;
		color: #6b7280;
		margin: 0.375rem 0 0 2rem;
		line-height: 1.4;
	}

	/* Actions */
	.cookie-actions {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		margin-bottom: 1rem;
	}

	.cookie-btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.375rem;
		padding: 0.625rem 1rem;
		font-size: 0.875rem;
		font-weight: 600;
		border-radius: 8px;
		border: none;
		cursor: pointer;
		transition: all 0.15s;
		white-space: nowrap;
		flex: 1;
		min-width: 100px;
	}

	.cookie-btn:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	.cookie-btn-primary {
		background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);
		color: white;
	}

	.cookie-btn-primary:hover {
		background: linear-gradient(135deg, #2563eb 0%, #1d4ed8 100%);
		transform: translateY(-1px);
	}

	.cookie-btn-secondary {
		background: #f3f4f6;
		color: #374151;
	}

	.cookie-btn-secondary:hover {
		background: #e5e7eb;
	}

	.cookie-btn-outline {
		background: transparent;
		border: 1px solid #d1d5db;
		color: #374151;
	}

	.cookie-btn-outline:hover {
		background: #f9fafb;
		border-color: #9ca3af;
	}

	/* Links */
	.cookie-links {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		font-size: 0.75rem;
	}

	.cookie-link {
		color: #6b7280;
		text-decoration: none;
		transition: color 0.15s;
	}

	.cookie-link:hover {
		color: #3b82f6;
		text-decoration: underline;
	}

	.cookie-link-separator {
		color: #d1d5db;
	}

	/* Animations */
	@keyframes slideUp {
		from {
			opacity: 0;
			transform: translateX(-50%) translateY(20px);
		}
		to {
			opacity: 1;
			transform: translateX(-50%) translateY(0);
		}
	}

	@keyframes slideDown {
		from {
			opacity: 1;
			transform: translateX(-50%) translateY(0);
		}
		to {
			opacity: 0;
			transform: translateX(-50%) translateY(20px);
		}
	}

	/* Position-specific animations */
	.cookie-position-bottom-left,
	.cookie-position-bottom-right {
		animation-name: slideUpCorner;
	}

	.cookie-position-bottom-left.exiting,
	.cookie-position-bottom-right.exiting {
		animation-name: slideDownCorner;
	}

	@keyframes slideUpCorner {
		from {
			opacity: 0;
			transform: translateY(20px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	@keyframes slideDownCorner {
		from {
			opacity: 1;
			transform: translateY(0);
		}
		to {
			opacity: 0;
			transform: translateY(20px);
		}
	}

	/* Mobile responsive */
	@media (max-width: 640px) {
		.cookie-consent {
			max-width: 100%;
			width: 100%;
			border-radius: 16px 16px 0 0;
			margin: 0;
		}

		.cookie-position-bottom,
		.cookie-position-bottom-left,
		.cookie-position-bottom-right {
			bottom: 0;
			left: 0;
			right: 0;
			transform: none;
		}

		.cookie-position-top {
			top: 0;
			left: 0;
			right: 0;
			transform: none;
			border-radius: 0 0 16px 16px;
		}

		.cookie-content {
			padding: 1.25rem;
			padding-bottom: calc(1.25rem + env(safe-area-inset-bottom, 0));
		}

		.cookie-actions {
			flex-direction: column;
		}

		.cookie-btn {
			width: 100%;
			flex: none;
		}

		@keyframes slideUp {
			from {
				opacity: 0;
				transform: translateY(100%);
			}
			to {
				opacity: 1;
				transform: translateY(0);
			}
		}

		@keyframes slideDown {
			from {
				opacity: 1;
				transform: translateY(0);
			}
			to {
				opacity: 0;
				transform: translateY(100%);
			}
		}
	}

	/* Accessibility: Reduced motion */
	@media (prefers-reduced-motion: reduce) {
		.cookie-consent {
			animation: none;
		}

		.cookie-btn {
			transition: none;
		}
	}

	/* Dark mode support */
	@media (prefers-color-scheme: dark) {
		.cookie-consent {
			background: #1f2937;
		}

		.cookie-title {
			color: #f9fafb;
		}

		.cookie-description {
			color: #9ca3af;
		}

		.cookie-preferences {
			border-color: #374151;
		}

		.cookie-category:hover {
			background: #374151;
		}

		.cookie-category + .cookie-category {
			border-color: #374151;
		}

		.cookie-category-name {
			color: #f9fafb;
		}

		.cookie-category-desc {
			color: #9ca3af;
		}

		.cookie-btn-secondary {
			background: #374151;
			color: #f3f4f6;
		}

		.cookie-btn-secondary:hover {
			background: #4b5563;
		}

		.cookie-btn-outline {
			border-color: #4b5563;
			color: #f3f4f6;
		}

		.cookie-btn-outline:hover {
			background: #374151;
			border-color: #6b7280;
		}

		.cookie-link {
			color: #9ca3af;
		}

		.cookie-link-separator {
			color: #4b5563;
		}
	}
</style>

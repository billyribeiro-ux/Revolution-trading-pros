<script lang="ts">
	/**
	 * SimplerConsentBanner - EXACT Simpler Trading Consent Magic Pro Match
	 *
	 * This component replicates the EXACT layout, styling, and wording
	 * from Simpler Trading's Consent Magic Pro implementation.
	 *
	 * @version 1.0.0 - December 2025
	 */

	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { fly } from 'svelte/transition';

	interface Props {
		termsUrl?: string;
		privacyUrl?: string;
		onAccept?: () => void;
		onReject?: () => void;
		onMoreOptions?: () => void;
	}

	let {
		termsUrl = '/terms-and-conditions',
		privacyUrl = '/privacy-policy',
		onAccept,
		onReject,
		onMoreOptions
	}: Props = $props();

	// State
	let showBanner = $state(false);
	let showPreferencesModal = $state(false);
	let isLoading = $state(true);
	let isSaving = $state(false);

	// Consent categories matching Consent Magic Pro
	interface ConsentCategory {
		id: string;
		name: string;
		description: string;
		required: boolean;
		enabled: boolean;
	}

	let categories: ConsentCategory[] = $state([
		{
			id: 'necessary',
			name: 'Strictly Necessary Cookies',
			description: 'These cookies are essential for the website to function properly and cannot be disabled.',
			required: true,
			enabled: true
		},
		{
			id: 'functional',
			name: 'Functional Cookies',
			description: 'These cookies enable enhanced functionality and personalization.',
			required: false,
			enabled: false
		},
		{
			id: 'analytics',
			name: 'Analytics Cookies',
			description: 'These cookies help us understand how visitors interact with our website.',
			required: false,
			enabled: false
		},
		{
			id: 'marketing',
			name: 'Marketing Cookies',
			description: 'These cookies are used to track visitors across websites to display relevant advertisements.',
			required: false,
			enabled: false
		}
	]);

	// Check if consent has been given
	function hasStoredConsent(): boolean {
		if (!browser) return true;
		return localStorage.getItem('revolution_consent_given') === 'true';
	}

	// Save consent to localStorage and API
	async function saveConsent(acceptAll: boolean): Promise<void> {
		if (!browser) return;
		isSaving = true;

		try {
			const consents: Record<string, boolean> = {};
			categories.forEach(cat => {
				consents[cat.id] = acceptAll ? true : (cat.required ? true : cat.enabled);
			});

			// Save to localStorage
			localStorage.setItem('revolution_consent_given', 'true');
			localStorage.setItem('revolution_consent_preferences', JSON.stringify(consents));
			localStorage.setItem('revolution_consent_date', new Date().toISOString());

			// Send to backend API
			try {
				await fetch('/api/consent/record', {
					method: 'POST',
					headers: { 'Content-Type': 'application/json' },
					body: JSON.stringify({
						consents,
						timestamp: new Date().toISOString(),
						user_agent: navigator.userAgent
					})
				});
			} catch {
				// Silent fail for API - consent is still saved locally
			}

			// Dispatch event for other scripts
			window.dispatchEvent(new CustomEvent('revolutionConsentChanged', { detail: consents }));

			// Apply consent changes
			applyConsent(consents);

			showBanner = false;
			showPreferencesModal = false;

			if (acceptAll) {
				onAccept?.();
			}
		} finally {
			isSaving = false;
		}
	}

	function applyConsent(consents: Record<string, boolean>): void {
		if (!browser) return;

		// Google Analytics consent mode
		if (typeof (window as any).gtag === 'function') {
			(window as any).gtag('consent', 'update', {
				analytics_storage: consents.analytics ? 'granted' : 'denied',
				ad_storage: consents.marketing ? 'granted' : 'denied',
				ad_user_data: consents.marketing ? 'granted' : 'denied',
				ad_personalization: consents.marketing ? 'granted' : 'denied'
			});
		}
	}

	function handleAccept(): void {
		saveConsent(true);
	}

	function handleReject(): void {
		// Reset all optional categories to false
		categories = categories.map(cat => ({
			...cat,
			enabled: cat.required ? true : false
		}));
		saveConsent(false);
		onReject?.();
	}

	function handleMoreOptions(): void {
		showPreferencesModal = true;
		onMoreOptions?.();
	}

	function handleClose(): void {
		showBanner = false;
	}

	function handleSavePreferences(): void {
		saveConsent(false);
	}

	function toggleCategory(id: string): void {
		categories = categories.map(cat => {
			if (cat.id === id && !cat.required) {
				return { ...cat, enabled: !cat.enabled };
			}
			return cat;
		});
	}

	function handleDisableAll(): void {
		categories = categories.map(cat => ({
			...cat,
			enabled: cat.required ? true : false
		}));
		saveConsent(false);
	}

	function handleAllowAll(): void {
		categories = categories.map(cat => ({
			...cat,
			enabled: true
		}));
		saveConsent(true);
	}

	onMount(() => {
		if (!browser) return;

		if (!hasStoredConsent()) {
			showBanner = true;
		} else {
			// Load and apply stored preferences
			const stored = localStorage.getItem('revolution_consent_preferences');
			if (stored) {
				const consents = JSON.parse(stored);
				applyConsent(consents);
			}
		}

		isLoading = false;
	});
</script>

{#if !isLoading && showBanner && !showPreferencesModal}
	<!-- Main Consent Banner - EXACT Simpler Trading Match -->
	<div
		class="cs-info-bar cs-public-cookie-bar bar_large cs-bar bottom cs_light_theme"
		role="dialog"
		aria-labelledby="consent-title"
		aria-modal="true"
		transition:fly={{ y: 100, duration: 300 }}
	>
		<div class="cs-wrapper">
			<!-- Close Button -->
			<button
				type="button"
				class="cs-modal-close"
				onclick={handleClose}
				aria-label="Close"
			>
				<svg viewBox="0 0 24 24">
					<path d="M19 6.41l-1.41-1.41-5.59 5.59-5.59-5.59-1.41 1.41 5.59 5.59-5.59 5.59 1.41 1.41 5.59-5.59 5.59 5.59 1.41-1.41-5.59-5.59z"></path>
					<path d="M0 0h24v24h-24z" fill="none"></path>
				</svg>
				<span class="cs-sr-only">Close</span>
			</button>

			<div class="cs-popup-row">
				<!-- Description -->
				<div class="bar-description">
					<div class="bar_description_text">
						<p>
							<em>By clicking "Accept," you agree to our </em>
							<a href={termsUrl}><em><u>Terms of Use</u></em></a>
							<em>, </em>
							<a href={privacyUrl}><em><u>Privacy Policy</u></em></a>
							<em> and consent to the use of cookies and similar tracking technologies to, among other things, serve you relevant ads ourselves or through our third-party ad partners with whom data from cookies is shared.</em>
						</p>
					</div>
					<div class="policy_wrap">
						<a href={privacyUrl} target="_blank" rel="noopener">Privacy Policy</a>
					</div>
				</div>

				<!-- Buttons -->
				<div class="btns_column cs_deny_all_btn">
					<div class="btn-row">
						<button
							type="button"
							class="btn options_btn"
							onclick={handleMoreOptions}
						>
							More options
						</button>
						<button
							type="button"
							class="btn disable_all_btn cs_action_btn"
							onclick={handleReject}
						>
							Reject
						</button>
						<button
							type="button"
							class="btn btn-grey allow_all_btn cs_action_btn"
							onclick={handleAccept}
						>
							Accept
						</button>
					</div>
				</div>
			</div>
		</div>
	</div>
{/if}

{#if showPreferencesModal}
	<!-- Preferences Modal - EXACT Simpler Trading Match -->
	<div class="cs-modal-overlay">
		<div
			class="cs-modal cs_settings_popup cs_light_theme cs_multi_design cs-deny-all"
			role="dialog"
			aria-labelledby="preferences-title"
			aria-modal="true"
		>
			<div class="cs-modal-dialog" role="document">
				<div class="cs-modal-content cs-bar-popup">
					<!-- Close Button -->
					<button
						type="button"
						class="cs-modal-close"
						onclick={() => showPreferencesModal = false}
						aria-label="Close"
					>
						<svg viewBox="0 0 24 24">
							<path d="M19 6.41l-1.41-1.41-5.59 5.59-5.59-5.59-1.41 1.41 5.59 5.59-5.59 5.59 1.41 1.41 5.59-5.59 5.59 5.59 1.41-1.41-5.59-5.59z"></path>
							<path d="M0 0h24v24h-24z" fill="none"></path>
						</svg>
						<span class="cs-sr-only">Close</span>
					</button>

					<div class="cs-modal-body">
						<div class="cs-container-fluid cs-tab-container ask_before_tracking cs_deny_all_btn">
							<!-- Privacy Overview -->
							<div class="cs-privacy-overview">
								<h4 id="preferences-title">Your privacy settings</h4>
								<div class="cs-privacy-content">
									<div class="cs-privacy-content-text">
										<p>
											<em>By clicking "Accept," you agree to our </em>
											<a href={termsUrl}><em><u>Terms of Use</u></em></a>
											<em>, </em>
											<a href={privacyUrl}><em><u>Privacy Policy</u></em></a>
											<em> and consent to the use of cookies and similar tracking technologies to, among other things, serve you relevant ads ourselves or through our third-party ad partners with whom data from cookies is shared.</em>
										</p>
										<div class="privacy-link">
											<div class="policy_wrap">
												<a href={privacyUrl} target="_blank" rel="noopener">Privacy Policy</a>
											</div>
										</div>
									</div>

									<div class="cs_policy_existing_page">
										<div class="btn-row">
											<button
												type="button"
												class="btn disable_all_btn cs_action_btn"
												onclick={handleDisableAll}
											>
												Disable all
											</button>
											<button
												type="button"
												class="btn btn-grey allow_all_btn cs_action_btn"
												onclick={handleAllowAll}
											>
												Allow all
											</button>
										</div>
									</div>
								</div>
							</div>

							<div class="line"></div>

							<!-- Manage Consent Preferences -->
							<div class="cs_popup_content">
								<h4>Manage Consent Preferences</h4>

								{#each categories as category (category.id)}
									<div class="cm-script-title-block">
										<div class="cm-script-header">
											<h4 class="cm-script-title">{category.name}</h4>
											<label class="cs-toggle-switch">
												<input
													type="checkbox"
													checked={category.enabled}
													disabled={category.required}
													onchange={() => toggleCategory(category.id)}
												/>
												<span class="cs-toggle-slider" class:required={category.required}></span>
												{#if category.required}
													<span class="cs-toggle-label">Always Active</span>
												{/if}
											</label>
										</div>
										<p class="cm-script-description">{category.description}</p>
									</div>
								{/each}
							</div>

							<!-- Save Button -->
							<div class="cs-modal-footer">
								<button
									type="button"
									class="btn btn-grey allow_all_btn cs_action_btn"
									onclick={handleSavePreferences}
									disabled={isSaving}
								>
									{isSaving ? 'Saving...' : 'Confirm My Choices'}
								</button>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
{/if}

<style>
	/* ============================================
	 * EXACT SIMPLER TRADING CONSENT MAGIC PRO CSS
	 * Matches WordPress implementation 100%
	 * ============================================ */

	/* Main Banner */
	.cs-info-bar {
		position: fixed;
		left: 0;
		right: 0;
		bottom: 0;
		z-index: 999999;
		background-color: #ffffff;
		border: 1px solid #333333;
		color: #5D5D5D;
		fill: #5D5D5D;
		padding: 45px 80px 40px 80px;
		font-size: 20px;
		font-weight: 400;
		min-height: 140px;
		font-family: Arial, sans-serif;
		box-shadow: 0 -4px 20px rgba(0, 0, 0, 0.15);
	}

	.cs-wrapper {
		max-width: 1400px;
		margin: 0 auto;
		position: relative;
	}

	/* Close Button */
	.cs-modal-close {
		position: absolute;
		top: -25px;
		right: -60px;
		width: 32px;
		height: 32px;
		background: transparent;
		border: none;
		cursor: pointer;
		padding: 0;
		color: #5D5D5D;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.cs-modal-close svg {
		width: 24px;
		height: 24px;
		fill: currentColor;
	}

	.cs-modal-close:hover {
		color: #333333;
	}

	.cs-sr-only {
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

	/* Layout */
	.cs-popup-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 40px;
		flex-wrap: wrap;
	}

	.bar-description {
		flex: 1;
		min-width: 400px;
	}

	.bar_description_text {
		color: #5D5D5D;
		font-size: 20px;
		font-weight: 400;
		line-height: 1.5;
	}

	.bar_description_text p {
		margin: 0;
	}

	.bar_description_text em {
		font-style: italic;
	}

	.bar_description_text a {
		color: #005BD3;
		text-decoration: none;
	}

	.bar_description_text a:hover {
		text-decoration: underline;
	}

	.bar_description_text u {
		text-decoration: underline;
		text-underline-offset: 2px;
	}

	.policy_wrap {
		margin-top: 15px;
	}

	.policy_wrap a {
		color: #005BD3;
		font-size: 16px;
		text-decoration: none;
	}

	.policy_wrap a:hover {
		text-decoration: underline;
	}

	/* Buttons Column */
	.btns_column {
		flex-shrink: 0;
	}

	.btn-row {
		display: flex;
		align-items: center;
		gap: 0;
		flex-wrap: wrap;
	}

	.btn {
		padding: 20px 25px 20px 20px;
		margin: 25px 20px 0px 0px;
		font-size: 16px;
		font-weight: 500;
		border: none;
		cursor: pointer;
		transition: all 0.2s ease;
		font-family: Arial, sans-serif;
	}

	.options_btn,
	.disable_all_btn {
		background-color: #F0F0F0;
		color: #212121;
	}

	.options_btn:hover,
	.disable_all_btn:hover {
		background-color: #E0E0E0;
	}

	.allow_all_btn {
		background-color: #E16B43;
		color: #ffffff;
	}

	.allow_all_btn:hover {
		background-color: #c95a36;
	}

	.allow_all_btn:disabled {
		opacity: 0.7;
		cursor: not-allowed;
	}

	/* Modal Overlay */
	.cs-modal-overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: rgba(0, 0, 0, 0.5);
		z-index: 9999999;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 20px;
	}

	/* Modal */
	.cs-modal {
		width: 100%;
		max-width: 700px;
		max-height: 90vh;
		overflow: hidden;
	}

	.cs-modal-dialog {
		width: 100%;
	}

	.cs-modal-content {
		background-color: #ffffff;
		border: 1px solid #333333;
		color: #5D5D5D;
		fill: #5D5D5D;
		padding: 40px 20px 40px 20px;
		font-size: 18px;
		font-weight: 400;
		min-height: 400px;
		border-radius: 0;
		position: relative;
		max-height: 80vh;
		overflow-y: auto;
	}

	.cs-modal .cs-modal-close {
		position: absolute;
		top: 15px;
		right: 15px;
	}

	.cs-modal-body {
		padding: 0;
	}

	/* Privacy Overview */
	.cs-privacy-overview {
		font-size: 18px;
		font-weight: 400;
	}

	.cs-privacy-overview > h4 {
		color: #252A31;
		font-size: 20px;
		font-weight: 500;
		margin: 0 0 16px 0;
	}

	.cs-privacy-content {
		display: flex;
		flex-direction: column;
		gap: 20px;
	}

	.cs-privacy-content-text {
		color: #5D5D5D;
		line-height: 1.5;
	}

	.cs-privacy-content-text p {
		margin: 0 0 10px 0;
	}

	.cs-privacy-content-text a {
		color: #005BD3;
		text-decoration: none;
	}

	.privacy-link {
		margin-top: 10px;
	}

	.cs_policy_existing_page {
		margin-top: 10px;
	}

	.cs_policy_existing_page .btn-row {
		justify-content: flex-start;
	}

	.cs_policy_existing_page .btn {
		padding: 20px 35px;
		margin: 0 0 0 20px;
	}

	.cs_policy_existing_page .btn:first-child {
		margin-left: 0;
	}

	/* Divider Line */
	.line {
		height: 1px;
		background: #E0E0E0;
		margin: 30px 0;
	}

	/* Popup Content */
	.cs_popup_content {
		padding: 0;
	}

	.cs_popup_content > h4 {
		color: #252A31;
		font-size: 20px;
		font-weight: 500;
		margin: 0 0 16px 0;
	}

	/* Cookie Category Block */
	.cm-script-title-block {
		padding: 16px;
		background: #F8F9FA;
		border-radius: 4px;
		margin-bottom: 12px;
	}

	.cm-script-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 8px;
	}

	.cm-script-title {
		color: #5D5D5D;
		font-size: 14px;
		font-weight: 600;
		margin: 0;
	}

	.cm-script-description {
		color: #5D5D5D;
		font-size: 13px;
		margin: 0;
		line-height: 1.5;
		opacity: 0.8;
	}

	/* Toggle Switch - Simpler Trading Style */
	.cs-toggle-switch {
		position: relative;
		display: flex;
		align-items: center;
		gap: 8px;
		cursor: pointer;
	}

	.cs-toggle-switch input {
		opacity: 0;
		width: 0;
		height: 0;
		position: absolute;
	}

	.cs-toggle-slider {
		position: relative;
		width: 44px;
		height: 24px;
		background-color: #CBD5E1;
		border-radius: 24px;
		transition: all 0.3s ease;
	}

	.cs-toggle-slider::before {
		content: '';
		position: absolute;
		width: 18px;
		height: 18px;
		left: 3px;
		top: 3px;
		background-color: white;
		border-radius: 50%;
		transition: all 0.3s ease;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
	}

	.cs-toggle-switch input:checked + .cs-toggle-slider {
		background-color: #0172CB;
	}

	.cs-toggle-switch input:checked + .cs-toggle-slider::before {
		transform: translateX(20px);
	}

	.cs-toggle-switch input:disabled + .cs-toggle-slider {
		background-color: #0172CB;
		opacity: 0.7;
		cursor: not-allowed;
	}

	.cs-toggle-label {
		font-size: 12px;
		color: #0172CB;
		font-weight: 500;
	}

	/* Modal Footer */
	.cs-modal-footer {
		margin-top: 24px;
		padding-top: 20px;
		border-top: 1px solid #E0E0E0;
		display: flex;
		justify-content: center;
	}

	.cs-modal-footer .btn {
		margin: 0;
		padding: 16px 40px;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.cs-info-bar {
			padding: 30px 20px;
			font-size: 16px;
		}

		.cs-popup-row {
			flex-direction: column;
			gap: 20px;
		}

		.bar-description {
			min-width: 100%;
		}

		.bar_description_text {
			font-size: 16px;
		}

		.btn-row {
			flex-direction: column;
			width: 100%;
		}

		.btn {
			width: 100%;
			margin: 10px 0 0 0;
			text-align: center;
		}

		.cs-modal-close {
			top: 10px;
			right: 10px;
		}

		.cs-modal-content {
			padding: 50px 15px 30px 15px;
		}

		.cs_policy_existing_page .btn-row {
			flex-direction: column;
		}

		.cs_policy_existing_page .btn {
			margin: 10px 0 0 0;
			width: 100%;
		}

		.cs_policy_existing_page .btn:first-child {
			margin-left: 0;
			margin-top: 0;
		}
	}
</style>

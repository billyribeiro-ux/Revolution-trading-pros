<script lang="ts">
	/**
	 * Admin Consent Settings Page
	 *
	 * Complete consent management configuration.
	 * Mirrors all functionality from Consent Magic Pro WordPress plugin.
	 *
	 * Features:
	 * - General settings (expiry, test mode, version)
	 * - Banner design customization
	 * - Script blocking configuration
	 * - Google/Bing Consent Mode integration
	 * - Geolocation-based rules
	 * - Proof of consent settings
	 *
	 * @version 2.0.0 - December 2025
	 */
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { adminFetch } from '$lib/utils/adminFetch';

	// Types
	interface ConsentSettings {
		// General
		consent_enabled: boolean;
		test_mode: boolean;
		expire_days: number;
		consent_version: number;
		policy_version: string;

		// Banner
		banner_enabled: boolean;
		banner_position: string;
		banner_layout: string;
		show_reject_button: boolean;
		show_settings_button: boolean;
		close_on_scroll: boolean;
		close_on_scroll_distance: number;

		// Script Blocking
		script_blocking_enabled: boolean;
		block_google_analytics: boolean;
		block_google_tag_manager: boolean;
		block_facebook_pixel: boolean;
		block_tiktok_pixel: boolean;
		block_twitter_pixel: boolean;
		block_linkedin_pixel: boolean;
		block_pinterest_tag: boolean;
		block_reddit_pixel: boolean;
		block_hotjar: boolean;
		block_youtube_embeds: boolean;
		block_vimeo_embeds: boolean;
		block_google_maps: boolean;

		// Integrations
		google_consent_mode_enabled: boolean;
		bing_consent_mode_enabled: boolean;

		// Geolocation
		geolocation_enabled: boolean;
		geo_default_strict: boolean;

		// Proof
		proof_consent_enabled: boolean;
		proof_retention_days: number;
		proof_auto_delete: boolean;

		[key: string]: any;
	}

	// State
	let settings: ConsentSettings = {
		consent_enabled: true,
		test_mode: false,
		expire_days: 365,
		consent_version: 1,
		policy_version: '1.0.0',
		banner_enabled: true,
		banner_position: 'bottom',
		banner_layout: 'bar',
		show_reject_button: true,
		show_settings_button: true,
		close_on_scroll: false,
		close_on_scroll_distance: 60,
		script_blocking_enabled: true,
		block_google_analytics: true,
		block_google_tag_manager: true,
		block_facebook_pixel: true,
		block_tiktok_pixel: true,
		block_twitter_pixel: true,
		block_linkedin_pixel: true,
		block_pinterest_tag: true,
		block_reddit_pixel: true,
		block_hotjar: true,
		block_youtube_embeds: true,
		block_vimeo_embeds: true,
		block_google_maps: true,
		google_consent_mode_enabled: true,
		bing_consent_mode_enabled: false,
		geolocation_enabled: false,
		geo_default_strict: true,
		proof_consent_enabled: true,
		proof_retention_days: 365,
		proof_auto_delete: true
	};

	let activeTab = 'general';
	let loading = true;
	let saving = false;
	let notification = '';
	let notificationType: 'success' | 'error' = 'success';

	// Tabs configuration
	const tabs = [
		{ id: 'general', label: 'General', icon: 'settings' },
		{ id: 'banner', label: 'Banner', icon: 'layout' },
		{ id: 'scripts', label: 'Script Blocking', icon: 'code' },
		{ id: 'integrations', label: 'Integrations', icon: 'plug' },
		{ id: 'geolocation', label: 'Geolocation', icon: 'globe' },
		{ id: 'proof', label: 'Proof of Consent', icon: 'file-check' }
	];

	// Script blocking options
	const scriptBlockingOptions = [
		{
			key: 'block_google_analytics',
			label: 'Google Analytics',
			description: 'Block GA4 and Universal Analytics tracking scripts',
			category: 'analytics'
		},
		{
			key: 'block_google_tag_manager',
			label: 'Google Tag Manager',
			description: 'Block GTM container scripts',
			category: 'marketing'
		},
		{
			key: 'block_facebook_pixel',
			label: 'Facebook/Meta Pixel',
			description: 'Block Meta Pixel for conversion tracking and remarketing',
			category: 'marketing'
		},
		{
			key: 'block_tiktok_pixel',
			label: 'TikTok Pixel',
			description: 'Block TikTok Pixel for ad tracking',
			category: 'marketing'
		},
		{
			key: 'block_twitter_pixel',
			label: 'Twitter/X Pixel',
			description: 'Block Twitter conversion tracking',
			category: 'marketing'
		},
		{
			key: 'block_linkedin_pixel',
			label: 'LinkedIn Insight Tag',
			description: 'Block LinkedIn conversion tracking',
			category: 'marketing'
		},
		{
			key: 'block_pinterest_tag',
			label: 'Pinterest Tag',
			description: 'Block Pinterest conversion tracking',
			category: 'marketing'
		},
		{
			key: 'block_reddit_pixel',
			label: 'Reddit Pixel',
			description: 'Block Reddit conversion tracking',
			category: 'marketing'
		},
		{
			key: 'block_hotjar',
			label: 'Hotjar',
			description: 'Block Hotjar heatmaps and session recordings',
			category: 'analytics'
		},
		{
			key: 'block_youtube_embeds',
			label: 'YouTube Embeds',
			description: 'Replace YouTube embeds with privacy-friendly placeholders',
			category: 'embedded_video'
		},
		{
			key: 'block_vimeo_embeds',
			label: 'Vimeo Embeds',
			description: 'Replace Vimeo embeds with privacy-friendly placeholders',
			category: 'embedded_video'
		},
		{
			key: 'block_google_maps',
			label: 'Google Maps',
			description: 'Block Google Maps embeds until consent is given',
			category: 'analytics'
		}
	];

	onMount(async () => {
		if (browser) {
			await loadSettings();
		}
	});

	async function loadSettings() {
		loading = true;
		try {
			const data = await adminFetch('/api/admin/consent/settings');
			if (data.success && data.data) {
				settings = { ...settings, ...data.data };
			}
		} catch (error) {
			console.error('Failed to load settings:', error);
			showNotification('Failed to load settings', 'error');
		}
		loading = false;
	}

	async function saveSettings() {
		saving = true;
		try {
			await adminFetch('/api/admin/consent/settings/bulk', {
				method: 'POST',
				body: JSON.stringify({ settings })
			});
			showNotification('Settings saved successfully', 'success');
		} catch (error) {
			console.error('Failed to save settings:', error);
			showNotification('Failed to save settings', 'error');
		}
		saving = false;
	}

	async function resetSettings() {
		if (!confirm('Are you sure you want to reset all settings to defaults?')) {
			return;
		}

		saving = true;
		try {
			const data = await adminFetch('/api/admin/consent/settings/reset', { method: 'POST' });
			if (data.success && data.data) {
				settings = { ...settings, ...data.data };
			}
			showNotification('Settings reset to defaults', 'success');
		} catch (error) {
			console.error('Failed to reset settings:', error);
			showNotification('Failed to reset settings', 'error');
		}
		saving = false;
	}

	function showNotification(message: string, type: 'success' | 'error') {
		notification = message;
		notificationType = type;
		setTimeout(() => {
			notification = '';
		}, 3000);
	}

	function incrementVersion() {
		settings.consent_version = settings.consent_version + 1;
	}
</script>

<svelte:head>
	<title>Consent Settings | Admin</title>
</svelte:head>

<div class="settings-page">
	<!-- Header -->
	<header class="page-header">
		<div class="header-content">
			<h1>Consent Management Settings</h1>
			<p>Configure your GDPR/CCPA-compliant cookie consent system</p>
		</div>
		<div class="header-actions">
			<button class="btn btn-secondary" onclick={resetSettings} disabled={saving}>
				Reset to Defaults
			</button>
			<button class="btn btn-primary" onclick={saveSettings} disabled={saving}>
				{#if saving}
					<span class="spinner"></span>
					Saving...
				{:else}
					Save Changes
				{/if}
			</button>
		</div>
	</header>

	<!-- Navigation Tabs -->
	<nav class="tabs">
		{#each tabs as tab}
			<button class="tab" class:active={activeTab === tab.id} onclick={() => (activeTab = tab.id)}>
				<span class="tab-icon"
					>{tab.icon === 'settings'
						? '⚙️'
						: tab.icon === 'layout'
							? '🎨'
							: tab.icon === 'code'
								? '📜'
								: tab.icon === 'plug'
									? '🔌'
									: tab.icon === 'globe'
										? '🌍'
										: '📋'}</span
				>
				{tab.label}
			</button>
		{/each}
	</nav>

	{#if loading}
		<div class="loading-state">
			<span class="spinner large"></span>
			<p>Loading settings...</p>
		</div>
	{:else}
		<div class="tab-content">
			<!-- General Settings -->
			{#if activeTab === 'general'}
				<section class="settings-section">
					<h2>General Settings</h2>

					<div class="setting-group">
						<label class="setting-row toggle-row">
							<div class="setting-info">
								<span class="setting-label">Enable Consent System</span>
								<span class="setting-description"
									>Enable or disable the entire consent management system</span
								>
							</div>
							<div class="toggle">
								<input id="page-settings-consent-enabled" name="page-settings-consent-enabled" type="checkbox" bind:checked={settings.consent_enabled} />
								<span class="toggle-slider"></span>
							</div>
						</label>

						<label class="setting-row toggle-row">
							<div class="setting-info">
								<span class="setting-label">Test Mode</span>
								<span class="setting-description"
									>Show the consent banner to logged-in admins only</span
								>
							</div>
							<div class="toggle">
								<input id="page-settings-test-mode" name="page-settings-test-mode" type="checkbox" bind:checked={settings.test_mode} />
								<span class="toggle-slider"></span>
							</div>
						</label>
					</div>

					<div class="setting-group">
						<div class="setting-row">
							<div class="setting-info">
								<span class="setting-label">Consent Expiry (Days)</span>
								<span class="setting-description"
									>How long before consent expires and banner is shown again</span
								>
							</div>
							<input
								id="page-settings-expire-days" name="page-settings-expire-days" type="number"
								class="input-number"
								bind:value={settings.expire_days}
								min="1"
								max="730"
							/>
						</div>

						<div class="setting-row">
							<div class="setting-info">
								<span class="setting-label">Consent Version</span>
								<span class="setting-description"
									>Increment to invalidate existing consents when policy changes</span
								>
							</div>
							<div class="version-input">
								<input
									id="page-settings-consent-version" name="page-settings-consent-version" type="number"
									class="input-number"
									bind:value={settings.consent_version}
									min="1"
									readonly
								/>
								<button class="btn btn-sm" onclick={incrementVersion}> Increment Version </button>
							</div>
						</div>

						<div class="setting-row">
							<div class="setting-info">
								<span class="setting-label">Policy Version</span>
								<span class="setting-description"
									>Version string for your privacy policy (e.g., 1.0.0)</span
								>
							</div>
							<input
								id="page-settings-policy-version" name="page-settings-policy-version" type="text"
								class="input-text"
								bind:value={settings.policy_version}
								placeholder="1.0.0"
							/>
						</div>
					</div>
				</section>
			{/if}

			<!-- Banner Settings -->
			{#if activeTab === 'banner'}
				<section class="settings-section">
					<h2>Banner Configuration</h2>

					<div class="setting-group">
						<label class="setting-row toggle-row">
							<div class="setting-info">
								<span class="setting-label">Enable Banner</span>
								<span class="setting-description">Show the consent banner to visitors</span>
							</div>
							<div class="toggle">
								<input id="page-settings-banner-enabled" name="page-settings-banner-enabled" type="checkbox" bind:checked={settings.banner_enabled} />
								<span class="toggle-slider"></span>
							</div>
						</label>
					</div>

					<div class="setting-group">
						<div class="setting-row">
							<div class="setting-info">
								<span class="setting-label">Banner Position</span>
								<span class="setting-description">Where to display the consent banner</span>
							</div>
							<select class="select" bind:value={settings.banner_position}>
								<option value="bottom">Bottom</option>
								<option value="top">Top</option>
								<option value="center">Center (Modal)</option>
							</select>
						</div>

						<div class="setting-row">
							<div class="setting-info">
								<span class="setting-label">Banner Layout</span>
								<span class="setting-description">Choose the banner display style</span>
							</div>
							<select class="select" bind:value={settings.banner_layout}>
								<option value="bar">Full-width Bar</option>
								<option value="popup">Popup Modal</option>
								<option value="floating">Floating Card</option>
								<option value="drawer">Side Drawer</option>
							</select>
						</div>
					</div>

					<div class="setting-group">
						<h3>Button Options</h3>

						<label class="setting-row toggle-row">
							<div class="setting-info">
								<span class="setting-label">Show Reject Button</span>
								<span class="setting-description"
									>Display a "Reject All" button (required for GDPR compliance)</span
								>
							</div>
							<div class="toggle">
								<input id="page-settings-show-reject-button" name="page-settings-show-reject-button" type="checkbox" bind:checked={settings.show_reject_button} />
								<span class="toggle-slider"></span>
							</div>
						</label>

						<label class="setting-row toggle-row">
							<div class="setting-info">
								<span class="setting-label">Show Settings Button</span>
								<span class="setting-description">Display a "Manage Preferences" button</span>
							</div>
							<div class="toggle">
								<input id="page-settings-show-settings-button" name="page-settings-show-settings-button" type="checkbox" bind:checked={settings.show_settings_button} />
								<span class="toggle-slider"></span>
							</div>
						</label>
					</div>

					<div class="setting-group">
						<h3>Behavior</h3>

						<label class="setting-row toggle-row">
							<div class="setting-info">
								<span class="setting-label">Close on Scroll</span>
								<span class="setting-description"
									>Automatically accept all cookies when user scrolls (not GDPR compliant)</span
								>
							</div>
							<div class="toggle">
								<input id="page-settings-close-on-scroll" name="page-settings-close-on-scroll" type="checkbox" bind:checked={settings.close_on_scroll} />
								<span class="toggle-slider"></span>
							</div>
						</label>

						{#if settings.close_on_scroll}
							<div class="setting-row">
								<div class="setting-info">
									<span class="setting-label">Scroll Distance (px)</span>
									<span class="setting-description">Pixels to scroll before accepting</span>
								</div>
								<input
									id="page-settings-close-on-scroll-distance" name="page-settings-close-on-scroll-distance" type="number"
									class="input-number"
									bind:value={settings.close_on_scroll_distance}
									min="10"
									max="500"
								/>
							</div>
						{/if}
					</div>

					<div class="banner-preview-link">
						<a href="/admin/consent/templates" class="btn btn-outline">
							Customize Banner Design →
						</a>
					</div>
				</section>
			{/if}

			<!-- Script Blocking -->
			{#if activeTab === 'scripts'}
				<section class="settings-section">
					<h2>Script Blocking</h2>
					<p class="section-description">
						Configure which third-party scripts should be blocked until consent is given. Scripts
						are categorized by their purpose.
					</p>

					<div class="setting-group">
						<label class="setting-row toggle-row highlight">
							<div class="setting-info">
								<span class="setting-label">Enable Script Blocking</span>
								<span class="setting-description">Block scripts based on consent categories</span>
							</div>
							<div class="toggle">
								<input id="page-settings-script-blocking-enabled" name="page-settings-script-blocking-enabled" type="checkbox" bind:checked={settings.script_blocking_enabled} />
								<span class="toggle-slider"></span>
							</div>
						</label>
					</div>

					{#if settings.script_blocking_enabled}
						<div class="script-categories">
							<div class="script-category">
								<h3>Analytics Scripts</h3>
								{#each scriptBlockingOptions.filter((s) => s.category === 'analytics') as script}
									<label class="setting-row toggle-row compact">
										<div class="setting-info">
											<span class="setting-label">{script.label}</span>
											<span class="setting-description">{script.description}</span>
										</div>
										<div class="toggle small">
											<input id="page-settings-script-key" name="page-settings-script-key" type="checkbox" bind:checked={settings[script.key]} />
											<span class="toggle-slider"></span>
										</div>
									</label>
								{/each}
							</div>

							<div class="script-category">
								<h3>Marketing Scripts</h3>
								{#each scriptBlockingOptions.filter((s) => s.category === 'marketing') as script}
									<label class="setting-row toggle-row compact">
										<div class="setting-info">
											<span class="setting-label">{script.label}</span>
											<span class="setting-description">{script.description}</span>
										</div>
										<div class="toggle small">
											<input id="page-settings-script-key" name="page-settings-script-key" type="checkbox" bind:checked={settings[script.key]} />
											<span class="toggle-slider"></span>
										</div>
									</label>
								{/each}
							</div>

							<div class="script-category">
								<h3>Embedded Content</h3>
								{#each scriptBlockingOptions.filter((s) => s.category === 'embedded_video') as script}
									<label class="setting-row toggle-row compact">
										<div class="setting-info">
											<span class="setting-label">{script.label}</span>
											<span class="setting-description">{script.description}</span>
										</div>
										<div class="toggle small">
											<input id="page-settings-script-key" name="page-settings-script-key" type="checkbox" bind:checked={settings[script.key]} />
											<span class="toggle-slider"></span>
										</div>
									</label>
								{/each}
							</div>
						</div>
					{/if}
				</section>
			{/if}

			<!-- Integrations -->
			{#if activeTab === 'integrations'}
				<section class="settings-section">
					<h2>Platform Integrations</h2>
					<p class="section-description">
						Configure consent mode integrations with advertising platforms.
					</p>

					<div class="integration-cards">
						<div class="integration-card">
							<div class="integration-header">
								<div class="integration-icon google"></div>
								<div class="integration-info">
									<h3>Google Consent Mode v2</h3>
									<p>Enable Google's consent mode for GA4, Google Ads, and Tag Manager</p>
								</div>
								<div class="toggle">
									<input id="page-settings-google-consent-mode-enabled" name="page-settings-google-consent-mode-enabled" type="checkbox" bind:checked={settings.google_consent_mode_enabled} />
									<span class="toggle-slider"></span>
								</div>
							</div>
							{#if settings.google_consent_mode_enabled}
								<div class="integration-details">
									<p>This will configure:</p>
									<ul>
										<li><code>ad_storage</code> - Advertising cookies</li>
										<li><code>analytics_storage</code> - Analytics cookies</li>
										<li><code>ad_user_data</code> - User data for ads</li>
										<li><code>ad_personalization</code> - Personalized ads</li>
									</ul>
								</div>
							{/if}
						</div>

						<div class="integration-card">
							<div class="integration-header">
								<div class="integration-icon microsoft"></div>
								<div class="integration-info">
									<h3>Microsoft/Bing Consent Mode</h3>
									<p>Enable Microsoft's consent mode for Bing Ads and Clarity</p>
								</div>
								<div class="toggle">
									<input id="page-settings-bing-consent-mode-enabled" name="page-settings-bing-consent-mode-enabled" type="checkbox" bind:checked={settings.bing_consent_mode_enabled} />
									<span class="toggle-slider"></span>
								</div>
							</div>
							{#if settings.bing_consent_mode_enabled}
								<div class="integration-details">
									<p>This will configure Microsoft UET tag consent signals.</p>
								</div>
							{/if}
						</div>
					</div>
				</section>
			{/if}

			<!-- Geolocation -->
			{#if activeTab === 'geolocation'}
				<section class="settings-section">
					<h2>Geolocation Rules</h2>
					<p class="section-description">
						Configure different consent behaviors based on visitor location. Requires a MaxMind
						GeoIP2 license for production use.
					</p>

					<div class="setting-group">
						<label class="setting-row toggle-row">
							<div class="setting-info">
								<span class="setting-label">Enable Geolocation</span>
								<span class="setting-description"
									>Apply different rules based on visitor country</span
								>
							</div>
							<div class="toggle">
								<input id="page-settings-geolocation-enabled" name="page-settings-geolocation-enabled" type="checkbox" bind:checked={settings.geolocation_enabled} />
								<span class="toggle-slider"></span>
							</div>
						</label>

						<label class="setting-row toggle-row">
							<div class="setting-info">
								<span class="setting-label">Default to Strict Mode</span>
								<span class="setting-description">Require explicit consent for unknown regions</span
								>
							</div>
							<div class="toggle">
								<input id="page-settings-geo-default-strict" name="page-settings-geo-default-strict" type="checkbox" bind:checked={settings.geo_default_strict} />
								<span class="toggle-slider"></span>
							</div>
						</label>
					</div>

					{#if settings.geolocation_enabled}
						<div class="geo-rules">
							<h3>Region Rules</h3>
							<div class="geo-rule-list">
								<div class="geo-rule">
									<div class="geo-region">
										<span class="flag">🇪🇺</span>
										<span>European Union (GDPR)</span>
									</div>
									<div class="geo-config">
										<span class="badge strict">Strict Consent Required</span>
									</div>
								</div>
								<div class="geo-rule">
									<div class="geo-region">
										<span class="flag">🇬🇧</span>
										<span>United Kingdom (UK GDPR)</span>
									</div>
									<div class="geo-config">
										<span class="badge strict">Strict Consent Required</span>
									</div>
								</div>
								<div class="geo-rule">
									<div class="geo-region">
										<span class="flag">🇺🇸</span>
										<span>California (CCPA)</span>
									</div>
									<div class="geo-config">
										<span class="badge notice">Notice Required</span>
									</div>
								</div>
								<div class="geo-rule">
									<div class="geo-region">
										<span class="flag">🇧🇷</span>
										<span>Brazil (LGPD)</span>
									</div>
									<div class="geo-config">
										<span class="badge strict">Strict Consent Required</span>
									</div>
								</div>
								<div class="geo-rule">
									<div class="geo-region">
										<span class="flag">🌍</span>
										<span>Rest of World</span>
									</div>
									<div class="geo-config">
										<span class="badge notice">Notice Only</span>
									</div>
								</div>
							</div>
						</div>
					{/if}
				</section>
			{/if}

			<!-- Proof of Consent -->
			{#if activeTab === 'proof'}
				<section class="settings-section">
					<h2>Proof of Consent</h2>
					<p class="section-description">
						Store and manage consent records for compliance auditing.
					</p>

					<div class="setting-group">
						<label class="setting-row toggle-row">
							<div class="setting-info">
								<span class="setting-label">Enable Proof Storage</span>
								<span class="setting-description">Store consent records in the database</span>
							</div>
							<div class="toggle">
								<input id="page-settings-proof-consent-enabled" name="page-settings-proof-consent-enabled" type="checkbox" bind:checked={settings.proof_consent_enabled} />
								<span class="toggle-slider"></span>
							</div>
						</label>
					</div>

					{#if settings.proof_consent_enabled}
						<div class="setting-group">
							<div class="setting-row">
								<div class="setting-info">
									<span class="setting-label">Retention Period (Days)</span>
									<span class="setting-description">How long to keep consent records</span>
								</div>
								<input
									id="page-settings-proof-retention-days" name="page-settings-proof-retention-days" type="number"
									class="input-number"
									bind:value={settings.proof_retention_days}
									min="30"
									max="3650"
								/>
							</div>

							<label class="setting-row toggle-row">
								<div class="setting-info">
									<span class="setting-label">Auto-Delete Expired Records</span>
									<span class="setting-description"
										>Automatically delete records older than retention period</span
									>
								</div>
								<div class="toggle">
									<input id="page-settings-proof-auto-delete" name="page-settings-proof-auto-delete" type="checkbox" bind:checked={settings.proof_auto_delete} />
									<span class="toggle-slider"></span>
								</div>
							</label>
						</div>

						<div class="proof-info">
							<h3>What's Stored</h3>
							<ul>
								<li>Unique consent ID</li>
								<li>User identifier (cookie ID, email, or user ID)</li>
								<li>Consent state (categories granted)</li>
								<li>Timestamp of consent</li>
								<li>IP address (hashed)</li>
								<li>User agent</li>
								<li>Consent method (banner, modal, API)</li>
								<li>Policy version at time of consent</li>
							</ul>
						</div>
					{/if}
				</section>
			{/if}
		</div>
	{/if}

	<!-- Notification -->
	{#if notification}
		<div class="notification" class:error={notificationType === 'error'}>
			{#if notificationType === 'success'}
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="18"
					height="18"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<path d="m9 12 2 2 4-4" />
					<circle cx="12" cy="12" r="10" />
				</svg>
			{:else}
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="18"
					height="18"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<circle cx="12" cy="12" r="10" />
					<line x1="15" y1="9" x2="9" y2="15" />
					<line x1="9" y1="9" x2="15" y2="15" />
				</svg>
			{/if}
			{notification}
		</div>
	{/if}
</div>

<style>
	.settings-page {
		background: #0a101c;
		color: #e2e8f0;
		padding: 2rem;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
		flex-wrap: wrap;
		gap: 1rem;
	}

	.page-header h1 {
		margin: 0;
		font-size: 1.75rem;
		font-weight: 700;
	}

	.page-header p {
		margin: 0.25rem 0 0;
		color: #64748b;
	}

	.header-actions {
		display: flex;
		gap: 0.5rem;
	}

	.btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		border-radius: 8px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
	}

	.btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-primary {
		background: linear-gradient(135deg, #0ea5e9, #06b6d4);
		color: white;
	}

	.btn-secondary {
		background: rgba(255, 255, 255, 0.1);
		color: white;
		border: 1px solid rgba(255, 255, 255, 0.2);
	}

	.btn-outline {
		background: transparent;
		color: #0ea5e9;
		border: 1px solid #0ea5e9;
	}

	.btn-sm {
		padding: 0.375rem 0.75rem;
		font-size: 0.8rem;
	}

	.btn:hover:not(:disabled) {
		filter: brightness(1.1);
		transform: translateY(-1px);
	}

	.tabs {
		display: flex;
		gap: 0.25rem;
		margin-bottom: 2rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
		padding-bottom: 0;
		overflow-x: auto;
	}

	.tab {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: transparent;
		border: none;
		color: #64748b;
		cursor: pointer;
		font-size: 0.875rem;
		font-weight: 500;
		border-bottom: 2px solid transparent;
		margin-bottom: -1px;
		transition: all 0.2s;
		white-space: nowrap;
	}

	.tab:hover {
		color: #e2e8f0;
	}

	.tab.active {
		color: #0ea5e9;
		border-bottom-color: #0ea5e9;
	}

	.tab-icon {
		font-size: 1rem;
	}

	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem;
		color: #64748b;
	}

	.spinner {
		width: 20px;
		height: 20px;
		border: 2px solid rgba(255, 255, 255, 0.2);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	.spinner.large {
		width: 40px;
		height: 40px;
		border-width: 3px;
		margin-bottom: 1rem;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.tab-content {
		animation: fadeIn 0.2s ease;
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	.settings-section {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.08);
		border-radius: 12px;
		padding: 1.5rem;
		margin-bottom: 1.5rem;
	}

	.settings-section h2 {
		font-size: 1.25rem;
		font-weight: 600;
		margin: 0 0 0.5rem;
		color: #f1f5f9;
	}

	.section-description {
		color: #64748b;
		font-size: 0.875rem;
		margin: 0 0 1.5rem;
	}

	.setting-group {
		margin-bottom: 1.5rem;
	}

	.setting-group h3 {
		font-size: 0.9rem;
		font-weight: 600;
		color: #94a3b8;
		margin: 1.5rem 0 0.75rem;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.setting-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem;
		background: rgba(255, 255, 255, 0.02);
		border: 1px solid rgba(255, 255, 255, 0.06);
		border-radius: 8px;
		margin-bottom: 0.5rem;
		gap: 1rem;
	}

	.setting-row.compact {
		padding: 0.75rem 1rem;
	}

	.setting-row.highlight {
		background: rgba(14, 165, 233, 0.1);
		border-color: rgba(14, 165, 233, 0.2);
	}

	.setting-info {
		flex: 1;
	}

	.setting-label {
		display: block;
		font-weight: 500;
		color: #f1f5f9;
		margin-bottom: 0.25rem;
	}

	.setting-description {
		display: block;
		font-size: 0.8rem;
		color: #64748b;
	}

	.toggle {
		position: relative;
		width: 48px;
		height: 26px;
		flex-shrink: 0;
	}

	.toggle.small {
		width: 40px;
		height: 22px;
	}

	.toggle input {
		opacity: 0;
		width: 0;
		height: 0;
	}

	.toggle-slider {
		position: absolute;
		cursor: pointer;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: #374151;
		transition: 0.3s;
		border-radius: 26px;
	}

	.toggle-slider::before {
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

	.toggle.small .toggle-slider::before {
		height: 16px;
		width: 16px;
	}

	.toggle input:checked + .toggle-slider {
		background: #0ea5e9;
	}

	.toggle input:checked + .toggle-slider::before {
		transform: translateX(22px);
	}

	.toggle.small input:checked + .toggle-slider::before {
		transform: translateX(18px);
	}

	.input-number,
	.input-text,
	.select {
		padding: 0.5rem 0.75rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.15);
		border-radius: 6px;
		color: #f1f5f9;
		font-size: 0.875rem;
	}

	.input-number {
		width: 100px;
		text-align: center;
	}

	.input-text {
		width: 200px;
	}

	.select {
		width: 200px;
	}

	.version-input {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.banner-preview-link {
		margin-top: 1.5rem;
		padding-top: 1.5rem;
		border-top: 1px solid rgba(255, 255, 255, 0.1);
	}

	.script-categories {
		display: grid;
		gap: 1.5rem;
	}

	.script-category h3 {
		margin-top: 0;
	}

	.integration-cards {
		display: grid;
		gap: 1rem;
	}

	.integration-card {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		overflow: hidden;
	}

	.integration-header {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem;
	}

	.integration-icon {
		width: 48px;
		height: 48px;
		border-radius: 10px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.integration-icon.google {
		background: linear-gradient(135deg, #4285f4, #34a853, #fbbc04, #ea4335);
	}

	.integration-icon.microsoft {
		background: linear-gradient(135deg, #f25022, #7fba00, #00a4ef, #ffb900);
	}

	.integration-info {
		flex: 1;
	}

	.integration-info h3 {
		margin: 0;
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
	}

	.integration-info p {
		margin: 0.25rem 0 0;
		font-size: 0.8rem;
		color: #64748b;
	}

	.integration-details {
		padding: 1rem 1.25rem;
		background: rgba(0, 0, 0, 0.2);
		border-top: 1px solid rgba(255, 255, 255, 0.05);
	}

	.integration-details p {
		margin: 0 0 0.5rem;
		font-size: 0.8rem;
		color: #94a3b8;
	}

	.integration-details ul {
		margin: 0;
		padding-left: 1.25rem;
	}

	.integration-details li {
		font-size: 0.8rem;
		color: #64748b;
		margin-bottom: 0.25rem;
	}

	.integration-details code {
		background: rgba(255, 255, 255, 0.1);
		padding: 0.1rem 0.4rem;
		border-radius: 4px;
		font-size: 0.75rem;
	}

	.geo-rules {
		margin-top: 1.5rem;
	}

	.geo-rules h3 {
		margin: 0 0 1rem;
		font-size: 1rem;
		font-weight: 600;
	}

	.geo-rule-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.geo-rule {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.875rem 1rem;
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.06);
		border-radius: 8px;
	}

	.geo-region {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.flag {
		font-size: 1.25rem;
	}

	.badge {
		padding: 0.25rem 0.75rem;
		border-radius: 20px;
		font-size: 0.75rem;
		font-weight: 500;
	}

	.badge.strict {
		background: rgba(239, 68, 68, 0.2);
		color: #f87171;
	}

	.badge.notice {
		background: rgba(251, 191, 36, 0.2);
		color: #fbbf24;
	}

	.proof-info {
		margin-top: 1.5rem;
		padding: 1rem;
		background: rgba(255, 255, 255, 0.03);
		border-radius: 8px;
	}

	.proof-info h3 {
		margin: 0 0 0.75rem;
		font-size: 0.9rem;
		font-weight: 600;
		color: #f1f5f9;
	}

	.proof-info ul {
		margin: 0;
		padding-left: 1.25rem;
	}

	.proof-info li {
		font-size: 0.8rem;
		color: #64748b;
		margin-bottom: 0.25rem;
	}

	.notification {
		position: fixed;
		bottom: 2rem;
		right: 2rem;
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.875rem 1.25rem;
		background: #22c55e;
		color: white;
		border-radius: 10px;
		font-size: 0.875rem;
		font-weight: 500;
		box-shadow: 0 10px 30px rgba(34, 197, 94, 0.3);
		animation: slideIn 0.3s ease;
		z-index: 100001;
	}

	.notification.error {
		background: #ef4444;
		box-shadow: 0 10px 30px rgba(239, 68, 68, 0.3);
	}

	@keyframes slideIn {
		from {
			opacity: 0;
			transform: translateY(10px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	@media (max-width: 768px) {
		.settings-page {
			padding: 1rem;
		}

		.page-header {
			flex-direction: column;
		}

		.header-actions {
			width: 100%;
		}

		.setting-row {
			flex-direction: column;
			align-items: flex-start;
		}

		.setting-row.toggle-row {
			flex-direction: row;
			align-items: center;
		}

		.input-number,
		.input-text,
		.select {
			width: 100%;
		}
	}
</style>

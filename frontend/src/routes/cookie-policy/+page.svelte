<!--
	URL: /cookie-policy
-->

<script lang="ts">
	/**
	 * Cookie Policy Page
	 *
	 * Auto-generated GDPR-compliant cookie declaration page
	 * that lists all cookies used on the site with full transparency.
	 */
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { scanCookies, openPreferencesModal, consentStore, getVendorInfo } from '$lib/consent';
	import { t, currentLanguage, getSupportedLanguages, setLanguage } from '$lib/consent/i18n';
	import {
		generateConsentReceipt,
		downloadReceiptAsJSON,
		printReceipt
	} from '$lib/consent/consent-receipt';
	import { getVersionInfo } from '$lib/consent/versioning';
	import type { CookieScanResult } from '$lib/consent';
	import type { SupportedLanguage } from '$lib/consent/i18n';

	let cookieScan: CookieScanResult | null = null;
	let vendorList: ReturnType<typeof getVendorInfo> = [];
	let versionInfo = getVersionInfo();

	const languageNames: Record<SupportedLanguage, string> = {
		en: 'English',
		de: 'Deutsch',
		fr: 'Français',
		es: 'Español',
		it: 'Italiano',
		nl: 'Nederlands',
		pt: 'Português',
		pl: 'Polski',
		sv: 'Svenska',
		da: 'Dansk',
		fi: 'Suomi',
		no: 'Norsk',
		cs: 'Čeština',
		ro: 'Română',
		hu: 'Magyar',
		el: 'Ελληνικά',
		bg: 'Български',
		hr: 'Hrvatski',
		sk: 'Slovenčina',
		sl: 'Slovenščina',
		et: 'Eesti',
		lv: 'Latviešu',
		lt: 'Lietuvių',
		ja: '日本語',
		zh: '中文',
		ko: '한국어'
	};

	onMount(() => {
		if (browser) {
			// Scan current cookies
			cookieScan = scanCookies();

			// Get vendor info
			vendorList = getVendorInfo();
		}
	});

	function handleManageCookies() {
		openPreferencesModal();
	}

	function handleDownloadReceipt() {
		const receipt = generateConsentReceipt($consentStore);
		downloadReceiptAsJSON(receipt);
	}

	function handlePrintReceipt() {
		const receipt = generateConsentReceipt($consentStore);
		printReceipt(receipt);
	}

	function handleLanguageChange(event: Event) {
		const select = event.target as HTMLSelectElement;
		setLanguage(select.value as SupportedLanguage);
	}

	const categoryColors: Record<string, string> = {
		necessary: 'bg-green-500/20 text-green-400 border-green-500/30',
		analytics: 'bg-blue-500/20 text-blue-400 border-blue-500/30',
		marketing: 'bg-purple-500/20 text-purple-400 border-purple-500/30',
		preferences: 'bg-amber-500/20 text-amber-400 border-amber-500/30',
		unknown: 'bg-gray-500/20 text-gray-400 border-gray-500/30'
	};
</script>

<svelte:head>
	<title>{$t.cookiePolicyTitle} | Revolution Trading Pros</title>
	<meta name="description" content={$t.cookiePolicyIntro} />
</svelte:head>

<div class="cookie-policy">
	<div class="container">
		<!-- Header -->
		<header class="header">
			<div class="header-top">
				<h1>{$t.cookiePolicyTitle}</h1>
				<select class="language-select" value={$currentLanguage} onchange={handleLanguageChange}>
					{#each getSupportedLanguages().slice(0, 7) as lang}
						<option value={lang}>{languageNames[lang]}</option>
					{/each}
				</select>
			</div>
			<p class="subtitle">{$t.cookiePolicyIntro}</p>
			<p class="last-updated">
				{$t.lastUpdated}: {versionInfo.formattedDate} (v{versionInfo.version})
			</p>
		</header>

		<!-- Quick Actions -->
		<section class="quick-actions">
			<button class="btn btn-primary" onclick={handleManageCookies}>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="18"
					height="18"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<path
						d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"
					/>
					<circle cx="12" cy="12" r="3" />
				</svg>
				{$t.cookieSettings}
			</button>
			{#if $consentStore.hasInteracted}
				<button class="btn btn-secondary" onclick={handleDownloadReceipt}>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="18"
						height="18"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
						<polyline points="7 10 12 15 17 10" />
						<line x1="12" y1="15" x2="12" y2="3" />
					</svg>
					{$t.downloadReceipt}
				</button>
				<button class="btn btn-secondary" onclick={handlePrintReceipt}>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="18"
						height="18"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<polyline points="6 9 6 2 18 2 18 9" />
						<path d="M6 18H4a2 2 0 0 1-2-2v-5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-2" />
						<rect x="6" y="14" width="12" height="8" />
					</svg>
					Print Receipt
				</button>
			{/if}
		</section>

		<!-- Current Consent Status -->
		{#if $consentStore.hasInteracted}
			<section class="section">
				<h2>Your Current Consent</h2>
				<div class="consent-status">
					<div class="status-item" class:enabled={$consentStore.necessary}>
						<span class="status-label">{$t.necessary}</span>
						<span class="status-badge enabled">{$t.required}</span>
					</div>
					<div class="status-item" class:enabled={$consentStore.analytics}>
						<span class="status-label">{$t.analytics}</span>
						<span class="status-badge" class:enabled={$consentStore.analytics}>
							{$consentStore.analytics ? $t.enabled : $t.disabled}
						</span>
					</div>
					<div class="status-item" class:enabled={$consentStore.marketing}>
						<span class="status-label">{$t.marketing}</span>
						<span class="status-badge" class:enabled={$consentStore.marketing}>
							{$consentStore.marketing ? $t.enabled : $t.disabled}
						</span>
					</div>
					<div class="status-item" class:enabled={$consentStore.preferences}>
						<span class="status-label">{$t.preferences}</span>
						<span class="status-badge" class:enabled={$consentStore.preferences}>
							{$consentStore.preferences ? $t.enabled : $t.disabled}
						</span>
					</div>
				</div>
				{#if $consentStore.consentId}
					<p class="consent-id">
						{$t.consentId}: <code>{$consentStore.consentId}</code>
					</p>
				{/if}
			</section>
		{/if}

		<!-- What Are Cookies -->
		<section class="section">
			<h2>{$t.whatAreCookies}</h2>
			<p>{$t.whatAreCookiesDescription}</p>
		</section>

		<!-- How We Use Cookies -->
		<section class="section">
			<h2>{$t.howWeUseCookies}</h2>
			<div class="category-grid">
				<div class="category-card necessary">
					<div class="category-header">
						<span class="category-icon">🔒</span>
						<h3>{$t.necessary}</h3>
						<span class="badge required">{$t.required}</span>
					</div>
					<p>{$t.necessaryDescription}</p>
				</div>
				<div class="category-card analytics">
					<div class="category-header">
						<span class="category-icon">📊</span>
						<h3>{$t.analytics}</h3>
						<span class="badge optional">{$t.optional}</span>
					</div>
					<p>{$t.analyticsDescription}</p>
				</div>
				<div class="category-card marketing">
					<div class="category-header">
						<span class="category-icon">📢</span>
						<h3>{$t.marketing}</h3>
						<span class="badge optional">{$t.optional}</span>
					</div>
					<p>{$t.marketingDescription}</p>
				</div>
				<div class="category-card preferences">
					<div class="category-header">
						<span class="category-icon">⚙️</span>
						<h3>{$t.preferences}</h3>
						<span class="badge optional">{$t.optional}</span>
					</div>
					<p>{$t.preferencesDescription}</p>
				</div>
			</div>
		</section>

		<!-- Cookie Declaration Table -->
		<section class="section">
			<h2>{$t.cookieDeclaration}</h2>

			{#if cookieScan}
				<div class="cookie-summary">
					<div class="summary-stat">
						<span class="stat-value">{cookieScan.totalCookies}</span>
						<span class="stat-label">Total Cookies</span>
					</div>
					<div class="summary-stat">
						<span class="stat-value">{cookieScan.categorizedCookies}</span>
						<span class="stat-label">Categorized</span>
					</div>
					<div class="summary-stat">
						<span class="stat-value">{cookieScan.uncategorizedCookies}</span>
						<span class="stat-label">Unknown</span>
					</div>
				</div>

				{#each Object.entries(cookieScan.byCategory) as [category, cookies]}
					{#if cookies.length > 0}
						<div class="cookie-category-section">
							<h3 class="category-title">
								<span
									class="category-badge {categoryColors[category] || categoryColors['unknown']}"
								>
									{category}
								</span>
								({cookies.length} cookies)
							</h3>
							<div class="cookie-table-wrapper">
								<table class="cookie-table">
									<thead>
										<tr>
											<th>{$t.cookieName}</th>
											<th>{$t.cookiePurpose}</th>
											<th>{$t.cookieDuration}</th>
											<th>{$t.cookieType}</th>
										</tr>
									</thead>
									<tbody>
										{#each cookies as cookie}
											<tr>
												<td class="cookie-name"><code>{cookie.name}</code></td>
												<td>{cookie.purpose || 'Not specified'}</td>
												<td>{cookie.duration || 'Session'}</td>
												<td>
													<span
														class="type-badge"
														class:first-party={cookie.type === 'first-party'}
													>
														{cookie.type || 'Unknown'}
													</span>
												</td>
											</tr>
										{/each}
									</tbody>
								</table>
							</div>
						</div>
					{/if}
				{/each}
			{:else}
				<p class="loading">Loading cookie information...</p>
			{/if}
		</section>

		<!-- Third-Party Vendors -->
		{#if vendorList.length > 0}
			<section class="section">
				<h2>Third-Party Services</h2>
				<div class="vendor-grid">
					{#each vendorList as vendor}
						<div class="vendor-card">
							<h3>{vendor.name}</h3>
							<p>{vendor.description}</p>
							<div class="vendor-meta">
								<span class="vendor-categories">
									Categories: {vendor.requiredCategories.join(', ')}
								</span>
								{#if vendor.dataLocations}
									<span class="vendor-locations">
										Data: {vendor.dataLocations.join(', ')}
									</span>
								{/if}
							</div>
							{#if vendor.privacyPolicyUrl}
								<a
									href={vendor.privacyPolicyUrl}
									target="_blank"
									rel="noopener noreferrer"
									class="vendor-link"
								>
									Privacy Policy →
								</a>
							{/if}
						</div>
					{/each}
				</div>
			</section>
		{/if}

		<!-- Privacy Signals -->
		{#if $consentStore.privacySignals}
			<section class="section">
				<h2>Privacy Signals Detected</h2>
				<div class="privacy-signals">
					{#if $consentStore.privacySignals.gpc}
						<div class="signal detected">
							<span class="signal-icon">🛡️</span>
							<span>{$t.gpcDetected}</span>
						</div>
					{/if}
					{#if $consentStore.privacySignals.dnt}
						<div class="signal detected">
							<span class="signal-icon">🚫</span>
							<span>{$t.dntDetected}</span>
						</div>
					{/if}
					{#if $consentStore.privacySignals.region}
						<div class="signal">
							<span class="signal-icon">🌍</span>
							<span>{$t.regionDetected}: {$consentStore.privacySignals.region}</span>
						</div>
					{/if}
				</div>
			</section>
		{/if}

		<!-- Footer -->
		<footer class="policy-footer">
			<p>
				For questions about our cookie policy, please contact us at
				<a href="mailto:privacy@revolutiontradingpros.com">privacy@revolutiontradingpros.com</a>
			</p>
		</footer>
	</div>
</div>

<style>
	.cookie-policy {
		min-height: 100vh;
		background: linear-gradient(to bottom, #0a101c, #0f172a);
		color: #e2e8f0;
		padding: 2rem 1rem 4rem;
	}

	.container {
		max-width: 1000px;
		margin: 0 auto;
	}

	.header {
		text-align: center;
		margin-bottom: 3rem;
		padding-bottom: 2rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	}

	.header-top {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 1rem;
		flex-wrap: wrap;
		margin-bottom: 1rem;
	}

	.header h1 {
		font-size: 2.5rem;
		font-weight: 700;
		background: linear-gradient(135deg, #0ea5e9, #06b6d4);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
		margin: 0;
	}

	.language-select {
		padding: 0.5rem 1rem;
		border-radius: 8px;
		background: rgba(255, 255, 255, 0.1);
		border: 1px solid rgba(255, 255, 255, 0.2);
		color: white;
		cursor: pointer;
	}

	.subtitle {
		font-size: 1.1rem;
		color: #94a3b8;
		max-width: 600px;
		margin: 0 auto;
	}

	.last-updated {
		font-size: 0.875rem;
		color: #64748b;
		margin-top: 0.5rem;
	}

	.quick-actions {
		display: flex;
		gap: 1rem;
		justify-content: center;
		flex-wrap: wrap;
		margin-bottom: 3rem;
	}

	.btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: 8px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
	}

	.btn-primary {
		background: linear-gradient(135deg, #0ea5e9, #06b6d4);
		color: white;
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(14, 165, 233, 0.3);
	}

	.btn-secondary {
		background: rgba(255, 255, 255, 0.1);
		color: white;
		border: 1px solid rgba(255, 255, 255, 0.2);
	}

	.btn-secondary:hover {
		background: rgba(255, 255, 255, 0.15);
	}

	.section {
		margin-bottom: 3rem;
	}

	.section h2 {
		font-size: 1.5rem;
		font-weight: 600;
		margin-bottom: 1rem;
		color: #f1f5f9;
	}

	.section p {
		color: #94a3b8;
		line-height: 1.7;
	}

	.consent-status {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 1rem;
	}

	.status-item {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem;
		background: rgba(255, 255, 255, 0.05);
		border-radius: 8px;
		border: 1px solid rgba(255, 255, 255, 0.1);
	}

	.status-badge {
		padding: 0.25rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 600;
		background: rgba(239, 68, 68, 0.2);
		color: #f87171;
	}

	.status-badge.enabled {
		background: rgba(34, 197, 94, 0.2);
		color: #4ade80;
	}

	.consent-id {
		margin-top: 1rem;
		font-size: 0.875rem;
		color: #64748b;
	}

	.consent-id code {
		background: rgba(255, 255, 255, 0.1);
		padding: 0.25rem 0.5rem;
		border-radius: 4px;
		font-family: monospace;
	}

	.category-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
		gap: 1.5rem;
	}

	.category-card {
		padding: 1.5rem;
		background: rgba(255, 255, 255, 0.05);
		border-radius: 12px;
		border: 1px solid rgba(255, 255, 255, 0.1);
	}

	.category-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 0.75rem;
	}

	.category-icon {
		font-size: 1.5rem;
	}

	.category-header h3 {
		margin: 0;
		font-size: 1.1rem;
	}

	.badge {
		padding: 0.2rem 0.5rem;
		border-radius: 4px;
		font-size: 0.7rem;
		font-weight: 600;
		text-transform: uppercase;
	}

	.badge.required {
		background: rgba(34, 197, 94, 0.2);
		color: #4ade80;
	}

	.badge.optional {
		background: rgba(148, 163, 184, 0.2);
		color: #94a3b8;
	}

	.category-card p {
		font-size: 0.875rem;
		margin: 0;
	}

	.cookie-summary {
		display: flex;
		gap: 2rem;
		margin-bottom: 2rem;
		flex-wrap: wrap;
	}

	.summary-stat {
		text-align: center;
	}

	.stat-value {
		display: block;
		font-size: 2rem;
		font-weight: 700;
		color: #0ea5e9;
	}

	.stat-label {
		font-size: 0.875rem;
		color: #64748b;
	}

	.cookie-category-section {
		margin-bottom: 2rem;
	}

	.category-title {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 1.1rem;
		margin-bottom: 1rem;
	}

	.category-badge {
		padding: 0.25rem 0.75rem;
		border-radius: 4px;
		font-size: 0.875rem;
		text-transform: capitalize;
		border: 1px solid;
	}

	.cookie-table-wrapper {
		overflow-x: auto;
	}

	.cookie-table {
		width: 100%;
		border-collapse: collapse;
		font-size: 0.875rem;
	}

	.cookie-table th,
	.cookie-table td {
		padding: 0.75rem;
		text-align: left;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	}

	.cookie-table th {
		color: #94a3b8;
		font-weight: 600;
		font-size: 0.75rem;
		text-transform: uppercase;
	}

	.cookie-name code {
		background: rgba(255, 255, 255, 0.1);
		padding: 0.2rem 0.4rem;
		border-radius: 4px;
		font-size: 0.8rem;
	}

	.type-badge {
		padding: 0.2rem 0.5rem;
		border-radius: 4px;
		font-size: 0.75rem;
		background: rgba(148, 163, 184, 0.2);
		color: #94a3b8;
	}

	.type-badge.first-party {
		background: rgba(34, 197, 94, 0.2);
		color: #4ade80;
	}

	.vendor-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
		gap: 1.5rem;
	}

	.vendor-card {
		padding: 1.5rem;
		background: rgba(255, 255, 255, 0.05);
		border-radius: 12px;
		border: 1px solid rgba(255, 255, 255, 0.1);
	}

	.vendor-card h3 {
		margin: 0 0 0.5rem 0;
		font-size: 1.1rem;
	}

	.vendor-card p {
		font-size: 0.875rem;
		margin: 0 0 1rem 0;
	}

	.vendor-meta {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
		font-size: 0.75rem;
		color: #64748b;
		margin-bottom: 0.75rem;
	}

	.vendor-link {
		color: #0ea5e9;
		text-decoration: none;
		font-size: 0.875rem;
	}

	.vendor-link:hover {
		text-decoration: underline;
	}

	.privacy-signals {
		display: flex;
		flex-wrap: wrap;
		gap: 1rem;
	}

	.signal {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		background: rgba(255, 255, 255, 0.05);
		border-radius: 8px;
		border: 1px solid rgba(255, 255, 255, 0.1);
	}

	.signal.detected {
		background: rgba(34, 197, 94, 0.1);
		border-color: rgba(34, 197, 94, 0.3);
	}

	.signal-icon {
		font-size: 1.25rem;
	}

	.policy-footer {
		margin-top: 4rem;
		padding-top: 2rem;
		border-top: 1px solid rgba(255, 255, 255, 0.1);
		text-align: center;
		color: #64748b;
		font-size: 0.875rem;
	}

	.policy-footer a {
		color: #0ea5e9;
	}

	.loading {
		text-align: center;
		color: #64748b;
		padding: 2rem;
	}

	@media (max-width: 640px) {
		.header h1 {
			font-size: 1.75rem;
		}

		.quick-actions {
			flex-direction: column;
		}

		.btn {
			width: 100%;
			justify-content: center;
		}
	}
</style>

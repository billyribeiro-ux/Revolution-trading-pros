<script lang="ts">
	/**
	 * Cookie Policy Page
	 *
	 * Auto-generated GDPR-compliant cookie declaration page
	 * that lists all cookies used on the site with full transparency.
	 */
	import InformationPage from '#lib/components/marketing/InformationPage.svelte';
	import ConsentPreferencesModal from '#lib/consent/components/ConsentPreferencesModal.svelte';
	import { onMount } from 'svelte';

	import {
		scanCookies,
		openPreferencesModal,
		consentStore,
		getVendorInfo,
		onConsentChange
	} from '#lib/consent/index.js';

	import { t, currentLanguage, getSupportedLanguages, setLanguage } from '#lib/consent/i18n.js';
	import {
		generateConsentReceipt,
		downloadReceiptAsJSON,
		printReceipt
	} from '#lib/consent/consent-receipt.js';
	import { getVersionInfo } from '#lib/consent/versioning.js';
	import type { CookieScanResult } from '#lib/consent/index.js';
	import type { SupportedLanguage } from '#lib/consent/i18n.js';

	let cookieScan: CookieScanResult | null = $state(scanCookies());
	let vendorList: ReturnType<typeof getVendorInfo> = $state(getVendorInfo());
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
		cookieScan = scanCookies();
		vendorList = getVendorInfo();
		return onConsentChange(() => {
			cookieScan = scanCookies();
		});
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

	const categoryClasses: Record<string, string> = {
		necessary: 'category-badge--necessary',
		analytics: 'category-badge--analytics',
		marketing: 'category-badge--marketing',
		preferences: 'category-badge--preferences',
		unknown: 'category-badge--unknown'
	};
</script>

<!--
	URL: /cookie-policy
-->

<InformationPage
	title={$t.cookiePolicyTitle}
	description={$t.cookiePolicyIntro}
	metadata={`${$t.lastUpdated}: ${versionInfo.formattedDate} (v${versionInfo.version})`}
>
	{#snippet actions()}
		<label class="language-field">
			<span>Policy language</span>
			<select
				class="language-select"
				aria-label="Select policy language"
				value={$currentLanguage}
				onchange={handleLanguageChange}
			>
				{#each getSupportedLanguages().slice(0, 7) as lang (lang)}<option value={lang}
						>{languageNames[lang]}</option
					>{/each}
			</select>
		</label>
	{/snippet}
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
				></path>
				<circle cx="12" cy="12" r="3"></circle>
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
					<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
					<polyline points="7 10 12 15 17 10"></polyline>
					<line x1="12" y1="15" x2="12" y2="3"></line>
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
					<polyline points="6 9 6 2 18 2 18 9"></polyline>

					<path d="M6 18H4a2 2 0 0 1-2-2v-5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-2"
					></path>

					<rect x="6" y="14" width="12" height="8"></rect>
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
				<div class={['status-item', { enabled: $consentStore.necessary }]}>
					<span class="status-label">{$t.necessary}</span>
					<span class="status-badge enabled">{$t.required}</span>
				</div>
				<div class={['status-item', { enabled: $consentStore.analytics }]}>
					<span class="status-label">{$t.analytics}</span>
					<span class={['status-badge', { enabled: $consentStore.analytics }]}>
						{$consentStore.analytics ? $t.enabled : $t.disabled}
					</span>
				</div>
				<div class={['status-item', { enabled: $consentStore.marketing }]}>
					<span class="status-label">{$t.marketing}</span>
					<span class={['status-badge', { enabled: $consentStore.marketing }]}>
						{$consentStore.marketing ? $t.enabled : $t.disabled}
					</span>
				</div>
				<div class={['status-item', { enabled: $consentStore.preferences }]}>
					<span class="status-label">{$t.preferences}</span>
					<span class={['status-badge', { enabled: $consentStore.preferences }]}>
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

			{#each Object.entries(cookieScan.byCategory) as [category, cookies] (category)}
				{#if cookies.length > 0}
					<div class="cookie-category-section">
						<h3 class="category-title">
							<span
								class={['category-badge', categoryClasses[category] || categoryClasses.unknown]}
							>
								{category}
							</span>
							({cookies.length} cookies)
						</h3>
						<div class="cookie-table-wrapper" role="region" aria-label={`${category} cookies`}>
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
									{#each cookies as cookie (cookie.name)}
										<tr>
											<td class="cookie-name"><code>{cookie.name}</code></td>
											<td>{cookie.purpose || 'Not specified'}</td>
											<td>{cookie.duration || 'Session'}</td>
											<td>
												<span
													class={['type-badge', { 'first-party': cookie.type === 'first-party' }]}
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
				{#each vendorList as vendor (vendor.name)}
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
</InformationPage>
<ConsentPreferencesModal />

<style>
	.language-select {
		padding: 0.5rem 1rem;
		border-radius: var(--rtp-radius-md);
		background: rgba(255, 255, 255, 0.1);
		border: 1px solid rgba(255, 255, 255, 0.2);
		color: white;
		cursor: pointer;
	}

	.quick-actions {
		display: flex;
		gap: 1rem;
		justify-content: flex-start;
		flex-wrap: wrap;
		margin-bottom: 3rem;
	}

	.btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: var(--rtp-radius-md);
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
	}

	.btn-primary {
		background: var(--rtp-primary-action);
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

	.consent-status {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(min(100%, 200px), 1fr));
		gap: 1rem;
	}

	.status-item {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem;
		background: rgba(255, 255, 255, 0.05);
		border-radius: var(--rtp-radius-md);
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
		color: var(--rtp-text-muted);
	}

	.consent-id code {
		background: rgba(255, 255, 255, 0.1);
		padding: 0.25rem 0.5rem;
		border-radius: 4px;
		font-family: monospace;
	}

	.category-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(min(100%, 280px), 1fr));
		gap: 1.5rem;
	}

	.category-card {
		padding: 1.5rem;
		background: rgba(255, 255, 255, 0.05);
		border-radius: var(--rtp-radius-lg);
		border: 1px solid rgba(255, 255, 255, 0.1);
	}

	.category-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		flex-wrap: wrap;
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
		color: var(--rtp-text-muted);
	}

	.category-card p {
		font-size: inherit;
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
		color: var(--rtp-blue-bright);
	}

	.stat-label {
		font-size: 0.875rem;
		color: var(--rtp-text-muted);
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

	.category-badge--necessary {
		background: rgba(34, 197, 94, 0.2);
		color: #4ade80;
		border-color: rgba(34, 197, 94, 0.3);
	}

	.category-badge--analytics {
		background: rgba(59, 130, 246, 0.2);
		color: #60a5fa;
		border-color: rgba(59, 130, 246, 0.3);
	}

	.category-badge--marketing {
		background: rgba(168, 85, 247, 0.2);
		color: #c084fc;
		border-color: rgba(168, 85, 247, 0.3);
	}

	.category-badge--preferences {
		background: rgba(245, 158, 11, 0.2);
		color: #fbbf24;
		border-color: rgba(245, 158, 11, 0.3);
	}

	.category-badge--unknown {
		background: rgba(107, 114, 128, 0.2);
		color: #9ca3af;
		border-color: rgba(107, 114, 128, 0.3);
	}

	.cookie-table-wrapper {
		max-width: 100%;
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
		color: var(--rtp-text-muted);
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
		color: var(--rtp-text-muted);
	}

	.type-badge.first-party {
		background: rgba(34, 197, 94, 0.2);
		color: #4ade80;
	}

	.vendor-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(min(100%, 300px), 1fr));
		gap: 1.5rem;
	}

	.vendor-card {
		padding: 1.5rem;
		background: rgba(255, 255, 255, 0.05);
		border-radius: var(--rtp-radius-lg);
		border: 1px solid rgba(255, 255, 255, 0.1);
	}

	.vendor-card h3 {
		margin: 0 0 0.5rem 0;
		font-size: 1.1rem;
	}

	.vendor-card p {
		font-size: inherit;
		margin: 0 0 1rem 0;
	}

	.vendor-meta {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
		font-size: 0.75rem;
		color: var(--rtp-text-muted);
		margin-bottom: 0.75rem;
	}

	.vendor-link {
		color: var(--rtp-blue-bright);
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
		border-radius: var(--rtp-radius-md);
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
		color: var(--rtp-text-muted);
		font-size: 0.875rem;
	}

	.policy-footer a {
		color: var(--rtp-blue-bright);
	}

	.loading {
		text-align: center;
		color: var(--rtp-text-muted);
		padding: 2rem;
	}

	@media (max-width: 640px) {
		.quick-actions {
			flex-direction: column;
		}

		.btn {
			width: 100%;
			justify-content: center;
		}
	}

	.language-field {
		display: flex;
		align-items: center;
		flex-wrap: wrap;
		gap: 0.75rem;
		color: var(--rtp-text-muted);
		font-size: 1rem;
	}
	.language-select {
		font: inherit;
		max-width: 100%;
		background: var(--rtp-surface);
		color: var(--rtp-text);
		border-color: var(--rtp-border-strong);
		min-height: 2.75rem;
	}
	.btn {
		font-size: 1rem;
		min-height: 2.75rem;
	}
	.cookie-table-wrapper:focus-visible {
		outline: 2px solid var(--rtp-blue-bright);
		outline-offset: 4px;
	}
	.status-item {
		gap: 0.75rem;
		flex-wrap: wrap;
	}
	.consent-id code {
		overflow-wrap: anywhere;
	}
</style>

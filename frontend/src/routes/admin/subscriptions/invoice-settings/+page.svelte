<script lang="ts">
	import { onMount } from 'svelte';
	/* eslint svelte/no-at-html-tags: "off" -- every {@html} in this file renders sanitizer-cleaned HTML (sanitizeHtml/sanitizeBlogContent/etc.) or serialized JSON-LD; audited 2026-05-30 */
	import { fade, slide } from 'svelte/transition';
	import { browser } from '$app/environment';
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';
	// FIX-2026-04-26: Tabler icons replace raw inline <svg> blocks.
	import IconEye from '@tabler/icons-svelte-runes/icons/eye';
	import IconFileDownload from '@tabler/icons-svelte-runes/icons/file-download';
	import IconCircleCheckFilled from '@tabler/icons-svelte-runes/icons/circle-check-filled';
	import IconCircleXFilled from '@tabler/icons-svelte-runes/icons/circle-x-filled';
	import IconCheck from '@tabler/icons-svelte-runes/icons/check';
	import IconPhoto from '@tabler/icons-svelte-runes/icons/photo';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	// FIX-2026-04-26 (audit 02 §P3-8): the invoice preview HTML is generated
	// from a server template that interpolates Stripe webhook payloads and
	// admin-uploaded company info. Trust boundary is "our own backend" but
	// not zero, so route through DOMPurify (`sanitizeHtml`, profile `rich`)
	// before `{@html}`.
	import { sanitizeHtml } from '$lib/utils/sanitize';

	// API URL
	const API_BASE = '/api/admin/invoice-settings';

	// Settings State
	let settings = $state({
		company_name: '',
		company_email: '',
		company_phone: '',
		company_address: '',
		company_city: '',
		company_state: '',
		company_zip: '',
		company_country: 'US',
		tax_id: '',
		logo_path: null as string | null,
		primary_color: '#2563eb',
		secondary_color: '#1f2937',
		accent_color: '#10b981',
		font_family: 'Inter, sans-serif',
		header_text: 'INVOICE',
		footer_text: 'Thank you for your business!',
		payment_terms: 'Due upon receipt',
		notes_template: '',
		show_logo: true,
		show_tax_id: true,
		show_payment_method: true,
		show_due_date: true,
		invoice_prefix: 'INV-',
		custom_css: ''
	});

	let logoUrl = $state<string | null>(null);
	let fonts = $state<{ value: string; label: string }[]>([]);
	let countries = $state<Record<string, string>>({});

	// UI State
	let loading = $state(true);
	let saving = $state(false);
	let uploading = $state(false);
	let error = $state('');
	let success = $state('');
	let activeTab = $state<'branding' | 'company' | 'content' | 'display' | 'advanced'>('branding');
	let previewHtml = $state('');
	let showPreviewModal = $state(false);

	// Reset confirmation modal state
	let showResetModal = $state(false);

	onMount(() => {
		if (browser) {
			loadSettings();
		}
	});

	async function loadSettings() {
		loading = true;
		error = '';

		try {
			const response = await fetch(API_BASE);
			const data = await response.json();

			settings = { ...settings, ...data.settings };
			logoUrl = data.logo_url;
			fonts = data.fonts || [];
			countries = data.countries || {};
		} catch (err) {
			error = 'Failed to load invoice settings';
			console.error(err);
		} finally {
			loading = false;
		}
	}

	async function saveSettings() {
		saving = true;
		error = '';
		success = '';

		try {
			const response = await fetch(API_BASE, {
				method: 'PUT',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(settings)
			});

			if (!response.ok) throw new Error('Failed to save');

			success = 'Settings saved successfully!';
			setTimeout(() => (success = ''), 3000);
		} catch (err) {
			error = 'Failed to save settings';
			console.error(err);
		} finally {
			saving = false;
		}
	}

	async function uploadLogo(event: Event) {
		const input = event.target as HTMLInputElement;
		const file = input.files?.[0];
		if (!file) return;

		uploading = true;
		error = '';

		try {
			const formData = new FormData();
			formData.append('logo', file);

			const response = await fetch(`${API_BASE}/logo`, {
				method: 'POST',
				body: formData
			});

			if (!response.ok) throw new Error('Upload failed');

			const data = await response.json();
			logoUrl = data.logo_url;
			settings.logo_path = data.logo_path;
			success = 'Logo uploaded successfully!';
			setTimeout(() => (success = ''), 3000);
		} catch (err) {
			error = 'Failed to upload logo';
			console.error(err);
		} finally {
			uploading = false;
		}
	}

	async function removeLogo() {
		try {
			await fetch(`${API_BASE}/logo`, { method: 'DELETE' });
			logoUrl = null;
			settings.logo_path = null;
			success = 'Logo removed';
			setTimeout(() => (success = ''), 3000);
		} catch (_err) {
			error = 'Failed to remove logo';
		}
	}

	async function loadPreview() {
		try {
			const response = await fetch(`${API_BASE}/preview-html`);
			const data = await response.json();
			previewHtml = data.html;
			showPreviewModal = true;
		} catch (_err) {
			error = 'Failed to load preview';
		}
	}

	function downloadPreview() {
		window.open(`${API_BASE}/preview`, '_blank');
	}

	function resetToDefaults() {
		showResetModal = true;
	}

	async function confirmResetToDefaults() {
		showResetModal = false;

		try {
			const response = await fetch(`${API_BASE}/reset`, { method: 'POST' });
			const data = await response.json();
			settings = { ...settings, ...data.settings };
			success = 'Settings reset to defaults';
			setTimeout(() => (success = ''), 3000);
		} catch (_err) {
			error = 'Failed to reset settings';
		}
	}

	// Tabs configuration
	const tabs = [
		{ id: 'branding', label: 'Branding', icon: 'palette' },
		{ id: 'company', label: 'Company Info', icon: 'building' },
		{ id: 'content', label: 'Content', icon: 'file-text' },
		{ id: 'display', label: 'Display Options', icon: 'eye' },
		{ id: 'advanced', label: 'Advanced', icon: 'code' }
	] as const;
</script>

<svelte:head>
	<title>Invoice Settings | Admin</title>
</svelte:head>

<div class="invoice-settings-page">
	<!-- Header -->
	<div class="page-header">
		<div>
			<h1 class="page-title">Invoice Customization</h1>
			<p class="page-subtitle">Customize your invoice appearance with logo, colors, and content</p>
		</div>
		<div class="page-actions">
			<button onclick={loadPreview} class="secondary-button">
				<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: eye (preview) -->
				<IconEye size={20} aria-hidden="true" />
				Preview
			</button>
			<button onclick={downloadPreview} class="secondary-button">
				<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: file-download -->
				<IconFileDownload size={20} aria-hidden="true" />
				Download PDF
			</button>
			<button onclick={saveSettings} disabled={saving} class="primary-button">
				{#if saving}
					<svg aria-hidden="true" class="button-spinner" fill="none" viewBox="0 0 24 24">
						<circle
							class="spinner-track"
							cx="12"
							cy="12"
							r="10"
							stroke="currentColor"
							stroke-width="4"
						></circle>
						<path
							class="spinner-mark"
							fill="currentColor"
							d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
						></path>
					</svg>
					Saving...
				{:else}
					<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: check (saved) -->
					<IconCheck size={20} aria-hidden="true" />
					Save Changes
				{/if}
			</button>
		</div>
	</div>

	<!-- Alerts -->
	{#if error}
		<div transition:slide class="alert alert--error">
			<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: circle-x error -->
			<IconCircleXFilled size={20} aria-hidden="true" />
			{error}
		</div>
	{/if}

	{#if success}
		<div transition:slide class="alert alert--success">
			<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: circle-check success -->
			<IconCircleCheckFilled size={20} aria-hidden="true" />
			{success}
		</div>
	{/if}

	{#if loading}
		<div class="loading-state">
			<div class="page-spinner"></div>
		</div>
	{:else}
		<div class="settings-layout">
			<!-- Settings Panel -->
			<div class="settings-main">
				<!-- Tabs -->
				<div class="settings-card">
					<div class="tabs-shell">
						<nav class="tabs">
							{#each tabs as tab (tab.id)}
								<button
									onclick={() => (activeTab = tab.id)}
									class={['tab-button', { active: activeTab === tab.id }]}
								>
									{tab.label}
								</button>
							{/each}
						</nav>
					</div>

					<div class="settings-card-body">
						<!-- Branding Tab -->
						{#if activeTab === 'branding'}
							<div class="tab-panel" transition:fade>
								<!-- Logo Upload -->
								<div class="form-field">
									<div class="form-label form-label--spaced">Company Logo</div>
									<div class="logo-row">
										<div class="logo-dropzone">
											{#if logoUrl}
												<img
													src={logoUrl}
													alt="Logo"
													class="logo-preview"
													width="160"
													height="96"
													loading="lazy"
												/>
											{:else}
												<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: photo (logo placeholder) -->
												<IconPhoto size={40} aria-hidden="true" />
											{/if}
										</div>
										<div class="logo-actions">
											<div class="button-row">
												<label class="primary-button file-button">
													{uploading ? 'Uploading...' : 'Upload Logo'}
													<input
														type="file"
														class="visually-hidden"
														accept="image/*"
														onchange={uploadLogo}
														disabled={uploading}
													/>
												</label>
												{#if logoUrl}
													<button onclick={removeLogo} class="danger-soft-button"> Remove </button>
												{/if}
											</div>
											<p class="field-help">PNG, JPG, SVG or WebP. Max 2MB.</p>
										</div>
									</div>
								</div>

								<!-- Show Logo Toggle -->
								<label for="show-logo-toggle" class="checkbox-row">
									<input
										type="checkbox"
										id="show-logo-toggle"
										name="show-logo-toggle"
										bind:checked={settings.show_logo}
										class="checkbox-input"
									/>
									<span class="checkbox-label">Show logo on invoices</span>
								</label>

								<!-- Colors -->
								<div class="form-grid form-grid--three">
									<div class="form-field">
										<label for="primary-color" class="form-label"> Primary Color </label>
										<div class="color-row">
											<input
												type="color"
												id="primary-color"
												name="primary-color"
												bind:value={settings.primary_color}
												class="color-input"
											/>
											<input
												type="text"
												id="primary-color-text"
												name="primary-color-text"
												bind:value={settings.primary_color}
												class="form-control mono-control"
											/>
										</div>
									</div>
									<div class="form-field">
										<label for="secondary-color" class="form-label"> Secondary Color </label>
										<div class="color-row">
											<input
												type="color"
												id="secondary-color"
												name="secondary-color"
												bind:value={settings.secondary_color}
												class="color-input"
											/>
											<input
												type="text"
												id="secondary-color-text"
												name="secondary-color-text"
												bind:value={settings.secondary_color}
												class="form-control mono-control"
											/>
										</div>
									</div>
									<div class="form-field">
										<label for="accent-color" class="form-label"> Accent Color </label>
										<div class="color-row">
											<input
												type="color"
												id="accent-color"
												name="accent-color"
												bind:value={settings.accent_color}
												class="color-input"
											/>
											<input
												type="text"
												id="accent-color-text"
												name="accent-color-text"
												bind:value={settings.accent_color}
												class="form-control mono-control"
											/>
										</div>
									</div>
								</div>

								<!-- Font Family -->
								<div class="form-field">
									<label for="font-family" class="form-label"> Font Family </label>
									<select id="font-family" bind:value={settings.font_family} class="form-control">
										{#each fonts as font (font.value)}
											<option value={font.value}>{font.label}</option>
										{/each}
									</select>
								</div>
							</div>
						{/if}

						<!-- Company Info Tab -->
						{#if activeTab === 'company'}
							<div class="tab-panel" transition:fade>
								<div class="form-grid form-grid--two">
									<div class="form-field">
										<label for="company-name" class="form-label"> Company Name </label>
										<input
											type="text"
											id="company-name"
											name="company-name"
											bind:value={settings.company_name}
											class="form-control"
											placeholder="Your Company Name"
										/>
									</div>
									<div class="form-field">
										<label for="company-email" class="form-label"> Email Address </label>
										<input
											type="email"
											id="company-email"
											name="company-email"
											autocomplete="email"
											bind:value={settings.company_email}
											class="form-control"
											placeholder="billing@company.com"
										/>
									</div>
								</div>

								<div class="form-grid form-grid--two">
									<div class="form-field">
										<label for="company-phone" class="form-label"> Phone Number </label>
										<input
											type="tel"
											id="company-phone"
											name="company-phone"
											autocomplete="tel"
											bind:value={settings.company_phone}
											class="form-control"
											placeholder="+1 (555) 123-4567"
										/>
									</div>
									<div class="form-field">
										<label for="tax-id" class="form-label"> Tax ID / VAT Number </label>
										<input
											type="text"
											id="tax-id"
											name="tax-id"
											bind:value={settings.tax_id}
											class="form-control"
											placeholder="US123456789"
										/>
									</div>
								</div>

								<div class="form-field">
									<label for="company-address" class="form-label"> Street Address </label>
									<input
										type="text"
										id="company-address"
										name="company-address"
										bind:value={settings.company_address}
										class="form-control"
										placeholder="123 Business Street, Suite 100"
									/>
								</div>

								<div class="form-grid form-grid--four">
									<div class="form-field">
										<label for="company-city" class="form-label">City</label>
										<input
											type="text"
											id="company-city"
											name="company-city"
											bind:value={settings.company_city}
											class="form-control"
										/>
									</div>
									<div class="form-field">
										<label for="company-state" class="form-label">State</label>
										<input
											type="text"
											id="company-state"
											name="company-state"
											bind:value={settings.company_state}
											class="form-control"
										/>
									</div>
									<div class="form-field">
										<label for="company-zip" class="form-label">ZIP</label>
										<input
											type="text"
											id="company-zip"
											name="company-zip"
											bind:value={settings.company_zip}
											class="form-control"
										/>
									</div>
									<div class="form-field">
										<label for="company-country" class="form-label">Country</label>
										<select
											id="company-country"
											bind:value={settings.company_country}
											class="form-control"
										>
											{#each Object.entries(countries) as [code, name] (code)}
												<option value={code}>{name}</option>
											{/each}
										</select>
									</div>
								</div>
							</div>
						{/if}

						<!-- Content Tab -->
						{#if activeTab === 'content'}
							<div class="tab-panel" transition:fade>
								<div class="form-grid form-grid--two">
									<div class="form-field">
										<label for="header-text" class="form-label"> Header Text </label>
										<input
											type="text"
											id="header-text"
											name="header-text"
											bind:value={settings.header_text}
											class="form-control"
											placeholder="INVOICE"
										/>
									</div>
									<div class="form-field">
										<label for="invoice-prefix" class="form-label"> Invoice Prefix </label>
										<input
											type="text"
											id="invoice-prefix"
											name="invoice-prefix"
											bind:value={settings.invoice_prefix}
											class="form-control"
											placeholder="INV-"
										/>
									</div>
								</div>

								<div class="form-field">
									<label for="payment-terms" class="form-label"> Payment Terms </label>
									<input
										type="text"
										id="payment-terms"
										name="payment-terms"
										bind:value={settings.payment_terms}
										class="form-control"
										placeholder="Due upon receipt"
									/>
								</div>

								<div class="form-field">
									<label for="footer-text" class="form-label"> Footer Text </label>
									<input
										type="text"
										id="footer-text"
										name="footer-text"
										bind:value={settings.footer_text}
										class="form-control"
										placeholder="Thank you for your business!"
									/>
								</div>

								<div class="form-field">
									<label for="notes-template" class="form-label">
										Default Notes (shown on all invoices)
									</label>
									<textarea
										id="notes-template"
										bind:value={settings.notes_template}
										rows="4"
										class="form-control textarea-control"
										placeholder="Add any terms, conditions, or notes to include on every invoice..."
									></textarea>
								</div>
							</div>
						{/if}

						<!-- Display Options Tab -->
						{#if activeTab === 'display'}
							<div class="option-list" transition:fade>
								<p class="section-copy">Choose which elements to display on your invoices:</p>

								<label for="display-show-logo" class="option-card">
									<input
										type="checkbox"
										id="display-show-logo"
										name="display-show-logo"
										bind:checked={settings.show_logo}
										class="checkbox-input"
									/>
									<div>
										<span class="option-title">Show Company Logo</span>
										<p class="option-copy">Display your uploaded logo at the top of invoices</p>
									</div>
								</label>

								<label for="display-show-tax-id" class="option-card">
									<input
										type="checkbox"
										id="display-show-tax-id"
										name="display-show-tax-id"
										bind:checked={settings.show_tax_id}
										class="checkbox-input"
									/>
									<div>
										<span class="option-title">Show Tax ID / VAT Number</span>
										<p class="option-copy">Include your tax identification number in the header</p>
									</div>
								</label>

								<label for="display-show-payment-method" class="option-card">
									<input
										type="checkbox"
										id="display-show-payment-method"
										name="display-show-payment-method"
										bind:checked={settings.show_payment_method}
										class="checkbox-input"
									/>
									<div>
										<span class="option-title">Show Payment Method</span>
										<p class="option-copy">
											Display the card type and last 4 digits used for payment
										</p>
									</div>
								</label>

								<label for="display-show-due-date" class="option-card">
									<input
										type="checkbox"
										id="display-show-due-date"
										name="display-show-due-date"
										bind:checked={settings.show_due_date}
										class="checkbox-input"
									/>
									<div>
										<span class="option-title">Show Due Date</span>
										<p class="option-copy">Include payment due date in the invoice details</p>
									</div>
								</label>
							</div>
						{/if}

						<!-- Advanced Tab -->
						{#if activeTab === 'advanced'}
							<div class="tab-panel" transition:fade>
								<div class="form-field">
									<label for="custom-css" class="form-label"> Custom CSS </label>
									<p class="field-help field-help--spaced">
										Add custom CSS to further customize your invoice appearance. Use CSS variables
										like <code class="inline-code">var(--primary-color)</code>.
									</p>
									<textarea
										id="custom-css"
										bind:value={settings.custom_css}
										rows="10"
										class="form-control textarea-control mono-control"
										placeholder={`.invoice-title { font-size: 36px; }
.items-table th { border-radius: 0; }
.footer-text { font-style: italic; }`}
									></textarea>
								</div>

								<div class="danger-zone">
									<button onclick={resetToDefaults} class="danger-soft-button">
										Reset to Defaults
									</button>
								</div>
							</div>
						{/if}
					</div>
				</div>
			</div>

			<!-- Live Preview Panel -->
			<div class="preview-sidebar">
				<div class="preview-sticky">
					<div class="preview-card">
						<div class="preview-card-header">
							<h3 class="preview-card-title">Live Preview</h3>
						</div>
						<div class="preview-card-body">
							<!-- Mini Preview -->
							<div class="mini-invoice" style:font-family={settings.font_family}>
								<div class="mini-invoice-inner">
									<!-- Header -->
									<div class="mini-invoice-header" style:border-color={settings.primary_color}>
										<div>
											{#if settings.show_logo && logoUrl}
												<img
													src={logoUrl}
													alt="Logo"
													class="mini-logo"
													width="96"
													height="24"
													loading="lazy"
												/>
											{:else}
												<div class="mini-brand" style:color={settings.primary_color}>
													{settings.company_name || 'Company'}
												</div>
											{/if}
										</div>
										<div class="mini-invoice-title-wrap">
											<div class="mini-title" style:color={settings.secondary_color}>
												{settings.header_text || 'INVOICE'}
											</div>
											<div class="mini-muted">#{settings.invoice_prefix}001</div>
										</div>
									</div>

									<!-- Addresses -->
									<div class="mini-addresses">
										<div>
											<div class="mini-section-label" style:color={settings.primary_color}>
												FROM
											</div>
											<div class="mini-text">{settings.company_name || 'Your Company'}</div>
										</div>
										<div class="mini-right">
											<div class="mini-section-label" style:color={settings.primary_color}>
												BILL TO
											</div>
											<div class="mini-text">John Doe</div>
										</div>
									</div>

									<!-- Table -->
									<table class="mini-table">
										<thead>
											<tr style:background={settings.primary_color} style:color="white">
												<th>Description</th>
												<th class="mini-right">Amount</th>
											</tr>
										</thead>
										<tbody>
											<tr>
												<td>Professional Plan</td>
												<td class="mini-right">$99.00</td>
											</tr>
										</tbody>
									</table>

									<!-- Total -->
									<div class="mini-total-row">
										<div
											class="mini-total"
											style:background={settings.primary_color}
											style:color="white"
										>
											<span class="mini-total-text">Total: $99.00</span>
										</div>
									</div>

									<!-- Footer -->
									<div class="mini-footer">
										{settings.footer_text || 'Thank you!'}
									</div>
								</div>
							</div>

							<div class="preview-actions">
								<button onclick={loadPreview} class="primary-button compact-button">
									Full Preview
								</button>
								<button onclick={downloadPreview} class="secondary-button compact-button">
									Download PDF
								</button>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	{/if}

	<!-- Preview Modal -->
	{#if showPreviewModal}
		<div class="preview-modal-backdrop" transition:fade>
			<div class="preview-modal">
				<div class="preview-modal-header">
					<h3 class="preview-modal-title">Invoice Preview</h3>
					<button
						onclick={() => (showPreviewModal = false)}
						class="icon-button"
						aria-label="Close preview"
					>
						<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: x (close preview) -->
						<IconX size={20} aria-hidden="true" />
					</button>
				</div>
				<div class="preview-modal-body">
					<div class="preview-document">
						<!-- FIX-2026-04-26 (audit 02 §P3-8): DOMPurify sanitization. -->
						{@html sanitizeHtml(previewHtml, 'rich')}
					</div>
				</div>
			</div>
		</div>
	{/if}
</div>

<ConfirmationModal
	isOpen={showResetModal}
	title="Reset Settings"
	message="Are you sure you want to reset all settings to defaults?"
	confirmText="Reset"
	variant="warning"
	onConfirm={confirmResetToDefaults}
	onCancel={() => {
		showResetModal = false;
	}}
/>

<style>
	.invoice-settings-page {
		width: min(100% - 2rem, 72rem);
		margin-inline: auto;
		padding-block: 2rem;
		color: #111827;
	}

	:global(.dark) .invoice-settings-page {
		color: #f9fafb;
	}

	.page-header,
	.page-actions,
	.secondary-button,
	.primary-button,
	.alert,
	.logo-row,
	.button-row,
	.checkbox-row,
	.color-row,
	.option-card,
	.mini-invoice-header,
	.mini-addresses,
	.mini-total-row,
	.preview-actions,
	.preview-modal-header {
		display: flex;
		align-items: center;
	}

	.page-header {
		justify-content: space-between;
		gap: 1rem;
		flex-wrap: wrap;
		margin-bottom: 2rem;
	}

	.page-title {
		margin: 0;
		font-size: 1.875rem;
		line-height: 2.25rem;
		font-weight: 700;
	}

	.page-subtitle,
	.section-copy {
		margin: 0.25rem 0 0;
		color: #4b5563;
	}

	:global(.dark) .page-subtitle,
	:global(.dark) .section-copy {
		color: #9ca3af;
	}

	.page-actions,
	.button-row,
	.preview-actions {
		gap: 0.75rem;
	}

	.secondary-button,
	.primary-button,
	.danger-soft-button,
	.icon-button {
		border: 0;
		border-radius: 0.5rem;
		font: inherit;
		cursor: pointer;
		transition:
			background-color 0.2s ease,
			opacity 0.2s ease;
	}

	.secondary-button,
	.primary-button {
		gap: 0.5rem;
		padding: 0.5rem 1rem;
	}

	.secondary-button {
		background: #f3f4f6;
		color: #374151;
	}

	.secondary-button:hover {
		background: #e5e7eb;
	}

	:global(.dark) .secondary-button {
		background: #374151;
		color: #d1d5db;
	}

	:global(.dark) .secondary-button:hover {
		background: #4b5563;
	}

	.primary-button {
		background: #2563eb;
		color: #ffffff;
	}

	.primary-button:hover:not(:disabled) {
		background: #1d4ed8;
	}

	.primary-button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.button-spinner,
	.page-spinner {
		animation: invoice-spin 1s linear infinite;
	}

	.button-spinner {
		width: 1.25rem;
		height: 1.25rem;
	}

	.spinner-track {
		opacity: 0.25;
	}

	.spinner-mark {
		opacity: 0.75;
	}

	.alert {
		gap: 0.75rem;
		margin-bottom: 1.5rem;
		padding: 1rem;
		border-radius: 0.5rem;
	}

	.alert--error {
		background: #fee2e2;
		color: #b91c1c;
	}

	.alert--success {
		background: #dcfce7;
		color: #15803d;
	}

	:global(.dark) .alert--error {
		background: rgba(127, 29, 29, 0.3);
		color: #f87171;
	}

	:global(.dark) .alert--success {
		background: rgba(20, 83, 45, 0.3);
		color: #4ade80;
	}

	.loading-state {
		display: flex;
		align-items: center;
		justify-content: center;
		padding-block: 5rem;
	}

	.page-spinner {
		width: 3rem;
		height: 3rem;
		border: 2px solid rgba(37, 99, 235, 0.25);
		border-bottom-color: #2563eb;
		border-radius: 999px;
	}

	.settings-layout {
		display: grid;
		grid-template-columns: 1fr;
		gap: 2rem;
	}

	.settings-card,
	.preview-card,
	.preview-modal {
		border: 1px solid #e5e7eb;
		border-radius: 0.75rem;
		background: #ffffff;
		box-shadow: 0 1px 2px rgba(15, 23, 42, 0.06);
		overflow: hidden;
	}

	:global(.dark) .settings-card,
	:global(.dark) .preview-card,
	:global(.dark) .preview-modal {
		border-color: #374151;
		background: #1f2937;
	}

	.tabs-shell,
	.preview-card-header,
	.preview-modal-header,
	.danger-zone {
		border-bottom: 1px solid #e5e7eb;
	}

	:global(.dark) .tabs-shell,
	:global(.dark) .preview-card-header,
	:global(.dark) .preview-modal-header,
	:global(.dark) .danger-zone {
		border-color: #374151;
	}

	.tabs {
		display: flex;
		overflow-x: auto;
	}

	.tab-button {
		padding: 1rem 1.5rem;
		border: 0;
		border-bottom: 2px solid transparent;
		background: transparent;
		color: #6b7280;
		font: inherit;
		font-size: 0.875rem;
		font-weight: 500;
		white-space: nowrap;
		cursor: pointer;
		transition:
			background-color 0.2s ease,
			border-color 0.2s ease,
			color 0.2s ease;
	}

	.tab-button:hover {
		color: #374151;
	}

	:global(.dark) .tab-button {
		color: #9ca3af;
	}

	:global(.dark) .tab-button:hover {
		color: #d1d5db;
	}

	.tab-button.active {
		border-color: #2563eb;
		background: #eff6ff;
		color: #2563eb;
	}

	:global(.dark) .tab-button.active {
		background: rgba(30, 64, 175, 0.2);
	}

	.settings-card-body,
	.preview-card-body {
		padding: 1.5rem;
	}

	.tab-panel,
	.option-list {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.option-list {
		gap: 1rem;
	}

	.form-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: 1.5rem;
	}

	.form-field {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.form-label,
	.checkbox-label,
	.option-title {
		color: #374151;
		font-size: 0.875rem;
		font-weight: 500;
	}

	:global(.dark) .form-label,
	:global(.dark) .checkbox-label,
	:global(.dark) .option-title {
		color: #d1d5db;
	}

	.form-label--spaced {
		margin-bottom: 0.25rem;
	}

	.form-control {
		width: 100%;
		padding: 0.75rem 1rem;
		border: 1px solid #d1d5db;
		border-radius: 0.5rem;
		background: #ffffff;
		color: #111827;
		font: inherit;
	}

	:global(.dark) .form-control {
		border-color: #4b5563;
		background: #374151;
		color: #ffffff;
	}

	.form-control:focus {
		outline: 2px solid #2563eb;
		outline-offset: 2px;
	}

	.textarea-control {
		resize: none;
	}

	.mono-control {
		font-family: var(--font-mono);
		font-size: 0.875rem;
	}

	.logo-row {
		align-items: flex-start;
		gap: 1.5rem;
	}

	.logo-dropzone {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 10rem;
		height: 6rem;
		border: 2px dashed #d1d5db;
		border-radius: 0.5rem;
		background: #f9fafb;
		overflow: hidden;
	}

	:global(.dark) .logo-dropzone {
		border-color: #4b5563;
		background: #111827;
	}

	.logo-preview {
		max-width: 100%;
		max-height: 100%;
		object-fit: contain;
	}

	.logo-actions {
		flex: 1;
		min-width: 0;
	}

	.file-button {
		display: inline-flex;
	}

	.visually-hidden {
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

	.danger-soft-button {
		padding: 0.5rem 1rem;
		background: #fee2e2;
		color: #dc2626;
	}

	.danger-soft-button:hover {
		background: #fecaca;
	}

	:global(.dark) .danger-soft-button {
		background: rgba(127, 29, 29, 0.3);
	}

	:global(.dark) .danger-soft-button:hover {
		background: rgba(127, 29, 29, 0.5);
	}

	.field-help,
	.option-copy {
		margin: 0;
		color: #6b7280;
		font-size: 0.875rem;
	}

	.option-copy {
		font-size: 0.75rem;
	}

	.field-help--spaced {
		margin-bottom: 0.75rem;
	}

	.color-row {
		gap: 0.75rem;
	}

	.color-input {
		width: 3rem;
		height: 3rem;
		border: 0;
		border-radius: 0.5rem;
		cursor: pointer;
	}

	.checkbox-row,
	.option-card {
		gap: 0.75rem;
		cursor: pointer;
	}

	.checkbox-input {
		width: 1.25rem;
		height: 1.25rem;
		accent-color: #2563eb;
	}

	.option-card {
		padding: 1rem;
		border-radius: 0.5rem;
		background: #f9fafb;
		transition: background-color 0.2s ease;
	}

	.option-card:hover {
		background: #f3f4f6;
	}

	:global(.dark) .option-card {
		background: #111827;
	}

	:global(.dark) .option-card:hover {
		background: #1f2937;
	}

	.inline-code {
		padding-inline: 0.25rem;
		border-radius: 0.25rem;
		background: #f3f4f6;
	}

	:global(.dark) .inline-code {
		background: #374151;
	}

	.danger-zone {
		padding-top: 1rem;
		border-bottom: 0;
		border-top: 1px solid #e5e7eb;
	}

	.preview-sticky {
		position: sticky;
		top: 1rem;
	}

	.preview-card-header,
	.preview-modal-header {
		justify-content: space-between;
		padding: 0.75rem 1rem;
	}

	.preview-card-title,
	.preview-modal-title {
		margin: 0;
		font-weight: 600;
	}

	.preview-card-title {
		color: #111827;
	}

	:global(.dark) .preview-card-title,
	:global(.dark) .preview-modal-title {
		color: #ffffff;
	}

	.mini-invoice {
		aspect-ratio: 8.5 / 11;
		overflow: hidden;
		border: 1px solid #e5e7eb;
		border-radius: 0.5rem;
		background: #ffffff;
		box-shadow: inset 0 2px 8px rgba(15, 23, 42, 0.08);
		color: #111827;
		font-size: 6px;
	}

	.mini-invoice-inner {
		padding: 0.75rem;
	}

	.mini-invoice-header {
		align-items: flex-start;
		justify-content: space-between;
		margin-bottom: 0.5rem;
		padding-bottom: 0.5rem;
		border-bottom: 2px solid;
	}

	.mini-logo {
		height: 1.5rem;
		object-fit: contain;
	}

	.mini-invoice-title-wrap,
	.mini-right {
		text-align: right;
	}

	.mini-title {
		font-size: 10px;
		font-weight: 700;
	}

	.mini-brand {
		font-size: 8px;
		font-weight: 700;
	}

	.mini-muted {
		color: #6b7280;
		font-size: 5px;
	}

	.mini-addresses {
		justify-content: space-between;
		margin-bottom: 0.5rem;
	}

	.mini-section-label {
		font-size: 4px;
		font-weight: 700;
	}

	.mini-text,
	.mini-total-text {
		font-size: 5px;
	}

	.mini-total-text {
		font-weight: 700;
	}

	.mini-table {
		width: 100%;
		margin-bottom: 0.5rem;
		border-collapse: collapse;
		font-size: 4px;
	}

	.mini-table th,
	.mini-table td {
		padding: 0.25rem;
		text-align: left;
	}

	.mini-table tbody tr {
		border-bottom: 1px solid #e5e7eb;
	}

	.mini-total-row {
		justify-content: flex-end;
	}

	.mini-total {
		padding: 0.25rem;
		border-radius: 0.25rem;
		text-align: right;
	}

	.mini-footer {
		margin-top: 0.5rem;
		padding-top: 0.25rem;
		border-top: 1px solid #e5e7eb;
		color: #6b7280;
		font-size: 4px;
		text-align: center;
	}

	.preview-actions {
		margin-top: 1rem;
	}

	.compact-button {
		flex: 1;
		justify-content: center;
		padding: 0.5rem 0.75rem;
		font-size: 0.875rem;
	}

	.preview-modal-backdrop {
		position: fixed;
		inset: 0;
		z-index: 50;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 1rem;
		background: rgba(0, 0, 0, 0.5);
	}

	.preview-modal {
		display: flex;
		flex-direction: column;
		width: min(100%, 56rem);
		max-height: 90vh;
	}

	.preview-modal-body {
		flex: 1;
		overflow: auto;
		padding: 1.5rem;
		background: #f3f4f6;
	}

	:global(.dark) .preview-modal-body {
		background: #111827;
	}

	.preview-document {
		max-width: 800px;
		margin-inline: auto;
		background: #ffffff;
		box-shadow: 0 10px 30px rgba(15, 23, 42, 0.18);
	}

	.icon-button {
		padding: 0.5rem;
		background: transparent;
	}

	.icon-button:hover {
		background: #f3f4f6;
	}

	:global(.dark) .icon-button:hover {
		background: #374151;
	}

	@media (min-width: 768px) {
		.form-grid--two {
			grid-template-columns: repeat(2, minmax(0, 1fr));
		}

		.form-grid--three {
			grid-template-columns: repeat(3, minmax(0, 1fr));
		}

		.form-grid--four {
			grid-template-columns: repeat(4, minmax(0, 1fr));
		}
	}

	@media (min-width: 1024px) {
		.settings-layout {
			grid-template-columns: minmax(0, 2fr) minmax(18rem, 1fr);
		}
	}

	@media (max-width: 640px) {
		.page-actions,
		.logo-row,
		.preview-actions {
			align-items: stretch;
			flex-direction: column;
		}

		.secondary-button,
		.primary-button,
		.danger-soft-button {
			justify-content: center;
			width: 100%;
		}
	}

	@keyframes invoice-spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>

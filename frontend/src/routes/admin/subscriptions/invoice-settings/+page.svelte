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

<div class="container mx-auto px-4 py-8 max-w-6xl">
	<!-- Header -->
	<div class="flex flex-wrap items-center justify-between gap-4 mb-8">
		<div>
			<h1 class="text-3xl font-bold text-gray-900 dark:text-white">Invoice Customization</h1>
			<p class="text-gray-600 dark:text-gray-400 mt-1">
				Customize your invoice appearance with logo, colors, and content
			</p>
		</div>
		<div class="flex gap-3">
			<button
				onclick={loadPreview}
				class="px-4 py-2 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-200 dark:hover:bg-gray-600 transition flex items-center gap-2"
			>
				<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: eye (preview) -->
				<IconEye size={20} aria-hidden="true" />
				Preview
			</button>
			<button
				onclick={downloadPreview}
				class="px-4 py-2 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-200 dark:hover:bg-gray-600 transition flex items-center gap-2"
			>
				<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: file-download -->
				<IconFileDownload size={20} aria-hidden="true" />
				Download PDF
			</button>
			<button
				onclick={saveSettings}
				disabled={saving}
				class="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition disabled:opacity-50 flex items-center gap-2"
			>
				{#if saving}
					<svg aria-hidden="true" class="animate-spin w-5 h-5" fill="none" viewBox="0 0 24 24">
						<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"
						></circle>
						<path
							class="opacity-75"
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
		<div
			transition:slide
			class="mb-6 p-4 bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-400 rounded-lg flex items-center gap-3"
		>
			<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: circle-x error -->
			<IconCircleXFilled size={20} aria-hidden="true" />
			{error}
		</div>
	{/if}

	{#if success}
		<div
			transition:slide
			class="mb-6 p-4 bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400 rounded-lg flex items-center gap-3"
		>
			<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: circle-check success -->
			<IconCircleCheckFilled size={20} aria-hidden="true" />
			{success}
		</div>
	{/if}

	{#if loading}
		<div class="flex items-center justify-center py-20">
			<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
		</div>
	{:else}
		<div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
			<!-- Settings Panel -->
			<div class="lg:col-span-2">
				<!-- Tabs -->
				<div
					class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 overflow-hidden"
				>
					<div class="border-b border-gray-200 dark:border-gray-700">
						<nav class="flex overflow-x-auto">
							{#each tabs as tab (tab.id)}
								<button
									onclick={() => (activeTab = tab.id)}
									class="px-6 py-4 text-sm font-medium whitespace-nowrap transition-colors
									{activeTab === tab.id
										? 'text-blue-600 border-b-2 border-blue-600 bg-blue-50 dark:bg-blue-900/20'
										: 'text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300'}"
								>
									{tab.label}
								</button>
							{/each}
						</nav>
					</div>

					<div class="p-6">
						<!-- Branding Tab -->
						{#if activeTab === 'branding'}
							<div class="space-y-6" transition:fade>
								<!-- Logo Upload -->
								<div>
									<div class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-3">
										Company Logo
									</div>
									<div class="flex items-start gap-6">
										<div
											class="w-40 h-24 border-2 border-dashed border-gray-300 dark:border-gray-600 rounded-lg flex items-center justify-center bg-gray-50 dark:bg-gray-900 overflow-hidden"
										>
											{#if logoUrl}
												<img
													src={logoUrl}
													alt="Logo"
													class="max-w-full max-h-full object-contain"
													width="160"
													height="96"
													loading="lazy"
												/>
											{:else}
												<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: photo (logo placeholder) -->
												<IconPhoto size={40} aria-hidden="true" />
											{/if}
										</div>
										<div class="flex-1">
											<div class="flex gap-3">
												<label
													class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 cursor-pointer transition"
												>
													{uploading ? 'Uploading...' : 'Upload Logo'}
													<input
														type="file"
														class="hidden"
														accept="image/*"
														onchange={uploadLogo}
														disabled={uploading}
													/>
												</label>
												{#if logoUrl}
													<button
														onclick={removeLogo}
														class="px-4 py-2 bg-red-100 dark:bg-red-900/30 text-red-600 rounded-lg hover:bg-red-200 dark:hover:bg-red-900/50 transition"
													>
														Remove
													</button>
												{/if}
											</div>
											<p class="text-sm text-gray-500 mt-2">PNG, JPG, SVG or WebP. Max 2MB.</p>
										</div>
									</div>
								</div>

								<!-- Show Logo Toggle -->
								<label for="show-logo-toggle" class="flex items-center gap-3 cursor-pointer">
									<input
										type="checkbox"
										id="show-logo-toggle"
										name="show-logo-toggle"
										bind:checked={settings.show_logo}
										class="w-5 h-5 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
									/>
									<span class="text-sm text-gray-700 dark:text-gray-300">Show logo on invoices</span
									>
								</label>

								<!-- Colors -->
								<div class="grid grid-cols-1 md:grid-cols-3 gap-6">
									<div>
										<label
											for="primary-color"
											class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
										>
											Primary Color
										</label>
										<div class="flex items-center gap-3">
											<input
												type="color"
												id="primary-color"
												name="primary-color"
												bind:value={settings.primary_color}
												class="w-12 h-12 rounded-lg cursor-pointer border-0"
											/>
											<input
												type="text"
												id="primary-color-text"
												name="primary-color-text"
												bind:value={settings.primary_color}
												class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white font-mono text-sm"
											/>
										</div>
									</div>
									<div>
										<label
											for="secondary-color"
											class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
										>
											Secondary Color
										</label>
										<div class="flex items-center gap-3">
											<input
												type="color"
												id="secondary-color"
												name="secondary-color"
												bind:value={settings.secondary_color}
												class="w-12 h-12 rounded-lg cursor-pointer border-0"
											/>
											<input
												type="text"
												id="secondary-color-text"
												name="secondary-color-text"
												bind:value={settings.secondary_color}
												class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white font-mono text-sm"
											/>
										</div>
									</div>
									<div>
										<label
											for="accent-color"
											class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
										>
											Accent Color
										</label>
										<div class="flex items-center gap-3">
											<input
												type="color"
												id="accent-color"
												name="accent-color"
												bind:value={settings.accent_color}
												class="w-12 h-12 rounded-lg cursor-pointer border-0"
											/>
											<input
												type="text"
												id="accent-color-text"
												name="accent-color-text"
												bind:value={settings.accent_color}
												class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white font-mono text-sm"
											/>
										</div>
									</div>
								</div>

								<!-- Font Family -->
								<div>
									<label
										for="font-family"
										class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
									>
										Font Family
									</label>
									<select
										id="font-family"
										bind:value={settings.font_family}
										class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
									>
										{#each fonts as font (font.value)}
											<option value={font.value}>{font.label}</option>
										{/each}
									</select>
								</div>
							</div>
						{/if}

						<!-- Company Info Tab -->
						{#if activeTab === 'company'}
							<div class="space-y-6" transition:fade>
								<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
									<div>
										<label
											for="company-name"
											class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
										>
											Company Name
										</label>
										<input
											type="text"
											id="company-name"
											name="company-name"
											bind:value={settings.company_name}
											class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
											placeholder="Your Company Name"
										/>
									</div>
									<div>
										<label
											for="company-email"
											class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
										>
											Email Address
										</label>
										<input
											type="email"
											id="company-email"
											name="company-email"
											autocomplete="email"
											bind:value={settings.company_email}
											class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
											placeholder="billing@company.com"
										/>
									</div>
								</div>

								<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
									<div>
										<label
											for="company-phone"
											class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
										>
											Phone Number
										</label>
										<input
											type="tel"
											id="company-phone"
											name="company-phone"
											autocomplete="tel"
											bind:value={settings.company_phone}
											class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
											placeholder="+1 (555) 123-4567"
										/>
									</div>
									<div>
										<label
											for="tax-id"
											class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
										>
											Tax ID / VAT Number
										</label>
										<input
											type="text"
											id="tax-id"
											name="tax-id"
											bind:value={settings.tax_id}
											class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
											placeholder="US123456789"
										/>
									</div>
								</div>

								<div>
									<label
										for="company-address"
										class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
									>
										Street Address
									</label>
									<input
										type="text"
										id="company-address"
										name="company-address"
										bind:value={settings.company_address}
										class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
										placeholder="123 Business Street, Suite 100"
									/>
								</div>

								<div class="grid grid-cols-2 md:grid-cols-4 gap-4">
									<div>
										<label
											for="company-city"
											class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
											>City</label
										>
										<input
											type="text"
											id="company-city"
											name="company-city"
											bind:value={settings.company_city}
											class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
										/>
									</div>
									<div>
										<label
											for="company-state"
											class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
											>State</label
										>
										<input
											type="text"
											id="company-state"
											name="company-state"
											bind:value={settings.company_state}
											class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
										/>
									</div>
									<div>
										<label
											for="company-zip"
											class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
											>ZIP</label
										>
										<input
											type="text"
											id="company-zip"
											name="company-zip"
											bind:value={settings.company_zip}
											class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
										/>
									</div>
									<div>
										<label
											for="company-country"
											class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
											>Country</label
										>
										<select
											id="company-country"
											bind:value={settings.company_country}
											class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
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
							<div class="space-y-6" transition:fade>
								<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
									<div>
										<label
											for="header-text"
											class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
										>
											Header Text
										</label>
										<input
											type="text"
											id="header-text"
											name="header-text"
											bind:value={settings.header_text}
											class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
											placeholder="INVOICE"
										/>
									</div>
									<div>
										<label
											for="invoice-prefix"
											class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
										>
											Invoice Prefix
										</label>
										<input
											type="text"
											id="invoice-prefix"
											name="invoice-prefix"
											bind:value={settings.invoice_prefix}
											class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
											placeholder="INV-"
										/>
									</div>
								</div>

								<div>
									<label
										for="payment-terms"
										class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
									>
										Payment Terms
									</label>
									<input
										type="text"
										id="payment-terms"
										name="payment-terms"
										bind:value={settings.payment_terms}
										class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
										placeholder="Due upon receipt"
									/>
								</div>

								<div>
									<label
										for="footer-text"
										class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
									>
										Footer Text
									</label>
									<input
										type="text"
										id="footer-text"
										name="footer-text"
										bind:value={settings.footer_text}
										class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
										placeholder="Thank you for your business!"
									/>
								</div>

								<div>
									<label
										for="notes-template"
										class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
									>
										Default Notes (shown on all invoices)
									</label>
									<textarea
										id="notes-template"
										bind:value={settings.notes_template}
										rows="4"
										class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white resize-none"
										placeholder="Add any terms, conditions, or notes to include on every invoice..."
									></textarea>
								</div>
							</div>
						{/if}

						<!-- Display Options Tab -->
						{#if activeTab === 'display'}
							<div class="space-y-4" transition:fade>
								<p class="text-sm text-gray-600 dark:text-gray-400 mb-4">
									Choose which elements to display on your invoices:
								</p>

								<label
									for="display-show-logo"
									class="flex items-center gap-3 p-4 bg-gray-50 dark:bg-gray-900 rounded-lg cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-800 transition"
								>
									<input
										type="checkbox"
										id="display-show-logo"
										name="display-show-logo"
										bind:checked={settings.show_logo}
										class="w-5 h-5 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
									/>
									<div>
										<span class="text-sm font-medium text-gray-700 dark:text-gray-300"
											>Show Company Logo</span
										>
										<p class="text-xs text-gray-500">
											Display your uploaded logo at the top of invoices
										</p>
									</div>
								</label>

								<label
									for="display-show-tax-id"
									class="flex items-center gap-3 p-4 bg-gray-50 dark:bg-gray-900 rounded-lg cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-800 transition"
								>
									<input
										type="checkbox"
										id="display-show-tax-id"
										name="display-show-tax-id"
										bind:checked={settings.show_tax_id}
										class="w-5 h-5 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
									/>
									<div>
										<span class="text-sm font-medium text-gray-700 dark:text-gray-300"
											>Show Tax ID / VAT Number</span
										>
										<p class="text-xs text-gray-500">
											Include your tax identification number in the header
										</p>
									</div>
								</label>

								<label
									for="display-show-payment-method"
									class="flex items-center gap-3 p-4 bg-gray-50 dark:bg-gray-900 rounded-lg cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-800 transition"
								>
									<input
										type="checkbox"
										id="display-show-payment-method"
										name="display-show-payment-method"
										bind:checked={settings.show_payment_method}
										class="w-5 h-5 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
									/>
									<div>
										<span class="text-sm font-medium text-gray-700 dark:text-gray-300"
											>Show Payment Method</span
										>
										<p class="text-xs text-gray-500">
											Display the card type and last 4 digits used for payment
										</p>
									</div>
								</label>

								<label
									for="display-show-due-date"
									class="flex items-center gap-3 p-4 bg-gray-50 dark:bg-gray-900 rounded-lg cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-800 transition"
								>
									<input
										type="checkbox"
										id="display-show-due-date"
										name="display-show-due-date"
										bind:checked={settings.show_due_date}
										class="w-5 h-5 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
									/>
									<div>
										<span class="text-sm font-medium text-gray-700 dark:text-gray-300"
											>Show Due Date</span
										>
										<p class="text-xs text-gray-500">
											Include payment due date in the invoice details
										</p>
									</div>
								</label>
							</div>
						{/if}

						<!-- Advanced Tab -->
						{#if activeTab === 'advanced'}
							<div class="space-y-6" transition:fade>
								<div>
									<label
										for="custom-css"
										class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
									>
										Custom CSS
									</label>
									<p class="text-sm text-gray-500 mb-3">
										Add custom CSS to further customize your invoice appearance. Use CSS variables
										like <code class="bg-gray-100 dark:bg-gray-700 px-1 rounded"
											>var(--primary-color)</code
										>.
									</p>
									<textarea
										id="custom-css"
										bind:value={settings.custom_css}
										rows="10"
										class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white font-mono text-sm resize-none"
										placeholder={`.invoice-title { font-size: 36px; }
.items-table th { border-radius: 0; }
.footer-text { font-style: italic; }`}
									></textarea>
								</div>

								<div class="pt-4 border-t border-gray-200 dark:border-gray-700">
									<button
										onclick={resetToDefaults}
										class="px-4 py-2 bg-red-100 dark:bg-red-900/30 text-red-600 rounded-lg hover:bg-red-200 dark:hover:bg-red-900/50 transition"
									>
										Reset to Defaults
									</button>
								</div>
							</div>
						{/if}
					</div>
				</div>
			</div>

			<!-- Live Preview Panel -->
			<div class="lg:col-span-1">
				<div class="sticky top-4">
					<div
						class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 overflow-hidden"
					>
						<div class="px-4 py-3 border-b border-gray-200 dark:border-gray-700">
							<h3 class="font-semibold text-gray-900 dark:text-white">Live Preview</h3>
						</div>
						<div class="p-4">
							<!-- Mini Preview -->
							<div
								class="aspect-[8.5/11] bg-white rounded-lg shadow-inner border overflow-hidden"
								style="font-family: {settings.font_family}; font-size: 6px;"
							>
								<div class="p-3">
									<!-- Header -->
									<div
										class="flex justify-between items-start border-b-2 pb-2 mb-2"
										style="border-color: {settings.primary_color};"
									>
										<div>
											{#if settings.show_logo && logoUrl}
												<img
													src={logoUrl}
													alt="Logo"
													class="h-6 object-contain"
													width="96"
													height="24"
													loading="lazy"
												/>
											{:else}
												<div class="font-bold text-[8px]" style="color: {settings.primary_color};">
													{settings.company_name || 'Company'}
												</div>
											{/if}
										</div>
										<div class="text-right">
											<div class="font-bold text-[10px]" style="color: {settings.secondary_color};">
												{settings.header_text || 'INVOICE'}
											</div>
											<div class="text-gray-500 text-[5px]">#{settings.invoice_prefix}001</div>
										</div>
									</div>

									<!-- Addresses -->
									<div class="flex justify-between mb-2">
										<div>
											<div class="text-[4px] font-bold" style="color: {settings.primary_color};">
												FROM
											</div>
											<div class="text-[5px]">{settings.company_name || 'Your Company'}</div>
										</div>
										<div class="text-right">
											<div class="text-[4px] font-bold" style="color: {settings.primary_color};">
												BILL TO
											</div>
											<div class="text-[5px]">John Doe</div>
										</div>
									</div>

									<!-- Table -->
									<table class="w-full text-[4px] mb-2">
										<thead>
											<tr style="background: {settings.primary_color}; color: white;">
												<th class="p-1 text-left">Description</th>
												<th class="p-1 text-right">Amount</th>
											</tr>
										</thead>
										<tbody>
											<tr class="border-b">
												<td class="p-1">Professional Plan</td>
												<td class="p-1 text-right">$99.00</td>
											</tr>
										</tbody>
									</table>

									<!-- Total -->
									<div class="flex justify-end">
										<div
											class="text-right p-1 rounded"
											style="background: {settings.primary_color}; color: white;"
										>
											<span class="text-[5px] font-bold">Total: $99.00</span>
										</div>
									</div>

									<!-- Footer -->
									<div class="mt-2 pt-1 border-t text-center text-[4px] text-gray-500">
										{settings.footer_text || 'Thank you!'}
									</div>
								</div>
							</div>

							<div class="mt-4 flex gap-2">
								<button
									onclick={loadPreview}
									class="flex-1 px-3 py-2 bg-blue-600 text-white text-sm rounded-lg hover:bg-blue-700 transition"
								>
									Full Preview
								</button>
								<button
									onclick={downloadPreview}
									class="flex-1 px-3 py-2 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 text-sm rounded-lg hover:bg-gray-200 dark:hover:bg-gray-600 transition"
								>
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
		<div
			class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4"
			transition:fade
		>
			<div
				class="bg-white dark:bg-gray-800 rounded-xl max-w-4xl w-full max-h-[90vh] overflow-hidden flex flex-col"
			>
				<div
					class="px-6 py-4 border-b border-gray-200 dark:border-gray-700 flex items-center justify-between"
				>
					<h3 class="text-lg font-semibold text-gray-900 dark:text-white">Invoice Preview</h3>
					<button
						onclick={() => (showPreviewModal = false)}
						class="p-2 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition"
						aria-label="Close preview"
					>
						<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: x (close preview) -->
						<IconX size={20} aria-hidden="true" />
					</button>
				</div>
				<div class="flex-1 overflow-auto p-6 bg-gray-100 dark:bg-gray-900">
					<div class="bg-white shadow-lg mx-auto" style="max-width: 800px;">
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

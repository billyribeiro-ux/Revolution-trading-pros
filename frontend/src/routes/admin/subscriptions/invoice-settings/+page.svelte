<script lang="ts">
	import { onMount } from 'svelte';
	import { fade, slide } from 'svelte/transition';

	// API URL
	const API_BASE = '/api/admin/invoice-settings';

	// Settings State
	let settings = {
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
	};

	let logoUrl: string | null = null;
	let fonts: { value: string; label: string }[] = [];
	let countries: Record<string, string> = {};

	// UI State
	let loading = true;
	let saving = false;
	let uploading = false;
	let error = '';
	let success = '';
	let activeTab: 'branding' | 'company' | 'content' | 'display' | 'advanced' = 'branding';
	let previewHtml = '';
	let showPreviewModal = false;

	onMount(async () => {
		await loadSettings();
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
		} catch (err) {
			error = 'Failed to remove logo';
		}
	}

	async function loadPreview() {
		try {
			const response = await fetch(`${API_BASE}/preview-html`);
			const data = await response.json();
			previewHtml = data.html;
			showPreviewModal = true;
		} catch (err) {
			error = 'Failed to load preview';
		}
	}

	function downloadPreview() {
		window.open(`${API_BASE}/preview`, '_blank');
	}

	async function resetToDefaults() {
		if (!confirm('Are you sure you want to reset all settings to defaults?')) return;

		try {
			const response = await fetch(`${API_BASE}/reset`, { method: 'POST' });
			const data = await response.json();
			settings = { ...settings, ...data.settings };
			success = 'Settings reset to defaults';
			setTimeout(() => (success = ''), 3000);
		} catch (err) {
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
				<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
				</svg>
				Preview
			</button>
			<button
				onclick={downloadPreview}
				class="px-4 py-2 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-200 dark:hover:bg-gray-600 transition flex items-center gap-2"
			>
				<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
				</svg>
				Download PDF
			</button>
			<button
				onclick={saveSettings}
				disabled={saving}
				class="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition disabled:opacity-50 flex items-center gap-2"
			>
				{#if saving}
					<svg class="animate-spin w-5 h-5" fill="none" viewBox="0 0 24 24">
						<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
						<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
					</svg>
					Saving...
				{:else}
					<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
					</svg>
					Save Changes
				{/if}
			</button>
		</div>
	</div>

	<!-- Alerts -->
	{#if error}
		<div transition:slide class="mb-6 p-4 bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-400 rounded-lg flex items-center gap-3">
			<svg class="w-5 h-5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
				<path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
			</svg>
			{error}
		</div>
	{/if}

	{#if success}
		<div transition:slide class="mb-6 p-4 bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400 rounded-lg flex items-center gap-3">
			<svg class="w-5 h-5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
				<path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
			</svg>
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
				<div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 overflow-hidden">
					<div class="border-b border-gray-200 dark:border-gray-700">
						<nav class="flex overflow-x-auto">
							{#each tabs as tab}
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
									<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-3">
										Company Logo
									</label>
									<div class="flex items-start gap-6">
										<div class="w-40 h-24 border-2 border-dashed border-gray-300 dark:border-gray-600 rounded-lg flex items-center justify-center bg-gray-50 dark:bg-gray-900 overflow-hidden">
											{#if logoUrl}
												<img src={logoUrl} alt="Logo" class="max-w-full max-h-full object-contain" />
											{:else}
												<svg class="w-10 h-10 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
													<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
												</svg>
											{/if}
										</div>
										<div class="flex-1">
											<div class="flex gap-3">
												<label class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 cursor-pointer transition">
													{uploading ? 'Uploading...' : 'Upload Logo'}
													<input type="file" class="hidden" accept="image/*" onchange={uploadLogo} disabled={uploading} />
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
								<label class="flex items-center gap-3 cursor-pointer">
									<input type="checkbox" bind:checked={settings.show_logo} class="w-5 h-5 rounded border-gray-300 text-blue-600 focus:ring-blue-500" />
									<span class="text-sm text-gray-700 dark:text-gray-300">Show logo on invoices</span>
								</label>

								<!-- Colors -->
								<div class="grid grid-cols-1 md:grid-cols-3 gap-6">
									<div>
										<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
											Primary Color
										</label>
										<div class="flex items-center gap-3">
											<input
												type="color"
												bind:value={settings.primary_color}
												class="w-12 h-12 rounded-lg cursor-pointer border-0"
											/>
											<input
												type="text"
												bind:value={settings.primary_color}
												class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white font-mono text-sm"
											/>
										</div>
									</div>
									<div>
										<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
											Secondary Color
										</label>
										<div class="flex items-center gap-3">
											<input
												type="color"
												bind:value={settings.secondary_color}
												class="w-12 h-12 rounded-lg cursor-pointer border-0"
											/>
											<input
												type="text"
												bind:value={settings.secondary_color}
												class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white font-mono text-sm"
											/>
										</div>
									</div>
									<div>
										<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
											Accent Color
										</label>
										<div class="flex items-center gap-3">
											<input
												type="color"
												bind:value={settings.accent_color}
												class="w-12 h-12 rounded-lg cursor-pointer border-0"
											/>
											<input
												type="text"
												bind:value={settings.accent_color}
												class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white font-mono text-sm"
											/>
										</div>
									</div>
								</div>

								<!-- Font Family -->
								<div>
									<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
										Font Family
									</label>
									<select
										bind:value={settings.font_family}
										class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
									>
										{#each fonts as font}
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
										<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
											Company Name
										</label>
										<input
											type="text"
											bind:value={settings.company_name}
											class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
											placeholder="Your Company Name"
										/>
									</div>
									<div>
										<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
											Email Address
										</label>
										<input
											type="email"
											bind:value={settings.company_email}
											class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
											placeholder="billing@company.com"
										/>
									</div>
								</div>

								<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
									<div>
										<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
											Phone Number
										</label>
										<input
											type="tel"
											bind:value={settings.company_phone}
											class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
											placeholder="+1 (555) 123-4567"
										/>
									</div>
									<div>
										<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
											Tax ID / VAT Number
										</label>
										<input
											type="text"
											bind:value={settings.tax_id}
											class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
											placeholder="US123456789"
										/>
									</div>
								</div>

								<div>
									<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
										Street Address
									</label>
									<input
										type="text"
										bind:value={settings.company_address}
										class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
										placeholder="123 Business Street, Suite 100"
									/>
								</div>

								<div class="grid grid-cols-2 md:grid-cols-4 gap-4">
									<div>
										<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">City</label>
										<input
											type="text"
											bind:value={settings.company_city}
											class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
										/>
									</div>
									<div>
										<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">State</label>
										<input
											type="text"
											bind:value={settings.company_state}
											class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
										/>
									</div>
									<div>
										<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">ZIP</label>
										<input
											type="text"
											bind:value={settings.company_zip}
											class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
										/>
									</div>
									<div>
										<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Country</label>
										<select
											bind:value={settings.company_country}
											class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
										>
											{#each Object.entries(countries) as [code, name]}
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
										<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
											Header Text
										</label>
										<input
											type="text"
											bind:value={settings.header_text}
											class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
											placeholder="INVOICE"
										/>
									</div>
									<div>
										<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
											Invoice Prefix
										</label>
										<input
											type="text"
											bind:value={settings.invoice_prefix}
											class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
											placeholder="INV-"
										/>
									</div>
								</div>

								<div>
									<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
										Payment Terms
									</label>
									<input
										type="text"
										bind:value={settings.payment_terms}
										class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
										placeholder="Due upon receipt"
									/>
								</div>

								<div>
									<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
										Footer Text
									</label>
									<input
										type="text"
										bind:value={settings.footer_text}
										class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
										placeholder="Thank you for your business!"
									/>
								</div>

								<div>
									<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
										Default Notes (shown on all invoices)
									</label>
									<textarea
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

								<label class="flex items-center gap-3 p-4 bg-gray-50 dark:bg-gray-900 rounded-lg cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-800 transition">
									<input type="checkbox" bind:checked={settings.show_logo} class="w-5 h-5 rounded border-gray-300 text-blue-600 focus:ring-blue-500" />
									<div>
										<span class="text-sm font-medium text-gray-700 dark:text-gray-300">Show Company Logo</span>
										<p class="text-xs text-gray-500">Display your uploaded logo at the top of invoices</p>
									</div>
								</label>

								<label class="flex items-center gap-3 p-4 bg-gray-50 dark:bg-gray-900 rounded-lg cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-800 transition">
									<input type="checkbox" bind:checked={settings.show_tax_id} class="w-5 h-5 rounded border-gray-300 text-blue-600 focus:ring-blue-500" />
									<div>
										<span class="text-sm font-medium text-gray-700 dark:text-gray-300">Show Tax ID / VAT Number</span>
										<p class="text-xs text-gray-500">Include your tax identification number in the header</p>
									</div>
								</label>

								<label class="flex items-center gap-3 p-4 bg-gray-50 dark:bg-gray-900 rounded-lg cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-800 transition">
									<input type="checkbox" bind:checked={settings.show_payment_method} class="w-5 h-5 rounded border-gray-300 text-blue-600 focus:ring-blue-500" />
									<div>
										<span class="text-sm font-medium text-gray-700 dark:text-gray-300">Show Payment Method</span>
										<p class="text-xs text-gray-500">Display the card type and last 4 digits used for payment</p>
									</div>
								</label>

								<label class="flex items-center gap-3 p-4 bg-gray-50 dark:bg-gray-900 rounded-lg cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-800 transition">
									<input type="checkbox" bind:checked={settings.show_due_date} class="w-5 h-5 rounded border-gray-300 text-blue-600 focus:ring-blue-500" />
									<div>
										<span class="text-sm font-medium text-gray-700 dark:text-gray-300">Show Due Date</span>
										<p class="text-xs text-gray-500">Include payment due date in the invoice details</p>
									</div>
								</label>
							</div>
						{/if}

						<!-- Advanced Tab -->
						{#if activeTab === 'advanced'}
							<div class="space-y-6" transition:fade>
								<div>
									<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
										Custom CSS
									</label>
									<p class="text-sm text-gray-500 mb-3">
										Add custom CSS to further customize your invoice appearance. Use CSS variables like <code class="bg-gray-100 dark:bg-gray-700 px-1 rounded">var(--primary-color)</code>.
									</p>
									<textarea
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
					<div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 overflow-hidden">
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
									<div class="flex justify-between items-start border-b-2 pb-2 mb-2" style="border-color: {settings.primary_color};">
										<div>
											{#if settings.show_logo && logoUrl}
												<img src={logoUrl} alt="Logo" class="h-6 object-contain" />
											{:else}
												<div class="font-bold text-[8px]" style="color: {settings.primary_color};">{settings.company_name || 'Company'}</div>
											{/if}
										</div>
										<div class="text-right">
											<div class="font-bold text-[10px]" style="color: {settings.secondary_color};">{settings.header_text || 'INVOICE'}</div>
											<div class="text-gray-500 text-[5px]">#{settings.invoice_prefix}001</div>
										</div>
									</div>

									<!-- Addresses -->
									<div class="flex justify-between mb-2">
										<div>
											<div class="text-[4px] font-bold" style="color: {settings.primary_color};">FROM</div>
											<div class="text-[5px]">{settings.company_name || 'Your Company'}</div>
										</div>
										<div class="text-right">
											<div class="text-[4px] font-bold" style="color: {settings.primary_color};">BILL TO</div>
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
										<div class="text-right p-1 rounded" style="background: {settings.primary_color}; color: white;">
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
		<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4" transition:fade>
			<div class="bg-white dark:bg-gray-800 rounded-xl max-w-4xl w-full max-h-[90vh] overflow-hidden flex flex-col">
				<div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700 flex items-center justify-between">
					<h3 class="text-lg font-semibold text-gray-900 dark:text-white">Invoice Preview</h3>
					<button
						onclick={() => (showPreviewModal = false)}
						class="p-2 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition"
					>
						<svg class="w-5 h-5 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
						</svg>
					</button>
				</div>
				<div class="flex-1 overflow-auto p-6 bg-gray-100 dark:bg-gray-900">
					<div class="bg-white shadow-lg mx-auto" style="max-width: 800px;">
						{@html previewHtml}
					</div>
				</div>
			</div>
		</div>
	{/if}
</div>

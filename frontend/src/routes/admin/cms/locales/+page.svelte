<!--
	CMS Localization - Apple ICT 11+ Principal Engineer Grade
	10/10 i18n & Translation Management
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import { fly, scale, fade } from 'svelte/transition';
	import { backOut } from 'svelte/easing';
	import {
		IconWorld,
		IconArrowLeft,
		IconPlus,
		IconCheck,
		IconX,
		IconEdit,
		IconTrash,
		IconLanguage,
		IconChevronRight,
		IconSearch,
		IconRefresh
	} from '$lib/icons';

	let mounted = false;
	let isLoading = true;
	let locales: any[] = [];
	let translations: any[] = [];
	let selectedLocale: any = null;
	let showAddLocale = false;
	let searchQuery = '';

	// Supported languages with flags
	const availableLocales = [
		{ code: 'en', name: 'English', flag: '🇺🇸', rtl: false },
		{ code: 'es', name: 'Spanish', flag: '🇪🇸', rtl: false },
		{ code: 'fr', name: 'French', flag: '🇫🇷', rtl: false },
		{ code: 'de', name: 'German', flag: '🇩🇪', rtl: false },
		{ code: 'pt', name: 'Portuguese', flag: '🇧🇷', rtl: false },
		{ code: 'zh', name: 'Chinese', flag: '🇨🇳', rtl: false },
		{ code: 'ja', name: 'Japanese', flag: '🇯🇵', rtl: false },
		{ code: 'ko', name: 'Korean', flag: '🇰🇷', rtl: false },
		{ code: 'ar', name: 'Arabic', flag: '🇸🇦', rtl: true },
		{ code: 'he', name: 'Hebrew', flag: '🇮🇱', rtl: true },
		{ code: 'ru', name: 'Russian', flag: '🇷🇺', rtl: false },
		{ code: 'it', name: 'Italian', flag: '🇮🇹', rtl: false }
	];

	async function fetchLocales() {
		isLoading = true;
		try {
			const response = await fetch('/api/admin/cms/locales', {
				credentials: 'include'
			});
			if (response.ok) {
				locales = await response.json();
			}
		} catch (e) {
			console.error('Failed to fetch locales:', e);
		} finally {
			isLoading = false;
		}
	}

	async function fetchTranslations(localeCode: string) {
		try {
			const params = new URLSearchParams();
			params.set('locale', localeCode);
			if (searchQuery) params.set('search', searchQuery);

			const response = await fetch(`/api/admin/cms/translations?${params}`, {
				credentials: 'include'
			});
			if (response.ok) {
				translations = await response.json();
			}
		} catch (e) {
			console.error('Failed to fetch translations:', e);
		}
	}

	async function addLocale(localeCode: string) {
		try {
			const locale = availableLocales.find((l) => l.code === localeCode);
			if (!locale) return;

			const response = await fetch('/api/admin/cms/locales', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				credentials: 'include',
				body: JSON.stringify({
					code: locale.code,
					name: locale.name,
					is_rtl: locale.rtl
				})
			});
			if (response.ok) {
				fetchLocales();
				showAddLocale = false;
			}
		} catch (e) {
			console.error('Failed to add locale:', e);
		}
	}

	async function toggleLocale(id: number, isActive: boolean) {
		try {
			const response = await fetch(`/api/admin/cms/locales/${id}`, {
				method: 'PATCH',
				headers: { 'Content-Type': 'application/json' },
				credentials: 'include',
				body: JSON.stringify({ is_active: !isActive })
			});
			if (response.ok) {
				fetchLocales();
			}
		} catch (e) {
			console.error('Failed to toggle locale:', e);
		}
	}

	async function deleteLocale(id: number) {
		if (!confirm('Are you sure you want to delete this locale? All translations will be lost.'))
			return;
		try {
			const response = await fetch(`/api/admin/cms/locales/${id}`, {
				method: 'DELETE',
				credentials: 'include'
			});
			if (response.ok) {
				fetchLocales();
				if (selectedLocale?.id === id) {
					selectedLocale = null;
					translations = [];
				}
			}
		} catch (e) {
			console.error('Failed to delete locale:', e);
		}
	}

	function selectLocale(locale: any) {
		selectedLocale = locale;
		fetchTranslations(locale.code);
	}

	function getLocaleInfo(code: string) {
		return (
			availableLocales.find((l) => l.code === code) || { code, name: code, flag: '🌐', rtl: false }
		);
	}

	function getCompletionRate(locale: any): number {
		if (!locale.total_strings || locale.total_strings === 0) return 0;
		return Math.round((locale.translated_strings / locale.total_strings) * 100);
	}

	function getUnusedLocales() {
		const usedCodes = locales.map((l) => l.code);
		return availableLocales.filter((l) => !usedCodes.includes(l.code));
	}

	onMount(() => {
		mounted = true;
		fetchLocales();
	});
</script>

<div class="locales-page" class:mounted>
	<!-- Header -->
	<header class="page-header" in:fly={{ y: -20, duration: 500 }}>
		<div class="header-left">
			<a href="/admin/cms" class="back-link">
				<IconArrowLeft size={18} />
				<span>Back to CMS</span>
			</a>
			<div class="header-title">
				<div class="header-icon">
					<IconWorld size={24} />
				</div>
				<div>
					<h1>Localization</h1>
					<p>Manage languages and content translations</p>
				</div>
			</div>
		</div>

		<button class="btn-add" onclick={() => (showAddLocale = true)}>
			<IconPlus size={18} />
			Add Language
		</button>
	</header>

	<div class="locales-layout">
		<!-- Locales List -->
		<section class="locales-panel" in:fly={{ x: -20, duration: 500, delay: 100 }}>
			<div class="panel-header">
				<h2>Languages</h2>
				<span class="locale-count">{locales.length} active</span>
			</div>

			{#if isLoading}
				<div class="loading-state small">
					<div class="spinner"></div>
				</div>
			{:else if locales.length === 0}
				<div class="empty-state small">
					<IconWorld size={32} />
					<p>No languages configured</p>
				</div>
			{:else}
				<div class="locales-list">
					{#each locales as locale, i}
						{@const info = getLocaleInfo(locale.code)}
						{@const completion = getCompletionRate(locale)}
						<div
							class="locale-item"
							class:selected={selectedLocale?.id === locale.id}
							class:inactive={!locale.is_active}
							in:fly={{ x: -20, duration: 300, delay: i * 50 }}
							onclick={() => selectLocale(locale)}
							role="button"
							tabindex="0"
							onkeydown={(e) => {
								if (e.key === 'Enter' || e.key === ' ') selectLocale(locale);
							}}
						>
							<span class="locale-flag">{info.flag}</span>
							<div class="locale-info">
								<span class="locale-name">{info.name}</span>
								<span class="locale-code">{locale.code.toUpperCase()}</span>
							</div>
							<div class="locale-progress">
								<div class="progress-bar">
									<div class="progress-fill" style="width: {completion}%"></div>
								</div>
								<span class="progress-text">{completion}%</span>
							</div>
							<div class="locale-actions">
								<button
									class="toggle-btn"
									class:active={locale.is_active}
									onclick={(e) => {
										e.stopPropagation();
										toggleLocale(locale.id, locale.is_active);
									}}
								>
									{#if locale.is_active}
										<IconCheck size={14} />
									{:else}
										<IconX size={14} />
									{/if}
								</button>
							</div>
							<IconChevronRight size={16} class="chevron" />
						</div>
					{/each}
				</div>
			{/if}
		</section>

		<!-- Translations Panel -->
		<section class="translations-panel" in:fly={{ x: 20, duration: 500, delay: 100 }}>
			{#if selectedLocale}
				{@const info = getLocaleInfo(selectedLocale.code)}
				<div class="panel-header">
					<div class="panel-title">
						<span class="locale-flag large">{info.flag}</span>
						<div>
							<h2>{info.name} Translations</h2>
							<span class="translation-count">
								{selectedLocale.translated_strings || 0} of {selectedLocale.total_strings || 0} strings
							</span>
						</div>
					</div>
					<div class="panel-actions">
						<div class="search-box">
							<IconSearch size={16} />
							<input
								id="page-searchquery" name="page-searchquery" type="text"
								placeholder="Search translations..."
								bind:value={searchQuery}
								onkeyup={(e) => e.key === 'Enter' && fetchTranslations(selectedLocale.code)}
							/>
						</div>
					</div>
				</div>

				{#if translations.length === 0}
					<div class="empty-state">
						<IconLanguage size={40} />
						<h3>No translations found</h3>
						<p>Translations will appear here when content is localized</p>
					</div>
				{:else}
					<div class="translations-list">
						{#each translations as translation, i}
							<div
								class="translation-item"
								class:rtl={info.rtl}
								in:fly={{ y: 10, duration: 200, delay: i * 30 }}
							>
								<div class="translation-key">
									<span class="key-label">Key</span>
									<span class="key-value">{translation.key}</span>
								</div>
								<div class="translation-value">
									<span class="value-label">Translation</span>
									<span class="value-text" dir={info.rtl ? 'rtl' : 'ltr'}>
										{translation.value || '—'}
									</span>
								</div>
								<div class="translation-meta">
									<span class="meta-type">{translation.content_type}</span>
									{#if translation.is_machine_translated}
										<span class="meta-badge machine">Auto</span>
									{/if}
								</div>
							</div>
						{/each}
					</div>
				{/if}
			{:else}
				<div class="no-selection">
					<IconWorld size={48} />
					<h3>Select a language</h3>
					<p>Choose a language from the list to view and manage translations</p>
				</div>
			{/if}
		</section>
	</div>

	<!-- Add Locale Modal -->
	{#if showAddLocale}
		<div
			class="modal-overlay"
			onclick={() => (showAddLocale = false)}
			onkeydown={(e) => {
				if (e.key === 'Escape') showAddLocale = false;
			}}
			role="button"
			tabindex="-1"
			aria-label="Close modal"
			transition:fade={{ duration: 200 }}
		></div>
		<div class="modal" in:fly={{ y: 50, duration: 400 }}>
			<div class="modal-header">
				<h2>Add Language</h2>
				<button class="close-btn" onclick={() => (showAddLocale = false)}>
					<IconX size={18} />
				</button>
			</div>

			<div class="modal-body">
				{#if getUnusedLocales().length === 0}
					<div class="empty-state small">
						<IconCheck size={32} />
						<p>All available languages have been added</p>
					</div>
				{:else}
					<div class="locale-options">
						{#each getUnusedLocales() as locale}
							<button class="locale-option" onclick={() => addLocale(locale.code)}>
								<span class="option-flag">{locale.flag}</span>
								<span class="option-name">{locale.name}</span>
								<span class="option-code">{locale.code.toUpperCase()}</span>
								{#if locale.rtl}
									<span class="option-rtl">RTL</span>
								{/if}
							</button>
						{/each}
					</div>
				{/if}
			</div>
		</div>
	{/if}
</div>

<style>
	.locales-page {
		max-width: 1400px;
		margin: 0 auto;
		padding: 0 1.5rem 3rem;
		opacity: 0;
		transform: translateY(10px);
		transition: all 0.5s ease;
	}

	.locales-page.mounted {
		opacity: 1;
		transform: translateY(0);
	}

	/* Header */
	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		padding: 1.5rem 0 2rem;
		flex-wrap: wrap;
		gap: 1.5rem;
	}

	.back-link {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.85rem;
		font-weight: 500;
		color: var(--primary-500);
		text-decoration: none;
		margin-bottom: 1rem;
	}

	.header-title {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.header-icon {
		width: 56px;
		height: 56px;
		border-radius: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, rgba(16, 185, 129, 0.12) 0%, rgba(16, 185, 129, 0.06) 100%);
		color: #059669;
	}

	.header-title h1 {
		font-size: 1.75rem;
		font-weight: 800;
		color: #1e293b;
		margin: 0;
	}

	.header-title p {
		font-size: 0.9rem;
		color: #64748b;
		margin: 0.25rem 0 0 0;
	}

	.btn-add {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.875rem 1.5rem;
		background: linear-gradient(135deg, #10b981 0%, #059669 100%);
		border: none;
		border-radius: 12px;
		font-size: 0.9rem;
		font-weight: 600;
		color: #ffffff;
		cursor: pointer;
		transition: all 0.25s;
	}

	.btn-add:hover {
		transform: translateY(-2px);
		box-shadow: 0 8px 25px rgba(16, 185, 129, 0.3);
	}

	/* Layout */
	.locales-layout {
		display: grid;
		grid-template-columns: 360px 1fr;
		gap: 1.5rem;
	}

	@media (max-width: 1024px) {
		.locales-layout {
			grid-template-columns: 1fr;
		}
	}

	/* Locales Panel */
	.locales-panel {
		background: #ffffff;
		border: 1px solid rgba(0, 0, 0, 0.06);
		border-radius: 20px;
		padding: 1.5rem;
	}

	.panel-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.25rem;
	}

	.panel-header h2 {
		font-size: 1.1rem;
		font-weight: 700;
		color: #1e293b;
		margin: 0;
	}

	.locale-count {
		font-size: 0.8rem;
		font-weight: 500;
		color: #94a3b8;
	}

	.locales-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.locale-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		background: linear-gradient(135deg, rgba(248, 250, 252, 0.9) 0%, rgba(255, 255, 255, 0.7) 100%);
		border: 1px solid rgba(0, 0, 0, 0.05);
		border-radius: 14px;
		cursor: pointer;
		transition: all 0.2s;
		text-align: left;
		width: 100%;
	}

	.locale-item:hover {
		border-color: rgba(16, 185, 129, 0.25);
	}

	.locale-item.selected {
		background: linear-gradient(135deg, rgba(16, 185, 129, 0.08) 0%, rgba(16, 185, 129, 0.04) 100%);
		border-color: rgba(16, 185, 129, 0.3);
	}

	.locale-item.inactive {
		opacity: 0.5;
	}

	.locale-flag {
		font-size: 1.5rem;
	}

	.locale-flag.large {
		font-size: 2rem;
	}

	.locale-info {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 0.1rem;
	}

	.locale-name {
		font-size: 0.9rem;
		font-weight: 600;
		color: #1e293b;
	}

	.locale-code {
		font-size: 0.7rem;
		font-weight: 500;
		color: #94a3b8;
	}

	.locale-progress {
		display: flex;
		flex-direction: column;
		align-items: flex-end;
		gap: 0.25rem;
	}

	.progress-bar {
		width: 60px;
		height: 4px;
		background: #e2e8f0;
		border-radius: 2px;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: linear-gradient(90deg, #10b981 0%, #059669 100%);
		border-radius: 2px;
		transition: width 0.3s ease;
	}

	.progress-text {
		font-size: 0.7rem;
		font-weight: 600;
		color: #059669;
	}

	.locale-actions {
		display: flex;
		gap: 0.25rem;
	}

	.toggle-btn {
		width: 28px;
		height: 28px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #f1f5f9;
		border: none;
		border-radius: 6px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.toggle-btn.active {
		background: rgba(16, 185, 129, 0.1);
		color: #059669;
	}

	.locale-item :global(.chevron) {
		color: #cbd5e1;
	}

	/* Translations Panel */
	.translations-panel {
		background: #ffffff;
		border: 1px solid rgba(0, 0, 0, 0.06);
		border-radius: 20px;
		padding: 1.5rem;
		min-height: 600px;
	}

	.panel-title {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.panel-title h2 {
		font-size: 1.1rem;
		font-weight: 700;
		color: #1e293b;
		margin: 0;
	}

	.translation-count {
		font-size: 0.8rem;
		color: #64748b;
	}

	.panel-actions {
		display: flex;
		gap: 0.75rem;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: #f8fafc;
		border: 1px solid rgba(0, 0, 0, 0.06);
		border-radius: 10px;
		color: #64748b;
	}

	.search-box input {
		border: none;
		background: none;
		font-size: 0.85rem;
		color: #1e293b;
		outline: none;
		width: 180px;
	}

	.translations-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		margin-top: 1.5rem;
	}

	.translation-item {
		padding: 1rem 1.25rem;
		background: linear-gradient(135deg, rgba(248, 250, 252, 0.9) 0%, rgba(255, 255, 255, 0.7) 100%);
		border: 1px solid rgba(0, 0, 0, 0.05);
		border-radius: 14px;
	}

	.translation-key,
	.translation-value {
		margin-bottom: 0.5rem;
	}

	.key-label,
	.value-label {
		display: block;
		font-size: 0.7rem;
		font-weight: 600;
		color: #94a3b8;
		text-transform: uppercase;
		margin-bottom: 0.25rem;
	}

	.key-value {
		font-size: 0.85rem;
		font-weight: 500;
		color: #475569;
		font-family: 'SF Mono', Consolas, monospace;
	}

	.value-text {
		font-size: 0.9rem;
		color: #1e293b;
		line-height: 1.5;
	}

	.translation-meta {
		display: flex;
		gap: 0.5rem;
		align-items: center;
	}

	.meta-type {
		font-size: 0.7rem;
		font-weight: 500;
		color: #94a3b8;
		text-transform: capitalize;
	}

	.meta-badge {
		font-size: 0.6rem;
		font-weight: 700;
		padding: 0.2rem 0.5rem;
		border-radius: 4px;
		text-transform: uppercase;
	}

	.meta-badge.machine {
		background: rgba(245, 158, 11, 0.1);
		color: #b45309;
	}

	/* No Selection & Empty States */
	.no-selection,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem;
		text-align: center;
		color: #94a3b8;
	}

	.no-selection h3,
	.empty-state h3 {
		font-size: 1rem;
		font-weight: 700;
		color: #475569;
		margin: 1rem 0 0.25rem 0;
	}

	.no-selection p,
	.empty-state p {
		font-size: 0.85rem;
		color: #94a3b8;
		margin: 0;
	}

	.loading-state {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 2rem;
	}

	.loading-state.small,
	.empty-state.small {
		padding: 2rem;
	}

	.spinner {
		width: 32px;
		height: 32px;
		border: 3px solid #f1f5f9;
		border-top-color: #10b981;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.4);
		backdrop-filter: blur(4px);
		z-index: 99;
	}

	.modal {
		position: fixed;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 90%;
		max-width: 480px;
		max-height: 80vh;
		background: #ffffff;
		border-radius: 24px;
		box-shadow: 0 25px 80px rgba(0, 0, 0, 0.15);
		z-index: 100;
		overflow: hidden;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem;
		border-bottom: 1px solid rgba(0, 0, 0, 0.06);
	}

	.modal-header h2 {
		font-size: 1.25rem;
		font-weight: 700;
		color: #1e293b;
		margin: 0;
	}

	.close-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #f8fafc;
		border: none;
		border-radius: 10px;
		color: #64748b;
		cursor: pointer;
	}

	.modal-body {
		padding: 1.5rem;
		max-height: calc(80vh - 100px);
		overflow-y: auto;
	}

	.locale-options {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
		gap: 0.75rem;
	}

	.locale-option {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		background: #f8fafc;
		border: 1px solid rgba(0, 0, 0, 0.06);
		border-radius: 12px;
		cursor: pointer;
		transition: all 0.2s;
		text-align: left;
	}

	.locale-option:hover {
		background: rgba(16, 185, 129, 0.08);
		border-color: rgba(16, 185, 129, 0.3);
	}

	.option-flag {
		font-size: 1.5rem;
	}

	.option-name {
		flex: 1;
		font-size: 0.9rem;
		font-weight: 600;
		color: #1e293b;
	}

	.option-code {
		font-size: 0.7rem;
		font-weight: 500;
		color: #94a3b8;
	}

	.option-rtl {
		font-size: 0.6rem;
		font-weight: 700;
		padding: 0.2rem 0.4rem;
		background: rgba(230, 184, 0, 0.1);
		color: var(--primary-600);
		border-radius: 4px;
	}
</style>

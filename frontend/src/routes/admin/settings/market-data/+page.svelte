<script lang="ts">
	/**
	 * Market Data Configuration - Admin Settings
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Allows admins to select which market data provider powers the options
	 * calculator and manage provider connections with live health checks.
	 *
	 * @version 1.0.0
	 * @author Revolution Trading Pros
	 */

	import { fade, fly, slide } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';
	import { adminFetch } from '$lib/utils/adminFetch';
	import {
		Icon,
		IconChartBar,
		IconCheck,
		IconCircleCheck,
		IconCircleX,
		IconLoader2,
		IconPlugConnected,
		IconRefresh,
		IconSettings,
		IconShieldCheck
	} from '$lib/icons';

	// ─── Types ─────────────────────────────────────────────────────────────────
	interface Provider {
		key: string;
		name: string;
		description: string;
		color: string;
		requiresKey: boolean;
	}

	interface ConnectionStatus {
		status: 'connected' | 'disconnected' | 'unknown';
		latency?: number;
		lastChecked?: string;
		error?: string;
	}

	// ─── Provider Definitions ──────────────────────────────────────────────────
	const providers: Provider[] = [
		{
			key: 'polygon',
			name: 'Polygon.io',
			description: 'Real-time and historical stock/options data',
			color: '#7C3AED',
			requiresKey: true
		},
		{
			key: 'tradier',
			name: 'Tradier',
			description: 'Free sandbox, good options chain support',
			color: '#3B82F6',
			requiresKey: true
		},
		{
			key: 'thetadata',
			name: 'ThetaData',
			description: 'Options-focused data with real-time greeks',
			color: '#10B981',
			requiresKey: true
		},
		{
			key: 'yahoo',
			name: 'Yahoo Finance',
			description: 'Free stock quotes (no API key needed)',
			color: '#6001D2',
			requiresKey: false
		}
	];

	// ─── State ─────────────────────────────────────────────────────────────────
	let activeProvider = $state<string>('yahoo');
	let selectedProvider = $state<string>('yahoo');
	let connectionStatuses = $state<Record<string, ConnectionStatus>>({});
	let testingProvider = $state<string | null>(null);
	let isSaving = $state(false);
	let isLoading = $state(true);
	let saveSuccess = $state(false);
	let saveError = $state<string | null>(null);
	let initialLoadDone = $state(false);

	// ─── Derived ───────────────────────────────────────────────────────────────
	let hasChanges = $derived(selectedProvider !== activeProvider);
	let connectedCount = $derived(
		Object.values(connectionStatuses).filter((s) => s.status === 'connected').length
	);

	// ─── Load current configuration ────────────────────────────────────────────
	async function loadConfig() {
		isLoading = true;
		try {
			const response = await adminFetch<{
				data?: { active_provider?: string };
				active_provider?: string;
			}>('/api/admin/options-calculator', { method: 'GET' });
			const provider = response?.data?.active_provider ?? response?.active_provider ?? 'yahoo';
			activeProvider = provider;
			selectedProvider = provider;
		} catch {
			// Default to yahoo if config endpoint not available yet
			activeProvider = 'yahoo';
			selectedProvider = 'yahoo';
		} finally {
			isLoading = false;
			initialLoadDone = true;
		}
	}

	// ─── Test provider connection ──────────────────────────────────────────────
	async function testConnection(providerKey: string) {
		testingProvider = providerKey;
		const startTime = performance.now();

		try {
			const response = await fetch(
				`/tools/options-calculator/api/quote?healthcheck=true&provider=${providerKey}`
			);
			const latency = Math.round(performance.now() - startTime);

			if (response.ok) {
				connectionStatuses[providerKey] = {
					status: 'connected',
					latency,
					lastChecked: new Date().toLocaleTimeString()
				};
			} else {
				const errorText = await response.text().catch(() => 'Unknown error');
				connectionStatuses[providerKey] = {
					status: 'disconnected',
					latency,
					lastChecked: new Date().toLocaleTimeString(),
					error: `HTTP ${response.status}: ${errorText.slice(0, 100)}`
				};
			}
		} catch (err) {
			const latency = Math.round(performance.now() - startTime);
			connectionStatuses[providerKey] = {
				status: 'disconnected',
				latency,
				lastChecked: new Date().toLocaleTimeString(),
				error: err instanceof Error ? err.message : 'Connection failed'
			};
		} finally {
			testingProvider = null;
		}
	}

	// ─── Save configuration ────────────────────────────────────────────────────
	async function saveConfig() {
		isSaving = true;
		saveError = null;
		saveSuccess = false;

		try {
			await adminFetch('/api/admin/options-calculator', {
				method: 'PUT',
				body: JSON.stringify({ active_provider: selectedProvider }),
				headers: { 'Content-Type': 'application/json' }
			});
			activeProvider = selectedProvider;
			saveSuccess = true;
			setTimeout(() => {
				saveSuccess = false;
			}, 3000);
		} catch (err) {
			saveError = err instanceof Error ? err.message : 'Failed to save configuration';
		} finally {
			isSaving = false;
		}
	}

	// ─── Initialize (runs once on mount, ssr=false so safe in browser) ─────────
	loadConfig();

	// ─── Helpers ───────────────────────────────────────────────────────────────
	function getStatusBadge(providerKey: string): {
		label: string;
		cssClass: string;
	} {
		const status = connectionStatuses[providerKey];
		if (!status) return { label: 'Not Tested', cssClass: 'badge-neutral' };
		if (status.status === 'connected') return { label: 'Connected', cssClass: 'badge-success' };
		if (status.status === 'disconnected')
			return { label: 'Disconnected', cssClass: 'badge-danger' };
		return { label: 'Unknown', cssClass: 'badge-neutral' };
	}

	function getProviderInitial(name: string): string {
		return name.charAt(0).toUpperCase();
	}
</script>

<svelte:head>
	<title>Market Data Configuration | Admin | Revolution Trading Pros</title>
</svelte:head>

<div class="market-data-page">
	<!-- Page Header -->
	{#if initialLoadDone}
		<header class="page-header" in:fly={{ y: -20, duration: 400, easing: quintOut }}>
			<div class="header-content">
				<div class="header-icon">
					<Icon icon={IconChartBar} size={28} />
				</div>
				<div class="header-text">
					<h1>Market Data Configuration</h1>
					<p>
						Select which market data provider powers the options calculator and manage provider
						connections.
					</p>
				</div>
			</div>
			<div class="header-stats">
				<div class="stat-chip">
					<Icon icon={IconPlugConnected} size={16} />
					<span>{connectedCount} of {providers.length} tested</span>
				</div>
				<div class="stat-chip active-chip">
					<Icon icon={IconShieldCheck} size={16} />
					<span>Active: {providers.find((p) => p.key === activeProvider)?.name ?? 'None'}</span>
				</div>
			</div>
		</header>
	{/if}

	<!-- Loading State -->
	{#if isLoading && !initialLoadDone}
		<div class="loading-state" in:fade={{ duration: 200 }}>
			<div class="loading-spinner">
				<Icon icon={IconLoader2} size={32} />
			</div>
			<p>Loading configuration...</p>
		</div>
	{/if}

	<!-- Provider Cards Grid -->
	{#if initialLoadDone}
		<div class="providers-grid" in:fade={{ duration: 300, delay: 150 }}>
			{#each providers as provider, i (provider.key)}
				{@const status = getStatusBadge(provider.key)}
				{@const isActive = provider.key === activeProvider}
				{@const isSelected = provider.key === selectedProvider}
				{@const isTesting = testingProvider === provider.key}
				{@const connStatus = connectionStatuses[provider.key]}

				<div
					class="provider-card"
					class:selected={isSelected}
					class:active={isActive}
					in:fly={{ y: 30, duration: 400, delay: i * 80, easing: quintOut }}
				>
					<!-- Active Badge -->
					{#if isActive}
						<div class="active-badge" in:fade={{ duration: 200 }}>
							<Icon icon={IconCheck} size={12} />
							<span>Active</span>
						</div>
					{/if}

					<!-- Card Header -->
					<div class="card-header">
						<div class="provider-icon" style:background={provider.color}>
							{getProviderInitial(provider.name)}
						</div>
						<div class="provider-info">
							<h3>{provider.name}</h3>
							<p>{provider.description}</p>
						</div>
					</div>

					<!-- Connection Status -->
					<div class="card-status">
						<span class="status-badge {status.cssClass}">
							{#if connStatus?.status === 'connected'}
								<Icon icon={IconCircleCheck} size={14} />
							{:else if connStatus?.status === 'disconnected'}
								<Icon icon={IconCircleX} size={14} />
							{/if}
							{status.label}
						</span>
						{#if connStatus?.latency != null}
							<span class="latency" in:fade={{ duration: 200 }}>
								{connStatus.latency}ms
							</span>
						{/if}
					</div>

					<!-- Error Message -->
					{#if connStatus?.error}
						<div class="error-message" transition:slide={{ duration: 250 }}>
							<p>{connStatus.error}</p>
						</div>
					{/if}

					<!-- Last Checked -->
					{#if connStatus?.lastChecked}
						<div class="last-checked" in:fade={{ duration: 200 }}>
							Last tested: {connStatus.lastChecked}
						</div>
					{/if}

					<!-- Card Actions -->
					<div class="card-actions">
						<!-- Radio Select -->
						<label class="radio-select" class:checked={isSelected}>
							<input
								type="radio"
								name="provider"
								value={provider.key}
								checked={isSelected}
								onchange={() => (selectedProvider = provider.key)}
							/>
							<span class="radio-indicator"></span>
							<span class="radio-label">
								{isSelected ? 'Selected' : 'Select as active'}
							</span>
						</label>

						<!-- Test Connection Button -->
						<button
							class="btn-test"
							onclick={() => testConnection(provider.key)}
							disabled={isTesting}
						>
							{#if isTesting}
								<span class="spinner">
									<Icon icon={IconLoader2} size={16} />
								</span>
								Testing...
							{:else}
								<Icon icon={IconRefresh} size={16} />
								Test Connection
							{/if}
						</button>
					</div>

					<!-- Requirement Note -->
					{#if !provider.requiresKey}
						<div class="free-badge">
							<Icon icon={IconCheck} size={12} />
							No API key required
						</div>
					{/if}
				</div>
			{/each}
		</div>
	{/if}

	<!-- Save Bar -->
	{#if initialLoadDone}
		<div
			class="save-bar"
			class:has-changes={hasChanges}
			in:fly={{ y: 20, duration: 400, delay: 400, easing: quintOut }}
		>
			{#if saveSuccess}
				<div class="save-feedback success" in:fade={{ duration: 200 }}>
					<Icon icon={IconCircleCheck} size={18} />
					<span>Configuration saved successfully</span>
				</div>
			{/if}

			{#if saveError}
				<div class="save-feedback error" transition:slide={{ duration: 250 }}>
					<Icon icon={IconCircleX} size={18} />
					<span>{saveError}</span>
				</div>
			{/if}

			<div class="save-actions">
				{#if hasChanges}
					<p class="change-notice" in:fade={{ duration: 200 }}>
						Switching from
						<strong>{providers.find((p) => p.key === activeProvider)?.name}</strong>
						to
						<strong>{providers.find((p) => p.key === selectedProvider)?.name}</strong>
					</p>
				{/if}

				<button
					class="btn-save"
					onclick={saveConfig}
					disabled={!hasChanges || isSaving}
				>
					{#if isSaving}
						<span class="spinner">
							<Icon icon={IconLoader2} size={18} />
						</span>
						Saving...
					{:else}
						<Icon icon={IconSettings} size={18} />
						Save Configuration
					{/if}
				</button>
			</div>
		</div>
	{/if}
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * MARKET DATA CONFIGURATION PAGE
	 * Admin dark theme using scoped CSS custom properties
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.market-data-page {
		max-width: 960px;
		margin: 0 auto;
		padding: 2rem 1.5rem;
	}

	/* ─── Page Header ───────────────────────────────────────────────────────── */

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		gap: 1.5rem;
		margin-bottom: 2rem;
		flex-wrap: wrap;
	}

	.header-content {
		display: flex;
		align-items: flex-start;
		gap: 1rem;
	}

	.header-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		background: linear-gradient(135deg, var(--primary-500, #e6b800), #d4a800);
		display: flex;
		align-items: center;
		justify-content: center;
		color: #0d1117;
		flex-shrink: 0;
	}

	.header-text h1 {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--text-primary, #f0f6fc);
		margin: 0 0 0.25rem 0;
		line-height: 1.3;
	}

	.header-text p {
		font-size: 0.875rem;
		color: var(--text-secondary, #8b949e);
		margin: 0;
		line-height: 1.5;
		max-width: 480px;
	}

	.header-stats {
		display: flex;
		gap: 0.75rem;
		flex-shrink: 0;
	}

	.stat-chip {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.875rem;
		background: var(--bg-surface, #1c2128);
		border: 1px solid rgba(240, 246, 252, 0.08);
		border-radius: 8px;
		font-size: 0.8125rem;
		color: var(--text-secondary, #8b949e);
		white-space: nowrap;
	}

	.stat-chip.active-chip {
		background: rgba(230, 184, 0, 0.1);
		border-color: rgba(230, 184, 0, 0.25);
		color: var(--primary-500, #e6b800);
	}

	/* ─── Loading State ─────────────────────────────────────────────────────── */

	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		gap: 1rem;
	}

	.loading-spinner {
		color: var(--primary-500, #e6b800);
		animation: spin 1s linear infinite;
	}

	.loading-state p {
		color: var(--text-secondary, #8b949e);
		font-size: 0.875rem;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	/* ─── Provider Cards Grid ───────────────────────────────────────────────── */

	.providers-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(400px, 1fr));
		gap: 1.25rem;
		margin-bottom: 2rem;
	}

	.provider-card {
		position: relative;
		background: var(--bg-elevated, #161b22);
		border: 1px solid rgba(240, 246, 252, 0.08);
		border-radius: 12px;
		padding: 1.5rem;
		transition: all 0.25s cubic-bezier(0.16, 1, 0.3, 1);
	}

	.provider-card:hover {
		border-color: rgba(240, 246, 252, 0.15);
		box-shadow: 0 4px 24px rgba(0, 0, 0, 0.2);
	}

	.provider-card.selected {
		border-color: rgba(230, 184, 0, 0.5);
		box-shadow: 0 0 0 1px rgba(230, 184, 0, 0.2);
	}

	.provider-card.active {
		border-color: rgba(230, 184, 0, 0.4);
		background: linear-gradient(
			135deg,
			var(--bg-elevated, #161b22),
			rgba(230, 184, 0, 0.04)
		);
	}

	/* ─── Active Badge ──────────────────────────────────────────────────────── */

	.active-badge {
		position: absolute;
		top: 0.75rem;
		right: 0.75rem;
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.25rem 0.625rem;
		background: rgba(230, 184, 0, 0.15);
		border: 1px solid rgba(230, 184, 0, 0.3);
		border-radius: 6px;
		font-size: 0.6875rem;
		font-weight: 600;
		color: var(--primary-500, #e6b800);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	/* ─── Card Header ───────────────────────────────────────────────────────── */

	.card-header {
		display: flex;
		align-items: flex-start;
		gap: 1rem;
		margin-bottom: 1rem;
	}

	.provider-icon {
		width: 44px;
		height: 44px;
		border-radius: 10px;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 1.25rem;
		font-weight: 700;
		color: white;
		flex-shrink: 0;
	}

	.provider-info h3 {
		font-size: 1rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
		margin: 0 0 0.25rem 0;
	}

	.provider-info p {
		font-size: 0.8125rem;
		color: var(--text-secondary, #8b949e);
		margin: 0;
		line-height: 1.4;
	}

	/* ─── Connection Status ─────────────────────────────────────────────────── */

	.card-status {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 0.75rem;
		padding: 0.625rem 0.75rem;
		background: var(--bg-surface, #1c2128);
		border-radius: 8px;
	}

	.status-badge {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.8125rem;
		font-weight: 500;
	}

	.badge-neutral {
		color: var(--text-secondary, #8b949e);
	}

	.badge-success {
		color: var(--success-500, #3fb950);
	}

	.badge-danger {
		color: var(--danger-500, #f85149);
	}

	.latency {
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--text-secondary, #8b949e);
		background: rgba(240, 246, 252, 0.06);
		padding: 0.2rem 0.5rem;
		border-radius: 4px;
		font-variant-numeric: tabular-nums;
	}

	/* ─── Error Message ─────────────────────────────────────────────────────── */

	.error-message {
		margin-bottom: 0.75rem;
		padding: 0.5rem 0.75rem;
		background: rgba(248, 81, 73, 0.08);
		border: 1px solid rgba(248, 81, 73, 0.2);
		border-radius: 6px;
	}

	.error-message p {
		font-size: 0.75rem;
		color: var(--danger-500, #f85149);
		margin: 0;
		word-break: break-word;
	}

	/* ─── Last Checked ──────────────────────────────────────────────────────── */

	.last-checked {
		font-size: 0.6875rem;
		color: var(--text-secondary, #8b949e);
		margin-bottom: 0.75rem;
		opacity: 0.8;
	}

	/* ─── Card Actions ──────────────────────────────────────────────────────── */

	.card-actions {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 0.75rem;
		padding-top: 0.75rem;
		border-top: 1px solid rgba(240, 246, 252, 0.06);
	}

	/* Radio Select */
	.radio-select {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
		font-size: 0.8125rem;
		color: var(--text-secondary, #8b949e);
		transition: color 0.2s ease;
	}

	.radio-select:hover {
		color: var(--text-primary, #f0f6fc);
	}

	.radio-select.checked {
		color: var(--primary-500, #e6b800);
	}

	.radio-select input[type='radio'] {
		position: absolute;
		opacity: 0;
		width: 0;
		height: 0;
	}

	.radio-indicator {
		width: 18px;
		height: 18px;
		border-radius: 50%;
		border: 2px solid rgba(240, 246, 252, 0.2);
		display: flex;
		align-items: center;
		justify-content: center;
		transition: all 0.2s ease;
		flex-shrink: 0;
	}

	.radio-select.checked .radio-indicator {
		border-color: var(--primary-500, #e6b800);
		background: var(--primary-500, #e6b800);
		box-shadow: inset 0 0 0 3px var(--bg-elevated, #161b22);
	}

	.radio-label {
		font-weight: 500;
	}

	/* Test Connection Button */
	.btn-test {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 0.875rem;
		background: transparent;
		border: 1px solid rgba(240, 246, 252, 0.12);
		border-radius: 8px;
		color: var(--text-secondary, #8b949e);
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
		white-space: nowrap;
	}

	.btn-test:hover:not(:disabled) {
		background: rgba(240, 246, 252, 0.06);
		border-color: rgba(240, 246, 252, 0.2);
		color: var(--text-primary, #f0f6fc);
	}

	.btn-test:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	/* ─── Free Badge ────────────────────────────────────────────────────────── */

	.free-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		margin-top: 0.75rem;
		padding: 0.3rem 0.625rem;
		background: rgba(63, 185, 80, 0.1);
		border: 1px solid rgba(63, 185, 80, 0.2);
		border-radius: 6px;
		font-size: 0.6875rem;
		font-weight: 500;
		color: var(--success-500, #3fb950);
	}

	/* ─── Save Bar ──────────────────────────────────────────────────────────── */

	.save-bar {
		background: var(--bg-elevated, #161b22);
		border: 1px solid rgba(240, 246, 252, 0.08);
		border-radius: 12px;
		padding: 1.25rem 1.5rem;
		transition: all 0.25s ease;
	}

	.save-bar.has-changes {
		border-color: rgba(230, 184, 0, 0.3);
		box-shadow: 0 0 20px rgba(230, 184, 0, 0.05);
	}

	.save-feedback {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 0.875rem;
		border-radius: 8px;
		font-size: 0.8125rem;
		font-weight: 500;
		margin-bottom: 1rem;
	}

	.save-feedback.success {
		background: rgba(63, 185, 80, 0.1);
		border: 1px solid rgba(63, 185, 80, 0.2);
		color: var(--success-500, #3fb950);
	}

	.save-feedback.error {
		background: rgba(248, 81, 73, 0.08);
		border: 1px solid rgba(248, 81, 73, 0.2);
		color: var(--danger-500, #f85149);
	}

	.save-actions {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
	}

	.change-notice {
		font-size: 0.8125rem;
		color: var(--text-secondary, #8b949e);
		margin: 0;
	}

	.change-notice strong {
		color: var(--text-primary, #f0f6fc);
	}

	.btn-save {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1.25rem;
		background: var(--primary-500, #e6b800);
		border: none;
		border-radius: 8px;
		color: #0d1117;
		font-size: 0.875rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
		white-space: nowrap;
		margin-left: auto;
	}

	.btn-save:hover:not(:disabled) {
		background: #ffd11a;
		box-shadow: 0 4px 16px rgba(230, 184, 0, 0.3);
	}

	.btn-save:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* ─── Spinner ───────────────────────────────────────────────────────────── */

	.spinner {
		display: inline-flex;
		animation: spin 1s linear infinite;
	}

	/* ─── Responsive ────────────────────────────────────────────────────────── */

	@media (max-width: 860px) {
		.providers-grid {
			grid-template-columns: 1fr;
		}

		.page-header {
			flex-direction: column;
		}

		.header-stats {
			flex-wrap: wrap;
		}

		.save-actions {
			flex-direction: column;
			align-items: stretch;
		}

		.btn-save {
			margin-left: 0;
			justify-content: center;
		}
	}
</style>

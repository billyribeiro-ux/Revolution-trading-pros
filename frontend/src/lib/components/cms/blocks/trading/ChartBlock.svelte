<!--
/**
 * Chart Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Trading chart display with TradingView embed or static image modes
 * Supports real-time market data visualization with theme synchronization
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */
-->

<script lang="ts">
	import {
		IconChartCandle,
		IconPhoto,
		IconAlertCircle,
		IconLoader2,
		IconRefresh,
		IconExternalLink,
		IconMaximize
	} from '$lib/icons';
	import type { Block, BlockContent, BlockSettings } from '../types';
	import type { BlockId } from '$lib/stores/blockState.svelte';
	import { browser } from '$app/environment';

	// ═══════════════════════════════════════════════════════════════════════════
	// Props Interface
	// ═══════════════════════════════════════════════════════════════════════════

	interface Props {
		block: Block;
		blockId: BlockId;
		isSelected: boolean;
		isEditing: boolean;
		onUpdate: (updates: Partial<Block>) => void;
		onError?: (error: Error) => void;
	}

	let props: Props = $props();

	// ═══════════════════════════════════════════════════════════════════════════
	// Content Interfaces
	// ═══════════════════════════════════════════════════════════════════════════

	interface ChartContent {
		mode: 'embed' | 'image';
		// Embed mode
		symbol: string;
		interval: '1D' | '1W' | '1M' | '3M' | '1Y';
		theme: 'light' | 'dark' | 'auto';
		// Image mode
		imageUrl: string;
		imageAlt: string;
		imageCaption: string;
	}

	interface ChartSettings {
		height: '300px' | '400px' | '500px' | '600px';
		showToolbar: boolean;
		allowFullscreen: boolean;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// State Management
	// ═══════════════════════════════════════════════════════════════════════════

	let isLoading = $state(true);
	let hasError = $state(false);
	let errorMessage = $state('');
	let isFullscreen = $state(false);
	let systemTheme = $state<'light' | 'dark'>('light');
	let iframeRef = $state<HTMLIFrameElement | null>(null);
	let imageLoaded = $state(false);

	// ═══════════════════════════════════════════════════════════════════════════
	// Derived State
	// ═══════════════════════════════════════════════════════════════════════════

	let mode = $derived<'embed' | 'image'>((props.block.content.chartMode as 'embed' | 'image') || 'embed');
	let symbol = $derived(props.block.content.chartSymbol || 'NASDAQ:AAPL');
	let interval = $derived((props.block.content.chartInterval as ChartContent['interval']) || '1D');
	let themePreference = $derived((props.block.content.chartTheme as ChartContent['theme']) || 'auto');
	let imageUrl = $derived(props.block.content.chartImageUrl || '');
	let imageAlt = $derived(props.block.content.chartImageAlt || 'Trading chart');
	let imageCaption = $derived(props.block.content.chartImageCaption || '');
	let height = $derived((props.block.settings.chartHeight as ChartSettings['height']) || '400px');
	let showToolbar = $derived(props.block.settings.chartShowToolbar !== false);
	let allowFullscreen = $derived(props.block.settings.chartAllowFullscreen !== false);

	// Resolve actual theme based on preference
	let resolvedTheme = $derived(
		themePreference === 'auto' ? systemTheme : themePreference
	);

	// TradingView interval mapping
	let tvInterval = $derived(() => {
		const intervalMap: Record<string, string> = {
			'1D': 'D',
			'1W': 'W',
			'1M': 'M',
			'3M': '3M',
			'1Y': '12M'
		};
		return intervalMap[interval] || 'D';
	});

	// Construct TradingView embed URL
	let embedUrl = $derived(
		mode === 'embed' && symbol
			? `https://s.tradingview.com/widgetembed/?symbol=${encodeURIComponent(symbol)}&interval=${tvInterval()}&theme=${resolvedTheme}&style=1&toolbar_bg=${resolvedTheme === 'dark' ? '1e293b' : 'f8fafc'}&hide_side_toolbar=0&allow_symbol_change=1&save_image=0&studies=[]&show_popup_button=1&popup_width=1000&popup_height=650`
			: ''
	);

	// Height value for styling
	let heightValue = $derived(parseInt(height) || 400);

	// ═══════════════════════════════════════════════════════════════════════════
	// Interval Options
	// ═══════════════════════════════════════════════════════════════════════════

	const intervalOptions: { value: ChartContent['interval']; label: string }[] = [
		{ value: '1D', label: '1 Day' },
		{ value: '1W', label: '1 Week' },
		{ value: '1M', label: '1 Month' },
		{ value: '3M', label: '3 Months' },
		{ value: '1Y', label: '1 Year' }
	];

	const heightOptions: { value: ChartSettings['height']; label: string }[] = [
		{ value: '300px', label: 'Small (300px)' },
		{ value: '400px', label: 'Medium (400px)' },
		{ value: '500px', label: 'Large (500px)' },
		{ value: '600px', label: 'Extra Large (600px)' }
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// Lifecycle & Theme Detection
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		if (!browser) return;

		// Detect system theme
		const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
		systemTheme = mediaQuery.matches ? 'dark' : 'light';

		// Also check for page dark mode class
		if (document.documentElement.classList.contains('dark')) {
			systemTheme = 'dark';
		}

		const handleThemeChange = (e: MediaQueryListEvent) => {
			systemTheme = e.matches ? 'dark' : 'light';
		};

		// Observe class changes on document for theme switching
		const observer = new MutationObserver(() => {
			systemTheme = document.documentElement.classList.contains('dark') ? 'dark' : 'light';
		});

		observer.observe(document.documentElement, {
			attributes: true,
			attributeFilter: ['class']
		});

		mediaQuery.addEventListener('change', handleThemeChange);

		return () => {
			mediaQuery.removeEventListener('change', handleThemeChange);
			observer.disconnect();
		};
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// Update Functions
	// ═══════════════════════════════════════════════════════════════════════════

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	function updateSettings(updates: Partial<BlockSettings>): void {
		props.onUpdate({ settings: { ...props.block.settings, ...updates } });
	}

	function handleModeChange(newMode: 'embed' | 'image'): void {
		updateContent({ chartMode: newMode });
		resetLoadingState();
	}

	function handleSymbolChange(e: Event): void {
		const input = e.target as HTMLInputElement;
		const value = input.value.toUpperCase().trim();
		updateContent({ chartSymbol: value });
		resetLoadingState();
	}

	function handleIntervalChange(e: Event): void {
		const select = e.target as HTMLSelectElement;
		updateContent({ chartInterval: select.value });
		resetLoadingState();
	}

	function handleThemeChange(e: Event): void {
		const select = e.target as HTMLSelectElement;
		updateContent({ chartTheme: select.value as 'light' | 'dark' | 'auto' });
		resetLoadingState();
	}

	function handleHeightChange(e: Event): void {
		const select = e.target as HTMLSelectElement;
		updateSettings({ chartHeight: select.value });
	}

	function handleImageUrlChange(e: Event): void {
		const input = e.target as HTMLInputElement;
		updateContent({ chartImageUrl: input.value });
		imageLoaded = false;
		hasError = false;
	}

	function handleImageAltChange(e: Event): void {
		const input = e.target as HTMLInputElement;
		updateContent({ chartImageAlt: input.value });
	}

	function handleImageCaptionChange(e: Event): void {
		const input = e.target as HTMLInputElement;
		updateContent({ chartImageCaption: input.value });
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Loading & Error Handling
	// ═══════════════════════════════════════════════════════════════════════════

	function resetLoadingState(): void {
		isLoading = true;
		hasError = false;
		errorMessage = '';
	}

	function handleIframeLoad(): void {
		isLoading = false;
		hasError = false;
	}

	function handleIframeError(): void {
		isLoading = false;
		hasError = true;
		errorMessage = 'Failed to load chart. Please check the symbol and try again.';
		if (props.onError) {
			props.onError(new Error(errorMessage));
		}
	}

	function handleImageLoad(): void {
		imageLoaded = true;
		isLoading = false;
		hasError = false;
	}

	function handleImageError(): void {
		imageLoaded = false;
		isLoading = false;
		hasError = true;
		errorMessage = 'Failed to load image. Please check the URL and try again.';
		if (props.onError) {
			props.onError(new Error(errorMessage));
		}
	}

	function retryLoad(): void {
		resetLoadingState();
		// Force iframe/image reload by toggling a key
		if (mode === 'embed' && iframeRef) {
			iframeRef.src = embedUrl;
		}
	}

	function toggleFullscreen(): void {
		isFullscreen = !isFullscreen;
	}

	function openInNewTab(): void {
		if (mode === 'embed' && symbol) {
			window.open(`https://www.tradingview.com/chart/?symbol=${encodeURIComponent(symbol)}`, '_blank');
		} else if (mode === 'image' && imageUrl) {
			window.open(imageUrl, '_blank');
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Validation
	// ═══════════════════════════════════════════════════════════════════════════

	let isValidSymbol = $derived(
		symbol.length > 0 && /^[A-Z0-9:._-]+$/i.test(symbol)
	);

	let isValidImageUrl = $derived(
		imageUrl.length === 0 || /^https?:\/\/.+/i.test(imageUrl)
	);

	// Generate aria-label for the chart
	let chartAriaLabel = $derived(
		mode === 'embed'
			? `Trading chart for ${symbol}, ${interval} interval`
			: imageAlt || 'Trading chart image'
	);
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════ -->
<!-- Template -->
<!-- ═══════════════════════════════════════════════════════════════════════════ -->

<div
	class="chart-block"
	class:fullscreen={isFullscreen}
	class:theme-dark={resolvedTheme === 'dark'}
	role="img"
	aria-label={chartAriaLabel}
>
	<!-- Header with controls -->
	{#if props.isEditing && props.isSelected}
		<div class="chart-settings">
			<div class="settings-row">
				<div class="mode-toggle">
					<button
						type="button"
						class="mode-btn"
						class:active={mode === 'embed'}
						onclick={() => handleModeChange('embed')}
						aria-pressed={mode === 'embed'}
					>
						<IconChartCandle size={16} aria-hidden="true" />
						<span>TradingView</span>
					</button>
					<button
						type="button"
						class="mode-btn"
						class:active={mode === 'image'}
						onclick={() => handleModeChange('image')}
						aria-pressed={mode === 'image'}
					>
						<IconPhoto size={16} aria-hidden="true" />
						<span>Image</span>
					</button>
				</div>

				<div class="settings-group">
					<label class="setting-field">
						<span>Height:</span>
						<select value={height} onchange={handleHeightChange}>
							{#each heightOptions as option}
								<option value={option.value}>{option.label}</option>
							{/each}
						</select>
					</label>
				</div>
			</div>

			{#if mode === 'embed'}
				<div class="settings-row embed-settings">
					<label class="setting-field symbol-field">
						<span>Symbol:</span>
						<input
							type="text"
							value={symbol}
							oninput={handleSymbolChange}
							placeholder="NASDAQ:AAPL"
							class:invalid={!isValidSymbol && symbol.length > 0}
							aria-invalid={!isValidSymbol && symbol.length > 0}
						/>
					</label>

					<label class="setting-field">
						<span>Interval:</span>
						<select value={interval} onchange={handleIntervalChange}>
							{#each intervalOptions as option}
								<option value={option.value}>{option.label}</option>
							{/each}
						</select>
					</label>

					<label class="setting-field">
						<span>Theme:</span>
						<select value={themePreference} onchange={handleThemeChange}>
							<option value="auto">Auto (sync)</option>
							<option value="light">Light</option>
							<option value="dark">Dark</option>
						</select>
					</label>
				</div>

				<div class="symbol-examples">
					<span class="examples-label">Examples:</span>
					<button type="button" onclick={() => updateContent({ chartSymbol: 'NASDAQ:AAPL' })}>NASDAQ:AAPL</button>
					<button type="button" onclick={() => updateContent({ chartSymbol: 'NYSE:SPY' })}>NYSE:SPY</button>
					<button type="button" onclick={() => updateContent({ chartSymbol: 'BINANCE:BTCUSDT' })}>BINANCE:BTCUSDT</button>
					<button type="button" onclick={() => updateContent({ chartSymbol: 'FOREXCOM:EURUSD' })}>FOREXCOM:EURUSD</button>
				</div>
			{:else}
				<div class="settings-row image-settings">
					<label class="setting-field image-url-field">
						<span>Image URL:</span>
						<input
							type="url"
							value={imageUrl}
							oninput={handleImageUrlChange}
							placeholder="https://example.com/chart.png"
							class:invalid={!isValidImageUrl && imageUrl.length > 0}
							aria-invalid={!isValidImageUrl && imageUrl.length > 0}
						/>
					</label>
				</div>

				<div class="settings-row">
					<label class="setting-field">
						<span>Alt Text:</span>
						<input
							type="text"
							value={imageAlt}
							oninput={handleImageAltChange}
							placeholder="Describe the chart for accessibility"
						/>
					</label>

					<label class="setting-field">
						<span>Caption:</span>
						<input
							type="text"
							value={imageCaption}
							oninput={handleImageCaptionChange}
							placeholder="Optional caption"
						/>
					</label>
				</div>
			{/if}
		</div>
	{/if}

	<!-- Chart Display Area -->
	<div class="chart-container" style="height: {heightValue}px;">
		{#if mode === 'embed'}
			{#if !isValidSymbol}
				<div class="chart-placeholder">
					<IconChartCandle size={48} aria-hidden="true" />
					<p>Enter a valid trading symbol</p>
					<span class="hint">Format: EXCHANGE:SYMBOL (e.g., NASDAQ:AAPL)</span>
				</div>
			{:else}
				{#if isLoading}
					<div class="chart-loading" role="status" aria-live="polite">
						<IconLoader2 size={32} class="spinner" aria-hidden="true" />
						<p>Loading chart...</p>
					</div>
				{/if}

				{#if hasError}
					<div class="chart-error" role="alert">
						<IconAlertCircle size={32} aria-hidden="true" />
						<p>{errorMessage}</p>
						<button type="button" class="retry-btn" onclick={retryLoad}>
							<IconRefresh size={16} aria-hidden="true" />
							<span>Retry</span>
						</button>
					</div>
				{:else}
					<iframe
						bind:this={iframeRef}
						src={embedUrl}
						title={chartAriaLabel}
						class="chart-iframe"
						class:loading={isLoading}
						onload={handleIframeLoad}
						onerror={handleIframeError}
						allow="fullscreen"
						sandbox="allow-scripts allow-same-origin allow-popups"
					></iframe>
				{/if}
			{/if}
		{:else}
			{#if !imageUrl}
				<div class="chart-placeholder">
					<IconPhoto size={48} aria-hidden="true" />
					<p>Enter an image URL</p>
					<span class="hint">Supports PNG, JPG, WebP, and GIF formats</span>
				</div>
			{:else if !isValidImageUrl}
				<div class="chart-error" role="alert">
					<IconAlertCircle size={32} aria-hidden="true" />
					<p>Please enter a valid URL starting with http:// or https://</p>
				</div>
			{:else}
				{#if isLoading && !imageLoaded}
					<div class="chart-loading" role="status" aria-live="polite">
						<IconLoader2 size={32} class="spinner" aria-hidden="true" />
						<p>Loading image...</p>
					</div>
				{/if}

				{#if hasError}
					<div class="chart-error" role="alert">
						<IconAlertCircle size={32} aria-hidden="true" />
						<p>{errorMessage}</p>
						<button type="button" class="retry-btn" onclick={retryLoad}>
							<IconRefresh size={16} aria-hidden="true" />
							<span>Retry</span>
						</button>
					</div>
				{:else}
					<img
						src={imageUrl}
						alt={imageAlt}
						class="chart-image"
						class:loading={!imageLoaded}
						onload={handleImageLoad}
						onerror={handleImageError}
					/>
				{/if}
			{/if}
		{/if}

		<!-- Toolbar overlay -->
		{#if !props.isEditing && showToolbar && !hasError && ((mode === 'embed' && isValidSymbol) || (mode === 'image' && imageUrl && isValidImageUrl))}
			<div class="chart-toolbar">
				{#if allowFullscreen}
					<button
						type="button"
						class="toolbar-btn"
						onclick={toggleFullscreen}
						aria-label={isFullscreen ? 'Exit fullscreen' : 'Enter fullscreen'}
					>
						<IconMaximize size={18} aria-hidden="true" />
					</button>
				{/if}
				<button
					type="button"
					class="toolbar-btn"
					onclick={openInNewTab}
					aria-label="Open in new tab"
				>
					<IconExternalLink size={18} aria-hidden="true" />
				</button>
				{#if mode === 'embed'}
					<div class="toolbar-info">
						<span class="symbol-badge">{symbol}</span>
						<span class="interval-badge">{interval}</span>
					</div>
				{/if}
			</div>
		{/if}
	</div>

	<!-- Caption for image mode -->
	{#if mode === 'image' && imageCaption && !props.isEditing}
		<div class="chart-caption">
			<p>{imageCaption}</p>
		</div>
	{/if}
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════ -->
<!-- Styles -->
<!-- ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	.chart-block {
		position: relative;
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		overflow: hidden;
		background: #ffffff;
		transition: all 0.3s ease;
	}

	.chart-block.fullscreen {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		z-index: 9999;
		border-radius: 0;
	}

	.chart-block.fullscreen .chart-container {
		height: 100vh !important;
	}

	/* Settings Panel */
	.chart-settings {
		padding: 1rem;
		background: #f8fafc;
		border-bottom: 1px solid #e5e7eb;
	}

	.settings-row {
		display: flex;
		flex-wrap: wrap;
		gap: 1rem;
		align-items: flex-end;
		margin-bottom: 0.75rem;
	}

	.settings-row:last-child {
		margin-bottom: 0;
	}

	.mode-toggle {
		display: flex;
		background: #e5e7eb;
		border-radius: 8px;
		padding: 4px;
	}

	.mode-btn {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 0.875rem;
		background: transparent;
		border: none;
		border-radius: 6px;
		font-size: 0.8125rem;
		font-weight: 500;
		color: #64748b;
		cursor: pointer;
		transition: all 0.15s;
	}

	.mode-btn:hover {
		color: #334155;
	}

	.mode-btn.active {
		background: #ffffff;
		color: #0f172a;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.settings-group {
		display: flex;
		gap: 0.75rem;
		margin-left: auto;
	}

	.setting-field {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
		font-size: 0.8125rem;
	}

	.setting-field span {
		color: #64748b;
		font-weight: 500;
	}

	.setting-field input,
	.setting-field select {
		padding: 0.5rem 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 6px;
		font-size: 0.875rem;
		background: #ffffff;
		min-width: 120px;
	}

	.setting-field input:focus,
	.setting-field select:focus {
		outline: none;
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.setting-field input.invalid {
		border-color: #ef4444;
	}

	.symbol-field input {
		min-width: 180px;
		text-transform: uppercase;
		font-family: monospace;
	}

	.image-url-field {
		flex: 1;
		min-width: 300px;
	}

	.image-url-field input {
		width: 100%;
	}

	.symbol-examples {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 0.5rem;
		padding-top: 0.5rem;
		border-top: 1px solid #e5e7eb;
		margin-top: 0.75rem;
	}

	.examples-label {
		font-size: 0.75rem;
		color: #94a3b8;
		font-weight: 500;
	}

	.symbol-examples button {
		padding: 0.25rem 0.5rem;
		background: #e0f2fe;
		border: none;
		border-radius: 4px;
		font-size: 0.75rem;
		font-family: monospace;
		color: #0369a1;
		cursor: pointer;
		transition: all 0.15s;
	}

	.symbol-examples button:hover {
		background: #bae6fd;
	}

	/* Chart Container */
	.chart-container {
		position: relative;
		width: 100%;
		background: #0f172a;
		overflow: hidden;
	}

	.chart-iframe {
		width: 100%;
		height: 100%;
		border: none;
		transition: opacity 0.3s;
	}

	.chart-iframe.loading {
		opacity: 0;
	}

	.chart-image {
		width: 100%;
		height: 100%;
		object-fit: contain;
		background: #f8fafc;
		transition: opacity 0.3s;
	}

	.chart-image.loading {
		opacity: 0;
	}

	/* Loading State */
	.chart-loading {
		position: absolute;
		inset: 0;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		background: rgba(15, 23, 42, 0.95);
		color: #94a3b8;
		z-index: 10;
	}

	.chart-loading :global(.spinner) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}

	/* Error State */
	.chart-error {
		position: absolute;
		inset: 0;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		background: linear-gradient(135deg, #450a0a 0%, #1e1b4b 100%);
		color: #fca5a5;
		padding: 2rem;
		text-align: center;
	}

	.chart-error p {
		margin: 0;
		max-width: 300px;
	}

	.retry-btn {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 1rem;
		background: rgba(255, 255, 255, 0.1);
		border: 1px solid rgba(255, 255, 255, 0.2);
		border-radius: 6px;
		color: #f8fafc;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.retry-btn:hover {
		background: rgba(255, 255, 255, 0.15);
	}

	/* Placeholder */
	.chart-placeholder {
		position: absolute;
		inset: 0;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%);
		color: #64748b;
		text-align: center;
		padding: 2rem;
	}

	.chart-placeholder p {
		margin: 0;
		font-size: 1rem;
		color: #94a3b8;
	}

	.chart-placeholder .hint {
		font-size: 0.8125rem;
		color: #64748b;
	}

	/* Toolbar */
	.chart-toolbar {
		position: absolute;
		top: 0.75rem;
		right: 0.75rem;
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem;
		background: rgba(15, 23, 42, 0.8);
		backdrop-filter: blur(8px);
		border-radius: 8px;
		opacity: 0;
		transform: translateY(-4px);
		transition: all 0.2s;
		z-index: 20;
	}

	.chart-container:hover .chart-toolbar {
		opacity: 1;
		transform: translateY(0);
	}

	.toolbar-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		background: rgba(255, 255, 255, 0.1);
		border: none;
		border-radius: 6px;
		color: #f8fafc;
		cursor: pointer;
		transition: all 0.15s;
	}

	.toolbar-btn:hover {
		background: rgba(255, 255, 255, 0.2);
	}

	.toolbar-info {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		margin-left: 0.5rem;
		padding-left: 0.75rem;
		border-left: 1px solid rgba(255, 255, 255, 0.2);
	}

	.symbol-badge {
		padding: 0.25rem 0.5rem;
		background: #3b82f6;
		border-radius: 4px;
		font-size: 0.75rem;
		font-weight: 600;
		color: #ffffff;
		font-family: monospace;
	}

	.interval-badge {
		padding: 0.25rem 0.375rem;
		background: rgba(255, 255, 255, 0.15);
		border-radius: 4px;
		font-size: 0.6875rem;
		font-weight: 500;
		color: #cbd5e1;
	}

	/* Caption */
	.chart-caption {
		padding: 0.75rem 1rem;
		background: #f8fafc;
		border-top: 1px solid #e5e7eb;
	}

	.chart-caption p {
		margin: 0;
		font-size: 0.8125rem;
		color: #64748b;
		text-align: center;
		font-style: italic;
	}

	/* Dark Mode */
	:global(.dark) .chart-block {
		background: #0f172a;
		border-color: #334155;
	}

	:global(.dark) .chart-settings {
		background: #1e293b;
		border-color: #334155;
	}

	:global(.dark) .mode-toggle {
		background: #334155;
	}

	:global(.dark) .mode-btn {
		color: #94a3b8;
	}

	:global(.dark) .mode-btn:hover {
		color: #e2e8f0;
	}

	:global(.dark) .mode-btn.active {
		background: #475569;
		color: #f8fafc;
	}

	:global(.dark) .setting-field span {
		color: #94a3b8;
	}

	:global(.dark) .setting-field input,
	:global(.dark) .setting-field select {
		background: #0f172a;
		border-color: #475569;
		color: #e2e8f0;
	}

	:global(.dark) .symbol-examples {
		border-color: #334155;
	}

	:global(.dark) .symbol-examples button {
		background: #1e3a5f;
		color: #7dd3fc;
	}

	:global(.dark) .symbol-examples button:hover {
		background: #1e4976;
	}

	:global(.dark) .chart-image {
		background: #1e293b;
	}

	:global(.dark) .chart-caption {
		background: #1e293b;
		border-color: #334155;
	}

	:global(.dark) .chart-caption p {
		color: #94a3b8;
	}
</style>

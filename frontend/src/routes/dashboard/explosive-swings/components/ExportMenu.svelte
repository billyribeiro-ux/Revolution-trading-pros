<script lang="ts">
	/**
	 * ExportMenu - Explosive Swings Export Dropdown
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Apple Principal Engineer ICT Level 7 Grade - January 2026
	 *
	 * Features:
	 * - Server-side CSV export for alerts and trades
	 * - Client-side PDF generation for performance reports
	 * - Date range filtering with presets
	 * - Loading states with progress indication
	 * - Error handling with toast notifications
	 *
	 * ARCHITECTURE:
	 * - Uses Svelte 5 runes ($state, $derived, $effect)
	 * - Integrates with server-side export API
	 * - Supports both light and dark themes
	 */

	import { fly, fade } from 'svelte/transition';
	import {
		downloadAlertsCsv,
		downloadTradesCsv,
		downloadPerformanceReportPdf,
		ExportError,
		type AlertExportFilters,
		type TradeExportFilters,
		type ReportDateRange
	} from '$lib/utils/export';

	// ═══════════════════════════════════════════════════════════════════════════
	// PROPS
	// ═══════════════════════════════════════════════════════════════════════════

	interface Props {
		/** Trading room slug (e.g., 'explosive-swings') */
		roomSlug?: string;
		/** Optional date range filter */
		dateRange?: { start?: string; end?: string };
		/** Size variant */
		size?: 'sm' | 'md' | 'lg';
		/** Disabled state */
		disabled?: boolean;
		/** Callback when export completes */
		onexport?: (detail: { type: string; success: boolean }) => void;
		/** Callback when export fails */
		onerror?: (error: ExportError) => void;
	}

	let {
		roomSlug = 'explosive-swings',
		dateRange = {},
		size = 'md',
		disabled = false,
		onexport,
		onerror
	}: Props = $props();

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let isOpen = $state(false);
	let isExporting = $state(false);
	let exportType = $state<'alerts' | 'trades' | 'report' | null>(null);
	let showFilters = $state(false);
	let buttonRef: HTMLButtonElement | undefined = $state();

	// Filter state - initialized empty, synced via $effect below
	let alertFilters = $state<AlertExportFilters>({});
	let tradeFilters = $state<TradeExportFilters>({});
	let reportRange = $state<ReportDateRange>({});

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED
	// ═══════════════════════════════════════════════════════════════════════════

	const sizeClasses = $derived(
		({
			sm: 'px-2.5 py-1.5 text-xs gap-1.5',
			md: 'px-4 py-2 text-sm gap-2',
			lg: 'px-5 py-2.5 text-base gap-2.5'
		})[size]
	);

	const iconSize = $derived(
		({
			sm: 14,
			md: 16,
			lg: 18
		})[size]
	);

	// ═══════════════════════════════════════════════════════════════════════════
	// HANDLERS
	// ═══════════════════════════════════════════════════════════════════════════

	function toggle() {
		if (!disabled) {
			isOpen = !isOpen;
			showFilters = false;
		}
	}

	function close() {
		isOpen = false;
		showFilters = false;
	}

	function handleClickOutside(event: MouseEvent) {
		if (isOpen && buttonRef && !buttonRef.contains(event.target as Node)) {
			const dropdown = document.getElementById('export-dropdown');
			if (dropdown && !dropdown.contains(event.target as Node)) {
				close();
			}
		}
	}

	async function handleExport(type: 'alerts' | 'trades' | 'report') {
		if (isExporting) return;

		isExporting = true;
		exportType = type;

		try {
			switch (type) {
				case 'alerts':
					await downloadAlertsCsv(roomSlug, alertFilters);
					break;
				case 'trades':
					await downloadTradesCsv(roomSlug, tradeFilters);
					break;
				case 'report':
					await downloadPerformanceReportPdf(roomSlug, reportRange);
					break;
			}

			onexport?.({ type, success: true });
		} catch (error) {
			const exportError =
				error instanceof ExportError ? error : new ExportError('Export failed unexpectedly');

			onerror?.(exportError);
			onexport?.({ type, success: false });
		} finally {
			isExporting = false;
			exportType = null;
			close();
		}
	}

	function setDatePreset(preset: 'week' | 'month' | 'quarter' | 'year' | 'all') {
		const now = new Date();
		let startDate: string | undefined;
		const endDate = now.toISOString().split('T')[0];

		switch (preset) {
			case 'week':
				startDate = new Date(now.getTime() - 7 * 24 * 60 * 60 * 1000).toISOString().split('T')[0];
				break;
			case 'month':
				startDate = new Date(now.getTime() - 30 * 24 * 60 * 60 * 1000).toISOString().split('T')[0];
				break;
			case 'quarter':
				startDate = new Date(now.getTime() - 90 * 24 * 60 * 60 * 1000).toISOString().split('T')[0];
				break;
			case 'year':
				startDate = new Date(now.getTime() - 365 * 24 * 60 * 60 * 1000)
					.toISOString()
					.split('T')[0];
				break;
			case 'all':
				startDate = undefined;
				break;
		}

		alertFilters = { ...alertFilters, startDate, endDate };
		tradeFilters = { ...tradeFilters, startDate, endDate };
		reportRange = { startDate, endDate };
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// EFFECTS
	// ═══════════════════════════════════════════════════════════════════════════

	// Sync date range prop with internal state (runs on mount and when dateRange changes)
	$effect(() => {
		const start = dateRange?.start;
		const end = dateRange?.end;
		alertFilters = { ...alertFilters, startDate: start, endDate: end };
		tradeFilters = { ...tradeFilters, startDate: start, endDate: end };
		reportRange = { startDate: start, endDate: end };
	});
</script>

<svelte:window onclick={handleClickOutside} />

<div class="export-menu" class:is-open={isOpen}>
	<!-- Trigger Button -->
	<button
		bind:this={buttonRef}
		onclick={toggle}
		class="export-trigger {sizeClasses}"
		disabled={disabled || isExporting}
		aria-expanded={isOpen}
		aria-haspopup="menu"
	>
		{#if isExporting}
			<svg
				class="animate-spin"
				width={iconSize}
				height={iconSize}
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
			>
				<path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83" />
			</svg>
		{:else}
			<svg
				width={iconSize}
				height={iconSize}
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
				<polyline points="7 10 12 15 17 10" />
				<line x1="12" y1="15" x2="12" y2="3" />
			</svg>
		{/if}
		<span>Export</span>
		<svg
			class="chevron"
			class:rotate={isOpen}
			width="12"
			height="12"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2.5"
		>
			<path d="M6 9l6 6 6-6" />
		</svg>
	</button>

	<!-- Dropdown Menu -->
	{#if isOpen}
		<div
			id="export-dropdown"
			class="export-dropdown"
			role="menu"
			transition:fly={{ y: -8, duration: 150 }}
		>
			<!-- Quick Presets -->
			<div class="preset-row">
				<button class="preset-btn" onclick={() => setDatePreset('week')}>7D</button>
				<button class="preset-btn" onclick={() => setDatePreset('month')}>30D</button>
				<button class="preset-btn" onclick={() => setDatePreset('quarter')}>90D</button>
				<button class="preset-btn" onclick={() => setDatePreset('year')}>1Y</button>
				<button class="preset-btn" onclick={() => setDatePreset('all')}>All</button>
			</div>

			<div class="menu-divider"></div>

			<!-- Export Options -->
			<button
				class="menu-item"
				role="menuitem"
				onclick={() => handleExport('alerts')}
				disabled={isExporting}
			>
				<svg
					class="item-icon csv"
					width="18"
					height="18"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
					<polyline points="14 2 14 8 20 8" />
					<line x1="16" y1="13" x2="8" y2="13" />
					<line x1="16" y1="17" x2="8" y2="17" />
				</svg>
				<span class="item-label">
					{#if isExporting && exportType === 'alerts'}
						Exporting...
					{:else}
						Alerts (CSV)
					{/if}
				</span>
			</button>

			<button
				class="menu-item"
				role="menuitem"
				onclick={() => handleExport('trades')}
				disabled={isExporting}
			>
				<svg
					class="item-icon csv"
					width="18"
					height="18"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
					<polyline points="14 2 14 8 20 8" />
					<line x1="16" y1="13" x2="8" y2="13" />
					<line x1="16" y1="17" x2="8" y2="17" />
				</svg>
				<span class="item-label">
					{#if isExporting && exportType === 'trades'}
						Exporting...
					{:else}
						Trades (CSV)
					{/if}
				</span>
			</button>

			<div class="menu-divider"></div>

			<button
				class="menu-item featured"
				role="menuitem"
				onclick={() => handleExport('report')}
				disabled={isExporting}
			>
				<svg
					class="item-icon pdf"
					width="18"
					height="18"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
					<polyline points="14 2 14 8 20 8" />
					<path d="M9 13h6" />
					<path d="M9 17h6" />
					<path d="M9 9h1" />
				</svg>
				<span class="item-label">
					{#if isExporting && exportType === 'report'}
						Generating...
					{:else}
						Performance Report (PDF)
					{/if}
				</span>
			</button>

			<!-- Footer with date info -->
			<div class="menu-footer">
				{#if alertFilters.startDate}
					<span class="date-info">
						{alertFilters.startDate} to {alertFilters.endDate || 'Today'}
					</span>
				{:else}
					<span class="date-info">All time</span>
				{/if}
			</div>
		</div>
	{/if}
</div>

<style>
	.export-menu {
		position: relative;
		display: inline-block;
	}

	.export-trigger {
		display: inline-flex;
		align-items: center;
		font-weight: 500;
		border-radius: 8px;
		background: var(--color-bg-elevated, #f1f5f9);
		border: 1px solid var(--color-border-default, #e2e8f0);
		color: var(--color-text-primary, #1e293b);
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.export-trigger:hover:not(:disabled) {
		background: var(--color-bg-hover, #e2e8f0);
		border-color: var(--color-border-hover, #cbd5e1);
	}

	.export-trigger:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.chevron {
		transition: transform 0.2s ease;
	}

	.chevron.rotate {
		transform: rotate(180deg);
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	.animate-spin {
		animation: spin 1s linear infinite;
	}

	.export-dropdown {
		position: absolute;
		top: calc(100% + 8px);
		right: 0;
		min-width: 220px;
		background: var(--color-bg-card, #ffffff);
		border: 1px solid var(--color-border-default, #e2e8f0);
		border-radius: 12px;
		box-shadow:
			0 10px 40px -10px rgba(0, 0, 0, 0.15),
			0 4px 12px -4px rgba(0, 0, 0, 0.1);
		z-index: 50;
		overflow: hidden;
	}

	.preset-row {
		display: flex;
		gap: 4px;
		padding: 10px 12px;
		background: var(--color-bg-subtle, #f8fafc);
	}

	.preset-btn {
		flex: 1;
		padding: 6px 8px;
		font-size: 11px;
		font-weight: 600;
		color: var(--color-text-secondary, #64748b);
		background: var(--color-bg-card, #ffffff);
		border: 1px solid var(--color-border-default, #e2e8f0);
		border-radius: 6px;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.preset-btn:hover {
		background: var(--color-brand-primary, #6366f1);
		color: white;
		border-color: var(--color-brand-primary, #6366f1);
	}

	.menu-divider {
		height: 1px;
		background: var(--color-border-default, #e2e8f0);
	}

	.menu-item {
		display: flex;
		align-items: center;
		gap: 12px;
		width: 100%;
		padding: 12px 16px;
		font-size: 14px;
		color: var(--color-text-primary, #1e293b);
		background: transparent;
		border: none;
		cursor: pointer;
		transition: background 0.15s ease;
		text-align: left;
	}

	.menu-item:hover:not(:disabled) {
		background: var(--color-bg-hover, #f1f5f9);
	}

	.menu-item:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.menu-item.featured {
		background: var(--color-bg-subtle, #f8fafc);
	}

	.menu-item.featured:hover:not(:disabled) {
		background: var(--color-bg-hover, #f1f5f9);
	}

	.item-icon {
		flex-shrink: 0;
	}

	.item-icon.csv {
		color: #10b981;
	}

	.item-icon.pdf {
		color: #ef4444;
	}

	.item-label {
		flex: 1;
	}

	.menu-footer {
		padding: 8px 16px;
		background: var(--color-bg-subtle, #f8fafc);
		border-top: 1px solid var(--color-border-default, #e2e8f0);
	}

	.date-info {
		font-size: 11px;
		color: var(--color-text-tertiary, #94a3b8);
	}

	/* Dark mode overrides */
	:global(.dark) .export-trigger {
		background: var(--color-bg-elevated, #334155);
		border-color: var(--color-border-default, #475569);
		color: var(--color-text-primary, #f1f5f9);
	}

	:global(.dark) .export-trigger:hover:not(:disabled) {
		background: var(--color-bg-hover, #475569);
	}

	:global(.dark) .export-dropdown {
		background: var(--color-bg-card, #1e293b);
		border-color: var(--color-border-default, #475569);
	}

	:global(.dark) .preset-row {
		background: var(--color-bg-subtle, #0f172a);
	}

	:global(.dark) .preset-btn {
		background: var(--color-bg-card, #1e293b);
		color: var(--color-text-secondary, #94a3b8);
		border-color: var(--color-border-default, #475569);
	}

	:global(.dark) .menu-item {
		color: var(--color-text-primary, #f1f5f9);
	}

	:global(.dark) .menu-item:hover:not(:disabled) {
		background: var(--color-bg-hover, #334155);
	}

	:global(.dark) .menu-item.featured {
		background: var(--color-bg-subtle, #0f172a);
	}

	:global(.dark) .menu-footer {
		background: var(--color-bg-subtle, #0f172a);
		border-color: var(--color-border-default, #475569);
	}
</style>

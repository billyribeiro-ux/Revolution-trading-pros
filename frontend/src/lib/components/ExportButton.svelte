<script lang="ts">
	/**
	 * Export Button Component - Apple ICT9+ Design
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * Provides export functionality for dashboard data with:
	 * - Multiple export formats (CSV, JSON, PDF)
	 * - Apple-level polish and animations
	 * - Dropdown menu for format selection
	 * - Progress indication during export
	 */

	import { fade, scale } from 'svelte/transition';
	import {
		IconDownload,
		IconFileTypeCsv,
		IconFileTypeJson,
		IconFileTypePdf,
		IconChevronDown,
		IconCheck,
		IconLoader2
	} from '@tabler/icons-svelte';
	import { toastStore } from '$lib/stores/toast';

	interface Props {
		/** Data to export (array of objects) */
		data: Record<string, unknown>[];
		/** Filename without extension */
		filename?: string;
		/** Available formats */
		formats?: ('csv' | 'json' | 'pdf')[];
		/** Button label */
		label?: string;
		/** Whether the button is disabled */
		disabled?: boolean;
		/** Custom export handler for PDF */
		onPdfExport?: () => Promise<void>;
	}

	let {
		data,
		filename = 'export',
		formats = ['csv', 'json'],
		label = 'Export',
		disabled = false,
		onPdfExport
	}: Props = $props();

	let isOpen = $state(false);
	let isExporting = $state(false);
	let exportedFormat = $state<string | null>(null);

	function toggleDropdown() {
		if (!disabled && !isExporting) {
			isOpen = !isOpen;
		}
	}

	function closeDropdown() {
		isOpen = false;
	}

	async function exportData(format: 'csv' | 'json' | 'pdf') {
		if (isExporting || !data || data.length === 0) {
			if (!data || data.length === 0) {
				toastStore.warning('No data to export');
			}
			return;
		}

		isExporting = true;
		exportedFormat = null;

		try {
			switch (format) {
				case 'csv':
					await exportToCsv();
					break;
				case 'json':
					await exportToJson();
					break;
				case 'pdf':
					if (onPdfExport) {
						await onPdfExport();
					} else {
						toastStore.warning('PDF export not configured');
					}
					break;
			}
			exportedFormat = format;
			toastStore.success(`Exported successfully as ${format.toUpperCase()}`);
		} catch (error) {
			toastStore.error(`Export failed: ${error instanceof Error ? error.message : 'Unknown error'}`);
		} finally {
			isExporting = false;
			setTimeout(() => {
				exportedFormat = null;
				closeDropdown();
			}, 1500);
		}
	}

	async function exportToCsv() {
		if (!data || data.length === 0) return;

		// Get headers from first object
		const headers = Object.keys(data[0]);

		// Build CSV content
		const csvContent = [
			headers.join(','),
			...data.map(row =>
				headers.map(header => {
					const value = row[header];
					// Handle different types
					if (value === null || value === undefined) return '';
					if (typeof value === 'string') {
						// Escape quotes and wrap in quotes if contains comma
						const escaped = value.replace(/"/g, '""');
						return value.includes(',') || value.includes('"') || value.includes('\n')
							? `"${escaped}"`
							: escaped;
					}
					if (typeof value === 'object') return JSON.stringify(value);
					return String(value);
				}).join(',')
			)
		].join('\n');

		// Download
		downloadFile(csvContent, `${filename}.csv`, 'text/csv');
	}

	async function exportToJson() {
		if (!data || data.length === 0) return;

		const jsonContent = JSON.stringify(data, null, 2);
		downloadFile(jsonContent, `${filename}.json`, 'application/json');
	}

	function downloadFile(content: string, filename: string, mimeType: string) {
		const blob = new Blob([content], { type: mimeType });
		const url = URL.createObjectURL(blob);
		const link = document.createElement('a');
		link.href = url;
		link.download = filename;
		document.body.appendChild(link);
		link.click();
		document.body.removeChild(link);
		URL.revokeObjectURL(url);
	}

	function getFormatIcon(format: string) {
		switch (format) {
			case 'csv': return IconFileTypeCsv;
			case 'json': return IconFileTypeJson;
			case 'pdf': return IconFileTypePdf;
			default: return IconDownload;
		}
	}

	function getFormatLabel(format: string) {
		switch (format) {
			case 'csv': return 'CSV Spreadsheet';
			case 'json': return 'JSON Data';
			case 'pdf': return 'PDF Report';
			default: return format.toUpperCase();
		}
	}
</script>

<svelte:window on:click={closeDropdown} />

<div class="export-container">
	<button
		class="export-btn"
		class:disabled
		class:open={isOpen}
		onclick={(e) => { e.stopPropagation(); toggleDropdown(); }}
		{disabled}
	>
		{#if isExporting}
			<IconLoader2 size={18} class="spinning" />
		{:else}
			<IconDownload size={18} />
		{/if}
		<span>{label}</span>
		<IconChevronDown size={16} class="chevron" class:rotated={isOpen} />
	</button>

	{#if isOpen}
		<div
			class="dropdown"
			in:scale={{ duration: 200, start: 0.95 }}
			out:fade={{ duration: 150 }}
			onclick={(e) => e.stopPropagation()}
			role="menu"
		>
			{#each formats as format}
				<button
					class="dropdown-item"
					class:success={exportedFormat === format}
					onclick={() => exportData(format)}
					disabled={isExporting}
					role="menuitem"
				>
					{#if exportedFormat === format}
						<IconCheck size={18} class="success-icon" />
					{:else}
						<svelte:component this={getFormatIcon(format)} size={18} />
					{/if}
					<span>{getFormatLabel(format)}</span>
				</button>
			{/each}
		</div>
	{/if}
</div>

<style>
	.export-container {
		position: relative;
		display: inline-block;
	}

	.export-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 10px;
		color: #a5b4fc;
		font-size: 0.875rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.export-btn:hover:not(.disabled) {
		background: rgba(99, 102, 241, 0.2);
		border-color: rgba(99, 102, 241, 0.4);
	}

	.export-btn.open {
		background: rgba(99, 102, 241, 0.2);
		border-color: rgba(99, 102, 241, 0.4);
	}

	.export-btn.disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.chevron {
		transition: transform 0.2s ease;
	}

	.chevron.rotated {
		transform: rotate(180deg);
	}

	:global(.spinning) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}

	.dropdown {
		position: absolute;
		top: calc(100% + 0.5rem);
		right: 0;
		min-width: 180px;
		background: linear-gradient(135deg, rgba(30, 41, 59, 0.98), rgba(15, 23, 42, 0.98));
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 12px;
		padding: 0.5rem;
		box-shadow: 0 10px 40px rgba(0, 0, 0, 0.4);
		backdrop-filter: blur(20px);
		z-index: 100;
	}

	.dropdown-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		width: 100%;
		padding: 0.75rem 1rem;
		background: transparent;
		border: none;
		border-radius: 8px;
		color: #cbd5e1;
		font-size: 0.875rem;
		text-align: left;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.dropdown-item:hover:not(:disabled) {
		background: rgba(99, 102, 241, 0.15);
		color: #f1f5f9;
	}

	.dropdown-item:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.dropdown-item.success {
		color: #4ade80;
		background: rgba(34, 197, 94, 0.1);
	}

	:global(.success-icon) {
		color: #4ade80;
	}
</style>

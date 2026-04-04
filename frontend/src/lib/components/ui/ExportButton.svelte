<script lang="ts">
	/**
	 * ExportButton - Enterprise Data Export Component
	 * Dropdown button for exporting data in multiple formats
	 *
	 * @version 1.0.0
	 * @author Revolution Trading Pros
	 * @level L8 Principal Engineer
	 */
	import { fly } from 'svelte/transition';
	import {
		exportToCSV,
		exportToPDF,
		exportToJSON,
		type ExportColumn,
		type ExportOptions
	} from '$lib/utils/export';
	import { Icon, IconCsv, IconDownload, IconJson, IconPdf } from '$lib/icons';

	interface Props {
		data?: Record<string, unknown>[];
		columns?: ExportColumn[];
		filename?: string;
		title?: string;
		subtitle?: string;
		disabled?: boolean;
		size?: 'sm' | 'md' | 'lg';
		onexport?: (detail: { format: string; count: number }) => void;
	}

	let props: Props = $props();
	let data = $derived(props.data ?? []);
	let columns = $derived(props.columns ?? []);
	let filename = $derived(props.filename ?? 'export');
	let title = $derived(props.title ?? '');
	let subtitle = $derived(props.subtitle ?? '');
	let disabled = $derived(props.disabled ?? false);
	let size = $derived(props.size ?? 'md');
	let onexport = $derived(props.onexport);

	let isOpen = $state(false);
	let buttonRef: HTMLButtonElement;


	function toggle() {
		if (!disabled && data.length > 0) {
			isOpen = !isOpen;
		}
	}

	function close() {
		isOpen = false;
	}

	function handleExport(format: 'csv' | 'pdf' | 'json') {
		const options: ExportOptions = {
			filename,
			columns,
			data,
			title: title || filename.replace(/_/g, ' '),
			subtitle,
			includeTimestamp: true
		};

		switch (format) {
			case 'csv':
				exportToCSV(options);
				break;
			case 'pdf':
				exportToPDF(options);
				break;
			case 'json':
				exportToJSON(options);
				break;
		}

		onexport?.({ format, count: data.length });
		close();
	}

	function handleClickOutside(event: MouseEvent) {
		if (isOpen && buttonRef && !buttonRef.contains(event.target as Node)) {
			close();
		}
	}
</script>

<svelte:window onclick={handleClickOutside} />

<div class="export-wrapper">
	<button
		bind:this={buttonRef}
		onclick={toggle}
		class="export-trigger"
		data-size={size}
		disabled={disabled || data.length === 0}
	>
		<Icon icon={IconDownload} size={size === 'sm' ? 14 : size === 'md' ? 16 : 18} />
		<span>Export</span>
		{#if data.length > 0}
			<span class="export-count">({data.length})</span>
		{/if}
		<svg
			class="export-chevron"
			class:export-chevron-open={isOpen}
			fill="none"
			stroke="currentColor"
			viewBox="0 0 24 24"
		>
			<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
		</svg>
	</button>

	{#if isOpen}
		<div transition:fly={{ y: -5, duration: 150 }} class="export-dropdown">
			<div class="export-options">
				<button onclick={() => handleExport('csv')} class="export-option">
					<Icon icon={IconCsv} size={18} class="export-icon-csv" />
					<span>Export as CSV</span>
				</button>

				<button onclick={() => handleExport('pdf')} class="export-option">
					<Icon icon={IconPdf} size={18} class="export-icon-pdf" />
					<span>Export as PDF</span>
				</button>

				<button onclick={() => handleExport('json')} class="export-option">
					<Icon icon={IconJson} size={18} class="export-icon-json" />
					<span>Export as JSON</span>
				</button>
			</div>

			<div class="export-footer">
				<p>{data.length} records to export</p>
			</div>
		</div>
	{/if}
</div>

<style>
	.export-wrapper {
		position: relative;
		display: inline-block;
	}

	.export-trigger {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		font-weight: var(--weight-medium);
		border-radius: var(--radius-lg);
		background-color: oklch(0.3 0.01 250 / 50%);
		border: 1px solid oklch(0.4 0.01 250 / 50%);
		color: oklch(0.7 0.01 250);
		transition: all var(--duration-fast) var(--ease-default);

		&[data-size='sm'] { padding-inline: 0.625rem; padding-block: 0.375rem; font-size: var(--text-xs); }
		&[data-size='md'] { padding-inline: var(--space-4); padding-block: var(--space-2); font-size: var(--text-sm); }
		&[data-size='lg'] { padding-inline: 1.25rem; padding-block: 0.625rem; font-size: var(--text-base); }

		&:hover:not(:disabled) {
			background-color: oklch(0.3 0.01 250);
			color: oklch(1 0 0);
		}

		&:disabled {
			opacity: 0.5;
			cursor: not-allowed;
		}
	}

	.export-count {
		color: oklch(0.55 0.01 250);
	}

	.export-chevron {
		inline-size: 1rem;
		block-size: 1rem;
		transition: transform var(--duration-fast) var(--ease-default);
	}

	.export-chevron-open {
		transform: rotate(180deg);
	}

	.export-dropdown {
		position: absolute;
		inset-inline-end: 0;
		margin-block-start: var(--space-2);
		inline-size: 12rem;
		background-color: oklch(0.25 0.01 250);
		border: 1px solid oklch(0.38 0.01 250 / 50%);
		border-radius: var(--radius-xl);
		box-shadow: var(--shadow-xl);
		overflow: hidden;
		z-index: 50;
	}

	.export-options {
		padding-block: var(--space-1);
	}

	.export-option {
		inline-size: 100%;
		display: flex;
		align-items: center;
		gap: var(--space-3);
		padding-inline: var(--space-4);
		padding-block: 0.625rem;
		font-size: var(--text-sm);
		color: oklch(0.7 0.01 250);
		background: none;
		border: none;
		cursor: pointer;
		transition: all var(--duration-fast) var(--ease-default);

		&:hover {
			background-color: oklch(0.3 0.01 250 / 50%);
			color: oklch(1 0 0);
		}
	}

	.export-footer {
		padding-inline: var(--space-4);
		padding-block: var(--space-2);
		border-block-start: 1px solid oklch(0.38 0.01 250 / 50%);
		background-color: oklch(0.25 0.01 250 / 50%);

		& p {
			font-size: var(--text-xs);
			color: oklch(0.55 0.01 250);
		}
	}

	:global(.export-icon-csv) { color: oklch(0.7 0.18 160); }
	:global(.export-icon-pdf) { color: oklch(0.7 0.18 25); }
	:global(.export-icon-json) { color: oklch(0.7 0.18 260); }
</style>

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
	import { IconDownload, IconCsv, IconPdf, IconJson } from '$lib/icons';
	import {
		exportToCSV,
		exportToPDF,
		exportToJSON,
		type ExportColumn,
		type ExportOptions
	} from '$lib/utils/export';

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

	let {
		data = [],
		columns = [],
		filename = 'export',
		title = '',
		subtitle = '',
		disabled = false,
		size = 'md',
		onexport
	}: Props = $props();

	let isOpen = $state(false);
	let buttonRef: HTMLButtonElement;

	const sizeClasses = {
		sm: 'px-2.5 py-1.5 text-xs',
		md: 'px-4 py-2 text-sm',
		lg: 'px-5 py-2.5 text-base'
	};

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

<div class="relative inline-block">
	<button
		bind:this={buttonRef}
		onclick={toggle}
		class="flex items-center gap-2 {sizeClasses[size]} font-medium rounded-lg
			bg-slate-700/50 hover:bg-slate-700 border border-slate-600/50
			text-slate-300 hover:text-white transition-all duration-200
			disabled:opacity-50 disabled:cursor-not-allowed"
		disabled={disabled || data.length === 0}
	>
		<IconDownload size={size === 'sm' ? 14 : size === 'md' ? 16 : 18} />
		<span>Export</span>
		{#if data.length > 0}
			<span class="text-slate-500">({data.length})</span>
		{/if}
		<svg
			class="w-4 h-4 transition-transform duration-200"
			class:rotate-180={isOpen}
			fill="none"
			stroke="currentColor"
			viewBox="0 0 24 24"
		>
			<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
		</svg>
	</button>

	{#if isOpen}
		<div
			transition:fly={{ y: -5, duration: 150 }}
			class="absolute right-0 mt-2 w-48 bg-slate-800 border border-slate-700/50
				rounded-xl shadow-xl overflow-hidden z-50"
		>
			<div class="py-1">
				<button
					onclick={() => handleExport('csv')}
					class="w-full flex items-center gap-3 px-4 py-2.5 text-sm text-slate-300
						hover:bg-slate-700/50 hover:text-white transition-colors"
				>
					<IconCsv size={18} class="text-emerald-400" />
					<span>Export as CSV</span>
				</button>

				<button
					onclick={() => handleExport('pdf')}
					class="w-full flex items-center gap-3 px-4 py-2.5 text-sm text-slate-300
						hover:bg-slate-700/50 hover:text-white transition-colors"
				>
					<IconPdf size={18} class="text-red-400" />
					<span>Export as PDF</span>
				</button>

				<button
					onclick={() => handleExport('json')}
					class="w-full flex items-center gap-3 px-4 py-2.5 text-sm text-slate-300
						hover:bg-slate-700/50 hover:text-white transition-colors"
				>
					<IconJson size={18} class="text-blue-400" />
					<span>Export as JSON</span>
				</button>
			</div>

			<div class="px-4 py-2 border-t border-slate-700/50 bg-slate-800/50">
				<p class="text-xs text-slate-500">{data.length} records to export</p>
			</div>
		</div>
	{/if}
</div>

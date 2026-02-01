<!--
	Support Documentation Section Component
	═══════════════════════════════════════════════════════════════════════════
	Displays supporting documentation links for indicators
	Used across all indicator detail pages
-->
<script lang="ts">
	import DownloadButton from './DownloadButton.svelte';

	interface SupportDoc {
		title: string;
		url: string;
	}

	interface Props {
		docs: SupportDoc[];
	}

	let { docs }: Props = $props();
</script>

<div class="st_box">
	<h2>
		<strong>Supporting Documentation</strong>
	</h2>
	<table>
		<tbody>
			{#each docs as doc}
				<tr>
					<td>{doc.title}</td>
					<td class="text-right">
						<DownloadButton
							href={doc.url}
							text="Click to View"
							target="_blank"
							rel="noopener noreferrer"
						/>
					</td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * Support Documentation Section - 2026 Mobile-First Responsive Design
	 * Features: responsive tables, touch-friendly targets, safe areas
	 * Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px), xl(1280px)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Mobile-first base styles */
	.st_box {
		border: 12px solid #f4f4f4;
		padding: 16px;
		padding-left: max(16px, env(safe-area-inset-left));
		padding-right: max(16px, env(safe-area-inset-right));
		margin-top: 20px;
		background: #ffffff;
		border-radius: 8px;
	}

	.st_box h2 {
		margin-bottom: 16px;
		font-weight: 700;
		color: #666666;
		font-family: 'Open Sans', sans-serif;
		/* Responsive typography */
		font-size: clamp(16px, 4vw, 20px);
		line-height: 1.3;
	}

	/* Table wrapper for horizontal scroll on mobile */
	table {
		width: 100%;
		margin: 0;
		border: 0;
		border-collapse: collapse;
		/* Mobile: stack cells vertically */
		display: block;
	}

	table tbody {
		display: block;
		width: 100%;
	}

	table tr {
		display: flex;
		flex-direction: column;
		padding: 12px 0;
		border-top: 1px solid #f4f4f4;
	}

	table tr:first-child {
		border-top: none;
	}

	table td {
		border: 0;
		padding: 6px 0;
		color: #666666;
		font-family: 'Open Sans', sans-serif;
		/* Responsive font size */
		font-size: clamp(14px, 3.5vw, 16px);
		display: block;
		width: 100%;
	}

	table td.text-right {
		text-align: left; /* Left align on mobile for stacked layout */
		padding-top: 10px;
	}

	/* sm: Small devices (≥ 640px) - horizontal layout */
	@media (min-width: 640px) {
		.st_box {
			border-width: 16px;
			padding: 20px;
		}

		table {
			display: table;
		}

		table tbody {
			display: table-row-group;
		}

		table tr {
			display: table-row;
			flex-direction: row;
		}

		table td {
			display: table-cell;
			padding: 16px 8px;
			vertical-align: middle;
		}

		table td:first-child {
			padding-left: 0;
		}

		table td.text-right {
			text-align: right;
			padding-right: 0;
		}
	}

	/* md: Medium devices (≥ 768px) */
	@media (min-width: 768px) {
		.st_box {
			border-width: 20px;
			margin-top: 30px;
		}

		.st_box h2 {
			margin-bottom: 20px;
		}

		table td {
			padding: 20px 0;
		}
	}

	/* lg: Large devices (≥ 1024px) */
	@media (min-width: 1024px) {
		.st_box {
			border-width: 24px;
		}
	}

	/* Touch device optimizations */
	@media (hover: none) and (pointer: coarse) {
		table tr {
			padding: 14px 0;
		}

		table td {
			padding: 8px 0;
		}
	}

	/* Reduced motion preference */
	@media (prefers-reduced-motion: reduce) {
		.st_box,
		table td {
			transition: none;
		}
	}

	/* High contrast mode */
	@media (prefers-contrast: high) {
		.st_box {
			border-color: #333;
		}

		table tr {
			border-top-color: #333;
		}
	}
</style>

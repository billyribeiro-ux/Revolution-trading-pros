<!--
	Platform Downloads Component
	═══════════════════════════════════════════════════════════════════════════
	Displays platform-specific download files and instructions
	Used across all indicator detail pages
-->
<script lang="ts">
	import DownloadButton from './DownloadButton.svelte';

	interface DownloadFile {
		name: string;
		downloadUrl: string;
	}

	interface Props {
		platform: string;
		logo: string;
		files: DownloadFile[];
		notes?: string;
	}

	let { platform, logo, files, notes }: Props = $props();
</script>

<div class="st_box {platform.toLowerCase()}">
	<img width="250" src={logo} alt={platform} />

	{#if files.length > 0}
		<table>
			<tbody>
				<tr>
					<th>{platform} Install File:</th>
					<th></th>
				</tr>
				{#each files as file}
					<tr>
						<td>{file.name}</td>
						<td class="text-right">
							<DownloadButton href={file.downloadUrl} />
						</td>
					</tr>
				{/each}
			</tbody>
		</table>
	{/if}

	{#if notes}
		<div class="platform_notes">
			{@html notes}
		</div>
	{/if}
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * Platform Downloads - 2026 Mobile-First Responsive Design
	 * Features: horizontal scroll on mobile, touch-friendly targets
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

	.st_box img {
		margin-bottom: 16px;
		max-width: 100%;
		width: auto;
		max-height: 60px;
		height: auto;
		object-fit: contain;
	}

	/* Table wrapper for horizontal scroll on mobile */
	.st_box :global(.table-wrapper) {
		width: 100%;
		overflow-x: auto;
		-webkit-overflow-scrolling: touch;
		margin-bottom: 16px;
	}

	table {
		width: 100%;
		min-width: 320px;
		margin: 0;
		border: 0;
		border-collapse: collapse;
	}

	table th {
		border: 0;
		padding: 10px 12px;
		text-align: left;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		color: #666666;
		/* Responsive font size */
		font-size: clamp(13px, 3vw, 15px);
		white-space: nowrap;
	}

	table th:first-child {
		padding-left: 0;
	}

	table td {
		border: 0;
		padding: 16px 8px;
		border-top: 1px solid #f4f4f4;
		color: #666666;
		font-family: 'Open Sans', sans-serif;
		/* Responsive font size */
		font-size: clamp(13px, 3vw, 15px);
		vertical-align: middle;
	}

	table td:first-child {
		padding-left: 0;
		/* Allow file name to wrap on mobile */
		word-break: break-word;
		min-width: 120px;
	}

	table td.text-right {
		text-align: right;
		padding-right: 0;
	}

	.platform_notes {
		padding-top: 16px;
		border-top: 1px solid #f4f4f4;
		color: #666666;
		line-height: 1.7;
		font-family: 'Open Sans', sans-serif;
		/* Responsive font size */
		font-size: clamp(14px, 3.5vw, 16px);
	}

	.platform_notes :global(a) {
		color: #1e73be;
		text-decoration: none;
		/* Touch-friendly links */
		display: inline-block;
		padding: 2px 0;
	}

	.platform_notes :global(a:hover) {
		color: #000000;
		text-decoration: underline;
	}

	/* xs: Extra small devices (≥ 360px) */
	@media (min-width: 360px) {
		.st_box {
			padding: 16px;
		}
	}

	/* sm: Small devices (≥ 640px) */
	@media (min-width: 640px) {
		.st_box {
			border-width: 16px;
			padding: 20px;
		}

		.st_box img {
			max-height: 80px;
		}

		table th {
			padding: 12px 15px;
		}

		table td {
			padding: 18px 10px;
		}
	}

	/* md: Medium devices (≥ 768px) */
	@media (min-width: 768px) {
		.st_box {
			border-width: 20px;
			padding: 20px;
			margin-top: 30px;
		}

		.st_box img {
			margin-bottom: 20px;
			max-height: 100px;
		}

		table {
			min-width: auto;
		}

		table td {
			padding: 20px 0;
		}

		.platform_notes {
			padding-top: 20px;
		}
	}

	/* lg: Large devices (≥ 1024px) */
	@media (min-width: 1024px) {
		.st_box {
			border-width: 24px;
		}

		.st_box img {
			max-width: 250px;
		}
	}

	/* Touch device optimizations */
	@media (hover: none) and (pointer: coarse) {
		table td {
			/* More padding for touch */
			padding: 18px 8px;
		}

		/* Scroll indicators for touch */
		.st_box :global(.table-wrapper) {
			position: relative;
		}

		.st_box :global(.table-wrapper)::after {
			content: '';
			position: absolute;
			right: 0;
			top: 0;
			bottom: 0;
			width: 20px;
			background: linear-gradient(to left, rgba(255, 255, 255, 0.8), transparent);
			pointer-events: none;
			opacity: 0;
			transition: opacity 0.3s;
		}

		.st_box :global(.table-wrapper.can-scroll)::after {
			opacity: 1;
		}
	}

	/* Reduced motion preference */
	@media (prefers-reduced-motion: reduce) {
		.st_box :global(.table-wrapper) {
			scroll-behavior: auto;
		}
	}

	/* High contrast mode */
	@media (prefers-contrast: high) {
		.st_box {
			border-color: #333;
		}

		table td {
			border-top-color: #333;
		}
	}
</style>

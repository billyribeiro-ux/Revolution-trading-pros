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
	.st_box {
		border: 24px solid #f4f4f4;
		padding: 20px;
		margin-top: 30px;
		background: #ffffff;
	}

	.st_box img {
		margin-bottom: 20px;
		max-width: 100%;
		height: auto;
	}

	table {
		width: 100%;
		margin: 0;
		border: 0;
		border-collapse: collapse;
	}

	table th {
		border: 0;
		padding: 12px 15px;
		text-align: left;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		color: #666666;
	}

	table th:first-child {
		padding-left: 0;
	}

	table td {
		border: 0;
		padding: 20px 0;
		border-top: 1px solid #f4f4f4;
		color: #666666;
		font-family: 'Open Sans', sans-serif;
	}

	table td.text-right {
		text-align: right;
	}

	.platform_notes {
		padding-top: 20px;
		border-top: 1px solid #f4f4f4;
		color: #666666;
		line-height: 1.7;
		font-family: 'Open Sans', sans-serif;
	}

	.platform_notes :global(a) {
		color: #1e73be;
		text-decoration: none;
	}

	.platform_notes :global(a:hover) {
		color: #000000;
		text-decoration: underline;
	}
</style>

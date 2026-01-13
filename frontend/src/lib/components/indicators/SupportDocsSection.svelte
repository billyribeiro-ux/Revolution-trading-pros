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
	.st_box {
		border: 24px solid #f4f4f4;
		padding: 20px;
		margin-top: 30px;
		background: #ffffff;
	}

	.st_box h2 {
		margin-bottom: 20px;
		font-weight: 700;
		color: #666666;
		font-family: 'Open Sans', sans-serif;
	}

	table {
		width: 100%;
		margin: 0;
		border: 0;
		border-collapse: collapse;
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
</style>

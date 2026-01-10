<script lang="ts">
	/**
	 * Class Downloads Preview Component
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 * 
	 * Wrapper for the ClassDownloads component in the page builder.
	 */

	import type { ClassDownloadsConfig } from '../../types';
	import ClassDownloads from '$lib/components/ClassDownloads.svelte';

	interface Props {
		config: ClassDownloadsConfig;
		isPreview: boolean;
	}

	let { config, isPreview }: Props = $props();
</script>

<div class="downloads-preview">
	{#if isPreview && (config.courseId || config.courseSlug)}
		<ClassDownloads 
			courseId={config.courseId}
			courseSlug={config.courseSlug}
			title={config.title}
			maxHeight={config.maxHeight}
		/>
	{:else}
		<div class="downloads-placeholder">
			<div class="placeholder-header">
				<h3>{config.title || 'Class Downloads'}</h3>
			</div>
			<div class="placeholder-body">
				<div class="file-row">
					<span class="file-icon">📄</span>
					<span class="file-name">Course_Materials.pdf</span>
					<span class="file-size">2.4 MB</span>
				</div>
				<div class="file-row alt">
					<span class="file-icon">📊</span>
					<span class="file-name">Trading_Spreadsheet.xlsx</span>
					<span class="file-size">156 KB</span>
				</div>
				<div class="file-row">
					<span class="file-icon">📝</span>
					<span class="file-name">Notes_Template.docx</span>
					<span class="file-size">84 KB</span>
				</div>
				<div class="file-note">
					<em>Files will load from course when published</em>
				</div>
			</div>
		</div>
	{/if}
</div>

<style>
	.downloads-preview {
		border-radius: 4px;
		overflow: hidden;
	}

	.downloads-placeholder {
		background: #FFFFFF;
		border: 1px solid #E5E7EB;
		border-radius: 4px;
	}

	.placeholder-header {
		padding: 16px 20px;
		border-bottom: 1px solid #E0E0E0;
	}

	.placeholder-header h3 {
		margin: 0;
		font-size: 1.25rem;
		font-weight: 600;
		color: #333333;
	}

	.placeholder-body {
		padding: 0;
	}

	.file-row {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 12px 20px;
		border-bottom: 1px solid #F0F0F0;
	}

	.file-row:last-of-type {
		border-bottom: none;
	}

	.file-row.alt {
		background: #F9F9F9;
	}

	.file-icon {
		font-size: 18px;
	}

	.file-name {
		flex: 1;
		font-size: 14px;
		color: #333333;
	}

	.file-size {
		font-size: 13px;
		color: #666666;
	}

	.file-note {
		padding: 12px 20px;
		background: #F9FAFB;
		border-top: 1px solid #E5E7EB;
		text-align: center;
		font-size: 13px;
		color: #6B7280;
	}
</style>

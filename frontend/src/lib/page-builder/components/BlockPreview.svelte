<script lang="ts">
	/**
	 * Block Preview - Renders Component Based on Type
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 * 
	 * Dynamically renders the appropriate component preview based on block type.
	 */

	import type { PageBlock, CourseHeaderConfig, VideoPlayerConfig, VideoStackConfig, ClassDownloadsConfig, SpacerConfig, DividerConfig } from '../types';
	
	// Preview components
	import CourseHeaderPreview from './previews/CourseHeaderPreview.svelte';
	import VideoPlayerPreview from './previews/VideoPlayerPreview.svelte';
	import VideoStackPreview from './previews/VideoStackPreview.svelte';
	import ClassDownloadsPreview from './previews/ClassDownloadsPreview.svelte';
	import SpacerPreview from './previews/SpacerPreview.svelte';
	import DividerPreview from './previews/DividerPreview.svelte';

	interface Props {
		block: PageBlock;
		isPreview: boolean;
	}

	let { block, isPreview }: Props = $props();
</script>

{#if block.type === 'course-header'}
	<CourseHeaderPreview config={block.config as CourseHeaderConfig} {isPreview} />
{:else if block.type === 'video-player'}
	<VideoPlayerPreview config={block.config as VideoPlayerConfig} {isPreview} />
{:else if block.type === 'video-stack'}
	<VideoStackPreview config={block.config as VideoStackConfig} {isPreview} />
{:else if block.type === 'class-downloads'}
	<ClassDownloadsPreview config={block.config as ClassDownloadsConfig} {isPreview} />
{:else if block.type === 'spacer'}
	<SpacerPreview config={block.config as SpacerConfig} {isPreview} />
{:else if block.type === 'divider'}
	<DividerPreview config={block.config as DividerConfig} {isPreview} />
{:else}
	<div class="unknown-block">
		<p>Unknown block type: {block.type}</p>
	</div>
{/if}

<style>
	.unknown-block {
		padding: 20px;
		background: #FEF3C7;
		border: 1px dashed #F59E0B;
		border-radius: 8px;
		text-align: center;
		color: #92400E;
	}
</style>

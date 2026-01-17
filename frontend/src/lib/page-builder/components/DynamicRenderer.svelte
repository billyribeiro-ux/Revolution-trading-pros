<script lang="ts">
	/**
	 * Dynamic Course Page Renderer
	 * Apple Principal Engineer ICT 11 Grade - January 2026
	 *
	 * Renders saved page layouts on public-facing course pages.
	 * Takes a layout from the database and renders all blocks dynamically.
	 */

	import type {
		PageLayout,
		CourseHeaderConfig,
		VideoPlayerConfig,
		VideoStackConfig,
		ClassDownloadsConfig,
		SpacerConfig,
		DividerConfig
	} from '../types';
	import CourseHeaderPreview from './previews/CourseHeaderPreview.svelte';
	import VideoPlayerPreview from './previews/VideoPlayerPreview.svelte';
	import VideoStackPreview from './previews/VideoStackPreview.svelte';
	import ClassDownloadsPreview from './previews/ClassDownloadsPreview.svelte';
	import SpacerPreview from './previews/SpacerPreview.svelte';
	import DividerPreview from './previews/DividerPreview.svelte';

	interface Props {
		layout: PageLayout | null;
		courseId?: string;
		isEnrolled?: boolean;
	}

	let { layout, courseId, isEnrolled = false }: Props = $props();

	// Sort blocks by order
	const sortedBlocks = $derived(
		layout?.blocks ? [...layout.blocks].sort((a, b) => a.order - b.order) : []
	);
</script>

{#if layout && sortedBlocks.length > 0}
	<div class="dynamic-renderer">
		{#each sortedBlocks as block (block.id)}
			<div class="rendered-block">
				{#if block.type === 'course-header'}
					<CourseHeaderPreview config={block.config as CourseHeaderConfig} isPreview={true} />
				{:else if block.type === 'video-player'}
					<VideoPlayerPreview config={block.config as VideoPlayerConfig} isPreview={true} />
				{:else if block.type === 'video-stack'}
					<VideoStackPreview config={block.config as VideoStackConfig} isPreview={true} />
				{:else if block.type === 'class-downloads'}
					<ClassDownloadsPreview
						config={{
							...(block.config as ClassDownloadsConfig),
							courseId: courseId || (block.config as ClassDownloadsConfig).courseId
						}}
						isPreview={true}
					/>
				{:else if block.type === 'spacer'}
					<SpacerPreview config={block.config as SpacerConfig} isPreview={true} />
				{:else if block.type === 'divider'}
					<DividerPreview config={block.config as DividerConfig} isPreview={true} />
				{/if}
			</div>
		{/each}
	</div>
{:else}
	<div class="no-layout">
		<p>Course content is being prepared...</p>
	</div>
{/if}

<style>
	.dynamic-renderer {
		width: 100%;
		max-width: 1200px;
		margin: 0 auto;
		padding: 0 24px;
	}

	.rendered-block {
		margin-bottom: 0;
	}

	.no-layout {
		text-align: center;
		padding: 64px 24px;
		color: #6b7280;
	}

	.no-layout p {
		font-size: 16px;
		margin: 0;
	}

	@media (max-width: 768px) {
		.dynamic-renderer {
			padding: 0 16px;
		}
	}
</style>

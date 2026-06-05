<!--
  MediaSkeleton Component
  ═══════════════════════════════════════════════════════════════════════════

  Content-shaped skeleton loaders for media components:
  - Grid skeleton for gallery views
  - Card skeleton for individual items
  - List skeleton for table views
  - Upload skeleton for upload progress

-->
<script lang="ts">
	import Icon from '$lib/components/Icon.svelte';
	// Props
	let {
		type = 'grid',
		count = 8,
		columns = 4,
		aspectRatio = '1/1',
		showText = true,
		className = ''
	}: {
		type?: 'grid' | 'card' | 'list' | 'upload' | 'single';
		count?: number;
		columns?: number;
		aspectRatio?: string;
		showText?: boolean;
		className?: string;
	} = $props();
</script>

{#if type === 'grid'}
	<!-- Grid skeleton for gallery views -->
	<div class={['media-skeleton-grid', className]} style:--columns={columns}>
		{#each Array(count) as _, i (i)}
			<div class="skeleton-card" style:animation-delay={`${i * 0.05}s`}>
				<div class="skeleton-media" style:aspect-ratio={aspectRatio}>
					<div class="skeleton-shimmer"></div>
				</div>
				{#if showText}
					<div class="skeleton-card__body">
						<div class="skeleton-line skeleton-line--title"></div>
						<div class="skeleton-line skeleton-line--meta"></div>
					</div>
				{/if}
			</div>
		{/each}
	</div>
{:else if type === 'card'}
	<!-- Single card skeleton -->
	<div class={['skeleton-card-large', className]}>
		<div class="skeleton-media" style:aspect-ratio={aspectRatio}>
			<div class="skeleton-shimmer"></div>
		</div>
		{#if showText}
			<div class="skeleton-card-large__body">
				<div class="skeleton-line skeleton-line--headline"></div>
				<div class="skeleton-line"></div>
				<div class="skeleton-line skeleton-line--meta"></div>
				<div class="skeleton-actions">
					<div class="skeleton-button"></div>
					<div class="skeleton-button"></div>
				</div>
			</div>
		{/if}
	</div>
{:else if type === 'list'}
	<!-- List/table skeleton -->
	<div class={['skeleton-list', className]}>
		{#each Array(count) as _, i (i)}
			<div class="skeleton-row" style:animation-delay={`${i * 0.05}s`}>
				<div class="skeleton-checkbox"></div>
				<div class="skeleton-thumbnail">
					<div class="skeleton-shimmer"></div>
				</div>
				<div class="skeleton-row__content">
					<div class="skeleton-line skeleton-line--name"></div>
					<div class="skeleton-line skeleton-line--description"></div>
				</div>
				<div class="skeleton-line skeleton-line--size"></div>
				<div class="skeleton-line skeleton-line--date"></div>
				<div class="skeleton-row__action"></div>
			</div>
		{/each}
	</div>
{:else if type === 'upload'}
	<!-- Upload progress skeleton -->
	<div class={['skeleton-upload', className]}>
		<div class="skeleton-upload__header">
			<div class="skeleton-line skeleton-line--upload-title"></div>
			<div class="skeleton-progress">
				<div class="skeleton-shimmer"></div>
			</div>
		</div>
		{#each Array(count) as _, i (i)}
			<div class="skeleton-upload-item" style:animation-delay={`${i * 0.1}s`}>
				<div class="skeleton-thumbnail">
					<div class="skeleton-shimmer"></div>
				</div>
				<div class="skeleton-upload-item__content">
					<div class="skeleton-line skeleton-line--upload-name"></div>
					<div class="skeleton-progress skeleton-progress--thin">
						<div class="skeleton-shimmer"></div>
					</div>
				</div>
				<div class="skeleton-upload-item__status"></div>
			</div>
		{/each}
	</div>
{:else if type === 'single'}
	<!-- Single image skeleton -->
	<div class={['skeleton-single', className]} style:aspect-ratio={aspectRatio}>
		<div class="skeleton-shimmer"></div>
		<div class="skeleton-single__icon">
			<Icon name="IconPhoto" size={48} />
		</div>
	</div>
{/if}

<style>
	/* Shimmer gradient animation */
	.skeleton-shimmer {
		position: absolute;
		inset: 0;
		background: linear-gradient(
			90deg,
			transparent 0%,
			rgba(255, 255, 255, 0.4) 50%,
			transparent 100%
		);
		animation: shimmer 1.5s infinite;
	}

	:global(.dark) .skeleton-shimmer {
		background: linear-gradient(
			90deg,
			transparent 0%,
			rgba(255, 255, 255, 0.1) 50%,
			transparent 100%
		);
	}

	/* Keyframe animations */
	@keyframes shimmer {
		0% {
			transform: translateX(-100%);
		}
		100% {
			transform: translateX(100%);
		}
	}

	@keyframes pulse {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.5;
		}
	}

	.skeleton-card,
	.skeleton-card-large,
	.skeleton-row,
	.skeleton-upload,
	.skeleton-upload-item,
	.skeleton-single {
		background: #f3f4f6;
		animation: pulse 2s ease-in-out infinite;
	}

	:global(.dark) .skeleton-card,
	:global(.dark) .skeleton-card-large,
	:global(.dark) .skeleton-row,
	:global(.dark) .skeleton-upload,
	:global(.dark) .skeleton-upload-item,
	:global(.dark) .skeleton-single {
		background: #1f2937;
	}

	.skeleton-media,
	.skeleton-line,
	.skeleton-button,
	.skeleton-checkbox,
	.skeleton-thumbnail,
	.skeleton-row__action,
	.skeleton-progress,
	.skeleton-upload-item__status,
	.skeleton-single {
		position: relative;
		overflow: hidden;
		background: #e5e7eb;
	}

	:global(.dark) .skeleton-media,
	:global(.dark) .skeleton-line,
	:global(.dark) .skeleton-button,
	:global(.dark) .skeleton-checkbox,
	:global(.dark) .skeleton-thumbnail,
	:global(.dark) .skeleton-row__action,
	:global(.dark) .skeleton-progress,
	:global(.dark) .skeleton-upload-item__status,
	:global(.dark) .skeleton-single {
		background: #374151;
	}

	.media-skeleton-grid {
		display: grid;
		grid-template-columns: repeat(var(--columns), 1fr);
		gap: 1rem;
	}

	.skeleton-card {
		overflow: hidden;
		border-radius: 0.5rem;
	}

	.skeleton-card-large {
		overflow: hidden;
		border-radius: 0.75rem;
	}

	.skeleton-card__body {
		display: grid;
		gap: 0.5rem;
		padding: 0.75rem;
	}

	.skeleton-card-large__body {
		display: grid;
		gap: 1rem;
		padding: 1.5rem;
	}

	.skeleton-line {
		width: 100%;
		height: 1rem;
		border-radius: 0.25rem;
	}

	.skeleton-line--title {
		width: 75%;
	}

	.skeleton-line--meta {
		width: 50%;
		height: 0.75rem;
	}

	.skeleton-line--headline {
		width: 66.666%;
		height: 1.5rem;
	}

	.skeleton-actions {
		display: flex;
		gap: 0.75rem;
		padding-top: 0.5rem;
	}

	.skeleton-button {
		width: 6rem;
		height: 2.5rem;
		border-radius: 0.5rem;
	}

	.skeleton-list {
		display: grid;
		gap: 0.5rem;
	}

	.skeleton-row {
		display: flex;
		align-items: center;
		gap: 1rem;
		border-radius: 0.5rem;
		padding: 0.75rem;
	}

	.skeleton-checkbox {
		width: 1.25rem;
		height: 1.25rem;
		border-radius: 0.25rem;
		flex: 0 0 auto;
	}

	.skeleton-thumbnail {
		width: 3rem;
		height: 3rem;
		border-radius: 0.5rem;
		flex: 0 0 auto;
	}

	.skeleton-row__content,
	.skeleton-upload-item__content {
		display: grid;
		gap: 0.5rem;
		flex: 1 1 auto;
		min-width: 0;
	}

	.skeleton-line--name {
		width: 12rem;
		max-width: 100%;
	}

	.skeleton-line--description {
		width: 8rem;
		max-width: 80%;
		height: 0.75rem;
	}

	.skeleton-line--size {
		width: 4rem;
	}

	.skeleton-line--date {
		width: 6rem;
	}

	.skeleton-row__action {
		width: 5rem;
		height: 2rem;
		border-radius: 0.25rem;
		flex: 0 0 auto;
	}

	.skeleton-upload {
		overflow: hidden;
		border-radius: 0.75rem;
	}

	.skeleton-upload__header {
		display: grid;
		gap: 0.5rem;
		border-bottom: 1px solid #e5e7eb;
		padding: 1rem;
	}

	:global(.dark) .skeleton-upload__header,
	:global(.dark) .skeleton-upload-item {
		border-color: #374151;
	}

	.skeleton-line--upload-title {
		width: 8rem;
		height: 1.25rem;
	}

	.skeleton-progress {
		width: 100%;
		height: 0.25rem;
		border-radius: 999px;
	}

	.skeleton-progress--thin {
		height: 0.5rem;
	}

	.skeleton-upload-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		border-bottom: 1px solid #e5e7eb;
		padding: 1rem;
	}

	.skeleton-upload-item:last-child {
		border-bottom: 0;
	}

	.skeleton-line--upload-name {
		width: 10rem;
		max-width: 100%;
	}

	.skeleton-upload-item__status {
		width: 2rem;
		height: 2rem;
		border-radius: 999px;
		flex: 0 0 auto;
	}

	.skeleton-single {
		border-radius: 0.5rem;
	}

	.skeleton-single__icon {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #d1d5db;
	}

	:global(.dark) .skeleton-single__icon {
		color: #4b5563;
	}

	/* Responsive grid adjustments */
	@media (max-width: 767.98px) {
		.media-skeleton-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 479.98px) {
		.media-skeleton-grid {
			grid-template-columns: repeat(1, 1fr);
		}
	}

	@media (max-width: 767.98px) {
		.skeleton-row {
			align-items: flex-start;
			flex-wrap: wrap;
		}

		.skeleton-line--size,
		.skeleton-line--date,
		.skeleton-row__action {
			display: none;
		}
	}
</style>

<!--
  MediaSkeleton Component
  ═══════════════════════════════════════════════════════════════════════════

  Content-shaped skeleton loaders for media components:
  - Grid skeleton for gallery views
  - Card skeleton for individual items
  - List skeleton for table views
  - Upload skeleton for upload progress

  @version 2.0.0
-->
<script lang="ts">
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
	<div class="skel-grid {className}" style="--columns: {columns};">
		{#each Array(count) as _, i (i)}
			<div class="skel-card" style="animation-delay: {i * 0.05}s;">
				<div class="skel-image" style="aspect-ratio: {aspectRatio};">
					<div class="skeleton-shimmer"></div>
				</div>
				{#if showText}
					<div class="skel-text-group">
						<div class="skel-line" style="inline-size: 75%;"></div>
						<div class="skel-line skel-line-sm" style="inline-size: 50%;"></div>
					</div>
				{/if}
			</div>
		{/each}
	</div>
{:else if type === 'card'}
	<!-- Single card skeleton -->
	<div class="skel-card skel-card-large {className}">
		<div class="skel-image" style="aspect-ratio: {aspectRatio};">
			<div class="skeleton-shimmer"></div>
		</div>
		{#if showText}
			<div class="skel-text-group skel-text-large">
				<div class="skel-line skel-line-lg" style="inline-size: 66%;"></div>
				<div class="skel-line"></div>
				<div class="skel-line" style="inline-size: 50%;"></div>
				<div class="skel-actions">
					<div class="skel-btn"></div>
					<div class="skel-btn"></div>
				</div>
			</div>
		{/if}
	</div>
{:else if type === 'list'}
	<!-- List/table skeleton -->
	<div class="skel-list {className}">
		{#each Array(count) as _, i (i)}
			<div class="skel-row" style="animation-delay: {i * 0.05}s;">
				<div class="skel-checkbox"></div>
				<div class="skel-thumb">
					<div class="skeleton-shimmer"></div>
				</div>
				<div class="skel-row-text">
					<div class="skel-line" style="inline-size: 12rem;"></div>
					<div class="skel-line skel-line-sm" style="inline-size: 8rem;"></div>
				</div>
				<div class="skel-line" style="inline-size: 4rem; block-size: 1rem;"></div>
				<div class="skel-line" style="inline-size: 6rem; block-size: 1rem;"></div>
				<div class="skel-line" style="inline-size: 5rem; block-size: 2rem;"></div>
			</div>
		{/each}
	</div>
{:else if type === 'upload'}
	<!-- Upload progress skeleton -->
	<div class="skel-upload {className}">
		<div class="skel-upload-header">
			<div class="skel-line" style="inline-size: 8rem; block-size: 1.25rem;"></div>
			<div class="skel-progress-track">
				<div class="skeleton-shimmer"></div>
			</div>
		</div>
		{#each Array(count) as _, i (i)}
			<div class="skel-upload-item" style="animation-delay: {i * 0.1}s;">
				<div class="skel-thumb">
					<div class="skeleton-shimmer"></div>
				</div>
				<div class="skel-row-text">
					<div class="skel-line" style="inline-size: 10rem;"></div>
					<div class="skel-progress-track">
						<div class="skeleton-shimmer"></div>
					</div>
				</div>
				<div class="skel-circle"></div>
			</div>
		{/each}
	</div>
{:else if type === 'single'}
	<!-- Single image skeleton -->
	<div class="skel-single {className}" style="aspect-ratio: {aspectRatio};">
		<div class="skeleton-shimmer"></div>
		<div class="skel-single-icon">
			<svg class="skel-svg-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="1.5"
					d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
				/>
			</svg>
		</div>
	</div>
{/if}

<style>
	/* Shimmer gradient animation */
	.skeleton-shimmer {
		position: absolute;
		inset: 0;
		background: linear-gradient(90deg, transparent 0%, oklch(1 0 0 / 40%) 50%, transparent 100%);
		animation: shimmer 1.5s infinite;
	}

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

	/* Shared bone style */
	.skel-line {
		block-size: 1rem;
		background-color: oklch(0.9 0.005 265);
		border-radius: var(--radius-sm);

		&.skel-line-sm {
			block-size: 0.75rem;
		}
		&.skel-line-lg {
			block-size: 1.5rem;
		}
	}

	.skel-image {
		position: relative;
		background-color: oklch(0.9 0.005 265);
		overflow: hidden;
	}

	/* ─── Grid ─── */
	.skel-grid {
		display: grid;
		grid-template-columns: repeat(var(--columns), 1fr);
		gap: var(--space-4);

		@media (max-width: 768px) {
			grid-template-columns: repeat(2, 1fr);
		}
		@media (max-width: 480px) {
			grid-template-columns: 1fr;
		}
	}

	.skel-card {
		background-color: oklch(0.95 0.002 265);
		border-radius: var(--radius-lg);
		overflow: hidden;
		animation: pulse 2s ease-in-out infinite;
	}

	.skel-card-large {
		border-radius: var(--radius-xl);
	}

	.skel-text-group {
		padding: var(--space-3);
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}

	.skel-text-large {
		padding: var(--space-6);
		gap: var(--space-4);
	}

	.skel-actions {
		display: flex;
		gap: var(--space-3);
		padding-block-start: var(--space-2);
	}

	.skel-btn {
		block-size: 2.5rem;
		inline-size: 6rem;
		background-color: oklch(0.9 0.005 265);
		border-radius: var(--radius-lg);
	}

	/* ─── List ─── */
	.skel-list {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}

	.skel-row {
		display: flex;
		align-items: center;
		gap: var(--space-4);
		padding: var(--space-3);
		background-color: oklch(0.95 0.002 265);
		border-radius: var(--radius-lg);
		animation: pulse 2s ease-in-out infinite;
	}

	.skel-checkbox {
		inline-size: 1.25rem;
		block-size: 1.25rem;
		background-color: oklch(0.9 0.005 265);
		border-radius: var(--radius-sm);
	}

	.skel-thumb {
		inline-size: 3rem;
		block-size: 3rem;
		background-color: oklch(0.9 0.005 265);
		border-radius: var(--radius-lg);
		position: relative;
		overflow: hidden;
	}

	.skel-row-text {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}

	/* ─── Upload ─── */
	.skel-upload {
		background-color: oklch(0.95 0.002 265);
		border-radius: var(--radius-xl);
		overflow: hidden;
		animation: pulse 2s ease-in-out infinite;
	}

	.skel-upload-header {
		padding: var(--space-4);
		border-block-end: 1px solid oklch(0.9 0.005 265);
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}

	.skel-progress-track {
		block-size: 0.25rem;
		background-color: oklch(0.9 0.005 265);
		border-radius: 9999px;
		position: relative;
		overflow: hidden;
	}

	.skel-upload-item {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		padding: var(--space-4);
		border-block-end: 1px solid oklch(0.9 0.005 265);
		animation: pulse 2s ease-in-out infinite;

		&:last-child {
			border-block-end: none;
		}
	}

	.skel-circle {
		inline-size: 2rem;
		block-size: 2rem;
		background-color: oklch(0.9 0.005 265);
		border-radius: 9999px;
	}

	/* ─── Single ─── */
	.skel-single {
		position: relative;
		background-color: oklch(0.9 0.005 265);
		border-radius: var(--radius-lg);
		overflow: hidden;
		animation: pulse 2s ease-in-out infinite;
	}

	.skel-single-icon {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.skel-svg-icon {
		inline-size: 3rem;
		block-size: 3rem;
		color: oklch(0.8 0.005 265);
	}
</style>

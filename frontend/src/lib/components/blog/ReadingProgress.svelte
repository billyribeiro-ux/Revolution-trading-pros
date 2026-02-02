<script lang="ts">
	/**
	 * Reading Progress Indicator - Svelte 5 Component
	 * Shows reading progress as a bar at the top of the page
	 *
	 * @version 2.0.0 - January 2026
	 * Updated: CSS layers, oklch colors, modern patterns
	 */

	interface Props {
		/** CSS selector for the content to track */
		contentSelector?: string;
		/** Bar height in pixels */
		height?: number;
		/** Progress bar color */
		color?: string;
		/** Background color */
		backgroundColor?: string;
		/** Position: 'top' or 'bottom' */
		position?: 'top' | 'bottom';
		/** Show percentage text */
		showPercentage?: boolean;
		/** Z-index for the bar */
		zIndex?: number;
	}

	let props: Props = $props();

	// Destructure with defaults for local use
	const contentSelector = $derived(props.contentSelector ?? '.post-body');
	const height = $derived(props.height ?? 4);
	const color = $derived(props.color ?? '#3b82f6');
	const backgroundColor = $derived(props.backgroundColor ?? 'rgba(59, 130, 246, 0.2)');
	const position = $derived(props.position ?? 'top');
	const showPercentage = $derived(props.showPercentage ?? false);
	const zIndex = $derived(props.zIndex ?? 9999);

	let progress = $state(0);
	let isVisible = $state(false);

	$effect(() => {
		if (typeof window === 'undefined') return;

		const contentElement = document.querySelector(contentSelector);
		if (!contentElement) return;

		let rafId: number | null = null;

		function updateProgress() {
			const rect = contentElement!.getBoundingClientRect();
			const viewportHeight = window.innerHeight;
			const contentTop = rect.top + window.scrollY;
			const contentHeight = rect.height;
			const scrollPosition = window.scrollY + viewportHeight;

			// Calculate how much of the content is visible/scrolled past
			const visibleContent = Math.max(0, scrollPosition - contentTop);
			const newProgress = Math.min(100, Math.max(0, (visibleContent / contentHeight) * 100));

			progress = Math.round(newProgress);

			// Show bar only after scrolling past the header area
			isVisible = window.scrollY > 100;
		}

		function handleScroll() {
			if (rafId !== null) return;
			rafId = requestAnimationFrame(() => {
				updateProgress();
				rafId = null;
			});
		}

		window.addEventListener('scroll', handleScroll, { passive: true });
		window.addEventListener('resize', handleScroll, { passive: true });

		// Initial calculation
		updateProgress();

		return () => {
			window.removeEventListener('scroll', handleScroll);
			window.removeEventListener('resize', handleScroll);
			if (rafId !== null) cancelAnimationFrame(rafId);
		};
	});
</script>

{#if isVisible}
	<div
		class="reading-progress"
		class:bottom={position === 'bottom'}
		style="
			--progress: {progress}%;
			--height: {height}px;
			--color: {color};
			--bg-color: {backgroundColor};
			--z-index: {zIndex};
		"
		role="progressbar"
		aria-valuenow={progress}
		aria-valuemin={0}
		aria-valuemax={100}
		aria-label="Reading progress"
	>
		<div class="progress-bar"></div>
		{#if showPercentage}
			<span class="progress-text">{progress}%</span>
		{/if}
	</div>
{/if}

<style>
	/* 2026 CSS Standards: CSS Layers, oklch colors */
	@layer components {
		.reading-progress {
			position: fixed;
			top: 0;
			left: 0;
			right: 0;
			height: var(--height);
			background: var(--bg-color);
			z-index: var(--z-index);
			transition: opacity 0.3s ease;
		}

		.reading-progress.bottom {
			top: auto;
			bottom: 0;
		}

		.progress-bar {
			height: 100%;
			width: var(--progress);
			background: var(--color);
			transition: width 0.1s ease-out;
			border-radius: 0 2px 2px 0;
			/* Modern gradient effect using color-mix */
			background-image: linear-gradient(
				90deg,
				var(--color),
				color-mix(in oklch, var(--color) 80%, oklch(0.8 0.15 290))
			);
		}

		.progress-text {
			position: absolute;
			right: 8px;
			top: 50%;
			transform: translateY(-50%);
			font-size: 10px;
			font-weight: 600;
			color: var(--color);
			background: oklch(0.1 0.02 260 / 0.9);
			padding: 2px 6px;
			border-radius: 4px;
		}
	}
</style>

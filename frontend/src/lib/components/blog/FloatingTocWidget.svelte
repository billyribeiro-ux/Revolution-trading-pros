<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { browser } from '$app/environment';
	import TableOfContents from './TableOfContents.svelte';

	export let contentBlocks: any[] = [];
	export let showAfterScroll: number = 300; // Show after scrolling this many pixels
	export let title: string = 'Contents';

	let isVisible = false;
	let isOpen = false;
	let scrollListener: (() => void) | null = null;

	function handleScroll() {
		if (!browser) return;
		isVisible = window.scrollY > showAfterScroll;
	}

	function toggleOpen() {
		isOpen = !isOpen;
	}

	function scrollToTop() {
		if (!browser) return;
		window.scrollTo({ top: 0, behavior: 'smooth' });
	}

	onMount(() => {
		if (browser) {
			scrollListener = handleScroll;
			window.addEventListener('scroll', scrollListener, { passive: true });
			handleScroll(); // Check initial position
		}
	});

	onDestroy(() => {
		if (scrollListener && browser) {
			window.removeEventListener('scroll', scrollListener);
		}
	});
</script>

{#if isVisible}
	<div class="floating-toc-widget" class:open={isOpen}>
		<!-- Toggle Button -->
		<button class="floating-toc-toggle" onclick={toggleOpen} aria-label="Toggle table of contents">
			<svg class="toc-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				{#if isOpen}
					<line x1="18" y1="6" x2="6" y2="18"></line>
					<line x1="6" y1="6" x2="18" y2="18"></line>
				{:else}
					<line x1="3" y1="6" x2="21" y2="6"></line>
					<line x1="3" y1="12" x2="15" y2="12"></line>
					<line x1="3" y1="18" x2="18" y2="18"></line>
				{/if}
			</svg>
			{#if !isOpen}
				<span class="toggle-label">{title}</span>
			{/if}
		</button>

		<!-- Scroll to top button -->
		<button class="scroll-top-btn" onclick={scrollToTop} aria-label="Scroll to top">
			<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<polyline points="18 15 12 9 6 15"></polyline>
			</svg>
		</button>

		<!-- TOC Panel -->
		{#if isOpen}
			<div class="floating-toc-panel">
				<TableOfContents
					{contentBlocks}
					{title}
					minHeadings={2}
					maxDepth={4}
					showNumbers={true}
					collapsible={false}
					defaultExpanded={true}
					sticky={false}
					showProgress={true}
					smoothScroll={true}
					highlightActive={true}
					position="inline"
				/>
			</div>
		{/if}
	</div>
{/if}

<style>
	.floating-toc-widget {
		position: fixed;
		bottom: 24px;
		right: 24px;
		z-index: 1000;
		display: flex;
		flex-direction: column;
		align-items: flex-end;
		gap: 12px;
		animation: fadeIn 0.3s ease;
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
			transform: translateY(20px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.floating-toc-toggle {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 12px 16px;
		background: linear-gradient(135deg, #3b82f6, #8b5cf6);
		border: none;
		border-radius: 50px;
		color: white;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		box-shadow: 0 8px 24px rgba(59, 130, 246, 0.4);
		transition: all 0.3s ease;
	}

	.floating-toc-toggle:hover {
		transform: scale(1.05);
		box-shadow: 0 12px 32px rgba(59, 130, 246, 0.5);
	}

	.floating-toc-widget.open .floating-toc-toggle {
		border-radius: 50%;
		padding: 12px;
	}

	.toc-icon {
		width: 20px;
		height: 20px;
	}

	.toggle-label {
		white-space: nowrap;
	}

	.scroll-top-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 44px;
		height: 44px;
		background: rgba(30, 41, 59, 0.95);
		backdrop-filter: blur(12px);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 50%;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.3s ease;
	}

	.scroll-top-btn:hover {
		background: rgba(96, 165, 250, 0.2);
		color: #60a5fa;
		border-color: #60a5fa;
		transform: translateY(-2px);
	}

	.scroll-top-btn svg {
		width: 20px;
		height: 20px;
	}

	.floating-toc-panel {
		position: absolute;
		bottom: 100%;
		right: 0;
		margin-bottom: 12px;
		width: 320px;
		max-height: 60vh;
		overflow: hidden;
		border-radius: 16px;
		box-shadow: 0 20px 50px rgba(0, 0, 0, 0.3);
		animation: slideUp 0.3s ease;
	}

	@keyframes slideUp {
		from {
			opacity: 0;
			transform: translateY(10px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	/* Responsive */
	@media (max-width: 768px) {
		.floating-toc-widget {
			bottom: 16px;
			right: 16px;
		}

		.floating-toc-panel {
			width: calc(100vw - 32px);
			max-width: 320px;
		}

		.toggle-label {
			display: none;
		}

		.floating-toc-toggle {
			padding: 12px;
			border-radius: 50%;
		}
	}
</style>

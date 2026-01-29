<script lang="ts">
	/**
	 * Floating TOC Widget - Svelte 5
	 *
	 * Updated: January 2026 - CSS layers, oklch colors, modern patterns
	 * Migrated to Svelte 5 runes ($props, $state, $effect)
	 */
	import { browser } from '$app/environment';
	import TableOfContents from './TableOfContents.svelte';

	interface ContentBlock {
		type: string;
		data?: {
			level?: number;
			text?: string;
			items?: string[];
			[key: string]: unknown;
		};
	}

	// Svelte 5: Props using $props() rune
	interface Props {
		contentBlocks?: ContentBlock[];
		showAfterScroll?: number;
		title?: string;
	}

	let { contentBlocks = [], showAfterScroll = 300, title = 'Contents' }: Props = $props();

	// Svelte 5: Reactive state using $state() rune
	let isVisible = $state(false);
	let isOpen = $state(false);

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

	// Svelte 5: Side effect with cleanup using $effect() rune
	$effect(() => {
		if (!browser) return;

		window.addEventListener('scroll', handleScroll, { passive: true });
		handleScroll(); // Check initial position

		// Cleanup function (returned from $effect)
		return () => {
			window.removeEventListener('scroll', handleScroll);
		};
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
	/* 2026 CSS Standards: CSS Layers, oklch colors */
	@layer components {
		.floating-toc-widget {
			--toc-accent-blue: oklch(0.6 0.19 250);
			--toc-accent-purple: oklch(0.65 0.18 290);
			--toc-bg-dark: oklch(0.2 0.02 260 / 0.95);
			--toc-text-muted: oklch(0.65 0.02 260);
			--toc-border: oklch(0.65 0.02 260 / 0.2);

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
			background: linear-gradient(135deg, var(--toc-accent-blue), var(--toc-accent-purple));
			border: none;
			border-radius: 50px;
			color: white;
			font-weight: 600;
			font-size: 0.875rem;
			cursor: pointer;
			box-shadow: 0 8px 24px color-mix(in oklch, var(--toc-accent-blue) 40%, transparent);
			transition: all 0.3s ease;
		}

		.floating-toc-toggle:hover {
			transform: scale(1.05);
			box-shadow: 0 12px 32px color-mix(in oklch, var(--toc-accent-blue) 50%, transparent);
		}

		.floating-toc-toggle:focus-visible {
			outline: 2px solid white;
			outline-offset: 2px;
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
			background: var(--toc-bg-dark);
			backdrop-filter: blur(12px);
			border: 1px solid var(--toc-border);
			border-radius: 50%;
			color: var(--toc-text-muted);
			cursor: pointer;
			transition: all 0.3s ease;
		}

		.scroll-top-btn:hover {
			background: color-mix(in oklch, var(--toc-accent-blue) 20%, transparent);
			color: var(--toc-accent-blue);
			border-color: var(--toc-accent-blue);
			transform: translateY(-2px);
		}

		.scroll-top-btn:focus-visible {
			outline: 2px solid var(--toc-accent-blue);
			outline-offset: 2px;
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
			box-shadow: 0 20px 50px oklch(0 0 0 / 0.3);
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
	}
</style>

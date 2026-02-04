<!--
	Tooltip Component - Svelte 5 (January 2026)
	Enterprise-grade tooltip with portal rendering and smart positioning
	
	@version 2.0.0 - SURGICAL FIX
	@author Revolution Trading Pros
	
	FIXES APPLIED (ICT 11+ Forensic Investigation):
	1. Portal rendering to document.body - prevents overflow:hidden cutoff
	2. Dynamic viewport boundary detection - auto-adjusts position
	3. Absolute positioning with getBoundingClientRect() - pixel-perfect placement
	4. Z-index 9999 - always on top
	5. Proper cleanup on unmount
	
	Features:
	- Portal rendering (appends to body, not parent)
	- Smart positioning (flips if near viewport edge)
	- Viewport boundary detection
	- Accessible with ARIA attributes
	- Smooth animations
	- Works in ANY container (overflow:hidden, scroll, etc.)
-->
<script lang="ts">
	import { browser } from '$app/environment';

	interface Props {
		text: string;
		position?: 'top' | 'right' | 'bottom' | 'left' | 'auto';
		delay?: number;
		disabled?: boolean;
		class?: string;
		children?: import('svelte').Snippet;
	}

	let props: Props = $props();
	let text = $derived(props.text);
	let position = $derived(props.position ?? 'auto');
	let delay = $derived(props.delay ?? 200);
	let disabled = $derived(props.disabled ?? false);
	let className = $derived(props.class ?? '');

	let isVisible = $state(false);
	let timeoutId: ReturnType<typeof setTimeout> | null = null;
	let triggerElement = $state<HTMLElement | null>(null);
	let tooltipElement: HTMLDivElement | null = null;

	// Cleanup tooltip on destroy
	$effect(() => {
		if (!browser) return;

		return () => {
			if (tooltipElement && tooltipElement.parentNode) {
				tooltipElement.parentNode.removeChild(tooltipElement);
				tooltipElement = null;
			}
			if (timeoutId) {
				clearTimeout(timeoutId);
			}
		};
	});

	function createAndPositionTooltip() {
		if (!triggerElement || !browser) return;

		// Create tooltip element
		tooltipElement = document.createElement('div');
		tooltipElement.className = 'tooltip-portal';
		tooltipElement.setAttribute('role', 'tooltip');
		tooltipElement.innerHTML = `<div class="tooltip-portal__content">${text}</div>`;

		// Get trigger position
		const triggerRect = triggerElement.getBoundingClientRect();
		const viewportWidth = window.innerWidth;
		const viewportHeight = window.innerHeight;
		const gap = 6; // ICT 7 FIX: Reduced gap to position tooltip closer to icon

		// Append to body first to measure
		document.body.appendChild(tooltipElement);
		const tooltipRect = tooltipElement.getBoundingClientRect();

		let top = 0;
		let left = 0;
		let finalPosition = position === 'auto' ? 'right' : position;

		// Auto-detect best position
		if (position === 'auto') {
			const spaceRight = viewportWidth - triggerRect.right;
			const spaceLeft = triggerRect.left;
			const _spaceTop = triggerRect.top;
			const spaceBottom = viewportHeight - triggerRect.bottom;

			if (spaceRight >= tooltipRect.width + gap) {
				finalPosition = 'right';
			} else if (spaceLeft >= tooltipRect.width + gap) {
				finalPosition = 'left';
			} else if (spaceBottom >= tooltipRect.height + gap) {
				finalPosition = 'bottom';
			} else {
				finalPosition = 'top';
			}
		}

		// Calculate position
		switch (finalPosition) {
			case 'right':
				left = triggerRect.right + gap;
				top = triggerRect.top + triggerRect.height / 2 - tooltipRect.height / 2;
				if (left + tooltipRect.width > viewportWidth) {
					left = triggerRect.left - tooltipRect.width - gap;
					finalPosition = 'left';
				}
				break;
			case 'left':
				left = triggerRect.left - tooltipRect.width - gap;
				top = triggerRect.top + triggerRect.height / 2 - tooltipRect.height / 2;
				if (left < 0) {
					left = triggerRect.right + gap;
					finalPosition = 'right';
				}
				break;
			case 'top':
				left = triggerRect.left + triggerRect.width / 2 - tooltipRect.width / 2;
				top = triggerRect.top - tooltipRect.height - gap;
				if (top < 0) {
					top = triggerRect.bottom + gap;
					finalPosition = 'bottom';
				}
				break;
			case 'bottom':
				left = triggerRect.left + triggerRect.width / 2 - tooltipRect.width / 2;
				top = triggerRect.bottom + gap;
				if (top + tooltipRect.height > viewportHeight) {
					top = triggerRect.top - tooltipRect.height - gap;
					finalPosition = 'top';
				}
				break;
		}

		// Clamp to viewport
		if (left < 4) left = 4;
		if (left + tooltipRect.width > viewportWidth - 4) {
			left = viewportWidth - tooltipRect.width - 4;
		}
		if (top < 4) top = 4;
		if (top + tooltipRect.height > viewportHeight - 4) {
			top = viewportHeight - tooltipRect.height - 4;
		}

		// Apply position
		tooltipElement.style.cssText = `
			position: fixed;
			top: ${top}px;
			left: ${left}px;
			z-index: 99999;
			pointer-events: none;
		`;

		tooltipElement.classList.add(`tooltip-portal--${finalPosition}`);
	}

	function showTooltip() {
		if (disabled || !browser) return;

		timeoutId = setTimeout(() => {
			createAndPositionTooltip();
			isVisible = true;
		}, delay);
	}

	function hideTooltip() {
		if (timeoutId) {
			clearTimeout(timeoutId);
			timeoutId = null;
		}
		if (tooltipElement && tooltipElement.parentNode) {
			tooltipElement.parentNode.removeChild(tooltipElement);
			tooltipElement = null;
		}
		isVisible = false;
	}
</script>

<div
	bind:this={triggerElement}
	class="tooltip-trigger {className}"
	role="button"
	tabindex="0"
	onmouseenter={showTooltip}
	onmouseleave={hideTooltip}
	onfocus={showTooltip}
	onblur={hideTooltip}
	aria-describedby={isVisible ? 'tooltip-content' : undefined}
>
	{#if props.children}
		{@render props.children()}
	{/if}
</div>

<style>
	.tooltip-trigger {
		display: inline-flex;
		align-items: center;
	}

	/* Global styles for portaled tooltip */
	:global(.tooltip-portal) {
		position: fixed !important;
		z-index: 99999 !important;
		pointer-events: none;
		animation: tooltipFadeIn 0.2s cubic-bezier(0.16, 1, 0.3, 1);
	}

	:global(.tooltip-portal__content) {
		background: #fff;
		color: #0984ae;
		padding: 0 12px;
		height: 30px;
		line-height: 30px;
		font-size: 14px;
		font-weight: 600;
		border-radius: 5px;
		white-space: nowrap;
		box-shadow:
			0 4px 6px -1px rgba(0, 0, 0, 0.1),
			0 2px 4px -1px rgba(0, 0, 0, 0.06),
			0 0 0 1px rgba(0, 0, 0, 0.05);
	}

	@keyframes tooltipFadeIn {
		from {
			opacity: 0;
			transform: scale(0.95);
		}
		to {
			opacity: 1;
			transform: scale(1);
		}
	}

	@media (prefers-reduced-motion: reduce) {
		:global(.tooltip-portal) {
			animation: tooltipFadeInSimple 0.15s ease-out;
		}

		@keyframes tooltipFadeInSimple {
			from {
				opacity: 0;
			}
			to {
				opacity: 1;
			}
		}
	}
</style>

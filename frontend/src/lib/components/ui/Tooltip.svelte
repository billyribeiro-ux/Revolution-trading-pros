<!--
	Tooltip Component - Svelte 5 (December 2025)
	Modern tooltip implementation using latest Svelte 5 runes and patterns
	
	@version 1.0.0
	@author Revolution Trading Pros
	
	Features:
	- Uses $state() and $derived() runes
	- Proper positioning with floating-ui patterns
	- Accessible with ARIA attributes
	- Smooth animations
	- Portal rendering for z-index issues
-->
<script lang="ts">
	import { browser } from '$app/environment';

	interface Props {
		text: string;
		position?: 'top' | 'right' | 'bottom' | 'left';
		delay?: number;
		disabled?: boolean;
		class?: string;
		children?: import('svelte').Snippet;
	}

	let {
		text,
		position = 'right',
		delay = 200,
		disabled = false,
		class: className = '',
		children
	}: Props = $props();

	let isVisible = $state(false);
	let timeoutId: ReturnType<typeof setTimeout> | null = null;
	let triggerElement = $state<HTMLElement | null>(null);

	function showTooltip() {
		if (disabled) return;
		
		timeoutId = setTimeout(() => {
			isVisible = true;
		}, delay);
	}

	function hideTooltip() {
		if (timeoutId) {
			clearTimeout(timeoutId);
			timeoutId = null;
		}
		isVisible = false;
	}

	// Position classes based on position prop
	let positionClasses = $derived(() => {
		const base = 'tooltip';
		switch (position) {
			case 'top':
				return `${base} ${base}--top`;
			case 'bottom':
				return `${base} ${base}--bottom`;
			case 'left':
				return `${base} ${base}--left`;
			case 'right':
			default:
				return `${base} ${base}--right`;
		}
	});
</script>

<div
	bind:this={triggerElement}
	class="tooltip-trigger {className}"
	onmouseenter={showTooltip}
	onmouseleave={hideTooltip}
	onfocus={showTooltip}
	onblur={hideTooltip}
	role="tooltip"
	aria-label={text}
>
	{#if children}
		{@render children()}
	{/if}
	
	{#if isVisible && browser}
		<div class={positionClasses()} role="tooltip">
			<div class="tooltip__content">
				{text}
			</div>
		</div>
	{/if}
</div>

<style>
	.tooltip-trigger {
		position: relative;
		display: inline-flex;
		align-items: center;
	}

	.tooltip {
		position: absolute;
		z-index: var(--z-tooltip, 700);
		pointer-events: none;
		animation: tooltipFadeIn 0.15s ease-out;
	}

	.tooltip__content {
		background: #fff;
		color: #0984ae;
		padding: 0 12px;
		height: 30px;
		line-height: 30px;
		font-size: 14px;
		font-weight: 600;
		border-radius: 5px;
		white-space: nowrap;
		box-shadow: 0 10px 30px rgba(0, 0, 0, 0.15);
	}

	/* Position variants */
	.tooltip--right {
		top: 50%;
		left: 100%;
		transform: translateY(-50%) translateX(5px);
		margin-left: -10px;
	}

	.tooltip--left {
		top: 50%;
		right: 100%;
		transform: translateY(-50%) translateX(-5px);
		margin-right: -10px;
	}

	.tooltip--top {
		bottom: 100%;
		left: 50%;
		transform: translateX(-50%) translateY(-5px);
		margin-bottom: 5px;
	}

	.tooltip--bottom {
		top: 100%;
		left: 50%;
		transform: translateX(-50%) translateY(5px);
		margin-top: 5px;
	}

	@keyframes tooltipFadeIn {
		from {
			opacity: 0;
			transform: translateY(-50%) translateX(10px);
		}
		to {
			opacity: 1;
			transform: translateY(-50%) translateX(5px);
		}
	}

	.tooltip--left {
		animation: tooltipFadeInLeft 0.15s ease-out;
	}

	@keyframes tooltipFadeInLeft {
		from {
			opacity: 0;
			transform: translateY(-50%) translateX(-10px);
		}
		to {
			opacity: 1;
			transform: translateY(-50%) translateX(-5px);
		}
	}

	.tooltip--top {
		animation: tooltipFadeInTop 0.15s ease-out;
	}

	@keyframes tooltipFadeInTop {
		from {
			opacity: 0;
			transform: translateX(-50%) translateY(-10px);
		}
		to {
			opacity: 1;
			transform: translateX(-50%) translateY(-5px);
		}
	}

	.tooltip--bottom {
		animation: tooltipFadeInBottom 0.15s ease-out;
	}

	@keyframes tooltipFadeInBottom {
		from {
			opacity: 0;
			transform: translateX(-50%) translateY(10px);
		}
		to {
			opacity: 1;
			transform: translateX(-50%) translateY(5px);
		}
	}
</style>

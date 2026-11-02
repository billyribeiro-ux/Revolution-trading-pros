<!--
	DropIndicator — floating drop-position indicator visible during drag operations.
	Extracted from BlockEditor.svelte (R7-C).
-->
<script lang="ts">
	import { fade } from 'svelte/transition';

	interface Props {
		y: number;
	}

	let { y }: Props = $props();
</script>

<div class="drop-indicator" style:top={`${y}px`} transition:fade={{ duration: 100 }}>
	<div class="drop-indicator-line"></div>
	<div class="drop-indicator-dot drop-indicator-dot-left"></div>
	<div class="drop-indicator-dot drop-indicator-dot-right"></div>
</div>

<style>
	/* Floating drop indicator */
	.drop-indicator {
		position: fixed;
		left: 0;
		right: 0;
		height: 4px;
		pointer-events: none;
		z-index: 10000;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.drop-indicator-line {
		position: absolute;
		left: 50%;
		transform: translateX(-50%);
		width: min(900px, calc(100% - 4rem));
		height: 4px;
		background: linear-gradient(
			90deg,
			transparent 0%,
			var(--primary-500, #e6b800) 5%,
			var(--primary-500, #e6b800) 50%,
			var(--primary-500, #e6b800) 95%,
			transparent 100%
		);
		border-radius: 2px;
		
	}

	.drop-indicator-dot {
		position: absolute;
		width: 12px;
		height: 12px;
		background: var(--primary-500, #e6b800);
		border: 2px solid var(--bg-elevated, #161b22);
		border-radius: 50%;
		
		animation: dropDotPulse 1s ease-in-out infinite;
	}

	.drop-indicator-dot-left {
		left: calc(50% - min(450px, calc(50% - 2rem)));
	}

	.drop-indicator-dot-right {
		right: calc(50% - min(450px, calc(50% - 2rem)));
	}

	@keyframes dropDotPulse {
		0%,
		100% {
			transform: scale(1);
		}
		50% {
			transform: scale(1.2);
		}
	}

	/* Reduced motion preference */
	@media (prefers-reduced-motion: reduce) {
		.drop-indicator-dot,
		.drop-indicator-line {
			animation: none;
			transition: none;
		}

	}

	/* Dark mode support */
	@media (prefers-color-scheme: dark) {
		.drop-indicator-dot {
			border-color: #1e1b4b;
		}
	}
</style>

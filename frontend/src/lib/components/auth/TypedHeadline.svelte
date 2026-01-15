<script lang="ts">
	/**
	 * TypedHeadline - Dynamic Typing Animation
	 * Netflix L11+ Principal Engineer Grade
	 *
	 * Uses Typed.js for premium typewriter effect
	 *
	 * @version 1.0.0
	 */
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import Typed from 'typed.js';

	interface Props {
		strings?: string[];
		typeSpeed?: number;
		backSpeed?: number;
		backDelay?: number;
		loop?: boolean;
		className?: string;
	}

	let {
		strings = ['Welcome Back, Trader', 'Ready to Trade?', 'Access Your Dashboard'],
		typeSpeed = 50,
		backSpeed = 30,
		backDelay = 2000,
		loop = true,
		className = ''
	}: Props = $props();

	let elementRef: HTMLSpanElement;
	let typed: Typed;

	onMount(() => {
		if (!browser || !elementRef) return;

		typed = new Typed(elementRef, {
			strings,
			typeSpeed,
			backSpeed,
			backDelay,
			loop,
			showCursor: true,
			cursorChar: '|',
			smartBackspace: true
		});

		return () => {
			typed?.destroy();
		};
	});
</script>

<span bind:this={elementRef} class="typed-headline {className}"></span>

<style>
	.typed-headline {
		display: inline-block;
		min-height: 1.2em;
	}

	:global(.typed-cursor) {
		font-weight: 300;
		opacity: 1;
		animation: blink 0.7s infinite;
		color: var(--auth-link, #818cf8);
	}

	@keyframes blink {
		0%, 100% { opacity: 1; }
		50% { opacity: 0; }
	}
</style>

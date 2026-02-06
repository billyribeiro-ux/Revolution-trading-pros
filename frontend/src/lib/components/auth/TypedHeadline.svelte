<script lang="ts">
	/**
	 * TypedHeadline - Dynamic Typing Animation
	 * Apple Principal Engineer ICT 11 Grade
	 *
	 * Uses Typed.js for premium typewriter effect
	 *
	 * ICT 11+ FIX: Hydration-safe implementation
	 * - SSR: Shows static first string as fallback
	 * - Client: Typed.js takes over after mount
	 * - Prevents hydration mismatch error
	 *
	 * @version 2.0.0 - Hydration-safe
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

	let props: Props = $props();
	const strings = $derived(props.strings ?? ['Welcome Back, Trader', 'Ready to Trade?', 'Access Your Dashboard']);
	const typeSpeed = $derived(props.typeSpeed ?? 50);
	const backSpeed = $derived(props.backSpeed ?? 30);
	const backDelay = $derived(props.backDelay ?? 2000);
	const loop = $derived(props.loop ?? true);
	const className = $derived(props.className ?? '');

	// ICT 11+ FIX: Track mount state for hydration-safe rendering
	let mounted = $state(false);
	let elementRef: HTMLSpanElement = $state(null!);
	let typed: Typed;

	onMount(() => {
		mounted = true;

		// Small delay to ensure DOM is ready after mount state change
		const initTimer = setTimeout(() => {
			if (!elementRef) return;

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
		}, 10);

		return () => {
			clearTimeout(initTimer);
			typed?.destroy();
		};
	});
</script>

<!--
	ICT 11+ Hydration Fix:
	- SSR: Show static first string (matches what client will render initially)
	- Client mounted: Empty span for Typed.js to populate
	This ensures SSR HTML matches client hydration expectation
-->
{#if mounted}
	<span bind:this={elementRef} class="typed-headline {className}"></span>
{:else}
	<span class="typed-headline {className}">{strings[0]}</span>
{/if}

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
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0;
		}
	}
</style>

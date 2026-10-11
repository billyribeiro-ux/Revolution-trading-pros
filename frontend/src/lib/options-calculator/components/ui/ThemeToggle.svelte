<script lang="ts">
	import IconSun from '@tabler/icons-svelte-runes/icons/sun';
	import IconMoon from '@tabler/icons-svelte-runes/icons/moon';
	import { backOut } from 'svelte/easing';
	import { prefersReducedMotion } from 'svelte/motion';
	import type { TransitionConfig } from 'svelte/transition';

	interface Props {
		theme: 'dark' | 'light';
		onToggle: () => void;
	}

	let { theme, onToggle }: Props = $props();

	/** The incoming icon spins and grows into place, with a slight overshoot. */
	function spinIn(_node: Element): TransitionConfig {
		return {
			duration: prefersReducedMotion.current ? 0 : 350,
			easing: backOut,
			css: (t, u) => `transform: rotate(${-90 * u}deg) scale(${0.5 + 0.5 * t}); opacity: ${t};`
		};
	}
</script>

<button
	type="button"
	onclick={onToggle}
	class="relative flex items-center justify-center w-9 h-9 rounded-lg transition-colors motion-reduce:transition-none cursor-pointer"
	style="background: var(--calc-surface-hover); border: 1px solid var(--calc-border); color: var(--calc-text-secondary);"
	aria-label="Dark mode"
	aria-pressed={theme === 'dark'}
>
	<!-- The branch swap mounts a fresh icon, so its intro plays on every toggle. -->
	{#if theme === 'dark'}
		<span class="flex items-center justify-center" in:spinIn
			><IconSun size={16} aria-hidden="true" /></span
		>
	{:else}
		<span class="flex items-center justify-center" in:spinIn
			><IconMoon size={16} aria-hidden="true" /></span
		>
	{/if}
</button>

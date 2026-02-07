<script lang="ts">
	import { Sun, Moon } from '@lucide/svelte';
	import gsap from 'gsap';

	interface Props {
		theme: 'dark' | 'light';
		onToggle: () => void;
	}

	let { theme, onToggle }: Props = $props();

	let iconEl: HTMLSpanElement | undefined = $state();

	$effect(() => {
		void theme;
		if (iconEl) {
			gsap.fromTo(
				iconEl,
				{ rotate: -90, scale: 0.5, opacity: 0 },
				{ rotate: 0, scale: 1, opacity: 1, duration: 0.35, ease: 'back.out(1.7)' }
			);
		}
	});
</script>

<button
	onclick={onToggle}
	class="relative flex items-center justify-center w-9 h-9 rounded-lg transition-colors cursor-pointer"
	style="background: var(--calc-surface-hover); border: 1px solid var(--calc-border); color: var(--calc-text-secondary);"
	aria-label="Toggle {theme === 'dark' ? 'light' : 'dark'} mode"
>
	<span bind:this={iconEl} class="flex items-center justify-center">
		{#if theme === 'dark'}
			<Sun size={16} />
		{:else}
			<Moon size={16} />
		{/if}
	</span>
</button>

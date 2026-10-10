<script lang="ts">
	import IconLock from '@tabler/icons-svelte-runes/icons/lock';
	import { quadOut } from 'svelte/easing';
	import { prefersReducedMotion } from 'svelte/motion';
	import { fade } from 'svelte/transition';
	import type { Snippet } from 'svelte';

	interface Props {
		/** Feature name for the "Unlock …" heading. */
		feature: string;
		isLocked: boolean;
		children: Snippet;
	}

	let { feature, isLocked, children }: Props = $props();
</script>

<div class="relative">
	<!-- Locked content stays visible behind the overlay, but cannot be focused or operated. -->
	<div class="contents" inert={isLocked}>
		{@render children()}
	</div>

	{#if isLocked}
		<div
			in:fade={{ duration: prefersReducedMotion.current ? 0 : 300, easing: quadOut }}
			class="absolute inset-0 z-10 flex flex-col items-center justify-center gap-3 rounded-xl"
			style="
				background: var(--calc-bg);
				opacity: 0.92;
				backdrop-filter: blur(8px);
			"
		>
			<div
				class="w-12 h-12 rounded-full flex items-center justify-center"
				style="background: rgba(99,102,241,0.1); border: 1px solid rgba(99,102,241,0.2);"
			>
				<IconLock size={20} style="color: var(--calc-accent);" aria-hidden="true" />
			</div>
			<p class="text-xs font-semibold text-center px-4" style="color: var(--calc-text);">
				Unlock {feature}
			</p>
			<p class="text-[10px] text-center px-6" style="color: var(--calc-text-muted);">
				This feature is available with Revolution Trading Pros membership.
			</p>
			<a
				href="https://revolutiontradingpros.com"
				target="_blank"
				rel="noopener"
				class="text-[10px] font-semibold px-4 py-1.5 rounded-lg transition-all duration-200"
				style="background: var(--calc-accent); color: white;"
			>
				Learn More
			</a>
		</div>
	{/if}
</div>

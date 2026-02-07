<script lang="ts">
	import { Lock } from '@lucide/svelte';
	import gsap from 'gsap';
	import type { Snippet } from 'svelte';

	interface Props {
		feature: string;
		isLocked: boolean;
		children: Snippet;
	}

	let { feature, isLocked, children }: Props = $props();

	let overlayEl: HTMLDivElement | undefined = $state();

	$effect(() => {
		if (isLocked && overlayEl) {
			gsap.fromTo(
				overlayEl,
				{ opacity: 0 },
				{ opacity: 1, duration: 0.3, ease: 'power2.out' },
			);
		}
	});
</script>

<div class="relative">
	{@render children()}

	{#if isLocked}
		<div
			bind:this={overlayEl}
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
				<Lock size={20} style="color: var(--calc-accent);" />
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

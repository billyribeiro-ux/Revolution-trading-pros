<script lang="ts">
	import { Info } from '@lucide/svelte';

	interface Props {
		content: string;
		class?: string;
	}

	let { content, class: className = '' }: Props = $props();

	let isVisible = $state(false);
</script>

<span
	class="relative inline-flex items-center {className}"
	onmouseenter={() => (isVisible = true)}
	onmouseleave={() => (isVisible = false)}
	onfocus={() => (isVisible = true)}
	onblur={() => (isVisible = false)}
	role="button"
	tabindex="0"
	aria-label="More information"
>
	<Info size={14} style="color: var(--calc-text-muted);" />

	{#if isVisible}
		<div
			class="absolute bottom-full left-1/2 -translate-x-1/2 mb-2 z-50 w-56 rounded-lg px-3 py-2 text-xs leading-relaxed pointer-events-none"
			style="
				background: var(--calc-glass);
				backdrop-filter: blur(var(--calc-blur));
				border: 1px solid var(--calc-glass-border);
				color: var(--calc-text-secondary);
				box-shadow: var(--calc-shadow-lg);
				font-family: var(--calc-font-body);
			"
			role="tooltip"
		>
			{content}
			<div
				class="absolute top-full left-1/2 -translate-x-1/2 w-2 h-2 rotate-45 -mt-1"
				style="background: var(--calc-glass); border-right: 1px solid var(--calc-glass-border); border-bottom: 1px solid var(--calc-glass-border);"
			></div>
		</div>
	{/if}
</span>

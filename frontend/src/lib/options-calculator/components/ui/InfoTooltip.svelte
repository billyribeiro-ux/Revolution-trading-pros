<script lang="ts">
	import IconInfoCircle from '@tabler/icons-svelte-runes/icons/info-circle';
	import type { ClassValue } from 'svelte/elements';

	interface Props {
		content: string;
		class?: ClassValue;
	}

	let { content, class: className }: Props = $props();

	const tooltipId = $props.id();
	let isVisible = $state(false);

	const show = () => (isVisible = true);
	const hide = () => (isVisible = false);
</script>

<span class={['relative inline-flex items-center', className]}>
	<button
		type="button"
		class="inline-flex items-center"
		aria-label="More information"
		aria-describedby={tooltipId}
		onmouseenter={show}
		onmouseleave={hide}
		onfocus={show}
		onblur={hide}
		onkeydown={(event) => {
			if (event.key === 'Escape') hide();
		}}
	>
		<IconInfoCircle size={14} style="color: var(--calc-text-muted);" aria-hidden="true" />
	</button>

	<!-- Always in the DOM (hidden when inactive) so the button's description is announced on focus. -->
	<span
		id={tooltipId}
		role="tooltip"
		hidden={!isVisible}
		class="absolute bottom-full left-1/2 -translate-x-1/2 mb-2 z-50 w-56 rounded-lg px-3 py-2 text-xs leading-relaxed pointer-events-none"
		style="
			background: var(--calc-glass);
			backdrop-filter: blur(var(--calc-blur));
			border: 1px solid var(--calc-glass-border);
			color: var(--calc-text-secondary);
			box-shadow: var(--calc-shadow-lg);
			font-family: var(--calc-font-body);
		"
	>
		{content}
		<span
			class="absolute top-full left-1/2 -translate-x-1/2 w-2 h-2 rotate-45 -mt-1"
			style="background: var(--calc-glass); border-right: 1px solid var(--calc-glass-border); border-bottom: 1px solid var(--calc-glass-border);"
		></span>
	</span>
</span>

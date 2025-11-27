<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		padding?: boolean;
		hover?: boolean;
		class?: string;
		children?: Snippet;
	}

	let {
		padding = true,
		hover = false,
		class: className = '',
		children
	}: Props = $props();
</script>

<div
	class="bg-white rounded-lg border border-gray-200 shadow-sm
    {padding ? 'p-6' : ''}
    {hover ? 'hover:shadow-md transition-shadow duration-200' : ''}
    {className}"
>
	{@render children?.()}
</div>

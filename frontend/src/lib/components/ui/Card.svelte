<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		padding?: boolean;
		hover?: boolean;
		class?: string;
		children?: Snippet;
	}

	let props: Props = $props();
	let padding = $derived(props.padding ?? true);
	let hover = $derived(props.hover ?? false);
	let className = $derived(props.class ?? '');
</script>

<div
	class="bg-white rounded-lg border border-gray-200 shadow-sm
    {padding ? 'p-6' : ''}
    {hover ? 'hover:shadow-md transition-shadow duration-200' : ''}
    {className}"
>
	{@render props.children?.()}
</div>

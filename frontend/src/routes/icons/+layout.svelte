<script lang="ts">
	import '$lib/icons/register-fontawesome-showcase';
	import type { Snippet } from 'svelte';

	interface Props {
		children: Snippet;
	}
	let { children }: Props = $props();
</script>

{@render children()}

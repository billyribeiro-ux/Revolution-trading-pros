<!--
	Maintenance Layout - Standalone (no NavBar/Footer chrome)
	Full-screen cinematic experience
-->
<script lang="ts">
	import '../../app.css';
	import type { Snippet } from 'svelte';

	interface Props {
		children: Snippet;
	}
	let { children }: Props = $props();
</script>

<svelte:head>
	<meta name="theme-color" content="#0a0e17" />
	<link rel="icon" type="image/x-icon" href="/favicon.ico" />
	<link rel="icon" type="image/png" sizes="128x128" href="/favicon.png" />
</svelte:head>

{@render children()}

<script lang="ts">
	import PublicVideoCatalog from '#lib/components/sections/PublicVideoCatalog.svelte';
	import type { PageProps } from './$types';
	let { data }: PageProps = $props();
</script>

<PublicVideoCatalog videos={data.videos} />

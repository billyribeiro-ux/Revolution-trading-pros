<script lang="ts">
	import { page } from '$app/state';
	import DealDetail from './DealDetail.svelte';
</script>

{#key page.params.id}<DealDetail dealId={page.params.id ?? ''} />{/key}

<script lang="ts">
	import { page } from '$app/state';
	import SequenceDetail from '#lib/components/admin/crm/SequenceDetail.svelte';
</script>

{#key page.params.id}<SequenceDetail id={page.params.id ?? ''} />{/key}

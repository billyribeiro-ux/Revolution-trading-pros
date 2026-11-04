<script lang="ts">
	import { page } from '$app/state';
	import ListMembers from '#lib/components/admin/crm/ListMembers.svelte';
</script>

{#key page.params.id}<ListMembers id={page.params.id ?? ''} />{/key}

<script lang="ts">
	import { page } from '$app/state';
	import ListForm from '#lib/components/admin/crm/ListForm.svelte';
</script>

{#key page.params.id}<ListForm id={page.params.id} />{/key}

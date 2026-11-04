<script lang="ts">
	import { page } from '$app/state';
	import DealForm from '#lib/components/admin/crm/DealForm.svelte';
</script>

{#key page.params.id}<DealForm id={page.params.id ?? ''} />{/key}

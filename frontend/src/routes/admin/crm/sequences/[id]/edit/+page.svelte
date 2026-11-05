<script lang="ts">
	import { page } from '$app/state';
	import SequenceForm from '#lib/components/admin/crm/SequenceForm.svelte';
</script>

{#key page.params.id}<SequenceForm id={page.params.id} />{/key}

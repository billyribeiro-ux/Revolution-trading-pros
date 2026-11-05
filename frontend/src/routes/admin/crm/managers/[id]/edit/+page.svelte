<script lang="ts">
	import { page } from '$app/state';
	import ManagerForm from '#lib/components/admin/crm/security/ManagerForm.svelte';
</script>

{#key page.params.id}<ManagerForm id={page.params.id} />{/key}

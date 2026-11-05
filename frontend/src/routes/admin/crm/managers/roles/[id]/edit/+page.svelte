<script lang="ts">
	import { page } from '$app/state';
	import RoleForm from '#lib/components/admin/crm/security/RoleForm.svelte';
</script>

{#key page.params.id}<RoleForm id={page.params.id} />{/key}

<script lang="ts">
	import { page } from '$app/state';
	import CompanyForm from '#lib/components/admin/crm/CompanyForm.svelte';
</script>

{#key page.params.id}
	<CompanyForm id={page.params.id} />
{/key}

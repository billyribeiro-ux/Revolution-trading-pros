<script lang="ts">
	import { page } from '$app/state';
	import TemplateForm from '#lib/components/admin/crm/TemplateForm.svelte';
</script>
{#key page.params.id}<TemplateForm id={page.params.id} />{/key}

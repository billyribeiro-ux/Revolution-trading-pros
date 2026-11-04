<script lang="ts">
	import { page } from '$app/state';
	import CampaignForm from '#lib/components/admin/crm/CampaignForm.svelte';
</script>

{#key page.params.id}<CampaignForm id={page.params.id ?? ''} />{/key}

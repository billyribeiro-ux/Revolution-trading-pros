<script lang="ts">
	import { page } from '$app/state';
	import CampaignReport from './CampaignReport.svelte';
</script>

{#key page.params.id}<CampaignReport id={page.params.id ?? ''} />{/key}

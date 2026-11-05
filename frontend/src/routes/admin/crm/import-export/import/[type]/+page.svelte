<script lang="ts">
    import {page} from '$app/state';
    import TransferImportForm from '#lib/components/admin/crm/TransferImportForm.svelte';
    let {data}:import('./$types').PageProps=$props();
</script>
{#key `${data.kind}:${page.url.searchParams.get('job') ?? ''}`}<TransferImportForm kind={data.kind} jobId={page.url.searchParams.get('job') ?? undefined} />{/key}

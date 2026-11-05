<script lang="ts">
    import {page} from '$app/state';
    import TransferImportForm from '#lib/components/admin/crm/TransferImportForm.svelte';
</script>
{#key page.url.searchParams.get('job')}<TransferImportForm jobId={page.url.searchParams.get('job') ?? undefined} />{/key}

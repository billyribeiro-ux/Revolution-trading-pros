<script lang="ts">
 import { page } from '$app/state';
 import SegmentForm from '#lib/components/admin/crm/SegmentForm.svelte';
</script>
{#key page.params.id}<SegmentForm id={page.params.id} />{/key}

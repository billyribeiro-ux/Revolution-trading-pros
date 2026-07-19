<!-- Stub — audit 06-crm.md P0 #1 (orphan-link epidemic). Backend deferred: docs/audits/admin-2026-04-26/06-crm-DEFERRED.md -->
<script lang="ts">
	import { page } from '$app/state';
	import ComingSoonStub from '$lib/components/admin/ComingSoonStub.svelte';
</script>

<ComingSoonStub
	entity="Edit Contact"
	backHref="/admin/crm/contacts"
	backLabel="Contacts"
	id={page.params.id ?? ''}
/>

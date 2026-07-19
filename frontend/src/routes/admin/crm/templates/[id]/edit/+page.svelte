<!-- Stub — audit 06-crm.md P0 #1 (orphan-link epidemic). Backend deferred: docs/audits/admin-2026-04-26/06-crm-DEFERRED.md -->
<script lang="ts">
	import { page } from '$app/state';
	import ComingSoonStub from '$lib/components/admin/ComingSoonStub.svelte';
</script>

<ComingSoonStub
	entity="Edit Email Template"
	backHref="/admin/crm/templates"
	backLabel="Templates"
	idLabel="Template ID"
	bodyLead="This edit form is not yet connected to the backend."
	id={page.params.id ?? ''}
/>

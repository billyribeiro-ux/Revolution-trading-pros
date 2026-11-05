<script lang="ts">
	import { page } from '$app/state';
	import ContactProfile from '../../../admin/crm/contacts/[id]/_components/ContactProfile.svelte';
</script>

{#key page.params.id}<ContactProfile contactId={page.params.id ?? ''} back="/crm/contacts" />{/key}

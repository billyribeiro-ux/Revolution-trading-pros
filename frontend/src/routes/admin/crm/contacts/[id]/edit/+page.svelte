<script lang="ts">
 import { page } from '$app/state';
 import ContactForm from '#lib/components/admin/crm/ContactForm.svelte';
</script>
{#key page.params.id}<ContactForm id={page.params.id ?? ''} kind="contacts" />{/key}

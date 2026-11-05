<script lang="ts">
	import { page } from '$app/state';
	import ContactProfile from './_components/ContactProfile.svelte';
</script>

{#key page.params.id}<ContactProfile contactId={page.params.id ?? ''} />{/key}

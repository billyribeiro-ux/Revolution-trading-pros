<script lang="ts">
	import { page } from '$app/state';
	import EmailTemplateEdit from './EmailTemplateEdit.svelte';
</script>

{#key page.params.id}<EmailTemplateEdit id={page.params.id ?? ''} />{/key}

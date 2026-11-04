<script lang="ts">
	import { page } from '$app/state';
	import EmailTemplatePreview from './EmailTemplatePreview.svelte';
</script>

{#key page.params.id}<EmailTemplatePreview id={page.params.id ?? ''} />{/key}

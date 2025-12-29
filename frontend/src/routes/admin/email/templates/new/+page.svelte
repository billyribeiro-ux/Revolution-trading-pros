<script lang="ts">
	import TemplateForm from '$lib/components/admin/TemplateForm.svelte';
</script>

<svelte:head>
	<title>New Email Template | Admin | Revolution Trading Pros</title>
</svelte:head>

<div class="new-template-page">
	<TemplateForm isEdit={false} />
</div>

<style>
	.new-template-page {
		max-width: 1000px;
		margin: 0 auto;
		padding: 2rem;
	}
</style>

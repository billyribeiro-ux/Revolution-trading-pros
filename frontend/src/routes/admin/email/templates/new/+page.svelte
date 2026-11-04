<script lang="ts">
	import TemplateForm from '#lib/components/admin/TemplateForm.svelte';
</script>

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

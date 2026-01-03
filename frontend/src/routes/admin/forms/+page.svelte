<!--
	URL: /admin/forms
-->

<script lang="ts">
	import { goto } from '$app/navigation';
	import FormList from '$lib/components/forms/FormList.svelte';
	import type { Form } from '$lib/api/forms';

	function handleEdit(form: Form) {
		goto(`/admin/forms/${form.id}/edit`);
	}

	function handleViewSubmissions(form: Form) {
		goto(`/admin/forms/${form.id}/submissions`);
	}

	function handleViewAnalytics(form: Form) {
		goto(`/admin/forms/${form.id}/analytics`);
	}

	function handleCreate() {
		goto('/admin/forms/create');
	}
</script>

<svelte:head>
	<title>Forms - Admin Dashboard</title>
</svelte:head>

<div class="forms-page">
	<div class="page-header">
		<div>
			<h1>Forms</h1>
			<p class="page-description">Create and manage your forms</p>
		</div>
		<button class="btn-create" onclick={handleCreate}> + Create New Form </button>
	</div>

	<FormList
		onEdit={handleEdit}
		onViewSubmissions={handleViewSubmissions}
		onViewAnalytics={handleViewAnalytics}
	/>
</div>

<style>
	.forms-page {
		max-width: 1400px;
		margin: 0 auto;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
		padding-bottom: 1.5rem;
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
	}

	h1 {
		font-size: 2rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.5rem 0;
	}

	.page-description {
		color: #94a3b8;
		font-size: 0.9375rem;
		margin: 0;
	}

	.btn-create {
		padding: 0.875rem 1.5rem;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		color: white;
		border: none;
		border-radius: 10px;
		font-weight: 600;
		font-size: 0.9375rem;
		cursor: pointer;
		transition: all 0.2s;
		box-shadow: 0 4px 12px rgba(99, 102, 241, 0.3);
	}

	.btn-create:hover {
		transform: translateY(-2px);
		box-shadow: 0 6px 20px rgba(99, 102, 241, 0.4);
	}

	@media (max-width: 768px) {
		.page-header {
			flex-direction: column;
			gap: 1rem;
			align-items: stretch;
		}

		.btn-create {
			width: 100%;
		}
	}
</style>

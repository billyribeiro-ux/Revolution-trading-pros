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

<div class="page">
	<div class="admin-page-container">
	<div class="page-header">
		<h1>Forms</h1>
		<p class="subtitle">Create and manage your forms</p>
		<div class="actions">
			<button class="btn-primary" onclick={handleCreate}> + Create New Form </button>
		</div>
	</div>

	<FormList
		onEdit={handleEdit}
		onViewSubmissions={handleViewSubmissions}
		onViewAnalytics={handleViewAnalytics}
	/>
	</div><!-- End admin-page-container -->
</div>

<style>
	.page {
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
	}

	.page-header {
		text-align: center;
		margin-bottom: 2rem;
		padding-bottom: 1.5rem;
		border-bottom: 1px solid rgba(230, 184, 0, 0.1);
	}

	h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.5rem 0;
	}

	.subtitle {
		color: #64748b;
		font-size: 0.875rem;
		margin: 0 0 1rem 0;
	}

	.actions {
		display: flex;
		justify-content: center;
		gap: 0.75rem;
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 6px 20px rgba(230, 184, 0, 0.4);
	}

	@media (max-width: 768px) {
		.page {
			padding: 1rem;
		}

		.actions {
			flex-direction: column;
		}

		.btn-primary {
			width: 100%;
		}
	}
</style>

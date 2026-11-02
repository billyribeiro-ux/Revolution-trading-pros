<!--
	URL: /admin/forms
-->

<script lang="ts">
	import { goto } from '$app/navigation';
	import FormList from '#lib/components/forms/FormList.svelte';
	import IconPlus from '@tabler/icons-svelte-runes/icons/plus';
	import type { Form } from '#lib/api/forms.js';

	function handleEdit(form: Form) {
		goto(`/admin/forms/${form.id}/edit`);
	}

	function handleViewSubmissions(form: Form) {
		goto(`/admin/forms/${form.id}/submissions`);
	}

	function handleViewAnalytics(form: Form) {
		goto(`/admin/forms/${form.id}/analytics`);
	}
</script>

<svelte:head>
	<title>Forms - Admin Dashboard</title>
</svelte:head>

<div class="admin-forms">
	<div class="admin-page-container">
		<header class="page-header">
			<h1>Forms</h1>
			<p class="subtitle">Create and manage your forms</p>
			<div class="header-actions">
				<a href="/admin/forms/create" class="btn-primary">
					<IconPlus size={18} aria-hidden="true" />
					Create New Form
				</a>
			</div>
		</header>

		<FormList
			onEdit={handleEdit}
			onViewSubmissions={handleViewSubmissions}
			onViewAnalytics={handleViewAnalytics}
		/>
	</div>
	<!-- End admin-page-container -->
</div>

<style>
	.admin-forms {
		background: var(--bg-base);
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
		color: var(--text-primary, #f0f6fc);
		margin: 0 0 0.5rem 0;
	}

	.subtitle {
		color: var(--text-tertiary, #6e7681);
		font-size: 0.875rem;
		margin: 0 0 1rem 0;
	}

	.header-actions {
		display: flex;
		justify-content: center;
		gap: 0.75rem;
	}

	.btn-primary {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		min-height: 40px;
		padding: 0 1.25rem;
		background: var(--primary-500, #e6b800);
		color: var(--bg-base, #0d1117);
		border: 1px solid var(--primary-500, #e6b800);
		border-radius: 8px;
		font-size: 0.875rem;
		font-weight: 600;
		text-decoration: none;
		cursor: pointer;
		transition: background-color 0.2s ease;
	}

	.btn-primary:hover {
		background: var(--primary-400, #ffd11a);
	}

	.btn-primary:focus-visible {
		outline: none;
		box-shadow: var(--admin-focus-ring, 0 0 0 3px rgba(230, 184, 0, 0.35));
	}

	@media (prefers-reduced-motion: reduce) {
		.btn-primary {
			transition: none;
		}
	}

	@media (max-width: 767.98px) {
		.admin-forms {
			padding: 1rem;
		}

		.header-actions {
			flex-direction: column;
		}

		.btn-primary {
			width: 100%;
		}
	}
</style>

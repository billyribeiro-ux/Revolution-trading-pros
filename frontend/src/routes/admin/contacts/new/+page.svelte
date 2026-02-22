<!--
	URL: /admin/contacts/new
-->

<script lang="ts">
import { logger } from '$lib/utils/logger';
	import { goto } from '$app/navigation';
	import { IconUserPlus, IconArrowLeft } from '$lib/icons';
	import { crmAPI } from '$lib/api/crm';

	let formData = $state({
		first_name: '',
		last_name: '',
		email: '',
		phone: '',
		job_title: ''
	});
	let isSubmitting = $state(false);
	let errorMessage = $state<string | null>(null);

	async function handleSubmit(event: Event) {
		event.preventDefault();
		isSubmitting = true;
		errorMessage = null;

		try {
			await crmAPI.createContact({
				first_name: formData.first_name,
				last_name: formData.last_name,
				email: formData.email,
				...(formData.phone && { phone: formData.phone }),
				...(formData.job_title && { job_title: formData.job_title })
			});
			goto('/admin/contacts');
		} catch (error) {
			logger.error('Failed to create contact:', error);
			errorMessage = error instanceof Error ? error.message : 'Failed to create contact';
			isSubmitting = false;
		}
	}
</script>

<svelte:head>
	<title>New Contact - Admin</title>
</svelte:head>

<div class="new-contact-page">
	<div class="page-header">
		<a href="/admin/contacts" class="back-link">
			<IconArrowLeft size={18} />
			Back to Contacts
		</a>
		<h1>Add New Contact</h1>
	</div>

	<form class="contact-form" onsubmit={handleSubmit}>
		{#if errorMessage}
			<div class="error-banner">
				{errorMessage}
			</div>
		{/if}

		<div class="form-row">
			<div class="form-group">
				<label for="first_name">First Name *</label>
				<input
					type="text"
					id="first_name"
					name="first_name"
					bind:value={formData.first_name}
					required
				/>
			</div>
			<div class="form-group">
				<label for="last_name">Last Name *</label>
				<input
					type="text"
					id="last_name"
					name="last_name"
					bind:value={formData.last_name}
					required
				/>
			</div>
		</div>

		<div class="form-group">
			<label for="email">Email Address *</label>
			<input
				type="email"
				id="email"
				name="email"
				autocomplete="email"
				bind:value={formData.email}
				required
			/>
		</div>

		<div class="form-group">
			<label for="phone">Phone Number</label>
			<input type="tel" id="phone" name="phone" autocomplete="tel" bind:value={formData.phone} />
		</div>

		<div class="form-group">
			<label for="job_title">Job Title</label>
			<input type="text" id="job_title" name="job_title" bind:value={formData.job_title} />
		</div>

		<div class="form-actions">
			<a href="/admin/contacts" class="btn-secondary">Cancel</a>
			<button type="submit" class="btn-primary" disabled={isSubmitting}>
				<IconUserPlus size={18} />
				{isSubmitting ? 'Creating...' : 'Create Contact'}
			</button>
		</div>
	</form>
</div>

<style>
	.new-contact-page {
		max-width: 600px;
	}

	.page-header {
		margin-bottom: 2rem;
	}

	.back-link {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		color: #64748b;
		text-decoration: none;
		font-size: 0.9rem;
		margin-bottom: 1rem;
	}

	.back-link:hover {
		color: var(--primary-500);
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0;
	}

	.contact-form {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 16px;
		padding: 2rem;
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	.form-group {
		margin-bottom: 1.5rem;
	}

	.form-group label {
		display: block;
		font-size: 0.9rem;
		font-weight: 500;
		color: #94a3b8;
		margin-bottom: 0.5rem;
	}

	.form-group input {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.8);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 10px;
		color: #e2e8f0;
		font-size: 0.95rem;
	}

	.form-group input:focus {
		outline: none;
		border-color: rgba(230, 184, 0, 0.5);
	}

	.form-actions {
		display: flex;
		justify-content: flex-end;
		gap: 1rem;
		margin-top: 2rem;
		padding-top: 1.5rem;
		border-top: 1px solid rgba(230, 184, 0, 0.1);
	}

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: linear-gradient(135deg, var(--primary-500) 0%, var(--primary-600) 100%);
		color: var(--bg-base);
		border: none;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(230, 184, 0, 0.4);
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-secondary {
		padding: 0.75rem 1.5rem;
		background: rgba(230, 184, 0, 0.1);
		color: #94a3b8;
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 10px;
		font-weight: 600;
		text-decoration: none;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		background: rgba(230, 184, 0, 0.2);
		color: #e2e8f0;
	}

	.error-banner {
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #f87171;
		padding: 0.75rem 1rem;
		border-radius: 10px;
		margin-bottom: 1.5rem;
		font-size: 0.9rem;
	}
</style>

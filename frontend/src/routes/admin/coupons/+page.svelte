<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { couponsApi, AdminApiError } from '$lib/api/admin';
	import { IconPlus, IconTicket, IconEdit, IconTrash } from '@tabler/icons-svelte';

	let loading = true;
	let coupons: any[] = [];
	let error = '';

	onMount(async () => {
		await loadCoupons();
	});

	async function loadCoupons() {
		loading = true;
		error = '';
		try {
			const response = await couponsApi.list();
			coupons = response.data || [];
		} catch (err) {
			if (err instanceof AdminApiError) {
				if (err.status === 401) {
					goto('/login');
					return;
				} else if (err.status === 403) {
					error = 'You are not authorized to view coupons.';
				} else {
					error = err.message;
				}
			} else {
				error = 'Error connecting to server';
			}
			console.error('Failed to load coupons:', err);
		} finally {
			loading = false;
		}
	}

	async function deleteCoupon(id: number) {
		if (!confirm('Are you sure you want to delete this coupon?')) return;
		try {
			await couponsApi.delete(id);
			await loadCoupons();
		} catch (err) {
			alert('Failed to delete coupon');
			console.error(err);
		}
	}
</script>

<div class="admin-page">
	<div class="page-header">
		<h1>Coupons Management</h1>
		<button class="btn-primary" on:click={() => goto('/admin/coupons/create')}>
			+ Create Coupon
		</button>
	</div>

	{#if loading}
		<div class="loading">Loading coupons...</div>
	{:else if error}
		<div class="error">{error}</div>
	{:else if coupons.length === 0}
		<div class="empty-state">
			<p>No coupons yet. Create your first coupon!</p>
		</div>
	{:else}
		<div class="coupons-grid">
			{#each coupons as coupon}
				<div class="coupon-card">
					<div class="coupon-header">
						<div class="coupon-code">{coupon.code}</div>
						<div class="coupon-status" class:active={coupon.is_active}>
							{coupon.is_active ? 'Active' : 'Inactive'}
						</div>
					</div>
					<div class="coupon-details">
						<span class="coupon-type">{coupon.type}</span>
						<span class="coupon-value"
							>{coupon.value}{coupon.type === 'percentage' ? '%' : '$'} off</span
						>
					</div>
					<div class="coupon-actions">
						<button
							class="action-btn edit"
							on:click={() => goto(`/admin/coupons/edit/${coupon.id}`)}
						>
							<IconEdit size={16} />
							Edit
						</button>
						<button class="action-btn delete" on:click={() => deleteCoupon(coupon.id)}>
							<IconTrash size={16} />
							Delete
						</button>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.admin-page {
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 2rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.btn-primary {
		padding: 0.75rem 1.5rem;
		background: linear-gradient(135deg, #3b82f6, #8b5cf6);
		color: white;
		border: none;
		border-radius: 8px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 10px 30px rgba(59, 130, 246, 0.3);
	}

	.coupons-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		gap: 1.5rem;
	}

	.coupon-card {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 12px;
		padding: 1.5rem;
		transition: all 0.2s;
	}

	.coupon-card:hover {
		border-color: rgba(59, 130, 246, 0.4);
		transform: translateY(-2px);
	}

	.coupon-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.coupon-code {
		font-size: 1.5rem;
		font-weight: 700;
		color: #60a5fa;
	}

	.coupon-details {
		display: flex;
		gap: 1rem;
		margin-bottom: 1rem;
	}

	.coupon-type,
	.coupon-value {
		padding: 0.25rem 0.75rem;
		background: rgba(148, 163, 184, 0.1);
		border-radius: 6px;
		font-size: 0.875rem;
		color: #cbd5e1;
	}

	.coupon-status {
		display: inline-block;
		padding: 0.25rem 0.75rem;
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
		border-radius: 6px;
		font-size: 0.875rem;
		font-weight: 600;
	}

	.coupon-status.active {
		background: rgba(16, 185, 129, 0.1);
		color: #34d399;
	}

	.coupon-actions {
		display: flex;
		gap: 0.5rem;
		padding-top: 1rem;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
	}

	.action-btn {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.625rem;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 6px;
		color: #cbd5e1;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.action-btn:hover {
		background: rgba(148, 163, 184, 0.2);
		border-color: rgba(148, 163, 184, 0.3);
	}

	.action-btn.edit:hover {
		background: rgba(59, 130, 246, 0.1);
		border-color: rgba(59, 130, 246, 0.3);
		color: #60a5fa;
	}

	.action-btn.delete:hover {
		background: rgba(239, 68, 68, 0.1);
		border-color: rgba(239, 68, 68, 0.3);
		color: #f87171;
	}

	.loading,
	.error,
	.empty-state {
		text-align: center;
		padding: 3rem;
		color: #94a3b8;
		font-size: 1.125rem;
	}

	.error {
		color: #f87171;
	}
</style>

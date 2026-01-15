<!--
	URL: /admin/products
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { productsApi, AdminApiError } from '$lib/api/admin';
	import {
		IconPlus,
		IconEdit,
		IconTrash,
		IconBook,
		IconChartLine,
		IconCrown,
		IconShoppingCart
	} from '$lib/icons';

	let loading = $state(true);
	let products = $state<any[]>([]);
	let error = $state('');
	let selectedType = $state('all');

	const productTypes = [
		{ value: 'all', label: 'All Products', icon: IconShoppingCart },
		{ value: 'course', label: 'Courses', icon: IconBook },
		{ value: 'indicator', label: 'Indicators', icon: IconChartLine },
		{ value: 'membership', label: 'Memberships', icon: IconCrown }
	];

	onMount(async () => {
		await loadProducts();
	});

	async function loadProducts() {
		loading = true;
		error = '';
		try {
			const params: any = {};
			if (selectedType !== 'all') {
				params.type = selectedType;
			}
			const response = await productsApi.list(params);
			products = response.data || [];
		} catch (err) {
			if (err instanceof AdminApiError) {
				if (err.status === 401) {
					goto('/login');
					return;
				}
				error = err.message;
			} else {
				error = 'Failed to load products';
			}
			console.error('Failed to load products:', err);
		} finally {
			loading = false;
		}
	}

	async function deleteProduct(id: number, name: string) {
		if (!confirm(`Delete "${name}"? This action cannot be undone.`)) return;
		try {
			await productsApi.delete(id);
			await loadProducts();
		} catch (err) {
			alert('Failed to delete product');
			console.error(err);
		}
	}

	function getTypeIcon(type: string) {
		const typeMap: any = {
			course: IconBook,
			indicator: IconChartLine,
			membership: IconCrown,
			bundle: IconShoppingCart
		};
		return typeMap[type] || IconShoppingCart;
	}

	function getTypeColor(type: string) {
		const colorMap: any = {
			course: 'bg-blue-500',
			indicator: 'bg-purple-500',
			membership: 'bg-yellow-500',
			bundle: 'bg-green-500'
		};
		return colorMap[type] || 'bg-gray-500';
	}

	// Reload products when type filter changes
	$effect(() => {
		if (selectedType) {
			loadProducts();
		}
	});
</script>

<svelte:head>
	<title>Products Management | Admin</title>
</svelte:head>

<div class="admin-page">
	<div class="page-header">
		<div>
			<h1>Products Management</h1>
			<p>Manage courses, indicators, and memberships</p>
		</div>
		<button class="btn-primary" onclick={() => goto('/admin/products/create')}>
			<IconPlus size={18} />
			Add Product
		</button>
	</div>

	<!-- Type Filter -->
	<div class="type-filter">
		{#each productTypes as type}
			{@const Icon = type.icon}
			<button
				class="type-btn"
				class:active={selectedType === type.value}
				onclick={() => (selectedType = type.value)}
			>
				<Icon size={20} />
				{type.label}
			</button>
		{/each}
	</div>

	{#if loading}
		<div class="loading">
			<div class="spinner"></div>
			<p>Loading products...</p>
		</div>
	{:else if error}
		<div class="alert error">
			{error}
		</div>
	{:else if products.length === 0}
		<div class="empty-state">
			<IconShoppingCart size={64} stroke={1} />
			<h3>No products found</h3>
			<p>Create your first {selectedType === 'all' ? 'product' : selectedType}</p>
			<button class="btn-primary" onclick={() => goto('/admin/products/create')}>
				<IconPlus size={18} />
				Add Product
			</button>
		</div>
	{:else}
		<div class="products-grid">
			{#each products as product}
				{@const TypeIcon = getTypeIcon(product.type)}
				<div class="product-card">
					<div class="product-header">
						<div class="product-type {getTypeColor(product.type)}">
							<TypeIcon size={16} />
							{product.type}
						</div>
						<div class="product-status">
							{#if product.is_active}
								<span class="status-badge active">Active</span>
							{:else}
								<span class="status-badge inactive">Inactive</span>
							{/if}
						</div>
					</div>

					<div class="product-content">
						<h3>{product.name}</h3>
						{#if product.description}
							<p class="description">{product.description}</p>
						{/if}
						<div class="product-price">${product.price}</div>
					</div>

					<div class="product-actions">
						<button
							class="action-btn edit"
							onclick={() => goto(`/admin/products/edit/${product.id}`)}
							title="Edit"
						>
							<IconEdit size={16} />
						</button>
						<button
							class="action-btn delete"
							onclick={() => deleteProduct(product.id, product.name)}
							title="Delete"
						>
							<IconTrash size={16} />
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
		background: #0f172a;
		min-height: 100vh;
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
		margin-bottom: 0.5rem;
	}

	.page-header p {
		color: #94a3b8;
		font-size: 0.95rem;
		font-family: 'Open Sans Pro', 'Open Sans', sans-serif;
	}

	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: 8px;
		font-weight: 600;
		border: none;
		cursor: pointer;
		transition: all 0.2s;
		background: var(--admin-btn-primary-bg, linear-gradient(135deg, #E6B800 0%, #B38F00 100%));
		color: var(--admin-btn-primary-text, #0D1117);
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 16px rgba(230, 184, 0, 0.35);
	}

	.type-filter {
		display: flex;
		gap: 1rem;
		margin-bottom: 2rem;
		flex-wrap: wrap;
	}

	.type-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #cbd5e1;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.type-btn:hover {
		background: rgba(30, 41, 59, 0.8);
		border-color: rgba(148, 163, 184, 0.4);
	}

	.type-btn.active {
		background: var(--admin-btn-primary-bg, linear-gradient(135deg, #E6B800 0%, #B38F00 100%));
		border-color: transparent;
		color: var(--admin-btn-primary-text, #0D1117);
	}

	.loading,
	.empty-state {
		text-align: center;
		padding: 4rem 2rem;
		color: #94a3b8;
	}

	.loading .spinner {
		width: 48px;
		height: 48px;
		border: 4px solid rgba(148, 163, 184, 0.1);
		border-top-color: var(--admin-accent-primary, #E6B800);
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin: 0 auto 1rem;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.empty-state :global(svg) {
		color: #475569;
		margin-bottom: 1rem;
	}

	.empty-state h3 {
		color: #f1f5f9;
		margin-bottom: 0.5rem;
		font-size: 1.5rem;
	}

	.empty-state p {
		margin-bottom: 1.5rem;
	}

	.alert {
		padding: 1rem 1.5rem;
		border-radius: 8px;
		margin-bottom: 1.5rem;
	}

	.alert.error {
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #f87171;
	}

	.products-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
		gap: 1.5rem;
	}

	.product-card {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 12px;
		overflow: hidden;
		transition: all 0.3s;
	}

	.product-card:hover {
		transform: translateY(-4px);
		box-shadow: 0 12px 24px rgba(0, 0, 0, 0.3);
		border-color: rgba(148, 163, 184, 0.4);
	}

	.product-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem 1.5rem;
		background: rgba(15, 23, 42, 0.8);
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.product-type {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.375rem 0.75rem;
		border-radius: 6px;
		font-size: 0.875rem;
		font-weight: 600;
		text-transform: capitalize;
		color: white;
	}

	.bg-blue-500 {
		background: #3b82f6;
	}

	.bg-purple-500 {
		background: #B38F00;
	}

	.bg-yellow-500 {
		background: #eab308;
	}

	.bg-green-500 {
		background: #10b981;
	}

	.product-status .status-badge {
		padding: 0.25rem 0.75rem;
		border-radius: 6px;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: uppercase;
	}

	.status-badge.active {
		background: rgba(16, 185, 129, 0.2);
		color: #34d399;
	}

	.status-badge.inactive {
		background: rgba(148, 163, 184, 0.2);
		color: #94a3b8;
	}

	.product-content {
		padding: 1.5rem;
	}

	.product-content h3 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin-bottom: 0.75rem;
	}

	.description {
		color: #94a3b8;
		font-size: 0.875rem;
		line-height: 1.5;
		margin-bottom: 1rem;
		font-family: 'Open Sans Pro', 'Open Sans', sans-serif;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.product-price {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--admin-accent-primary, #E6B800);
	}

	.product-actions {
		display: flex;
		gap: 0.5rem;
		padding: 1rem 1.5rem;
		background: rgba(15, 23, 42, 0.4);
		border-top: 1px solid rgba(148, 163, 184, 0.1);
	}

	.action-btn {
		flex: 1;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		border-radius: 6px;
		font-weight: 500;
		border: none;
		cursor: pointer;
		transition: all 0.2s;
		font-size: 0.875rem;
	}

	.action-btn.edit {
		background: rgba(59, 130, 246, 0.1);
		color: #60a5fa;
		border: 1px solid rgba(59, 130, 246, 0.3);
	}

	.action-btn.edit:hover {
		background: rgba(59, 130, 246, 0.2);
		border-color: rgba(59, 130, 246, 0.5);
	}

	.action-btn.delete {
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
		border: 1px solid rgba(239, 68, 68, 0.3);
	}

	.action-btn.delete:hover {
		background: rgba(239, 68, 68, 0.2);
		border-color: rgba(239, 68, 68, 0.5);
	}

	@media (max-width: 768px) {
		.page-header {
			flex-direction: column;
			align-items: flex-start;
			gap: 1rem;
		}

		.products-grid {
			grid-template-columns: 1fr;
		}
	}
</style>

<!--
	URL: /admin/products
-->

<script lang="ts">
	import { untrack } from 'svelte';
	import { goto } from '$app/navigation';
	import { productsApi, AdminApiError, type Product } from '$lib/api/admin';
	import {
		IconPlus,
		IconEdit,
		IconTrash,
		IconBook,
		IconChartLine,
		IconCrown,
		IconShoppingCart,
		IconRefresh,
		IconSearch
	} from '$lib/icons';

	// Svelte 5 state runes
	let loading = $state(true);
	let products = $state<Product[]>([]);
	let error = $state('');
	let selectedType = $state('all');
	let searchQuery = $state('');
	let deleting = $state<number | null>(null);

	// Product type configuration
	const productTypes = [
		{ value: 'all', label: 'All Products', icon: IconShoppingCart },
		{ value: 'course', label: 'Courses', icon: IconBook },
		{ value: 'indicator', label: 'Indicators', icon: IconChartLine },
		{ value: 'membership', label: 'Memberships', icon: IconCrown }
	] as const;

	// Derived state for filtered products (client-side search)
	let filteredProducts = $derived(
		products.filter((product) => {
			if (!searchQuery.trim()) return true;
			const query = searchQuery.toLowerCase();
			return (
				product.name.toLowerCase().includes(query) ||
				product.description?.toLowerCase().includes(query) ||
				product.slug.toLowerCase().includes(query)
			);
		})
	);

	// Product count by type (derived)
	let productCountByType = $derived(
		products.reduce(
			(acc, product) => {
				acc[product.type] = (acc[product.type] || 0) + 1;
				return acc;
			},
			{} as Record<string, number>
		)
	);

	// Total count for display
	let totalCount = $derived(filteredProducts.length);

	// Load products from API
	async function loadProducts() {
		loading = true;
		error = '';
		try {
			const params: Record<string, string> = {};
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

	// Delete product with confirmation
	async function deleteProduct(id: number, name: string) {
		if (!confirm(`Delete "${name}"? This action cannot be undone.`)) return;

		deleting = id;
		try {
			await productsApi.delete(id);
			// Remove from local state immediately for better UX
			products = products.filter((p) => p.id !== id);
		} catch (err) {
			if (err instanceof AdminApiError) {
				error = `Failed to delete: ${err.message}`;
			} else {
				error = 'Failed to delete product';
			}
			console.error('Delete failed:', err);
		} finally {
			deleting = null;
		}
	}

	// Toggle product active status
	async function toggleActive(product: Product) {
		try {
			const response = await productsApi.update(product.id, {
				is_active: !product.is_active
			});
			// Update local state
			const index = products.findIndex((p) => p.id === product.id);
			if (index !== -1) {
				products[index] = { ...products[index], is_active: !product.is_active };
				products = [...products]; // Trigger reactivity
			}
		} catch (err) {
			error = 'Failed to update product status';
			console.error(err);
		}
	}

	// Helper functions
	function getTypeIcon(type: string) {
		const typeMap: Record<string, typeof IconBook> = {
			course: IconBook,
			indicator: IconChartLine,
			membership: IconCrown,
			bundle: IconShoppingCart
		};
		return typeMap[type] || IconShoppingCart;
	}

	function getTypeColor(type: string) {
		const colorMap: Record<string, string> = {
			course: 'bg-blue-500',
			indicator: 'bg-purple-500',
			membership: 'bg-yellow-500',
			bundle: 'bg-green-500'
		};
		return colorMap[type] || 'bg-gray-500';
	}

	function formatPrice(price: number, salePrice?: number | null) {
		if (salePrice && salePrice < price) {
			return { original: `$${price.toFixed(2)}`, sale: `$${salePrice.toFixed(2)}` };
		}
		return { original: `$${price.toFixed(2)}`, sale: null };
	}

	// Effect: Load products when type filter changes
	$effect(() => {
		// Track selectedType for reactivity
		const type = selectedType;
		// Use untrack to avoid infinite loops when updating products
		untrack(() => {
			loadProducts();
		});
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
		<div class="header-actions">
			<button class="btn-secondary" onclick={() => loadProducts()} disabled={loading}>
				<IconRefresh size={18} class={loading ? 'spinning' : ''} />
				Refresh
			</button>
			<button class="btn-primary" onclick={() => goto('/admin/products/create')}>
				<IconPlus size={18} />
				Add Product
			</button>
		</div>
	</div>

	<!-- Search and Filter Bar -->
	<div class="filter-bar">
		<div class="search-box">
			<IconSearch size={20} />
			<input
				type="text"
				placeholder="Search products..."
				bind:value={searchQuery}
			/>
		</div>
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
					{#if type.value !== 'all' && productCountByType[type.value]}
						<span class="count-badge">{productCountByType[type.value]}</span>
					{/if}
				</button>
			{/each}
		</div>
	</div>

	<!-- Results summary -->
	{#if !loading && products.length > 0}
		<div class="results-summary">
			Showing {totalCount} of {products.length} products
			{#if searchQuery}
				<span class="search-term">matching "{searchQuery}"</span>
			{/if}
		</div>
	{/if}

	<!-- Error Alert -->
	{#if error}
		<div class="alert error">
			<span>{error}</span>
			<button class="alert-dismiss" onclick={() => (error = '')}>Dismiss</button>
		</div>
	{/if}

	{#if loading}
		<div class="loading">
			<div class="spinner"></div>
			<p>Loading products...</p>
		</div>
	{:else if filteredProducts.length === 0}
		<div class="empty-state">
			<IconShoppingCart size={64} stroke={1} />
			<h3>No products found</h3>
			{#if searchQuery}
				<p>No products match your search "{searchQuery}"</p>
				<button class="btn-secondary" onclick={() => (searchQuery = '')}>
					Clear Search
				</button>
			{:else}
				<p>Create your first {selectedType === 'all' ? 'product' : selectedType}</p>
				<button class="btn-primary" onclick={() => goto('/admin/products/create')}>
					<IconPlus size={18} />
					Add Product
				</button>
			{/if}
		</div>
	{:else}
		<div class="products-grid">
			{#each filteredProducts as product (product.id)}
				{@const TypeIcon = getTypeIcon(product.type)}
				{@const pricing = formatPrice(product.price, product.sale_price)}
				<div class="product-card" class:deleting={deleting === product.id}>
					<div class="product-header">
						<div class="product-type {getTypeColor(product.type)}">
							<TypeIcon size={16} />
							{product.type}
						</div>
						<button
							class="status-toggle"
							class:active={product.is_active}
							onclick={() => toggleActive(product)}
							title={product.is_active ? 'Click to deactivate' : 'Click to activate'}
						>
							{#if product.is_active}
								<span class="status-badge active">Active</span>
							{:else}
								<span class="status-badge inactive">Inactive</span>
							{/if}
						</button>
					</div>

					{#if product.thumbnail}
						<div class="product-thumbnail">
							<img src={product.thumbnail} alt={product.name} />
						</div>
					{/if}

					<div class="product-content">
						<h3>{product.name}</h3>
						<p class="slug">/{product.slug}</p>
						{#if product.description}
							<p class="description">{product.description}</p>
						{/if}
						<div class="product-price">
							{#if pricing.sale}
								<span class="original-price">{pricing.original}</span>
								<span class="sale-price">{pricing.sale}</span>
							{:else}
								<span class="current-price">{pricing.original}</span>
							{/if}
						</div>
					</div>

					<div class="product-actions">
						<button
							class="action-btn edit"
							onclick={() => goto(`/admin/products/${product.id}/edit`)}
							title="Edit"
						>
							<IconEdit size={16} />
							Edit
						</button>
						<button
							class="action-btn delete"
							onclick={() => deleteProduct(product.id, product.name)}
							disabled={deleting === product.id}
							title="Delete"
						>
							{#if deleting === product.id}
								<div class="btn-spinner"></div>
							{:else}
								<IconTrash size={16} />
							{/if}
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

	.header-actions {
		display: flex;
		gap: 1rem;
		align-items: center;
	}

	.btn-primary,
	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: 8px;
		font-weight: 600;
		border: none;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary {
		background: linear-gradient(135deg, #3b82f6, #8b5cf6);
		color: white;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 4px 16px rgba(230, 184, 0, 0.35);
	}

	.btn-secondary {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		color: #cbd5e1;
	}

	.btn-secondary:hover:not(:disabled) {
		background: rgba(30, 41, 59, 0.8);
		border-color: rgba(148, 163, 184, 0.4);
	}

	.btn-secondary:disabled,
	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	/* Filter Bar */
	.filter-bar {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		max-width: 400px;
	}

	.search-box :global(svg) {
		color: #64748b;
		flex-shrink: 0;
	}

	.search-box input {
		flex: 1;
		background: transparent;
		border: none;
		color: #f1f5f9;
		font-size: 0.95rem;
		outline: none;
	}

	.search-box input::placeholder {
		color: #64748b;
	}

	.type-filter {
		display: flex;
		gap: 0.75rem;
		flex-wrap: wrap;
	}

	.type-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1.25rem;
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

	.count-badge {
		background: rgba(255, 255, 255, 0.2);
		padding: 0.125rem 0.5rem;
		border-radius: 12px;
		font-size: 0.75rem;
		font-weight: 600;
	}

	.results-summary {
		color: #94a3b8;
		font-size: 0.875rem;
		margin-bottom: 1rem;
	}

	.search-term {
		color: #60a5fa;
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
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem 1.5rem;
		border-radius: 8px;
		margin-bottom: 1.5rem;
	}

	.alert.error {
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #f87171;
	}

	.alert-dismiss {
		background: transparent;
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #f87171;
		padding: 0.375rem 0.75rem;
		border-radius: 4px;
		cursor: pointer;
		font-size: 0.75rem;
		font-weight: 500;
		transition: all 0.2s;
	}

	.alert-dismiss:hover {
		background: rgba(239, 68, 68, 0.2);
	}

	/* Spinning animation for refresh */
	:global(.spinning) {
		animation: spin 1s linear infinite;
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

	.product-card.deleting {
		opacity: 0.5;
		pointer-events: none;
	}

	.product-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem 1.5rem;
		background: rgba(15, 23, 42, 0.8);
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.product-thumbnail {
		height: 160px;
		overflow: hidden;
		background: rgba(15, 23, 42, 0.5);
	}

	.product-thumbnail img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.status-toggle {
		background: transparent;
		border: none;
		cursor: pointer;
		padding: 0;
		transition: all 0.2s;
	}

	.status-toggle:hover {
		transform: scale(1.05);
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
		margin-bottom: 0.25rem;
	}

	.slug {
		font-size: 0.75rem;
		color: #64748b;
		margin-bottom: 0.75rem;
		font-family: monospace;
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
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.current-price {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--admin-accent-primary, #E6B800);
	}

	.original-price {
		font-size: 1rem;
		color: #64748b;
		text-decoration: line-through;
	}

	.sale-price {
		font-size: 1.5rem;
		font-weight: 700;
		color: #10b981;
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

	.action-btn.delete:hover:not(:disabled) {
		background: rgba(239, 68, 68, 0.2);
		border-color: rgba(239, 68, 68, 0.5);
	}

	.action-btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-spinner {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(248, 113, 113, 0.3);
		border-top-color: #f87171;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
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

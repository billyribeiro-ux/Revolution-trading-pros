<script lang="ts">
	/**
	 * Dashboard - My Addresses Page (Billing Address)
	 * WordPress Revolution Trading Exact Match
	 */
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { isAuthenticated, authStore, user } from '$lib/stores/auth';
	import { IconMapPin, IconEdit, IconPlus, IconTrash } from '$lib/icons';

	onMount(() => {
		if (!$isAuthenticated && !$authStore.isInitializing) {
			goto('/login?redirect=/dashboard/addresses', { replaceState: true });
		}
	});

	interface Address {
		id: string;
		type: 'billing' | 'shipping';
		name: string;
		company?: string;
		address1: string;
		address2?: string;
		city: string;
		state: string;
		zip: string;
		country: string;
		phone?: string;
		isDefault: boolean;
	}

	let addresses: Address[] = $state([
		{
			id: 'addr_1',
			type: 'billing',
			name: $user?.name || 'John Doe',
			address1: '123 Trading Street',
			city: 'Austin',
			state: 'TX',
			zip: '78701',
			country: 'United States',
			phone: '(512) 555-0123',
			isDefault: true
		}
	]);

	let isEditing = $state(false);
	let editingAddress = $state<Address | null>(null);

	function handleEdit(address: Address) {
		editingAddress = { ...address };
		isEditing = true;
	}

	function handleSave() {
		if (editingAddress) {
			addresses = addresses.map(a => a.id === editingAddress!.id ? editingAddress! : a);
		}
		isEditing = false;
		editingAddress = null;
	}

	function handleCancel() {
		isEditing = false;
		editingAddress = null;
	}
</script>

<svelte:head>
	<title>My Addresses | Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

{#if $isAuthenticated}
	<header class="dashboard__header">
		<h1 class="dashboard__page-title">Addresses</h1>
		<p class="dashboard__page-subtitle">Manage your billing and shipping addresses</p>
	</header>

	<div class="dashboard__content">
		<div class="dashboard__content-main">
			{#if !isEditing}
				<div class="addresses-grid">
					<!-- Billing Address -->
					<div class="address-card">
						<div class="address-card__header">
							<h3>Billing Address</h3>
							{#if addresses.find(a => a.type === 'billing')}
								<button class="edit-btn" onclick={() => handleEdit(addresses.find(a => a.type === 'billing')!)}>
									<IconEdit size={16} />
									Edit
								</button>
							{/if}
						</div>
						<div class="address-card__body">
							{#if addresses.find(a => a.type === 'billing')}
								{@const addr = addresses.find(a => a.type === 'billing')!}
								<p class="address-name">{addr.name}</p>
								{#if addr.company}<p>{addr.company}</p>{/if}
								<p>{addr.address1}</p>
								{#if addr.address2}<p>{addr.address2}</p>{/if}
								<p>{addr.city}, {addr.state} {addr.zip}</p>
								<p>{addr.country}</p>
								{#if addr.phone}<p class="address-phone">{addr.phone}</p>{/if}
							{:else}
								<p class="no-address">No billing address set</p>
								<button class="add-btn">
									<IconPlus size={16} />
									Add Address
								</button>
							{/if}
						</div>
					</div>

					<!-- Shipping Address -->
					<div class="address-card">
						<div class="address-card__header">
							<h3>Shipping Address</h3>
						</div>
						<div class="address-card__body">
							<p class="no-address">Digital products only - no shipping address required</p>
						</div>
					</div>
				</div>
			{:else if editingAddress}
				<!-- Edit Form -->
				<div class="edit-form">
					<h3>Edit Billing Address</h3>
					<form onsubmit={(e) => { e.preventDefault(); handleSave(); }}>
						<div class="form-row">
							<div class="form-group">
								<label for="name">Full Name *</label>
								<input type="text" id="name" bind:value={editingAddress.name} required />
							</div>
							<div class="form-group">
								<label for="company">Company (optional)</label>
								<input type="text" id="company" bind:value={editingAddress.company} />
							</div>
						</div>
						<div class="form-group">
							<label for="address1">Address Line 1 *</label>
							<input type="text" id="address1" bind:value={editingAddress.address1} required />
						</div>
						<div class="form-group">
							<label for="address2">Address Line 2 (optional)</label>
							<input type="text" id="address2" bind:value={editingAddress.address2} />
						</div>
						<div class="form-row">
							<div class="form-group">
								<label for="city">City *</label>
								<input type="text" id="city" bind:value={editingAddress.city} required />
							</div>
							<div class="form-group">
								<label for="state">State *</label>
								<input type="text" id="state" bind:value={editingAddress.state} required />
							</div>
							<div class="form-group">
								<label for="zip">ZIP Code *</label>
								<input type="text" id="zip" bind:value={editingAddress.zip} required />
							</div>
						</div>
						<div class="form-row">
							<div class="form-group">
								<label for="country">Country *</label>
								<select id="country" bind:value={editingAddress.country}>
									<option value="United States">United States</option>
									<option value="Canada">Canada</option>
									<option value="United Kingdom">United Kingdom</option>
								</select>
							</div>
							<div class="form-group">
								<label for="phone">Phone (optional)</label>
								<input type="tel" id="phone" bind:value={editingAddress.phone} />
							</div>
						</div>
						<div class="form-actions">
							<button type="button" class="cancel-btn" onclick={handleCancel}>Cancel</button>
							<button type="submit" class="save-btn">Save Address</button>
						</div>
					</form>
				</div>
			{/if}
		</div>
	</div>
{:else}
	<div class="loading-state"><p>Redirecting to login...</p></div>
{/if}

<style>
	.dashboard__header { background: #fff; padding: 30px; border-bottom: 1px solid #ededed; }
	.dashboard__page-title { font-size: 32px; font-weight: 700; color: #333; margin: 0 0 8px; font-family: 'Open Sans Condensed', 'Open Sans', sans-serif; }
	.dashboard__page-subtitle { font-size: 15px; color: #666; margin: 0; }
	.dashboard__content { padding: 30px; background: #fff; min-height: 400px; }
	.dashboard__content-main { max-width: 900px; }

	.addresses-grid { display: grid; grid-template-columns: repeat(2, 1fr); gap: 24px; }
	.address-card { border: 1px solid #ededed; border-radius: 8px; overflow: hidden; }
	.address-card__header { display: flex; justify-content: space-between; align-items: center; padding: 16px 20px; background: #f8f8f8; border-bottom: 1px solid #ededed; }
	.address-card__header h3 { margin: 0; font-size: 16px; font-weight: 600; color: #333; }
	.address-card__body { padding: 20px; }
	.address-card__body p { margin: 0 0 4px; color: #333; font-size: 14px; }
	.address-name { font-weight: 600; }
	.address-phone { margin-top: 12px; color: #666; }
	.no-address { color: #999; font-style: italic; }

	.edit-btn { display: inline-flex; align-items: center; gap: 6px; padding: 6px 12px; background: transparent; border: 1px solid #dbdbdb; border-radius: 6px; color: #666; font-size: 13px; cursor: pointer; transition: all 0.15s; }
	.edit-btn:hover { border-color: #0984ae; color: #0984ae; }
	.add-btn { display: inline-flex; align-items: center; gap: 6px; margin-top: 12px; padding: 8px 16px; background: #0984ae; border: none; border-radius: 6px; color: #fff; font-size: 13px; cursor: pointer; }

	.edit-form { max-width: 600px; }
	.edit-form h3 { margin: 0 0 24px; font-size: 20px; color: #333; }
	.form-row { display: grid; grid-template-columns: repeat(auto-fit, minmax(min(100%, 150px), 1fr)); gap: 16px; }
	.form-group { margin-bottom: 16px; }
	.form-group label { display: block; margin-bottom: 6px; font-size: 13px; font-weight: 500; color: #333; }
	.form-group input, .form-group select { width: 100%; padding: 10px 12px; border: 1px solid #dbdbdb; border-radius: 6px; font-size: 14px; color: #333; background: #fff; }
	.form-group input:focus, .form-group select:focus { outline: none; border-color: #0984ae; }
	.form-actions { display: flex; gap: 12px; margin-top: 24px; }
	.cancel-btn { padding: 10px 20px; background: #f4f4f4; border: 1px solid #dbdbdb; border-radius: 6px; color: #333; font-weight: 500; cursor: pointer; }
	.save-btn { padding: 10px 20px; background: #0984ae; border: none; border-radius: 6px; color: #fff; font-weight: 500; cursor: pointer; }
	.save-btn:hover { background: #076787; }

	.loading-state { display: flex; align-items: center; justify-content: center; min-height: 300px; color: #666; }

	@media (max-width: 768px) {
		.dashboard__header { padding: 20px; }
		.dashboard__page-title { font-size: 26px; }
		.dashboard__content { padding: 20px; }
		.addresses-grid { grid-template-columns: 1fr; }
	}
</style>

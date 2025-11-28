<script lang="ts">
	import { authStore, isAuthenticated } from '$lib/stores/auth';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import apiClient, { type Membership, type Product } from '$lib/api/client';

	// Only redirect on client-side - use replaceState to prevent history pollution
	$: if (browser && !$isAuthenticated && !$authStore.isLoading && !$authStore.isInitializing) {
		goto('/login?redirect=/account', { replaceState: true });
	}

	let memberships: Membership[] = [];
	let products: Product[] = [];
	let loadingMemberships = true;
	let loadingProducts = true;

	onMount(async () => {
		if ($authStore.user) {
			try {
				memberships = await apiClient.getMyMemberships();
			} catch (error) {
				console.error('Failed to load memberships:', error);
			} finally {
				loadingMemberships = false;
			}

			try {
				products = await apiClient.getMyProducts();
			} catch (error) {
				console.error('Failed to load products:', error);
			} finally {
				loadingProducts = false;
			}
		}
	});

	async function handleLogout() {
		await authStore.logout();
		// Use replaceState so back button doesn't return to protected page
		goto('/', { replaceState: true });
	}
</script>

<svelte:head>
	<title>My Account - Revolution Trading Pros</title>
	<meta name="description" content="Manage your Revolution Trading Pros account" />
</svelte:head>

{#if $authStore.user}
	<div class="min-h-[calc(100vh-120px)] bg-rtp-bg py-12 px-4">
		<div class="max-w-7xl mx-auto">
			<!-- Header -->
			<div class="bg-rtp-surface rounded-2xl shadow-lg p-8 border border-rtp-border mb-8">
				<div class="flex flex-col md:flex-row justify-between items-start md:items-center gap-4">
					<div>
						<h1 class="text-3xl font-heading font-bold text-rtp-text mb-2">My Account</h1>
						<p class="text-rtp-muted">Welcome back, {$authStore.user.name}!</p>
					</div>
					<button
						on:click={handleLogout}
						class="px-6 py-2 border-2 border-rtp-primary text-rtp-primary rounded-lg font-semibold hover:bg-rtp-primary hover:text-white transition-all"
					>
						Sign Out
					</button>
				</div>
			</div>

			<div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
				<!-- Left Column: Profile Info -->
				<div class="lg:col-span-1 space-y-6">
					<!-- Profile Card -->
					<div class="bg-rtp-surface rounded-2xl shadow-lg p-6 border border-rtp-border">
						<h2 class="text-xl font-heading font-bold text-rtp-text mb-4">Profile Information</h2>
						<div class="space-y-4">
							<div>
								<span class="text-sm font-semibold text-rtp-muted">Full Name</span>
								<p class="text-rtp-text mt-1">{$authStore.user.name}</p>
							</div>
							<div>
								<span class="text-sm font-semibold text-rtp-muted">Email Address</span>
								<p class="text-rtp-text mt-1">{$authStore.user.email}</p>
							</div>
							<div>
								<span class="text-sm font-semibold text-rtp-muted">Account Status</span>
								<p class="mt-1">
									{#if $authStore.user.email_verified_at}
										<span
											class="inline-flex items-center px-3 py-1 rounded-full text-xs font-semibold bg-green-100 text-green-800"
										>
											<svg class="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
												<path
													fill-rule="evenodd"
													d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
													clip-rule="evenodd"
												/>
											</svg>
											Verified
										</span>
									{:else}
										<span
											class="inline-flex items-center px-3 py-1 rounded-full text-xs font-semibold bg-yellow-100 text-yellow-800"
										>
											<svg class="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
												<path
													fill-rule="evenodd"
													d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z"
													clip-rule="evenodd"
												/>
											</svg>
											Pending Verification
										</span>
									{/if}
								</p>
							</div>
							<div>
								<span class="text-sm font-semibold text-rtp-muted">Member Since</span>
								<p class="text-rtp-text mt-1">
									{new Date($authStore.user.created_at).toLocaleDateString('en-US', {
										month: 'long',
										year: 'numeric'
									})}
								</p>
							</div>
						</div>
						<button
							class="mt-6 w-full px-4 py-2 border border-rtp-border rounded-lg font-semibold text-rtp-text hover:bg-rtp-bg transition-all"
						>
							Edit Profile
						</button>
					</div>

					<!-- Quick Actions -->
					<div class="bg-rtp-surface rounded-2xl shadow-lg p-6 border border-rtp-border">
						<h3 class="text-lg font-heading font-bold text-rtp-text mb-4">Quick Actions</h3>
						<div class="space-y-2">
							<a
								href="/live-trading-rooms/day-trading"
								class="block px-4 py-3 rounded-lg hover:bg-rtp-bg transition-all text-rtp-text"
							>
								<div class="flex items-center">
									<svg
										class="w-5 h-5 mr-3 text-rtp-primary"
										fill="none"
										stroke="currentColor"
										viewBox="0 0 24 24"
									>
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"
										/>
									</svg>
									Join Live Room
								</div>
							</a>
							<a
								href="/indicators"
								class="block px-4 py-3 rounded-lg hover:bg-rtp-bg transition-all text-rtp-text"
							>
								<div class="flex items-center">
									<svg
										class="w-5 h-5 mr-3 text-rtp-primary"
										fill="none"
										stroke="currentColor"
										viewBox="0 0 24 24"
									>
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M7 12l3-3 3 3 4-4M8 21l4-4 4 4M3 4h18M4 4h16v12a1 1 0 01-1 1H5a1 1 0 01-1-1V4z"
										/>
									</svg>
									Browse Indicators
								</div>
							</a>
							<a
								href="/courses"
								class="block px-4 py-3 rounded-lg hover:bg-rtp-bg transition-all text-rtp-text"
							>
								<div class="flex items-center">
									<svg
										class="w-5 h-5 mr-3 text-rtp-primary"
										fill="none"
										stroke="currentColor"
										viewBox="0 0 24 24"
									>
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"
										/>
									</svg>
									View Courses
								</div>
							</a>
						</div>
					</div>
				</div>

				<!-- Right Column: Memberships & Products -->
				<div class="lg:col-span-2 space-y-6">
					<!-- Active Memberships -->
					<div class="bg-rtp-surface rounded-2xl shadow-lg p-6 border border-rtp-border">
						<h2 class="text-xl font-heading font-bold text-rtp-text mb-4">Active Memberships</h2>
						{#if loadingMemberships}
							<div class="flex justify-center py-8">
								<svg
									class="animate-spin h-8 w-8 text-rtp-primary"
									xmlns="http://www.w3.org/2000/svg"
									fill="none"
									viewBox="0 0 24 24"
								>
									<circle
										class="opacity-25"
										cx="12"
										cy="12"
										r="10"
										stroke="currentColor"
										stroke-width="4"
									></circle>
									<path
										class="opacity-75"
										fill="currentColor"
										d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
									></path>
								</svg>
							</div>
						{:else if memberships.length === 0}
							<div class="text-center py-12">
								<svg
									class="w-16 h-16 mx-auto text-rtp-muted mb-4"
									fill="none"
									stroke="currentColor"
									viewBox="0 0 24 24"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4"
									/>
								</svg>
								<h3 class="text-lg font-semibold text-rtp-text mb-2">No Active Memberships</h3>
								<p class="text-rtp-muted mb-4">Unlock premium features with a membership plan</p>
								<a
									href="/live-trading-rooms/day-trading"
									class="inline-block px-6 py-3 bg-gradient-to-r from-rtp-primary to-rtp-blue text-white font-bold rounded-lg hover:shadow-lg transition-all"
								>
									View Membership Plans
								</a>
							</div>
						{:else}
							<div class="space-y-4">
								{#each memberships as membership}
									<div class="border border-rtp-border rounded-lg p-4">
										<div class="flex justify-between items-start">
											<div>
												<h3 class="text-lg font-bold text-rtp-text">{membership.plan.name}</h3>
												<p class="text-sm text-rtp-muted mt-1">
													Started: {new Date(membership.starts_at).toLocaleDateString()}
												</p>
												{#if membership.expires_at}
													<p class="text-sm text-rtp-muted">
														Expires: {new Date(membership.expires_at).toLocaleDateString()}
													</p>
												{/if}
											</div>
											<span
												class="px-3 py-1 rounded-full text-xs font-semibold bg-green-100 text-green-800"
											>
												{membership.status}
											</span>
										</div>
									</div>
								{/each}
							</div>
						{/if}
					</div>

					<!-- My Products -->
					<div class="bg-rtp-surface rounded-2xl shadow-lg p-6 border border-rtp-border">
						<h2 class="text-xl font-heading font-bold text-rtp-text mb-4">My Products</h2>
						{#if loadingProducts}
							<div class="flex justify-center py-8">
								<svg
									class="animate-spin h-8 w-8 text-rtp-primary"
									xmlns="http://www.w3.org/2000/svg"
									fill="none"
									viewBox="0 0 24 24"
								>
									<circle
										class="opacity-25"
										cx="12"
										cy="12"
										r="10"
										stroke="currentColor"
										stroke-width="4"
									></circle>
									<path
										class="opacity-75"
										fill="currentColor"
										d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
									></path>
								</svg>
							</div>
						{:else if products.length === 0}
							<div class="text-center py-12">
								<svg
									class="w-16 h-16 mx-auto text-rtp-muted mb-4"
									fill="none"
									stroke="currentColor"
									viewBox="0 0 24 24"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z"
									/>
								</svg>
								<h3 class="text-lg font-semibold text-rtp-text mb-2">No Products Yet</h3>
								<p class="text-rtp-muted mb-4">Browse our courses and indicators to get started</p>
								<div class="flex gap-4 justify-center">
									<a
										href="/courses"
										class="px-6 py-3 bg-gradient-to-r from-rtp-primary to-rtp-blue text-white font-bold rounded-lg hover:shadow-lg transition-all"
									>
										Browse Courses
									</a>
									<a
										href="/indicators"
										class="px-6 py-3 border-2 border-rtp-primary text-rtp-primary font-bold rounded-lg hover:bg-rtp-primary hover:text-white transition-all"
									>
										View Indicators
									</a>
								</div>
							</div>
						{:else}
							<div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
								{#each products as product}
									<div
										class="border border-rtp-border rounded-lg p-4 hover:shadow-lg transition-all"
									>
										<div class="flex items-start justify-between mb-2">
											<h3 class="text-lg font-bold text-rtp-text">{product.name}</h3>
											<span
												class="px-2 py-1 rounded text-xs font-semibold bg-rtp-primarySoft text-rtp-primary capitalize"
											>
												{product.type}
											</span>
										</div>
										<p class="text-sm text-rtp-muted mb-4">{product.description}</p>
										<a
											href={product.type === 'indicator'
												? `/indicators/${product.slug}`
												: `/courses/${product.slug}`}
											class="inline-block px-4 py-2 bg-rtp-primary text-white font-semibold rounded-lg hover:bg-rtp-blue transition-all text-sm"
										>
											View Details
										</a>
									</div>
								{/each}
							</div>
						{/if}
					</div>
				</div>
			</div>
		</div>
	</div>
{/if}

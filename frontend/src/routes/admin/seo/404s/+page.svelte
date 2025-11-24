<script lang="ts">
	import { onMount } from 'svelte';
	import { Card, Button, Badge, Table } from '$lib/components/ui';
	import { addToast } from '$lib/utils/toast';
	import { seoApi, type Error404 } from '$lib/api/seo';
	import { IconAlertCircle, IconTrash } from '@tabler/icons-svelte';

	let errors: Error404[] = [];
	let loading = true;
	let stats = {
		total: 0,
		resolved: 0,
		unresolved: 0,
		total_hits: 0
	};

	onMount(async () => {
		await loadErrors();
		await loadStats();
	});

	async function loadErrors() {
		try {
			loading = true;
			// list404s returns Error404[] directly
			errors = await seoApi.list404s() || [];
		} catch (error) {
			addToast({ type: 'error', message: 'Failed to load 404 errors' });
		} finally {
			loading = false;
		}
	}

	async function loadStats() {
		try {
			const response = await seoApi.get404Stats();
			stats = response as any;
		} catch (error) {
			console.error('Failed to load stats:', error);
		}
	}

	async function handleBulkDelete(resolvedOnly: boolean = true) {
		const message = resolvedOnly
			? 'Delete all resolved 404 errors?'
			: 'Delete all 404 errors? This action cannot be undone.';

		if (!confirm(message)) return;

		try {
			await seoApi.bulkDelete404s(resolvedOnly);
			addToast({ type: 'success', message: '404 errors deleted successfully' });
			await loadErrors();
			await loadStats();
		} catch (error) {
			addToast({ type: 'error', message: 'Failed to delete 404 errors' });
		}
	}
</script>

<svelte:head>
	<title>404 Errors | Revolution Admin</title>
</svelte:head>

<div>
	<!-- Header -->
	<div class="flex items-center justify-between mb-6">
		<div>
			<h1 class="text-3xl font-bold text-gray-900">404 Error Monitor</h1>
			<p class="text-gray-600 mt-1">Track and fix broken links on your site</p>
		</div>
		<Button variant="danger" on:click={() => handleBulkDelete(true)}>
			<IconTrash size={20} />
			Delete Resolved
		</Button>
	</div>

	<!-- Stats -->
	<div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
		<Card>
			<p class="text-sm text-gray-600">Total 404s</p>
			<p class="text-2xl font-bold mt-1">{stats.total}</p>
		</Card>
		<Card>
			<p class="text-sm text-gray-600">Unresolved</p>
			<p class="text-2xl font-bold mt-1 text-red-600">{stats.unresolved}</p>
		</Card>
		<Card>
			<p class="text-sm text-gray-600">Resolved</p>
			<p class="text-2xl font-bold mt-1 text-green-600">{stats.resolved}</p>
		</Card>
		<Card>
			<p class="text-sm text-gray-600">Total Hits</p>
			<p class="text-2xl font-bold mt-1">{stats.total_hits}</p>
		</Card>
	</div>

	<!-- Errors List -->
	{#if loading}
		<Card>
			<div class="text-center py-12">
				<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto"></div>
				<p class="mt-4 text-gray-600">Loading 404 errors...</p>
			</div>
		</Card>
	{:else if errors.length === 0}
		<Card>
			<div class="text-center py-12">
				<IconAlertCircle size={48} class="mx-auto text-green-500 mb-4" />
				<p class="text-gray-500">No 404 errors found. Great job!</p>
			</div>
		</Card>
	{:else}
		<Card padding={false}>
			<Table headers={['URL', 'Hit Count', 'Status', 'First Seen', 'Last Seen']}>
				{#each errors as error}
					<tr>
						<td class="font-mono text-sm max-w-md truncate">{error.url}</td>
						<td>
							<Badge variant={error.hit_count > 10 ? 'danger' : 'warning'}>
								{error.hit_count} hits
							</Badge>
						</td>
						<td>
							<Badge variant={error.is_resolved ? 'success' : 'danger'}>
								{error.is_resolved ? 'Resolved' : 'Active'}
							</Badge>
						</td>
						<td class="text-gray-600 text-sm">
							{new Date(error.first_seen_at).toLocaleDateString()}
						</td>
						<td class="text-gray-600 text-sm">
							{new Date(error.last_seen_at).toLocaleDateString()}
						</td>
					</tr>
				{/each}
			</Table>
		</Card>

		<!-- Suggested Actions -->
		<Card class="mt-6">
			<h3 class="text-lg font-bold text-gray-900 mb-4">Recommended Actions</h3>
			<ul class="space-y-2 text-gray-700">
				<li class="flex items-start gap-2">
					<span class="text-blue-600 mt-1">→</span>
					<span>Review high-traffic 404s and create redirects to relevant content</span>
				</li>
				<li class="flex items-start gap-2">
					<span class="text-blue-600 mt-1">→</span>
					<span>Check internal links that may be pointing to these broken URLs</span>
				</li>
				<li class="flex items-start gap-2">
					<span class="text-blue-600 mt-1">→</span>
					<span>Consider creating content for frequently accessed missing pages</span>
				</li>
			</ul>
		</Card>
	{/if}
</div>

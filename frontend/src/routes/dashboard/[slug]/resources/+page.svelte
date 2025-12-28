<script lang="ts">
	import IconDownload from '@tabler/icons-svelte/icons/download';
	import IconFileText from '@tabler/icons-svelte/icons/file-text';
	let { data }: { data: any } = $props();
	const room = $derived(data.room);
	const slug = $derived(data.slug);

	const resources = [
		{ id: 1, title: 'Trading Room Rules', description: 'Guidelines for the trading room.', type: 'PDF', downloadUrl: '/trading-room-rules.pdf' },
		{ id: 2, title: 'Options Cheat Sheet', description: 'Quick reference for options.', type: 'PDF', downloadUrl: '/options-cheat-sheet.pdf' },
		{ id: 3, title: 'Trade Journal Template', description: 'Track your trades.', type: 'Excel', downloadUrl: '/trade-journal.xlsx' },
		{ id: 4, title: 'Platform Setup Guide', description: 'Set up your trading platform.', type: 'PDF', downloadUrl: '/platform-setup.pdf' }
	];
</script>

<svelte:head><title>Resources - {room.name} | Revolution Trading Pros</title></svelte:head>

<div class="resources">
	<header class="page-header"><h1 class="page-title">{room.name} Resources</h1><p class="page-subtitle">Download helpful guides and reference materials.</p></header>
	<div class="resources-grid">
		{#each resources as resource}
			<article class="resource-card">
				<div class="resource-icon"><IconFileText size={28} /></div>
				<div class="resource-content">
					<h3 class="resource-title">{resource.title}</h3>
					<p class="resource-description">{resource.description}</p>
					<div class="resource-meta">
						<span class="resource-type">{resource.type}</span>
						<a href={resource.downloadUrl} class="download-btn" download><IconDownload size={16} /> Download</a>
					</div>
				</div>
			</article>
		{/each}
	</div>
</div>

<style>
	.resources { padding: 30px; max-width: 1200px; margin: 0 auto; }
	.page-header { margin-bottom: 40px; }
	.page-title { font-size: 28px; font-weight: 700; color: #333; margin: 0 0 8px; }
	.page-subtitle { font-size: 14px; color: #666; margin: 0; }
	.resources-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(350px, 1fr)); gap: 20px; }
	.resource-card { display: flex; gap: 20px; background: #fff; border-radius: 12px; padding: 24px; box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1); }
	.resource-icon { flex-shrink: 0; display: flex; align-items: center; justify-content: center; width: 56px; height: 56px; background: #f0f7fa; border-radius: 12px; color: #0984ae; }
	.resource-content { flex: 1; }
	.resource-title { font-size: 16px; font-weight: 700; color: #333; margin: 0 0 8px; }
	.resource-description { font-size: 14px; color: #666; margin: 0 0 16px; }
	.resource-meta { display: flex; align-items: center; justify-content: space-between; }
	.resource-type { padding: 4px 10px; background: #f0f0f0; color: #666; border-radius: 4px; font-size: 12px; font-weight: 600; }
	.download-btn { display: inline-flex; align-items: center; gap: 6px; padding: 8px 16px; background: #0984ae; color: #fff; text-decoration: none; border-radius: 4px; font-size: 13px; font-weight: 600; }
</style>

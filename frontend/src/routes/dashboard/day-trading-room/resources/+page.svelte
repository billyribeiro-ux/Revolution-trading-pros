<script lang="ts">
	/**
	 * Day Trading Room - Resources
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Trading guides, tools, and downloadable resources
	 *
	 * @version 1.0.0
	 */
	import IconArrowLeft from '@tabler/icons-svelte/icons/arrow-left';
	import IconBook from '@tabler/icons-svelte/icons/book';
	import IconDownload from '@tabler/icons-svelte/icons/download';
	import IconFileText from '@tabler/icons-svelte/icons/file-text';
	import IconChartLine from '@tabler/icons-svelte/icons/chart-line';

	const resources = [
		{
			category: 'Trading Guides',
			items: [
				{ title: 'Day Trading Starter Guide', type: 'PDF', size: '2.4 MB' },
				{ title: 'Risk Management Handbook', type: 'PDF', size: '1.8 MB' },
				{ title: 'Chart Patterns Reference', type: 'PDF', size: '3.2 MB' }
			]
		},
		{
			category: 'Tools & Templates',
			items: [
				{ title: 'Trading Journal Template', type: 'Excel', size: '156 KB' },
				{ title: 'Position Size Calculator', type: 'Excel', size: '89 KB' },
				{ title: 'Watchlist Template', type: 'Excel', size: '124 KB' }
			]
		},
		{
			category: 'Market Analysis',
			items: [
				{ title: 'Weekly Market Outlook', type: 'PDF', size: '1.2 MB' },
				{ title: 'Sector Rotation Guide', type: 'PDF', size: '980 KB' },
				{ title: 'Economic Calendar', type: 'PDF', size: '450 KB' }
			]
		}
	];
</script>

<svelte:head>
	<title>Resources - Day Trading Room | Revolution Trading Pros</title>
</svelte:head>

<div class="resources-page">
	<a href="/dashboard/day-trading-room" class="back-link">
		<IconArrowLeft size={18} />
		Back to Day Trading Room
	</a>

	<header class="page-header">
		<div class="header-icon">
			<IconBook size={32} />
		</div>
		<div class="header-content">
			<h1 class="page-title">Resources</h1>
			<p class="page-subtitle">Download trading guides, tools, and reference materials</p>
		</div>
	</header>

	<div class="content-wrapper">
		{#each resources as resourceCategory}
			<section class="resource-category">
				<h2 class="category-title">{resourceCategory.category}</h2>
				<div class="resources-list">
					{#each resourceCategory.items as item}
						<article class="resource-item">
							<div class="resource-icon">
								{#if item.type === 'PDF'}
									<IconFileText size={24} />
								{:else if item.type === 'Excel'}
									<IconChartLine size={24} />
								{:else}
									<IconBook size={24} />
								{/if}
							</div>
							<div class="resource-info">
								<h3 class="resource-title">{item.title}</h3>
								<div class="resource-meta">
									<span class="resource-type">{item.type}</span>
									<span class="resource-size">{item.size}</span>
								</div>
							</div>
							<button class="download-btn">
								<IconDownload size={20} />
								Download
							</button>
						</article>
					{/each}
				</div>
			</section>
		{/each}
	</div>
</div>

<style>
	.resources-page {
		padding: 30px;
		max-width: 1200px;
		margin: 0 auto;
	}

	.back-link {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		color: #1e73be;
		text-decoration: none;
		font-size: 14px;
		font-weight: 500;
		margin-bottom: 24px;
	}

	.back-link:hover {
		text-decoration: underline;
	}

	.page-header {
		display: flex;
		align-items: center;
		gap: 20px;
		margin-bottom: 40px;
	}

	.header-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 64px;
		height: 64px;
		background: linear-gradient(135deg, #0984ae 0%, #076787 100%);
		border-radius: 16px;
		color: #fff;
	}

	.page-title {
		font-size: 28px;
		font-weight: 600;
		color: #333;
		margin: 0 0 4px;
	}

	.page-subtitle {
		font-size: 14px;
		color: #666;
		margin: 0;
	}

	.resource-category {
		margin-bottom: 40px;
	}

	.category-title {
		font-size: 20px;
		font-weight: 600;
		color: #333;
		margin: 0 0 20px;
	}

	.resources-list {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.resource-item {
		display: flex;
		align-items: center;
		gap: 16px;
		padding: 20px;
		background: #fff;
		border-radius: 8px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
		transition: all 0.15s;
	}

	.resource-item:hover {
		box-shadow: 0 4px 16px rgba(0, 0, 0, 0.12);
	}

	.resource-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 48px;
		height: 48px;
		background: linear-gradient(135deg, #0984ae 0%, #076787 100%);
		border-radius: 12px;
		color: #fff;
		flex-shrink: 0;
	}

	.resource-info {
		flex: 1;
		min-width: 0;
	}

	.resource-title {
		font-size: 16px;
		font-weight: 600;
		color: #333;
		margin: 0 0 4px;
	}

	.resource-meta {
		display: flex;
		gap: 12px;
		font-size: 13px;
		color: #666;
	}

	.resource-type {
		font-weight: 500;
		color: #0984ae;
	}

	.download-btn {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 20px;
		background: #0984ae;
		color: #fff;
		border: none;
		border-radius: 6px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.15s;
		flex-shrink: 0;
	}

	.download-btn:hover {
		background: #076787;
	}

	@media (max-width: 768px) {
		.resources-page {
			padding: 20px;
		}

		.resource-item {
			flex-wrap: wrap;
		}

		.download-btn {
			width: 100%;
			justify-content: center;
		}
	}
</style>

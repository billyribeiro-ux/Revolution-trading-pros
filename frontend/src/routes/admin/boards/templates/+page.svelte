<script lang="ts">
	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import { boardsAPI } from '$lib/api/boards';
	import type { BoardTemplate } from '$lib/boards/types';
	import {
		IconTemplate,
		IconArrowLeft,
		IconSearch,
		IconPlus,
		IconLayoutKanban,
		IconCode,
		IconBriefcase,
		IconUsers,
		IconRocket,
		IconBook,
		IconChartBar
	} from '$lib/icons';

	// State
	let templates = $state<BoardTemplate[]>([]);
	let loading = $state(true);
	let searchQuery = $state('');
	let selectedCategory = $state<string | null>(null);
	let creatingBoard = $state<string | null>(null);

	// Categories
	const categories = [
		{ id: 'software', name: 'Software Development', icon: IconCode },
		{ id: 'marketing', name: 'Marketing', icon: IconChartBar },
		{ id: 'business', name: 'Business', icon: IconBriefcase },
		{ id: 'team', name: 'Team Management', icon: IconUsers },
		{ id: 'product', name: 'Product Launch', icon: IconRocket },
		{ id: 'education', name: 'Education', icon: IconBook }
	];

	// Default templates (in case API doesn't return any)
	const defaultTemplates: BoardTemplate[] = [
		{
			id: 'kanban-basic',
			title: 'Basic Kanban',
			description: 'Simple kanban board with To Do, In Progress, and Done columns',
			category: 'team',
			stages: [
				{ title: 'To Do', color: '#6b7280', position: 1, auto_complete: false },
				{ title: 'In Progress', color: '#3b82f6', position: 2, auto_complete: false },
				{ title: 'Done', color: '#22c55e', position: 3, auto_complete: true }
			],
			labels: [
				{ title: 'Bug', color: '#ef4444' },
				{ title: 'Feature', color: '#B38F00' },
				{ title: 'Enhancement', color: '#06b6d4' }
			],
			custom_fields: [],
			is_default: true,
			created_at: new Date().toISOString()
		},
		{
			id: 'software-sprint',
			title: 'Software Sprint',
			description: 'Agile sprint board with backlog, development, review, and deployment stages',
			category: 'software',
			stages: [
				{ title: 'Backlog', color: '#6b7280', position: 1, auto_complete: false },
				{ title: 'Sprint Planning', color: '#B38F00', position: 2, auto_complete: false },
				{ title: 'In Development', color: '#3b82f6', position: 3, auto_complete: false },
				{ title: 'Code Review', color: '#f59e0b', position: 4, auto_complete: false },
				{ title: 'QA Testing', color: '#ec4899', position: 5, auto_complete: false },
				{ title: 'Done', color: '#22c55e', position: 6, auto_complete: true }
			],
			labels: [
				{ title: 'Bug', color: '#ef4444' },
				{ title: 'Feature', color: '#B38F00' },
				{ title: 'Tech Debt', color: '#f59e0b' },
				{ title: 'Documentation', color: '#06b6d4' },
				{ title: 'Critical', color: '#dc2626' }
			],
			custom_fields: [],
			is_default: false,
			created_at: new Date().toISOString()
		},
		{
			id: 'content-pipeline',
			title: 'Content Pipeline',
			description: 'Track content from ideation to publication',
			category: 'marketing',
			stages: [
				{ title: 'Ideas', color: '#B38F00', position: 1, auto_complete: false },
				{ title: 'Research', color: '#3b82f6', position: 2, auto_complete: false },
				{ title: 'Writing', color: '#f59e0b', position: 3, auto_complete: false },
				{ title: 'Editing', color: '#ec4899', position: 4, auto_complete: false },
				{ title: 'Review', color: '#06b6d4', position: 5, auto_complete: false },
				{ title: 'Scheduled', color: '#10b981', position: 6, auto_complete: false },
				{ title: 'Published', color: '#22c55e', position: 7, auto_complete: true }
			],
			labels: [
				{ title: 'Blog', color: '#3b82f6' },
				{ title: 'Social', color: '#ec4899' },
				{ title: 'Video', color: '#ef4444' },
				{ title: 'Newsletter', color: '#B38F00' }
			],
			custom_fields: [],
			is_default: false,
			created_at: new Date().toISOString()
		},
		{
			id: 'course-creation',
			title: 'Course Creation',
			description: 'Manage online course development from planning to launch',
			category: 'education',
			stages: [
				{ title: 'Planning', color: '#6b7280', position: 1, auto_complete: false },
				{ title: 'Scripting', color: '#B38F00', position: 2, auto_complete: false },
				{ title: 'Recording', color: '#3b82f6', position: 3, auto_complete: false },
				{ title: 'Editing', color: '#f59e0b', position: 4, auto_complete: false },
				{ title: 'Review', color: '#ec4899', position: 5, auto_complete: false },
				{ title: 'Published', color: '#22c55e', position: 6, auto_complete: true }
			],
			labels: [
				{ title: 'Video', color: '#ef4444' },
				{ title: 'Quiz', color: '#B38F00' },
				{ title: 'Resource', color: '#06b6d4' },
				{ title: 'Assignment', color: '#f59e0b' }
			],
			custom_fields: [],
			is_default: false,
			created_at: new Date().toISOString()
		},
		{
			id: 'product-launch',
			title: 'Product Launch',
			description: 'Plan and execute a successful product launch',
			category: 'product',
			stages: [
				{ title: 'Planning', color: '#6b7280', position: 1, auto_complete: false },
				{ title: 'Development', color: '#3b82f6', position: 2, auto_complete: false },
				{ title: 'Testing', color: '#f59e0b', position: 3, auto_complete: false },
				{ title: 'Marketing Prep', color: '#ec4899', position: 4, auto_complete: false },
				{ title: 'Pre-Launch', color: '#B38F00', position: 5, auto_complete: false },
				{ title: 'Launched', color: '#22c55e', position: 6, auto_complete: true }
			],
			labels: [
				{ title: 'Must Have', color: '#ef4444' },
				{ title: 'Nice to Have', color: '#f59e0b' },
				{ title: 'Marketing', color: '#ec4899' },
				{ title: 'Technical', color: '#3b82f6' }
			],
			custom_fields: [],
			is_default: false,
			created_at: new Date().toISOString()
		},
		{
			id: 'crm-pipeline',
			title: 'Sales Pipeline',
			description: 'Track leads and deals through your sales process',
			category: 'business',
			stages: [
				{ title: 'Lead', color: '#6b7280', position: 1, auto_complete: false },
				{ title: 'Contacted', color: '#3b82f6', position: 2, auto_complete: false },
				{ title: 'Qualified', color: '#B38F00', position: 3, auto_complete: false },
				{ title: 'Proposal', color: '#f59e0b', position: 4, auto_complete: false },
				{ title: 'Negotiation', color: '#ec4899', position: 5, auto_complete: false },
				{ title: 'Won', color: '#22c55e', position: 6, auto_complete: true },
				{ title: 'Lost', color: '#ef4444', position: 7, auto_complete: false }
			],
			labels: [
				{ title: 'Hot Lead', color: '#ef4444' },
				{ title: 'Warm Lead', color: '#f59e0b' },
				{ title: 'Cold Lead', color: '#3b82f6' },
				{ title: 'Enterprise', color: '#B38F00' }
			],
			custom_fields: [],
			is_default: false,
			created_at: new Date().toISOString()
		}
	];

	$effect(() => {
		if (!browser) return;
		loadTemplates();
	});

	async function loadTemplates() {
		loading = true;
		try {
			const res = await boardsAPI.getTemplates();
			templates = res.length > 0 ? res : defaultTemplates;
		} catch (error) {
			console.error('Failed to load templates:', error);
			templates = defaultTemplates;
		} finally {
			loading = false;
		}
	}

	let filteredTemplates = $derived.by(() => {
		return templates.filter((t) => {
			if (searchQuery) {
				const query = searchQuery.toLowerCase();
				if (
					!t.title.toLowerCase().includes(query) &&
					!t.description.toLowerCase().includes(query)
				) {
					return false;
				}
			}
			if (selectedCategory && t.category !== selectedCategory) {
				return false;
			}
			return true;
		});
	});

	async function useTemplate(template: BoardTemplate) {
		creatingBoard = template.id;
		try {
			const board = await boardsAPI.createFromTemplate(template.id, template.title);
			goto(`/admin/boards/${board.id}`);
		} catch (error) {
			console.error('Failed to create board from template:', error);
			// Fallback: Create board manually with template config
			try {
				const board = await boardsAPI.createBoard({
					title: template.title,
					description: template.description,
					type: 'kanban'
				});

				// Create stages
				for (const stage of template.stages) {
					await boardsAPI.createStage(board.id, stage);
				}

				// Create labels
				for (const label of template.labels) {
					await boardsAPI.createLabel(board.id, label);
				}

				goto(`/admin/boards/${board.id}`);
			} catch (err) {
				console.error('Failed to create board:', err);
			}
		} finally {
			creatingBoard = null;
		}
	}

	function getCategoryIcon(categoryId: string) {
		const category = categories.find((c) => c.id === categoryId);
		return category?.icon || IconLayoutKanban;
	}
</script>

<svelte:head>
	<title>Board Templates | Project Boards</title>
</svelte:head>

<div class="bg-gray-50 dark:bg-gray-900">
	<!-- Header -->
	<div class="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
			<div class="flex items-center justify-between">
				<div class="flex items-center gap-4">
					<a
						href="/admin/boards"
						class="p-2 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg"
					>
						<IconArrowLeft class="w-5 h-5" />
					</a>
					<div class="flex items-center gap-3">
						<div class="p-2 bg-[#E6B800]/10 dark:bg-[#B38F00]/30 rounded-lg">
							<IconTemplate class="w-6 h-6 text-[#E6B800] dark:text-[#FFD11A]" />
						</div>
						<div>
							<h1 class="text-2xl font-bold text-gray-900 dark:text-white">Board Templates</h1>
							<p class="text-sm text-gray-500 dark:text-gray-400">
								Start with a pre-built board configuration
							</p>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>

	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
		<!-- Search and Filters -->
		<div class="flex flex-col md:flex-row gap-4 mb-8">
			<div class="flex-1 relative">
				<IconSearch class="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400" />
				<input
					id="page-searchquery" name="page-searchquery" type="text"
					placeholder="Search templates..."
					bind:value={searchQuery}
					class="w-full pl-10 pr-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:ring-2 focus:ring-[#E6B800]"
				/>
			</div>

			<div class="flex items-center gap-2 overflow-x-auto pb-2 md:pb-0">
				<button
					onclick={() => (selectedCategory = null)}
					class="px-4 py-2 text-sm whitespace-nowrap rounded-lg {selectedCategory === null
						? 'bg-[#E6B800] text-[#0D1117]'
						: 'bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 border border-gray-300 dark:border-gray-600 hover:bg-gray-50 dark:hover:bg-gray-700'}"
				>
					All
				</button>
				{#each categories as category}
					<button
						onclick={() => (selectedCategory = category.id)}
						class="px-4 py-2 text-sm whitespace-nowrap rounded-lg flex items-center gap-2 {selectedCategory ===
						category.id
							? 'bg-[#E6B800] text-[#0D1117]'
							: 'bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 border border-gray-300 dark:border-gray-600 hover:bg-gray-50 dark:hover:bg-gray-700'}"
					>
						<category.icon class="w-4 h-4" />
						{category.name}
					</button>
				{/each}
			</div>
		</div>

		<!-- Templates Grid -->
		{#if loading}
			<div class="flex items-center justify-center py-12">
				<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-[#E6B800]"></div>
			</div>
		{:else if filteredTemplates.length === 0}
			<div
				class="text-center py-12 bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700"
			>
				<IconTemplate class="w-12 h-12 text-gray-400 mx-auto mb-4" />
				<h3 class="text-lg font-medium text-gray-900 dark:text-white mb-2">No templates found</h3>
				<p class="text-gray-500 dark:text-gray-400">Try adjusting your search or filters</p>
			</div>
		{:else}
			<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
				{#each filteredTemplates as template}
					{@const CategoryIcon = getCategoryIcon(template.category)}
					<div
						class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 overflow-hidden hover:shadow-lg transition-shadow"
					>
						<!-- Preview Header -->
						<div class="h-32 bg-gradient-to-br from-[#E6B800] to-[#B38F00] p-4 flex items-end">
							<div class="flex gap-2">
								{#each template.stages.slice(0, 4) as stage}
									<div
										class="w-16 h-20 rounded-t-lg opacity-90"
										style="background-color: {stage.color}"
									>
										<div class="p-1.5">
											<div class="text-[8px] text-white font-medium truncate">{stage.title}</div>
											<div class="mt-1 space-y-1">
												<div class="h-1.5 bg-white/30 rounded"></div>
												<div class="h-1.5 bg-white/30 rounded w-3/4"></div>
											</div>
										</div>
									</div>
								{/each}
								{#if template.stages.length > 4}
									<div
										class="w-16 h-20 rounded-t-lg bg-gray-400/50 flex items-center justify-center"
									>
										<span class="text-white text-xs">+{template.stages.length - 4}</span>
									</div>
								{/if}
							</div>
						</div>

						<!-- Content -->
						<div class="p-4">
							<div class="flex items-center gap-2 mb-2">
								<CategoryIcon class="w-4 h-4 text-gray-400" />
								<span class="text-xs text-gray-500 dark:text-gray-400 capitalize"
									>{template.category}</span
								>
								{#if template.is_default}
									<span
										class="text-xs bg-[#E6B800]/10 dark:bg-[#B38F00]/30 text-[#E6B800] dark:text-[#FFD11A] px-2 py-0.5 rounded"
										>Default</span
									>
								{/if}
							</div>
							<h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-2">
								{template.title}
							</h3>
							<p class="text-sm text-gray-500 dark:text-gray-400 mb-4 line-clamp-2">
								{template.description}
							</p>

							<!-- Template Info -->
							<div class="flex items-center gap-4 text-xs text-gray-500 dark:text-gray-400 mb-4">
								<span>{template.stages.length} stages</span>
								<span>{template.labels.length} labels</span>
								{#if template.custom_fields.length > 0}
									<span>{template.custom_fields.length} custom fields</span>
								{/if}
							</div>

							<!-- Labels Preview -->
							<div class="flex flex-wrap gap-1 mb-4">
								{#each template.labels.slice(0, 4) as label}
									<span
										class="px-2 py-0.5 text-xs text-white rounded"
										style="background-color: {label.color}"
									>
										{label.title}
									</span>
								{/each}
								{#if template.labels.length > 4}
									<span class="text-xs text-gray-500">+{template.labels.length - 4}</span>
								{/if}
							</div>

							<!-- Action -->
							<button
								onclick={() => useTemplate(template)}
								disabled={creatingBoard === template.id}
								class="w-full px-4 py-2 bg-[#E6B800] hover:bg-[#B38F00] text-[#0D1117] rounded-lg flex items-center justify-center gap-2 disabled:opacity-50"
							>
								{#if creatingBoard === template.id}
									<div class="animate-spin rounded-full h-4 w-4 border-b-2 border-white"></div>
									Creating...
								{:else}
									<IconPlus class="w-4 h-4" />
									Use Template
								{/if}
							</button>
						</div>
					</div>
				{/each}
			</div>
		{/if}

		<!-- Create Custom Template -->
		<div
			class="mt-8 p-6 bg-[#E6B800]/5 dark:bg-[#B38F00]/20 border border-[#E6B800]/20 dark:border-[#B38F00] rounded-xl"
		>
			<div class="flex items-center justify-between">
				<div>
					<h3 class="text-lg font-semibold text-[#B38F00] dark:text-[#FFD11A]">
						Create Custom Template
					</h3>
					<p class="text-sm text-[#B38F00]/80 dark:text-[#FFD11A]/80">
						Save any board as a template to reuse later
					</p>
				</div>
				<a
					href="/admin/boards"
					class="px-4 py-2 bg-[#E6B800] hover:bg-[#B38F00] text-[#0D1117] rounded-lg"
				>
					Go to Boards
				</a>
			</div>
		</div>
	</div>
</div>

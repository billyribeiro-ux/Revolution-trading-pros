<script lang="ts">
	import { Card } from '$lib/components/ui';
	import type { ComponentType } from 'svelte';

	interface Props {
		title: string;
		value: string | number;
		icon: ComponentType;
		trend?: string;
		trendUp?: boolean;
		color?: 'blue' | 'green' | 'purple' | 'orange';
	}

	let {
		title,
		value,
		icon: Icon,
		trend = '',
		trendUp = true,
		color = 'blue'
	}: Props = $props();

	const colors = {
		blue: 'bg-blue-100 text-blue-600',
		green: 'bg-green-100 text-green-600',
		purple: 'bg-purple-100 text-purple-600',
		orange: 'bg-orange-100 text-orange-600'
	};
</script>

<Card hover>
	<div class="flex items-start justify-between">
		<div class="flex-1">
			<p class="text-sm font-medium text-gray-600">{title}</p>
			<p class="text-3xl font-bold text-gray-900 mt-2">{value}</p>
			{#if trend}
				<p class="text-sm mt-2 {trendUp ? 'text-green-600' : 'text-red-600'}">
					{trendUp ? '↑' : '↓'}
					{trend}
				</p>
			{/if}
		</div>
		<div class="p-3 rounded-lg {colors[color]}">
			<Icon size={24} />
		</div>
	</div>
</Card>

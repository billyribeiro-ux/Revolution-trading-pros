<!--
  OptimizationStats Component
  ═══════════════════════════════════════════════════════════════════════════

  Display image optimization statistics with visual feedback:
  - Original vs optimized size comparison
  - Percentage savings
  - Format conversions
  - Processing time

  @version 2.0.0
-->
<script lang="ts">
	import { fade, slide } from 'svelte/transition';
	import { tweened } from 'svelte/motion';
	import { cubicOut } from 'svelte/easing';
	import Icon from '$lib/components/Icon.svelte';

	interface Stats {
		originalSize: number;
		optimizedSize: number;
		savingsPercent: number;
		originalFormat?: string;
		optimizedFormat?: string;
		processingTime?: number;
		variantsCount?: number;
	}

	// Props
	let {
		stats,
		showDetails = true,
		compact = false,
		animated = true,
		className = ''
	}: {
		stats: Stats;
		showDetails?: boolean;
		compact?: boolean;
		animated?: boolean;
		className?: string;
	} = $props();

	// Animated savings
	const animatedSavings = tweened(0, {
		duration: 1000,
		easing: cubicOut
	});

	$effect(() => {
		const duration = animated ? 1000 : 0;
		animatedSavings.set(stats.savingsPercent, { duration });
	});

	// Format bytes
	function formatBytes(bytes: number): string {
		if (bytes === 0) return '0 B';
		const k = 1024;
		const sizes = ['B', 'KB', 'MB', 'GB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
	}

	// Get savings color
	function getSavingsColor(percent: number): string {
		if (percent >= 70) return 'text-green-500';
		if (percent >= 50) return 'text-green-400';
		if (percent >= 30) return 'text-yellow-500';
		return 'text-orange-500';
	}

	// Get ring color
	function getRingColor(percent: number): string {
		if (percent >= 70) return 'stroke-green-500';
		if (percent >= 50) return 'stroke-green-400';
		if (percent >= 30) return 'stroke-yellow-500';
		return 'stroke-orange-500';
	}
</script>

{#if compact}
	<!-- Compact inline version -->
	<div class="inline-flex items-center gap-2 text-sm {className}" transition:fade>
		<span class="text-gray-400 line-through">{formatBytes(stats.originalSize)}</span>
		<Icon name="IconArrowRight" size={16} class="text-gray-400" />
		<span class="font-medium text-gray-700 dark:text-gray-300"
			>{formatBytes(stats.optimizedSize)}</span
		>
		<span
			class="px-2 py-0.5 rounded-full text-xs font-semibold bg-green-100 dark:bg-green-900/30 {getSavingsColor(
				stats.savingsPercent
			)}"
		>
			-{Math.round($animatedSavings)}%
		</span>
	</div>
{:else}
	<!-- Full card version -->
	<div
		class="bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-xl overflow-hidden {className}"
		transition:slide
	>
		<!-- Success header -->
		<div
			class="flex items-center gap-3 p-4 bg-green-100/50 dark:bg-green-900/30 border-b border-green-200 dark:border-green-800"
		>
			<div class="w-10 h-10 rounded-full bg-green-500 text-white flex items-center justify-center">
				<Icon name="IconCheck" size={24} />
			</div>
			<div>
				<h3 class="text-lg font-semibold text-green-800 dark:text-green-200">
					Optimized Successfully
				</h3>
				{#if stats.processingTime}
					<p class="text-sm text-green-600 dark:text-green-400">
						Processed in {stats.processingTime}ms
					</p>
				{/if}
			</div>
		</div>

		<!-- Main stats -->
		<div class="flex items-center justify-around p-6 gap-6">
			<!-- Circular progress -->
			<div class="relative flex items-center justify-center">
				<svg aria-hidden="true" viewBox="0 0 100 100" class="w-24 h-24">
					<!-- Background ring -->
					<circle
						cx="50"
						cy="50"
						r="40"
						fill="none"
						stroke="currentColor"
						stroke-width="8"
						class="text-gray-200 dark:text-gray-700"
					/>
					<!-- Progress ring -->
					<circle
						cx="50"
						cy="50"
						r="40"
						fill="none"
						stroke-width="8"
						stroke-linecap="round"
						class="{getRingColor(stats.savingsPercent)} transition-all duration-1000"
						stroke-dasharray="{$animatedSavings * 2.51} 251"
						transform="rotate(-90 50 50)"
					/>
				</svg>
				<div class="absolute inset-0 flex flex-col items-center justify-center">
					<span class="text-2xl font-bold {getSavingsColor(stats.savingsPercent)}">
						{Math.round($animatedSavings)}%
					</span>
					<span class="text-xs text-gray-500">smaller</span>
				</div>
			</div>

			<!-- Size comparison -->
			<div class="flex flex-col items-center gap-2">
				<div class="flex items-center gap-2">
					<span class="text-xs text-gray-500 dark:text-gray-400 w-16">Original</span>
					<span class="text-sm text-gray-500 line-through">{formatBytes(stats.originalSize)}</span>
					{#if stats.originalFormat}
						<span
							class="px-1.5 py-0.5 text-[10px] font-medium rounded bg-gray-200 text-gray-600 dark:bg-gray-700 dark:text-gray-400"
							>{stats.originalFormat.toUpperCase()}</span
						>
					{/if}
				</div>
				<div class="py-1">
					<Icon name="IconArrowDown" size={20} class="text-green-500" />
				</div>
				<div class="flex items-center gap-2">
					<span class="text-xs text-gray-500 dark:text-gray-400 w-16">Optimized</span>
					<span class="text-sm text-green-600 dark:text-green-400 font-semibold"
						>{formatBytes(stats.optimizedSize)}</span
					>
					{#if stats.optimizedFormat}
						<span
							class="px-1.5 py-0.5 text-[10px] font-medium rounded bg-green-200 text-green-700 dark:bg-green-800 dark:text-green-300"
							>{stats.optimizedFormat.toUpperCase()}</span
						>
					{/if}
				</div>
			</div>
		</div>

		<!-- Additional details -->
		{#if showDetails}
			<div
				class="flex items-center justify-center gap-6 px-4 py-3 border-t border-green-200 dark:border-green-800 text-xs text-gray-600 dark:text-gray-400"
			>
				<div class="flex items-center gap-1.5">
					<Icon name="IconShieldCheck" size={16} class="text-gray-400" />
					<span>Lossless quality</span>
				</div>
				{#if stats.variantsCount && stats.variantsCount > 0}
					<div class="flex items-center gap-1.5">
						<Icon name="IconLayoutGrid" size={16} class="text-gray-400" />
						<span>{stats.variantsCount} responsive sizes</span>
					</div>
				{/if}
				<div class="flex items-center gap-1.5">
					<Icon name="IconBolt" size={16} class="text-gray-400" />
					<span>CDN-ready</span>
				</div>
			</div>
		{/if}
	</div>
{/if}

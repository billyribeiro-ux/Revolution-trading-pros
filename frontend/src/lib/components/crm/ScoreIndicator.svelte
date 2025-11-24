<script lang="ts">
	interface Props {
		score: number;
		label: string;
		size?: 'sm' | 'md' | 'lg';
	}

	let { score, label, size = 'md' }: Props = $props();

	function getScoreColor(score: number): string {
		if (score >= 75) return 'emerald';
		if (score >= 50) return 'sky';
		if (score >= 25) return 'amber';
		return 'rose';
	}

	function getScoreLabel(score: number): string {
		if (score >= 75) return 'Excellent';
		if (score >= 50) return 'Good';
		if (score >= 25) return 'Fair';
		return 'Poor';
	}

	let color = $derived(getScoreColor(score));
	let scoreLabel = $derived(getScoreLabel(score));
	let sizeClasses = $derived(
		{
			sm: { container: 'w-16 h-16', text: 'text-lg', label: 'text-[10px]' },
			md: { container: 'w-20 h-20', text: 'text-2xl', label: 'text-xs' },
			lg: { container: 'w-28 h-28', text: 'text-3xl', label: 'text-sm' }
		}[size]
	);
</script>

<div class="flex flex-col items-center gap-2">
	<div class="relative {sizeClasses.container}">
		<!-- Background Circle -->
		<svg class="absolute inset-0 -rotate-90" viewBox="0 0 100 100">
			<circle
				cx="50"
				cy="50"
				r="45"
				fill="none"
				stroke="currentColor"
				stroke-width="8"
				class="text-slate-800"
			/>
			<circle
				cx="50"
				cy="50"
				r="45"
				fill="none"
				stroke="currentColor"
				stroke-width="8"
				stroke-linecap="round"
				class="text-{color}-400 transition-all duration-500"
				style="stroke-dasharray: {2 * Math.PI * 45}; stroke-dashoffset: {2 *
					Math.PI *
					45 *
					(1 - score / 100)};"
			/>
		</svg>

		<!-- Score Text -->
		<div class="absolute inset-0 flex items-center justify-center">
			<span class="font-bold text-slate-50 {sizeClasses.text}">{score}</span>
		</div>
	</div>

	<div class="text-center">
		<p class="font-medium text-slate-200 {sizeClasses.label}">{label}</p>
		<p class="text-[10px] text-{color}-400">{scoreLabel}</p>
	</div>
</div>

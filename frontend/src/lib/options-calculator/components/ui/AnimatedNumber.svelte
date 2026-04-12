<script lang="ts">
	import gsap from 'gsap';

	interface Props {
		value: number;
		decimals?: number;
		prefix?: string;
		suffix?: string;
		duration?: number;
		colorize?: boolean;
		size?: 'sm' | 'md' | 'lg' | 'hero';
		class?: string;
	}

	let {
		value,
		decimals = 2,
		prefix = '',
		suffix = '',
		duration = 0.6,
		colorize = false,
		size = 'md',
		class: className = ''
	}: Props = $props();

	let displayEl: HTMLSpanElement | undefined = $state();
	let tweenObj = { val: 0 };
	let displayText = $state('');
	let prevValue: number | null = $state(null);
	let flashClass = $state('');
	let flashTimeout: ReturnType<typeof setTimeout> | undefined;
	let activeTween: gsap.core.Tween | undefined;

	let colorClass = $derived(
		colorize ? (value > 0.0001 ? 'text-call' : value < -0.0001 ? 'text-put' : '') : ''
	);

	let sizeClass = $derived(
		size === 'sm'
			? 'animated-number--sm'
			: size === 'lg'
				? 'animated-number--lg'
				: size === 'hero'
					? 'animated-number--hero'
					: 'animated-number--md'
	);

	$effect(() => {
		const targetVal = value;
		const pfx = prefix;
		const sfx = suffix;
		const dec = decimals;
		const dur = duration;

		// Direction change flash
		if (prevValue !== null && targetVal !== prevValue) {
			flashClass = targetVal > prevValue ? 'flash-up' : 'flash-down';
			clearTimeout(flashTimeout);
			flashTimeout = setTimeout(() => {
				flashClass = '';
			}, 500);
		}
		prevValue = targetVal;

		if (!displayEl) {
			tweenObj.val = targetVal;
			displayText = `${pfx}${targetVal.toFixed(dec)}${sfx}`;
			return;
		}

		activeTween?.kill();
		activeTween = gsap.to(tweenObj, {
			val: targetVal,
			duration: dur,
			ease: 'power2.out',
			onUpdate() {
				displayText = `${pfx}${tweenObj.val.toFixed(dec)}${sfx}`;
			}
		});

		return () => {
			clearTimeout(flashTimeout);
			activeTween?.kill();
		};
	});
</script>

<span
	bind:this={displayEl}
	class="number-display {sizeClass} {colorClass} {flashClass} {className}"
>
	{displayText}
</span>

<style>
	.animated-number--sm {
		font-size: 0.75rem;
	}
	.animated-number--md {
		font-size: 1rem;
	}
	.animated-number--lg {
		font-size: 1.5rem;
	}
	.animated-number--hero {
		font-size: 2.5rem;
		font-weight: 700;
		letter-spacing: -0.02em;
	}

	.flash-up {
		animation: flash-green 0.5s ease-out;
	}
	.flash-down {
		animation: flash-red 0.5s ease-out;
	}

	@keyframes flash-green {
		0% {
			color: #22c55e;
			text-shadow: 0 0 8px rgba(34, 197, 94, 0.6);
		}
		100% {
			color: inherit;
			text-shadow: none;
		}
	}

	@keyframes flash-red {
		0% {
			color: #ef4444;
			text-shadow: 0 0 8px rgba(239, 68, 68, 0.6);
		}
		100% {
			color: inherit;
			text-shadow: none;
		}
	}
</style>

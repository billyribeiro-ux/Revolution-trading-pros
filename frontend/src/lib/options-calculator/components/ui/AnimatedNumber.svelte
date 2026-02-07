<script lang="ts">
	import gsap from 'gsap';

	interface Props {
		value: number;
		decimals?: number;
		prefix?: string;
		suffix?: string;
		duration?: number;
		colorize?: boolean;
		class?: string;
	}

	let {
		value,
		decimals = 2,
		prefix = '',
		suffix = '',
		duration = 0.6,
		colorize = false,
		class: className = '',
	}: Props = $props();

	let displayEl: HTMLSpanElement | undefined = $state();
	let tweenObj = { val: 0 };
	let displayText = $state('');

	let colorClass = $derived(
		colorize ? (value > 0.0001 ? 'text-call' : value < -0.0001 ? 'text-put' : '') : ''
	);

	$effect(() => {
		const targetVal = value;
		const pfx = prefix;
		const sfx = suffix;
		const dec = decimals;
		const dur = duration;

		if (!displayEl) {
			tweenObj.val = targetVal;
			displayText = `${pfx}${targetVal.toFixed(dec)}${sfx}`;
			return;
		}

		gsap.to(tweenObj, {
			val: targetVal,
			duration: dur,
			ease: 'power2.out',
			onUpdate() {
				displayText = `${pfx}${tweenObj.val.toFixed(dec)}${sfx}`;
			},
		});
	});
</script>

<span bind:this={displayEl} class="number-display {colorClass} {className}">
	{displayText}
</span>

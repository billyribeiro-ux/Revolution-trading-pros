<script lang="ts">
	import { cubicOut } from 'svelte/easing';
	import { prefersReducedMotion } from 'svelte/motion';
	import { devicePixelRatio } from 'svelte/reactivity/window';
	import { fade } from 'svelte/transition';
	import type { Attachment } from 'svelte/attachments';
	import { DoubleSide } from 'three/src/constants.js';
	import { BufferAttribute } from 'three/src/core/BufferAttribute.js';
	import { PerspectiveCamera } from 'three/src/cameras/PerspectiveCamera.js';
	import { PlaneGeometry } from 'three/src/geometries/PlaneGeometry.js';
	import { GridHelper } from 'three/src/helpers/GridHelper.js';
	import { AmbientLight } from 'three/src/lights/AmbientLight.js';
	import { DirectionalLight } from 'three/src/lights/DirectionalLight.js';
	import { PointLight } from 'three/src/lights/PointLight.js';
	import { MeshBasicMaterial } from 'three/src/materials/MeshBasicMaterial.js';
	import { MeshPhongMaterial } from 'three/src/materials/MeshPhongMaterial.js';
	import { Color } from 'three/src/math/Color.js';
	import { Mesh } from 'three/src/objects/Mesh.js';
	import { WebGLRenderer } from 'three/src/renderers/WebGLRenderer.js';
	import { Scene } from 'three/src/scenes/Scene.js';
	import { OrbitControls } from 'three/addons/controls/OrbitControls.js';
	import { price as bsPrice } from '../../engine/black-scholes.js';
	import { getCalculator } from '../../state/context.js';

	const calc = getCalculator();

	/** The surface is a SEGMENTS × SEGMENTS plane, SIZE world units on a side. */
	const SEGMENTS = 30;
	const SIZE = 4;
	/** World-space height between the largest loss and the largest profit. */
	const HEIGHT_SCALE = 1.5;

	// Auto-rotation follows the reduced-motion preference until the viewer toggles it.
	let autoRotate = $derived(!prefersReducedMotion.current);
	let width = $state(0);
	let height = $state(0);
	let webglUnavailable = $state(false);

	/** Axis ranges: strikes ±30% around spot, expiries from ~4 days to the current expiry. */
	const strikeRange = $derived([calc.spotPrice * 0.7, calc.spotPrice * 1.3] as const);
	const expiryRange = $derived([0.01, Math.max(calc.timeToExpiry, 0.02)] as const);

	/**
	 * The option re-priced at every grid point (strike along u, expiry along v; index
	 * `v * (SEGMENTS + 1) + u`), minus the current premium, scaled to 0 (largest loss) … 1
	 * (largest profit).
	 */
	const plLevels = $derived.by(() => {
		const inputs = calc.inputs;
		const premium = calc.currentPrice;
		const isCall = calc.optionType === 'call';
		const [strikeLow, strikeHigh] = strikeRange;
		const [timeLow, timeHigh] = expiryRange;
		const side = SEGMENTS + 1;
		const pl = new Float64Array(side * side);
		let min = Infinity;
		let max = -Infinity;
		for (let v = 0; v < side; v++) {
			for (let u = 0; u < side; u++) {
				const pricing = bsPrice({
					...inputs,
					strikePrice: strikeLow + (u / SEGMENTS) * (strikeHigh - strikeLow),
					timeToExpiry: timeLow + (v / SEGMENTS) * (timeHigh - timeLow)
				});
				const value = (isCall ? pricing.callPrice : pricing.putPrice) - premium;
				pl[v * side + u] = value;
				min = Math.min(min, value);
				max = Math.max(max, value);
			}
		}
		const range = Math.max(max - min, 0.01);
		return pl.map((value) => (value - min) / range);
	});

	const description = $derived.by(() => {
		const usd = (value: number) => `$${value.toFixed(2)}`;
		const days = (years: number) => Math.round(years * 365);
		return (
			`3D surface of the ${calc.optionType}'s profit or loss against its current premium, ` +
			`for strikes from ${usd(strikeRange[0])} to ${usd(strikeRange[1])} ` +
			`and ${days(expiryRange[0])} to ${days(expiryRange[1])} days to expiry`
		);
	});

	const surface: Attachment<HTMLCanvasElement> = (canvas) => {
		let renderer: WebGLRenderer;
		try {
			renderer = new WebGLRenderer({ canvas, antialias: true, alpha: true });
		} catch {
			webglUnavailable = true;
			return;
		}

		const scene = new Scene();
		scene.background = new Color(0x0a0e1a);

		const camera = new PerspectiveCamera(50, 1.5, 0.1, 1000);
		camera.position.set(3, 2.5, 3);
		camera.lookAt(0, 0, 0);

		const controls = new OrbitControls(camera, canvas);
		controls.enableDamping = true;
		controls.dampingFactor = 0.05;
		controls.autoRotateSpeed = 0.5;

		const ambientLight = new AmbientLight(0x404060, 0.6);
		const dirLight = new DirectionalLight(0xffffff, 0.8);
		dirLight.position.set(5, 5, 5);
		const pointLight = new PointLight(0x00d4aa, 0.4, 20);
		pointLight.position.set(0, -2, 0);
		scene.add(ambientLight, dirLight, pointLight);

		// One geometry for the canvas's lifetime, shared by the shaded surface and its wireframe;
		// input changes rewrite its vertices in place instead of rebuilding the scene.
		const geometry = new PlaneGeometry(SIZE, SIZE, SEGMENTS, SEGMENTS);
		const positions = geometry.attributes.position;
		const colors = new BufferAttribute(new Float32Array(positions.count * 3), 3);
		geometry.setAttribute('color', colors);

		const surfaceMaterial = new MeshPhongMaterial({
			vertexColors: true,
			side: DoubleSide,
			shininess: 40,
			transparent: true,
			opacity: 0.92
		});
		const wireMaterial = new MeshBasicMaterial({
			color: 0xffffff,
			wireframe: true,
			transparent: true,
			opacity: 0.06
		});
		const surfaceMesh = new Mesh(geometry, surfaceMaterial);
		surfaceMesh.rotation.x = -Math.PI / 2;
		const wireMesh = new Mesh(geometry, wireMaterial);
		wireMesh.rotation.x = -Math.PI / 2;

		const grid = new GridHelper(4, 20, 0x1a1e2e, 0x1a1e2e);
		grid.position.y = -HEIGHT_SCALE / 2 - 0.1;
		scene.add(surfaceMesh, wireMesh, grid);

		// Nested effects push the P/L levels, size and settings into the live scene, while this
		// outer attachment (renderer, scene, controls) runs once per canvas.
		$effect(() => {
			const levels = plLevels;
			for (let i = 0; i < positions.count; i++) {
				// Plane vertices span ±SIZE/2 in x and y: map each back to its grid point.
				const u = Math.round(((positions.getX(i) + SIZE / 2) / SIZE) * SEGMENTS);
				const v = Math.round(((positions.getY(i) + SIZE / 2) / SIZE) * SEGMENTS);
				const t = levels[v * (SEGMENTS + 1) + u];
				positions.setZ(i, (t - 0.5) * HEIGHT_SCALE);

				// Red (loss) → dark (breakeven) → green (profit).
				if (t < 0.5) {
					const s = t / 0.5;
					colors.setXYZ(i, 1 - s * 0.7, s * 0.3, s * 0.3);
				} else {
					const s = (t - 0.5) / 0.5;
					colors.setXYZ(i, 0.3 - s * 0.3, 0.3 + s * 0.53, 0.3 + s * 0.37);
				}
			}

			positions.needsUpdate = true;
			colors.needsUpdate = true;
			geometry.computeVertexNormals();
			geometry.computeBoundingSphere();
		});

		$effect(() => {
			renderer.setPixelRatio(Math.min(devicePixelRatio.current ?? 1, 2));
		});

		$effect(() => {
			if (width <= 0 || height <= 0) return;
			camera.aspect = width / height;
			camera.updateProjectionMatrix();
			renderer.setSize(width, height, false);
		});

		$effect(() => {
			controls.autoRotate = autoRotate;
		});

		let animId = 0;
		const frame = () => {
			animId = requestAnimationFrame(frame);
			controls.update();
			renderer.render(scene, camera);
		};

		// Render only while on screen. Entries arrive oldest first, so the last one is current.
		const visibility = new IntersectionObserver((entries) => {
			cancelAnimationFrame(animId);
			if (entries.at(-1)?.isIntersecting) frame();
		});
		visibility.observe(canvas);

		return () => {
			cancelAnimationFrame(animId);
			visibility.disconnect();
			controls.dispose();
			geometry.dispose();
			surfaceMaterial.dispose();
			wireMaterial.dispose();
			grid.dispose();
			ambientLight.dispose();
			dirLight.dispose();
			pointLight.dispose();
			renderer.dispose();
			// Release the context now rather than at GC: reopening the tab repeatedly would
			// otherwise run into the browser's live-context limit.
			renderer.forceContextLoss();
		};
	};
</script>

<div class="flex flex-col gap-3">
	<!-- Controls -->
	<div class="flex items-center gap-3">
		<button
			type="button"
			onclick={() => (autoRotate = !autoRotate)}
			aria-pressed={autoRotate}
			class="text-xs px-2.5 py-1 rounded-lg border transition-all cursor-pointer"
			style:background={autoRotate ? 'var(--calc-accent-glow)' : 'var(--calc-surface)'}
			style:color={autoRotate ? 'var(--calc-accent)' : 'var(--calc-text-muted)'}
			style:border-color={autoRotate ? 'var(--calc-accent)' : 'var(--calc-border)'}
		>
			<!-- The name stays "Auto-Rotate"; aria-pressed carries the on/off state. -->
			Auto-Rotate<span aria-hidden="true">: {autoRotate ? 'ON' : 'OFF'}</span>
		</button>
		<span
			class="text-[10px]"
			style="color: var(--calc-text-muted); font-family: var(--calc-font-body);"
		>
			Drag to rotate · Scroll to zoom · Right-click to pan
		</span>
	</div>

	<!-- 3D Canvas -->
	{#if webglUnavailable}
		<div
			class="flex h-100 items-center justify-center rounded-xl border px-6 text-center text-xs"
			style="border-color: var(--calc-border); color: var(--calc-text-muted);"
		>
			The 3D surface needs WebGL, which this browser has disabled.
		</div>
	{:else}
		<div
			class="w-full rounded-xl overflow-hidden"
			style="background: #0a0e1a;"
			role="img"
			aria-label={description}
		>
			<canvas
				{@attach surface}
				bind:clientWidth={width}
				bind:clientHeight={height}
				in:fade|global={{ duration: prefersReducedMotion.current ? 0 : 600, easing: cubicOut }}
				class="block h-100 w-full"
				aria-hidden="true"
			></canvas>
		</div>
	{/if}

	<!-- Legend -->
	<div class="flex items-center justify-center gap-4">
		<div class="flex items-center gap-1.5">
			<div class="w-3 h-3 rounded-sm" style="background: #cc3333;"></div>
			<span class="text-[10px]" style="color: var(--calc-text-muted);">Max Loss</span>
		</div>
		<div class="flex items-center gap-1.5">
			<div class="w-3 h-3 rounded-sm" style="background: #4d4d4d;"></div>
			<span class="text-[10px]" style="color: var(--calc-text-muted);">Breakeven</span>
		</div>
		<div class="flex items-center gap-1.5">
			<div class="w-3 h-3 rounded-sm" style="background: #00d4aa;"></div>
			<span class="text-[10px]" style="color: var(--calc-text-muted);">Max Profit</span>
		</div>
	</div>
</div>

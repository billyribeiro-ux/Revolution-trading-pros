<script lang="ts">
	import { price as bsPrice } from '../../engine/black-scholes.js';
	import gsap from 'gsap';
	import type { CalculatorState } from '../../state/calculator.svelte.js';
	import type { BSInputs } from '../../engine/types.js';

	interface Props {
		calc: CalculatorState;
	}

	let { calc }: Props = $props();

	let canvasEl: HTMLCanvasElement | undefined = $state();
	let containerEl: HTMLDivElement | undefined = $state();
	let isLoading = $state(true);
	let autoRotate = $state(true);
	let width = $state(600);
	let height = $state(400);

	$effect(() => {
		if (!containerEl) return;
		const observer = new ResizeObserver((entries) => {
			for (const entry of entries) {
				width = entry.contentRect.width;
				height = Math.max(320, Math.min(450, entry.contentRect.height));
			}
		});
		observer.observe(containerEl);
		return () => observer.disconnect();
	});

	$effect(() => {
		if (!canvasEl || typeof window === 'undefined') return;

		let disposed = false;
		let animId = 0;

		async function init() {
			const THREE = await import('three');
			const { OrbitControls } = await import('three/addons/controls/OrbitControls.js');

			if (disposed) return;
			isLoading = false;

			const scene = new THREE.Scene();
			scene.background = new THREE.Color(0x0a0e1a);

			const camera = new THREE.PerspectiveCamera(50, width / height, 0.1, 1000);
			camera.position.set(3, 2.5, 3);
			camera.lookAt(0, 0, 0);

			const renderer = new THREE.WebGLRenderer({ canvas: canvasEl, antialias: true, alpha: true });
			renderer.setSize(width, height);
			renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));

			const controls = new OrbitControls(camera, renderer.domElement);
			controls.enableDamping = true;
			controls.dampingFactor = 0.05;
			controls.autoRotate = autoRotate;
			controls.autoRotateSpeed = 0.5;

			// Lighting
			const ambientLight = new THREE.AmbientLight(0x404060, 0.6);
			scene.add(ambientLight);
			const dirLight = new THREE.DirectionalLight(0xffffff, 0.8);
			dirLight.position.set(5, 5, 5);
			scene.add(dirLight);
			const pointLight = new THREE.PointLight(0x00d4aa, 0.4, 20);
			pointLight.position.set(0, -2, 0);
			scene.add(pointLight);

			// Build surface
			function buildSurface() {
				const existingMesh = scene.getObjectByName('plSurface');
				if (existingMesh) scene.remove(existingMesh);
				const existingWire = scene.getObjectByName('plWireframe');
				if (existingWire) scene.remove(existingWire);

				const inputs = calc.inputs;
				const optType = calc.optionType;
				const gridSize = 30;

				const strikeLow = inputs.spotPrice * 0.7;
				const strikeHigh = inputs.spotPrice * 1.3;
				const timeLow = 0.01;
				const timeHigh = Math.max(inputs.timeToExpiry, 0.02);

				const geometry = new THREE.PlaneGeometry(4, 4, gridSize, gridSize);
				const positions = geometry.attributes.position;
				const colors: number[] = [];

				let minPL = Infinity;
				let maxPL = -Infinity;
				const plValues: number[] = [];

				for (let i = 0; i < positions.count; i++) {
					const u = (positions.getX(i) + 2) / 4;
					const v = (positions.getY(i) + 2) / 4;

					const strike = strikeLow + u * (strikeHigh - strikeLow);
					const tte = timeLow + v * (timeHigh - timeLow);

					const testInputs: BSInputs = { ...inputs, strikePrice: strike, timeToExpiry: tte };
					const pricing = bsPrice(testInputs);
					const optionPrice = optType === 'call' ? pricing.callPrice : pricing.putPrice;
					const currentPremium =
						optType === 'call' ? bsPrice(inputs).callPrice : bsPrice(inputs).putPrice;
					const pl = optionPrice - currentPremium;

					plValues.push(pl);
					minPL = Math.min(minPL, pl);
					maxPL = Math.max(maxPL, pl);
				}

				const plRange = Math.max(maxPL - minPL, 0.01);
				const heightScale = 1.5;

				for (let i = 0; i < positions.count; i++) {
					const normalizedPL = (plValues[i] - minPL) / plRange;
					const yPos = (normalizedPL - 0.5) * heightScale;
					positions.setZ(i, yPos);

					// Color: red (loss) → dark (breakeven) → green (profit)
					const t = normalizedPL;
					let r: number, g: number, b: number;
					if (t < 0.5) {
						const s = t / 0.5;
						r = 1.0 - s * 0.7;
						g = s * 0.3;
						b = s * 0.3;
					} else {
						const s = (t - 0.5) / 0.5;
						r = 0.3 - s * 0.3;
						g = 0.3 + s * 0.53;
						b = 0.3 + s * 0.37;
					}
					colors.push(r, g, b);
				}

				geometry.setAttribute('color', new THREE.Float32BufferAttribute(colors, 3));
				geometry.computeVertexNormals();

				const material = new THREE.MeshPhongMaterial({
					vertexColors: true,
					side: THREE.DoubleSide,
					shininess: 40,
					transparent: true,
					opacity: 0.92
				});

				const mesh = new THREE.Mesh(geometry, material);
				mesh.name = 'plSurface';
				mesh.rotation.x = -Math.PI / 2;
				scene.add(mesh);

				// Wireframe overlay
				const wireMat = new THREE.MeshBasicMaterial({
					color: 0xffffff,
					wireframe: true,
					transparent: true,
					opacity: 0.06
				});
				const wireMesh = new THREE.Mesh(geometry.clone(), wireMat);
				wireMesh.name = 'plWireframe';
				wireMesh.rotation.x = -Math.PI / 2;
				scene.add(wireMesh);

				// Grid helper
				const existingGrid = scene.getObjectByName('gridHelper');
				if (!existingGrid) {
					const gridHelper = new THREE.GridHelper(4, 20, 0x1a1e2e, 0x1a1e2e);
					gridHelper.name = 'gridHelper';
					gridHelper.position.y = -heightScale / 2 - 0.1;
					scene.add(gridHelper);
				}
			}

			buildSurface();

			// Fade in
			if (canvasEl) {
				gsap.fromTo(canvasEl, { opacity: 0 }, { opacity: 1, duration: 0.6, ease: 'power2.out' });
			}

			function animate() {
				if (disposed) return;
				animId = requestAnimationFrame(animate);
				controls.autoRotate = autoRotate;
				controls.update();
				renderer.render(scene, camera);
			}
			animate();

			// Watch for input changes and rebuild surface
			const unwatch = $effect.root(() => {
				$effect(() => {
					void calc.spotPrice;
					void calc.strikePrice;
					void calc.volatility;
					void calc.timeToExpiry;
					void calc.riskFreeRate;
					void calc.dividendYield;
					void calc.optionType;
					buildSurface();
				});

				$effect(() => {
					void width;
					void height;
					camera.aspect = width / height;
					camera.updateProjectionMatrix();
					renderer.setSize(width, height);
				});
			});

			return () => {
				disposed = true;
				cancelAnimationFrame(animId);
				unwatch();
				renderer.dispose();
				controls.dispose();
			};
		}

		const cleanupPromise = init();

		return () => {
			cleanupPromise.then((cleanup) => cleanup?.());
		};
	});
</script>

<div class="flex flex-col gap-3">
	<!-- Controls -->
	<div class="flex items-center gap-3">
		<button
			onclick={() => (autoRotate = !autoRotate)}
			class="text-xs px-2.5 py-1 rounded-lg transition-all cursor-pointer"
			style={autoRotate
				? 'background: var(--calc-accent-glow); color: var(--calc-accent); border: 1px solid var(--calc-accent);'
				: 'background: var(--calc-surface); color: var(--calc-text-muted); border: 1px solid var(--calc-border);'}
		>
			{autoRotate ? 'Auto-Rotate: ON' : 'Auto-Rotate: OFF'}
		</button>
		<span
			class="text-[10px]"
			style="color: var(--calc-text-muted); font-family: var(--calc-font-body);"
		>
			Drag to rotate · Scroll to zoom · Right-click to pan
		</span>
	</div>

	<!-- 3D Canvas -->
	<div
		bind:this={containerEl}
		class="relative w-full rounded-xl overflow-hidden"
		style="min-height: 320px; background: #0a0e1a;"
	>
		{#if isLoading}
			<div
				class="absolute inset-0 flex items-center justify-center"
				style="color: var(--calc-text-muted);"
			>
				<div class="flex flex-col items-center gap-2">
					<div
						class="w-6 h-6 border-2 border-current border-t-transparent rounded-full animate-spin"
					></div>
					<span class="text-xs">Loading 3D surface...</span>
				</div>
			</div>
		{/if}
		<canvas bind:this={canvasEl} style="width: {width}px; height: {height}px;"></canvas>
	</div>

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

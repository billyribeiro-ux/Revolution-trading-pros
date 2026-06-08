<script lang="ts">
	import { price as bsPrice } from '../../engine/black-scholes.js';
	import gsap from 'gsap';
	import { DoubleSide } from 'three/src/constants.js';
	import { Float32BufferAttribute } from 'three/src/core/BufferAttribute.js';
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
	import type { Attachment } from 'svelte/attachments';
	import type { CalculatorState } from '../../state/calculator.svelte.js';
	import type { BSInputs } from '../../engine/types.js';

	interface Props {
		calc: CalculatorState;
	}

	let { calc }: Props = $props();

	let isLoading = $state(true);
	let autoRotate = $state(true);
	let width = $state(600);
	let height = $state(400);

	const surfaceKey = $derived(
		[
			width,
			height,
			calc.spotPrice,
			calc.strikePrice,
			calc.volatility,
			calc.timeToExpiry,
			calc.riskFreeRate,
			calc.dividendYield,
			calc.optionType
		].join(':')
	);

	const observeContainer: Attachment<HTMLDivElement> = (node) => {
		const updateSize = () => {
			const rect = node.getBoundingClientRect();
			width = rect.width || 600;
			height = Math.max(320, Math.min(450, rect.height || 400));
		};

		updateSize();
		const observer = new ResizeObserver((entries) => {
			for (const entry of entries) {
				width = entry.contentRect.width;
				height = Math.max(320, Math.min(450, entry.contentRect.height));
			}
		});
		observer.observe(node);
		return () => observer.disconnect();
	};

	const initCanvas: Attachment<HTMLCanvasElement> = (canvas) => {
		if (typeof window === 'undefined') return;

		let disposed = false;
		let animId = 0;

		isLoading = false;

		const scene = new Scene();
		scene.background = new Color(0x0a0e1a);

		const camera = new PerspectiveCamera(50, width / height, 0.1, 1000);
		camera.position.set(3, 2.5, 3);
		camera.lookAt(0, 0, 0);

		const renderer = new WebGLRenderer({ canvas, antialias: true, alpha: true });
		renderer.setSize(width, height);
		renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));

		const controls = new OrbitControls(camera, renderer.domElement);
		controls.enableDamping = true;
		controls.dampingFactor = 0.05;
		controls.autoRotate = autoRotate;
		controls.autoRotateSpeed = 0.5;

		// Lighting
		const ambientLight = new AmbientLight(0x404060, 0.6);
		scene.add(ambientLight);
		const dirLight = new DirectionalLight(0xffffff, 0.8);
		dirLight.position.set(5, 5, 5);
		scene.add(dirLight);
		const pointLight = new PointLight(0x00d4aa, 0.4, 20);
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

			const geometry = new PlaneGeometry(4, 4, gridSize, gridSize);
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

				// Color: red (loss) -> dark (breakeven) -> green (profit)
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

			geometry.setAttribute('color', new Float32BufferAttribute(colors, 3));
			geometry.computeVertexNormals();

			const material = new MeshPhongMaterial({
				vertexColors: true,
				side: DoubleSide,
				shininess: 40,
				transparent: true,
				opacity: 0.92
			});

			const mesh = new Mesh(geometry, material);
			mesh.name = 'plSurface';
			mesh.rotation.x = -Math.PI / 2;
			scene.add(mesh);

			// Wireframe overlay
			const wireMat = new MeshBasicMaterial({
				color: 0xffffff,
				wireframe: true,
				transparent: true,
				opacity: 0.06
			});
			const wireMesh = new Mesh(geometry.clone(), wireMat);
			wireMesh.name = 'plWireframe';
			wireMesh.rotation.x = -Math.PI / 2;
			scene.add(wireMesh);

			// Grid helper
			const existingGrid = scene.getObjectByName('gridHelper');
			if (!existingGrid) {
				const gridHelper = new GridHelper(4, 20, 0x1a1e2e, 0x1a1e2e);
				gridHelper.name = 'gridHelper';
				gridHelper.position.y = -heightScale / 2 - 0.1;
				scene.add(gridHelper);
			}
		}

		buildSurface();

		gsap.fromTo(canvas, { opacity: 0 }, { opacity: 1, duration: 0.6, ease: 'power2.out' });

		function animate() {
			if (disposed) return;
			animId = requestAnimationFrame(animate);
			controls.autoRotate = autoRotate;
			controls.update();
			renderer.render(scene, camera);
		}
		animate();

		return () => {
			disposed = true;
			cancelAnimationFrame(animId);
			renderer.dispose();
			controls.dispose();
		};
	};
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
		{@attach observeContainer}
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
		{#key surfaceKey}
			<canvas {@attach initCanvas} style:width={`${width}px`} style:height={`${height}px`}></canvas>
		{/key}
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

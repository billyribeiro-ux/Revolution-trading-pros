<script lang="ts">
	import { prefersReducedMotion } from 'svelte/motion';
	import { PerspectiveCamera } from 'three/src/cameras/PerspectiveCamera.js';
	import { BufferAttribute } from 'three/src/core/BufferAttribute.js';
	import { BufferGeometry } from 'three/src/core/BufferGeometry.js';
	import { IcosahedronGeometry } from 'three/src/geometries/IcosahedronGeometry.js';
	import { SphereGeometry } from 'three/src/geometries/SphereGeometry.js';
	import { AmbientLight } from 'three/src/lights/AmbientLight.js';
	import { PointLight } from 'three/src/lights/PointLight.js';
	import { LineBasicMaterial } from 'three/src/materials/LineBasicMaterial.js';
	import { MeshBasicMaterial } from 'three/src/materials/MeshBasicMaterial.js';
	import { MeshStandardMaterial } from 'three/src/materials/MeshStandardMaterial.js';
	import { SpriteMaterial } from 'three/src/materials/SpriteMaterial.js';
	import { LineLoop } from 'three/src/objects/LineLoop.js';
	import { Mesh } from 'three/src/objects/Mesh.js';
	import { Sprite } from 'three/src/objects/Sprite.js';
	import { WebGLRenderer } from 'three/src/renderers/WebGLRenderer.js';
	import { Scene } from 'three/src/scenes/Scene.js';
	import { CanvasTexture } from 'three/src/textures/CanvasTexture.js';
	import { OrbitControls } from 'three/addons/controls/OrbitControls.js';
	import type { Attachment } from 'svelte/attachments';
	import type { ThemeMode } from '../../engine/types.js';

	interface Props {
		delta?: number;
		gamma?: number;
		theta?: number;
		vega?: number;
		rho?: number;
		theme?: ThemeMode;
	}

	let { delta = 0, gamma = 0, theta = 0, vega = 0, rho = 0, theme = 'dark' }: Props = $props();

	/** `maxRange` is the magnitude at which a sphere reaches full size. */
	const GREEKS = [
		{ name: 'Delta', symbol: 'Δ', maxRange: 1 },
		{ name: 'Gamma', symbol: 'Γ', maxRange: 0.1 },
		{ name: 'Theta', symbol: 'Θ', maxRange: 0.5 },
		{ name: 'Vega', symbol: 'ν', maxRange: 1 },
		{ name: 'Rho', symbol: 'ρ', maxRange: 0.1 }
	] as const;

	const ORBIT_RADIUS = 2.2;
	const MIN_SPHERE_RADIUS = 0.15;
	const SPHERE_RADIUS_RANGE = 0.6;
	const POSITIVE = { hex: 0x10b981, css: '#10b981' } as const;
	const NEGATIVE = { hex: 0xef4444, css: '#ef4444' } as const;
	const LABEL_VALUE_COLOR: Record<ThemeMode, string> = {
		dark: 'rgba(255, 255, 255, 0.85)',
		light: 'rgba(17, 24, 39, 0.85)'
	};

	const values = $derived([delta, gamma, theta, vega, rho]);
	const description = $derived(
		GREEKS.map((greek, i) => `${greek.name} ${values[i].toFixed(4)}`).join(', ')
	);

	let webglUnavailable = $state(false);

	interface OrbitNode {
		x: number;
		z: number;
		radius: number;
		normalized: number;
		sphere: Mesh;
		material: MeshStandardMaterial;
		label: Sprite;
		labelMaterial: SpriteMaterial;
		labelCanvas: HTMLCanvasElement;
		texture: CanvasTexture;
	}

	function drawLabel(
		node: OrbitNode,
		greek: (typeof GREEKS)[number],
		value: number,
		valueColor: string
	): void {
		const ctx = node.labelCanvas.getContext('2d');
		if (!ctx) return;
		const { width, height } = node.labelCanvas;
		ctx.clearRect(0, 0, width, height);
		ctx.textAlign = 'center';
		ctx.font = '700 26px "Plus Jakarta Sans", system-ui, sans-serif';
		ctx.fillStyle = value >= 0 ? POSITIVE.css : NEGATIVE.css;
		ctx.fillText(`${greek.name} (${greek.symbol})`, width / 2, 45);
		ctx.font = '22px "JetBrains Mono", ui-monospace, monospace';
		ctx.fillStyle = valueColor;
		ctx.fillText(value.toFixed(4), width / 2, 80);
		node.texture.needsUpdate = true;
	}

	const initScene: Attachment<HTMLCanvasElement> = (canvas) => {
		let renderer: WebGLRenderer;
		try {
			renderer = new WebGLRenderer({ canvas, antialias: true, alpha: true });
		} catch {
			webglUnavailable = true;
			return;
		}
		renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
		renderer.setClearColor(0x000000, 0);

		const scene = new Scene();
		const camera = new PerspectiveCamera(50, 2, 0.1, 100);
		camera.position.set(2, 2.5, 6);
		camera.lookAt(0, 0, 0);

		const resize = (width: number, height: number) => {
			if (width <= 0 || height <= 0) return;
			camera.aspect = width / height;
			camera.updateProjectionMatrix();
			renderer.setSize(width, height, false);
		};
		resize(canvas.clientWidth, canvas.clientHeight);

		const controls = new OrbitControls(camera, canvas);
		controls.enableDamping = true;
		controls.dampingFactor = 0.08;
		controls.autoRotateSpeed = 0.8;
		controls.enablePan = false;
		controls.minDistance = 3;
		controls.maxDistance = 12;

		scene.add(new AmbientLight(0x404060, 0.5));
		const keyLight = new PointLight(0xffffff, 1.2, 30);
		keyLight.position.set(4, 6, 4);
		const fillLight = new PointLight(0x6366f1, 0.4, 20);
		fillLight.position.set(-4, -2, -3);
		scene.add(keyLight, fillLight);

		const coreGeometry = new IcosahedronGeometry(0.25, 0);
		const coreMaterial = new MeshBasicMaterial({
			color: 0x6366f1,
			wireframe: true,
			transparent: true,
			opacity: 0.6
		});
		const core = new Mesh(coreGeometry, coreMaterial);
		scene.add(core);

		// One unit sphere shared by every node; each node scales it to its radius.
		const sphereGeometry = new SphereGeometry(1, 32, 32);
		const nodes: OrbitNode[] = GREEKS.map((_, i) => {
			const angle = (i / GREEKS.length) * Math.PI * 2 - Math.PI / 2;
			const material = new MeshStandardMaterial({
				color: POSITIVE.hex,
				emissive: POSITIVE.hex,
				emissiveIntensity: 0.3,
				metalness: 0.3,
				roughness: 0.4,
				transparent: true,
				opacity: 0.9
			});
			const sphere = new Mesh(sphereGeometry, material);
			const labelCanvas = document.createElement('canvas');
			labelCanvas.width = 256;
			labelCanvas.height = 128;
			const texture = new CanvasTexture(labelCanvas);
			const labelMaterial = new SpriteMaterial({
				map: texture,
				transparent: true,
				depthTest: false
			});
			const label = new Sprite(labelMaterial);
			label.scale.set(1.2, 0.6, 1);
			scene.add(sphere, label);
			return {
				x: Math.cos(angle) * ORBIT_RADIUS,
				z: Math.sin(angle) * ORBIT_RADIUS,
				radius: MIN_SPHERE_RADIUS,
				normalized: 0,
				sphere,
				material,
				label,
				labelMaterial,
				labelCanvas,
				texture
			};
		});

		// Closed ring linking the spheres, re-positioned every frame as they float.
		const ringPositions = new Float32Array(GREEKS.length * 3);
		const ringAttribute = new BufferAttribute(ringPositions, 3);
		const ringGeometry = new BufferGeometry();
		ringGeometry.setAttribute('position', ringAttribute);
		const ringMaterial = new LineBasicMaterial({
			color: 0x6366f1,
			transparent: true,
			opacity: 0.2
		});
		scene.add(new LineLoop(ringGeometry, ringMaterial));

		let motion = true;
		let animId = 0;
		const start = performance.now();

		const frame = () => {
			animId = requestAnimationFrame(frame);
			const t = motion ? (performance.now() - start) / 1000 : 0;
			core.rotation.set(t * 0.3, t * 0.5, 0);
			nodes.forEach((node, i) => {
				const floatY = Math.sin(t * 1.2 + i * 1.26) * 0.15;
				const pulse = motion
					? 1 + Math.sin(t * 2 + i * 0.8) * 0.08 * (0.3 + node.normalized * 0.7)
					: 1;
				node.sphere.position.set(node.x, floatY, node.z);
				node.sphere.scale.setScalar(node.radius * pulse);
				node.label.position.set(node.x, floatY + node.radius + 0.6, node.z);
				ringPositions.set([node.x, floatY, node.z], i * 3);
			});
			ringAttribute.needsUpdate = true;
			controls.update();
			renderer.render(scene, camera);
		};

		// Render only while on screen; the page keeps running with the panel scrolled away.
		const visibility = new IntersectionObserver(([entry]) => {
			cancelAnimationFrame(animId);
			if (entry?.isIntersecting) frame();
		});
		visibility.observe(canvas);

		const sizeObserver = new ResizeObserver(([entry]) => {
			if (entry) resize(entry.contentRect.width, entry.contentRect.height);
		});
		sizeObserver.observe(canvas);

		$effect(() => {
			const current = values;
			const valueColor = LABEL_VALUE_COLOR[theme];
			nodes.forEach((node, i) => {
				const greek = GREEKS[i];
				const value = current[i];
				const color = value >= 0 ? POSITIVE.hex : NEGATIVE.hex;
				node.normalized = Math.min(Math.abs(value) / greek.maxRange, 1);
				node.radius = MIN_SPHERE_RADIUS + node.normalized * SPHERE_RADIUS_RANGE;
				node.material.color.setHex(color);
				node.material.emissive.setHex(color);
				drawLabel(node, greek, value, valueColor);
			});
		});

		$effect(() => {
			motion = !prefersReducedMotion.current;
			controls.autoRotate = motion;
		});

		return () => {
			cancelAnimationFrame(animId);
			visibility.disconnect();
			sizeObserver.disconnect();
			controls.dispose();
			for (const node of nodes) {
				node.material.dispose();
				node.texture.dispose();
				node.labelMaterial.dispose();
			}
			sphereGeometry.dispose();
			coreGeometry.dispose();
			coreMaterial.dispose();
			ringGeometry.dispose();
			ringMaterial.dispose();
			renderer.dispose();
			// Release the context now rather than at GC: toggling 2D/3D repeatedly
			// would otherwise run into the browser's live-context limit.
			renderer.forceContextLoss();
		};
	};
</script>

<div class="relative h-[300px] w-full">
	{#if webglUnavailable}
		<div
			class="flex h-full items-center justify-center rounded-xl border text-xs"
			style="border-color: var(--calc-border); color: var(--calc-text-muted);"
		>
			The 3D view needs WebGL, which this browser has disabled.
		</div>
	{:else}
		<div
			class="h-full w-full"
			role="img"
			aria-label="3D orbit of the first-order Greeks: {description}"
		>
			<canvas
				{@attach initScene}
				class="block h-full w-full cursor-grab active:cursor-grabbing"
				aria-hidden="true"
			></canvas>
		</div>
	{/if}
</div>

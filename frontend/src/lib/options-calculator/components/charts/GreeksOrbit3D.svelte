<script lang="ts">
	import { on } from 'svelte/events';
	import { prefersReducedMotion } from 'svelte/motion';
	import { devicePixelRatio } from 'svelte/reactivity/window';
	import type { Attachment } from 'svelte/attachments';
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
	type Greek = (typeof GREEKS)[number];

	const CAMERA_FOV = 50;
	const ORBIT_RADIUS = 2.2;
	const MIN_SPHERE_RADIUS = 0.15;
	const SPHERE_RADIUS_RANGE = 0.6;
	const POSITIVE_HEX = 0x10b981;
	const NEGATIVE_HEX = 0xef4444;

	/** Labels keep this on-screen size in CSS px at any zoom; the texture is drawn at 2×. */
	const LABEL_WIDTH = 104;
	const LABEL_HEIGHT = 40;
	const LABEL_RESOLUTION = 2;
	/** World-space gap between the top of a sphere and the bottom edge of its label. */
	const LABEL_GAP = 0.15;

	interface LabelColors {
		positive: string;
		negative: string;
		value: string;
	}

	/** 500 shades on the dark card; 700 shades on the light one (≥ 4.5:1 against white). */
	const LABEL_COLORS: Record<ThemeMode, LabelColors> = {
		dark: { positive: '#10b981', negative: '#ef4444', value: 'rgba(255, 255, 255, 0.85)' },
		light: { positive: '#047857', negative: '#b91c1c', value: 'rgba(17, 24, 39, 0.85)' }
	};

	const values = $derived([delta, gamma, theta, vega, rho]);
	const description = $derived(
		GREEKS.map((greek, i) => `${greek.name} ${values[i].toFixed(4)}`).join(', ')
	);

	let width = $state(0);
	let height = $state(0);
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

	function drawLabel(node: OrbitNode, greek: Greek, value: number, colors: LabelColors): void {
		const ctx = node.labelCanvas.getContext('2d');
		if (!ctx) return;
		// Draw in CSS px; the transform maps them onto the 2× texture.
		ctx.setTransform(LABEL_RESOLUTION, 0, 0, LABEL_RESOLUTION, 0, 0);
		ctx.clearRect(0, 0, LABEL_WIDTH, LABEL_HEIGHT);
		ctx.textAlign = 'center';
		ctx.font = '700 12px "Plus Jakarta Sans", system-ui, sans-serif';
		ctx.fillStyle = value >= 0 ? colors.positive : colors.negative;
		ctx.fillText(`${greek.name} (${greek.symbol})`, LABEL_WIDTH / 2, 15);
		ctx.font = '12px "JetBrains Mono", ui-monospace, monospace';
		ctx.fillStyle = colors.value;
		ctx.fillText(value.toFixed(4), LABEL_WIDTH / 2, 33);
		node.texture.needsUpdate = true;
	}

	const orbit: Attachment<HTMLCanvasElement> = (canvas) => {
		let renderer: WebGLRenderer;
		try {
			renderer = new WebGLRenderer({ canvas, antialias: true, alpha: true });
		} catch {
			webglUnavailable = true;
			return;
		}
		renderer.setClearColor(0x000000, 0);

		const scene = new Scene();
		const camera = new PerspectiveCamera(CAMERA_FOV, 2, 0.1, 100);
		camera.position.set(2, 2.5, 6);
		camera.lookAt(0, 0, 0);

		const controls = new OrbitControls(camera, canvas);
		controls.enableDamping = true;
		controls.dampingFactor = 0.08;
		controls.autoRotateSpeed = 0.8;
		controls.enablePan = false;
		controls.minDistance = 3;
		controls.maxDistance = 12;

		// The canvas spans the panel, so it must not trap page scrolling. A plain wheel scrolls
		// the page and zoom needs Ctrl/⌘ (trackpad pinch reports ctrlKey). This capture listener
		// runs before OrbitControls' own wheel listener on the same element.
		const releaseWheel = on(
			canvas,
			'wheel',
			(event) => {
				if (!event.ctrlKey && !event.metaKey) event.stopImmediatePropagation();
			},
			{ capture: true, passive: true }
		);
		// OrbitControls sets `touch-action: none`; pan-y gives vertical swipes back to the page
		// while horizontal drags still rotate and two-finger pinches still zoom.
		canvas.style.touchAction = 'pan-y';

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
				color: POSITIVE_HEX,
				emissive: POSITIVE_HEX,
				emissiveIntensity: 0.3,
				metalness: 0.3,
				roughness: 0.4,
				transparent: true,
				opacity: 0.9
			});
			const sphere = new Mesh(sphereGeometry, material);
			const labelCanvas = document.createElement('canvas');
			labelCanvas.width = LABEL_WIDTH * LABEL_RESOLUTION;
			labelCanvas.height = LABEL_HEIGHT * LABEL_RESOLUTION;
			const texture = new CanvasTexture(labelCanvas);
			// sizeAttenuation off: the label keeps its screen size (set on resize) at any zoom.
			const labelMaterial = new SpriteMaterial({
				map: texture,
				transparent: true,
				depthTest: false,
				sizeAttenuation: false
			});
			const label = new Sprite(labelMaterial);
			label.center.set(0.5, 0); // anchor the bottom edge, so the label sits above its sphere
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
				node.label.position.set(node.x, floatY + node.radius + LABEL_GAP, node.z);
				ringPositions.set([node.x, floatY, node.z], i * 3);
			});
			ringAttribute.needsUpdate = true;
			controls.update();
			renderer.render(scene, camera);
		};

		// Render only while on screen. Entries arrive oldest first, so the last one is current.
		const visibility = new IntersectionObserver((entries) => {
			cancelAnimationFrame(animId);
			if (entries.at(-1)?.isIntersecting) frame();
		});
		visibility.observe(canvas);

		// Nested effects: props, size and settings update the live scene, while this outer
		// attachment (renderer, scene, controls) runs once per canvas.
		$effect(() => {
			const current = values;
			const colors = LABEL_COLORS[theme];
			nodes.forEach((node, i) => {
				const greek = GREEKS[i];
				const value = current[i];
				const color = value >= 0 ? POSITIVE_HEX : NEGATIVE_HEX;
				node.normalized = Math.min(Math.abs(value) / greek.maxRange, 1);
				node.radius = MIN_SPHERE_RADIUS + node.normalized * SPHERE_RADIUS_RANGE;
				node.material.color.setHex(color);
				node.material.emissive.setHex(color);
				drawLabel(node, greek, value, colors);
			});
		});

		$effect(() => {
			renderer.setPixelRatio(Math.min(devicePixelRatio.current ?? 1, 2));
		});

		$effect(() => {
			if (width <= 0 || height <= 0) return;
			camera.aspect = width / height;
			camera.updateProjectionMatrix();
			renderer.setSize(width, height, false);
			// Without size attenuation a sprite's scale is relative to the viewport height:
			// this maps each label to exactly LABEL_WIDTH × LABEL_HEIGHT CSS px.
			const unit = (2 * Math.tan((CAMERA_FOV * Math.PI) / 360)) / height;
			for (const node of nodes) node.label.scale.set(LABEL_WIDTH * unit, LABEL_HEIGHT * unit, 1);
		});

		$effect(() => {
			motion = !prefersReducedMotion.current;
			controls.autoRotate = motion;
		});

		return () => {
			cancelAnimationFrame(animId);
			visibility.disconnect();
			releaseWheel();
			controls.dispose();
			for (const node of nodes) {
				node.material.dispose();
				node.texture.dispose();
				node.labelMaterial.dispose();
			}
			// Every Sprite shares one module-level geometry that three never disposes. Its
			// 'dispose' listener would keep this renderer's internals reachable after unmount;
			// disposing it drops them (three re-uploads it the next time a sprite renders).
			nodes[0].label.geometry.dispose();
			sphereGeometry.dispose();
			coreGeometry.dispose();
			coreMaterial.dispose();
			ringGeometry.dispose();
			ringMaterial.dispose();
			keyLight.dispose();
			fillLight.dispose();
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
				{@attach orbit}
				bind:clientWidth={width}
				bind:clientHeight={height}
				class="block h-full w-full cursor-grab active:cursor-grabbing"
				aria-hidden="true"
			></canvas>
		</div>
		<p
			class="pointer-events-none absolute right-3 bottom-2 text-[10px]"
			style="color: var(--calc-text-muted);"
		>
			<span class="pointer-coarse:hidden">Drag to rotate · Ctrl + scroll to zoom</span>
			<span class="hidden pointer-coarse:inline">Swipe sideways to rotate · Pinch to zoom</span>
		</p>
	{/if}
</div>

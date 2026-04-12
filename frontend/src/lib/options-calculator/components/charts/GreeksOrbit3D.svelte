<script lang="ts">
	import { browser } from '$app/environment';

	interface Props {
		delta?: number;
		gamma?: number;
		theta?: number;
		vega?: number;
		rho?: number;
	}

	let { delta = 0, gamma = 0, theta = 0, vega = 0, rho = 0 }: Props = $props();

	let containerEl: HTMLDivElement | undefined = $state();
	let canvasEl: HTMLCanvasElement | undefined = $state();
	let isReady = $state(false);

	// Normalization ranges for each Greek
	function normalize(value: number, max: number): number {
		return Math.min(Math.abs(value) / max, 1);
	}

	const GREEKS_CONFIG = [
		{ name: 'Delta', symbol: 'Δ', maxRange: 1 },
		{ name: 'Gamma', symbol: 'Γ', maxRange: 0.1 },
		{ name: 'Theta', symbol: 'Θ', maxRange: 0.5 },
		{ name: 'Vega', symbol: 'ν', maxRange: 1 },
		{ name: 'Rho', symbol: 'ρ', maxRange: 0.1 }
	] as const;

	$effect(() => {
		if (!browser || !containerEl || !canvasEl) return;

		let disposed = false;
		let animId = 0;

		async function init() {
			const THREE = await import('three');
			const { OrbitControls } = await import('three/addons/controls/OrbitControls.js');

			if (disposed || !containerEl || !canvasEl) return;

			// --- Scene ---
			const scene = new THREE.Scene();

			const camera = new THREE.PerspectiveCamera(50, containerEl.clientWidth / containerEl.clientHeight, 0.1, 100);
			camera.position.set(2, 2.5, 6);
			camera.lookAt(0, 0, 0);

			const renderer = new THREE.WebGLRenderer({ canvas: canvasEl, antialias: true, alpha: true });
			renderer.setSize(containerEl.clientWidth, containerEl.clientHeight);
			renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
			renderer.setClearColor(0x000000, 0);

			// --- Controls ---
			const controls = new OrbitControls(camera, renderer.domElement);
			controls.enableDamping = true;
			controls.dampingFactor = 0.08;
			controls.autoRotate = true;
			controls.autoRotateSpeed = 0.8;
			controls.enablePan = false;
			controls.minDistance = 3;
			controls.maxDistance = 12;

			// --- Lighting ---
			const ambientLight = new THREE.AmbientLight(0x404060, 0.5);
			scene.add(ambientLight);

			const pointLight = new THREE.PointLight(0xffffff, 1.2, 30);
			pointLight.position.set(4, 6, 4);
			scene.add(pointLight);

			const fillLight = new THREE.PointLight(0x6366f1, 0.4, 20);
			fillLight.position.set(-4, -2, -3);
			scene.add(fillLight);

			// --- Central icosahedron ---
			const icoGeometry = new THREE.IcosahedronGeometry(0.25, 0);
			const icoMaterial = new THREE.MeshBasicMaterial({
				color: 0x6366f1,
				wireframe: true,
				transparent: true,
				opacity: 0.6
			});
			const icosahedron = new THREE.Mesh(icoGeometry, icoMaterial);
			scene.add(icosahedron);

			// --- Greek spheres ---
			const EMERALD = 0x10b981;
			const RED = 0xef4444;
			const orbitRadius = 2.2;
			const spheres: any[] = [];
			const labels: any[] = [];
			const lines: any[] = [];

			function getGreekValue(index: number): number {
				switch (index) {
					case 0: return delta;
					case 1: return gamma;
					case 2: return theta;
					case 3: return vega;
					case 4: return rho;
					default: return 0;
				}
			}

			function createTextSprite(text: string, color: string): any {
				const canvas = document.createElement('canvas');
				canvas.width = 256;
				canvas.height = 128;
				const ctx = canvas.getContext('2d')!;
				ctx.clearRect(0, 0, 256, 128);
				ctx.font = 'bold 28px sans-serif';
				ctx.textAlign = 'center';
				ctx.fillStyle = color;
				ctx.fillText(text, 128, 50);
				ctx.font = '22px sans-serif';
				ctx.fillStyle = 'rgba(255, 255, 255, 0.7)';
				ctx.fillText('', 128, 85);

				const texture = new THREE.CanvasTexture(canvas);
				texture.needsUpdate = true;
				const spriteMat = new THREE.SpriteMaterial({ map: texture, transparent: true, depthTest: false });
				const sprite = new THREE.Sprite(spriteMat);
				sprite.scale.set(1.2, 0.6, 1);
				return sprite;
			}

			function updateLabelText(sprite: any, name: string, symbol: string, value: number, isPositive: boolean) {
				const canvas = document.createElement('canvas');
				canvas.width = 256;
				canvas.height = 128;
				const ctx = canvas.getContext('2d')!;
				ctx.clearRect(0, 0, 256, 128);

				ctx.font = 'bold 26px sans-serif';
				ctx.textAlign = 'center';
				ctx.fillStyle = isPositive ? '#10b981' : '#ef4444';
				ctx.fillText(`${name} (${symbol})`, 128, 45);

				ctx.font = '22px monospace';
				ctx.fillStyle = 'rgba(255, 255, 255, 0.8)';
				ctx.fillText(value.toFixed(4), 128, 80);

				const oldTexture = (sprite.material as any).map;
				if (oldTexture) oldTexture.dispose();

				const texture = new THREE.CanvasTexture(canvas);
				texture.needsUpdate = true;
				(sprite.material as any).map = texture;
			}

			// Create each sphere and label
			for (let i = 0; i < 5; i++) {
				const angle = (i / 5) * Math.PI * 2 - Math.PI / 2;
				const val = getGreekValue(i);
				const config = GREEKS_CONFIG[i];
				const normalizedVal = normalize(val, config.maxRange);
				const radius = 0.15 + normalizedVal * 0.6;
				const isPositive = val >= 0;

				const geometry = new THREE.SphereGeometry(radius, 32, 32);
				const color = isPositive ? EMERALD : RED;
				const material = new THREE.MeshStandardMaterial({
					color,
					emissive: new THREE.Color(color),
					emissiveIntensity: 0.3,
					metalness: 0.3,
					roughness: 0.4,
					transparent: true,
					opacity: 0.9
				});
				const sphere = new THREE.Mesh(geometry, material);

				const x = Math.cos(angle) * orbitRadius;
				const z = Math.sin(angle) * orbitRadius;
				sphere.position.set(x, 0, z);
				scene.add(sphere);
				spheres.push(sphere);

				// Label
				const label = createTextSprite(`${config.name}`, isPositive ? '#10b981' : '#ef4444');
				updateLabelText(label, config.name, config.symbol, val, isPositive);
				label.position.set(x, radius + 0.6, z);
				scene.add(label);
				labels.push(label);
			}

			// --- Connection lines between adjacent spheres ---
			const lineGeometries: any[] = [];
			for (let i = 0; i < 5; i++) {
				const next = (i + 1) % 5;
				const points = [spheres[i].position.clone(), spheres[next].position.clone()];
				const geometry = new THREE.BufferGeometry().setFromPoints(points);
				const material = new THREE.LineBasicMaterial({
					color: 0x6366f1,
					transparent: true,
					opacity: 0.2
				});
				const line = new THREE.Line(geometry, material);
				scene.add(line);
				lines.push(line);
				lineGeometries.push(geometry);
			}

			isReady = true;

			// --- Update spheres when Greek values change ---
			function updateSpheres() {
				for (let i = 0; i < 5; i++) {
					const val = getGreekValue(i);
					const config = GREEKS_CONFIG[i];
					const normalizedVal = normalize(val, config.maxRange);
					const newRadius = 0.15 + normalizedVal * 0.6;
					const isPositive = val >= 0;
					const color = isPositive ? EMERALD : RED;

					const sphere = spheres[i];
					const oldGeometry = sphere.geometry;
					sphere.geometry = new THREE.SphereGeometry(newRadius, 32, 32);
					oldGeometry.dispose();

					const mat = sphere.material as any;
					mat.color.setHex(color);
					mat.emissive.setHex(color);

					updateLabelText(labels[i], config.name, config.symbol, val, isPositive);
				}
			}

			// --- Animation loop ---
			const clock = new THREE.Clock();

			function animate() {
				if (disposed) return;
				animId = requestAnimationFrame(animate);

				const elapsed = clock.getElapsedTime();

				// Icosahedron rotation
				icosahedron.rotation.x = elapsed * 0.3;
				icosahedron.rotation.y = elapsed * 0.5;

				// Sphere floating and pulsing
				for (let i = 0; i < 5; i++) {
					const angle = (i / 5) * Math.PI * 2 - Math.PI / 2;
					const val = getGreekValue(i);
					const config = GREEKS_CONFIG[i];
					const normalizedVal = normalize(val, config.maxRange);

					// Float up/down with phase offset
					const floatY = Math.sin(elapsed * 1.2 + i * 1.26) * 0.15;
					const x = Math.cos(angle) * orbitRadius;
					const z = Math.sin(angle) * orbitRadius;
					spheres[i].position.set(x, floatY, z);

					// Pulse scale
					const pulseScale = 1 + Math.sin(elapsed * 2 + i * 0.8) * 0.08 * (0.3 + normalizedVal * 0.7);
					spheres[i].scale.setScalar(pulseScale);

					// Update label position to track sphere
					const sphereRadius = 0.15 + normalizedVal * 0.6;
					labels[i].position.set(x, floatY + sphereRadius + 0.6, z);
				}

				// Update connection line positions
				for (let i = 0; i < 5; i++) {
					const next = (i + 1) % 5;
					const positions = lines[i].geometry.attributes.position as any;
					positions.setXYZ(0, spheres[i].position.x, spheres[i].position.y, spheres[i].position.z);
					positions.setXYZ(1, spheres[next].position.x, spheres[next].position.y, spheres[next].position.z);
					positions.needsUpdate = true;
				}

				controls.update();
				renderer.render(scene, camera);
			}
			animate();

			// --- Watch for Greek value changes ---
			const unwatchValues = $effect.root(() => {
				$effect(() => {
					void delta;
					void gamma;
					void theta;
					void vega;
					void rho;
					updateSpheres();
				});
			});

			// --- ResizeObserver ---
			const resizeObserver = new ResizeObserver((entries) => {
				for (const entry of entries) {
					const w = entry.contentRect.width;
					const h = entry.contentRect.height;
					if (w > 0 && h > 0) {
						camera.aspect = w / h;
						camera.updateProjectionMatrix();
						renderer.setSize(w, h);
					}
				}
			});
			resizeObserver.observe(containerEl!);

			// --- Cleanup ---
			return () => {
				disposed = true;
				cancelAnimationFrame(animId);
				unwatchValues();
				resizeObserver.disconnect();

				// Dispose spheres
				for (const sphere of spheres) {
					sphere.geometry.dispose();
					(sphere.material as any).dispose();
				}

				// Dispose labels
				for (const label of labels) {
					const mat = label.material as any;
					if (mat.map) mat.map.dispose();
					mat.dispose();
				}

				// Dispose lines
				for (const line of lines) {
					line.geometry.dispose();
					(line.material as any).dispose();
				}

				// Dispose line geometries
				for (const geo of lineGeometries) {
					geo.dispose();
				}

				// Dispose central icosahedron
				icoGeometry.dispose();
				icoMaterial.dispose();

				controls.dispose();
				renderer.dispose();
			};
		}

		const cleanupPromise = init();

		return () => {
			cleanupPromise.then((cleanup) => cleanup?.());
		};
	});
</script>

<div
	bind:this={containerEl}
	class="greeks-orbit-container"
	style="width: 100%; height: 300px; position: relative;"
>
	<canvas bind:this={canvasEl} style="width: 100%; height: 100%; display: block;"></canvas>
	{#if !isReady}
		<div class="flex items-center justify-center h-full text-sm opacity-50 absolute inset-0" style="color: var(--calc-text-muted);">
			Loading 3D view...
		</div>
	{/if}
</div>

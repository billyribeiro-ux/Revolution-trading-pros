<script lang="ts">
	/**
	 * Scene3D - 3D Trading Scene Content
	 * Netflix L11+ Principal Engineer Grade
	 *
	 * Contains camera, lights, and 3D candlestick objects
	 *
	 * @version 1.0.0
	 */
	import { T, useTask } from '@threlte/core';
	import { OrbitControls, Float } from '@threlte/extras';
	import { onMount } from 'svelte';
	import * as THREE from 'three';

	// Generate candlestick data
	interface Candle {
		x: number;
		open: number;
		close: number;
		high: number;
		low: number;
		bullish: boolean;
	}

	function generateCandles(count: number): Candle[] {
		const candles: Candle[] = [];
		let price = 50;

		for (let i = 0; i < count; i++) {
			const change = (Math.random() - 0.45) * 15;
			const open = price;
			const close = price + change;
			const high = Math.max(open, close) + Math.random() * 5;
			const low = Math.min(open, close) - Math.random() * 5;

			candles.push({
				x: (i - count / 2) * 3,
				open,
				close,
				high,
				low,
				bullish: close >= open
			});

			price = close;
		}

		return candles;
	}

	let candles = generateCandles(12);

	// Colors
	const bullColor = new THREE.Color('#22c55e');
	const bearColor = new THREE.Color('#ef4444');
	const gridColor = new THREE.Color('#6366f1');

	// Camera rotation
	let cameraAngle = $state(0);
	let time = $state(0);

	useTask((delta) => {
		time += delta * 0.5;
		cameraAngle = Math.sin(time * 0.3) * 0.3;
	});
</script>

<!-- Camera -->
<T.PerspectiveCamera
	makeDefault
	position={[Math.sin(cameraAngle) * 60, 30, Math.cos(cameraAngle) * 60]}
	fov={45}
	near={0.1}
	far={1000}
>
	<OrbitControls
		enableZoom={false}
		enablePan={false}
		autoRotate
		autoRotateSpeed={0.3}
		maxPolarAngle={Math.PI / 2.2}
		minPolarAngle={Math.PI / 4}
	/>
</T.PerspectiveCamera>

<!-- Ambient Light -->
<T.AmbientLight intensity={0.4} color="#c7d2fe" />

<!-- Main Directional Light -->
<T.DirectionalLight position={[20, 40, 20]} intensity={1} color="#ffffff" castShadow />

<!-- Fill Light -->
<T.PointLight position={[-20, 20, -20]} intensity={0.5} color="#818cf8" />

<!-- Grid Floor -->
<T.GridHelper args={[100, 40, gridColor, new THREE.Color('#1e1b4b')]} position.y={-20} />

<!-- Candlesticks -->
{#each candles as candle, i}
	{@const bodyHeight = Math.abs(candle.close - candle.open)}
	{@const bodyCenter = (candle.open + candle.close) / 2 - 50}
	{@const wickHeight = candle.high - candle.low}
	{@const wickCenter = (candle.high + candle.low) / 2 - 50}
	{@const color = candle.bullish ? bullColor : bearColor}

	<Float
		floatIntensity={0.5}
		rotationIntensity={0.1}
		speed={2}
	>
		<T.Group position.x={candle.x} position.z={i * 0.5 - 3}>
			<!-- Candle Body -->
			<T.Mesh position.y={bodyCenter} castShadow>
				<T.BoxGeometry args={[2, Math.max(bodyHeight, 1), 2]} />
				<T.MeshStandardMaterial
					color={color}
					emissive={color}
					emissiveIntensity={0.3}
					metalness={0.2}
					roughness={0.4}
				/>
			</T.Mesh>

			<!-- Candle Wick -->
			<T.Mesh position.y={wickCenter}>
				<T.BoxGeometry args={[0.3, wickHeight, 0.3]} />
				<T.MeshStandardMaterial
					color={color}
					emissive={color}
					emissiveIntensity={0.2}
					metalness={0.1}
					roughness={0.6}
				/>
			</T.Mesh>

			<!-- Glow Ring at Base -->
			<T.Mesh position.y={-20} rotation.x={-Math.PI / 2}>
				<T.RingGeometry args={[1.5, 2, 32]} />
				<T.MeshBasicMaterial
					color={color}
					transparent
					opacity={0.3}
					side={THREE.DoubleSide}
				/>
			</T.Mesh>
		</T.Group>
	</Float>
{/each}

<!-- Floating Particles -->
{#each { length: 30 } as _, i}
	{@const x = (Math.random() - 0.5) * 80}
	{@const y = Math.random() * 40 - 10}
	{@const z = (Math.random() - 0.5) * 80}

	<Float
		floatIntensity={1}
		speed={1 + Math.random()}
	>
		<T.Mesh position={[x, y, z]}>
			<T.SphereGeometry args={[0.15, 8, 8]} />
			<T.MeshBasicMaterial
				color={gridColor}
				transparent
				opacity={0.6}
			/>
		</T.Mesh>
	</Float>
{/each}

<!-- Moving Average Line -->
<T.Line>
	<T.BufferGeometry>
		{@const positions = new Float32Array(
			candles.flatMap((c, i) => {
				const maWindow = 3;
				let sum = 0;
				let count = 0;
				for (let j = Math.max(0, i - maWindow + 1); j <= i; j++) {
					sum += (candles[j].open + candles[j].close) / 2;
					count++;
				}
				const ma = sum / count - 50;
				return [c.x, ma, 0];
			})
		)}
		<T.BufferAttribute
			args={[positions, 3]}
			attach={(parent, self) => {
				parent.setAttribute('position', self);
			}}
		/>
	</T.BufferGeometry>
	<T.LineBasicMaterial color="#8b5cf6" linewidth={2} />
</T.Line>

<!-- Fog for Depth -->
<T.Fog args={['#0a0a1a', 50, 150]} attach="fog" />

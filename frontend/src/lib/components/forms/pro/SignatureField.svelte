<script lang="ts">
	/**
	 * SignatureField Component (FluentForms 6.1.5 - November 2025)
	 *
	 * Digital signature capture field with canvas drawing support.
	 * Works in both regular forms and conversational forms.
	 */

	interface Props {
		name: string;
		label?: string;
		required?: boolean;
		disabled?: boolean;
		width?: number;
		height?: number;
		penColor?: string;
		penWidth?: number;
		backgroundColor?: string;
		showClearButton?: boolean;
		showUndoButton?: boolean;
		error?: string;
		onchange?: (dataUrl: string | null) => void;
	}

	import Icon from '$lib/components/Icon.svelte';

	let {
		name,
		label = 'Signature',
		required = false,
		disabled = false,
		width = 400,
		height = 200,
		penColor = '#000000',
		penWidth = 2,
		backgroundColor = '#ffffff',
		showClearButton = true,
		showUndoButton = true,
		error = '',
		onchange
	}: Props = $props();

	let canvas: HTMLCanvasElement;
	let ctx: CanvasRenderingContext2D | null = null;
	let isDrawing = $state(false);
	let hasSignature = $state(false);
	let signatureData = $state<string | null>(null);
	let paths = $state<{ x: number; y: number }[][]>([]);
	let currentPath = $state<{ x: number; y: number }[]>([]);

	$effect(() => {
		if (canvas) {
			ctx = canvas.getContext('2d');
			if (ctx) {
				ctx.strokeStyle = penColor;
				ctx.lineWidth = penWidth;
				ctx.lineCap = 'round';
				ctx.lineJoin = 'round';
				clearCanvas();
			}
		}
	});

	function getPosition(e: MouseEvent | TouchEvent): { x: number; y: number } {
		const rect = canvas.getBoundingClientRect();
		const scaleX = canvas.width / rect.width;
		const scaleY = canvas.height / rect.height;

		if ('touches' in e) {
			const touch = e.touches[0];
			return {
				x: (touch.clientX - rect.left) * scaleX,
				y: (touch.clientY - rect.top) * scaleY
			};
		} else {
			return {
				x: (e.clientX - rect.left) * scaleX,
				y: (e.clientY - rect.top) * scaleY
			};
		}
	}

	function startDrawing(e: MouseEvent | TouchEvent) {
		if (disabled) return;
		e.preventDefault();

		isDrawing = true;
		currentPath = [];

		const pos = getPosition(e);
		currentPath.push(pos);

		if (ctx) {
			ctx.beginPath();
			ctx.moveTo(pos.x, pos.y);
		}
	}

	function draw(e: MouseEvent | TouchEvent) {
		if (!isDrawing || disabled || !ctx) return;
		e.preventDefault();

		const pos = getPosition(e);
		currentPath.push(pos);

		ctx.lineTo(pos.x, pos.y);
		ctx.stroke();
	}

	function stopDrawing() {
		if (!isDrawing) return;

		isDrawing = false;

		if (currentPath.length > 0) {
			paths = [...paths, currentPath];
			currentPath = [];
			hasSignature = true;
			saveSignature();
		}
	}

	function saveSignature() {
		if (canvas) {
			signatureData = canvas.toDataURL('image/png');
			if (onchange) {
				onchange(signatureData);
			}
		}
	}

	function clearCanvas() {
		if (ctx && canvas) {
			ctx.fillStyle = backgroundColor;
			ctx.fillRect(0, 0, canvas.width, canvas.height);
			ctx.strokeStyle = penColor;
			paths = [];
			currentPath = [];
			hasSignature = false;
			signatureData = null;
			if (onchange) {
				onchange(null);
			}
		}
	}

	function undoLastStroke() {
		if (paths.length === 0 || !ctx) return;

		paths = paths.slice(0, -1);
		redrawCanvas();

		if (paths.length === 0) {
			hasSignature = false;
			signatureData = null;
			if (onchange) {
				onchange(null);
			}
		} else {
			saveSignature();
		}
	}

	function redrawCanvas() {
		if (!ctx || !canvas) return;

		ctx.fillStyle = backgroundColor;
		ctx.fillRect(0, 0, canvas.width, canvas.height);
		ctx.strokeStyle = penColor;
		ctx.lineWidth = penWidth;

		for (const path of paths) {
			if (path.length === 0) continue;

			ctx.beginPath();
			ctx.moveTo(path[0].x, path[0].y);

			for (let i = 1; i < path.length; i++) {
				ctx.lineTo(path[i].x, path[i].y);
			}

			ctx.stroke();
		}
	}
</script>

<div class="signature-field" class:disabled class:has-error={error}>
	<label for="signature-{name}" class="signature-label">
		{label}
		{#if required}
			<span class="required-marker">*</span>
		{/if}
	</label>

	<div class="signature-pad-container" style="max-width: {width}px">
		<canvas
			bind:this={canvas}
			{width}
			{height}
			class="signature-canvas"
			class:has-signature={hasSignature}
			onmousedown={startDrawing}
			onmousemove={draw}
			onmouseup={stopDrawing}
			onmouseleave={stopDrawing}
			ontouchstart={startDrawing}
			ontouchmove={draw}
			ontouchend={stopDrawing}
		></canvas>

		{#if !hasSignature && !disabled}
			<div class="signature-placeholder">
				<Icon name="IconPencil" size={24} />
				<span>Sign here</span>
			</div>
		{/if}

		<div class="signature-actions">
			{#if showUndoButton}
				<button
					type="button"
					class="action-btn undo"
					onclick={undoLastStroke}
					disabled={disabled || paths.length === 0}
					title="Undo"
					aria-label="Undo last stroke"
				>
					<Icon name="IconArrowBackUp" size={16} />
				</button>
			{/if}
			{#if showClearButton}
				<button
					type="button"
					class="action-btn clear"
					onclick={clearCanvas}
					disabled={disabled || !hasSignature}
					title="Clear"
					aria-label="Clear signature"
				>
					<Icon name="IconTrash" size={16} />
				</button>
			{/if}
		</div>
	</div>

	<input id="signature-{name}" type="hidden" {name} value={signatureData || ''} />

	{#if error}
		<div class="error-message">
			<Icon name="IconAlertCircle" size={14} />
			{error}
		</div>
	{/if}

	<p class="signature-hint">Draw your signature using mouse or touch</p>
</div>

<style>
	.signature-field {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.signature-label {
		font-size: 0.9375rem;
		font-weight: 600;
		color: #374151;
	}

	.required-marker {
		color: #ef4444;
		margin-left: 0.25rem;
	}

	.signature-pad-container {
		position: relative;
		width: 100%;
	}

	.signature-canvas {
		display: block;
		width: 100%;
		height: auto;
		border: 2px dashed #d1d5db;
		border-radius: 0.5rem;
		background-color: #ffffff;
		cursor: crosshair;
		touch-action: none;
	}

	.signature-canvas.has-signature {
		border-style: solid;
		border-color: #3b82f6;
	}

	.signature-placeholder {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
		color: #9ca3af;
		pointer-events: none;
	}

	.signature-placeholder span {
		font-size: 0.875rem;
	}

	.signature-actions {
		position: absolute;
		top: 0.5rem;
		right: 0.5rem;
		display: flex;
		gap: 0.25rem;
	}

	/* 2026 Mobile-First: 44px touch targets */
	.action-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 44px; /* Touch target */
		height: 44px; /* Touch target */
		min-width: 44px;
		min-height: 44px;
		background-color: white;
		border: 1px solid #e5e7eb;
		border-radius: 0.375rem;
		color: #6b7280;
		cursor: pointer;
		transition: all 0.15s;
		touch-action: manipulation;
	}

	.action-btn:hover:not(:disabled) {
		background-color: #f3f4f6;
		color: #374151;
	}

	.action-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.action-btn.clear:hover:not(:disabled) {
		background-color: #fef2f2;
		border-color: #fca5a5;
		color: #dc2626;
	}

	.signature-hint {
		margin: 0;
		font-size: 0.75rem;
		color: #9ca3af;
	}

	/* Disabled State */
	.disabled .signature-canvas {
		cursor: not-allowed;
		opacity: 0.6;
	}

	/* Error State */
	.has-error .signature-canvas {
		border-color: #fca5a5;
	}

	.error-message {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: #dc2626;
		font-size: 0.8125rem;
	}

	/* Responsive */
	@media (max-width: 479.98px) {
		.signature-canvas {
			height: 150px;
		}
	}
</style>

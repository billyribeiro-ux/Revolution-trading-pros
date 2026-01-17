<script lang="ts">
	import { onMount } from 'svelte';
	import * as d3 from 'd3';

	interface Props {
		type?: 'line' | 'area' | 'candle' | 'bar' | 'pie' | 'scatter' | 'heatmap' | 'radar';
		size?: number;
		color?: string;
	}

	let { type = 'line', size = 48, color = '#3b82f6' }: Props = $props();

	let svgElement: SVGSVGElement;

	onMount(() => {
		if (!svgElement) return;

		const svg = d3.select(svgElement);
		svg.selectAll('*').remove();

		const margin = size * 0.1;
		const width = size - margin * 2;
		const height = size - margin * 2;

		const g = svg.append('g').attr('transform', `translate(${margin}, ${margin})`);

		switch (type) {
			case 'line':
				drawLineChart(g, width, height, color);
				break;
			case 'area':
				drawAreaChart(g, width, height, color);
				break;
			case 'candle':
				drawCandlestickChart(g, width, height, color);
				break;
			case 'bar':
				drawBarChart(g, width, height, color);
				break;
			case 'pie':
				drawPieChart(g, width, height, color);
				break;
			case 'scatter':
				drawScatterChart(g, width, height, color);
				break;
			case 'heatmap':
				drawHeatmap(g, width, height, color);
				break;
			case 'radar':
				drawRadarChart(g, width, height, color);
				break;
		}
	});

	function drawLineChart(g: any, width: number, height: number, color: string) {
		const data = [0.3, 0.5, 0.4, 0.7, 0.6, 0.8, 0.9, 0.7];
		const x = d3
			.scaleLinear()
			.domain([0, data.length - 1])
			.range([0, width]);
		const y = d3.scaleLinear().domain([0, 1]).range([height, 0]);

		const line = d3
			.line<number>()
			.x((_d: number, i: number) => x(i))
			.y((d: number) => y(d))
			.curve(d3.curveMonotoneX);

		g.append('path')
			.datum(data)
			.attr('fill', 'none')
			.attr('stroke', color)
			.attr('stroke-width', 2)
			.attr('d', line);
	}

	function drawAreaChart(g: any, width: number, height: number, color: string) {
		const data = [0.3, 0.5, 0.4, 0.7, 0.6, 0.8, 0.9, 0.7];
		const x = d3
			.scaleLinear()
			.domain([0, data.length - 1])
			.range([0, width]);
		const y = d3.scaleLinear().domain([0, 1]).range([height, 0]);

		const area = d3
			.area<number>()
			.x((_d: number, i: number) => x(i))
			.y0(height)
			.y1((d: number) => y(d))
			.curve(d3.curveMonotoneX);

		g.append('path').datum(data).attr('fill', color).attr('fill-opacity', 0.3).attr('d', area);

		const line = d3
			.line<number>()
			.x((_d: number, i: number) => x(i))
			.y((d: number) => y(d))
			.curve(d3.curveMonotoneX);

		g.append('path')
			.datum(data)
			.attr('fill', 'none')
			.attr('stroke', color)
			.attr('stroke-width', 2)
			.attr('d', line);
	}

	function drawCandlestickChart(g: any, width: number, height: number, _color: string) {
		const candles = [
			{ open: 0.4, close: 0.6, high: 0.7, low: 0.3 },
			{ open: 0.6, close: 0.5, high: 0.65, low: 0.45 },
			{ open: 0.5, close: 0.7, high: 0.75, low: 0.45 },
			{ open: 0.7, close: 0.8, high: 0.85, low: 0.65 }
		];

		const x = d3
			.scaleBand()
			.domain(candles.map((_d, i) => i.toString()))
			.range([0, width])
			.padding(0.3);
		const y = d3.scaleLinear().domain([0, 1]).range([height, 0]);

		candles.forEach((d, i) => {
			const isGreen = d.close > d.open;
			const candleColor = isGreen ? '#10b981' : '#ef4444';
			const xPos = x(i.toString())! + x.bandwidth() / 2;

			// Wick
			g.append('line')
				.attr('x1', xPos)
				.attr('x2', xPos)
				.attr('y1', y(d.high))
				.attr('y2', y(d.low))
				.attr('stroke', candleColor)
				.attr('stroke-width', 1);

			// Body
			g.append('rect')
				.attr('x', x(i.toString()))
				.attr('y', y(Math.max(d.open, d.close)))
				.attr('width', x.bandwidth())
				.attr('height', Math.abs(y(d.open) - y(d.close)) || 1)
				.attr('fill', candleColor);
		});
	}

	function drawBarChart(g: any, width: number, height: number, color: string) {
		const data = [0.4, 0.6, 0.5, 0.8, 0.7, 0.9];
		const x = d3
			.scaleBand()
			.domain(data.map((_d, i) => i.toString()))
			.range([0, width])
			.padding(0.2);
		const y = d3.scaleLinear().domain([0, 1]).range([height, 0]);

		g.selectAll('rect')
			.data(data)
			.enter()
			.append('rect')
			.attr('x', (_d: number, i: number) => x(i.toString())!)
			.attr('y', (d: number) => y(d))
			.attr('width', x.bandwidth())
			.attr('height', (d: number) => height - y(d))
			.attr('fill', color)
			.attr('rx', 1);
	}

	function drawPieChart(g: any, width: number, height: number, color: string) {
		const data = [30, 20, 25, 25];
		const radius = Math.min(width, height) / 2;

		const pie = d3.pie<number>().value((d: number) => d);
		const arc = d3.arc<any>().innerRadius(0).outerRadius(radius);

		const colors = d3
			.scaleOrdinal()
			.domain(data.map((_d, i) => i.toString()))
			.range([
				color,
				d3.rgb(color).brighter(0.5).toString(),
				d3.rgb(color).brighter(1).toString(),
				d3.rgb(color).darker(0.5).toString()
			]);

		g.attr('transform', `translate(${width / 2}, ${height / 2})`);

		g.selectAll('path')
			.data(pie(data))
			.enter()
			.append('path')
			.attr('d', arc)
			.attr('fill', (_d: any, i: number) => colors(i.toString()) as string)
			.attr('stroke', 'rgba(0,0,0,0.2)')
			.attr('stroke-width', 1);
	}

	function drawScatterChart(g: any, width: number, height: number, color: string) {
		const data = Array.from({ length: 20 }, () => ({
			x: Math.random(),
			y: Math.random()
		}));

		const x = d3.scaleLinear().domain([0, 1]).range([0, width]);
		const y = d3.scaleLinear().domain([0, 1]).range([height, 0]);

		g.selectAll('circle')
			.data(data)
			.enter()
			.append('circle')
			.attr('cx', (d: any) => x(d.x))
			.attr('cy', (d: any) => y(d.y))
			.attr('r', 2)
			.attr('fill', color)
			.attr('opacity', 0.7);
	}

	function drawHeatmap(g: any, width: number, height: number, _color: string) {
		const data = Array.from({ length: 5 }, (_, i) =>
			Array.from({ length: 5 }, (_, j) => ({
				row: i,
				col: j,
				value: Math.random()
			}))
		).flat();

		const cellSize = Math.min(width, height) / 5;
		const colorScale = d3.scaleSequential(d3.interpolateBlues).domain([0, 1]);

		g.selectAll('rect')
			.data(data)
			.enter()
			.append('rect')
			.attr('x', (d: any) => d.col * cellSize)
			.attr('y', (d: any) => d.row * cellSize)
			.attr('width', cellSize - 1)
			.attr('height', cellSize - 1)
			.attr('fill', (d: any) => colorScale(d.value))
			.attr('rx', 1);
	}

	function drawRadarChart(g: any, width: number, height: number, color: string) {
		const data = [0.8, 0.6, 0.9, 0.7, 0.5];
		const radius = Math.min(width, height) / 2;
		const angleSlice = (Math.PI * 2) / data.length;

		g.attr('transform', `translate(${width / 2}, ${height / 2})`);

		// Draw axes
		data.forEach((_d, i) => {
			const angle = angleSlice * i - Math.PI / 2;
			g.append('line')
				.attr('x1', 0)
				.attr('y1', 0)
				.attr('x2', radius * Math.cos(angle))
				.attr('y2', radius * Math.sin(angle))
				.attr('stroke', 'rgba(255,255,255,0.1)')
				.attr('stroke-width', 1);
		});

		// Draw data
		const radarLine = d3
			.lineRadial<number>()
			.angle((_d: number, i: number) => angleSlice * i)
			.radius((d: number) => d * radius)
			.curve(d3.curveLinearClosed);

		g.append('path')
			.datum(data)
			.attr('d', radarLine)
			.attr('fill', color)
			.attr('fill-opacity', 0.3)
			.attr('stroke', color)
			.attr('stroke-width', 2);
	}
</script>

<svg bind:this={svgElement} width={size} height={size} class="chart-icon"></svg>

<style>
	.chart-icon {
		display: block;
	}
</style>

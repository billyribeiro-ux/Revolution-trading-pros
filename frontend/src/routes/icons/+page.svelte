<script lang="ts">
	import ChartIcons from '$lib/components/ChartIcons.svelte';
	import ApexChartIcons from '$lib/components/ApexChartIcons.svelte';
	import ChartJsIcons from '$lib/components/ChartJsIcons.svelte';
	import LightweightChartIcons from '$lib/components/LightweightChartIcons.svelte';
	import IconifyIcon from '@iconify/svelte';

	// D3 Charts variations
	const d3Charts = [
		{ type: 'line', name: 'Line Chart', color: '#3b82f6' },
		{ type: 'line', name: 'Smooth Line', color: '#ef4444' },
		{ type: 'line', name: 'Multi-Line', color: '#06b6d4' },
		{ type: 'line', name: 'Step Chart', color: '#8b5cf6' },
		{ type: 'line', name: 'Curved Line', color: '#10b981' },
		{ type: 'area', name: 'Area Chart', color: '#8b5cf6' },
		{ type: 'area', name: 'Gradient Area', color: '#f59e0b' },
		{ type: 'area', name: 'Stream Graph', color: '#06b6d4' },
		{ type: 'area', name: 'Stacked Area', color: '#10b981' },
		{ type: 'area', name: 'Filled Area', color: '#ec4899' },
		{ type: 'candle', name: 'Candlestick', color: '#10b981' },
		{ type: 'candle', name: 'OHLC Chart', color: '#22c55e' },
		{ type: 'candle', name: 'Hollow Candle', color: '#16a34a' },
		{ type: 'bar', name: 'Bar Chart', color: '#f59e0b' },
		{ type: 'bar', name: 'Grouped Bar', color: '#84cc16' },
		{ type: 'bar', name: 'Stacked Bar', color: '#eab308' },
		{ type: 'pie', name: 'Pie Chart', color: '#ec4899' },
		{ type: 'pie', name: 'Donut Chart', color: '#f472b6' },
		{ type: 'scatter', name: 'Scatter Plot', color: '#06b6d4' },
		{ type: 'scatter', name: 'Bubble Chart', color: '#a855f7' },
		{ type: 'heatmap', name: 'Heatmap', color: '#8b5cf6' },
		{ type: 'radar', name: 'Radar Chart', color: '#10b981' }
	] as const;

	// FontAwesome Solid icons via @iconify/svelte
	const fontAwesomeSolid = [
		{ icon: 'fa-solid:chart-line', name: 'Chart Line' },
		{ icon: 'fa-solid:chart-area', name: 'Chart Area' },
		{ icon: 'fa-solid:chart-bar', name: 'Chart Bar' },
		{ icon: 'fa-solid:chart-pie', name: 'Chart Pie' },
		{ icon: 'fa6-solid:chart-column', name: 'Chart Column' },
		{ icon: 'fa6-solid:chart-simple', name: 'Chart Simple' },
		{ icon: 'fa6-solid:chart-gantt', name: 'Chart Gantt' },
		{ icon: 'fa6-solid:arrow-trend-up', name: 'Trend Up' },
		{ icon: 'fa6-solid:arrow-trend-down', name: 'Trend Down' },
		{ icon: 'fa-solid:arrow-up', name: 'Arrow Up' },
		{ icon: 'fa-solid:arrow-down', name: 'Arrow Down' },
		{ icon: 'fa-solid:arrow-right', name: 'Arrow Right' },
		{ icon: 'fa-solid:arrow-left', name: 'Arrow Left' },
		{ icon: 'fa-solid:angles-up', name: 'Angles Up' },
		{ icon: 'fa-solid:angles-down', name: 'Angles Down' },
		{ icon: 'fa-solid:circle-up', name: 'Circle Up' },
		{ icon: 'fa-solid:circle-down', name: 'Circle Down' },
		{ icon: 'fa6-solid:square-arrow-up-right', name: 'Square Arrow' },
		{ icon: 'fa6-solid:arrows-up-down', name: 'Arrows UpDown' },
		{ icon: 'fa-solid:arrows-alt-h', name: 'Arrows LeftRight' },
		{ icon: 'fa-solid:redo', name: 'Rotate Right' },
		{ icon: 'fa6-solid:rotate', name: 'Rotate' },
		{ icon: 'fa-solid:sync', name: 'Sync' },
		{ icon: 'fa6-solid:money-bill-trend-up', name: 'Money Trend Up' },
		{ icon: 'fa-solid:money-bill', name: 'Money Bill' },
		{ icon: 'fa6-solid:money-bill-1', name: 'Money Bill 1' },
		{ icon: 'fa6-solid:money-bills', name: 'Money Bills' },
		{ icon: 'fa-solid:wallet', name: 'Wallet' },
		{ icon: 'fa-solid:coins', name: 'Coins' },
		{ icon: 'fa-solid:sack-dollar', name: 'Sack Dollar' },
		{ icon: 'fa-solid:hand-holding-usd', name: 'Hand Dollar' },
		{ icon: 'fa-solid:dollar-sign', name: 'Dollar Sign' },
		{ icon: 'fa-solid:money-check', name: 'Money Check' },
		{ icon: 'fa-solid:credit-card', name: 'Credit Card' },
		{ icon: 'fa-solid:receipt', name: 'Receipt' },
		{ icon: 'fa-solid:cash-register', name: 'Cash Register' },
		{ icon: 'fa-solid:percent', name: 'Percent' },
		{ icon: 'fa-solid:calculator', name: 'Calculator' },
		{ icon: 'fa6-solid:building-columns', name: 'Building Columns' },
		{ icon: 'fa-solid:landmark', name: 'Landmark' },
		{ icon: 'fa-solid:university', name: 'Bank' },
		{ icon: 'fa-solid:university', name: 'University' },
		{ icon: 'fa-solid:building', name: 'Building' },
		{ icon: 'fa-solid:trophy', name: 'Trophy' },
		{ icon: 'fa-solid:medal', name: 'Medal' },
		{ icon: 'fa-solid:award', name: 'Award' },
		{ icon: 'fa-solid:star', name: 'Star' },
		{ icon: 'fa-solid:crown', name: 'Crown' },
		{ icon: 'fa-solid:gem', name: 'Gem' },
		{ icon: 'fa6-solid:gauge-high', name: 'Gauge High' },
		{ icon: 'fa6-solid:gauge', name: 'Gauge' },
		{ icon: 'fa-solid:stopwatch', name: 'Stopwatch' },
		{ icon: 'fa-solid:clock', name: 'Clock' },
		{ icon: 'fa-solid:hourglass', name: 'Hourglass' },
		{ icon: 'fa-solid:bullseye', name: 'Bullseye' },
		{ icon: 'fa-solid:crosshairs', name: 'Crosshairs' },
		{ icon: 'fa-solid:compass', name: 'Compass' },
		{ icon: 'fa-solid:location-dot', name: 'Location Dot' },
		{ icon: 'fa-solid:map-location-dot', name: 'Map Location' },
		{ icon: 'fa-solid:shield-halved', name: 'Shield Halved' },
		{ icon: 'fa-solid:shield', name: 'Shield' },
		{ icon: 'fa-solid:lock', name: 'Lock' },
		{ icon: 'fa-solid:key', name: 'Key' },
		{ icon: 'fa-solid:scale-balanced', name: 'Scale Balanced' },
		{ icon: 'fa-solid:eye', name: 'Eye' },
		{ icon: 'fa-solid:magnifying-glass', name: 'Magnifying Glass' },
		{ icon: 'fa6-solid:magnifying-glass-chart', name: 'Mag Glass Chart' },
		{ icon: 'fa6-solid:magnifying-glass-dollar', name: 'Mag Glass Dollar' },
		{ icon: 'fa-solid:brain', name: 'Brain' },
		{ icon: 'fa-solid:lightbulb', name: 'Lightbulb' },
		{ icon: 'fa-solid:flask', name: 'Flask' },
		{ icon: 'fa-solid:bell', name: 'Bell' },
		{ icon: 'fa-solid:bullhorn', name: 'Bullhorn' },
		{ icon: 'fa6-solid:triangle-exclamation', name: 'Triangle Exclamation' },
		{ icon: 'fa-solid:exclamation-circle', name: 'Circle Exclamation' },
		{ icon: 'fa-solid:info-circle', name: 'Circle Info' },
		{ icon: 'fa-solid:check-circle', name: 'Circle Check' },
		{ icon: 'fa-solid:cog', name: 'Gear' },
		{ icon: 'fa-solid:sliders-h', name: 'Sliders' },
		{ icon: 'fa-solid:wrench', name: 'Wrench' },
		{ icon: 'fa-solid:toolbox', name: 'Toolbox' },
		{ icon: 'fa-solid:database', name: 'Database' },
		{ icon: 'fa-solid:server', name: 'Server' },
		{ icon: 'fa-solid:microchip', name: 'Microchip' },
		{ icon: 'fa-solid:memory', name: 'Memory' },
		{ icon: 'fa-solid:network-wired', name: 'Network Wired' },
		{ icon: 'fa-solid:signal', name: 'Signal' },
		{ icon: 'fa-solid:layer-group', name: 'Layer Group' },
		{ icon: 'fa-solid:cubes', name: 'Cubes' },
		{ icon: 'fa-solid:user', name: 'User' },
		{ icon: 'fa-solid:users', name: 'Users' }
	];

	// Heroicons via @iconify/svelte
	const heroicons = [
		{ icon: 'heroicons:chart-bar', name: 'Chart Bar' },
		{ icon: 'heroicons:chart-pie', name: 'Chart Pie' },
		{ icon: 'heroicons:presentation-chart-line', name: 'Presentation Chart Line' },
		{ icon: 'heroicons:presentation-chart-bar', name: 'Presentation Chart Bar' },
		{ icon: 'heroicons:document-chart-bar', name: 'Document Chart Bar' },
		{ icon: 'heroicons:arrow-trending-up', name: 'Trending Up' },
		{ icon: 'heroicons:arrow-trending-down', name: 'Trending Down' },
		{ icon: 'heroicons:arrow-up', name: 'Arrow Up' },
		{ icon: 'heroicons:arrow-down', name: 'Arrow Down' },
		{ icon: 'heroicons:arrow-right', name: 'Arrow Right' },
		{ icon: 'heroicons:arrow-left', name: 'Arrow Left' },
		{ icon: 'heroicons:arrows-up-down', name: 'Arrows UpDown' },
		{ icon: 'heroicons:arrows-right-left', name: 'Arrows RightLeft' },
		{ icon: 'heroicons:arrow-path', name: 'Arrow Path' },
		{ icon: 'heroicons:arrow-path-rounded-square', name: 'Arrow Path Rounded' },
		{ icon: 'heroicons:banknotes', name: 'Banknotes' },
		{ icon: 'heroicons:currency-dollar', name: 'Currency Dollar' },
		{ icon: 'heroicons:credit-card', name: 'Credit Card' },
		{ icon: 'heroicons:receipt-percent', name: 'Receipt Percent' },
		{ icon: 'heroicons:receipt-refund', name: 'Receipt Refund' },
		{ icon: 'heroicons:building-office-2', name: 'Building Office' },
		{ icon: 'heroicons:building-library', name: 'Building Library' },
		{ icon: 'heroicons:building-storefront', name: 'Building Storefront' },
		{ icon: 'heroicons:home', name: 'Home' },
		{ icon: 'heroicons:home-modern', name: 'Home Modern' },
		{ icon: 'heroicons:trophy', name: 'Trophy' },
		{ icon: 'heroicons:star', name: 'Star' },
		{ icon: 'heroicons:fire', name: 'Fire' },
		{ icon: 'heroicons:bolt', name: 'Bolt' },
		{ icon: 'heroicons:sparkles', name: 'Sparkles' },
		{ icon: 'heroicons:rocket-launch', name: 'Rocket Launch' },
		{ icon: 'heroicons:clock', name: 'Clock' },
		{ icon: 'heroicons:calendar', name: 'Calendar' },
		{ icon: 'heroicons:calendar-days', name: 'Calendar Days' },
		{ icon: 'heroicons:map', name: 'Map' },
		{ icon: 'heroicons:map-pin', name: 'Map Pin' },
		{ icon: 'heroicons:globe-alt', name: 'Globe' },
		{ icon: 'heroicons:globe-americas', name: 'Globe Americas' },
		{ icon: 'heroicons:shield-check', name: 'Shield Check' },
		{ icon: 'heroicons:shield-exclamation', name: 'Shield Exclamation' },
		{ icon: 'heroicons:lock-closed', name: 'Lock Closed' },
		{ icon: 'heroicons:lock-open', name: 'Lock Open' },
		{ icon: 'heroicons:key', name: 'Key' },
		{ icon: 'heroicons:scale', name: 'Scale' },
		{ icon: 'heroicons:eye', name: 'Eye' },
		{ icon: 'heroicons:eye-slash', name: 'Eye Slash' },
		{ icon: 'heroicons:magnifying-glass', name: 'Magnifying Glass' },
		{ icon: 'heroicons:magnifying-glass-circle', name: 'Mag Glass Circle' },
		{ icon: 'heroicons:magnifying-glass-plus', name: 'Mag Glass Plus' },
		{ icon: 'heroicons:magnifying-glass-minus', name: 'Mag Glass Minus' },
		{ icon: 'heroicons:light-bulb', name: 'Light Bulb' },
		{ icon: 'heroicons:beaker', name: 'Beaker' },
		{ icon: 'heroicons:academic-cap', name: 'Academic Cap' },
		{ icon: 'heroicons:bell', name: 'Bell' },
		{ icon: 'heroicons:bell-alert', name: 'Bell Alert' },
		{ icon: 'heroicons:bell-slash', name: 'Bell Slash' },
		{ icon: 'heroicons:exclamation-circle', name: 'Exclamation Circle' },
		{ icon: 'heroicons:exclamation-triangle', name: 'Exclamation Triangle' },
		{ icon: 'heroicons:information-circle', name: 'Information Circle' },
		{ icon: 'heroicons:check-circle', name: 'Check Circle' },
		{ icon: 'heroicons:cog-6-tooth', name: 'Cog 6' },
		{ icon: 'heroicons:cog-8-tooth', name: 'Cog 8' },
		{ icon: 'heroicons:adjustments-horizontal', name: 'Adjustments Horizontal' },
		{ icon: 'heroicons:adjustments-vertical', name: 'Adjustments Vertical' },
		{ icon: 'heroicons:calculator', name: 'Calculator' },
		{ icon: 'heroicons:wrench-screwdriver', name: 'Wrench Screwdriver' },
		{ icon: 'heroicons:cpu-chip', name: 'CPU Chip' },
		{ icon: 'heroicons:server', name: 'Server' },
		{ icon: 'heroicons:circle-stack', name: 'Circle Stack' },
		{ icon: 'heroicons:command-line', name: 'Command Line' },
		{ icon: 'heroicons:wifi', name: 'WiFi' },
		{ icon: 'heroicons:square-3-stack-3d', name: 'Square Stack 3D' },
		{ icon: 'heroicons:squares-2x2', name: 'Squares 2x2' },
		{ icon: 'heroicons:folder', name: 'Folder' },
		{ icon: 'heroicons:document', name: 'Document' },
		{ icon: 'heroicons:clipboard-document-list', name: 'Clipboard List' },
		{ icon: 'heroicons:user', name: 'User' },
		{ icon: 'heroicons:users', name: 'Users' },
		{ icon: 'heroicons:user-group', name: 'User Group' },
		{ icon: 'heroicons:user-circle', name: 'User Circle' }
	];

	// ApexCharts types
	const apexCharts = [
		{ type: 'line', name: 'Apex Line' },
		{ type: 'area', name: 'Apex Area' },
		{ type: 'bar', name: 'Apex Bar' },
		{ type: 'pie', name: 'Apex Pie' },
		{ type: 'donut', name: 'Apex Donut' },
		{ type: 'radialBar', name: 'Apex Radial' },
		{ type: 'scatter', name: 'Apex Scatter' },
		{ type: 'bubble', name: 'Apex Bubble' },
		{ type: 'heatmap', name: 'Apex Heatmap' },
		{ type: 'treemap', name: 'Apex Treemap' },
		{ type: 'candlestick', name: 'Apex Candlestick' },
		{ type: 'boxPlot', name: 'Apex BoxPlot' },
		{ type: 'radar', name: 'Apex Radar' },
		{ type: 'polarArea', name: 'Apex Polar' },
		{ type: 'rangeBar', name: 'Apex Range Bar' }
	];

	// Chart.js types
	const chartJsCharts = [
		{ type: 'line', name: 'ChartJS Line' },
		{ type: 'bar', name: 'ChartJS Bar' },
		{ type: 'pie', name: 'ChartJS Pie' },
		{ type: 'doughnut', name: 'ChartJS Doughnut' },
		{ type: 'polarArea', name: 'ChartJS Polar' },
		{ type: 'radar', name: 'ChartJS Radar' },
		{ type: 'scatter', name: 'ChartJS Scatter' },
		{ type: 'bubble', name: 'ChartJS Bubble' }
	];

	// Lightweight Charts types
	const lightweightCharts = [
		{ type: 'line', name: 'LW Line' },
		{ type: 'area', name: 'LW Area' },
		{ type: 'candlestick', name: 'LW Candlestick' },
		{ type: 'bar', name: 'LW Bar' },
		{ type: 'histogram', name: 'LW Histogram' }
	];

	const totalIcons =
		d3Charts.length +
		apexCharts.length +
		chartJsCharts.length +
		lightweightCharts.length +
		fontAwesomeSolid.length +
		heroicons.length;
</script>

<div class="bg-slate-950 text-white p-4 md:p-8">
	<div class="max-w-7xl mx-auto">
		<div class="mb-12 text-center">
			<h1
				class="text-4xl md:text-6xl font-bold mb-4 bg-linear-to-r from-blue-400 via-purple-400 to-pink-400 bg-clip-text text-transparent"
			>
				TRADING ICON LIBRARY
			</h1>
			<p class="text-slate-400 text-lg md:text-xl mb-2">
				{totalIcons} TRADING & DASHBOARD ICONS
			</p>
			<p class="text-slate-500 text-sm">
				D3: {d3Charts.length} • Apex: {apexCharts.length} • ChartJS: {chartJsCharts.length} • LW: {lightweightCharts.length}
				• FA: {fontAwesomeSolid.length} • Hero: {heroicons.length}
			</p>
		</div>

		<!-- D3 Charts -->
		<div class="mb-16">
			<div class="flex items-center gap-4 mb-8">
				<h2
					class="text-3xl font-bold bg-linear-to-r from-yellow-400 to-orange-500 bg-clip-text text-transparent"
				>
					D3.js Charts
				</h2>
				<span
					class="px-3 py-1 bg-yellow-500/20 border border-yellow-500/30 rounded-full text-sm font-bold text-yellow-300"
					>{d3Charts.length}</span
				>
			</div>
			<div class="grid grid-cols-4 sm:grid-cols-6 md:grid-cols-8 lg:grid-cols-10 gap-3">
				{#each d3Charts as chart}
					<div
						class="group bg-slate-900/50 border border-white/5 rounded-xl p-3 hover:border-yellow-500/30 transition-all hover:-translate-y-1"
					>
						<div class="flex flex-col items-center gap-2">
							<ChartIcons type={chart.type as any} size={40} color={chart.color} />
							<span
								class="text-[9px] text-center text-slate-500 group-hover:text-slate-300 leading-tight"
								>{chart.name}</span
							>
						</div>
					</div>
				{/each}
			</div>
		</div>

		<!-- ApexCharts -->
		<div class="mb-16">
			<div class="flex items-center gap-4 mb-8">
				<h2
					class="text-3xl font-bold bg-linear-to-r from-pink-400 to-rose-500 bg-clip-text text-transparent"
				>
					ApexCharts
				</h2>
				<span
					class="px-3 py-1 bg-pink-500/20 border border-pink-500/30 rounded-full text-sm font-bold text-pink-300"
					>{apexCharts.length}</span
				>
			</div>
			<div class="grid grid-cols-4 sm:grid-cols-6 md:grid-cols-8 lg:grid-cols-10 gap-3">
				{#each apexCharts as chart}
					<div
						class="group bg-slate-900/50 border border-white/5 rounded-xl p-3 hover:border-pink-500/30 transition-all hover:-translate-y-1"
					>
						<div class="flex flex-col items-center gap-2">
							<ApexChartIcons type={chart.type} size={40} />
							<span
								class="text-[9px] text-center text-slate-500 group-hover:text-slate-300 leading-tight"
								>{chart.name}</span
							>
						</div>
					</div>
				{/each}
			</div>
		</div>

		<!-- Chart.js -->
		<div class="mb-16">
			<div class="flex items-center gap-4 mb-8">
				<h2
					class="text-3xl font-bold bg-linear-to-r from-indigo-400 to-violet-500 bg-clip-text text-transparent"
				>
					Chart.js
				</h2>
				<span
					class="px-3 py-1 bg-indigo-500/20 border border-indigo-500/30 rounded-full text-sm font-bold text-indigo-300"
					>{chartJsCharts.length}</span
				>
			</div>
			<div class="grid grid-cols-4 sm:grid-cols-6 md:grid-cols-8 lg:grid-cols-10 gap-3">
				{#each chartJsCharts as chart}
					<div
						class="group bg-slate-900/50 border border-white/5 rounded-xl p-3 hover:border-indigo-500/30 transition-all hover:-translate-y-1"
					>
						<div class="flex flex-col items-center gap-2">
							<ChartJsIcons type={chart.type} size={40} />
							<span
								class="text-[9px] text-center text-slate-500 group-hover:text-slate-300 leading-tight"
								>{chart.name}</span
							>
						</div>
					</div>
				{/each}
			</div>
		</div>

		<!-- Lightweight Charts -->
		<div class="mb-16">
			<div class="flex items-center gap-4 mb-8">
				<h2
					class="text-3xl font-bold bg-linear-to-r from-teal-400 to-cyan-500 bg-clip-text text-transparent"
				>
					Lightweight Charts
				</h2>
				<span
					class="px-3 py-1 bg-teal-500/20 border border-teal-500/30 rounded-full text-sm font-bold text-teal-300"
					>{lightweightCharts.length}</span
				>
			</div>
			<div class="grid grid-cols-4 sm:grid-cols-6 md:grid-cols-8 lg:grid-cols-10 gap-3">
				{#each lightweightCharts as chart}
					<div
						class="group bg-slate-900/50 border border-white/5 rounded-xl p-3 hover:border-teal-500/30 transition-all hover:-translate-y-1"
					>
						<div class="flex flex-col items-center gap-2">
							<LightweightChartIcons type={chart.type} size={40} />
							<span
								class="text-[9px] text-center text-slate-500 group-hover:text-slate-300 leading-tight"
								>{chart.name}</span
							>
						</div>
					</div>
				{/each}
			</div>
		</div>

		<!-- FontAwesome Solid -->
		<div class="mb-16">
			<div class="flex items-center gap-4 mb-8">
				<h2 class="text-3xl font-bold text-blue-400">FontAwesome Trading</h2>
				<span
					class="px-3 py-1 bg-blue-500/20 border border-blue-500/30 rounded-full text-sm font-bold text-blue-300"
					>{fontAwesomeSolid.length}</span
				>
			</div>
			<div class="grid grid-cols-4 sm:grid-cols-6 md:grid-cols-8 lg:grid-cols-10 gap-3">
				{#each fontAwesomeSolid as item}
					<div
						class="group bg-slate-900/50 border border-white/5 rounded-xl p-3 hover:border-blue-500/30 transition-all hover:-translate-y-1"
					>
						<div class="flex flex-col items-center gap-2">
							<div class="text-slate-400 group-hover:text-blue-400 transition-colors">
								<FontAwesomeIcon icon={item.icon} class="w-7 h-7" />
							</div>
							<span
								class="text-[9px] text-center text-slate-500 group-hover:text-slate-300 leading-tight"
								>{item.name}</span
							>
						</div>
					</div>
				{/each}
			</div>
		</div>

		<!-- Heroicons -->
		<div class="mb-16">
			<div class="flex items-center gap-4 mb-8">
				<h2 class="text-3xl font-bold text-purple-400">Heroicons Trading</h2>
				<span
					class="px-3 py-1 bg-purple-500/20 border border-purple-500/30 rounded-full text-sm font-bold text-purple-300"
					>{heroicons.length}</span
				>
			</div>
			<div class="grid grid-cols-4 sm:grid-cols-6 md:grid-cols-8 lg:grid-cols-10 gap-3">
				{#each heroicons as item}
					{@const HeroIcon = item.icon}
					<div
						class="group bg-slate-900/50 border border-white/5 rounded-xl p-3 hover:border-purple-500/30 transition-all hover:-translate-y-1"
					>
						<div class="flex flex-col items-center gap-2">
							<div class="text-slate-400 group-hover:text-purple-400 transition-colors">
								<HeroIcon class="w-7 h-7" />
							</div>
							<span
								class="text-[9px] text-center text-slate-500 group-hover:text-slate-300 leading-tight"
								>{item.name}</span
							>
						</div>
					</div>
				{/each}
			</div>
		</div>

		<div class="text-center py-8 border-t border-slate-800">
			<p class="text-emerald-400 text-lg font-bold mb-2">
				{totalIcons} TRADING & DASHBOARD ICONS
			</p>
			<p class="text-slate-500 text-sm mb-2">
				Charts • Trends • Money • Markets • Performance • Analytics • Risk • Monitoring
			</p>
			<p class="text-slate-600 text-xs">D3.js v7.9 • FontAwesome v7.1 • Heroicons v2.0</p>
		</div>
	</div>
</div>

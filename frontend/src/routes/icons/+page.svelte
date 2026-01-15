<script lang="ts">
    import ChartIcons from '$lib/components/ChartIcons.svelte';
    import ApexChartIcons from '$lib/components/ApexChartIcons.svelte';
    import ChartJsIcons from '$lib/components/ChartJsIcons.svelte';
    import LightweightChartIcons from '$lib/components/LightweightChartIcons.svelte';
    import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';

    // FontAwesome Solid - TRADING ICONS
    import {
        faChartLine, faChartArea, faChartBar, faChartPie, faChartColumn, faChartSimple, faChartGantt,
        faArrowTrendUp, faArrowTrendDown, faArrowUp, faArrowDown, faArrowRight, faArrowLeft,
        faAnglesUp, faAnglesDown, faCircleUp, faCircleDown, faSquareArrowUpRight,
        faArrowsUpDown, faArrowsLeftRight, faArrowRotateRight, faRotate, faSync,
        faMoneyBillTrendUp, faMoneyBill, faMoneyBill1, faMoneyBills, faWallet,
        faCoins, faSackDollar, faHandHoldingDollar, faDollarSign, faMoneyCheck,
        faCreditCard, faReceipt, faCashRegister, faPercent, faCalculator,
        faBuildingColumns, faLandmark, faBank, faUniversity, faBuilding,
        faTrophy, faMedal, faAward, faStar, faCrown, faGem,
        faGaugeHigh, faGauge, faStopwatch, faClock, faHourglass,
        faBullseye, faCrosshairs, faCompass, faLocationDot, faMapLocationDot,
        faShieldHalved, faShield, faLock, faKey, faScaleBalanced,
        faEye, faMagnifyingGlass, faMagnifyingGlassChart, faMagnifyingGlassDollar,
        faBrain, faLightbulb, faFlask,
        faBell, faBullhorn, faTriangleExclamation, faCircleExclamation, faCircleInfo, faCircleCheck,
        faGear, faSliders, faWrench, faToolbox,
        faDatabase, faServer, faMicrochip, faMemory, faNetworkWired, faSignal,
        faLayerGroup, faCubes, faUser, faUsers
    } from '@fortawesome/free-solid-svg-icons';

    // Heroicons - TRADING ICONS
    import {
        ChartBarIcon, ChartPieIcon, PresentationChartLineIcon, PresentationChartBarIcon, DocumentChartBarIcon,
        ArrowTrendingUpIcon, ArrowTrendingDownIcon, ArrowUpIcon, ArrowDownIcon, ArrowRightIcon, ArrowLeftIcon,
        ArrowsUpDownIcon, ArrowsRightLeftIcon, ArrowPathIcon, ArrowPathRoundedSquareIcon,
        BanknotesIcon, CurrencyDollarIcon, CreditCardIcon, ReceiptPercentIcon, ReceiptRefundIcon,
        BuildingOffice2Icon, BuildingLibraryIcon, BuildingStorefrontIcon, HomeIcon, HomeModernIcon,
        TrophyIcon, StarIcon, FireIcon, BoltIcon, SparklesIcon, RocketLaunchIcon,
        ClockIcon, CalendarIcon, CalendarDaysIcon,
        MapIcon, MapPinIcon, GlobeAltIcon, GlobeAmericasIcon,
        ShieldCheckIcon, ShieldExclamationIcon, LockClosedIcon, LockOpenIcon, KeyIcon, ScaleIcon,
        EyeIcon, EyeSlashIcon, MagnifyingGlassIcon, MagnifyingGlassCircleIcon, MagnifyingGlassPlusIcon, MagnifyingGlassMinusIcon,
        LightBulbIcon, BeakerIcon, AcademicCapIcon,
        BellIcon, BellAlertIcon, BellSlashIcon, ExclamationCircleIcon, ExclamationTriangleIcon, InformationCircleIcon, CheckCircleIcon,
        Cog6ToothIcon, Cog8ToothIcon, AdjustmentsHorizontalIcon, AdjustmentsVerticalIcon, CalculatorIcon, WrenchScrewdriverIcon,
        CpuChipIcon, ServerIcon, CircleStackIcon, CommandLineIcon, WifiIcon,
        Squares2x2Icon, Square3Stack3dIcon, FolderIcon, DocumentIcon, ClipboardDocumentListIcon,
        UserIcon, UsersIcon, UserGroupIcon, UserCircleIcon
    } from 'heroicons-svelte/24/outline';

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

    const fontAwesomeSolid = [
        { icon: faChartLine, name: 'Chart Line' }, { icon: faChartArea, name: 'Chart Area' },
        { icon: faChartBar, name: 'Chart Bar' }, { icon: faChartPie, name: 'Chart Pie' },
        { icon: faChartColumn, name: 'Chart Column' }, { icon: faChartSimple, name: 'Chart Simple' },
        { icon: faChartGantt, name: 'Chart Gantt' }, { icon: faArrowTrendUp, name: 'Trend Up' },
        { icon: faArrowTrendDown, name: 'Trend Down' }, { icon: faArrowUp, name: 'Arrow Up' },
        { icon: faArrowDown, name: 'Arrow Down' }, { icon: faArrowRight, name: 'Arrow Right' },
        { icon: faArrowLeft, name: 'Arrow Left' }, { icon: faAnglesUp, name: 'Angles Up' },
        { icon: faAnglesDown, name: 'Angles Down' }, { icon: faCircleUp, name: 'Circle Up' },
        { icon: faCircleDown, name: 'Circle Down' }, { icon: faSquareArrowUpRight, name: 'Square Arrow' },
        { icon: faArrowsUpDown, name: 'Arrows UpDown' }, { icon: faArrowsLeftRight, name: 'Arrows LeftRight' },
        { icon: faArrowRotateRight, name: 'Rotate Right' }, { icon: faRotate, name: 'Rotate' },
        { icon: faSync, name: 'Sync' }, { icon: faMoneyBillTrendUp, name: 'Money Trend Up' },
        { icon: faMoneyBill, name: 'Money Bill' }, { icon: faMoneyBill1, name: 'Money Bill 1' },
        { icon: faMoneyBills, name: 'Money Bills' }, { icon: faWallet, name: 'Wallet' },
        { icon: faCoins, name: 'Coins' }, { icon: faSackDollar, name: 'Sack Dollar' },
        { icon: faHandHoldingDollar, name: 'Hand Dollar' }, { icon: faDollarSign, name: 'Dollar Sign' },
        { icon: faMoneyCheck, name: 'Money Check' }, { icon: faCreditCard, name: 'Credit Card' },
        { icon: faReceipt, name: 'Receipt' }, { icon: faCashRegister, name: 'Cash Register' },
        { icon: faPercent, name: 'Percent' }, { icon: faCalculator, name: 'Calculator' },
        { icon: faBuildingColumns, name: 'Building Columns' }, { icon: faLandmark, name: 'Landmark' },
        { icon: faBank, name: 'Bank' }, { icon: faUniversity, name: 'University' },
        { icon: faBuilding, name: 'Building' }, { icon: faTrophy, name: 'Trophy' },
        { icon: faMedal, name: 'Medal' }, { icon: faAward, name: 'Award' },
        { icon: faStar, name: 'Star' }, { icon: faCrown, name: 'Crown' },
        { icon: faGem, name: 'Gem' }, { icon: faGaugeHigh, name: 'Gauge High' },
        { icon: faGauge, name: 'Gauge' }, { icon: faStopwatch, name: 'Stopwatch' },
        { icon: faClock, name: 'Clock' }, { icon: faHourglass, name: 'Hourglass' },
        { icon: faBullseye, name: 'Bullseye' }, { icon: faCrosshairs, name: 'Crosshairs' },
        { icon: faCompass, name: 'Compass' }, { icon: faLocationDot, name: 'Location Dot' },
        { icon: faMapLocationDot, name: 'Map Location' }, { icon: faShieldHalved, name: 'Shield Halved' },
        { icon: faShield, name: 'Shield' }, { icon: faLock, name: 'Lock' },
        { icon: faKey, name: 'Key' }, { icon: faScaleBalanced, name: 'Scale Balanced' },
        { icon: faEye, name: 'Eye' }, { icon: faMagnifyingGlass, name: 'Magnifying Glass' },
        { icon: faMagnifyingGlassChart, name: 'Mag Glass Chart' }, { icon: faMagnifyingGlassDollar, name: 'Mag Glass Dollar' },
        { icon: faBrain, name: 'Brain' }, { icon: faLightbulb, name: 'Lightbulb' },
        { icon: faFlask, name: 'Flask' }, { icon: faBell, name: 'Bell' },
        { icon: faBullhorn, name: 'Bullhorn' }, { icon: faTriangleExclamation, name: 'Triangle Exclamation' },
        { icon: faCircleExclamation, name: 'Circle Exclamation' }, { icon: faCircleInfo, name: 'Circle Info' },
        { icon: faCircleCheck, name: 'Circle Check' }, { icon: faGear, name: 'Gear' },
        { icon: faSliders, name: 'Sliders' }, { icon: faWrench, name: 'Wrench' },
        { icon: faToolbox, name: 'Toolbox' }, { icon: faDatabase, name: 'Database' },
        { icon: faServer, name: 'Server' }, { icon: faMicrochip, name: 'Microchip' },
        { icon: faMemory, name: 'Memory' }, { icon: faNetworkWired, name: 'Network Wired' },
        { icon: faSignal, name: 'Signal' }, { icon: faLayerGroup, name: 'Layer Group' },
        { icon: faCubes, name: 'Cubes' }, { icon: faUser, name: 'User' }, { icon: faUsers, name: 'Users' }
    ];

    const heroicons = [
        { icon: ChartBarIcon, name: 'Chart Bar' }, { icon: ChartPieIcon, name: 'Chart Pie' },
        { icon: PresentationChartLineIcon, name: 'Presentation Chart Line' }, { icon: PresentationChartBarIcon, name: 'Presentation Chart Bar' },
        { icon: DocumentChartBarIcon, name: 'Document Chart Bar' }, { icon: ArrowTrendingUpIcon, name: 'Trending Up' },
        { icon: ArrowTrendingDownIcon, name: 'Trending Down' }, { icon: ArrowUpIcon, name: 'Arrow Up' },
        { icon: ArrowDownIcon, name: 'Arrow Down' }, { icon: ArrowRightIcon, name: 'Arrow Right' },
        { icon: ArrowLeftIcon, name: 'Arrow Left' }, { icon: ArrowsUpDownIcon, name: 'Arrows UpDown' },
        { icon: ArrowsRightLeftIcon, name: 'Arrows RightLeft' }, { icon: ArrowPathIcon, name: 'Arrow Path' },
        { icon: ArrowPathRoundedSquareIcon, name: 'Arrow Path Rounded' }, { icon: BanknotesIcon, name: 'Banknotes' },
        { icon: CurrencyDollarIcon, name: 'Currency Dollar' }, { icon: CreditCardIcon, name: 'Credit Card' },
        { icon: ReceiptPercentIcon, name: 'Receipt Percent' }, { icon: ReceiptRefundIcon, name: 'Receipt Refund' },
        { icon: BuildingOffice2Icon, name: 'Building Office' }, { icon: BuildingLibraryIcon, name: 'Building Library' },
        { icon: BuildingStorefrontIcon, name: 'Building Storefront' }, { icon: HomeIcon, name: 'Home' },
        { icon: HomeModernIcon, name: 'Home Modern' }, { icon: TrophyIcon, name: 'Trophy' },
        { icon: StarIcon, name: 'Star' }, { icon: FireIcon, name: 'Fire' },
        { icon: BoltIcon, name: 'Bolt' }, { icon: SparklesIcon, name: 'Sparkles' },
        { icon: RocketLaunchIcon, name: 'Rocket Launch' }, { icon: ClockIcon, name: 'Clock' },
        { icon: CalendarIcon, name: 'Calendar' }, { icon: CalendarDaysIcon, name: 'Calendar Days' },
        { icon: MapIcon, name: 'Map' }, { icon: MapPinIcon, name: 'Map Pin' },
        { icon: GlobeAltIcon, name: 'Globe' }, { icon: GlobeAmericasIcon, name: 'Globe Americas' },
        { icon: ShieldCheckIcon, name: 'Shield Check' }, { icon: ShieldExclamationIcon, name: 'Shield Exclamation' },
        { icon: LockClosedIcon, name: 'Lock Closed' }, { icon: LockOpenIcon, name: 'Lock Open' },
        { icon: KeyIcon, name: 'Key' }, { icon: ScaleIcon, name: 'Scale' },
        { icon: EyeIcon, name: 'Eye' }, { icon: EyeSlashIcon, name: 'Eye Slash' },
        { icon: MagnifyingGlassIcon, name: 'Magnifying Glass' }, { icon: MagnifyingGlassCircleIcon, name: 'Mag Glass Circle' },
        { icon: MagnifyingGlassPlusIcon, name: 'Mag Glass Plus' }, { icon: MagnifyingGlassMinusIcon, name: 'Mag Glass Minus' },
        { icon: LightBulbIcon, name: 'Light Bulb' }, { icon: BeakerIcon, name: 'Beaker' },
        { icon: AcademicCapIcon, name: 'Academic Cap' }, { icon: BellIcon, name: 'Bell' },
        { icon: BellAlertIcon, name: 'Bell Alert' }, { icon: BellSlashIcon, name: 'Bell Slash' },
        { icon: ExclamationCircleIcon, name: 'Exclamation Circle' }, { icon: ExclamationTriangleIcon, name: 'Exclamation Triangle' },
        { icon: InformationCircleIcon, name: 'Information Circle' }, { icon: CheckCircleIcon, name: 'Check Circle' },
        { icon: Cog6ToothIcon, name: 'Cog 6' }, { icon: Cog8ToothIcon, name: 'Cog 8' },
        { icon: AdjustmentsHorizontalIcon, name: 'Adjustments Horizontal' }, { icon: AdjustmentsVerticalIcon, name: 'Adjustments Vertical' },
        { icon: CalculatorIcon, name: 'Calculator' }, { icon: WrenchScrewdriverIcon, name: 'Wrench Screwdriver' },
        { icon: CpuChipIcon, name: 'CPU Chip' }, { icon: ServerIcon, name: 'Server' },
        { icon: CircleStackIcon, name: 'Circle Stack' }, { icon: CommandLineIcon, name: 'Command Line' },
        { icon: WifiIcon, name: 'WiFi' },
        { icon: Square3Stack3dIcon, name: 'Square Stack 3D' }, { icon: Squares2x2Icon, name: 'Squares 2x2' },
        { icon: FolderIcon, name: 'Folder' }, { icon: DocumentIcon, name: 'Document' },
        { icon: ClipboardDocumentListIcon, name: 'Clipboard List' }, { icon: UserIcon, name: 'User' },
        { icon: UsersIcon, name: 'Users' }, { icon: UserGroupIcon, name: 'User Group' },
        { icon: UserCircleIcon, name: 'User Circle' }
    ];

    // ApexCharts types
    const apexCharts = [
        { type: 'line', name: 'Apex Line' }, { type: 'area', name: 'Apex Area' },
        { type: 'bar', name: 'Apex Bar' }, { type: 'pie', name: 'Apex Pie' },
        { type: 'donut', name: 'Apex Donut' }, { type: 'radialBar', name: 'Apex Radial' },
        { type: 'scatter', name: 'Apex Scatter' }, { type: 'bubble', name: 'Apex Bubble' },
        { type: 'heatmap', name: 'Apex Heatmap' }, { type: 'treemap', name: 'Apex Treemap' },
        { type: 'candlestick', name: 'Apex Candlestick' }, { type: 'boxPlot', name: 'Apex BoxPlot' },
        { type: 'radar', name: 'Apex Radar' }, { type: 'polarArea', name: 'Apex Polar' },
        { type: 'rangeBar', name: 'Apex Range Bar' }
    ];

    // Chart.js types
    const chartJsCharts = [
        { type: 'line', name: 'ChartJS Line' }, { type: 'bar', name: 'ChartJS Bar' },
        { type: 'pie', name: 'ChartJS Pie' }, { type: 'doughnut', name: 'ChartJS Doughnut' },
        { type: 'polarArea', name: 'ChartJS Polar' }, { type: 'radar', name: 'ChartJS Radar' },
        { type: 'scatter', name: 'ChartJS Scatter' }, { type: 'bubble', name: 'ChartJS Bubble' }
    ];

    // Lightweight Charts types
    const lightweightCharts = [
        { type: 'line', name: 'LW Line' }, { type: 'area', name: 'LW Area' },
        { type: 'candlestick', name: 'LW Candlestick' }, { type: 'bar', name: 'LW Bar' },
        { type: 'histogram', name: 'LW Histogram' }
    ];

    const totalIcons = d3Charts.length + apexCharts.length + chartJsCharts.length + lightweightCharts.length + fontAwesomeSolid.length + heroicons.length;
</script>

<div class="min-h-screen bg-slate-950 text-white p-4 md:p-8">
    <div class="max-w-7xl mx-auto">
        <div class="mb-12 text-center">
            <h1 class="text-4xl md:text-6xl font-bold mb-4 bg-gradient-to-r from-blue-400 via-purple-400 to-pink-400 bg-clip-text text-transparent">
                TRADING ICON LIBRARY
            </h1>
            <p class="text-slate-400 text-lg md:text-xl mb-2">
                {totalIcons} TRADING & DASHBOARD ICONS
            </p>
            <p class="text-slate-500 text-sm">
                D3: {d3Charts.length} • Apex: {apexCharts.length} • ChartJS: {chartJsCharts.length} • LW: {lightweightCharts.length} • FA: {fontAwesomeSolid.length} • Hero: {heroicons.length}
            </p>
        </div>

        <!-- D3 Charts -->
        <div class="mb-16">
            <div class="flex items-center gap-4 mb-8">
                <h2 class="text-3xl font-bold bg-gradient-to-r from-yellow-400 to-orange-500 bg-clip-text text-transparent">D3.js Charts</h2>
                <span class="px-3 py-1 bg-yellow-500/20 border border-yellow-500/30 rounded-full text-sm font-bold text-yellow-300">{d3Charts.length}</span>
            </div>
            <div class="grid grid-cols-4 sm:grid-cols-6 md:grid-cols-8 lg:grid-cols-10 gap-3">
                {#each d3Charts as chart}
                    <div class="group bg-slate-900/50 border border-white/5 rounded-xl p-3 hover:border-yellow-500/30 transition-all hover:-translate-y-1">
                        <div class="flex flex-col items-center gap-2">
                            <ChartIcons type={chart.type as any} size={40} color={chart.color} />
                            <span class="text-[9px] text-center text-slate-500 group-hover:text-slate-300 leading-tight">{chart.name}</span>
                        </div>
                    </div>
                {/each}
            </div>
        </div>

        <!-- ApexCharts -->
        <div class="mb-16">
            <div class="flex items-center gap-4 mb-8">
                <h2 class="text-3xl font-bold bg-gradient-to-r from-pink-400 to-rose-500 bg-clip-text text-transparent">ApexCharts</h2>
                <span class="px-3 py-1 bg-pink-500/20 border border-pink-500/30 rounded-full text-sm font-bold text-pink-300">{apexCharts.length}</span>
            </div>
            <div class="grid grid-cols-4 sm:grid-cols-6 md:grid-cols-8 lg:grid-cols-10 gap-3">
                {#each apexCharts as chart}
                    <div class="group bg-slate-900/50 border border-white/5 rounded-xl p-3 hover:border-pink-500/30 transition-all hover:-translate-y-1">
                        <div class="flex flex-col items-center gap-2">
                            <ApexChartIcons type={chart.type} size={40} />
                            <span class="text-[9px] text-center text-slate-500 group-hover:text-slate-300 leading-tight">{chart.name}</span>
                        </div>
                    </div>
                {/each}
            </div>
        </div>

        <!-- Chart.js -->
        <div class="mb-16">
            <div class="flex items-center gap-4 mb-8">
                <h2 class="text-3xl font-bold bg-gradient-to-r from-indigo-400 to-violet-500 bg-clip-text text-transparent">Chart.js</h2>
                <span class="px-3 py-1 bg-indigo-500/20 border border-indigo-500/30 rounded-full text-sm font-bold text-indigo-300">{chartJsCharts.length}</span>
            </div>
            <div class="grid grid-cols-4 sm:grid-cols-6 md:grid-cols-8 lg:grid-cols-10 gap-3">
                {#each chartJsCharts as chart}
                    <div class="group bg-slate-900/50 border border-white/5 rounded-xl p-3 hover:border-indigo-500/30 transition-all hover:-translate-y-1">
                        <div class="flex flex-col items-center gap-2">
                            <ChartJsIcons type={chart.type} size={40} />
                            <span class="text-[9px] text-center text-slate-500 group-hover:text-slate-300 leading-tight">{chart.name}</span>
                        </div>
                    </div>
                {/each}
            </div>
        </div>

        <!-- Lightweight Charts -->
        <div class="mb-16">
            <div class="flex items-center gap-4 mb-8">
                <h2 class="text-3xl font-bold bg-gradient-to-r from-teal-400 to-cyan-500 bg-clip-text text-transparent">Lightweight Charts</h2>
                <span class="px-3 py-1 bg-teal-500/20 border border-teal-500/30 rounded-full text-sm font-bold text-teal-300">{lightweightCharts.length}</span>
            </div>
            <div class="grid grid-cols-4 sm:grid-cols-6 md:grid-cols-8 lg:grid-cols-10 gap-3">
                {#each lightweightCharts as chart}
                    <div class="group bg-slate-900/50 border border-white/5 rounded-xl p-3 hover:border-teal-500/30 transition-all hover:-translate-y-1">
                        <div class="flex flex-col items-center gap-2">
                            <LightweightChartIcons type={chart.type} size={40} />
                            <span class="text-[9px] text-center text-slate-500 group-hover:text-slate-300 leading-tight">{chart.name}</span>
                        </div>
                    </div>
                {/each}
            </div>
        </div>

        <!-- FontAwesome Solid -->
        <div class="mb-16">
            <div class="flex items-center gap-4 mb-8">
                <h2 class="text-3xl font-bold text-blue-400">FontAwesome Trading</h2>
                <span class="px-3 py-1 bg-blue-500/20 border border-blue-500/30 rounded-full text-sm font-bold text-blue-300">{fontAwesomeSolid.length}</span>
            </div>
            <div class="grid grid-cols-4 sm:grid-cols-6 md:grid-cols-8 lg:grid-cols-10 gap-3">
                {#each fontAwesomeSolid as item}
                    <div class="group bg-slate-900/50 border border-white/5 rounded-xl p-3 hover:border-blue-500/30 transition-all hover:-translate-y-1">
                        <div class="flex flex-col items-center gap-2">
                            <div class="text-slate-400 group-hover:text-blue-400 transition-colors">
                                <FontAwesomeIcon icon={item.icon} class="w-7 h-7" />
                            </div>
                            <span class="text-[9px] text-center text-slate-500 group-hover:text-slate-300 leading-tight">{item.name}</span>
                        </div>
                    </div>
                {/each}
            </div>
        </div>

        <!-- Heroicons -->
        <div class="mb-16">
            <div class="flex items-center gap-4 mb-8">
                <h2 class="text-3xl font-bold text-purple-400">Heroicons Trading</h2>
                <span class="px-3 py-1 bg-purple-500/20 border border-purple-500/30 rounded-full text-sm font-bold text-purple-300">{heroicons.length}</span>
            </div>
            <div class="grid grid-cols-4 sm:grid-cols-6 md:grid-cols-8 lg:grid-cols-10 gap-3">
                {#each heroicons as item}
                    {@const HeroIcon = item.icon}
                    <div class="group bg-slate-900/50 border border-white/5 rounded-xl p-3 hover:border-purple-500/30 transition-all hover:-translate-y-1">
                        <div class="flex flex-col items-center gap-2">
                            <div class="text-slate-400 group-hover:text-purple-400 transition-colors">
                                <HeroIcon class="w-7 h-7" />
                            </div>
                            <span class="text-[9px] text-center text-slate-500 group-hover:text-slate-300 leading-tight">{item.name}</span>
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
            <p class="text-slate-600 text-xs">
                D3.js v7.9 • FontAwesome v7.1 • Heroicons v2.0
            </p>
        </div>
    </div>
</div>

<script lang="ts">
    import ChartIcons from '$lib/components/ChartIcons.svelte';
    import ApexChartIcons from '$lib/components/ApexChartIcons.svelte';
    import ChartJsIcons from '$lib/components/ChartJsIcons.svelte';
    import LightweightChartIcons from '$lib/components/LightweightChartIcons.svelte';
    import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
    
    // FontAwesome Solid - 50 TRADING ICONS ONLY
    import {
        // Charts & Analytics
        faChartLine, faChartArea, faChartBar, faChartPie, faChartColumn, faChartSimple, faChartGantt,
        // Trending & Market Movement
        faArrowTrendUp, faArrowTrendDown, faArrowUp, faArrowDown, faArrowRight, faArrowLeft,
        faAnglesUp, faAnglesDown, faCircleUp, faCircleDown, faSquareArrowUpRight,
        faArrowsUpDown, faArrowsLeftRight, faArrowRotateRight, faRotate, faSync,
        // Money & Finance
        faMoneyBillTrendUp, faMoneyBill, faMoneyBill1, faMoneyBills, faWallet,
        faCoins, faSackDollar, faHandHoldingDollar, faDollarSign, faMoneyCheck,
        faCreditCard, faReceipt, faCashRegister, faPercent, faCalculator,
        // Institutions & Markets
        faBuildingColumns, faLandmark, faBank, faUniversity, faBuilding,
        // Performance & Success
        faTrophy, faMedal, faAward, faStar, faCrown, faGem,
        // Speed & Metrics
        faGaugeHigh, faGauge, faStopwatch, faClock, faHourglass,
        // Targeting & Analysis
        faBullseye, faCrosshairs, faCompass, faLocationDot, faMapLocationDot,
        // Risk & Security
        faShieldHalved, faShield, faLock, faKey, faScaleBalanced,
        // Monitoring & Research
        faEye, faMagnifyingGlass, faMagnifyingGlassChart, faMagnifyingGlassDollar,
        // Intelligence & Strategy
        faBrain, faLightbulb, faFlask,
        // Alerts & Notifications
        faBell, faBullhorn, faTriangleExclamation, faCircleExclamation, faCircleInfo, faCircleCheck,
        // Tools & Settings
        faGear, faSliders, faWrench, faToolbox,
        // Technical Infrastructure
        faDatabase, faServer, faMicrochip, faMemory, faNetworkWired, faSignal,
        // Organization & Structure
        faLayerGroup, faCubes, faUser, faUsers
    } from '@fortawesome/free-solid-svg-icons';
    
    // Heroicons - 50 TRADING ICONS
    import {
        // Charts & Analytics
        ChartBarIcon, ChartPieIcon, PresentationChartLineIcon, PresentationChartBarIcon, DocumentChartBarIcon,
        // Market Movement
        ArrowTrendingUpIcon, ArrowTrendingDownIcon, ArrowUpIcon, ArrowDownIcon, ArrowRightIcon, ArrowLeftIcon,
        ArrowsUpDownIcon, ArrowsRightLeftIcon, ArrowPathIcon, ArrowPathRoundedSquareIcon,
        // Money & Finance
        BanknotesIcon, CurrencyDollarIcon, CreditCardIcon, ReceiptPercentIcon, ReceiptRefundIcon,
        // Buildings & Markets
        BuildingOffice2Icon, BuildingLibraryIcon, BuildingStorefrontIcon, HomeIcon, HomeModernIcon,
        // Performance
        TrophyIcon, StarIcon, FireIcon, BoltIcon, SparklesIcon, RocketLaunchIcon,
        // Time & Scheduling
        ClockIcon, CalendarIcon, CalendarDaysIcon,
        // Location & Markets
        MapIcon, MapPinIcon, GlobeAltIcon, GlobeAmericasIcon,
        // Security & Risk
        ShieldCheckIcon, ShieldExclamationIcon, LockClosedIcon, LockOpenIcon, KeyIcon, ScaleIcon,
        // Monitoring & Analysis
        EyeIcon, EyeSlashIcon, MagnifyingGlassIcon, MagnifyingGlassCircleIcon, MagnifyingGlassPlusIcon, MagnifyingGlassMinusIcon,
        // Intelligence
        LightBulbIcon, BeakerIcon, AcademicCapIcon,
        // Alerts
        BellIcon, BellAlertIcon, BellSlashIcon, ExclamationCircleIcon, ExclamationTriangleIcon, InformationCircleIcon, CheckCircleIcon,
        // Settings & Tools
        Cog6ToothIcon, Cog8ToothIcon, AdjustmentsHorizontalIcon, AdjustmentsVerticalIcon, CalculatorIcon, WrenchScrewdriverIcon,
        // Technical
        CpuChipIcon, ServerIcon, CircleStackIcon, CommandLineIcon, SignalIcon, WifiIcon,
        // Organization
        Square3Stack3DOutline, Squares2X2Outline, FolderIcon, DocumentIcon, ClipboardDocumentListIcon,
        // Users & Teams
        UserIcon, UsersIcon, UserGroupIcon, UserCircleIcon
    } from 'heroicons-svelte/24/outline';
    
    // Tabler Icons - 50 TRADING ICONS
    import {
        // Charts
        IconChartLine, IconChartBar, IconChartCandle, IconChartDots, IconChartArcs, IconChartInfographic, IconChartHistogram,
        // Market Movement
        IconTrendingUp, IconTrendingDown, IconTrendingUp2, IconTrendingDown2, IconTrendingUp3, IconTrendingDown3,
        IconArrowUp, IconArrowDown, IconArrowRight, IconArrowLeft, IconArrowUpRight, IconArrowDownRight,
        IconArrowsUpDown, IconArrowsLeftRight, IconArrowsSort,
        // Money & Finance
        IconCurrencyDollar, IconCoin, IconCoins, IconCash, IconCashBanknote, IconPigMoney, IconMoneybag,
        IconCreditCard, IconReceipt, IconReceiptTax, IconDiscount, IconPercentage,
        // Buildings & Markets
        IconBuildingBank, IconBuilding, IconBuildingSkyscraper, IconHome, IconHomeDollar,
        // Performance
        IconTrophy, IconMedal, IconAward, IconCertificate, IconBadge, IconStar, IconStarFilled, IconFlame, IconBolt, IconRocket,
        // Speed & Metrics
        IconGauge, IconSpeedboat, IconClock, IconHourglass, IconCalendar,
        // Targeting & Analysis
        IconTarget, IconTargetArrow, IconBullseye, IconFocus, IconFocusCentered, IconCompass, IconLocation, IconMapPin, IconMap,
        // Security & Risk
        IconShield, IconShieldCheck, IconShieldLock, IconLock, IconLockOpen, IconKey, IconScale,
        // Monitoring
        IconEye, IconEyeCheck, IconRadar, IconRadar2, IconScan, IconSearch, IconZoomIn, IconZoomOut, IconZoomCheck, IconZoomMoney,
        // Intelligence
        IconBrain, IconBulb, IconFlask,
        // Alerts
        IconBell, IconBellRinging, IconAlertTriangle, IconAlertCircle, IconInfoCircle, IconCircleCheck, IconCircleX,
        // Settings & Tools
        IconSettings, IconAdjustments, IconSliders, IconTool, IconCalculator,
        // Technical
        IconDatabase, IconServer, IconCpu, IconChip, IconNetwork, IconWifi, IconSignal, IconAntenna,
        // Organization
        IconLayersLinked, IconStack, IconBox, IconFolder, IconFile, IconClipboard, IconList, IconListCheck,
        // Activity & Performance
        IconActivity, IconActivityHeartbeat, IconDashboard, IconGraph, IconTimeline,
        // Users & Teams
        IconUser, IconUsers, IconUsersGroup
    } from '@tabler/icons-svelte';
    
    // D3 Charts - 50 variations
    const d3Charts = [
        // Line Charts (10)
        { type: 'line', name: 'Line Chart', color: '#3b82f6' },
        { type: 'line', name: 'Smooth Line', color: '#ef4444' },
        { type: 'line', name: 'Multi-Line', color: '#06b6d4' },
        { type: 'line', name: 'Step Chart', color: '#8b5cf6' },
        { type: 'line', name: 'Curved Line', color: '#10b981' },
        { type: 'line', name: 'Dashed Line', color: '#f59e0b' },
        { type: 'line', name: 'Dotted Line', color: '#ec4899' },
        { type: 'line', name: 'Thick Line', color: '#a855f7' },
        { type: 'line', name: 'Thin Line', color: '#14b8a6' },
        { type: 'line', name: 'Gradient Line', color: '#f43f5e' },
        // Area Charts (10)
        { type: 'area', name: 'Area Chart', color: '#8b5cf6' },
        { type: 'area', name: 'Gradient Area', color: '#f59e0b' },
        { type: 'area', name: 'Stream Graph', color: '#06b6d4' },
        { type: 'area', name: 'Stacked Area', color: '#10b981' },
        { type: 'area', name: 'Filled Area', color: '#ec4899' },
        { type: 'area', name: 'Smooth Area', color: '#3b82f6' },
        { type: 'area', name: 'Multi Area', color: '#ef4444' },
        { type: 'area', name: 'Range Area', color: '#a855f7' },
        { type: 'area', name: 'Band Area', color: '#14b8a6' },
        { type: 'area', name: 'Horizon Area', color: '#eab308' },
        // Candlestick Charts (5)
        { type: 'candle', name: 'Candlestick', color: '#10b981' },
        { type: 'candle', name: 'OHLC Chart', color: '#22c55e' },
        { type: 'candle', name: 'Hollow Candle', color: '#16a34a' },
        { type: 'candle', name: 'Heikin Ashi', color: '#15803d' },
        { type: 'candle', name: 'Renko Chart', color: '#166534' },
        // Bar Charts (10)
        { type: 'bar', name: 'Bar Chart', color: '#f59e0b' },
        { type: 'bar', name: 'Grouped Bar', color: '#84cc16' },
        { type: 'bar', name: 'Stacked Bar', color: '#eab308' },
        { type: 'bar', name: 'Horizontal Bar', color: '#fbbf24' },
        { type: 'bar', name: 'Waterfall Bar', color: '#fcd34d' },
        { type: 'bar', name: 'Rounded Bar', color: '#fde047' },
        { type: 'bar', name: 'Gradient Bar', color: '#facc15' },
        { type: 'bar', name: 'Clustered Bar', color: '#fef08a' },
        { type: 'bar', name: 'Range Bar', color: '#fef9c3' },
        { type: 'bar', name: 'Diverging Bar', color: '#fef3c7' },
        // Pie/Donut Charts (5)
        { type: 'pie', name: 'Pie Chart', color: '#ec4899' },
        { type: 'pie', name: 'Donut Chart', color: '#f472b6' },
        { type: 'pie', name: 'Semi Donut', color: '#f9a8d4' },
        { type: 'pie', name: 'Exploded Pie', color: '#fbcfe8' },
        { type: 'pie', name: 'Nested Pie', color: '#fce7f3' },
        // Scatter/Bubble (5)
        { type: 'scatter', name: 'Scatter Plot', color: '#06b6d4' },
        { type: 'scatter', name: 'Bubble Chart', color: '#a855f7' },
        { type: 'scatter', name: 'Sized Scatter', color: '#22d3ee' },
        { type: 'scatter', name: 'Colored Scatter', color: '#67e8f9' },
        { type: 'scatter', name: 'Connected Scatter', color: '#a5f3fc' },
        // Heatmap (3)
        { type: 'heatmap', name: 'Heatmap', color: '#8b5cf6' },
        { type: 'heatmap', name: 'Correlation Matrix', color: '#f43f5e' },
        { type: 'heatmap', name: 'Calendar Heatmap', color: '#a78bfa' },
        // Radar (2)
        { type: 'radar', name: 'Radar Chart', color: '#10b981' },
        { type: 'radar', name: 'Spider Chart', color: '#14b8a6' }
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
        { icon: SignalIcon, name: 'Signal' }, { icon: WifiIcon, name: 'WiFi' },
        { icon: Square3Stack3DOutline, name: 'Square Stack 3D' }, { icon: Squares2X2Outline, name: 'Squares 2x2' },
        { icon: FolderIcon, name: 'Folder' }, { icon: DocumentIcon, name: 'Document' },
        { icon: ClipboardDocumentListIcon, name: 'Clipboard List' }, { icon: UserIcon, name: 'User' },
        { icon: UsersIcon, name: 'Users' }, { icon: UserGroupIcon, name: 'User Group' },
        { icon: UserCircleIcon, name: 'User Circle' }
    ];
    
    const tabler = [
        { icon: IconChartLine, name: 'Chart Line' }, { icon: IconChartBar, name: 'Chart Bar' },
        { icon: IconChartCandle, name: 'Chart Candle' }, { icon: IconChartDots, name: 'Chart Dots' },
        { icon: IconChartArcs, name: 'Chart Arcs' }, { icon: IconChartInfographic, name: 'Chart Infographic' },
        { icon: IconChartHistogram, name: 'Chart Histogram' }, { icon: IconTrendingUp, name: 'Trending Up' },
        { icon: IconTrendingDown, name: 'Trending Down' }, { icon: IconTrendingUp2, name: 'Trending Up 2' },
        { icon: IconTrendingDown2, name: 'Trending Down 2' }, { icon: IconTrendingUp3, name: 'Trending Up 3' },
        { icon: IconTrendingDown3, name: 'Trending Down 3' }, { icon: IconArrowUp, name: 'Arrow Up' },
        { icon: IconArrowDown, name: 'Arrow Down' }, { icon: IconArrowRight, name: 'Arrow Right' },
        { icon: IconArrowLeft, name: 'Arrow Left' }, { icon: IconArrowUpRight, name: 'Arrow Up Right' },
        { icon: IconArrowDownRight, name: 'Arrow Down Right' }, { icon: IconArrowsUpDown, name: 'Arrows UpDown' },
        { icon: IconArrowsLeftRight, name: 'Arrows LeftRight' }, { icon: IconArrowsSort, name: 'Arrows Sort' },
        { icon: IconCurrencyDollar, name: 'Currency Dollar' }, { icon: IconCoin, name: 'Coin' },
        { icon: IconCoins, name: 'Coins' }, { icon: IconCash, name: 'Cash' },
        { icon: IconCashBanknote, name: 'Cash Banknote' }, { icon: IconPigMoney, name: 'Pig Money' },
        { icon: IconMoneybag, name: 'Moneybag' }, { icon: IconCreditCard, name: 'Credit Card' },
        { icon: IconReceipt, name: 'Receipt' }, { icon: IconReceiptTax, name: 'Receipt Tax' },
        { icon: IconDiscount, name: 'Discount' }, { icon: IconPercentage, name: 'Percentage' },
        { icon: IconBuildingBank, name: 'Building Bank' }, { icon: IconBuilding, name: 'Building' },
        { icon: IconBuildingSkyscraper, name: 'Building Skyscraper' }, { icon: IconHome, name: 'Home' },
        { icon: IconHomeDollar, name: 'Home Dollar' }, { icon: IconTrophy, name: 'Trophy' },
        { icon: IconMedal, name: 'Medal' }, { icon: IconAward, name: 'Award' },
        { icon: IconCertificate, name: 'Certificate' }, { icon: IconBadge, name: 'Badge' },
        { icon: IconStar, name: 'Star' }, { icon: IconStarFilled, name: 'Star Filled' },
        { icon: IconFlame, name: 'Flame' }, { icon: IconBolt, name: 'Bolt' },
        { icon: IconRocket, name: 'Rocket' }, { icon: IconGauge, name: 'Gauge' },
        { icon: IconSpeedboat, name: 'Speedboat' }, { icon: IconClock, name: 'Clock' },
        { icon: IconHourglass, name: 'Hourglass' }, { icon: IconCalendar, name: 'Calendar' },
        { icon: IconTarget, name: 'Target' }, { icon: IconTargetArrow, name: 'Target Arrow' },
        { icon: IconBullseye, name: 'Bullseye' }, { icon: IconFocus, name: 'Focus' },
        { icon: IconFocusCentered, name: 'Focus Centered' }, { icon: IconCompass, name: 'Compass' },
        { icon: IconLocation, name: 'Location' }, { icon: IconMapPin, name: 'Map Pin' },
        { icon: IconMap, name: 'Map' }, { icon: IconShield, name: 'Shield' },
        { icon: IconShieldCheck, name: 'Shield Check' }, { icon: IconShieldLock, name: 'Shield Lock' },
        { icon: IconLock, name: 'Lock' }, { icon: IconLockOpen, name: 'Lock Open' },
        { icon: IconKey, name: 'Key' }, { icon: IconScale, name: 'Scale' },
        { icon: IconEye, name: 'Eye' }, { icon: IconEyeCheck, name: 'Eye Check' },
        { icon: IconRadar, name: 'Radar' }, { icon: IconRadar2, name: 'Radar 2' },
        { icon: IconScan, name: 'Scan' }, { icon: IconSearch, name: 'Search' },
        { icon: IconZoomIn, name: 'Zoom In' }, { icon: IconZoomOut, name: 'Zoom Out' },
        { icon: IconZoomCheck, name: 'Zoom Check' }, { icon: IconZoomMoney, name: 'Zoom Money' },
        { icon: IconBrain, name: 'Brain' }, { icon: IconBulb, name: 'Bulb' },
        { icon: IconFlask, name: 'Flask' }, { icon: IconBell, name: 'Bell' },
        { icon: IconBellRinging, name: 'Bell Ringing' }, { icon: IconAlertTriangle, name: 'Alert Triangle' },
        { icon: IconAlertCircle, name: 'Alert Circle' }, { icon: IconInfoCircle, name: 'Info Circle' },
        { icon: IconCircleCheck, name: 'Circle Check' }, { icon: IconCircleX, name: 'Circle X' },
        { icon: IconSettings, name: 'Settings' }, { icon: IconAdjustments, name: 'Adjustments' },
        { icon: IconSliders, name: 'Sliders' }, { icon: IconTool, name: 'Tool' },
        { icon: IconCalculator, name: 'Calculator' }, { icon: IconDatabase, name: 'Database' },
        { icon: IconServer, name: 'Server' }, { icon: IconCpu, name: 'CPU' },
        { icon: IconChip, name: 'Chip' }, { icon: IconNetwork, name: 'Network' },
        { icon: IconWifi, name: 'WiFi' }, { icon: IconSignal, name: 'Signal' },
        { icon: IconAntenna, name: 'Antenna' }, { icon: IconLayersLinked, name: 'Layers Linked' },
        { icon: IconStack, name: 'Stack' }, { icon: IconBox, name: 'Box' },
        { icon: IconFolder, name: 'Folder' }, { icon: IconFile, name: 'File' },
        { icon: IconClipboard, name: 'Clipboard' }, { icon: IconList, name: 'List' },
        { icon: IconListCheck, name: 'List Check' }, { icon: IconActivity, name: 'Activity' },
        { icon: IconActivityHeartbeat, name: 'Activity Heartbeat' }, { icon: IconDashboard, name: 'Dashboard' },
        { icon: IconGraph, name: 'Graph' }, { icon: IconTimeline, name: 'Timeline' },
        { icon: IconUser, name: 'User' }, { icon: IconUsers, name: 'Users' },
        { icon: IconUsersGroup, name: 'Users Group' }
    ];
    
    // ApexCharts - 50 chart types
    const apexCharts = [
        { type: 'line', name: 'Apex Line' }, { type: 'area', name: 'Apex Area' },
        { type: 'bar', name: 'Apex Bar' }, { type: 'pie', name: 'Apex Pie' },
        { type: 'donut', name: 'Apex Donut' }, { type: 'radialBar', name: 'Apex Radial' },
        { type: 'scatter', name: 'Apex Scatter' }, { type: 'bubble', name: 'Apex Bubble' },
        { type: 'heatmap', name: 'Apex Heatmap' }, { type: 'treemap', name: 'Apex Treemap' },
        { type: 'candlestick', name: 'Apex Candlestick' }, { type: 'boxPlot', name: 'Apex BoxPlot' },
        { type: 'radar', name: 'Apex Radar' }, { type: 'polarArea', name: 'Apex Polar' },
        { type: 'rangeBar', name: 'Apex Range Bar' }, { type: 'line', name: 'Apex Sparkline' },
        { type: 'area', name: 'Apex Gradient' }, { type: 'bar', name: 'Apex Stacked' },
        { type: 'line', name: 'Apex Multi' }, { type: 'area', name: 'Apex Stream' },
        { type: 'bar', name: 'Apex Grouped' }, { type: 'line', name: 'Apex Smooth' },
        { type: 'area', name: 'Apex Filled' }, { type: 'bar', name: 'Apex Horizontal' },
        { type: 'line', name: 'Apex Step' }, { type: 'area', name: 'Apex Range' },
        { type: 'bar', name: 'Apex Waterfall' }, { type: 'line', name: 'Apex Dashed' },
        { type: 'area', name: 'Apex Band' }, { type: 'bar', name: 'Apex Rounded' },
        { type: 'line', name: 'Apex Curved' }, { type: 'area', name: 'Apex Stacked' },
        { type: 'bar', name: 'Apex Clustered' }, { type: 'line', name: 'Apex Dotted' },
        { type: 'area', name: 'Apex Multi Area' }, { type: 'bar', name: 'Apex Diverging' },
        { type: 'line', name: 'Apex Thick' }, { type: 'area', name: 'Apex Horizon' },
        { type: 'bar', name: 'Apex Range Bar' }, { type: 'line', name: 'Apex Thin' },
        { type: 'area', name: 'Apex Smooth Area' }, { type: 'bar', name: 'Apex Gradient Bar' },
        { type: 'line', name: 'Apex Multi Line' }, { type: 'area', name: 'Apex Filled Area' },
        { type: 'bar', name: 'Apex Stacked Bar' }, { type: 'line', name: 'Apex Step Line' },
        { type: 'area', name: 'Apex Range Area' }, { type: 'bar', name: 'Apex Waterfall Bar' },
        { type: 'line', name: 'Apex Dashed Line' }, { type: 'area', name: 'Apex Band Area' }
    ];
    
    // Chart.js - 50 chart types
    const chartJsCharts = [
        { type: 'line', name: 'ChartJS Line' }, { type: 'bar', name: 'ChartJS Bar' },
        { type: 'pie', name: 'ChartJS Pie' }, { type: 'doughnut', name: 'ChartJS Doughnut' },
        { type: 'polarArea', name: 'ChartJS Polar' }, { type: 'radar', name: 'ChartJS Radar' },
        { type: 'scatter', name: 'ChartJS Scatter' }, { type: 'bubble', name: 'ChartJS Bubble' },
        { type: 'line', name: 'ChartJS Smooth' }, { type: 'bar', name: 'ChartJS Stacked' },
        { type: 'line', name: 'ChartJS Multi' }, { type: 'bar', name: 'ChartJS Grouped' },
        { type: 'line', name: 'ChartJS Step' }, { type: 'bar', name: 'ChartJS Horizontal' },
        { type: 'line', name: 'ChartJS Curved' }, { type: 'bar', name: 'ChartJS Rounded' },
        { type: 'line', name: 'ChartJS Dashed' }, { type: 'bar', name: 'ChartJS Gradient' },
        { type: 'line', name: 'ChartJS Dotted' }, { type: 'bar', name: 'ChartJS Clustered' },
        { type: 'line', name: 'ChartJS Thick' }, { type: 'bar', name: 'ChartJS Range' },
        { type: 'line', name: 'ChartJS Thin' }, { type: 'bar', name: 'ChartJS Diverging' },
        { type: 'line', name: 'ChartJS Gradient' }, { type: 'bar', name: 'ChartJS Waterfall' },
        { type: 'line', name: 'ChartJS Multi Line' }, { type: 'bar', name: 'ChartJS Stacked Bar' },
        { type: 'line', name: 'ChartJS Step Line' }, { type: 'bar', name: 'ChartJS Horizontal Bar' },
        { type: 'line', name: 'ChartJS Curved Line' }, { type: 'bar', name: 'ChartJS Rounded Bar' },
        { type: 'line', name: 'ChartJS Dashed Line' }, { type: 'bar', name: 'ChartJS Gradient Bar' },
        { type: 'line', name: 'ChartJS Dotted Line' }, { type: 'bar', name: 'ChartJS Clustered Bar' },
        { type: 'line', name: 'ChartJS Thick Line' }, { type: 'bar', name: 'ChartJS Range Bar' },
        { type: 'line', name: 'ChartJS Thin Line' }, { type: 'bar', name: 'ChartJS Diverging Bar' },
        { type: 'line', name: 'ChartJS Gradient Line' }, { type: 'bar', name: 'ChartJS Waterfall Bar' },
        { type: 'line', name: 'ChartJS Sparkline' }, { type: 'bar', name: 'ChartJS Mini Bar' },
        { type: 'line', name: 'ChartJS Area Line' }, { type: 'bar', name: 'ChartJS Column' },
        { type: 'line', name: 'ChartJS Smooth Line' }, { type: 'bar', name: 'ChartJS Grouped Bar' }
    ];
    
    // Lightweight Charts - 50 chart types
    const lightweightCharts = [
        { type: 'line', name: 'LW Line' }, { type: 'area', name: 'LW Area' },
        { type: 'candlestick', name: 'LW Candlestick' }, { type: 'bar', name: 'LW Bar' },
        { type: 'histogram', name: 'LW Histogram' }, { type: 'line', name: 'LW Smooth' },
        { type: 'area', name: 'LW Gradient' }, { type: 'candlestick', name: 'LW OHLC' },
        { type: 'bar', name: 'LW Volume' }, { type: 'histogram', name: 'LW Volume Hist' },
        { type: 'line', name: 'LW Multi Line' }, { type: 'area', name: 'LW Stacked' },
        { type: 'candlestick', name: 'LW Hollow' }, { type: 'bar', name: 'LW Colored' },
        { type: 'histogram', name: 'LW Colored Hist' }, { type: 'line', name: 'LW Step' },
        { type: 'area', name: 'LW Filled' }, { type: 'candlestick', name: 'LW Heikin' },
        { type: 'bar', name: 'LW Range' }, { type: 'histogram', name: 'LW Range Hist' },
        { type: 'line', name: 'LW Curved' }, { type: 'area', name: 'LW Smooth Area' },
        { type: 'candlestick', name: 'LW Renko' }, { type: 'bar', name: 'LW Grouped' },
        { type: 'histogram', name: 'LW Grouped Hist' }, { type: 'line', name: 'LW Dashed' },
        { type: 'area', name: 'LW Multi Area' }, { type: 'candlestick', name: 'LW Kagi' },
        { type: 'bar', name: 'LW Stacked' }, { type: 'histogram', name: 'LW Stacked Hist' },
        { type: 'line', name: 'LW Dotted' }, { type: 'area', name: 'LW Range Area' },
        { type: 'candlestick', name: 'LW Line Break' }, { type: 'bar', name: 'LW Waterfall' },
        { type: 'histogram', name: 'LW Waterfall Hist' }, { type: 'line', name: 'LW Thick' },
        { type: 'area', name: 'LW Band' }, { type: 'candlestick', name: 'LW Point Figure' },
        { type: 'bar', name: 'LW Diverging' }, { type: 'histogram', name: 'LW Diverging Hist' },
        { type: 'line', name: 'LW Thin' }, { type: 'area', name: 'LW Horizon' },
        { type: 'candlestick', name: 'LW Three Line' }, { type: 'bar', name: 'LW Rounded' },
        { type: 'histogram', name: 'LW Rounded Hist' }, { type: 'line', name: 'LW Gradient' },
        { type: 'area', name: 'LW Stream' }, { type: 'candlestick', name: 'LW Baseline' }
    ];
    
    const totalIcons = d3Charts.length + apexCharts.length + chartJsCharts.length + lightweightCharts.length + fontAwesomeSolid.length + heroicons.length + tabler.length;
</script>

<div class="min-h-screen bg-slate-950 text-white p-4 md:p-8">
    <div class="max-w-7xl mx-auto">
        <div class="mb-12 text-center">
            <h1 class="text-4xl md:text-6xl font-bold mb-4 bg-gradient-to-r from-blue-400 via-purple-400 to-pink-400 bg-clip-text text-transparent">
                ULTIMATE TRADING ICON LIBRARY
            </h1>
            <p class="text-slate-400 text-lg md:text-xl mb-2">
                {totalIcons} TRADING & DASHBOARD ICONS
            </p>
            <p class="text-slate-500 text-sm">
                D3: {d3Charts.length} • Apex: {apexCharts.length} • ChartJS: {chartJsCharts.length} • LW: {lightweightCharts.length} • FA: {fontAwesomeSolid.length} • Hero: {heroicons.length} • Tabler: {tabler.length}
            </p>
            <p class="text-emerald-400 text-xs mt-2 font-bold">
                ✓ ALL TRADING RELATED • NO SOCIAL MEDIA BS
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
                    <div class="group bg-slate-900/50 border border-white/5 rounded-xl p-3 hover:border-purple-500/30 transition-all hover:-translate-y-1">
                        <div class="flex flex-col items-center gap-2">
                            <div class="text-slate-400 group-hover:text-purple-400 transition-colors">
                                <svelte:component this={item.icon} class="w-7 h-7" />
                            </div>
                            <span class="text-[9px] text-center text-slate-500 group-hover:text-slate-300 leading-tight">{item.name}</span>
                        </div>
                    </div>
                {/each}
            </div>
        </div>

        <!-- Tabler -->
        <div class="mb-16">
            <div class="flex items-center gap-4 mb-8">
                <h2 class="text-3xl font-bold text-emerald-400">Tabler Trading</h2>
                <span class="px-3 py-1 bg-emerald-500/20 border border-emerald-500/30 rounded-full text-sm font-bold text-emerald-300">{tabler.length}</span>
            </div>
            <div class="grid grid-cols-4 sm:grid-cols-6 md:grid-cols-8 lg:grid-cols-10 gap-3">
                {#each tabler as item}
                    <div class="group bg-slate-900/50 border border-white/5 rounded-xl p-3 hover:border-emerald-500/30 transition-all hover:-translate-y-1">
                        <div class="flex flex-col items-center gap-2">
                            <div class="text-slate-400 group-hover:text-emerald-400 transition-colors">
                                <svelte:component this={item.icon} size={28} stroke={1.5} />
                            </div>
                            <span class="text-[9px] text-center text-slate-500 group-hover:text-slate-300 leading-tight">{item.name}</span>
                        </div>
                    </div>
                {/each}
            </div>
        </div>

        <div class="text-center py-8 border-t border-slate-800">
            <p class="text-emerald-400 text-lg font-bold mb-2">
                ✓ {totalIcons} TRADING & DASHBOARD ICONS
            </p>
            <p class="text-slate-500 text-sm mb-2">
                Charts • Trends • Money • Markets • Performance • Analytics • Risk • Monitoring
            </p>
            <p class="text-slate-600 text-xs">
                D3.js v7.9 • FontAwesome v7.1 • Heroicons v2.0 • Tabler v3.35
            </p>
        </div>
    </div>
</div>

/**
 * Components - Main Barrel Export
 * Revolution Trading Pros Component Library
 * 
 * @version 2.0.0
 * @author Revolution Trading Pros
 */

// ═══════════════════════════════════════════════════════════════════════════
// UI Primitives
// ═══════════════════════════════════════════════════════════════════════════
export {
	Button,
	Card,
	Input,
	Badge,
	Table,
	Modal,
	Select,
	Toast,
	AnimatedNumber,
	SkeletonLoader,
	EnterpriseStatCard,
	EnterpriseStatsGrid,
	NotificationPanel,
	ExportButton,
	MobileResponsiveTable,
	CommandPalette,
	NotificationCenter,
	KeyboardShortcutsHelp,
	RateLimitIndicator,
	ConnectionHealthPanel,
	OfflineIndicator,
	BatchOperations,
} from './ui';

// ═══════════════════════════════════════════════════════════════════════════
// Pattern Components
// ═══════════════════════════════════════════════════════════════════════════
export {
	DataTable,
	StatCard,
	EmptyState,
	ErrorBoundary,
	PageHeader,
} from './patterns';

// ═══════════════════════════════════════════════════════════════════════════
// Layout Components
// ═══════════════════════════════════════════════════════════════════════════
export {
	MarketingFooter,
	AppSidebar,
	AdminSidebar,
	TradingRoomShell,
} from './layout';

// ═══════════════════════════════════════════════════════════════════════════
// Chart Components
// ═══════════════════════════════════════════════════════════════════════════
export {
	EnterpriseChart,
	Chart,
	ChartTheme,
	ChartColors,
	ChartFonts,
	getLightweightChartsOptions,
	getAreaSeriesOptions,
	getLineSeriesOptions,
	getCandlestickSeriesOptions,
	getHistogramSeriesOptions,
	getApexChartsTheme,
	getChartJsTheme,
} from './charts';

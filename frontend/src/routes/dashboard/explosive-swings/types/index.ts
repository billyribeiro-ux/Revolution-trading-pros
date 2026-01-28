/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Explosive Swings - Type Exports
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @description Central re-export point for all Explosive Swings types.
 * @version 2.0.0 - Nuclear Refactor Consolidation
 *
 * USAGE:
 *   import type { Alert, Trade, AlertType } from './types';
 *   import { alertColors, positionStatusColors } from './types';
 *
 * This module re-exports everything from the main types.ts file for cleaner imports.
 * ═══════════════════════════════════════════════════════════════════════════════
 */

// Re-export all types from the main types file
export * from '../types';

// ═══════════════════════════════════════════════════════════════════════════
// Type Categories Reference
// ═══════════════════════════════════════════════════════════════════════════
//
// CLOSED TRADES:
//   - ClosedTrade
//
// ACTIVE POSITIONS:
//   - PositionStatus
//   - PositionTarget
//   - ActivePosition
//
// ALERTS:
//   - AlertType
//   - Alert
//   - AlertFilterType
//   - AlertFilter
//
// PERFORMANCE METRICS:
//   - WeeklyPerformance
//   - ThirtyDayPerformance
//
// VIDEO CONTENT:
//   - WeeklyVideo
//   - Video
//   - VideoUpdate
//
// DASHBOARD DATA:
//   - ExplosiveSwingsDashboardData
//   - WeeklyContent
//   - QuickStats
//
// TRADE PLAN:
//   - TradeBias
//   - TradePlanEntry
//
// TRADE ENUMS:
//   - TradeStatus
//   - TradeType
//   - TradeAction
//   - OptionType
//   - OrderType
//
// API RESPONSE TYPES:
//   - ApiResponse<T>
//   - PaginationMeta
//   - ApiTrade
//   - ApiWeeklyVideo
//   - PaginationState
//
// SEARCH TYPES:
//   - SearchFilters
//   - SearchResult
//   - SearchResponse
//
// ANALYTICS TYPES:
//   - AnalyticsSummary
//   - TickerPerformance
//   - MonthlyReturn
//   - EquityPoint
//   - DrawdownPeriod
//   - RoomAnalytics
//
// WEBSOCKET TYPES:
//   - WsEventType
//   - WsMessage<T>
//   - AlertCreatedEvent
//   - TradeClosedEvent
//
// EXPORT TYPES:
//   - ExportOptions
//
// FORM TYPES:
//   - AlertFormData
//   - TradeFormData
//   - CloseTradeFormData
//
// UTILITY TYPES:
//   - LoadingState
//   - ApiError
//   - AsyncState<T>
//
// COMPONENT PROPS:
//   - PerformanceSummaryProps
//   - ActivePositionCardProps
//   - PerformanceCardProps
//   - TickerPillProps
//
// COLOR CONSTANTS:
//   - alertColors
//   - positionStatusColors
//   - performanceColors
//
// ═══════════════════════════════════════════════════════════════════════════

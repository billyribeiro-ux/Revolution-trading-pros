//! Room Analytics Service - Explosive Swings Performance Analytics
//! ═══════════════════════════════════════════════════════════════════════════════════
//! Apple Principal Engineer ICT 11+ Grade - January 2026
//!
//! Comprehensive analytics service for trading room performance:
//! - Summary statistics (win rate, profit factor, drawdown)
//! - Ticker performance breakdown
//! - Setup performance analysis
//! - Monthly returns tracking
//! - Equity curve generation
//! - Drawdown period analysis
//!
//! Features efficient SQL queries with window functions for calculations.
//! ═══════════════════════════════════════════════════════════════════════════════════

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

// ═══════════════════════════════════════════════════════════════════════════════════
// ANALYTICS TYPES
// ═══════════════════════════════════════════════════════════════════════════════════

/// Complete room analytics response
#[derive(Debug, Serialize)]
pub struct RoomAnalytics {
    pub summary: AnalyticsSummary,
    pub ticker_performance: Vec<TickerPerformance>,
    pub setup_performance: Vec<SetupPerformance>,
    pub monthly_returns: Vec<MonthlyReturn>,
    pub equity_curve: Vec<EquityPoint>,
    pub drawdown_periods: Vec<DrawdownPeriod>,
}

/// Summary statistics for a trading room
#[derive(Debug, Serialize, Default)]
pub struct AnalyticsSummary {
    pub total_trades: i64,
    pub wins: i64,
    pub losses: i64,
    pub win_rate: f64,
    pub profit_factor: f64,
    pub avg_win: f64,
    pub avg_loss: f64,
    pub largest_win: f64,
    pub largest_loss: f64,
    pub max_drawdown: f64,
    pub avg_holding_days: f64,
    pub current_streak: i32,
    pub streak_type: String, // "win" or "loss"
}

/// Performance metrics by ticker symbol
#[derive(Debug, Serialize, FromRow)]
pub struct TickerPerformance {
    pub ticker: String,
    pub total_trades: i64,
    pub wins: i64,
    pub losses: i64,
    pub win_rate: f64,
    pub total_pnl: f64,
    pub avg_pnl: f64,
}

/// Performance metrics by setup type
#[derive(Debug, Serialize, FromRow)]
pub struct SetupPerformance {
    pub setup: String,
    pub total_trades: i64,
    pub wins: i64,
    pub losses: i64,
    pub win_rate: f64,
    pub total_pnl: f64,
    pub avg_pnl: f64,
}

/// Monthly return data
#[derive(Debug, Serialize)]
pub struct MonthlyReturn {
    pub month: String, // "2026-01" format
    pub pnl: f64,
    pub trades: i64,
    pub win_rate: f64,
}

/// Point on the equity curve
#[derive(Debug, Serialize, FromRow)]
pub struct EquityPoint {
    pub date: NaiveDate,
    pub cumulative_pnl: f64,
    pub trade_id: i64,
}

/// Drawdown period data
#[derive(Debug, Serialize)]
pub struct DrawdownPeriod {
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub drawdown_amount: f64,
    pub drawdown_percent: f64,
    pub recovery_days: Option<i32>,
}

/// Query parameters for analytics date filtering
#[derive(Debug, Deserialize)]
pub struct AnalyticsDateRange {
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// INTERNAL QUERY RESULT TYPES
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, FromRow)]
struct SummaryRow {
    total_trades: Option<i64>,
    wins: Option<i64>,
    losses: Option<i64>,
    total_wins_pnl: Option<f64>,
    total_losses_pnl: Option<f64>,
    avg_win: Option<f64>,
    avg_loss: Option<f64>,
    largest_win: Option<f64>,
    largest_loss: Option<f64>,
    avg_holding_days: Option<f64>,
}

#[derive(Debug, FromRow)]
struct StreakRow {
    current_streak: Option<i32>,
    streak_type: Option<String>,
}

#[derive(Debug, FromRow)]
struct MonthlyRow {
    month: Option<String>,
    pnl: Option<f64>,
    trades: Option<i64>,
    wins: Option<i64>,
}

#[derive(Debug, FromRow)]
struct EquityCurveRow {
    exit_date: Option<NaiveDate>,
    cumulative_pnl: Option<f64>,
    id: i64,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// SERVICE IMPLEMENTATION
// ═══════════════════════════════════════════════════════════════════════════════════

/// Room Analytics Service
///
/// Provides comprehensive performance analytics for trading rooms
/// with efficient SQL-based calculations using window functions.
#[derive(Clone)]
pub struct RoomAnalyticsService {
    pool: PgPool,
}

impl RoomAnalyticsService {
    /// Create a new RoomAnalyticsService instance
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get complete room analytics with all metrics
    ///
    /// # Arguments
    /// * `room_slug` - The room identifier
    /// * `from` - Optional start date filter
    /// * `to` - Optional end date filter
    pub async fn get_room_analytics(
        &self,
        room_slug: &str,
        from: Option<NaiveDate>,
        to: Option<NaiveDate>,
    ) -> Result<RoomAnalytics, sqlx::Error> {
        // Execute all queries in parallel for performance
        let (summary, ticker_perf, setup_perf, monthly, equity, drawdowns) = tokio::join!(
            self.get_summary(room_slug, from, to),
            self.get_ticker_performance(room_slug, from, to),
            self.get_setup_performance(room_slug, from, to),
            self.get_monthly_returns(room_slug, from, to),
            self.get_equity_curve(room_slug, from, to),
            self.get_drawdown_periods(room_slug, from, to)
        );

        Ok(RoomAnalytics {
            summary: summary?,
            ticker_performance: ticker_perf?,
            setup_performance: setup_perf?,
            monthly_returns: monthly?,
            equity_curve: equity?,
            drawdown_periods: drawdowns?,
        })
    }

    /// Get summary analytics for a room
    async fn get_summary(
        &self,
        room_slug: &str,
        from: Option<NaiveDate>,
        to: Option<NaiveDate>,
    ) -> Result<AnalyticsSummary, sqlx::Error> {
        // Build date filter clause
        let date_filter = Self::build_date_filter("exit_date", from, to);

        let query = format!(
            r#"
            SELECT
                COUNT(*) as total_trades,
                COUNT(*) FILTER (WHERE result = 'WIN') as wins,
                COUNT(*) FILTER (WHERE result = 'LOSS') as losses,
                COALESCE(SUM(pnl) FILTER (WHERE result = 'WIN'), 0) as total_wins_pnl,
                COALESCE(ABS(SUM(pnl) FILTER (WHERE result = 'LOSS')), 0) as total_losses_pnl,
                COALESCE(AVG(pnl) FILTER (WHERE result = 'WIN'), 0) as avg_win,
                COALESCE(ABS(AVG(pnl) FILTER (WHERE result = 'LOSS')), 0) as avg_loss,
                COALESCE(MAX(pnl), 0) as largest_win,
                COALESCE(MIN(pnl), 0) as largest_loss,
                COALESCE(AVG(holding_days), 0) as avg_holding_days
            FROM room_trades
            WHERE room_slug = $1
            AND status = 'closed'
            AND deleted_at IS NULL
            {}
            "#,
            date_filter
        );

        let mut query_builder = sqlx::query_as::<_, SummaryRow>(&query).bind(room_slug);

        if let Some(from_date) = from {
            query_builder = query_builder.bind(from_date);
        }
        if let Some(to_date) = to {
            query_builder = query_builder.bind(to_date);
        }

        let row = query_builder.fetch_one(&self.pool).await?;

        // Calculate derived metrics
        let total_trades = row.total_trades.unwrap_or(0);
        let wins = row.wins.unwrap_or(0);
        let losses = row.losses.unwrap_or(0);
        let total_wins_pnl = row.total_wins_pnl.unwrap_or(0.0);
        let total_losses_pnl = row.total_losses_pnl.unwrap_or(0.0);

        let win_rate = if total_trades > 0 {
            (wins as f64 / total_trades as f64) * 100.0
        } else {
            0.0
        };

        let profit_factor = if total_losses_pnl > 0.0 {
            total_wins_pnl / total_losses_pnl
        } else if total_wins_pnl > 0.0 {
            f64::INFINITY
        } else {
            0.0
        };

        // Get current streak
        let streak = self.get_current_streak(room_slug).await?;

        // Get max drawdown from equity curve
        let max_drawdown = self.calculate_max_drawdown(room_slug, from, to).await?;

        Ok(AnalyticsSummary {
            total_trades,
            wins,
            losses,
            win_rate: (win_rate * 100.0).round() / 100.0,
            profit_factor: (profit_factor * 100.0).round() / 100.0,
            avg_win: row.avg_win.unwrap_or(0.0),
            avg_loss: row.avg_loss.unwrap_or(0.0),
            largest_win: row.largest_win.unwrap_or(0.0),
            largest_loss: row.largest_loss.unwrap_or(0.0),
            max_drawdown,
            avg_holding_days: row.avg_holding_days.unwrap_or(0.0),
            current_streak: streak.current_streak.unwrap_or(0),
            streak_type: streak.streak_type.unwrap_or_else(|| "none".to_string()),
        })
    }

    /// Get current win/loss streak
    async fn get_current_streak(&self, room_slug: &str) -> Result<StreakRow, sqlx::Error> {
        let row: Option<StreakRow> = sqlx::query_as(
            r#"
            WITH recent_trades AS (
                SELECT result, exit_date,
                       ROW_NUMBER() OVER (ORDER BY exit_date DESC) as rn
                FROM room_trades
                WHERE room_slug = $1
                AND status = 'closed'
                AND deleted_at IS NULL
                AND result IS NOT NULL
                ORDER BY exit_date DESC
            ),
            streak_calc AS (
                SELECT result,
                       COUNT(*) OVER (
                           ORDER BY rn
                           ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW
                       ) as streak_count
                FROM recent_trades
                WHERE rn = 1 OR result = (SELECT result FROM recent_trades WHERE rn = 1)
            )
            SELECT
                (SELECT COUNT(*)::INT FROM streak_calc) as current_streak,
                (SELECT result FROM recent_trades WHERE rn = 1) as streak_type
            "#,
        )
        .bind(room_slug)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.unwrap_or(StreakRow {
            current_streak: Some(0),
            streak_type: Some("none".to_string()),
        }))
    }

    /// Calculate maximum drawdown from equity curve
    async fn calculate_max_drawdown(
        &self,
        room_slug: &str,
        from: Option<NaiveDate>,
        to: Option<NaiveDate>,
    ) -> Result<f64, sqlx::Error> {
        let date_filter = Self::build_date_filter("exit_date", from, to);

        let query = format!(
            r#"
            WITH equity AS (
                SELECT
                    exit_date,
                    SUM(pnl) OVER (ORDER BY exit_date) as cumulative_pnl
                FROM room_trades
                WHERE room_slug = $1
                AND status = 'closed'
                AND deleted_at IS NULL
                {}
                ORDER BY exit_date
            ),
            peaks AS (
                SELECT
                    exit_date,
                    cumulative_pnl,
                    MAX(cumulative_pnl) OVER (ORDER BY exit_date) as running_peak
                FROM equity
            ),
            drawdowns AS (
                SELECT
                    cumulative_pnl - running_peak as drawdown
                FROM peaks
            )
            SELECT COALESCE(MIN(drawdown), 0) as max_drawdown
            FROM drawdowns
            "#,
            date_filter
        );

        let mut query_builder = sqlx::query_scalar::<_, f64>(&query).bind(room_slug);

        if let Some(from_date) = from {
            query_builder = query_builder.bind(from_date);
        }
        if let Some(to_date) = to {
            query_builder = query_builder.bind(to_date);
        }

        let max_drawdown = query_builder.fetch_one(&self.pool).await?;
        Ok(max_drawdown.abs())
    }

    /// Get performance metrics grouped by ticker
    pub async fn get_ticker_performance(
        &self,
        room_slug: &str,
        from: Option<NaiveDate>,
        to: Option<NaiveDate>,
    ) -> Result<Vec<TickerPerformance>, sqlx::Error> {
        let date_filter = Self::build_date_filter("exit_date", from, to);

        let query = format!(
            r#"
            SELECT
                ticker,
                COUNT(*)::BIGINT as total_trades,
                COUNT(*) FILTER (WHERE result = 'WIN')::BIGINT as wins,
                COUNT(*) FILTER (WHERE result = 'LOSS')::BIGINT as losses,
                COALESCE(
                    ROUND(100.0 * COUNT(*) FILTER (WHERE result = 'WIN') / NULLIF(COUNT(*), 0), 2),
                    0
                ) as win_rate,
                COALESCE(SUM(pnl), 0) as total_pnl,
                COALESCE(AVG(pnl), 0) as avg_pnl
            FROM room_trades
            WHERE room_slug = $1
            AND status = 'closed'
            AND deleted_at IS NULL
            {}
            GROUP BY ticker
            ORDER BY total_pnl DESC
            "#,
            date_filter
        );

        let mut query_builder = sqlx::query_as::<_, TickerPerformance>(&query).bind(room_slug);

        if let Some(from_date) = from {
            query_builder = query_builder.bind(from_date);
        }
        if let Some(to_date) = to {
            query_builder = query_builder.bind(to_date);
        }

        query_builder.fetch_all(&self.pool).await
    }

    /// Get performance metrics grouped by setup type
    pub async fn get_setup_performance(
        &self,
        room_slug: &str,
        from: Option<NaiveDate>,
        to: Option<NaiveDate>,
    ) -> Result<Vec<SetupPerformance>, sqlx::Error> {
        let date_filter = Self::build_date_filter("exit_date", from, to);

        let query = format!(
            r#"
            SELECT
                COALESCE(setup, 'Unknown') as setup,
                COUNT(*)::BIGINT as total_trades,
                COUNT(*) FILTER (WHERE result = 'WIN')::BIGINT as wins,
                COUNT(*) FILTER (WHERE result = 'LOSS')::BIGINT as losses,
                COALESCE(
                    ROUND(100.0 * COUNT(*) FILTER (WHERE result = 'WIN') / NULLIF(COUNT(*), 0), 2),
                    0
                ) as win_rate,
                COALESCE(SUM(pnl), 0) as total_pnl,
                COALESCE(AVG(pnl), 0) as avg_pnl
            FROM room_trades
            WHERE room_slug = $1
            AND status = 'closed'
            AND deleted_at IS NULL
            {}
            GROUP BY COALESCE(setup, 'Unknown')
            ORDER BY total_pnl DESC
            "#,
            date_filter
        );

        let mut query_builder = sqlx::query_as::<_, SetupPerformance>(&query).bind(room_slug);

        if let Some(from_date) = from {
            query_builder = query_builder.bind(from_date);
        }
        if let Some(to_date) = to {
            query_builder = query_builder.bind(to_date);
        }

        query_builder.fetch_all(&self.pool).await
    }

    /// Get monthly returns
    pub async fn get_monthly_returns(
        &self,
        room_slug: &str,
        from: Option<NaiveDate>,
        to: Option<NaiveDate>,
    ) -> Result<Vec<MonthlyReturn>, sqlx::Error> {
        let date_filter = Self::build_date_filter("exit_date", from, to);

        let query = format!(
            r#"
            SELECT
                TO_CHAR(exit_date, 'YYYY-MM') as month,
                COALESCE(SUM(pnl), 0) as pnl,
                COUNT(*)::BIGINT as trades,
                COUNT(*) FILTER (WHERE result = 'WIN')::BIGINT as wins
            FROM room_trades
            WHERE room_slug = $1
            AND status = 'closed'
            AND deleted_at IS NULL
            {}
            GROUP BY TO_CHAR(exit_date, 'YYYY-MM')
            ORDER BY month DESC
            "#,
            date_filter
        );

        let mut query_builder = sqlx::query_as::<_, MonthlyRow>(&query).bind(room_slug);

        if let Some(from_date) = from {
            query_builder = query_builder.bind(from_date);
        }
        if let Some(to_date) = to {
            query_builder = query_builder.bind(to_date);
        }

        let rows = query_builder.fetch_all(&self.pool).await?;

        Ok(rows
            .into_iter()
            .filter_map(|r| {
                let month = r.month?;
                let trades = r.trades.unwrap_or(0);
                let wins = r.wins.unwrap_or(0);
                let win_rate = if trades > 0 {
                    (wins as f64 / trades as f64) * 100.0
                } else {
                    0.0
                };

                Some(MonthlyReturn {
                    month,
                    pnl: r.pnl.unwrap_or(0.0),
                    trades,
                    win_rate: (win_rate * 100.0).round() / 100.0,
                })
            })
            .collect())
    }

    /// Get equity curve data points
    pub async fn get_equity_curve(
        &self,
        room_slug: &str,
        from: Option<NaiveDate>,
        to: Option<NaiveDate>,
    ) -> Result<Vec<EquityPoint>, sqlx::Error> {
        let date_filter = Self::build_date_filter("exit_date", from, to);

        let query = format!(
            r#"
            SELECT
                exit_date,
                SUM(pnl) OVER (ORDER BY exit_date, id) as cumulative_pnl,
                id
            FROM room_trades
            WHERE room_slug = $1
            AND status = 'closed'
            AND deleted_at IS NULL
            {}
            ORDER BY exit_date, id
            "#,
            date_filter
        );

        let mut query_builder = sqlx::query_as::<_, EquityCurveRow>(&query).bind(room_slug);

        if let Some(from_date) = from {
            query_builder = query_builder.bind(from_date);
        }
        if let Some(to_date) = to {
            query_builder = query_builder.bind(to_date);
        }

        let rows = query_builder.fetch_all(&self.pool).await?;

        Ok(rows
            .into_iter()
            .filter_map(|r| {
                Some(EquityPoint {
                    date: r.exit_date?,
                    cumulative_pnl: r.cumulative_pnl.unwrap_or(0.0),
                    trade_id: r.id,
                })
            })
            .collect())
    }

    /// Get drawdown periods analysis
    pub async fn get_drawdown_periods(
        &self,
        room_slug: &str,
        from: Option<NaiveDate>,
        to: Option<NaiveDate>,
    ) -> Result<Vec<DrawdownPeriod>, sqlx::Error> {
        let date_filter = Self::build_date_filter("exit_date", from, to);

        let query = format!(
            r#"
            WITH equity AS (
                SELECT
                    exit_date,
                    pnl,
                    SUM(pnl) OVER (ORDER BY exit_date, id) as cumulative_pnl
                FROM room_trades
                WHERE room_slug = $1
                AND status = 'closed'
                AND deleted_at IS NULL
                {}
                ORDER BY exit_date, id
            ),
            peaks AS (
                SELECT
                    exit_date,
                    cumulative_pnl,
                    MAX(cumulative_pnl) OVER (ORDER BY exit_date) as running_peak,
                    cumulative_pnl - MAX(cumulative_pnl) OVER (ORDER BY exit_date) as drawdown
                FROM equity
            ),
            drawdown_groups AS (
                SELECT
                    exit_date,
                    cumulative_pnl,
                    running_peak,
                    drawdown,
                    CASE WHEN drawdown < 0 THEN 1 ELSE 0 END as in_drawdown,
                    SUM(CASE WHEN drawdown >= 0 THEN 1 ELSE 0 END) OVER (ORDER BY exit_date) as group_id
                FROM peaks
            ),
            drawdown_summary AS (
                SELECT
                    group_id,
                    MIN(exit_date) as start_date,
                    MAX(exit_date) as end_date,
                    MIN(drawdown) as max_drawdown,
                    MAX(running_peak) as peak_value
                FROM drawdown_groups
                WHERE in_drawdown = 1
                GROUP BY group_id
                HAVING MIN(drawdown) < -100  -- Only significant drawdowns (> $100)
            )
            SELECT
                start_date,
                end_date,
                max_drawdown as drawdown_amount,
                CASE WHEN peak_value > 0
                     THEN ROUND((max_drawdown / peak_value) * 100, 2)
                     ELSE 0
                END as drawdown_percent,
                (end_date - start_date)::INT as recovery_days
            FROM drawdown_summary
            ORDER BY start_date DESC
            LIMIT 10
            "#,
            date_filter
        );

        #[derive(Debug, FromRow)]
        struct DrawdownRow {
            start_date: NaiveDate,
            end_date: NaiveDate,
            drawdown_amount: f64,
            drawdown_percent: f64,
            recovery_days: i32,
        }

        let mut query_builder = sqlx::query_as::<_, DrawdownRow>(&query).bind(room_slug);

        if let Some(from_date) = from {
            query_builder = query_builder.bind(from_date);
        }
        if let Some(to_date) = to {
            query_builder = query_builder.bind(to_date);
        }

        let rows = query_builder.fetch_all(&self.pool).await?;

        Ok(rows
            .into_iter()
            .map(|r| DrawdownPeriod {
                start_date: r.start_date,
                end_date: Some(r.end_date),
                drawdown_amount: r.drawdown_amount.abs(),
                drawdown_percent: r.drawdown_percent.abs(),
                recovery_days: Some(r.recovery_days),
            })
            .collect())
    }

    /// Get analytics for a specific ticker
    pub async fn get_ticker_analytics(
        &self,
        room_slug: &str,
        ticker: &str,
    ) -> Result<Option<TickerPerformance>, sqlx::Error> {
        let result: Option<TickerPerformance> = sqlx::query_as(
            r#"
            SELECT
                ticker,
                COUNT(*)::BIGINT as total_trades,
                COUNT(*) FILTER (WHERE result = 'WIN')::BIGINT as wins,
                COUNT(*) FILTER (WHERE result = 'LOSS')::BIGINT as losses,
                COALESCE(
                    ROUND(100.0 * COUNT(*) FILTER (WHERE result = 'WIN') / NULLIF(COUNT(*), 0), 2),
                    0
                ) as win_rate,
                COALESCE(SUM(pnl), 0) as total_pnl,
                COALESCE(AVG(pnl), 0) as avg_pnl
            FROM room_trades
            WHERE room_slug = $1
            AND UPPER(ticker) = UPPER($2)
            AND status = 'closed'
            AND deleted_at IS NULL
            GROUP BY ticker
            "#,
        )
        .bind(room_slug)
        .bind(ticker)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    /// Helper to build date filter SQL clause
    fn build_date_filter(
        date_column: &str,
        from: Option<NaiveDate>,
        to: Option<NaiveDate>,
    ) -> String {
        let mut filters = Vec::new();
        let mut param_num = 2; // $1 is room_slug

        if from.is_some() {
            param_num += 1;
            filters.push(format!("AND {} >= ${}", date_column, param_num));
        }

        if to.is_some() {
            param_num += 1;
            filters.push(format!("AND {} <= ${}", date_column, param_num));
        }

        filters.join(" ")
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_date_filter_no_dates() {
        let filter = RoomAnalyticsService::build_date_filter("exit_date", None, None);
        assert_eq!(filter, "");
    }

    #[test]
    fn test_build_date_filter_from_only() {
        let from = NaiveDate::from_ymd_opt(2026, 1, 1);
        let filter = RoomAnalyticsService::build_date_filter("exit_date", from, None);
        assert_eq!(filter, "AND exit_date >= $3");
    }

    #[test]
    fn test_build_date_filter_both_dates() {
        let from = NaiveDate::from_ymd_opt(2026, 1, 1);
        let to = NaiveDate::from_ymd_opt(2026, 12, 31);
        let filter = RoomAnalyticsService::build_date_filter("exit_date", from, to);
        assert_eq!(filter, "AND exit_date >= $3 AND exit_date <= $4");
    }

    #[test]
    fn test_analytics_summary_default() {
        let summary = AnalyticsSummary::default();
        assert_eq!(summary.total_trades, 0);
        assert_eq!(summary.win_rate, 0.0);
        assert_eq!(summary.streak_type, "");
    }
}

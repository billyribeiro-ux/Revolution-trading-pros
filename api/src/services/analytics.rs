//! Analytics Service - Revolution Trading Pros
//! ═══════════════════════════════════════════════════════════════════════════════════
//! Apple Principal Engineer ICT 7+ Grade - January 2026
//!
//! Enterprise-grade analytics service for trading room performance analysis.
//! Provides comprehensive metrics including:
//! - Performance summaries with Sharpe ratio, profit factor, max drawdown
//! - Ticker-level performance breakdown
//! - Setup effectiveness analysis
//! - Monthly/daily P&L tracking
//! - Alert-to-trade conversion analytics
//! - Win/loss streak analysis
//! - Equity curve and drawdown calculations

use chrono::{Datelike, Duration, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::{error, info};

// ═══════════════════════════════════════════════════════════════════════════════════
// TYPE DEFINITIONS - Analytics Data Structures
// ═══════════════════════════════════════════════════════════════════════════════════

/// Complete room analytics response
#[derive(Debug, Clone, Serialize)]
pub struct RoomAnalytics {
    pub summary: AnalyticsSummary,
    pub performance_by_ticker: Vec<TickerPerformance>,
    pub performance_by_setup: Vec<SetupPerformance>,
    pub monthly_performance: Vec<MonthlyPerformance>,
    pub daily_pnl: Vec<DailyPnL>,
    pub alert_effectiveness: AlertEffectiveness,
    pub streak_analysis: StreakAnalysis,
}

/// High-level performance summary with key metrics
#[derive(Debug, Clone, Serialize, Default)]
pub struct AnalyticsSummary {
    pub total_alerts: i64,
    pub total_trades: i64,
    pub win_rate: f64,
    pub profit_factor: f64,
    pub sharpe_ratio: f64,
    pub max_drawdown: f64,
    pub max_drawdown_percent: f64,
    pub avg_holding_days: f64,
    pub total_pnl: f64,
    pub total_pnl_percent: f64,
    pub best_month: Option<MonthlyPerformance>,
    pub worst_month: Option<MonthlyPerformance>,
    pub avg_win_percent: f64,
    pub avg_loss_percent: f64,
    pub largest_win_percent: f64,
    pub largest_loss_percent: f64,
    pub risk_reward_ratio: f64,
    pub expectancy: f64,
}

/// Performance metrics per ticker symbol
#[derive(Debug, Clone, Serialize)]
pub struct TickerPerformance {
    pub ticker: String,
    pub total_trades: i64,
    pub wins: i64,
    pub losses: i64,
    pub win_rate: f64,
    pub total_pnl: f64,
    pub total_pnl_percent: f64,
    pub avg_pnl: f64,
    pub avg_pnl_percent: f64,
    pub avg_holding_days: f64,
    pub largest_win_percent: f64,
    pub largest_loss_percent: f64,
}

/// Performance metrics per trade setup type
#[derive(Debug, Clone, Serialize)]
pub struct SetupPerformance {
    pub setup: String,
    pub total_trades: i64,
    pub wins: i64,
    pub losses: i64,
    pub win_rate: f64,
    pub total_pnl: f64,
    pub avg_pnl: f64,
    pub profit_factor: f64,
}

/// Monthly performance breakdown
#[derive(Debug, Clone, Serialize, Default)]
pub struct MonthlyPerformance {
    pub year: i32,
    pub month: i32,
    pub month_name: String,
    pub total_trades: i64,
    pub wins: i64,
    pub losses: i64,
    pub win_rate: f64,
    pub pnl: f64,
    pub pnl_percent: f64,
    pub is_positive: bool,
}

/// Daily P&L for equity curve calculation
#[derive(Debug, Clone, Serialize)]
pub struct DailyPnL {
    pub date: NaiveDate,
    pub pnl: f64,
    pub pnl_percent: f64,
    pub cumulative_pnl: f64,
    pub cumulative_pnl_percent: f64,
    pub trade_count: i64,
}

/// Alert-to-trade conversion effectiveness metrics
#[derive(Debug, Clone, Serialize, Default)]
pub struct AlertEffectiveness {
    pub total_alerts: i64,
    pub alerts_with_trades: i64,
    pub alerts_without_trades: i64,
    pub conversion_rate: f64,
    pub profitable_conversion_rate: f64,
    pub avg_time_to_trade_hours: f64,
    pub entry_alerts: i64,
    pub update_alerts: i64,
    pub exit_alerts: i64,
}

/// Win/loss streak analysis
#[derive(Debug, Clone, Serialize, Default)]
pub struct StreakAnalysis {
    pub current_streak: i32,
    pub current_streak_type: String,
    pub max_win_streak: i32,
    pub max_loss_streak: i32,
    pub avg_win_streak: f64,
    pub avg_loss_streak: f64,
}

/// Equity curve data point
#[derive(Debug, Clone, Serialize)]
pub struct EquityPoint {
    pub date: NaiveDate,
    pub equity: f64,
    pub equity_percent: f64,
    pub drawdown: f64,
    pub drawdown_percent: f64,
}

/// Drawdown period details
#[derive(Debug, Clone, Serialize)]
pub struct DrawdownPeriod {
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub recovery_date: Option<NaiveDate>,
    pub max_drawdown: f64,
    pub max_drawdown_percent: f64,
    pub duration_days: i64,
    pub recovery_days: Option<i64>,
    pub is_recovered: bool,
}

/// Date range filter for analytics queries
#[derive(Debug, Clone, Deserialize)]
pub struct DateRange {
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
}

impl Default for DateRange {
    fn default() -> Self {
        Self {
            from: Some(Utc::now().date_naive() - Duration::days(365)),
            to: Some(Utc::now().date_naive()),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════
// DATABASE ROW TYPES - For SQLx query mapping
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, sqlx::FromRow)]
struct TradeRow {
    id: i64,
    ticker: String,
    entry_date: NaiveDate,
    exit_date: Option<NaiveDate>,
    entry_price: f64,
    exit_price: Option<f64>,
    pnl: Option<f64>,
    pnl_percent: Option<f64>,
    result: Option<String>,
    setup: Option<String>,
    holding_days: Option<i32>,
    direction: String,
}

#[derive(Debug, sqlx::FromRow)]
struct AlertRow {
    id: i64,
    alert_type: String,
    ticker: String,
    published_at: chrono::DateTime<Utc>,
    trade_plan_id: Option<i64>,
}

#[derive(Debug, sqlx::FromRow)]
struct MonthlyStatsRow {
    year: i32,
    month: i32,
    total_trades: i64,
    wins: i64,
    losses: i64,
    total_pnl: f64,
    total_pnl_percent: f64,
}

#[derive(Debug, sqlx::FromRow)]
struct TickerStatsRow {
    ticker: String,
    total_trades: i64,
    wins: i64,
    losses: i64,
    total_pnl: f64,
    total_pnl_percent: f64,
    avg_pnl: f64,
    avg_pnl_percent: f64,
    avg_holding_days: f64,
    largest_win_percent: f64,
    largest_loss_percent: f64,
}

#[derive(Debug, sqlx::FromRow)]
struct SetupStatsRow {
    setup: String,
    total_trades: i64,
    wins: i64,
    losses: i64,
    total_pnl: f64,
    avg_pnl: f64,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// ANALYTICS SERVICE
// ═══════════════════════════════════════════════════════════════════════════════════

/// Analytics service for computing trading performance metrics
#[derive(Clone)]
pub struct AnalyticsService {
    pool: PgPool,
}

impl AnalyticsService {
    /// Create new analytics service instance
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get comprehensive room analytics
    pub async fn get_room_analytics(
        &self,
        room_slug: &str,
        date_range: DateRange,
    ) -> Result<RoomAnalytics, sqlx::Error> {
        let from_date = date_range
            .from
            .unwrap_or_else(|| Utc::now().date_naive() - Duration::days(365));
        let to_date = date_range.to.unwrap_or_else(|| Utc::now().date_naive());

        info!(
            "Calculating analytics for {} from {} to {}",
            room_slug, from_date, to_date
        );

        // Fetch all closed trades in date range
        let trades = self.fetch_trades(room_slug, from_date, to_date).await?;

        // Fetch alerts in date range
        let alerts = self.fetch_alerts(room_slug, from_date, to_date).await?;

        // Calculate all metrics
        let summary = self.calculate_summary(&trades, &alerts);
        let performance_by_ticker = self
            .calculate_ticker_performance(room_slug, from_date, to_date)
            .await?;
        let performance_by_setup = self
            .calculate_setup_performance(room_slug, from_date, to_date)
            .await?;
        let monthly_performance = self
            .calculate_monthly_performance(room_slug, from_date, to_date)
            .await?;
        let daily_pnl = self.calculate_daily_pnl(&trades);
        let alert_effectiveness = self.calculate_alert_effectiveness(&alerts, &trades);
        let streak_analysis = self.calculate_streak_analysis(&trades);

        Ok(RoomAnalytics {
            summary,
            performance_by_ticker,
            performance_by_setup,
            monthly_performance,
            daily_pnl,
            alert_effectiveness,
            streak_analysis,
        })
    }

    /// Get equity curve data for charting
    pub async fn get_equity_curve(
        &self,
        room_slug: &str,
        date_range: DateRange,
    ) -> Result<Vec<EquityPoint>, sqlx::Error> {
        let from_date = date_range
            .from
            .unwrap_or_else(|| Utc::now().date_naive() - Duration::days(365));
        let to_date = date_range.to.unwrap_or_else(|| Utc::now().date_naive());

        let trades = self.fetch_trades(room_slug, from_date, to_date).await?;
        let daily_pnl = self.calculate_daily_pnl(&trades);

        let mut equity_curve = Vec::new();
        let mut peak_equity = 0.0f64;
        let starting_equity = 10000.0; // Normalized starting equity

        for day in &daily_pnl {
            let equity = starting_equity + day.cumulative_pnl;
            let equity_percent = (day.cumulative_pnl / starting_equity) * 100.0;

            peak_equity = peak_equity.max(equity);
            let drawdown = peak_equity - equity;
            let drawdown_percent = if peak_equity > 0.0 {
                (drawdown / peak_equity) * 100.0
            } else {
                0.0
            };

            equity_curve.push(EquityPoint {
                date: day.date,
                equity,
                equity_percent,
                drawdown,
                drawdown_percent,
            });
        }

        Ok(equity_curve)
    }

    /// Get drawdown periods analysis
    pub async fn get_drawdown_periods(
        &self,
        room_slug: &str,
        date_range: DateRange,
    ) -> Result<Vec<DrawdownPeriod>, sqlx::Error> {
        let equity_curve = self.get_equity_curve(room_slug, date_range).await?;
        let mut drawdown_periods = Vec::new();
        let mut current_drawdown: Option<DrawdownPeriod> = None;
        let mut peak_equity = 0.0f64;
        let mut peak_date = equity_curve.first().map(|e| e.date);

        for point in &equity_curve {
            if point.equity > peak_equity {
                // New peak - end current drawdown if any
                if let Some(mut dd) = current_drawdown.take() {
                    dd.end_date = Some(point.date);
                    dd.recovery_date = Some(point.date);
                    dd.is_recovered = true;
                    if let Some(start) = dd.end_date {
                        dd.recovery_days = Some((start - dd.start_date).num_days());
                    }
                    drawdown_periods.push(dd);
                }
                peak_equity = point.equity;
                peak_date = Some(point.date);
            } else if point.drawdown > 0.0 {
                // In drawdown
                match &mut current_drawdown {
                    Some(dd) => {
                        // Update if deeper drawdown
                        if point.drawdown > dd.max_drawdown {
                            dd.max_drawdown = point.drawdown;
                            dd.max_drawdown_percent = point.drawdown_percent;
                        }
                        dd.duration_days = (point.date - dd.start_date).num_days();
                    }
                    None => {
                        // Start new drawdown period
                        current_drawdown = Some(DrawdownPeriod {
                            start_date: peak_date.unwrap_or(point.date),
                            end_date: None,
                            recovery_date: None,
                            max_drawdown: point.drawdown,
                            max_drawdown_percent: point.drawdown_percent,
                            duration_days: 0,
                            recovery_days: None,
                            is_recovered: false,
                        });
                    }
                }
            }
        }

        // Add any ongoing drawdown
        if let Some(dd) = current_drawdown {
            drawdown_periods.push(dd);
        }

        // Filter to significant drawdowns (> 1%)
        drawdown_periods.retain(|dd| dd.max_drawdown_percent > 1.0);

        Ok(drawdown_periods)
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // PRIVATE HELPER METHODS
    // ═══════════════════════════════════════════════════════════════════════════════

    /// Fetch closed trades from database
    async fn fetch_trades(
        &self,
        room_slug: &str,
        from_date: NaiveDate,
        to_date: NaiveDate,
    ) -> Result<Vec<TradeRow>, sqlx::Error> {
        sqlx::query_as::<_, TradeRow>(
            r#"
            SELECT id, ticker, entry_date, exit_date, entry_price, exit_price,
                   pnl, pnl_percent, result, setup, holding_days, direction
            FROM room_trades
            WHERE room_slug = $1
              AND status = 'closed'
              AND deleted_at IS NULL
              AND exit_date >= $2
              AND exit_date <= $3
            ORDER BY exit_date ASC
            "#,
        )
        .bind(room_slug)
        .bind(from_date)
        .bind(to_date)
        .fetch_all(&self.pool)
        .await
    }

    /// Fetch alerts from database
    async fn fetch_alerts(
        &self,
        room_slug: &str,
        from_date: NaiveDate,
        to_date: NaiveDate,
    ) -> Result<Vec<AlertRow>, sqlx::Error> {
        sqlx::query_as::<_, AlertRow>(
            r#"
            SELECT id, alert_type, ticker, published_at, trade_plan_id
            FROM room_alerts
            WHERE room_slug = $1
              AND deleted_at IS NULL
              AND is_published = true
              AND DATE(published_at) >= $2
              AND DATE(published_at) <= $3
            ORDER BY published_at ASC
            "#,
        )
        .bind(room_slug)
        .bind(from_date)
        .bind(to_date)
        .fetch_all(&self.pool)
        .await
    }

    /// Calculate comprehensive summary metrics
    fn calculate_summary(&self, trades: &[TradeRow], alerts: &[AlertRow]) -> AnalyticsSummary {
        if trades.is_empty() {
            return AnalyticsSummary {
                total_alerts: alerts.len() as i64,
                ..Default::default()
            };
        }

        let total_trades = trades.len() as i64;
        let wins: Vec<_> = trades
            .iter()
            .filter(|t| t.result.as_deref() == Some("WIN"))
            .collect();
        let losses: Vec<_> = trades
            .iter()
            .filter(|t| t.result.as_deref() == Some("LOSS"))
            .collect();

        let win_count = wins.len() as i64;
        let loss_count = losses.len() as i64;
        let win_rate = if total_trades > 0 {
            (win_count as f64 / total_trades as f64) * 100.0
        } else {
            0.0
        };

        // P&L calculations
        let total_pnl: f64 = trades.iter().filter_map(|t| t.pnl).sum();
        let total_pnl_percent: f64 = trades.iter().filter_map(|t| t.pnl_percent).sum();

        let gross_profit: f64 = wins.iter().filter_map(|t| t.pnl).filter(|p| *p > 0.0).sum();
        let gross_loss: f64 = losses
            .iter()
            .filter_map(|t| t.pnl)
            .filter(|p| *p < 0.0)
            .map(|p| p.abs())
            .sum();

        let profit_factor = if gross_loss > 0.0 {
            gross_profit / gross_loss
        } else if gross_profit > 0.0 {
            f64::INFINITY
        } else {
            0.0
        };

        // Win/loss averages
        let avg_win_percent = if !wins.is_empty() {
            wins.iter().filter_map(|t| t.pnl_percent).sum::<f64>() / wins.len() as f64
        } else {
            0.0
        };

        let avg_loss_percent = if !losses.is_empty() {
            losses
                .iter()
                .filter_map(|t| t.pnl_percent)
                .map(|p| p.abs())
                .sum::<f64>()
                / losses.len() as f64
        } else {
            0.0
        };

        // Largest win/loss
        let largest_win_percent = trades
            .iter()
            .filter_map(|t| t.pnl_percent)
            .filter(|p| *p > 0.0)
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0);

        let largest_loss_percent = trades
            .iter()
            .filter_map(|t| t.pnl_percent)
            .filter(|p| *p < 0.0)
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0)
            .abs();

        // Risk/reward ratio
        let risk_reward_ratio = if avg_loss_percent > 0.0 {
            avg_win_percent / avg_loss_percent
        } else {
            0.0
        };

        // Expectancy (expected value per trade)
        let expectancy =
            (win_rate / 100.0 * avg_win_percent) - ((100.0 - win_rate) / 100.0 * avg_loss_percent);

        // Average holding days
        let avg_holding_days = if !trades.is_empty() {
            trades.iter().filter_map(|t| t.holding_days).sum::<i32>() as f64 / trades.len() as f64
        } else {
            0.0
        };

        // Sharpe ratio calculation (simplified - using daily returns)
        let daily_pnl = self.calculate_daily_pnl(trades);
        let sharpe_ratio = self.calculate_sharpe_ratio(&daily_pnl);

        // Max drawdown
        let (max_drawdown, max_drawdown_percent) = self.calculate_max_drawdown(&daily_pnl);

        // Best/worst month - will be set by monthly calculation
        AnalyticsSummary {
            total_alerts: alerts.len() as i64,
            total_trades,
            win_rate,
            profit_factor,
            sharpe_ratio,
            max_drawdown,
            max_drawdown_percent,
            avg_holding_days,
            total_pnl,
            total_pnl_percent,
            best_month: None,
            worst_month: None,
            avg_win_percent,
            avg_loss_percent,
            largest_win_percent,
            largest_loss_percent,
            risk_reward_ratio,
            expectancy,
        }
    }

    /// Calculate daily P&L from trades
    fn calculate_daily_pnl(&self, trades: &[TradeRow]) -> Vec<DailyPnL> {
        let mut daily_map: HashMap<NaiveDate, (f64, f64, i64)> = HashMap::new();

        for trade in trades {
            if let Some(exit_date) = trade.exit_date {
                let entry = daily_map.entry(exit_date).or_insert((0.0, 0.0, 0));
                entry.0 += trade.pnl.unwrap_or(0.0);
                entry.1 += trade.pnl_percent.unwrap_or(0.0);
                entry.2 += 1;
            }
        }

        let mut dates: Vec<_> = daily_map.keys().cloned().collect();
        dates.sort();

        let mut cumulative_pnl = 0.0;
        let mut cumulative_pnl_percent = 0.0;
        let mut daily_pnl = Vec::new();

        for date in dates {
            if let Some((pnl, pnl_percent, count)) = daily_map.get(&date) {
                cumulative_pnl += pnl;
                cumulative_pnl_percent += pnl_percent;

                daily_pnl.push(DailyPnL {
                    date,
                    pnl: *pnl,
                    pnl_percent: *pnl_percent,
                    cumulative_pnl,
                    cumulative_pnl_percent,
                    trade_count: *count,
                });
            }
        }

        daily_pnl
    }

    /// Calculate Sharpe ratio from daily returns
    fn calculate_sharpe_ratio(&self, daily_pnl: &[DailyPnL]) -> f64 {
        if daily_pnl.len() < 2 {
            return 0.0;
        }

        let returns: Vec<f64> = daily_pnl.iter().map(|d| d.pnl_percent).collect();
        let mean_return = returns.iter().sum::<f64>() / returns.len() as f64;

        let variance = returns
            .iter()
            .map(|r| (r - mean_return).powi(2))
            .sum::<f64>()
            / returns.len() as f64;

        let std_dev = variance.sqrt();

        if std_dev > 0.0 {
            // Annualized Sharpe (assuming 252 trading days)
            (mean_return / std_dev) * (252.0_f64).sqrt()
        } else {
            0.0
        }
    }

    /// Calculate maximum drawdown
    fn calculate_max_drawdown(&self, daily_pnl: &[DailyPnL]) -> (f64, f64) {
        if daily_pnl.is_empty() {
            return (0.0, 0.0);
        }

        let starting_equity = 10000.0;
        let mut peak_equity = starting_equity;
        let mut max_drawdown = 0.0f64;
        let mut max_drawdown_percent = 0.0f64;

        for day in daily_pnl {
            let equity = starting_equity + day.cumulative_pnl;
            peak_equity = peak_equity.max(equity);

            let drawdown = peak_equity - equity;
            let drawdown_percent = if peak_equity > 0.0 {
                (drawdown / peak_equity) * 100.0
            } else {
                0.0
            };

            max_drawdown = max_drawdown.max(drawdown);
            max_drawdown_percent = max_drawdown_percent.max(drawdown_percent);
        }

        (max_drawdown, max_drawdown_percent)
    }

    /// Calculate performance by ticker
    async fn calculate_ticker_performance(
        &self,
        room_slug: &str,
        from_date: NaiveDate,
        to_date: NaiveDate,
    ) -> Result<Vec<TickerPerformance>, sqlx::Error> {
        let rows = sqlx::query_as::<_, TickerStatsRow>(
            r#"
            SELECT
                ticker,
                COUNT(*) as total_trades,
                COUNT(*) FILTER (WHERE result = 'WIN') as wins,
                COUNT(*) FILTER (WHERE result = 'LOSS') as losses,
                COALESCE(SUM(pnl), 0) as total_pnl,
                COALESCE(SUM(pnl_percent), 0) as total_pnl_percent,
                COALESCE(AVG(pnl), 0) as avg_pnl,
                COALESCE(AVG(pnl_percent), 0) as avg_pnl_percent,
                COALESCE(AVG(holding_days), 0) as avg_holding_days,
                COALESCE(MAX(pnl_percent) FILTER (WHERE pnl_percent > 0), 0) as largest_win_percent,
                COALESCE(ABS(MIN(pnl_percent) FILTER (WHERE pnl_percent < 0)), 0) as largest_loss_percent
            FROM room_trades
            WHERE room_slug = $1
              AND status = 'closed'
              AND deleted_at IS NULL
              AND exit_date >= $2
              AND exit_date <= $3
            GROUP BY ticker
            ORDER BY total_pnl DESC
            "#,
        )
        .bind(room_slug)
        .bind(from_date)
        .bind(to_date)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| {
                let win_rate = if r.total_trades > 0 {
                    (r.wins as f64 / r.total_trades as f64) * 100.0
                } else {
                    0.0
                };

                TickerPerformance {
                    ticker: r.ticker,
                    total_trades: r.total_trades,
                    wins: r.wins,
                    losses: r.losses,
                    win_rate,
                    total_pnl: r.total_pnl,
                    total_pnl_percent: r.total_pnl_percent,
                    avg_pnl: r.avg_pnl,
                    avg_pnl_percent: r.avg_pnl_percent,
                    avg_holding_days: r.avg_holding_days,
                    largest_win_percent: r.largest_win_percent,
                    largest_loss_percent: r.largest_loss_percent,
                }
            })
            .collect())
    }

    /// Calculate performance by setup type
    async fn calculate_setup_performance(
        &self,
        room_slug: &str,
        from_date: NaiveDate,
        to_date: NaiveDate,
    ) -> Result<Vec<SetupPerformance>, sqlx::Error> {
        let rows = sqlx::query_as::<_, SetupStatsRow>(
            r#"
            SELECT
                COALESCE(setup, 'Unknown') as setup,
                COUNT(*) as total_trades,
                COUNT(*) FILTER (WHERE result = 'WIN') as wins,
                COUNT(*) FILTER (WHERE result = 'LOSS') as losses,
                COALESCE(SUM(pnl), 0) as total_pnl,
                COALESCE(AVG(pnl), 0) as avg_pnl
            FROM room_trades
            WHERE room_slug = $1
              AND status = 'closed'
              AND deleted_at IS NULL
              AND exit_date >= $2
              AND exit_date <= $3
            GROUP BY COALESCE(setup, 'Unknown')
            ORDER BY total_trades DESC
            "#,
        )
        .bind(room_slug)
        .bind(from_date)
        .bind(to_date)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| {
                let win_rate = if r.total_trades > 0 {
                    (r.wins as f64 / r.total_trades as f64) * 100.0
                } else {
                    0.0
                };

                // Calculate profit factor for this setup
                let gross_profit = if r.avg_pnl > 0.0 {
                    r.avg_pnl * r.wins as f64
                } else {
                    0.0
                };
                let gross_loss = if r.avg_pnl < 0.0 {
                    r.avg_pnl.abs() * r.losses as f64
                } else {
                    r.total_pnl.abs() - gross_profit.abs()
                };
                let profit_factor = if gross_loss > 0.0 {
                    gross_profit / gross_loss
                } else if gross_profit > 0.0 {
                    f64::INFINITY
                } else {
                    0.0
                };

                SetupPerformance {
                    setup: r.setup,
                    total_trades: r.total_trades,
                    wins: r.wins,
                    losses: r.losses,
                    win_rate,
                    total_pnl: r.total_pnl,
                    avg_pnl: r.avg_pnl,
                    profit_factor: if profit_factor.is_infinite() {
                        999.99
                    } else {
                        profit_factor
                    },
                }
            })
            .collect())
    }

    /// Calculate monthly performance breakdown
    async fn calculate_monthly_performance(
        &self,
        room_slug: &str,
        from_date: NaiveDate,
        to_date: NaiveDate,
    ) -> Result<Vec<MonthlyPerformance>, sqlx::Error> {
        let rows = sqlx::query_as::<_, MonthlyStatsRow>(
            r#"
            SELECT
                EXTRACT(YEAR FROM exit_date)::int as year,
                EXTRACT(MONTH FROM exit_date)::int as month,
                COUNT(*) as total_trades,
                COUNT(*) FILTER (WHERE result = 'WIN') as wins,
                COUNT(*) FILTER (WHERE result = 'LOSS') as losses,
                COALESCE(SUM(pnl), 0) as total_pnl,
                COALESCE(SUM(pnl_percent), 0) as total_pnl_percent
            FROM room_trades
            WHERE room_slug = $1
              AND status = 'closed'
              AND deleted_at IS NULL
              AND exit_date >= $2
              AND exit_date <= $3
            GROUP BY EXTRACT(YEAR FROM exit_date), EXTRACT(MONTH FROM exit_date)
            ORDER BY year DESC, month DESC
            "#,
        )
        .bind(room_slug)
        .bind(from_date)
        .bind(to_date)
        .fetch_all(&self.pool)
        .await?;

        let month_names = [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ];

        Ok(rows
            .into_iter()
            .map(|r| {
                let win_rate = if r.total_trades > 0 {
                    (r.wins as f64 / r.total_trades as f64) * 100.0
                } else {
                    0.0
                };

                MonthlyPerformance {
                    year: r.year,
                    month: r.month,
                    month_name: month_names
                        .get((r.month - 1) as usize)
                        .unwrap_or(&"Unknown")
                        .to_string(),
                    total_trades: r.total_trades,
                    wins: r.wins,
                    losses: r.losses,
                    win_rate,
                    pnl: r.total_pnl,
                    pnl_percent: r.total_pnl_percent,
                    is_positive: r.total_pnl > 0.0,
                }
            })
            .collect())
    }

    /// Calculate alert effectiveness metrics
    fn calculate_alert_effectiveness(
        &self,
        alerts: &[AlertRow],
        trades: &[TradeRow],
    ) -> AlertEffectiveness {
        let total_alerts = alerts.len() as i64;

        let entry_alerts = alerts
            .iter()
            .filter(|a| a.alert_type.to_uppercase() == "ENTRY")
            .count() as i64;
        let update_alerts = alerts
            .iter()
            .filter(|a| a.alert_type.to_uppercase() == "UPDATE")
            .count() as i64;
        let exit_alerts = alerts
            .iter()
            .filter(|a| a.alert_type.to_uppercase() == "EXIT")
            .count() as i64;

        // Count alerts that led to trades (simplified - match by ticker)
        let alert_tickers: std::collections::HashSet<_> = alerts
            .iter()
            .filter(|a| a.alert_type.to_uppercase() == "ENTRY")
            .map(|a| &a.ticker)
            .collect();

        let trade_tickers: std::collections::HashSet<_> =
            trades.iter().map(|t| &t.ticker).collect();
        let matched_tickers = alert_tickers.intersection(&trade_tickers).count() as i64;

        let alerts_with_trades = matched_tickers;
        let alerts_without_trades = entry_alerts - alerts_with_trades;

        let conversion_rate = if entry_alerts > 0 {
            (alerts_with_trades as f64 / entry_alerts as f64) * 100.0
        } else {
            0.0
        };

        // Profitable conversion rate
        let profitable_trades = trades.iter().filter(|t| t.pnl.unwrap_or(0.0) > 0.0).count() as i64;
        let profitable_conversion_rate = if total_alerts > 0 {
            (profitable_trades as f64 / total_alerts as f64) * 100.0
        } else {
            0.0
        };

        AlertEffectiveness {
            total_alerts,
            alerts_with_trades,
            alerts_without_trades,
            conversion_rate,
            profitable_conversion_rate,
            avg_time_to_trade_hours: 0.0, // Would need more data to calculate
            entry_alerts,
            update_alerts,
            exit_alerts,
        }
    }

    /// Calculate win/loss streak analysis
    fn calculate_streak_analysis(&self, trades: &[TradeRow]) -> StreakAnalysis {
        if trades.is_empty() {
            return StreakAnalysis::default();
        }

        let mut current_streak = 0i32;
        let mut current_streak_type = String::new();
        let mut max_win_streak = 0i32;
        let mut max_loss_streak = 0i32;
        let mut win_streaks: Vec<i32> = Vec::new();
        let mut loss_streaks: Vec<i32> = Vec::new();
        let mut temp_streak = 0i32;
        let mut last_result: Option<&str> = None;

        for trade in trades {
            let result = trade.result.as_deref();

            match (result, last_result) {
                (Some("WIN"), Some("WIN")) => {
                    temp_streak += 1;
                }
                (Some("WIN"), _) => {
                    if last_result == Some("LOSS") && temp_streak > 0 {
                        loss_streaks.push(temp_streak);
                        max_loss_streak = max_loss_streak.max(temp_streak);
                    }
                    temp_streak = 1;
                }
                (Some("LOSS"), Some("LOSS")) => {
                    temp_streak += 1;
                }
                (Some("LOSS"), _) => {
                    if last_result == Some("WIN") && temp_streak > 0 {
                        win_streaks.push(temp_streak);
                        max_win_streak = max_win_streak.max(temp_streak);
                    }
                    temp_streak = 1;
                }
                _ => {}
            }

            last_result = result;
        }

        // Handle last streak
        if let Some(result) = last_result {
            if result == "WIN" {
                win_streaks.push(temp_streak);
                max_win_streak = max_win_streak.max(temp_streak);
                current_streak = temp_streak;
                current_streak_type = "WIN".to_string();
            } else if result == "LOSS" {
                loss_streaks.push(temp_streak);
                max_loss_streak = max_loss_streak.max(temp_streak);
                current_streak = temp_streak;
                current_streak_type = "LOSS".to_string();
            }
        }

        let avg_win_streak = if !win_streaks.is_empty() {
            win_streaks.iter().sum::<i32>() as f64 / win_streaks.len() as f64
        } else {
            0.0
        };

        let avg_loss_streak = if !loss_streaks.is_empty() {
            loss_streaks.iter().sum::<i32>() as f64 / loss_streaks.len() as f64
        } else {
            0.0
        };

        StreakAnalysis {
            current_streak,
            current_streak_type,
            max_win_streak,
            max_loss_streak,
            avg_win_streak,
            avg_loss_streak,
        }
    }
}

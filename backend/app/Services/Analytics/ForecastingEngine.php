<?php

declare(strict_types=1);

namespace App\Services\Analytics;

use App\Models\Analytics\AnalyticsKpiValue;
use App\Models\Analytics\AnalyticsForecast;
use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;

/**
 * ForecastingEngine - Predictive Analytics Engine
 *
 * Generates forecasts using multiple statistical models.
 *
 * @package App\Services\Analytics
 */
class ForecastingEngine
{
    /**
     * Minimum data points for forecasting
     */
    private const MIN_HISTORICAL_POINTS = 14;

    /**
     * Default confidence level
     */
    private const DEFAULT_CONFIDENCE = 0.95;

    public function __construct(
        private readonly AnalyticsCacheManager $cacheManager,
    ) {}

    /**
     * Generate forecast for a KPI
     */
    public function forecast(
        string $kpiKey,
        string $granularity,
        int $periodsAhead,
        string $model = 'linear'
    ): array {
        // Get historical data
        $historical = $this->getHistoricalData($kpiKey, $granularity);

        if ($historical->count() < self::MIN_HISTORICAL_POINTS) {
            return [
                'error' => 'Insufficient historical data',
                'required' => self::MIN_HISTORICAL_POINTS,
                'available' => $historical->count(),
            ];
        }

        // Generate forecasts based on model
        $forecasts = match ($model) {
            'linear' => $this->linearForecast($historical, $periodsAhead),
            'exponential' => $this->exponentialForecast($historical, $periodsAhead),
            'moving_average' => $this->movingAverageForecast($historical, $periodsAhead),
            'seasonal' => $this->seasonalForecast($historical, $periodsAhead, $granularity),
            default => $this->linearForecast($historical, $periodsAhead),
        };

        // Calculate model accuracy from historical data
        $accuracy = $this->calculateModelAccuracy($historical, $model);

        // Store forecasts
        $this->storeForecast($kpiKey, $granularity, $model, $forecasts);

        return [
            'metric_key' => $kpiKey,
            'model' => $model,
            'granularity' => $granularity,
            'historical_points' => $historical->count(),
            'model_accuracy' => round($accuracy, 4),
            'forecasts' => $forecasts,
        ];
    }

    /**
     * Get historical data for forecasting
     */
    protected function getHistoricalData(string $kpiKey, string $granularity): Collection
    {
        $periodType = match ($granularity) {
            'daily' => 'daily',
            'weekly' => 'weekly',
            'monthly' => 'monthly',
            default => 'daily',
        };

        return AnalyticsKpiValue::byKpiKey($kpiKey)
            ->byPeriodType($periodType)
            ->noSegment()
            ->orderBy('period_start')
            ->get()
            ->pluck('value', 'period_start');
    }

    /**
     * Linear regression forecast
     */
    protected function linearForecast(Collection $historical, int $periodsAhead): array
    {
        $values = $historical->values()->toArray();
        $n = count($values);

        // Calculate linear regression coefficients
        $sumX = 0;
        $sumY = 0;
        $sumXY = 0;
        $sumXX = 0;

        for ($i = 0; $i < $n; $i++) {
            $sumX += $i;
            $sumY += $values[$i];
            $sumXY += $i * $values[$i];
            $sumXX += $i * $i;
        }

        $slope = ($n * $sumXY - $sumX * $sumY) / ($n * $sumXX - $sumX * $sumX);
        $intercept = ($sumY - $slope * $sumX) / $n;

        // Calculate standard error for confidence intervals
        $residuals = [];
        for ($i = 0; $i < $n; $i++) {
            $predicted = $intercept + $slope * $i;
            $residuals[] = $values[$i] - $predicted;
        }
        $stdError = sqrt(array_sum(array_map(fn ($r) => $r * $r, $residuals)) / ($n - 2));

        // Generate forecasts
        $forecasts = [];
        $lastDate = $historical->keys()->last();
        $zScore = 1.96; // 95% confidence

        for ($i = 1; $i <= $periodsAhead; $i++) {
            $x = $n + $i - 1;
            $predicted = $intercept + $slope * $x;

            // Confidence interval widens as we forecast further
            $margin = $zScore * $stdError * sqrt(1 + 1 / $n + pow($x - ($n - 1) / 2, 2) / $sumXX);

            $forecastDate = Carbon::parse($lastDate)->addDays($i);

            $forecasts[] = [
                'date' => $forecastDate->format('Y-m-d'),
                'predicted_value' => round(max(0, $predicted), 2),
                'lower_bound' => round(max(0, $predicted - $margin), 2),
                'upper_bound' => round($predicted + $margin, 2),
                'confidence' => self::DEFAULT_CONFIDENCE,
            ];
        }

        return $forecasts;
    }

    /**
     * Exponential smoothing forecast
     */
    protected function exponentialForecast(Collection $historical, int $periodsAhead): array
    {
        $values = $historical->values()->toArray();
        $n = count($values);

        // Optimize alpha using grid search
        $bestAlpha = $this->optimizeAlpha($values);

        // Apply exponential smoothing
        $smoothed = [$values[0]];
        for ($i = 1; $i < $n; $i++) {
            $smoothed[$i] = $bestAlpha * $values[$i] + (1 - $bestAlpha) * $smoothed[$i - 1];
        }

        // Calculate trend
        $trend = ($smoothed[$n - 1] - $smoothed[0]) / ($n - 1);

        // Calculate error for confidence intervals
        $errors = [];
        for ($i = 1; $i < $n; $i++) {
            $errors[] = abs($values[$i] - $smoothed[$i - 1]);
        }
        $avgError = count($errors) > 0 ? array_sum($errors) / count($errors) : 0;

        // Generate forecasts
        $forecasts = [];
        $lastDate = $historical->keys()->last();
        $lastSmoothed = $smoothed[$n - 1];

        for ($i = 1; $i <= $periodsAhead; $i++) {
            $predicted = $lastSmoothed + $trend * $i;
            $margin = $avgError * sqrt($i) * 1.96;

            $forecastDate = Carbon::parse($lastDate)->addDays($i);

            $forecasts[] = [
                'date' => $forecastDate->format('Y-m-d'),
                'predicted_value' => round(max(0, $predicted), 2),
                'lower_bound' => round(max(0, $predicted - $margin), 2),
                'upper_bound' => round($predicted + $margin, 2),
                'confidence' => self::DEFAULT_CONFIDENCE,
            ];
        }

        return $forecasts;
    }

    /**
     * Optimize exponential smoothing alpha
     */
    protected function optimizeAlpha(array $values): float
    {
        $bestAlpha = 0.3;
        $bestError = PHP_FLOAT_MAX;

        for ($alpha = 0.1; $alpha <= 0.9; $alpha += 0.1) {
            $smoothed = [$values[0]];
            $error = 0;

            for ($i = 1; $i < count($values); $i++) {
                $smoothed[$i] = $alpha * $values[$i] + (1 - $alpha) * $smoothed[$i - 1];
                $error += pow($values[$i] - $smoothed[$i - 1], 2);
            }

            if ($error < $bestError) {
                $bestError = $error;
                $bestAlpha = $alpha;
            }
        }

        return $bestAlpha;
    }

    /**
     * Moving average forecast
     */
    protected function movingAverageForecast(Collection $historical, int $periodsAhead, int $window = 7): array
    {
        $values = $historical->values()->toArray();
        $n = count($values);

        // Calculate recent moving average
        $recentValues = array_slice($values, -$window);
        $movingAvg = array_sum($recentValues) / count($recentValues);

        // Calculate standard deviation for confidence
        $variance = 0;
        foreach ($recentValues as $v) {
            $variance += pow($v - $movingAvg, 2);
        }
        $stdDev = sqrt($variance / count($recentValues));

        // Generate forecasts
        $forecasts = [];
        $lastDate = $historical->keys()->last();

        for ($i = 1; $i <= $periodsAhead; $i++) {
            $margin = 1.96 * $stdDev;
            $forecastDate = Carbon::parse($lastDate)->addDays($i);

            $forecasts[] = [
                'date' => $forecastDate->format('Y-m-d'),
                'predicted_value' => round(max(0, $movingAvg), 2),
                'lower_bound' => round(max(0, $movingAvg - $margin), 2),
                'upper_bound' => round($movingAvg + $margin, 2),
                'confidence' => self::DEFAULT_CONFIDENCE,
            ];
        }

        return $forecasts;
    }

    /**
     * Seasonal forecast (with weekly seasonality)
     */
    protected function seasonalForecast(
        Collection $historical,
        int $periodsAhead,
        string $granularity
    ): array {
        $values = $historical->values()->toArray();
        $n = count($values);

        $seasonLength = match ($granularity) {
            'daily' => 7, // Weekly seasonality
            'weekly' => 4, // Monthly seasonality
            'monthly' => 12, // Yearly seasonality
            default => 7,
        };

        // Calculate seasonal indices
        $seasonalIndices = [];
        $seasonalCounts = [];

        for ($i = 0; $i < $seasonLength; $i++) {
            $seasonalIndices[$i] = 0;
            $seasonalCounts[$i] = 0;
        }

        $overallAvg = array_sum($values) / $n;

        for ($i = 0; $i < $n; $i++) {
            $seasonIndex = $i % $seasonLength;
            $seasonalIndices[$seasonIndex] += $values[$i] / $overallAvg;
            $seasonalCounts[$seasonIndex]++;
        }

        for ($i = 0; $i < $seasonLength; $i++) {
            $seasonalIndices[$i] = $seasonalCounts[$i] > 0
                ? $seasonalIndices[$i] / $seasonalCounts[$i]
                : 1;
        }

        // Calculate deseasonalized trend
        $recentAvg = array_sum(array_slice($values, -$seasonLength)) / $seasonLength;

        // Generate forecasts
        $forecasts = [];
        $lastDate = $historical->keys()->last();

        for ($i = 1; $i <= $periodsAhead; $i++) {
            $seasonIndex = ($n + $i - 1) % $seasonLength;
            $predicted = $recentAvg * $seasonalIndices[$seasonIndex];

            $forecastDate = Carbon::parse($lastDate)->addDays($i);

            $forecasts[] = [
                'date' => $forecastDate->format('Y-m-d'),
                'predicted_value' => round(max(0, $predicted), 2),
                'lower_bound' => round(max(0, $predicted * 0.8), 2),
                'upper_bound' => round($predicted * 1.2, 2),
                'confidence' => self::DEFAULT_CONFIDENCE,
                'seasonal_index' => round($seasonalIndices[$seasonIndex], 3),
            ];
        }

        return $forecasts;
    }

    /**
     * Calculate model accuracy using holdout validation
     */
    protected function calculateModelAccuracy(Collection $historical, string $model): float
    {
        $values = $historical->values()->toArray();
        $n = count($values);

        if ($n < 14) {
            return 0;
        }

        // Use last 7 points as test set
        $trainSize = $n - 7;
        $trainData = collect(array_slice($values, 0, $trainSize));

        // Generate forecast for test period
        $testForecasts = match ($model) {
            'linear' => $this->linearForecast($trainData->combine(range(0, $trainSize - 1)), 7),
            'exponential' => $this->exponentialForecast($trainData->combine(range(0, $trainSize - 1)), 7),
            'moving_average' => $this->movingAverageForecast($trainData->combine(range(0, $trainSize - 1)), 7),
            default => $this->linearForecast($trainData->combine(range(0, $trainSize - 1)), 7),
        };

        // Calculate MAPE (Mean Absolute Percentage Error)
        $totalError = 0;
        $testValues = array_slice($values, $trainSize);

        foreach ($testForecasts as $i => $forecast) {
            if (isset($testValues[$i]) && $testValues[$i] > 0) {
                $totalError += abs($testValues[$i] - $forecast['predicted_value']) / $testValues[$i];
            }
        }

        $mape = (count($testForecasts) > 0) ? ($totalError / count($testForecasts)) : 1;

        // Return accuracy (1 - MAPE), bounded between 0 and 1
        return max(0, min(1, 1 - $mape));
    }

    /**
     * Store forecast in database
     */
    protected function storeForecast(
        string $kpiKey,
        string $granularity,
        string $model,
        array $forecasts
    ): void {
        foreach ($forecasts as $forecast) {
            AnalyticsForecast::updateOrCreate(
                [
                    'metric_key' => $kpiKey,
                    'forecast_date' => $forecast['date'],
                    'granularity' => $granularity,
                ],
                [
                    'model_type' => $model,
                    'predicted_value' => $forecast['predicted_value'],
                    'lower_bound' => $forecast['lower_bound'],
                    'upper_bound' => $forecast['upper_bound'],
                    'confidence_level' => $forecast['confidence'],
                    'generated_at' => now(),
                ]
            );
        }
    }

    /**
     * Get forecast accuracy report
     */
    public function getAccuracyReport(string $kpiKey, string $granularity): array
    {
        // Get forecasts that have actual values now
        $forecasts = AnalyticsForecast::where('metric_key', $kpiKey)
            ->where('granularity', $granularity)
            ->where('forecast_date', '<=', now())
            ->whereNotNull('actual_value')
            ->get();

        if ($forecasts->isEmpty()) {
            return ['message' => 'No completed forecasts available'];
        }

        $totalError = 0;
        $withinBounds = 0;

        foreach ($forecasts as $forecast) {
            $error = abs($forecast->actual_value - $forecast->predicted_value);
            $totalError += $forecast->actual_value > 0 ? $error / $forecast->actual_value : 0;

            if ($forecast->actual_value >= $forecast->lower_bound &&
                $forecast->actual_value <= $forecast->upper_bound) {
                $withinBounds++;
            }
        }

        $mape = $forecasts->count() > 0 ? ($totalError / $forecasts->count()) * 100 : 0;
        $boundAccuracy = $forecasts->count() > 0 ? ($withinBounds / $forecasts->count()) * 100 : 0;

        return [
            'metric_key' => $kpiKey,
            'forecasts_evaluated' => $forecasts->count(),
            'mape' => round($mape, 2),
            'accuracy' => round(100 - $mape, 2),
            'within_confidence_bounds' => round($boundAccuracy, 2),
        ];
    }
}

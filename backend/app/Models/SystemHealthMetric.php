<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;

class SystemHealthMetric extends Model
{
    use HasFactory;

    public $timestamps = false;

    protected $fillable = [
        'metric_type',
        'service_name',
        'value',
        'unit',
        'status',
        'metadata',
        'recorded_at',
    ];

    protected $casts = [
        'value' => 'decimal:2',
        'metadata' => 'array',
        'recorded_at' => 'datetime',
    ];

    public static function recordMetric(
        string $metricType,
        string $serviceName,
        float $value,
        string $unit,
        string $status = 'healthy',
        ?array $metadata = null
    ): self {
        return self::create([
            'metric_type' => $metricType,
            'service_name' => $serviceName,
            'value' => $value,
            'unit' => $unit,
            'status' => $status,
            'metadata' => $metadata,
            'recorded_at' => now(),
        ]);
    }

    public function scopeForService($query, string $serviceName)
    {
        return $query->where('service_name', $serviceName);
    }

    public function scopeByType($query, string $metricType)
    {
        return $query->where('metric_type', $metricType);
    }

    public function scopeByStatus($query, string $status)
    {
        return $query->where('status', $status);
    }

    public function scopeRecent($query, int $hours = 24)
    {
        return $query->where('recorded_at', '>=', now()->subHours($hours));
    }

    public static function getServiceHealth(string $serviceName): array
    {
        $metrics = self::forService($serviceName)
            ->recent(1)
            ->orderBy('recorded_at', 'desc')
            ->get()
            ->groupBy('metric_type');

        $criticalCount = self::forService($serviceName)
            ->recent(1)
            ->byStatus('critical')
            ->count();

        $warningCount = self::forService($serviceName)
            ->recent(1)
            ->byStatus('warning')
            ->count();

        $overallStatus = $criticalCount > 0 ? 'critical' : ($warningCount > 0 ? 'warning' : 'healthy');

        return [
            'service_name' => $serviceName,
            'overall_status' => $overallStatus,
            'metrics' => $metrics,
            'critical_count' => $criticalCount,
            'warning_count' => $warningCount,
        ];
    }
}

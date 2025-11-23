<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Cache;
use InvalidArgumentException;

/**
 * Security Event Model
 * 
 * Tracks security-related events for auditing, threat detection, and compliance.
 * Monitors login attempts, permission changes, suspicious activities, and policy violations.
 *
 * @property int $id
 * @property int|null $user_id User who triggered the event (null for anonymous)
 * @property string $type Event type (login_success, login_failed, etc.)
 * @property string $severity Severity level (critical, high, medium, low, info)
 * @property string $ip_address IP address of the request
 * @property string|null $user_agent User agent string
 * @property string|null $location Geographic location (city, country)
 * @property array $metadata Additional event context
 * @property string|null $session_id Session identifier
 * @property string|null $device_fingerprint Unique device identifier
 * @property bool $is_suspicious Flagged as suspicious activity
 * @property bool $is_blocked Action was blocked
 * @property string|null $blocked_reason Reason for blocking
 * @property string|null $threat_level Threat assessment (none, low, medium, high, critical)
 * @property float|null $risk_score Calculated risk score (0-100)
 * @property string|null $action_taken Action taken by system (none, blocked, rate_limited, mfa_required)
 * @property string|null $related_event_id Related security event ID
 * @property \Illuminate\Support\Carbon $created_at
 * @property \Illuminate\Support\Carbon $updated_at
 * @property-read \App\Models\User|null $user
 */
class SecurityEvent extends Model
{
    use HasFactory;

    /**
     * Event types - Authentication
     */
    public const TYPE_LOGIN_SUCCESS = 'login_success';
    public const TYPE_LOGIN_FAILED = 'login_failed';
    public const TYPE_LOGIN_BLOCKED = 'login_blocked';
    public const TYPE_LOGOUT = 'logout';
    public const TYPE_PASSWORD_RESET_REQUESTED = 'password_reset_requested';
    public const TYPE_PASSWORD_RESET_COMPLETED = 'password_reset_completed';
    public const TYPE_PASSWORD_CHANGED = 'password_changed';
    public const TYPE_MFA_ENABLED = 'mfa_enabled';
    public const TYPE_MFA_DISABLED = 'mfa_disabled';
    public const TYPE_MFA_VERIFIED = 'mfa_verified';
    public const TYPE_MFA_FAILED = 'mfa_failed';
    public const TYPE_SESSION_EXPIRED = 'session_expired';
    public const TYPE_SESSION_HIJACK_ATTEMPT = 'session_hijack_attempt';

    /**
     * Event types - Authorization
     */
    public const TYPE_PERMISSION_GRANTED = 'permission_granted';
    public const TYPE_PERMISSION_REVOKED = 'permission_revoked';
    public const TYPE_ROLE_ASSIGNED = 'role_assigned';
    public const TYPE_ROLE_REMOVED = 'role_removed';
    public const TYPE_ACCESS_DENIED = 'access_denied';
    public const TYPE_PRIVILEGE_ESCALATION_ATTEMPT = 'privilege_escalation_attempt';

    /**
     * Event types - Account Activity
     */
    public const TYPE_ACCOUNT_CREATED = 'account_created';
    public const TYPE_ACCOUNT_UPDATED = 'account_updated';
    public const TYPE_ACCOUNT_DELETED = 'account_deleted';
    public const TYPE_ACCOUNT_LOCKED = 'account_locked';
    public const TYPE_ACCOUNT_UNLOCKED = 'account_unlocked';
    public const TYPE_ACCOUNT_SUSPENDED = 'account_suspended';
    public const TYPE_ACCOUNT_REACTIVATED = 'account_reactivated';
    public const TYPE_EMAIL_CHANGED = 'email_changed';
    public const TYPE_EMAIL_VERIFIED = 'email_verified';

    /**
     * Event types - Suspicious Activity
     */
    public const TYPE_BRUTE_FORCE_ATTEMPT = 'brute_force_attempt';
    public const TYPE_RATE_LIMIT_EXCEEDED = 'rate_limit_exceeded';
    public const TYPE_SUSPICIOUS_LOCATION = 'suspicious_location';
    public const TYPE_SUSPICIOUS_DEVICE = 'suspicious_device';
    public const TYPE_SUSPICIOUS_IP = 'suspicious_ip';
    public const TYPE_SQL_INJECTION_ATTEMPT = 'sql_injection_attempt';
    public const TYPE_XSS_ATTEMPT = 'xss_attempt';
    public const TYPE_CSRF_ATTACK = 'csrf_attack';
    public const TYPE_MALICIOUS_FILE_UPLOAD = 'malicious_file_upload';
    public const TYPE_API_ABUSE = 'api_abuse';
    public const TYPE_BOT_DETECTED = 'bot_detected';

    /**
     * Event types - Data Access
     */
    public const TYPE_SENSITIVE_DATA_ACCESS = 'sensitive_data_access';
    public const TYPE_DATA_EXPORT = 'data_export';
    public const TYPE_BULK_DATA_ACCESS = 'bulk_data_access';
    public const TYPE_UNAUTHORIZED_DATA_ACCESS = 'unauthorized_data_access';

    /**
     * Event types - System
     */
    public const TYPE_SYSTEM_CONFIG_CHANGED = 'system_config_changed';
    public const TYPE_SECURITY_POLICY_VIOLATED = 'security_policy_violated';
    public const TYPE_AUDIT_LOG_TAMPER_ATTEMPT = 'audit_log_tamper_attempt';

    /**
     * Severity levels
     */
    public const SEVERITY_CRITICAL = 'critical';
    public const SEVERITY_HIGH = 'high';
    public const SEVERITY_MEDIUM = 'medium';
    public const SEVERITY_LOW = 'low';
    public const SEVERITY_INFO = 'info';

    /**
     * Threat levels
     */
    public const THREAT_NONE = 'none';
    public const THREAT_LOW = 'low';
    public const THREAT_MEDIUM = 'medium';
    public const THREAT_HIGH = 'high';
    public const THREAT_CRITICAL = 'critical';

    /**
     * Actions taken by system
     */
    public const ACTION_NONE = 'none';
    public const ACTION_BLOCKED = 'blocked';
    public const ACTION_RATE_LIMITED = 'rate_limited';
    public const ACTION_MFA_REQUIRED = 'mfa_required';
    public const ACTION_ACCOUNT_LOCKED = 'account_locked';
    public const ACTION_IP_BANNED = 'ip_banned';
    public const ACTION_SESSION_TERMINATED = 'session_terminated';
    public const ACTION_ALERT_SENT = 'alert_sent';

    /**
     * Valid event types
     */
    public const VALID_TYPES = [
        // Authentication
        self::TYPE_LOGIN_SUCCESS,
        self::TYPE_LOGIN_FAILED,
        self::TYPE_LOGIN_BLOCKED,
        self::TYPE_LOGOUT,
        self::TYPE_PASSWORD_RESET_REQUESTED,
        self::TYPE_PASSWORD_RESET_COMPLETED,
        self::TYPE_PASSWORD_CHANGED,
        self::TYPE_MFA_ENABLED,
        self::TYPE_MFA_DISABLED,
        self::TYPE_MFA_VERIFIED,
        self::TYPE_MFA_FAILED,
        self::TYPE_SESSION_EXPIRED,
        self::TYPE_SESSION_HIJACK_ATTEMPT,
        // Authorization
        self::TYPE_PERMISSION_GRANTED,
        self::TYPE_PERMISSION_REVOKED,
        self::TYPE_ROLE_ASSIGNED,
        self::TYPE_ROLE_REMOVED,
        self::TYPE_ACCESS_DENIED,
        self::TYPE_PRIVILEGE_ESCALATION_ATTEMPT,
        // Account Activity
        self::TYPE_ACCOUNT_CREATED,
        self::TYPE_ACCOUNT_UPDATED,
        self::TYPE_ACCOUNT_DELETED,
        self::TYPE_ACCOUNT_LOCKED,
        self::TYPE_ACCOUNT_UNLOCKED,
        self::TYPE_ACCOUNT_SUSPENDED,
        self::TYPE_ACCOUNT_REACTIVATED,
        self::TYPE_EMAIL_CHANGED,
        self::TYPE_EMAIL_VERIFIED,
        // Suspicious Activity
        self::TYPE_BRUTE_FORCE_ATTEMPT,
        self::TYPE_RATE_LIMIT_EXCEEDED,
        self::TYPE_SUSPICIOUS_LOCATION,
        self::TYPE_SUSPICIOUS_DEVICE,
        self::TYPE_SUSPICIOUS_IP,
        self::TYPE_SQL_INJECTION_ATTEMPT,
        self::TYPE_XSS_ATTEMPT,
        self::TYPE_CSRF_ATTACK,
        self::TYPE_MALICIOUS_FILE_UPLOAD,
        self::TYPE_API_ABUSE,
        self::TYPE_BOT_DETECTED,
        // Data Access
        self::TYPE_SENSITIVE_DATA_ACCESS,
        self::TYPE_DATA_EXPORT,
        self::TYPE_BULK_DATA_ACCESS,
        self::TYPE_UNAUTHORIZED_DATA_ACCESS,
        // System
        self::TYPE_SYSTEM_CONFIG_CHANGED,
        self::TYPE_SECURITY_POLICY_VIOLATED,
        self::TYPE_AUDIT_LOG_TAMPER_ATTEMPT,
    ];

    /**
     * Severity mapping by event type
     */
    protected const SEVERITY_MAP = [
        self::TYPE_LOGIN_SUCCESS => self::SEVERITY_INFO,
        self::TYPE_LOGIN_FAILED => self::SEVERITY_LOW,
        self::TYPE_LOGIN_BLOCKED => self::SEVERITY_MEDIUM,
        self::TYPE_BRUTE_FORCE_ATTEMPT => self::SEVERITY_HIGH,
        self::TYPE_SESSION_HIJACK_ATTEMPT => self::SEVERITY_CRITICAL,
        self::TYPE_PRIVILEGE_ESCALATION_ATTEMPT => self::SEVERITY_CRITICAL,
        self::TYPE_SQL_INJECTION_ATTEMPT => self::SEVERITY_CRITICAL,
        self::TYPE_XSS_ATTEMPT => self::SEVERITY_HIGH,
        self::TYPE_CSRF_ATTACK => self::SEVERITY_HIGH,
        self::TYPE_UNAUTHORIZED_DATA_ACCESS => self::SEVERITY_CRITICAL,
        self::TYPE_AUDIT_LOG_TAMPER_ATTEMPT => self::SEVERITY_CRITICAL,
    ];

    protected $fillable = [
        'user_id',
        'type',
        'severity',
        'ip_address',
        'user_agent',
        'location',
        'metadata',
        'session_id',
        'device_fingerprint',
        'is_suspicious',
        'is_blocked',
        'blocked_reason',
        'threat_level',
        'risk_score',
        'action_taken',
        'related_event_id',
    ];

    protected $casts = [
        'user_id' => 'integer',
        'metadata' => 'array',
        'is_suspicious' => 'boolean',
        'is_blocked' => 'boolean',
        'risk_score' => 'float',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
    ];

    protected $attributes = [
        'severity' => self::SEVERITY_INFO,
        'threat_level' => self::THREAT_NONE,
        'is_suspicious' => false,
        'is_blocked' => false,
        'risk_score' => 0.0,
        'action_taken' => self::ACTION_NONE,
        'metadata' => '[]',
    ];

    /**
     * Boot the model
     */
    protected static function boot(): void
    {
        parent::boot();

        static::creating(function (self $model): void {
            $model->validateType();
            $model->enrichEventData();
            $model->calculateRiskScore();
            $model->detectAnomalies();
        });
    }

    /**
     * Get the user who triggered this event
     */
    public function user(): BelongsTo
    {
        return $this->belongsTo(User::class);
    }

    /**
     * Validate event type
     */
    protected function validateType(): void
    {
        if (!in_array($this->type, self::VALID_TYPES, true)) {
            throw new InvalidArgumentException(
                sprintf('Invalid security event type: %s', $this->type)
            );
        }
    }

    /**
     * Enrich event data with additional context
     */
    protected function enrichEventData(): void
    {
        // Auto-set severity if not provided
        if (empty($this->severity)) {
            $this->severity = self::SEVERITY_MAP[$this->type] ?? self::SEVERITY_INFO;
        }

        // Parse user agent if available
        if ($this->user_agent && empty($this->metadata['parsed_user_agent'])) {
            $this->setMetadataValue('parsed_user_agent', $this->parseUserAgent($this->user_agent));
        }

        // Add timestamp
        $this->setMetadataValue('event_timestamp', now()->toISOString());
    }

    /**
     * Calculate risk score based on various factors
     */
    protected function calculateRiskScore(): void
    {
        $score = 0.0;

        // Base score by severity
        $score += match($this->severity) {
            self::SEVERITY_CRITICAL => 50.0,
            self::SEVERITY_HIGH => 30.0,
            self::SEVERITY_MEDIUM => 15.0,
            self::SEVERITY_LOW => 5.0,
            default => 0.0,
        };

        // Check for suspicious patterns
        if ($this->is_suspicious) {
            $score += 20.0;
        }

        // Check for known malicious IPs
        if ($this->isIpSuspicious()) {
            $score += 25.0;
        }

        // Check for unusual location
        if ($this->user_id && $this->isLocationAnomaly()) {
            $score += 15.0;
        }

        // Recent failed attempts
        if ($this->hasRecentFailedAttempts()) {
            $score += 20.0;
        }

        $this->risk_score = min($score, 100.0);
        $this->threat_level = $this->calculateThreatLevel($this->risk_score);
    }

    /**
     * Calculate threat level from risk score
     */
    protected function calculateThreatLevel(float $score): string
    {
        return match(true) {
            $score >= 80.0 => self::THREAT_CRITICAL,
            $score >= 60.0 => self::THREAT_HIGH,
            $score >= 40.0 => self::THREAT_MEDIUM,
            $score >= 20.0 => self::THREAT_LOW,
            default => self::THREAT_NONE,
        };
    }

    /**
     * Detect anomalies and suspicious patterns
     */
    protected function detectAnomalies(): void
    {
        // Check for brute force patterns
        if ($this->detectBruteForce()) {
            $this->is_suspicious = true;
            $this->setMetadataValue('anomaly', 'brute_force_pattern_detected');
        }

        // Check for impossible travel
        if ($this->detectImpossibleTravel()) {
            $this->is_suspicious = true;
            $this->setMetadataValue('anomaly', 'impossible_travel_detected');
        }

        // Check for device mismatch
        if ($this->detectDeviceMismatch()) {
            $this->is_suspicious = true;
            $this->setMetadataValue('anomaly', 'device_mismatch_detected');
        }
    }

    /**
     * Check if IP is suspicious
     */
    protected function isIpSuspicious(): bool
    {
        // Check against known malicious IPs (could use external service)
        $suspiciousIps = Cache::get('security:suspicious_ips', []);
        return in_array($this->ip_address, $suspiciousIps, true);
    }

    /**
     * Check for location anomaly
     */
    protected function isLocationAnomaly(): bool
    {
        if (!$this->user_id || !$this->location) {
            return false;
        }

        // Get user's typical locations
        $typicalLocations = Cache::remember(
            "security:user:{$this->user_id}:locations",
            3600,
            fn() => static::where('user_id', $this->user_id)
                ->whereNotNull('location')
                ->where('created_at', '>=', now()->subDays(30))
                ->pluck('location')
                ->unique()
                ->toArray()
        );

        return !empty($typicalLocations) && !in_array($this->location, $typicalLocations, true);
    }

    /**
     * Detect brute force patterns
     */
    protected function detectBruteForce(): bool
    {
        if (!in_array($this->type, [self::TYPE_LOGIN_FAILED, self::TYPE_MFA_FAILED], true)) {
            return false;
        }

        $recentAttempts = static::where('ip_address', $this->ip_address)
            ->whereIn('type', [self::TYPE_LOGIN_FAILED, self::TYPE_MFA_FAILED])
            ->where('created_at', '>=', now()->subMinutes(10))
            ->count();

        return $recentAttempts >= 5;
    }

    /**
     * Check for recent failed attempts
     */
    protected function hasRecentFailedAttempts(): bool
    {
        if (!$this->user_id) {
            return false;
        }

        return static::where('user_id', $this->user_id)
            ->whereIn('type', [self::TYPE_LOGIN_FAILED, self::TYPE_ACCESS_DENIED])
            ->where('created_at', '>=', now()->subHour())
            ->exists();
    }

    /**
     * Detect impossible travel
     */
    protected function detectImpossibleTravel(): bool
    {
        if (!$this->user_id || !$this->location) {
            return false;
        }

        $lastEvent = static::where('user_id', $this->user_id)
            ->whereNotNull('location')
            ->where('id', '!=', $this->id)
            ->latest()
            ->first();

        if (!$lastEvent || $lastEvent->location === $this->location) {
            return false;
        }

        // If locations are different and time gap is < 1 hour, flag as impossible
        $timeDiff = $lastEvent->created_at->diffInMinutes(now());
        return $timeDiff < 60;
    }

    /**
     * Detect device mismatch
     */
    protected function detectDeviceMismatch(): bool
    {
        if (!$this->user_id || !$this->device_fingerprint) {
            return false;
        }

        $knownDevices = Cache::remember(
            "security:user:{$this->user_id}:devices",
            3600,
            fn() => static::where('user_id', $this->user_id)
                ->whereNotNull('device_fingerprint')
                ->where('created_at', '>=', now()->subDays(30))
                ->pluck('device_fingerprint')
                ->unique()
                ->toArray()
        );

        return !empty($knownDevices) && !in_array($this->device_fingerprint, $knownDevices, true);
    }

    /**
     * Parse user agent string
     */
    protected function parseUserAgent(string $userAgent): array
    {
        // Simple parsing - in production, use a proper parser library
        return [
            'full' => $userAgent,
            'is_mobile' => str_contains(strtolower($userAgent), 'mobile'),
            'is_bot' => str_contains(strtolower($userAgent), 'bot'),
        ];
    }

    /**
     * Get metadata value
     */
    public function getMetadataValue(string $key, mixed $default = null): mixed
    {
        return $this->metadata[$key] ?? $default;
    }

    /**
     * Set metadata value
     */
    public function setMetadataValue(string $key, mixed $value): self
    {
        $metadata = $this->metadata;
        $metadata[$key] = $value;
        $this->metadata = $metadata;

        return $this;
    }

    /**
     * Check if event is critical
     */
    public function isCritical(): bool
    {
        return $this->severity === self::SEVERITY_CRITICAL;
    }

    /**
     * Check if event is high severity
     */
    public function isHighSeverity(): bool
    {
        return in_array($this->severity, [self::SEVERITY_CRITICAL, self::SEVERITY_HIGH], true);
    }

    /**
     * Check if event indicates attack
     */
    public function isAttack(): bool
    {
        return in_array($this->type, [
            self::TYPE_BRUTE_FORCE_ATTEMPT,
            self::TYPE_SESSION_HIJACK_ATTEMPT,
            self::TYPE_PRIVILEGE_ESCALATION_ATTEMPT,
            self::TYPE_SQL_INJECTION_ATTEMPT,
            self::TYPE_XSS_ATTEMPT,
            self::TYPE_CSRF_ATTACK,
            self::TYPE_MALICIOUS_FILE_UPLOAD,
        ], true);
    }

    /**
     * Check if event is authentication-related
     */
    public function isAuthenticationEvent(): bool
    {
        return str_starts_with($this->type, 'login_')
            || str_starts_with($this->type, 'mfa_')
            || str_starts_with($this->type, 'password_');
    }

    /**
     * Check if event is authorization-related
     */
    public function isAuthorizationEvent(): bool
    {
        return str_contains($this->type, 'permission_')
            || str_contains($this->type, 'role_')
            || str_contains($this->type, 'access_');
    }

    /**
     * Get severity color for UI
     */
    public function getSeverityColorAttribute(): string
    {
        return match($this->severity) {
            self::SEVERITY_CRITICAL => '#dc2626', // red-600
            self::SEVERITY_HIGH => '#ea580c', // orange-600
            self::SEVERITY_MEDIUM => '#f59e0b', // amber-500
            self::SEVERITY_LOW => '#3b82f6', // blue-500
            self::SEVERITY_INFO => '#6b7280', // gray-500
        };
    }

    /**
     * Get threat level color for UI
     */
    public function getThreatLevelColorAttribute(): string
    {
        return match($this->threat_level) {
            self::THREAT_CRITICAL => '#7f1d1d', // red-900
            self::THREAT_HIGH => '#dc2626', // red-600
            self::THREAT_MEDIUM => '#f59e0b', // amber-500
            self::THREAT_LOW => '#3b82f6', // blue-500
            default => '#6b7280', // gray-500
        };
    }

    /**
     * Get human-readable type label
     */
    public function getTypeLabelAttribute(): string
    {
        return ucwords(str_replace('_', ' ', $this->type));
    }

    /**
     * Scope: Filter by type
     */
    public function scopeOfType(Builder $query, string $type): Builder
    {
        return $query->where('type', $type);
    }

    /**
     * Scope: Filter by severity
     */
    public function scopeOfSeverity(Builder $query, string $severity): Builder
    {
        return $query->where('severity', $severity);
    }

    /**
     * Scope: Get suspicious events
     */
    public function scopeSuspicious(Builder $query): Builder
    {
        return $query->where('is_suspicious', true);
    }

    /**
     * Scope: Get blocked events
     */
    public function scopeBlocked(Builder $query): Builder
    {
        return $query->where('is_blocked', true);
    }

    /**
     * Scope: Get critical events
     */
    public function scopeCritical(Builder $query): Builder
    {
        return $query->where('severity', self::SEVERITY_CRITICAL);
    }

    /**
     * Scope: Get high severity events
     */
    public function scopeHighSeverity(Builder $query): Builder
    {
        return $query->whereIn('severity', [self::SEVERITY_CRITICAL, self::SEVERITY_HIGH]);
    }

    /**
     * Scope: Get attack events
     */
    public function scopeAttacks(Builder $query): Builder
    {
        return $query->whereIn('type', [
            self::TYPE_BRUTE_FORCE_ATTEMPT,
            self::TYPE_SESSION_HIJACK_ATTEMPT,
            self::TYPE_PRIVILEGE_ESCALATION_ATTEMPT,
            self::TYPE_SQL_INJECTION_ATTEMPT,
            self::TYPE_XSS_ATTEMPT,
            self::TYPE_CSRF_ATTACK,
            self::TYPE_MALICIOUS_FILE_UPLOAD,
        ]);
    }

    /**
     * Scope: Get authentication events
     */
    public function scopeAuthentication(Builder $query): Builder
    {
        return $query->where(function($q) {
            $q->where('type', 'LIKE', 'login_%')
              ->orWhere('type', 'LIKE', 'mfa_%')
              ->orWhere('type', 'LIKE', 'password_%');
        });
    }

    /**
     * Scope: Get failed login attempts
     */
    public function scopeFailedLogins(Builder $query): Builder
    {
        return $query->whereIn('type', [
            self::TYPE_LOGIN_FAILED,
            self::TYPE_LOGIN_BLOCKED,
        ]);
    }

    /**
     * Scope: Filter by IP address
     */
    public function scopeFromIp(Builder $query, string $ip): Builder
    {
        return $query->where('ip_address', $ip);
    }

    /**
     * Scope: Filter by user
     */
    public function scopeForUser(Builder $query, int $userId): Builder
    {
        return $query->where('user_id', $userId);
    }

    /**
     * Scope: Recent events (within hours)
     */
    public function scopeRecent(Builder $query, int $hours = 24): Builder
    {
        return $query->where('created_at', '>=', now()->subHours($hours));
    }

    /**
     * Scope: High risk events
     */
    public function scopeHighRisk(Builder $query, float $threshold = 60.0): Builder
    {
        return $query->where('risk_score', '>=', $threshold);
    }

    /**
     * Scope: Order by risk score
     */
    public function scopeOrderByRisk(Builder $query, string $direction = 'desc'): Builder
    {
        return $query->orderBy('risk_score', $direction);
    }

    /**
     * Static: Log security event
     */
    public static function log(array $attributes): self
    {
        // Merge with request context if not provided
        $attributes = array_merge([
            'ip_address' => request()->ip(),
            'user_agent' => request()->userAgent(),
            'session_id' => session()->getId(),
        ], $attributes);

        return static::create($attributes);
    }

    /**
     * Static: Get security dashboard summary
     */
    public static function getDashboardSummary(int $hours = 24): array
    {
        $recent = static::recent($hours);

        return [
            'total_events' => $recent->count(),
            'critical_events' => $recent->clone()->critical()->count(),
            'suspicious_events' => $recent->clone()->suspicious()->count(),
            'blocked_events' => $recent->clone()->blocked()->count(),
            'attack_attempts' => $recent->clone()->attacks()->count(),
            'failed_logins' => $recent->clone()->failedLogins()->count(),
            'high_risk_events' => $recent->clone()->highRisk()->count(),
            'unique_ips' => $recent->clone()->distinct('ip_address')->count('ip_address'),
            'affected_users' => $recent->clone()->whereNotNull('user_id')->distinct('user_id')->count('user_id'),
        ];
    }

    /**
     * Static: Get events by type grouped
     */
    public static function getGroupedByType(int $hours = 24): Collection
    {
        return static::recent($hours)
            ->get()
            ->groupBy('type')
            ->map(fn($events) => [
                'count' => $events->count(),
                'suspicious' => $events->where('is_suspicious', true)->count(),
                'blocked' => $events->where('is_blocked', true)->count(),
                'avg_risk_score' => $events->avg('risk_score'),
            ])
            ->sortByDesc('count');
    }

    /**
     * Static: Get top threatening IPs
     */
    public static function getTopThreateningIps(int $limit = 10, int $hours = 24): Collection
    {
        return static::recent($hours)
            ->suspicious()
            ->get()
            ->groupBy('ip_address')
            ->map(fn($events) => [
                'ip' => $events->first()->ip_address,
                'event_count' => $events->count(),
                'attack_count' => $events->filter(fn($e) => $e->isAttack())->count(),
                'avg_risk_score' => round($events->avg('risk_score'), 2),
                'max_risk_score' => $events->max('risk_score'),
                'latest_event' => $events->sortByDesc('created_at')->first(),
            ])
            ->sortByDesc('max_risk_score')
            ->take($limit)
            ->values();
    }

    /**
     * Static: Get user security timeline
     */
    public static function getUserTimeline(int $userId, int $days = 30): Collection
    {
        return static::forUser($userId)
            ->where('created_at', '>=', now()->subDays($days))
            ->orderByDesc('created_at')
            ->get()
            ->map(fn($event) => [
                'id' => $event->id,
                'type' => $event->type,
                'type_label' => $event->type_label,
                'severity' => $event->severity,
                'is_suspicious' => $event->is_suspicious,
                'risk_score' => $event->risk_score,
                'ip_address' => $event->ip_address,
                'location' => $event->location,
                'created_at' => $event->created_at->toISOString(),
            ]);
    }

    /**
     * Static: Detect anomalies for user
     */
    public static function detectUserAnomalies(int $userId): array
    {
        $events = static::forUser($userId)->recent(168)->get(); // 7 days

        $anomalies = [];

        // Check for unusual login times
        $loginHours = $events->where('type', self::TYPE_LOGIN_SUCCESS)
            ->pluck('created_at')
            ->map(fn($dt) => $dt->hour)
            ->unique();

        if ($loginHours->count() > 0) {
            $avgHour = $loginHours->avg();
            $latestLogin = $events->where('type', self::TYPE_LOGIN_SUCCESS)->first();
            
            if ($latestLogin && abs($latestLogin->created_at->hour - $avgHour) > 6) {
                $anomalies[] = 'unusual_login_time';
            }
        }

        // Check for multiple locations
        $locations = $events->whereNotNull('location')->pluck('location')->unique();
        if ($locations->count() > 3) {
            $anomalies[] = 'multiple_locations';
        }

        // Check for failed attempts
        if ($events->whereIn('type', [self::TYPE_LOGIN_FAILED, self::TYPE_ACCESS_DENIED])->count() > 5) {
            $anomalies[] = 'multiple_failed_attempts';
        }

        return $anomalies;
    }

    /**
     * Export to array for API
     */
    public function toSecurityEventArray(): array
    {
        return [
            'id' => $this->id,
            'user_id' => $this->user_id,
            'type' => $this->type,
            'type_label' => $this->type_label,
            'severity' => $this->severity,
            'severity_color' => $this->severity_color,
            'threat_level' => $this->threat_level,
            'threat_level_color' => $this->threat_level_color,
            'risk_score' => $this->risk_score,
            'is_suspicious' => $this->is_suspicious,
            'is_blocked' => $this->is_blocked,
            'is_attack' => $this->isAttack(),
            'ip_address' => $this->ip_address,
            'location' => $this->location,
            'action_taken' => $this->action_taken,
            'metadata' => $this->metadata,
            'created_at' => $this->created_at->toISOString(),
        ];
    }
}
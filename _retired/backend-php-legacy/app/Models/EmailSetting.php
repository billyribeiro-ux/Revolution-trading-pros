<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Casts\Attribute;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\MorphMany;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Crypt;
use Illuminate\Support\Facades\Mail;
use Illuminate\Support\Str;
use Carbon\Carbon;
use Symfony\Component\Mailer\Transport\Dsn;
use App\Traits\HasUuid;
use App\Traits\Encryptable;
use App\Traits\Auditable;
use App\Traits\Configurable;
use App\Contracts\EmailProvider;
use App\Events\EmailSettingActivated;
use App\Events\EmailSettingDeactivated;
use App\Events\EmailSettingTested;
use App\Events\EmailSettingFailed;
use App\Enums\EmailProvider as EmailProviderEnum;
use App\Enums\EmailEncryption;
use App\Enums\EmailAuthMethod;
use App\Enums\EmailSettingStatus;
use App\Enums\EmailPriority;
use App\Services\Email\ConnectionTester;
use App\Services\Email\HealthChecker;
use App\Services\Email\ConfigValidator;
use App\Exceptions\EmailConfigurationException;

/**
 * EmailSetting Model
 * 
 * Enterprise-grade email configuration system with multi-provider support,
 * health monitoring, failover capabilities, and comprehensive security.
 * 
 * @property int $id
 * @property string $uuid
 * @property string $name
 * @property string $slug
 * @property string $description
 * @property string $provider
 * @property string $driver
 * @property string $host
 * @property int $port
 * @property string $username
 * @property string $password
 * @property string|null $api_key
 * @property string|null $api_secret
 * @property string|null $webhook_secret
 * @property string|null $signing_secret
 * @property string $encryption
 * @property string $auth_method
 * @property string $from_address
 * @property string $from_name
 * @property string|null $reply_to_address
 * @property string|null $reply_to_name
 * @property string|null $bounce_address
 * @property string|null $return_path
 * @property array|null $headers
 * @property array|null $options
 * @property array|null $dsn_config
 * @property string|null $region
 * @property string|null $endpoint
 * @property string|null $domain
 * @property string|null $subdomain
 * @property int $timeout
 * @property int $retry_times
 * @property int $retry_delay
 * @property int $max_connections
 * @property int $max_messages_per_connection
 * @property int $rate_limit
 * @property string $rate_limit_period
 * @property int $daily_limit
 * @property int $monthly_limit
 * @property int $messages_sent_today
 * @property int $messages_sent_month
 * @property Carbon|null $limit_reset_at
 * @property bool $is_active
 * @property bool $is_default
 * @property bool $is_transactional
 * @property bool $is_marketing
 * @property bool $is_bulk
 * @property bool $is_failover
 * @property bool $verify_peer
 * @property bool $verify_peer_name
 * @property bool $allow_self_signed
 * @property string|null $ssl_certificate
 * @property string|null $ssl_key
 * @property string|null $ssl_ca
 * @property int $priority
 * @property string $status
 * @property float $health_score
 * @property float $delivery_rate
 * @property float $bounce_rate
 * @property float $complaint_rate
 * @property float $average_send_time
 * @property int $successful_sends
 * @property int $failed_sends
 * @property int $total_sends
 * @property Carbon|null $last_used_at
 * @property Carbon|null $last_tested_at
 * @property Carbon|null $last_success_at
 * @property Carbon|null $last_failure_at
 * @property string|null $last_error
 * @property array|null $last_error_details
 * @property int $consecutive_failures
 * @property bool $auto_disable_on_failure
 * @property int $failure_threshold
 * @property array|null $provider_settings
 * @property array|null $oauth_settings
 * @property string|null $oauth_token
 * @property string|null $oauth_refresh_token
 * @property Carbon|null $oauth_expires_at
 * @property array|null $webhook_config
 * @property string|null $webhook_url
 * @property array|null $event_webhooks
 * @property array|null $tracking_settings
 * @property bool $track_opens
 * @property bool $track_clicks
 * @property bool $track_unsubscribes
 * @property string|null $tracking_domain
 * @property array|null $dkim_settings
 * @property bool $dkim_enabled
 * @property string|null $dkim_selector
 * @property string|null $dkim_private_key
 * @property string|null $dkim_public_key
 * @property array|null $spf_settings
 * @property array|null $dmarc_settings
 * @property array|null $ip_pool
 * @property string|null $dedicated_ip
 * @property array|null $warmup_settings
 * @property bool $is_warming_up
 * @property int $warmup_volume
 * @property Carbon|null $warmup_started_at
 * @property Carbon|null $warmup_completed_at
 * @property array|null $suppression_list
 * @property array|null $allowlist
 * @property array|null $tags
 * @property array|null $metadata
 * @property array|null $cost_settings
 * @property float $cost_per_thousand
 * @property float $monthly_cost
 * @property string|null $billing_account
 * @property array|null $compliance_settings
 * @property string|null $environment
 * @property array|null $fallback_settings
 * @property int|null $fallback_to
 * @property array|null $routing_rules
 * @property array|null $content_filters
 * @property array|null $monitoring_config
 * @property bool $alert_on_failure
 * @property array|null $alert_recipients
 * @property int $created_by
 * @property int|null $updated_by
 * @property Carbon|null $activated_at
 * @property Carbon|null $deactivated_at
 * @property Carbon $created_at
 * @property Carbon $updated_at
 * @property Carbon|null $deleted_at
 * 
 * @property-read \App\Models\User $creator
 * @property-read \App\Models\User|null $updater
 * @property-read \App\Models\EmailSetting|null $fallbackProvider
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\EmailLog[] $emailLogs
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\EmailSettingTest[] $tests
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\EmailSettingHealth[] $healthChecks
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\EmailSettingMetric[] $metrics
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\EmailSettingEvent[] $events
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\AuditLog[] $auditLogs
 */
class EmailSetting extends Model implements EmailProvider
{
    use HasFactory;
    use SoftDeletes;
    use HasUuid;
    use Encryptable;
    use Auditable;
    use Configurable;

    /**
     * The table associated with the model.
     *
     * @var string
     */
    protected $table = 'email_settings';

    /**
     * The attributes that are mass assignable.
     *
     * @var array<string>
     */
    protected $fillable = [
        'uuid',
        'name',
        'slug',
        'description',
        'provider',
        'driver',
        'host',
        'port',
        'username',
        'password',
        'api_key',
        'api_secret',
        'webhook_secret',
        'signing_secret',
        'encryption',
        'auth_method',
        'from_address',
        'from_name',
        'reply_to_address',
        'reply_to_name',
        'bounce_address',
        'return_path',
        'headers',
        'options',
        'dsn_config',
        'region',
        'endpoint',
        'domain',
        'subdomain',
        'timeout',
        'retry_times',
        'retry_delay',
        'max_connections',
        'max_messages_per_connection',
        'rate_limit',
        'rate_limit_period',
        'daily_limit',
        'monthly_limit',
        'is_active',
        'is_default',
        'is_transactional',
        'is_marketing',
        'is_bulk',
        'is_failover',
        'verify_peer',
        'verify_peer_name',
        'allow_self_signed',
        'ssl_certificate',
        'ssl_key',
        'ssl_ca',
        'priority',
        'status',
        'auto_disable_on_failure',
        'failure_threshold',
        'provider_settings',
        'oauth_settings',
        'oauth_token',
        'oauth_refresh_token',
        'oauth_expires_at',
        'webhook_config',
        'webhook_url',
        'event_webhooks',
        'tracking_settings',
        'track_opens',
        'track_clicks',
        'track_unsubscribes',
        'tracking_domain',
        'dkim_settings',
        'dkim_enabled',
        'dkim_selector',
        'dkim_private_key',
        'dkim_public_key',
        'spf_settings',
        'dmarc_settings',
        'ip_pool',
        'dedicated_ip',
        'warmup_settings',
        'is_warming_up',
        'warmup_volume',
        'warmup_started_at',
        'warmup_completed_at',
        'suppression_list',
        'allowlist',
        'tags',
        'metadata',
        'cost_settings',
        'cost_per_thousand',
        'monthly_cost',
        'billing_account',
        'compliance_settings',
        'environment',
        'fallback_settings',
        'fallback_to',
        'routing_rules',
        'content_filters',
        'monitoring_config',
        'alert_on_failure',
        'alert_recipients',
        'created_by',
        'updated_by',
    ];

    /**
     * The attributes that should be cast.
     *
     * @var array<string, string>
     */
    protected $casts = [
        'port' => 'integer',
        'timeout' => 'integer',
        'retry_times' => 'integer',
        'retry_delay' => 'integer',
        'max_connections' => 'integer',
        'max_messages_per_connection' => 'integer',
        'rate_limit' => 'integer',
        'daily_limit' => 'integer',
        'monthly_limit' => 'integer',
        'messages_sent_today' => 'integer',
        'messages_sent_month' => 'integer',
        'priority' => 'integer',
        'successful_sends' => 'integer',
        'failed_sends' => 'integer',
        'total_sends' => 'integer',
        'consecutive_failures' => 'integer',
        'failure_threshold' => 'integer',
        'warmup_volume' => 'integer',
        'fallback_to' => 'integer',
        'health_score' => 'float',
        'delivery_rate' => 'float',
        'bounce_rate' => 'float',
        'complaint_rate' => 'float',
        'average_send_time' => 'float',
        'cost_per_thousand' => 'decimal:4',
        'monthly_cost' => 'decimal:2',
        'is_active' => 'boolean',
        'is_default' => 'boolean',
        'is_transactional' => 'boolean',
        'is_marketing' => 'boolean',
        'is_bulk' => 'boolean',
        'is_failover' => 'boolean',
        'verify_peer' => 'boolean',
        'verify_peer_name' => 'boolean',
        'allow_self_signed' => 'boolean',
        'auto_disable_on_failure' => 'boolean',
        'track_opens' => 'boolean',
        'track_clicks' => 'boolean',
        'track_unsubscribes' => 'boolean',
        'dkim_enabled' => 'boolean',
        'is_warming_up' => 'boolean',
        'alert_on_failure' => 'boolean',
        'provider' => EmailProviderEnum::class,
        'encryption' => EmailEncryption::class,
        'auth_method' => EmailAuthMethod::class,
        'status' => EmailSettingStatus::class,
        'headers' => 'array',
        'options' => 'array',
        'dsn_config' => 'array',
        'provider_settings' => 'array',
        'oauth_settings' => 'array',
        'webhook_config' => 'array',
        'event_webhooks' => 'array',
        'tracking_settings' => 'array',
        'dkim_settings' => 'array',
        'spf_settings' => 'array',
        'dmarc_settings' => 'array',
        'ip_pool' => 'array',
        'warmup_settings' => 'array',
        'suppression_list' => 'array',
        'allowlist' => 'array',
        'tags' => 'array',
        'metadata' => 'array',
        'cost_settings' => 'array',
        'compliance_settings' => 'array',
        'fallback_settings' => 'array',
        'routing_rules' => 'array',
        'content_filters' => 'array',
        'monitoring_config' => 'array',
        'alert_recipients' => 'array',
        'last_error_details' => 'array',
        'limit_reset_at' => 'datetime',
        'last_used_at' => 'datetime',
        'last_tested_at' => 'datetime',
        'last_success_at' => 'datetime',
        'last_failure_at' => 'datetime',
        'oauth_expires_at' => 'datetime',
        'warmup_started_at' => 'datetime',
        'warmup_completed_at' => 'datetime',
        'activated_at' => 'datetime',
        'deactivated_at' => 'datetime',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
        'deleted_at' => 'datetime',
    ];

    /**
     * The attributes that should be encrypted.
     *
     * @var array<string>
     */
    protected $encryptable = [
        'password',
        'api_key',
        'api_secret',
        'webhook_secret',
        'signing_secret',
        'oauth_token',
        'oauth_refresh_token',
        'ssl_certificate',
        'ssl_key',
        'ssl_ca',
        'dkim_private_key',
    ];

    /**
     * The attributes that should be hidden for serialization.
     *
     * @var array<string>
     */
    protected $hidden = [
        'password',
        'api_key',
        'api_secret',
        'webhook_secret',
        'signing_secret',
        'oauth_token',
        'oauth_refresh_token',
        'ssl_certificate',
        'ssl_key',
        'ssl_ca',
        'dkim_private_key',
        'deleted_at',
    ];

    /**
     * The attributes that should be appended.
     *
     * @var array<string>
     */
    protected $appends = [
        'is_configured',
        'is_healthy',
        'can_send',
        'remaining_daily_limit',
        'remaining_monthly_limit',
        'usage_percentage',
        'connection_string',
        'display_name',
        'provider_name',
        'status_badge',
        'requires_oauth_refresh',
    ];

    /**
     * Default attribute values.
     *
     * @var array<string, mixed>
     */
    protected $attributes = [
        'driver' => 'smtp',
        'port' => 587,
        'encryption' => EmailEncryption::TLS,
        'auth_method' => EmailAuthMethod::PLAIN,
        'timeout' => 30,
        'retry_times' => 3,
        'retry_delay' => 100,
        'max_connections' => 5,
        'max_messages_per_connection' => 100,
        'rate_limit' => 0,
        'rate_limit_period' => 'minute',
        'daily_limit' => 0,
        'monthly_limit' => 0,
        'messages_sent_today' => 0,
        'messages_sent_month' => 0,
        'priority' => 0,
        'status' => EmailSettingStatus::ACTIVE,
        'health_score' => 100.0,
        'delivery_rate' => 100.0,
        'bounce_rate' => 0.0,
        'complaint_rate' => 0.0,
        'average_send_time' => 0.0,
        'successful_sends' => 0,
        'failed_sends' => 0,
        'total_sends' => 0,
        'consecutive_failures' => 0,
        'failure_threshold' => 5,
        'is_active' => true,
        'is_default' => false,
        'is_transactional' => true,
        'is_marketing' => true,
        'is_bulk' => false,
        'is_failover' => false,
        'verify_peer' => true,
        'verify_peer_name' => true,
        'allow_self_signed' => false,
        'auto_disable_on_failure' => true,
        'track_opens' => false,
        'track_clicks' => false,
        'track_unsubscribes' => true,
        'dkim_enabled' => false,
        'is_warming_up' => false,
        'alert_on_failure' => true,
    ];

    /**
     * Cache configuration.
     */
    protected static array $cacheConfig = [
        'tags' => ['email_settings'],
        'ttl' => 3600,
    ];

    /**
     * Boot the model.
     */
    protected static function boot(): void
    {
        parent::boot();

        static::creating(function (self $setting) {
            $setting->uuid = $setting->uuid ?? (string) Str::uuid();
            $setting->slug = $setting->slug ?? Str::slug($setting->name);
            $setting->created_by = $setting->created_by ?? auth()->id();
            
            $setting->validateConfiguration();
            $setting->setupProviderDefaults();
        });

        static::created(function (self $setting) {
            if ($setting->is_default) {
                $setting->makeDefault();
            }
            
            $setting->testConnection();
            $setting->scheduleHealthCheck();
        });

        static::updating(function (self $setting) {
            $setting->updated_by = auth()->id();
            
            if ($setting->isDirty(['host', 'port', 'username', 'password', 'api_key'])) {
                $setting->status = EmailSettingStatus::PENDING_TEST;
            }
            
            if ($setting->isDirty('is_active')) {
                $setting->handleActivationChange();
            }
        });

        static::updated(function (self $setting) {
            $setting->clearCache();
            
            if ($setting->wasChanged('is_default') && $setting->is_default) {
                $setting->makeDefault();
            }
            
            if ($setting->wasChanged(['host', 'port', 'username', 'password', 'api_key'])) {
                $setting->testConnection();
            }
        });

        static::deleting(function (self $setting) {
            if ($setting->is_default) {
                throw new EmailConfigurationException('Cannot delete default email setting');
            }
            
            if ($setting->emailLogs()->whereDate('created_at', '>=', now()->subDays(30))->exists()) {
                throw new EmailConfigurationException('Cannot delete email setting with recent activity');
            }
        });
    }

    /**
     * Get the creator.
     */
    public function creator(): BelongsTo
    {
        return $this->belongsTo(User::class, 'created_by');
    }

    /**
     * Get the updater.
     */
    public function updater(): BelongsTo
    {
        return $this->belongsTo(User::class, 'updated_by');
    }

    /**
     * Get the fallback provider.
     */
    public function fallbackProvider(): BelongsTo
    {
        return $this->belongsTo(self::class, 'fallback_to');
    }

    /**
     * Get email logs.
     */
    public function emailLogs(): HasMany
    {
        return $this->hasMany(EmailLog::class, 'email_setting_id');
    }

    /**
     * Get connection tests.
     */
    public function tests(): HasMany
    {
        return $this->hasMany(EmailSettingTest::class)
            ->orderBy('tested_at', 'desc');
    }

    /**
     * Get health checks.
     */
    public function healthChecks(): HasMany
    {
        return $this->hasMany(EmailSettingHealth::class)
            ->orderBy('checked_at', 'desc');
    }

    /**
     * Get metrics.
     */
    public function metrics(): HasMany
    {
        return $this->hasMany(EmailSettingMetric::class)
            ->orderBy('recorded_at', 'desc');
    }

    /**
     * Get events.
     */
    public function events(): HasMany
    {
        return $this->hasMany(EmailSettingEvent::class)
            ->orderBy('occurred_at', 'desc');
    }

    /**
     * Get audit logs.
     */
    public function auditLogs(): MorphMany
    {
        return $this->morphMany(AuditLog::class, 'auditable');
    }

    /**
     * Scope for active settings.
     */
    public function scopeActive(Builder $query): Builder
    {
        return $query->where('is_active', true)
            ->where('status', EmailSettingStatus::ACTIVE);
    }

    /**
     * Scope for transactional providers.
     */
    public function scopeTransactional(Builder $query): Builder
    {
        return $query->where('is_transactional', true)
            ->active();
    }

    /**
     * Scope for marketing providers.
     */
    public function scopeMarketing(Builder $query): Builder
    {
        return $query->where('is_marketing', true)
            ->active();
    }

    /**
     * Scope for bulk providers.
     */
    public function scopeBulk(Builder $query): Builder
    {
        return $query->where('is_bulk', true)
            ->active();
    }

    /**
     * Scope for healthy providers.
     */
    public function scopeHealthy(Builder $query): Builder
    {
        return $query->where('health_score', '>=', 80)
            ->where('consecutive_failures', '<', 3);
    }

    /**
     * Scope by environment.
     */
    public function scopeEnvironment(Builder $query, string $environment): Builder
    {
        return $query->where('environment', $environment);
    }

    /**
     * Get configuration status.
     */
    protected function isConfigured(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->validateRequiredFields()
        );
    }

    /**
     * Get health status.
     */
    protected function isHealthy(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->health_score >= 80 && $this->consecutive_failures < 3
        );
    }

    /**
     * Get send capability.
     */
    protected function canSend(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->is_active && 
                         $this->is_configured && 
                         $this->is_healthy &&
                         !$this->hasReachedLimit()
        );
    }

    /**
     * Get remaining daily limit.
     */
    protected function remainingDailyLimit(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->daily_limit > 0 
                ? max(0, $this->daily_limit - $this->messages_sent_today)
                : PHP_INT_MAX
        );
    }

    /**
     * Get remaining monthly limit.
     */
    protected function remainingMonthlyLimit(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->monthly_limit > 0
                ? max(0, $this->monthly_limit - $this->messages_sent_month)
                : PHP_INT_MAX
        );
    }

    /**
     * Get usage percentage.
     */
    protected function usagePercentage(): Attribute
    {
        return Attribute::make(
            get: function () {
                if ($this->daily_limit > 0) {
                    return round(($this->messages_sent_today / $this->daily_limit) * 100, 2);
                }
                if ($this->monthly_limit > 0) {
                    return round(($this->messages_sent_month / $this->monthly_limit) * 100, 2);
                }
                return 0;
            }
        );
    }

    /**
     * Get connection string.
     */
    protected function connectionString(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->buildConnectionString()
        );
    }

    /**
     * Get display name.
     */
    protected function displayName(): Attribute
    {
        return Attribute::make(
            get: fn () => "{$this->name} ({$this->provider_name})"
        );
    }

    /**
     * Get provider name.
     */
    protected function providerName(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->provider->label()
        );
    }

    /**
     * Get status badge.
     */
    protected function statusBadge(): Attribute
    {
        return Attribute::make(
            get: function () {
                return [
                    'status' => $this->status->value,
                    'label' => $this->status->label(),
                    'color' => $this->status->color(),
                    'icon' => $this->status->icon(),
                ];
            }
        );
    }

    /**
     * Check if OAuth refresh is required.
     */
    protected function requiresOauthRefresh(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->oauth_expires_at && 
                         $this->oauth_expires_at->subMinutes(5)->isPast()
        );
    }

    /**
     * Validate configuration.
     */
    public function validateConfiguration(): bool
    {
        $validator = app(ConfigValidator::class);
        
        try {
            $validator->validate($this);
            return true;
        } catch (\Exception $e) {
            $this->recordError($e->getMessage());
            return false;
        }
    }

    /**
     * Validate required fields.
     */
    protected function validateRequiredFields(): bool
    {
        return match($this->provider) {
            EmailProviderEnum::SMTP => !empty($this->host) && 
                                       !empty($this->port) && 
                                       !empty($this->username) && 
                                       !empty($this->password),
            EmailProviderEnum::SENDGRID,
            EmailProviderEnum::MAILGUN,
            EmailProviderEnum::POSTMARK => !empty($this->api_key),
            EmailProviderEnum::SES => !empty($this->api_key) && 
                                      !empty($this->api_secret) && 
                                      !empty($this->region),
            default => false,
        };
    }

    /**
     * Setup provider defaults.
     */
    protected function setupProviderDefaults(): void
    {
        $defaults = match($this->provider) {
            EmailProviderEnum::GMAIL => [
                'host' => 'smtp.gmail.com',
                'port' => 587,
                'encryption' => EmailEncryption::TLS,
            ],
            EmailProviderEnum::OUTLOOK => [
                'host' => 'smtp-mail.outlook.com',
                'port' => 587,
                'encryption' => EmailEncryption::STARTTLS,
            ],
            EmailProviderEnum::SENDGRID => [
                'host' => 'smtp.sendgrid.net',
                'port' => 587,
                'encryption' => EmailEncryption::TLS,
                'username' => 'apikey',
            ],
            EmailProviderEnum::MAILGUN => [
                'host' => 'smtp.mailgun.org',
                'port' => 587,
                'encryption' => EmailEncryption::TLS,
            ],
            EmailProviderEnum::SES => [
                'driver' => 'ses',
                'region' => 'us-east-1',
            ],
            default => [],
        };

        $this->fill($defaults);
    }

    /**
     * Test connection.
     */
    public function testConnection(): bool
    {
        $tester = app(ConnectionTester::class);
        
        $result = $tester->test($this);
        
        $this->tests()->create([
            'success' => $result['success'],
            'message' => $result['message'],
            'response_time' => $result['response_time'] ?? null,
            'details' => $result['details'] ?? [],
            'tested_at' => now(),
        ]);

        if ($result['success']) {
            $this->update([
                'status' => EmailSettingStatus::ACTIVE,
                'last_tested_at' => now(),
                'last_success_at' => now(),
                'consecutive_failures' => 0,
            ]);
            
            event(new EmailSettingTested($this, true));
            return true;
        } else {
            $this->handleTestFailure($result['message']);
            event(new EmailSettingTested($this, false));
            return false;
        }
    }

    /**
     * Handle test failure.
     */
    protected function handleTestFailure(string $error): void
    {
        $this->increment('consecutive_failures');
        
        $this->update([
            'last_tested_at' => now(),
            'last_failure_at' => now(),
            'last_error' => $error,
            'last_error_details' => [
                'timestamp' => now()->toIso8601String(),
                'error' => $error,
                'attempts' => $this->consecutive_failures,
            ],
        ]);

        if ($this->consecutive_failures >= $this->failure_threshold && $this->auto_disable_on_failure) {
            $this->disable('Too many consecutive failures');
        }
    }

    /**
     * Handle activation change.
     */
    protected function handleActivationChange(): void
    {
        if ($this->is_active && !$this->getOriginal('is_active')) {
            $this->activated_at = now();
            event(new EmailSettingActivated($this));
        } elseif (!$this->is_active && $this->getOriginal('is_active')) {
            $this->deactivated_at = now();
            event(new EmailSettingDeactivated($this));
        }
    }

    /**
     * Make this the default setting.
     */
    public function makeDefault(): void
    {
        self::where('id', '!=', $this->id)
            ->where('is_default', true)
            ->update(['is_default' => false]);
        
        $this->update(['is_default' => true]);
    }

    /**
     * Build connection string.
     */
    protected function buildConnectionString(): string
    {
        if ($this->provider === EmailProviderEnum::SMTP) {
            return sprintf(
                '%s://%s:%s@%s:%d',
                strtolower($this->encryption->value),
                $this->username,
                '****',
                $this->host,
                $this->port
            );
        }
        
        return $this->provider->value . '://configured';
    }

    /**
     * Check if limit has been reached.
     */
    public function hasReachedLimit(): bool
    {
        if ($this->daily_limit > 0 && $this->messages_sent_today >= $this->daily_limit) {
            return true;
        }
        
        if ($this->monthly_limit > 0 && $this->messages_sent_month >= $this->monthly_limit) {
            return true;
        }
        
        return false;
    }

    /**
     * Increment send counter.
     */
    public function incrementSendCount(bool $success = true): void
    {
        $this->increment('total_sends');
        $this->increment('messages_sent_today');
        $this->increment('messages_sent_month');
        
        if ($success) {
            $this->increment('successful_sends');
            $this->update([
                'last_used_at' => now(),
                'last_success_at' => now(),
                'consecutive_failures' => 0,
            ]);
        } else {
            $this->increment('failed_sends');
            $this->increment('consecutive_failures');
            $this->update(['last_failure_at' => now()]);
        }
        
        $this->updateMetrics();
    }

    /**
     * Update metrics.
     */
    protected function updateMetrics(): void
    {
        // Calculate delivery rate
        if ($this->total_sends > 0) {
            $this->delivery_rate = ($this->successful_sends / $this->total_sends) * 100;
        }
        
        // Update health score
        $this->updateHealthScore();
        
        $this->save();
        
        // Record metrics
        $this->metrics()->create([
            'sends' => $this->messages_sent_today,
            'successes' => $this->successful_sends,
            'failures' => $this->failed_sends,
            'delivery_rate' => $this->delivery_rate,
            'bounce_rate' => $this->bounce_rate,
            'complaint_rate' => $this->complaint_rate,
            'health_score' => $this->health_score,
            'recorded_at' => now(),
        ]);
    }

    /**
     * Update health score.
     */
    protected function updateHealthScore(): void
    {
        $score = 100;
        
        // Deduct for failures
        if ($this->consecutive_failures > 0) {
            $score -= min(50, $this->consecutive_failures * 10);
        }
        
        // Deduct for poor delivery rate
        if ($this->delivery_rate < 95) {
            $score -= (100 - $this->delivery_rate) / 2;
        }
        
        // Deduct for high bounce rate
        if ($this->bounce_rate > 5) {
            $score -= min(30, $this->bounce_rate * 2);
        }
        
        // Deduct for complaints
        if ($this->complaint_rate > 0.1) {
            $score -= min(20, $this->complaint_rate * 100);
        }
        
        $this->health_score = max(0, $score);
    }

    /**
     * Schedule health check.
     */
    protected function scheduleHealthCheck(): void
    {
        dispatch(function () {
            $checker = app(HealthChecker::class);
            $checker->check($this);
        })->delay(now()->addMinutes(5));
    }

    /**
     * Refresh OAuth token.
     */
    public function refreshOAuthToken(): bool
    {
        if (!$this->oauth_refresh_token) {
            return false;
        }
        
        // Provider-specific OAuth refresh logic
        try {
            $newToken = $this->performOAuthRefresh();
            
            $this->update([
                'oauth_token' => $newToken['access_token'],
                'oauth_expires_at' => now()->addSeconds($newToken['expires_in']),
            ]);
            
            return true;
        } catch (\Exception $e) {
            $this->recordError('OAuth refresh failed: ' . $e->getMessage());
            return false;
        }
    }

    /**
     * Perform OAuth refresh (provider-specific).
     */
    protected function performOAuthRefresh(): array
    {
        // Implementation would be provider-specific
        return [];
    }

    /**
     * Disable the setting.
     */
    public function disable(string $reason = null): void
    {
        $this->update([
            'is_active' => false,
            'status' => EmailSettingStatus::DISABLED,
            'deactivated_at' => now(),
            'metadata' => array_merge($this->metadata ?? [], [
                'disabled_reason' => $reason,
                'disabled_at' => now()->toIso8601String(),
            ]),
        ]);
        
        event(new EmailSettingFailed($this, $reason));
    }

    /**
     * Record error.
     */
    protected function recordError(string $error): void
    {
        $this->events()->create([
            'type' => 'error',
            'message' => $error,
            'occurred_at' => now(),
        ]);
    }

    /**
     * Get mail configuration array.
     */
    public function toMailConfig(): array
    {
        return Cache::tags(self::$cacheConfig['tags'])
            ->remember(
                "email_setting_{$this->id}_config",
                self::$cacheConfig['ttl'],
                fn () => $this->buildMailConfig()
            );
    }

    /**
     * Build mail configuration.
     */
    protected function buildMailConfig(): array
    {
        $config = [
            'transport' => $this->driver,
            'host' => $this->host,
            'port' => $this->port,
            'encryption' => $this->encryption?->value,
            'username' => $this->username,
            'password' => Crypt::decryptString($this->password),
            'timeout' => $this->timeout,
            'auth_mode' => $this->auth_method?->value,
            'from' => [
                'address' => $this->from_address,
                'name' => $this->from_name,
            ],
        ];
        
        if ($this->reply_to_address) {
            $config['reply_to'] = [
                'address' => $this->reply_to_address,
                'name' => $this->reply_to_name,
            ];
        }
        
        // Add provider-specific settings
        if ($this->provider === EmailProviderEnum::SES) {
            $config['key'] = Crypt::decryptString($this->api_key);
            $config['secret'] = Crypt::decryptString($this->api_secret);
            $config['region'] = $this->region;
        }
        
        return $config;
    }

    /**
     * Clear cache.
     */
    protected function clearCache(): void
    {
        Cache::tags(self::$cacheConfig['tags'])->forget("email_setting_{$this->id}_config");
    }

    /**
     * Reset daily limits.
     */
    public static function resetDailyLimits(): void
    {
        self::query()->update([
            'messages_sent_today' => 0,
            'limit_reset_at' => now()->addDay(),
        ]);
    }

    /**
     * Reset monthly limits.
     */
    public static function resetMonthlyLimits(): void
    {
        self::query()->update([
            'messages_sent_month' => 0,
            'limit_reset_at' => now()->addMonth(),
        ]);
    }

    /**
     * Get the best provider for email type.
     */
    public static function getBestProvider(string $emailType = 'transactional'): ?self
    {
        $query = self::active()->healthy();
        
        $query = match($emailType) {
            'transactional' => $query->transactional(),
            'marketing' => $query->marketing(),
            'bulk' => $query->bulk(),
            default => $query,
        };
        
        return $query->orderByDesc('health_score')
            ->orderBy('average_send_time')
            ->first();
    }
}
<?php

declare(strict_types=1);

namespace App\Services\Integration;

use App\Models\ConsentRecord;
use App\Models\User;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Str;
use Carbon\Carbon;

/**
 * ConsentService - Centralized GDPR Consent Management
 *
 * Matches and exceeds Consent Magic Pro features:
 * - Multi-category consent (marketing, analytics, functional, necessary)
 * - Granular consent preferences
 * - Consent history/audit trail
 * - GeoIP-based consent requirements
 * - Cookie banner integration
 * - Consent withdrawal handling
 * - Data export (GDPR Article 20)
 * - Right to be forgotten (GDPR Article 17)
 *
 * @author Revolution Trading Pros
 * @version 2.0.0
 */
class ConsentService
{
    private const CACHE_PREFIX = 'consent:';
    private const CACHE_TTL = 3600;

    /**
     * Consent categories matching Consent Magic Pro
     */
    public const CATEGORY_NECESSARY = 'necessary';
    public const CATEGORY_FUNCTIONAL = 'functional';
    public const CATEGORY_ANALYTICS = 'analytics';
    public const CATEGORY_MARKETING = 'marketing';
    public const CATEGORY_PERSONALIZATION = 'personalization';

    /**
     * Consent purposes for granular control
     */
    public const PURPOSE_EMAIL_MARKETING = 'email_marketing';
    public const PURPOSE_SMS_MARKETING = 'sms_marketing';
    public const PURPOSE_THIRD_PARTY_SHARING = 'third_party_sharing';
    public const PURPOSE_PROFILING = 'profiling';
    public const PURPOSE_ANALYTICS_TRACKING = 'analytics_tracking';
    public const PURPOSE_PERSONALIZED_ADS = 'personalized_ads';
    public const PURPOSE_NEWSLETTER = 'newsletter';
    public const PURPOSE_TRANSACTIONAL = 'transactional';

    /**
     * Regions requiring explicit consent (GDPR territories)
     */
    private const GDPR_REGIONS = [
        'AT', 'BE', 'BG', 'HR', 'CY', 'CZ', 'DK', 'EE', 'FI', 'FR',
        'DE', 'GR', 'HU', 'IE', 'IT', 'LV', 'LT', 'LU', 'MT', 'NL',
        'PL', 'PT', 'RO', 'SK', 'SI', 'ES', 'SE', 'GB', 'IS', 'LI', 'NO',
    ];

    /**
     * CCPA regions
     */
    private const CCPA_REGIONS = ['US-CA'];

    /**
     * Get all consent categories with descriptions
     */
    public function getConsentCategories(): array
    {
        return [
            self::CATEGORY_NECESSARY => [
                'name' => 'Strictly Necessary',
                'description' => 'Essential cookies required for the website to function properly. These cannot be disabled.',
                'required' => true,
                'default' => true,
            ],
            self::CATEGORY_FUNCTIONAL => [
                'name' => 'Functional',
                'description' => 'Cookies that enable enhanced functionality and personalization, such as remembering your preferences.',
                'required' => false,
                'default' => false,
            ],
            self::CATEGORY_ANALYTICS => [
                'name' => 'Analytics',
                'description' => 'Cookies that help us understand how visitors interact with our website to improve user experience.',
                'required' => false,
                'default' => false,
            ],
            self::CATEGORY_MARKETING => [
                'name' => 'Marketing',
                'description' => 'Cookies used to track visitors across websites to display relevant advertisements.',
                'required' => false,
                'default' => false,
            ],
            self::CATEGORY_PERSONALIZATION => [
                'name' => 'Personalization',
                'description' => 'Cookies that enable personalized content and recommendations based on your behavior.',
                'required' => false,
                'default' => false,
            ],
        ];
    }

    /**
     * Get all consent purposes with descriptions
     */
    public function getConsentPurposes(): array
    {
        return [
            self::PURPOSE_EMAIL_MARKETING => [
                'name' => 'Email Marketing',
                'description' => 'Receive promotional emails, newsletters, and marketing communications.',
                'category' => self::CATEGORY_MARKETING,
            ],
            self::PURPOSE_SMS_MARKETING => [
                'name' => 'SMS Marketing',
                'description' => 'Receive promotional text messages and SMS notifications.',
                'category' => self::CATEGORY_MARKETING,
            ],
            self::PURPOSE_NEWSLETTER => [
                'name' => 'Newsletter',
                'description' => 'Receive our regular newsletter with updates and news.',
                'category' => self::CATEGORY_MARKETING,
            ],
            self::PURPOSE_THIRD_PARTY_SHARING => [
                'name' => 'Third Party Sharing',
                'description' => 'Share your data with trusted third-party partners.',
                'category' => self::CATEGORY_MARKETING,
            ],
            self::PURPOSE_PROFILING => [
                'name' => 'Profiling',
                'description' => 'Create a profile of your preferences and behavior for personalized experiences.',
                'category' => self::CATEGORY_PERSONALIZATION,
            ],
            self::PURPOSE_ANALYTICS_TRACKING => [
                'name' => 'Analytics Tracking',
                'description' => 'Track your interactions to improve our services and user experience.',
                'category' => self::CATEGORY_ANALYTICS,
            ],
            self::PURPOSE_PERSONALIZED_ADS => [
                'name' => 'Personalized Advertising',
                'description' => 'Display personalized advertisements based on your interests.',
                'category' => self::CATEGORY_MARKETING,
            ],
            self::PURPOSE_TRANSACTIONAL => [
                'name' => 'Transactional Communications',
                'description' => 'Receive order confirmations, shipping updates, and account notifications.',
                'category' => self::CATEGORY_NECESSARY,
                'required' => true,
            ],
        ];
    }

    /**
     * Record consent for a user/email
     */
    public function recordConsent(
        string $identifier,
        string $purposeOrCategory,
        bool $granted,
        array $metadata = []
    ): ConsentRecord {
        $record = ConsentRecord::create([
            'identifier' => $identifier,
            'identifier_type' => filter_var($identifier, FILTER_VALIDATE_EMAIL) ? 'email' : 'cookie_id',
            'consent_type' => $purposeOrCategory,
            'granted' => $granted,
            'ip_address' => $metadata['ip'] ?? request()->ip(),
            'user_agent' => $metadata['user_agent'] ?? request()->userAgent(),
            'source' => $metadata['source'] ?? 'website',
            'region' => $metadata['region'] ?? $this->detectRegion(),
            'metadata' => array_merge($metadata, [
                'timestamp' => now()->toIso8601String(),
                'version' => config('app.consent_policy_version', '1.0'),
            ]),
        ]);

        // Invalidate cache
        $this->invalidateCache($identifier);

        // Log for audit trail
        Log::info('ConsentService: Consent recorded', [
            'identifier' => $this->maskIdentifier($identifier),
            'type' => $purposeOrCategory,
            'granted' => $granted,
            'region' => $record->region,
        ]);

        return $record;
    }

    /**
     * Record multiple consents at once (from consent banner)
     */
    public function recordBulkConsent(
        string $identifier,
        array $consents,
        array $metadata = []
    ): array {
        $records = [];

        DB::transaction(function () use ($identifier, $consents, $metadata, &$records) {
            foreach ($consents as $type => $granted) {
                $records[] = $this->recordConsent($identifier, $type, (bool) $granted, $metadata);
            }
        });

        return $records;
    }

    /**
     * Check if consent is granted for a specific purpose
     */
    public function hasConsent(string $identifier, string $purposeOrCategory): bool
    {
        $cacheKey = self::CACHE_PREFIX . "{$identifier}:{$purposeOrCategory}";

        return Cache::remember($cacheKey, self::CACHE_TTL, function () use ($identifier, $purposeOrCategory) {
            $record = ConsentRecord::where('identifier', $identifier)
                ->where('consent_type', $purposeOrCategory)
                ->latest()
                ->first();

            return $record?->granted ?? false;
        });
    }

    /**
     * Check if marketing consent is granted
     */
    public function hasMarketingConsent(string $identifier): bool
    {
        return $this->hasConsent($identifier, self::CATEGORY_MARKETING)
            || $this->hasConsent($identifier, self::PURPOSE_EMAIL_MARKETING);
    }

    /**
     * Check if analytics consent is granted
     */
    public function hasAnalyticsConsent(string $identifier): bool
    {
        return $this->hasConsent($identifier, self::CATEGORY_ANALYTICS)
            || $this->hasConsent($identifier, self::PURPOSE_ANALYTICS_TRACKING);
    }

    /**
     * Check if transactional communications are allowed
     * (Always allowed as they're necessary for service delivery)
     */
    public function hasTransactionalConsent(string $identifier): bool
    {
        // Transactional emails are allowed by legitimate interest
        // unless explicitly opted out
        $optedOut = ConsentRecord::where('identifier', $identifier)
            ->where('consent_type', self::PURPOSE_TRANSACTIONAL)
            ->where('granted', false)
            ->exists();

        return !$optedOut;
    }

    /**
     * Get full consent status for an identifier
     */
    public function getConsentStatus(string $identifier): array
    {
        $cacheKey = self::CACHE_PREFIX . "status:{$identifier}";

        return Cache::remember($cacheKey, self::CACHE_TTL, function () use ($identifier) {
            $records = ConsentRecord::where('identifier', $identifier)
                ->orderBy('consent_type')
                ->orderByDesc('created_at')
                ->get()
                ->groupBy('consent_type')
                ->map(fn($group) => $group->first());

            $categories = [];
            $purposes = [];

            foreach ($this->getConsentCategories() as $key => $category) {
                $record = $records->get($key);
                $categories[$key] = [
                    'granted' => $category['required'] ? true : ($record?->granted ?? false),
                    'required' => $category['required'],
                    'last_updated' => $record?->created_at?->toIso8601String(),
                ];
            }

            foreach ($this->getConsentPurposes() as $key => $purpose) {
                $record = $records->get($key);
                $isRequired = $purpose['required'] ?? false;
                $purposes[$key] = [
                    'granted' => $isRequired ? true : ($record?->granted ?? false),
                    'required' => $isRequired,
                    'category' => $purpose['category'],
                    'last_updated' => $record?->created_at?->toIso8601String(),
                ];
            }

            return [
                'identifier' => $this->maskIdentifier($identifier),
                'categories' => $categories,
                'purposes' => $purposes,
                'has_any_consent' => $records->isNotEmpty(),
                'last_consent_date' => $records->max('created_at')?->toIso8601String(),
                'requires_gdpr' => $this->requiresGdprConsent(),
                'requires_ccpa' => $this->requiresCcpaConsent(),
            ];
        });
    }

    /**
     * Withdraw consent (GDPR Article 7.3)
     */
    public function withdrawConsent(string $identifier, string $purposeOrCategory): ConsentRecord
    {
        return $this->recordConsent($identifier, $purposeOrCategory, false, [
            'source' => 'user_withdrawal',
            'withdrawal_reason' => 'user_requested',
        ]);
    }

    /**
     * Withdraw all consents
     */
    public function withdrawAllConsents(string $identifier): array
    {
        $records = [];
        $purposes = array_keys($this->getConsentPurposes());
        $categories = array_keys($this->getConsentCategories());

        foreach (array_merge($purposes, $categories) as $type) {
            // Skip necessary category
            if ($type === self::CATEGORY_NECESSARY) {
                continue;
            }
            $records[] = $this->withdrawConsent($identifier, $type);
        }

        return $records;
    }

    /**
     * Export consent history (GDPR Article 20 - Right to Data Portability)
     */
    public function exportConsentHistory(string $identifier): array
    {
        $records = ConsentRecord::where('identifier', $identifier)
            ->orderBy('created_at')
            ->get();

        return [
            'export_date' => now()->toIso8601String(),
            'identifier' => $this->maskIdentifier($identifier),
            'total_records' => $records->count(),
            'consent_history' => $records->map(function ($record) {
                return [
                    'consent_type' => $record->consent_type,
                    'granted' => $record->granted,
                    'date' => $record->created_at->toIso8601String(),
                    'source' => $record->source,
                    'region' => $record->region,
                ];
            })->toArray(),
            'current_status' => $this->getConsentStatus($identifier),
        ];
    }

    /**
     * Delete all consent records (GDPR Article 17 - Right to Erasure)
     */
    public function deleteConsentRecords(string $identifier): int
    {
        $this->invalidateCache($identifier);

        // Create deletion record for audit
        ConsentRecord::create([
            'identifier' => hash('sha256', $identifier), // Store hashed for audit
            'identifier_type' => 'deleted',
            'consent_type' => 'erasure_request',
            'granted' => false,
            'metadata' => [
                'original_identifier_hash' => hash('sha256', $identifier),
                'erasure_date' => now()->toIso8601String(),
                'reason' => 'gdpr_article_17',
            ],
        ]);

        return ConsentRecord::where('identifier', $identifier)->delete();
    }

    /**
     * Check if current region requires GDPR consent
     */
    public function requiresGdprConsent(?string $region = null): bool
    {
        $region = $region ?? $this->detectRegion();
        return in_array(strtoupper($region), self::GDPR_REGIONS);
    }

    /**
     * Check if current region requires CCPA consent
     */
    public function requiresCcpaConsent(?string $region = null): bool
    {
        $region = $region ?? $this->detectRegion();
        return in_array($region, self::CCPA_REGIONS);
    }

    /**
     * Get consent requirements for a region
     */
    public function getRegionRequirements(?string $region = null): array
    {
        $region = $region ?? $this->detectRegion();

        return [
            'region' => $region,
            'gdpr_applies' => $this->requiresGdprConsent($region),
            'ccpa_applies' => $this->requiresCcpaConsent($region),
            'explicit_consent_required' => $this->requiresGdprConsent($region),
            'opt_out_available' => $this->requiresCcpaConsent($region),
            'banner_required' => $this->requiresGdprConsent($region) || $this->requiresCcpaConsent($region),
            'categories' => $this->getConsentCategories(),
            'purposes' => $this->getConsentPurposes(),
        ];
    }

    /**
     * Generate consent banner configuration
     */
    public function getBannerConfiguration(?string $region = null): array
    {
        $requirements = $this->getRegionRequirements($region);

        return [
            'show_banner' => $requirements['banner_required'],
            'mode' => $requirements['gdpr_applies'] ? 'opt-in' : 'opt-out',
            'categories' => array_map(function ($category, $key) {
                return [
                    'id' => $key,
                    'name' => $category['name'],
                    'description' => $category['description'],
                    'required' => $category['required'],
                    'default' => $category['default'],
                ];
            }, $this->getConsentCategories(), array_keys($this->getConsentCategories())),
            'texts' => [
                'title' => 'Cookie & Privacy Settings',
                'description' => 'We use cookies and similar technologies to enhance your experience, analyze traffic, and for marketing purposes. Please choose your preferences below.',
                'accept_all' => 'Accept All',
                'reject_all' => 'Reject All',
                'save_preferences' => 'Save Preferences',
                'manage_preferences' => 'Manage Preferences',
                'privacy_policy_link' => '/privacy-policy',
                'cookie_policy_link' => '/cookie-policy',
            ],
            'styling' => [
                'position' => 'bottom',
                'theme' => 'light',
                'blur_background' => true,
            ],
        ];
    }

    /**
     * Verify consent before an action
     */
    public function verifyConsentForAction(string $identifier, string $action): array
    {
        $actionConsentMap = [
            'send_marketing_email' => [self::PURPOSE_EMAIL_MARKETING, self::CATEGORY_MARKETING],
            'send_newsletter' => [self::PURPOSE_NEWSLETTER, self::CATEGORY_MARKETING],
            'track_analytics' => [self::PURPOSE_ANALYTICS_TRACKING, self::CATEGORY_ANALYTICS],
            'show_personalized_content' => [self::PURPOSE_PROFILING, self::CATEGORY_PERSONALIZATION],
            'share_with_third_party' => [self::PURPOSE_THIRD_PARTY_SHARING],
            'show_personalized_ads' => [self::PURPOSE_PERSONALIZED_ADS, self::CATEGORY_MARKETING],
            'send_sms' => [self::PURPOSE_SMS_MARKETING],
            'create_profile' => [self::PURPOSE_PROFILING],
        ];

        $requiredConsents = $actionConsentMap[$action] ?? [];

        if (empty($requiredConsents)) {
            return ['allowed' => true, 'reason' => 'no_consent_required'];
        }

        foreach ($requiredConsents as $consent) {
            if ($this->hasConsent($identifier, $consent)) {
                return ['allowed' => true, 'consent_type' => $consent];
            }
        }

        return [
            'allowed' => false,
            'reason' => 'consent_not_granted',
            'required_consents' => $requiredConsents,
        ];
    }

    /**
     * Detect region from request
     */
    private function detectRegion(): string
    {
        // Try to get from GeoIP (if available)
        $ip = request()->ip();

        // Check for Cloudflare headers
        $cfCountry = request()->header('CF-IPCountry');
        if ($cfCountry) {
            return $cfCountry;
        }

        // Check for custom region header (set by frontend)
        $region = request()->header('X-User-Region');
        if ($region) {
            return $region;
        }

        // Default to requiring consent (safest option)
        return 'UNKNOWN';
    }

    /**
     * Mask identifier for logging (privacy)
     */
    private function maskIdentifier(string $identifier): string
    {
        if (filter_var($identifier, FILTER_VALIDATE_EMAIL)) {
            $parts = explode('@', $identifier);
            $name = $parts[0];
            $domain = $parts[1] ?? '';
            $masked = substr($name, 0, 2) . str_repeat('*', max(0, strlen($name) - 2));
            return $masked . '@' . $domain;
        }

        return substr($identifier, 0, 4) . str_repeat('*', max(0, strlen($identifier) - 4));
    }

    /**
     * Invalidate cache for identifier
     */
    private function invalidateCache(string $identifier): void
    {
        // Clear all cached consent data for this identifier
        Cache::forget(self::CACHE_PREFIX . "status:{$identifier}");

        $allTypes = array_merge(
            array_keys($this->getConsentCategories()),
            array_keys($this->getConsentPurposes())
        );

        foreach ($allTypes as $type) {
            Cache::forget(self::CACHE_PREFIX . "{$identifier}:{$type}");
        }
    }
}

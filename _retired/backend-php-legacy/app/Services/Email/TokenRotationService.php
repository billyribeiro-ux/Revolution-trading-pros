<?php

declare(strict_types=1);

namespace App\Services\Email;

use App\Models\NewsletterSubscription;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Str;
use Carbon\Carbon;

/**
 * TokenRotationService - Security Token Rotation for Email Marketing
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Provides enterprise-grade token security through automatic rotation:
 * - Automatic rotation of unsubscribe tokens
 * - Verification token management
 * - Token versioning and migration
 * - Compromised token detection
 * - Audit logging for token operations
 *
 * @version 1.0.0
 */
class TokenRotationService
{
    /**
     * Token rotation periods
     */
    private const UNSUBSCRIBE_TOKEN_ROTATION_DAYS = 90;
    private const VERIFICATION_TOKEN_EXPIRY_HOURS = 48;
    private const PREFERENCE_TOKEN_ROTATION_DAYS = 30;

    /**
     * Cache keys
     */
    private const TOKEN_MAP_PREFIX = 'token_map:';
    private const ROTATION_LOCK_PREFIX = 'token_rotation_lock:';
    private const COMPROMISED_PREFIX = 'compromised_token:';

    /**
     * Rotate unsubscribe token for a subscriber
     */
    public function rotateUnsubscribeToken(NewsletterSubscription $subscription): string
    {
        $oldToken = $subscription->unsubscribe_token;
        $newToken = $this->generateSecureToken();

        // Store mapping from old to new (for grace period)
        $this->storeTokenMapping($oldToken, $newToken, 'unsubscribe', 24 * 7); // 7 day grace period

        // Update subscriber
        $subscription->update([
            'unsubscribe_token' => $newToken,
            'token_rotated_at' => now(),
        ]);

        Log::info('[TokenRotationService] Unsubscribe token rotated', [
            'subscriber_id' => $subscription->id,
            'email' => $this->maskEmail($subscription->email),
        ]);

        return $newToken;
    }

    /**
     * Rotate verification token
     */
    public function rotateVerificationToken(NewsletterSubscription $subscription): string
    {
        $oldToken = $subscription->verification_token;
        $newToken = $this->generateSecureToken();

        // Store mapping (short grace period for verification)
        if ($oldToken) {
            $this->storeTokenMapping($oldToken, $newToken, 'verification', 1); // 1 hour grace
        }

        $subscription->update([
            'verification_token' => hash('sha256', $newToken),
        ]);

        return $newToken;
    }

    /**
     * Generate preference center token
     */
    public function generatePreferenceToken(NewsletterSubscription $subscription): string
    {
        $token = $this->generateSecureToken();
        $hash = hash('sha256', $token);

        // Store in cache with expiry
        Cache::put(
            "preference_token:{$hash}",
            [
                'subscriber_id' => $subscription->id,
                'created_at' => now()->toIso8601String(),
            ],
            now()->addDays(self::PREFERENCE_TOKEN_ROTATION_DAYS)
        );

        return $token;
    }

    /**
     * Validate and resolve token (handles old tokens during grace period)
     */
    public function resolveToken(string $token, string $type = 'unsubscribe'): ?string
    {
        // Check if token is compromised
        if ($this->isCompromised($token)) {
            Log::warning('[TokenRotationService] Attempted use of compromised token', [
                'token_prefix' => substr($token, 0, 8),
                'type' => $type,
            ]);
            return null;
        }

        // Check if this is an old token with a mapping
        $mapping = $this->getTokenMapping($token, $type);
        if ($mapping) {
            return $mapping['new_token'];
        }

        // Return original token if no mapping exists
        return $token;
    }

    /**
     * Batch rotate tokens for subscribers
     */
    public function batchRotateTokens(int $limit = 1000): array
    {
        $lockKey = self::ROTATION_LOCK_PREFIX . 'batch';

        // Prevent concurrent rotation
        if (!Cache::add($lockKey, true, 300)) {
            return ['status' => 'locked', 'message' => 'Rotation already in progress'];
        }

        try {
            $cutoffDate = now()->subDays(self::UNSUBSCRIBE_TOKEN_ROTATION_DAYS);

            $subscribers = NewsletterSubscription::where('status', 'active')
                ->where(function ($query) use ($cutoffDate) {
                    $query->whereNull('token_rotated_at')
                        ->orWhere('token_rotated_at', '<', $cutoffDate);
                })
                ->limit($limit)
                ->get();

            $rotated = 0;
            $failed = 0;

            foreach ($subscribers as $subscription) {
                try {
                    $this->rotateUnsubscribeToken($subscription);
                    $rotated++;
                } catch (\Throwable $e) {
                    $failed++;
                    Log::error('[TokenRotationService] Token rotation failed', [
                        'subscriber_id' => $subscription->id,
                        'error' => $e->getMessage(),
                    ]);
                }
            }

            Log::info('[TokenRotationService] Batch rotation completed', [
                'rotated' => $rotated,
                'failed' => $failed,
            ]);

            return [
                'status' => 'success',
                'rotated' => $rotated,
                'failed' => $failed,
                'remaining' => NewsletterSubscription::where('status', 'active')
                    ->where(function ($query) use ($cutoffDate) {
                        $query->whereNull('token_rotated_at')
                            ->orWhere('token_rotated_at', '<', $cutoffDate);
                    })
                    ->count(),
            ];
        } finally {
            Cache::forget($lockKey);
        }
    }

    /**
     * Mark token as compromised
     */
    public function markCompromised(string $token, string $reason = ''): void
    {
        Cache::put(
            self::COMPROMISED_PREFIX . hash('sha256', $token),
            [
                'reason' => $reason,
                'compromised_at' => now()->toIso8601String(),
            ],
            now()->addYear()
        );

        Log::warning('[TokenRotationService] Token marked as compromised', [
            'token_prefix' => substr($token, 0, 8),
            'reason' => $reason,
        ]);

        // If this is an unsubscribe token, rotate it immediately
        $subscription = NewsletterSubscription::where('unsubscribe_token', $token)->first();
        if ($subscription) {
            $this->rotateUnsubscribeToken($subscription);
        }
    }

    /**
     * Check if token is compromised
     */
    public function isCompromised(string $token): bool
    {
        return Cache::has(self::COMPROMISED_PREFIX . hash('sha256', $token));
    }

    /**
     * Generate secure token
     */
    private function generateSecureToken(): string
    {
        return Str::random(64);
    }

    /**
     * Store token mapping for grace period
     */
    private function storeTokenMapping(string $oldToken, string $newToken, string $type, int $hours): void
    {
        if (empty($oldToken)) {
            return;
        }

        $key = self::TOKEN_MAP_PREFIX . $type . ':' . hash('sha256', $oldToken);

        Cache::put($key, [
            'new_token' => $newToken,
            'mapped_at' => now()->toIso8601String(),
        ], now()->addHours($hours));
    }

    /**
     * Get token mapping
     */
    private function getTokenMapping(string $oldToken, string $type): ?array
    {
        $key = self::TOKEN_MAP_PREFIX . $type . ':' . hash('sha256', $oldToken);
        return Cache::get($key);
    }

    /**
     * Clean expired token mappings (for maintenance)
     */
    public function cleanExpiredMappings(): int
    {
        // Redis handles expiry automatically
        // This method is for any additional cleanup needed

        Log::info('[TokenRotationService] Token mapping cleanup completed');
        return 0;
    }

    /**
     * Get rotation statistics
     */
    public function getStatistics(): array
    {
        $cutoffDate = now()->subDays(self::UNSUBSCRIBE_TOKEN_ROTATION_DAYS);

        return [
            'total_subscribers' => NewsletterSubscription::where('status', 'active')->count(),
            'tokens_needing_rotation' => NewsletterSubscription::where('status', 'active')
                ->where(function ($query) use ($cutoffDate) {
                    $query->whereNull('token_rotated_at')
                        ->orWhere('token_rotated_at', '<', $cutoffDate);
                })
                ->count(),
            'tokens_rotated_today' => NewsletterSubscription::whereDate('token_rotated_at', today())->count(),
            'tokens_rotated_this_week' => NewsletterSubscription::whereBetween('token_rotated_at', [
                now()->startOfWeek(),
                now()->endOfWeek(),
            ])->count(),
            'rotation_period_days' => self::UNSUBSCRIBE_TOKEN_ROTATION_DAYS,
        ];
    }

    /**
     * Verify token ownership (for security checks)
     */
    public function verifyTokenOwnership(string $token, string $email, string $type = 'unsubscribe'): bool
    {
        $subscription = NewsletterSubscription::where('email', strtolower($email))->first();

        if (!$subscription) {
            return false;
        }

        if ($type === 'unsubscribe') {
            if ($subscription->unsubscribe_token === $token) {
                return true;
            }

            // Check if it's an old token during grace period
            $mapping = $this->getTokenMapping($token, $type);
            if ($mapping && $subscription->unsubscribe_token === $mapping['new_token']) {
                return true;
            }
        }

        return false;
    }

    /**
     * Generate time-limited token (for one-time actions)
     */
    public function generateTimeLimitedToken(int $subscriberId, string $action, int $expiryMinutes = 60): string
    {
        $token = $this->generateSecureToken();
        $hash = hash('sha256', $token);

        Cache::put(
            "action_token:{$hash}",
            [
                'subscriber_id' => $subscriberId,
                'action' => $action,
                'created_at' => now()->toIso8601String(),
            ],
            now()->addMinutes($expiryMinutes)
        );

        return $token;
    }

    /**
     * Validate time-limited token
     */
    public function validateTimeLimitedToken(string $token, string $expectedAction): ?array
    {
        $hash = hash('sha256', $token);
        $data = Cache::get("action_token:{$hash}");

        if (!$data) {
            return null;
        }

        if ($data['action'] !== $expectedAction) {
            return null;
        }

        // Invalidate token after use (one-time use)
        Cache::forget("action_token:{$hash}");

        return $data;
    }

    /**
     * Mask email for logging
     */
    private function maskEmail(string $email): string
    {
        $parts = explode('@', $email);
        $name = $parts[0];
        $domain = $parts[1] ?? '';
        $masked = substr($name, 0, 2) . str_repeat('*', max(0, strlen($name) - 2));
        return $masked . '@' . $domain;
    }
}

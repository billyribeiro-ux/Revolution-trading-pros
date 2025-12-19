<?php

declare(strict_types=1);

namespace App\Contracts\Seo;

/**
 * SEO Data Provider Interface
 *
 * Apple Principal Engineer ICT11+ Architecture
 * ══════════════════════════════════════════════════════════════════════════════
 *
 * Unified interface for all SEO data providers (Google, Bing, Third-party).
 * Enables seamless switching between data sources while maintaining consistency.
 *
 * Design Principles:
 * - Protocol-Oriented Design (Swift-inspired)
 * - Dependency Inversion (SOLID)
 * - Provider Agnostic
 * - Graceful Degradation
 *
 * @version 2.0.0
 * @level ICT11+ Principal Engineer
 */
interface SeoDataProviderInterface
{
    /**
     * Provider identification constants
     */
    public const PROVIDER_GOOGLE = 'google';
    public const PROVIDER_BING = 'bing';
    public const PROVIDER_SERPAPI = 'serpapi';
    public const PROVIDER_INTERNAL = 'internal';

    /**
     * Get the provider identifier.
     */
    public function getProviderId(): string;

    /**
     * Get provider display name.
     */
    public function getProviderName(): string;

    /**
     * Check if provider is available and authenticated.
     */
    public function isAvailable(): bool;

    /**
     * Get provider health status with detailed diagnostics.
     *
     * @return array{
     *     status: string,
     *     latency_ms: int,
     *     rate_limit_remaining: int|null,
     *     last_successful_call: string|null,
     *     error_rate_24h: float,
     *     capabilities: array<string>
     * }
     */
    public function getHealthStatus(): array;

    /**
     * Get provider capabilities.
     *
     * @return array<string> List of capability identifiers
     */
    public function getCapabilities(): array;

    /**
     * Check if provider supports a specific capability.
     */
    public function hasCapability(string $capability): bool;

    /**
     * Get provider priority (lower = higher priority).
     */
    public function getPriority(): int;

    /**
     * Get rate limit configuration.
     *
     * @return array{
     *     requests_per_minute: int,
     *     requests_per_day: int,
     *     current_usage: int,
     *     reset_at: string|null
     * }
     */
    public function getRateLimitConfig(): array;
}

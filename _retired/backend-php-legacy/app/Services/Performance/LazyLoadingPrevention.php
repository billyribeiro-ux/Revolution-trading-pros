<?php

declare(strict_types=1);

namespace App\Services\Performance;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\Relation;
use Illuminate\Support\Facades\Log;

/**
 * Lazy Loading Prevention Service (ICT9+ Enterprise Grade)
 *
 * Prevents N+1 queries by:
 * - Detecting lazy loading in development
 * - Throwing exceptions or logging
 * - Suggesting eager loading
 *
 * @version 1.0.0
 * @level ICT9+ Principal Engineer Grade
 */
class LazyLoadingPrevention
{
    /**
     * Enable strict mode (throws exceptions)
     */
    public static function enableStrict(): void
    {
        Model::preventLazyLoading(!app()->isProduction());
    }

    /**
     * Enable logging mode (logs instead of throwing)
     */
    public static function enableLogging(): void
    {
        Model::preventLazyLoading();

        Model::handleLazyLoadingViolationUsing(function (Model $model, string $relation) {
            Log::warning('Lazy loading detected', [
                'model' => get_class($model),
                'relation' => $relation,
                'suggestion' => "Add '{$relation}' to eager loading: ->with('{$relation}')",
                'stack' => collect(debug_backtrace(DEBUG_BACKTRACE_IGNORE_ARGS, 10))
                    ->filter(fn($frame) => isset($frame['file']) && !str_contains($frame['file'], 'vendor'))
                    ->map(fn($frame) => ($frame['file'] ?? '') . ':' . ($frame['line'] ?? ''))
                    ->values()
                    ->take(5)
                    ->toArray(),
            ]);
        });
    }

    /**
     * Get common eager loading configurations
     */
    public static function getEagerLoadingMap(): array
    {
        return [
            // User related
            'App\Models\User' => ['memberships', 'subscriptions', 'orders'],

            // Contact related
            'App\Models\Contact' => ['company', 'owner', 'tags', 'lists'],

            // CRM related
            'App\Models\EmailSequence' => ['emails', 'trackers', 'creator'],
            'App\Models\AutomationFunnel' => ['actions', 'subscribers', 'creator'],
            'App\Models\SmartLink' => ['clicks', 'creator'],
            'App\Models\RecurringCampaign' => ['emails', 'creator'],

            // Content related
            'App\Models\Post' => ['author', 'categories', 'tags', 'seoAnalysis'],
            'App\Models\Product' => ['categories', 'variants', 'media'],

            // Forms
            'App\Models\Form' => ['fields', 'submissions', 'integrations'],
            'App\Models\FormSubmission' => ['data', 'contact', 'payments'],
        ];
    }

    /**
     * Auto-apply eager loading based on model type
     */
    public static function autoEagerLoad(string $modelClass, array $additionalRelations = []): array
    {
        $map = self::getEagerLoadingMap();
        $defaults = $map[$modelClass] ?? [];

        return array_unique(array_merge($defaults, $additionalRelations));
    }
}

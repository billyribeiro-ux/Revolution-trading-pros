<?php

declare(strict_types=1);

namespace App\Observers;

use App\Services\Seo\BingSeoService;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Support\Facades\Log;

/**
 * ContentPublishObserver
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Automatically submits content to Bing IndexNow when published.
 * This gives your content a MASSIVE advantage - indexed in minutes instead of days.
 *
 * @version 1.0.0
 */
class ContentPublishObserver
{
    public function __construct(
        private readonly BingSeoService $bingSeoService
    ) {}

    /**
     * Handle the created event
     */
    public function created(Model $model): void
    {
        $this->submitIfPublished($model);
    }

    /**
     * Handle the updated event
     */
    public function updated(Model $model): void
    {
        // Check if status changed to published
        if ($model->wasChanged('status') || $model->wasChanged('published_at')) {
            $this->submitIfPublished($model);
        }

        // Also submit if content was significantly updated
        if ($this->hasSignificantChanges($model)) {
            $this->submitIfPublished($model);
        }
    }

    /**
     * Submit to Bing if content is published
     */
    private function submitIfPublished(Model $model): void
    {
        // Check if auto-submit is enabled
        if (!config('seo.bing.auto_submit', true)) {
            return;
        }

        // Check if content is published
        if (!$this->isPublished($model)) {
            return;
        }

        // Get the public URL
        $url = $this->getPublicUrl($model);

        if (!$url) {
            return;
        }

        // Submit to IndexNow asynchronously
        try {
            dispatch(function () use ($url, $model) {
                $this->bingSeoService->onContentPublished(
                    $url,
                    $this->getContentType($model)
                );
            })->afterResponse();

            Log::info('[ContentPublishObserver] Queued URL for Bing submission', [
                'model' => get_class($model),
                'id' => $model->getKey(),
                'url' => $url,
            ]);
        } catch (\Exception $e) {
            Log::warning('[ContentPublishObserver] Failed to queue Bing submission', [
                'error' => $e->getMessage(),
                'model' => get_class($model),
            ]);
        }
    }

    /**
     * Check if model is published
     */
    private function isPublished(Model $model): bool
    {
        // Check common status fields
        if ($model->status === 'published') {
            return true;
        }

        if ($model->is_published ?? false) {
            return true;
        }

        // Check published_at date
        if (isset($model->published_at) && $model->published_at) {
            return $model->published_at <= now();
        }

        return false;
    }

    /**
     * Get public URL for the model
     */
    private function getPublicUrl(Model $model): ?string
    {
        $baseUrl = config('app.url');

        // Check for explicit url attribute
        if (isset($model->url)) {
            return $model->url;
        }

        // Check for slug-based URL
        if (isset($model->slug)) {
            $type = $this->getContentType($model);

            return match($type) {
                'post' => "{$baseUrl}/blog/{$model->slug}",
                'page' => "{$baseUrl}/{$model->slug}",
                'product' => "{$baseUrl}/products/{$model->slug}",
                'indicator' => "{$baseUrl}/indicators/{$model->slug}",
                'category' => "{$baseUrl}/category/{$model->slug}",
                default => "{$baseUrl}/{$model->slug}",
            };
        }

        return null;
    }

    /**
     * Get content type from model
     */
    private function getContentType(Model $model): string
    {
        $className = class_basename($model);

        return match($className) {
            'Post' => 'post',
            'Page' => 'page',
            'Product' => 'product',
            'Indicator' => 'indicator',
            'Category' => 'category',
            default => strtolower($className),
        };
    }

    /**
     * Check if model has significant content changes worth re-indexing
     */
    private function hasSignificantChanges(Model $model): bool
    {
        $significantFields = [
            'title', 'name', 'content', 'body', 'description',
            'meta_title', 'meta_description', 'slug',
        ];

        foreach ($significantFields as $field) {
            if ($model->wasChanged($field)) {
                return true;
            }
        }

        return false;
    }
}

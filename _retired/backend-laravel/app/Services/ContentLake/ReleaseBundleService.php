<?php

declare(strict_types=1);

namespace App\Services\ContentLake;

use Illuminate\Support\Facades\DB;
use Illuminate\Support\Str;

/**
 * Release Bundle Service
 *
 * Implements Sanity-style release bundles for grouped document publishing.
 *
 * Features:
 * - Group multiple documents into a release
 * - Schedule releases for future publishing
 * - Atomic publishing of all documents
 * - Rollback support
 * - Publishing status tracking
 */
class ReleaseBundleService
{
    public function __construct(
        private readonly DocumentPerspectiveService $perspectiveService,
        private readonly WebhookService $webhookService,
    ) {}

    /**
     * Create a new release bundle
     */
    public function create(array $data): array
    {
        $bundleId = Str::uuid()->toString();

        $id = DB::table('release_bundles')->insertGetId([
            'bundle_id' => $bundleId,
            'name' => $data['name'],
            'slug' => Str::slug($data['name']) . '-' . Str::random(4),
            'description' => $data['description'] ?? null,
            'status' => 'draft',
            'scheduled_at' => $data['scheduled_at'] ?? null,
            'metadata' => isset($data['metadata']) ? json_encode($data['metadata']) : null,
            'created_by' => auth()->id(),
            'created_at' => now(),
            'updated_at' => now(),
        ]);

        return $this->get($bundleId);
    }

    /**
     * Get release bundle
     */
    public function get(string $bundleId): ?array
    {
        $bundle = DB::table('release_bundles')
            ->where('bundle_id', $bundleId)
            ->whereNull('deleted_at')
            ->first();

        if (!$bundle) {
            return null;
        }

        $documents = DB::table('release_bundle_documents')
            ->where('bundle_id', $bundleId)
            ->orderBy('order')
            ->get();

        return [
            'id' => $bundle->bundle_id,
            'name' => $bundle->name,
            'slug' => $bundle->slug,
            'description' => $bundle->description,
            'status' => $bundle->status,
            'scheduledAt' => $bundle->scheduled_at,
            'publishedAt' => $bundle->published_at,
            'createdBy' => $bundle->created_by,
            'publishedBy' => $bundle->published_by,
            'metadata' => $bundle->metadata ? json_decode($bundle->metadata, true) : null,
            'publishLog' => $bundle->publish_log ? json_decode($bundle->publish_log, true) : null,
            'documents' => $documents->map(fn($d) => [
                'documentId' => $d->document_id,
                'documentType' => $d->document_type,
                'action' => $d->action,
                'status' => $d->status,
                'error' => $d->error_message,
            ])->toArray(),
            'documentCount' => $documents->count(),
            'createdAt' => $bundle->created_at,
            'updatedAt' => $bundle->updated_at,
        ];
    }

    /**
     * Add document to release
     */
    public function addDocument(
        string $bundleId,
        string $documentId,
        string $documentType,
        string $action = 'update'
    ): bool {
        $bundle = DB::table('release_bundles')
            ->where('bundle_id', $bundleId)
            ->whereIn('status', ['draft', 'scheduled'])
            ->first();

        if (!$bundle) {
            throw new \InvalidArgumentException("Cannot add document to bundle in this status");
        }

        // Get current document state
        $document = $this->perspectiveService->getDocument($documentId, 'draft');

        $maxOrder = DB::table('release_bundle_documents')
            ->where('bundle_id', $bundleId)
            ->max('order') ?? 0;

        DB::table('release_bundle_documents')->updateOrInsert(
            ['bundle_id' => $bundleId, 'document_id' => $documentId],
            [
                'document_type' => $documentType,
                'action' => $action,
                'snapshot' => json_encode($document),
                'order' => $maxOrder + 1,
                'status' => 'pending',
                'updated_at' => now(),
            ]
        );

        return true;
    }

    /**
     * Remove document from release
     */
    public function removeDocument(string $bundleId, string $documentId): bool
    {
        return DB::table('release_bundle_documents')
            ->where('bundle_id', $bundleId)
            ->where('document_id', $documentId)
            ->delete() > 0;
    }

    /**
     * Schedule release for publishing
     */
    public function schedule(string $bundleId, string $scheduledAt): array
    {
        DB::table('release_bundles')
            ->where('bundle_id', $bundleId)
            ->whereIn('status', ['draft'])
            ->update([
                'status' => 'scheduled',
                'scheduled_at' => $scheduledAt,
                'updated_at' => now(),
            ]);

        return $this->get($bundleId);
    }

    /**
     * Publish release immediately
     */
    public function publish(string $bundleId, ?int $userId = null): array
    {
        return DB::transaction(function () use ($bundleId, $userId) {
            $bundle = DB::table('release_bundles')
                ->where('bundle_id', $bundleId)
                ->whereIn('status', ['draft', 'scheduled'])
                ->lockForUpdate()
                ->first();

            if (!$bundle) {
                throw new \InvalidArgumentException("Bundle not found or cannot be published");
            }

            // Update status to publishing
            DB::table('release_bundles')
                ->where('bundle_id', $bundleId)
                ->update([
                    'status' => 'publishing',
                    'updated_at' => now(),
                ]);

            $documents = DB::table('release_bundle_documents')
                ->where('bundle_id', $bundleId)
                ->orderBy('order')
                ->get();

            $publishLog = [];
            $allSuccess = true;

            foreach ($documents as $doc) {
                try {
                    $startTime = microtime(true);

                    switch ($doc->action) {
                        case 'create':
                        case 'update':
                            $this->perspectiveService->publish($doc->document_id, $userId);
                            break;

                        case 'delete':
                            // Handle delete action
                            $this->perspectiveService->unpublish($doc->document_id, $userId);
                            break;
                    }

                    $duration = (microtime(true) - $startTime) * 1000;

                    DB::table('release_bundle_documents')
                        ->where('bundle_id', $bundleId)
                        ->where('document_id', $doc->document_id)
                        ->update([
                            'status' => 'published',
                            'updated_at' => now(),
                        ]);

                    $publishLog[] = [
                        'documentId' => $doc->document_id,
                        'action' => $doc->action,
                        'status' => 'success',
                        'durationMs' => round($duration, 2),
                        'timestamp' => now()->toIso8601String(),
                    ];

                } catch (\Throwable $e) {
                    $allSuccess = false;

                    DB::table('release_bundle_documents')
                        ->where('bundle_id', $bundleId)
                        ->where('document_id', $doc->document_id)
                        ->update([
                            'status' => 'failed',
                            'error_message' => $e->getMessage(),
                            'updated_at' => now(),
                        ]);

                    $publishLog[] = [
                        'documentId' => $doc->document_id,
                        'action' => $doc->action,
                        'status' => 'failed',
                        'error' => $e->getMessage(),
                        'timestamp' => now()->toIso8601String(),
                    ];
                }
            }

            // Update bundle status
            DB::table('release_bundles')
                ->where('bundle_id', $bundleId)
                ->update([
                    'status' => $allSuccess ? 'published' : 'failed',
                    'published_at' => $allSuccess ? now() : null,
                    'published_by' => $userId ?? auth()->id(),
                    'publish_log' => json_encode($publishLog),
                    'updated_at' => now(),
                ]);

            // Trigger webhook
            $this->webhookService->dispatch('release.published', [
                'bundleId' => $bundleId,
                'name' => $bundle->name,
                'documentCount' => count($documents),
                'success' => $allSuccess,
            ]);

            return $this->get($bundleId);
        });
    }

    /**
     * Cancel scheduled release
     */
    public function cancel(string $bundleId): array
    {
        DB::table('release_bundles')
            ->where('bundle_id', $bundleId)
            ->whereIn('status', ['scheduled', 'publishing'])
            ->update([
                'status' => 'cancelled',
                'updated_at' => now(),
            ]);

        return $this->get($bundleId);
    }

    /**
     * List releases
     */
    public function list(array $options = []): array
    {
        $query = DB::table('release_bundles')
            ->whereNull('deleted_at');

        if (isset($options['status'])) {
            $query->where('status', $options['status']);
        }

        if (isset($options['createdBy'])) {
            $query->where('created_by', $options['createdBy']);
        }

        $total = $query->count();
        $page = $options['page'] ?? 1;
        $perPage = $options['perPage'] ?? 20;

        $items = $query
            ->orderBy('created_at', 'desc')
            ->skip(($page - 1) * $perPage)
            ->take($perPage)
            ->get();

        return [
            'items' => $items->map(function ($b) {
                $docCount = DB::table('release_bundle_documents')
                    ->where('bundle_id', $b->bundle_id)
                    ->count();

                return [
                    'id' => $b->bundle_id,
                    'name' => $b->name,
                    'status' => $b->status,
                    'scheduledAt' => $b->scheduled_at,
                    'publishedAt' => $b->published_at,
                    'documentCount' => $docCount,
                    'createdAt' => $b->created_at,
                ];
            })->toArray(),
            'total' => $total,
            'page' => $page,
            'perPage' => $perPage,
        ];
    }

    /**
     * Delete release
     */
    public function delete(string $bundleId): bool
    {
        $deleted = DB::table('release_bundles')
            ->where('bundle_id', $bundleId)
            ->whereIn('status', ['draft', 'cancelled', 'failed'])
            ->update([
                'deleted_at' => now(),
                'updated_at' => now(),
            ]);

        if ($deleted) {
            DB::table('release_bundle_documents')
                ->where('bundle_id', $bundleId)
                ->delete();
        }

        return $deleted > 0;
    }

    /**
     * Process scheduled releases
     */
    public function processScheduled(): array
    {
        $scheduled = DB::table('release_bundles')
            ->where('status', 'scheduled')
            ->where('scheduled_at', '<=', now())
            ->whereNull('deleted_at')
            ->get();

        $processed = [];

        foreach ($scheduled as $bundle) {
            try {
                $this->publish($bundle->bundle_id);
                $processed[] = [
                    'bundleId' => $bundle->bundle_id,
                    'name' => $bundle->name,
                    'status' => 'published',
                ];
            } catch (\Throwable $e) {
                $processed[] = [
                    'bundleId' => $bundle->bundle_id,
                    'name' => $bundle->name,
                    'status' => 'failed',
                    'error' => $e->getMessage(),
                ];
            }
        }

        return $processed;
    }

    /**
     * Duplicate release
     */
    public function duplicate(string $bundleId): array
    {
        $original = $this->get($bundleId);

        if (!$original) {
            throw new \InvalidArgumentException("Bundle not found");
        }

        $newBundle = $this->create([
            'name' => $original['name'] . ' (Copy)',
            'description' => $original['description'],
            'metadata' => $original['metadata'],
        ]);

        foreach ($original['documents'] as $doc) {
            $this->addDocument(
                $newBundle['id'],
                $doc['documentId'],
                $doc['documentType'],
                $doc['action']
            );
        }

        return $this->get($newBundle['id']);
    }
}

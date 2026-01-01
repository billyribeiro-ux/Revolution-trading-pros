<?php

declare(strict_types=1);

namespace App\Services\ContentLake;

use App\Models\Post;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Str;
use InvalidArgumentException;

/**
 * Document Perspective Service
 *
 * Implements Sanity-style draft/published document perspectives.
 * Each document exists as a pair: draft and published versions.
 *
 * Features:
 * - Draft/Published document pairs
 * - Preview tokens for sharing drafts
 * - Perspective switching
 * - Diff between draft and published
 * - Atomic publishing
 */
class DocumentPerspectiveService
{
    private const PREVIEW_TOKEN_LENGTH = 64;
    private const PREVIEW_TOKEN_TTL = 86400; // 24 hours

    private array $documentTypeMap = [
        'post' => Post::class,
        'category' => \App\Models\Category::class,
        'product' => \App\Models\Product::class,
    ];

    /**
     * Get or create perspective record for a document
     */
    public function getOrCreatePerspective(Model $document): array
    {
        $documentType = $this->getDocumentType($document);
        $documentId = $document->document_id ?? Str::uuid()->toString();

        // Ensure document has document_id
        if (!$document->document_id) {
            $document->update(['document_id' => $documentId]);
        }

        $perspective = DB::table('document_perspectives')
            ->where('document_id', $documentId)
            ->first();

        if (!$perspective) {
            $content = $document->toArray();

            $perspectiveId = DB::table('document_perspectives')->insertGetId([
                'document_id' => $documentId,
                'document_type' => $documentType,
                'source_id' => $document->id,
                'perspective' => 'draft',
                'draft_content' => json_encode($content),
                'published_content' => $document->status === 'published' ? json_encode($content) : null,
                'draft_updated_at' => now(),
                'published_at' => $document->status === 'published' ? $document->published_at : null,
                'draft_by' => auth()->id(),
                'published_by' => $document->status === 'published' ? auth()->id() : null,
                'has_unpublished_changes' => $document->status !== 'published',
                'created_at' => now(),
                'updated_at' => now(),
            ]);

            $perspective = DB::table('document_perspectives')
                ->where('id', $perspectiveId)
                ->first();
        }

        return (array) $perspective;
    }

    /**
     * Update draft content
     */
    public function updateDraft(string $documentId, array $content, ?int $userId = null): array
    {
        $perspective = DB::table('document_perspectives')
            ->where('document_id', $documentId)
            ->first();

        if (!$perspective) {
            throw new InvalidArgumentException("Document perspective not found: {$documentId}");
        }

        // Calculate if there are unpublished changes
        $publishedContent = $perspective->published_content
            ? json_decode($perspective->published_content, true)
            : null;

        $hasChanges = $publishedContent === null ||
            json_encode($content) !== json_encode($publishedContent);

        DB::table('document_perspectives')
            ->where('document_id', $documentId)
            ->update([
                'draft_content' => json_encode($content),
                'draft_updated_at' => now(),
                'draft_by' => $userId ?? auth()->id(),
                'has_unpublished_changes' => $hasChanges,
                'updated_at' => now(),
            ]);

        // Create history entry
        app(DocumentHistoryService::class)->recordChange(
            $documentId,
            $perspective->document_type,
            $content,
            'update',
            $userId
        );

        return $this->getDocument($documentId, 'draft');
    }

    /**
     * Publish draft to published
     */
    public function publish(string $documentId, ?int $userId = null): array
    {
        return DB::transaction(function () use ($documentId, $userId) {
            $perspective = DB::table('document_perspectives')
                ->where('document_id', $documentId)
                ->lockForUpdate()
                ->first();

            if (!$perspective) {
                throw new InvalidArgumentException("Document perspective not found: {$documentId}");
            }

            $draftContent = json_decode($perspective->draft_content, true);

            // Update perspective
            DB::table('document_perspectives')
                ->where('document_id', $documentId)
                ->update([
                    'published_content' => $perspective->draft_content,
                    'published_at' => now(),
                    'published_by' => $userId ?? auth()->id(),
                    'has_unpublished_changes' => false,
                    'updated_at' => now(),
                ]);

            // Update source document
            $modelClass = $this->documentTypeMap[$perspective->document_type] ?? null;
            if ($modelClass) {
                $model = $modelClass::find($perspective->source_id);
                if ($model) {
                    $this->applyContentToModel($model, $draftContent);
                    $model->status = 'published';
                    $model->published_at = now();
                    $model->save();
                }
            }

            // Create history entry
            app(DocumentHistoryService::class)->recordChange(
                $documentId,
                $perspective->document_type,
                $draftContent,
                'publish',
                $userId
            );

            // Trigger webhooks
            app(WebhookService::class)->dispatch('document.published', [
                'documentId' => $documentId,
                'documentType' => $perspective->document_type,
                'content' => $draftContent,
            ]);

            return $this->getDocument($documentId, 'published');
        });
    }

    /**
     * Unpublish document
     */
    public function unpublish(string $documentId, ?int $userId = null): array
    {
        return DB::transaction(function () use ($documentId, $userId) {
            $perspective = DB::table('document_perspectives')
                ->where('document_id', $documentId)
                ->lockForUpdate()
                ->first();

            if (!$perspective) {
                throw new InvalidArgumentException("Document perspective not found: {$documentId}");
            }

            // Keep published content for reference but mark as unpublished
            DB::table('document_perspectives')
                ->where('document_id', $documentId)
                ->update([
                    'has_unpublished_changes' => true,
                    'updated_at' => now(),
                ]);

            // Update source document
            $modelClass = $this->documentTypeMap[$perspective->document_type] ?? null;
            if ($modelClass) {
                $model = $modelClass::find($perspective->source_id);
                if ($model) {
                    $model->status = 'draft';
                    $model->save();
                }
            }

            // Create history entry
            app(DocumentHistoryService::class)->recordChange(
                $documentId,
                $perspective->document_type,
                json_decode($perspective->draft_content, true),
                'unpublish',
                $userId
            );

            // Trigger webhooks
            app(WebhookService::class)->dispatch('document.unpublished', [
                'documentId' => $documentId,
                'documentType' => $perspective->document_type,
            ]);

            return $this->getDocument($documentId, 'draft');
        });
    }

    /**
     * Discard draft changes (revert to published)
     */
    public function discardDraft(string $documentId, ?int $userId = null): array
    {
        $perspective = DB::table('document_perspectives')
            ->where('document_id', $documentId)
            ->first();

        if (!$perspective) {
            throw new InvalidArgumentException("Document perspective not found: {$documentId}");
        }

        if (!$perspective->published_content) {
            throw new InvalidArgumentException("No published version to revert to");
        }

        DB::table('document_perspectives')
            ->where('document_id', $documentId)
            ->update([
                'draft_content' => $perspective->published_content,
                'draft_updated_at' => now(),
                'draft_by' => $userId ?? auth()->id(),
                'has_unpublished_changes' => false,
                'updated_at' => now(),
            ]);

        return $this->getDocument($documentId, 'draft');
    }

    /**
     * Get document by perspective
     */
    public function getDocument(string $documentId, string $perspectiveType = 'draft'): array
    {
        $perspective = DB::table('document_perspectives')
            ->where('document_id', $documentId)
            ->first();

        if (!$perspective) {
            throw new InvalidArgumentException("Document perspective not found: {$documentId}");
        }

        $content = $perspectiveType === 'published'
            ? json_decode($perspective->published_content ?? '{}', true)
            : json_decode($perspective->draft_content ?? '{}', true);

        return [
            '_id' => $documentId,
            '_type' => $perspective->document_type,
            '_perspective' => $perspectiveType,
            '_hasUnpublishedChanges' => (bool) $perspective->has_unpublished_changes,
            '_createdAt' => $perspective->created_at,
            '_updatedAt' => $perspectiveType === 'published'
                ? $perspective->published_at
                : $perspective->draft_updated_at,
            ...$content,
        ];
    }

    /**
     * Get diff between draft and published
     */
    public function getDiff(string $documentId): array
    {
        $perspective = DB::table('document_perspectives')
            ->where('document_id', $documentId)
            ->first();

        if (!$perspective) {
            throw new InvalidArgumentException("Document perspective not found: {$documentId}");
        }

        $draft = json_decode($perspective->draft_content ?? '{}', true);
        $published = json_decode($perspective->published_content ?? '{}', true);

        return $this->calculateDiff($published, $draft);
    }

    /**
     * Calculate diff between two documents
     */
    private function calculateDiff(array $old, array $new, string $path = ''): array
    {
        $diff = [];

        // Find changed and added fields
        foreach ($new as $key => $value) {
            $fieldPath = $path ? "{$path}.{$key}" : $key;

            if (!array_key_exists($key, $old)) {
                $diff[] = [
                    'op' => 'add',
                    'path' => $fieldPath,
                    'value' => $value,
                ];
            } elseif (is_array($value) && is_array($old[$key])) {
                $nestedDiff = $this->calculateDiff($old[$key], $value, $fieldPath);
                $diff = array_merge($diff, $nestedDiff);
            } elseif ($old[$key] !== $value) {
                $diff[] = [
                    'op' => 'replace',
                    'path' => $fieldPath,
                    'oldValue' => $old[$key],
                    'value' => $value,
                ];
            }
        }

        // Find removed fields
        foreach ($old as $key => $value) {
            if (!array_key_exists($key, $new)) {
                $fieldPath = $path ? "{$path}.{$key}" : $key;
                $diff[] = [
                    'op' => 'remove',
                    'path' => $fieldPath,
                    'oldValue' => $value,
                ];
            }
        }

        return $diff;
    }

    /**
     * Create preview token
     */
    public function createPreviewToken(
        string $documentId,
        ?int $userId = null,
        ?int $ttlSeconds = null
    ): string {
        $token = Str::random(self::PREVIEW_TOKEN_LENGTH);
        $expiresAt = now()->addSeconds($ttlSeconds ?? self::PREVIEW_TOKEN_TTL);

        DB::table('document_perspectives')
            ->where('document_id', $documentId)
            ->update([
                'preview_token' => $token,
                'preview_token_expires_at' => $expiresAt,
                'updated_at' => now(),
            ]);

        // Also create a preview session
        DB::table('preview_sessions')->insert([
            'token' => $token,
            'document_id' => $documentId,
            'document_type' => DB::table('document_perspectives')
                ->where('document_id', $documentId)
                ->value('document_type'),
            'user_id' => $userId ?? auth()->id(),
            'perspective' => 'previewDrafts',
            'expires_at' => $expiresAt,
            'created_at' => now(),
            'updated_at' => now(),
        ]);

        return $token;
    }

    /**
     * Validate preview token
     */
    public function validatePreviewToken(string $token): ?array
    {
        $session = DB::table('preview_sessions')
            ->where('token', $token)
            ->where('expires_at', '>', now())
            ->first();

        if (!$session) {
            return null;
        }

        return [
            'documentId' => $session->document_id,
            'documentType' => $session->document_type,
            'perspective' => $session->perspective,
            'expiresAt' => $session->expires_at,
        ];
    }

    /**
     * Get document by preview token
     */
    public function getByPreviewToken(string $token): ?array
    {
        $session = $this->validatePreviewToken($token);

        if (!$session) {
            return null;
        }

        return $this->getDocument($session['documentId'], 'draft');
    }

    /**
     * List documents with perspective info
     */
    public function listDocuments(
        string $type,
        string $perspective = 'draft',
        array $options = []
    ): array {
        $query = DB::table('document_perspectives')
            ->where('document_type', $type);

        if ($perspective === 'published') {
            $query->whereNotNull('published_content');
        }

        if (isset($options['hasUnpublishedChanges'])) {
            $query->where('has_unpublished_changes', $options['hasUnpublishedChanges']);
        }

        // Pagination
        $page = $options['page'] ?? 1;
        $perPage = $options['perPage'] ?? 20;

        $total = $query->count();
        $items = $query
            ->orderBy('updated_at', 'desc')
            ->skip(($page - 1) * $perPage)
            ->take($perPage)
            ->get();

        return [
            'items' => $items->map(function ($item) use ($perspective) {
                $content = $perspective === 'published'
                    ? json_decode($item->published_content ?? '{}', true)
                    : json_decode($item->draft_content ?? '{}', true);

                return [
                    '_id' => $item->document_id,
                    '_type' => $item->document_type,
                    '_hasUnpublishedChanges' => (bool) $item->has_unpublished_changes,
                    ...$content,
                ];
            })->toArray(),
            'total' => $total,
            'page' => $page,
            'perPage' => $perPage,
            'totalPages' => (int) ceil($total / $perPage),
        ];
    }

    /**
     * Get document type from model
     */
    private function getDocumentType(Model $model): string
    {
        $class = get_class($model);
        return array_search($class, $this->documentTypeMap, true) ?:
            Str::snake(class_basename($model));
    }

    /**
     * Apply content array to model
     */
    private function applyContentToModel(Model $model, array $content): void
    {
        // Remove internal fields
        unset($content['_id'], $content['_type'], $content['_perspective']);
        unset($content['_hasUnpublishedChanges'], $content['_createdAt'], $content['_updatedAt']);

        foreach ($content as $key => $value) {
            if ($model->isFillable($key) || in_array($key, $model->getFillable(), true)) {
                $model->{$key} = $value;
            }
        }
    }

    /**
     * Register document type mapping
     */
    public function registerType(string $type, string $modelClass): self
    {
        $this->documentTypeMap[$type] = $modelClass;
        return $this;
    }
}

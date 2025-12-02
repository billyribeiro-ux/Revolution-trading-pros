<?php

declare(strict_types=1);

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Services\ContentLake\DocumentHistoryService;
use App\Services\ContentLake\DocumentPerspectiveService;
use App\Services\ContentLake\GroqQueryService;
use App\Services\ContentLake\ImageHotspotService;
use App\Services\ContentLake\PortableTextService;
use App\Services\ContentLake\ReleaseBundleService;
use App\Services\ContentLake\SchemaService;
use App\Services\ContentLake\WebhookService;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;

/**
 * Content Lake Controller
 *
 * Unified API for all Sanity-equivalent CMS features.
 */
class ContentLakeController extends Controller
{
    public function __construct(
        private readonly GroqQueryService $groqService,
        private readonly PortableTextService $portableTextService,
        private readonly DocumentPerspectiveService $perspectiveService,
        private readonly DocumentHistoryService $historyService,
        private readonly WebhookService $webhookService,
        private readonly ImageHotspotService $hotspotService,
        private readonly SchemaService $schemaService,
        private readonly ReleaseBundleService $releaseService,
    ) {}

    // ═══════════════════════════════════════════════════════════════════════
    // GROQ QUERY API
    // ═══════════════════════════════════════════════════════════════════════

    /**
     * Execute GROQ query
     */
    public function query(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'query' => 'required|string|max:10000',
            'params' => 'nullable|array',
            'useCache' => 'nullable|boolean',
        ]);

        try {
            $result = $this->groqService->query(
                $validated['query'],
                $validated['params'] ?? [],
                $validated['useCache'] ?? true
            );

            return response()->json($result->toArray());
        } catch (\Throwable $e) {
            return response()->json([
                'error' => $e->getMessage(),
                'type' => 'QueryError',
            ], 400);
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PORTABLE TEXT API
    // ═══════════════════════════════════════════════════════════════════════

    /**
     * Render Portable Text to HTML
     */
    public function renderPortableText(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'blocks' => 'required|array',
        ]);

        try {
            $parsed = $this->portableTextService->parse($validated['blocks']);
            $html = $this->portableTextService->toHtml($parsed);
            $plainText = $this->portableTextService->toPlainText($parsed);

            return response()->json([
                'html' => $html,
                'plainText' => $plainText,
                'wordCount' => $this->portableTextService->wordCount($parsed),
                'readingTime' => $this->portableTextService->readingTime($parsed),
            ]);
        } catch (\Throwable $e) {
            return response()->json(['error' => $e->getMessage()], 400);
        }
    }

    /**
     * Validate Portable Text
     */
    public function validatePortableText(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'blocks' => 'required|array',
        ]);

        $result = $this->portableTextService->validate($validated['blocks']);

        return response()->json($result->toArray());
    }

    // ═══════════════════════════════════════════════════════════════════════
    // DOCUMENT PERSPECTIVE API
    // ═══════════════════════════════════════════════════════════════════════

    /**
     * Get document by ID
     */
    public function getDocument(Request $request, string $documentId): JsonResponse
    {
        $perspective = $request->query('perspective', 'draft');

        try {
            $document = $this->perspectiveService->getDocument($documentId, $perspective);
            return response()->json(['data' => $document]);
        } catch (\Throwable $e) {
            return response()->json(['error' => $e->getMessage()], 404);
        }
    }

    /**
     * Update draft
     */
    public function updateDraft(Request $request, string $documentId): JsonResponse
    {
        $validated = $request->validate([
            'content' => 'required|array',
        ]);

        try {
            $document = $this->perspectiveService->updateDraft(
                $documentId,
                $validated['content'],
                auth()->id()
            );

            return response()->json(['data' => $document]);
        } catch (\Throwable $e) {
            return response()->json(['error' => $e->getMessage()], 400);
        }
    }

    /**
     * Publish document
     */
    public function publishDocument(string $documentId): JsonResponse
    {
        try {
            $document = $this->perspectiveService->publish($documentId, auth()->id());
            return response()->json(['data' => $document]);
        } catch (\Throwable $e) {
            return response()->json(['error' => $e->getMessage()], 400);
        }
    }

    /**
     * Unpublish document
     */
    public function unpublishDocument(string $documentId): JsonResponse
    {
        try {
            $document = $this->perspectiveService->unpublish($documentId, auth()->id());
            return response()->json(['data' => $document]);
        } catch (\Throwable $e) {
            return response()->json(['error' => $e->getMessage()], 400);
        }
    }

    /**
     * Get document diff
     */
    public function getDocumentDiff(string $documentId): JsonResponse
    {
        try {
            $diff = $this->perspectiveService->getDiff($documentId);
            return response()->json(['data' => $diff]);
        } catch (\Throwable $e) {
            return response()->json(['error' => $e->getMessage()], 400);
        }
    }

    /**
     * Create preview token
     */
    public function createPreviewToken(Request $request, string $documentId): JsonResponse
    {
        $validated = $request->validate([
            'ttl' => 'nullable|integer|min:60|max:604800', // 1 minute to 7 days
        ]);

        try {
            $token = $this->perspectiveService->createPreviewToken(
                $documentId,
                auth()->id(),
                $validated['ttl'] ?? null
            );

            return response()->json([
                'token' => $token,
                'previewUrl' => url("/preview?token={$token}"),
            ]);
        } catch (\Throwable $e) {
            return response()->json(['error' => $e->getMessage()], 400);
        }
    }

    /**
     * Get document by preview token (public)
     */
    public function getByPreviewToken(Request $request): JsonResponse
    {
        $token = $request->query('token');

        if (!$token) {
            return response()->json(['error' => 'Token required'], 400);
        }

        $document = $this->perspectiveService->getByPreviewToken($token);

        if (!$document) {
            return response()->json(['error' => 'Invalid or expired token'], 404);
        }

        return response()->json(['data' => $document]);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // DOCUMENT HISTORY API
    // ═══════════════════════════════════════════════════════════════════════

    /**
     * Get document history
     */
    public function getHistory(Request $request, string $documentId): JsonResponse
    {
        $limit = $request->query('limit', 50);
        $offset = $request->query('offset', 0);

        $history = $this->historyService->getHistory($documentId, (int) $limit, (int) $offset);

        return response()->json($history);
    }

    /**
     * Get specific revision
     */
    public function getRevision(string $documentId, int $revisionNumber): JsonResponse
    {
        $revision = $this->historyService->getRevision($documentId, $revisionNumber);

        if (!$revision) {
            return response()->json(['error' => 'Revision not found'], 404);
        }

        return response()->json(['data' => $revision]);
    }

    /**
     * Restore to revision
     */
    public function restoreRevision(string $documentId, int $revisionNumber): JsonResponse
    {
        try {
            $revision = $this->historyService->restore($documentId, $revisionNumber, auth()->id());
            return response()->json(['data' => $revision]);
        } catch (\Throwable $e) {
            return response()->json(['error' => $e->getMessage()], 400);
        }
    }

    /**
     * Compare revisions
     */
    public function compareRevisions(Request $request, string $documentId): JsonResponse
    {
        $validated = $request->validate([
            'from' => 'required|integer|min:1',
            'to' => 'required|integer|min:1',
        ]);

        try {
            $comparison = $this->historyService->compareRevisions(
                $documentId,
                $validated['from'],
                $validated['to']
            );

            return response()->json(['data' => $comparison]);
        } catch (\Throwable $e) {
            return response()->json(['error' => $e->getMessage()], 400);
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // WEBHOOKS API
    // ═══════════════════════════════════════════════════════════════════════

    /**
     * List webhooks
     */
    public function listWebhooks(Request $request): JsonResponse
    {
        $options = $request->only(['page', 'perPage', 'is_active']);
        return response()->json($this->webhookService->list($options));
    }

    /**
     * Create webhook
     */
    public function createWebhook(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'name' => 'required|string|max:255',
            'url' => 'required|url|max:2048',
            'events' => 'required|array|min:1',
            'events.*' => 'string',
            'filter' => 'nullable|array',
            'projection' => 'nullable|array',
            'headers' => 'nullable|array',
            'is_active' => 'nullable|boolean',
        ]);

        try {
            $webhook = $this->webhookService->create($validated);
            return response()->json(['data' => $webhook], 201);
        } catch (\Throwable $e) {
            return response()->json(['error' => $e->getMessage()], 400);
        }
    }

    /**
     * Update webhook
     */
    public function updateWebhook(Request $request, string $webhookId): JsonResponse
    {
        $validated = $request->validate([
            'name' => 'nullable|string|max:255',
            'url' => 'nullable|url|max:2048',
            'events' => 'nullable|array',
            'filter' => 'nullable|array',
            'is_active' => 'nullable|boolean',
        ]);

        $success = $this->webhookService->update($webhookId, $validated);

        return response()->json(['success' => $success]);
    }

    /**
     * Delete webhook
     */
    public function deleteWebhook(string $webhookId): JsonResponse
    {
        $success = $this->webhookService->delete($webhookId);
        return response()->json(['success' => $success]);
    }

    /**
     * Test webhook
     */
    public function testWebhook(string $webhookId): JsonResponse
    {
        try {
            $result = $this->webhookService->test($webhookId);
            return response()->json($result);
        } catch (\Throwable $e) {
            return response()->json(['error' => $e->getMessage()], 400);
        }
    }

    /**
     * Get webhook deliveries
     */
    public function getWebhookDeliveries(Request $request, string $webhookId): JsonResponse
    {
        $options = $request->only(['page', 'perPage', 'status']);
        return response()->json($this->webhookService->getDeliveries($webhookId, $options));
    }

    // ═══════════════════════════════════════════════════════════════════════
    // IMAGE HOTSPOT API
    // ═══════════════════════════════════════════════════════════════════════

    /**
     * Set image hotspot
     */
    public function setHotspot(Request $request, int $mediaId): JsonResponse
    {
        $validated = $request->validate([
            'x' => 'required|numeric|min:0|max:1',
            'y' => 'required|numeric|min:0|max:1',
            'name' => 'nullable|string|max:100',
        ]);

        try {
            $hotspot = $this->hotspotService->setHotspot(
                $mediaId,
                $validated['x'],
                $validated['y'],
                $validated['name'] ?? null
            );

            return response()->json(['data' => $hotspot]);
        } catch (\Throwable $e) {
            return response()->json(['error' => $e->getMessage()], 400);
        }
    }

    /**
     * Set image crop
     */
    public function setCrop(Request $request, int $mediaId): JsonResponse
    {
        $validated = $request->validate([
            'name' => 'required|string|max:100',
            'top' => 'required|numeric|min:0|max:1',
            'left' => 'required|numeric|min:0|max:1',
            'bottom' => 'required|numeric|min:0|max:1',
            'right' => 'required|numeric|min:0|max:1',
            'aspectRatio' => 'nullable|string',
        ]);

        try {
            $crop = $this->hotspotService->setCrop(
                $mediaId,
                $validated['name'],
                $validated['top'],
                $validated['left'],
                $validated['bottom'],
                $validated['right'],
                $validated['aspectRatio'] ?? null
            );

            return response()->json(['data' => $crop]);
        } catch (\Throwable $e) {
            return response()->json(['error' => $e->getMessage()], 400);
        }
    }

    /**
     * Get image URL with transformations
     */
    public function getImageUrl(Request $request, int $mediaId): JsonResponse
    {
        $options = $request->only([
            'width', 'height', 'fit', 'crop', 'hotspot',
            'format', 'quality', 'blur', 'dpr'
        ]);

        try {
            $url = $this->hotspotService->buildUrl($mediaId, $options);

            return response()->json([
                'url' => $url,
                'srcset' => $this->hotspotService->getSrcSet($mediaId, [320, 640, 1280], $options),
            ]);
        } catch (\Throwable $e) {
            return response()->json(['error' => $e->getMessage()], 400);
        }
    }

    /**
     * Generate LQIP and BlurHash
     */
    public function generatePlaceholders(int $mediaId): JsonResponse
    {
        try {
            $lqip = $this->hotspotService->generateLqip($mediaId);
            $blurhash = $this->hotspotService->generateBlurHash($mediaId);

            return response()->json([
                'lqip' => $lqip,
                'blurhash' => $blurhash,
            ]);
        } catch (\Throwable $e) {
            return response()->json(['error' => $e->getMessage()], 400);
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // SCHEMA API
    // ═══════════════════════════════════════════════════════════════════════

    /**
     * List schemas
     */
    public function listSchemas(Request $request): JsonResponse
    {
        $options = $request->only(['type', 'system']);
        return response()->json(['data' => $this->schemaService->list($options)]);
    }

    /**
     * Get schema
     */
    public function getSchema(string $name): JsonResponse
    {
        $schema = $this->schemaService->get($name);

        if (!$schema) {
            return response()->json(['error' => 'Schema not found'], 404);
        }

        return response()->json(['data' => $schema]);
    }

    /**
     * Register schema
     */
    public function registerSchema(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'name' => 'required|string|max:100',
            'title' => 'nullable|string|max:255',
            'fields' => 'required|array|min:1',
            'fields.*.name' => 'required|string',
            'fields.*.type' => 'required|string',
            'fieldsets' => 'nullable|array',
            'preview' => 'nullable|array',
            'orderings' => 'nullable|array',
            'validation' => 'nullable|array',
            'initialValue' => 'nullable|array',
        ]);

        try {
            $schema = $this->schemaService->register($validated);
            return response()->json(['data' => $schema], 201);
        } catch (\Throwable $e) {
            return response()->json(['error' => $e->getMessage()], 400);
        }
    }

    /**
     * Validate document against schema
     */
    public function validateAgainstSchema(Request $request, string $schemaName): JsonResponse
    {
        $validated = $request->validate([
            'document' => 'required|array',
        ]);

        try {
            $result = $this->schemaService->validate($schemaName, $validated['document']);
            return response()->json($result);
        } catch (\Throwable $e) {
            return response()->json(['error' => $e->getMessage()], 400);
        }
    }

    /**
     * Generate TypeScript for schema
     */
    public function generateTypeScript(string $schemaName): JsonResponse
    {
        try {
            $typescript = $this->schemaService->generateTypeScript($schemaName);
            return response()->json(['typescript' => $typescript]);
        } catch (\Throwable $e) {
            return response()->json(['error' => $e->getMessage()], 400);
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // RELEASE BUNDLES API
    // ═══════════════════════════════════════════════════════════════════════

    /**
     * List releases
     */
    public function listReleases(Request $request): JsonResponse
    {
        $options = $request->only(['page', 'perPage', 'status', 'createdBy']);
        return response()->json($this->releaseService->list($options));
    }

    /**
     * Get release
     */
    public function getRelease(string $bundleId): JsonResponse
    {
        $release = $this->releaseService->get($bundleId);

        if (!$release) {
            return response()->json(['error' => 'Release not found'], 404);
        }

        return response()->json(['data' => $release]);
    }

    /**
     * Create release
     */
    public function createRelease(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'name' => 'required|string|max:255',
            'description' => 'nullable|string',
            'scheduled_at' => 'nullable|date|after:now',
            'metadata' => 'nullable|array',
        ]);

        try {
            $release = $this->releaseService->create($validated);
            return response()->json(['data' => $release], 201);
        } catch (\Throwable $e) {
            return response()->json(['error' => $e->getMessage()], 400);
        }
    }

    /**
     * Add document to release
     */
    public function addDocumentToRelease(Request $request, string $bundleId): JsonResponse
    {
        $validated = $request->validate([
            'documentId' => 'required|string',
            'documentType' => 'required|string',
            'action' => 'nullable|in:create,update,delete',
        ]);

        try {
            $this->releaseService->addDocument(
                $bundleId,
                $validated['documentId'],
                $validated['documentType'],
                $validated['action'] ?? 'update'
            );

            return response()->json(['success' => true]);
        } catch (\Throwable $e) {
            return response()->json(['error' => $e->getMessage()], 400);
        }
    }

    /**
     * Remove document from release
     */
    public function removeDocumentFromRelease(string $bundleId, string $documentId): JsonResponse
    {
        $success = $this->releaseService->removeDocument($bundleId, $documentId);
        return response()->json(['success' => $success]);
    }

    /**
     * Schedule release
     */
    public function scheduleRelease(Request $request, string $bundleId): JsonResponse
    {
        $validated = $request->validate([
            'scheduled_at' => 'required|date|after:now',
        ]);

        try {
            $release = $this->releaseService->schedule($bundleId, $validated['scheduled_at']);
            return response()->json(['data' => $release]);
        } catch (\Throwable $e) {
            return response()->json(['error' => $e->getMessage()], 400);
        }
    }

    /**
     * Publish release
     */
    public function publishRelease(string $bundleId): JsonResponse
    {
        try {
            $release = $this->releaseService->publish($bundleId, auth()->id());
            return response()->json(['data' => $release]);
        } catch (\Throwable $e) {
            return response()->json(['error' => $e->getMessage()], 400);
        }
    }

    /**
     * Cancel release
     */
    public function cancelRelease(string $bundleId): JsonResponse
    {
        try {
            $release = $this->releaseService->cancel($bundleId);
            return response()->json(['data' => $release]);
        } catch (\Throwable $e) {
            return response()->json(['error' => $e->getMessage()], 400);
        }
    }

    /**
     * Delete release
     */
    public function deleteRelease(string $bundleId): JsonResponse
    {
        $success = $this->releaseService->delete($bundleId);
        return response()->json(['success' => $success]);
    }
}

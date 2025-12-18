<?php

declare(strict_types=1);

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Http\Requests\Api\ContentLake\AddDocumentToReleaseRequest;
use App\Http\Requests\Api\ContentLake\CompareRevisionsRequest;
use App\Http\Requests\Api\ContentLake\GroqQueryRequest;
use App\Http\Requests\Api\ContentLake\ImageCropRequest;
use App\Http\Requests\Api\ContentLake\ImageHotspotRequest;
use App\Http\Requests\Api\ContentLake\PortableTextRequest;
use App\Http\Requests\Api\ContentLake\PreviewTokenRequest;
use App\Http\Requests\Api\ContentLake\ReleaseRequest;
use App\Http\Requests\Api\ContentLake\ScheduleReleaseRequest;
use App\Http\Requests\Api\ContentLake\SchemaRequest;
use App\Http\Requests\Api\ContentLake\UpdateDraftRequest;
use App\Http\Requests\Api\ContentLake\WebhookRequest;
use App\Http\Traits\ApiResponse;
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
 * Enterprise-grade implementation with FormRequest validation and standardized responses.
 *
 * @level ICT11 Principal Engineer
 */
class ContentLakeController extends Controller
{
    use ApiResponse;

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
    public function query(GroqQueryRequest $request): JsonResponse
    {
        try {
            $result = $this->groqService->query(
                $request->validated('query'),
                $request->validated('params', []),
                $request->validated('useCache', true)
            );

            return $this->successResponse($result->toArray());
        } catch (\Throwable $e) {
            return $this->errorResponse($e->getMessage(), 400, [], 'QUERY_ERROR');
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PORTABLE TEXT API
    // ═══════════════════════════════════════════════════════════════════════

    /**
     * Render Portable Text to HTML
     */
    public function renderPortableText(PortableTextRequest $request): JsonResponse
    {
        try {
            $parsed = $this->portableTextService->parse($request->validated('blocks'));
            $html = $this->portableTextService->toHtml($parsed);
            $plainText = $this->portableTextService->toPlainText($parsed);

            return $this->successResponse([
                'html' => $html,
                'plainText' => $plainText,
                'wordCount' => $this->portableTextService->wordCount($parsed),
                'readingTime' => $this->portableTextService->readingTime($parsed),
            ]);
        } catch (\Throwable $e) {
            return $this->errorResponse($e->getMessage(), 400, [], 'RENDER_ERROR');
        }
    }

    /**
     * Validate Portable Text
     */
    public function validatePortableText(PortableTextRequest $request): JsonResponse
    {
        $result = $this->portableTextService->validate($request->validated('blocks'));

        return $this->successResponse($result->toArray());
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
            return $this->successResponse($document);
        } catch (\Throwable $e) {
            return $this->notFoundResponse('Document');
        }
    }

    /**
     * Update draft
     */
    public function updateDraft(UpdateDraftRequest $request, string $documentId): JsonResponse
    {
        try {
            $document = $this->perspectiveService->updateDraft(
                $documentId,
                $request->validated('content'),
                auth()->id()
            );

            return $this->successResponse($document, 'Draft updated successfully');
        } catch (\Throwable $e) {
            return $this->errorResponse($e->getMessage(), 400, [], 'UPDATE_ERROR');
        }
    }

    /**
     * Publish document
     */
    public function publishDocument(string $documentId): JsonResponse
    {
        try {
            $document = $this->perspectiveService->publish($documentId, auth()->id());
            return $this->successResponse($document, 'Document published successfully');
        } catch (\Throwable $e) {
            return $this->errorResponse($e->getMessage(), 400, [], 'PUBLISH_ERROR');
        }
    }

    /**
     * Unpublish document
     */
    public function unpublishDocument(string $documentId): JsonResponse
    {
        try {
            $document = $this->perspectiveService->unpublish($documentId, auth()->id());
            return $this->successResponse($document, 'Document unpublished successfully');
        } catch (\Throwable $e) {
            return $this->errorResponse($e->getMessage(), 400, [], 'UNPUBLISH_ERROR');
        }
    }

    /**
     * Get document diff
     */
    public function getDocumentDiff(string $documentId): JsonResponse
    {
        try {
            $diff = $this->perspectiveService->getDiff($documentId);
            return $this->successResponse($diff);
        } catch (\Throwable $e) {
            return $this->errorResponse($e->getMessage(), 400, [], 'DIFF_ERROR');
        }
    }

    /**
     * Create preview token
     */
    public function createPreviewToken(PreviewTokenRequest $request, string $documentId): JsonResponse
    {
        try {
            $token = $this->perspectiveService->createPreviewToken(
                $documentId,
                auth()->id(),
                $request->validated('ttl')
            );

            return $this->successResponse([
                'token' => $token,
                'previewUrl' => url("/preview?token={$token}"),
            ]);
        } catch (\Throwable $e) {
            return $this->errorResponse($e->getMessage(), 400, [], 'TOKEN_ERROR');
        }
    }

    /**
     * Get document by preview token (public)
     */
    public function getByPreviewToken(Request $request): JsonResponse
    {
        $token = $request->query('token');

        if (!$token) {
            return $this->errorResponse('Token required', 400, [], 'MISSING_TOKEN');
        }

        $document = $this->perspectiveService->getByPreviewToken($token);

        if (!$document) {
            return $this->errorResponse('Invalid or expired token', 404, [], 'INVALID_TOKEN');
        }

        return $this->successResponse($document);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // DOCUMENT HISTORY API
    // ═══════════════════════════════════════════════════════════════════════

    /**
     * Get document history
     */
    public function getHistory(Request $request, string $documentId): JsonResponse
    {
        $limit = min((int) $request->query('limit', 50), 100);
        $offset = max((int) $request->query('offset', 0), 0);

        $history = $this->historyService->getHistory($documentId, $limit, $offset);

        return $this->successResponse($history);
    }

    /**
     * Get specific revision
     */
    public function getRevision(string $documentId, int $revisionNumber): JsonResponse
    {
        $revision = $this->historyService->getRevision($documentId, $revisionNumber);

        if (!$revision) {
            return $this->notFoundResponse('Revision');
        }

        return $this->successResponse($revision);
    }

    /**
     * Restore to revision
     */
    public function restoreRevision(string $documentId, int $revisionNumber): JsonResponse
    {
        try {
            $revision = $this->historyService->restore($documentId, $revisionNumber, auth()->id());
            return $this->successResponse($revision, 'Revision restored successfully');
        } catch (\Throwable $e) {
            return $this->errorResponse($e->getMessage(), 400, [], 'RESTORE_ERROR');
        }
    }

    /**
     * Compare revisions
     */
    public function compareRevisions(CompareRevisionsRequest $request, string $documentId): JsonResponse
    {
        try {
            $comparison = $this->historyService->compareRevisions(
                $documentId,
                $request->validated('from'),
                $request->validated('to')
            );

            return $this->successResponse($comparison);
        } catch (\Throwable $e) {
            return $this->errorResponse($e->getMessage(), 400, [], 'COMPARE_ERROR');
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
        return $this->successResponse($this->webhookService->list($options));
    }

    /**
     * Create webhook
     */
    public function createWebhook(WebhookRequest $request): JsonResponse
    {
        try {
            $webhook = $this->webhookService->create($request->validated());
            return $this->createdResponse($webhook, 'Webhook created successfully');
        } catch (\Throwable $e) {
            return $this->errorResponse($e->getMessage(), 400, [], 'CREATE_ERROR');
        }
    }

    /**
     * Update webhook
     */
    public function updateWebhook(WebhookRequest $request, string $webhookId): JsonResponse
    {
        $success = $this->webhookService->update($webhookId, $request->validated());

        if ($success) {
            return $this->successResponse(null, 'Webhook updated successfully');
        }

        return $this->notFoundResponse('Webhook');
    }

    /**
     * Delete webhook
     */
    public function deleteWebhook(string $webhookId): JsonResponse
    {
        $success = $this->webhookService->delete($webhookId);

        if ($success) {
            return $this->noContentResponse();
        }

        return $this->notFoundResponse('Webhook');
    }

    /**
     * Test webhook
     */
    public function testWebhook(string $webhookId): JsonResponse
    {
        try {
            $result = $this->webhookService->test($webhookId);
            return $this->successResponse($result);
        } catch (\Throwable $e) {
            return $this->errorResponse($e->getMessage(), 400, [], 'TEST_ERROR');
        }
    }

    /**
     * Get webhook deliveries
     */
    public function getWebhookDeliveries(Request $request, string $webhookId): JsonResponse
    {
        $options = $request->only(['page', 'perPage', 'status']);
        return $this->successResponse($this->webhookService->getDeliveries($webhookId, $options));
    }

    // ═══════════════════════════════════════════════════════════════════════
    // IMAGE HOTSPOT API
    // ═══════════════════════════════════════════════════════════════════════

    /**
     * Set image hotspot
     */
    public function setHotspot(ImageHotspotRequest $request, int $mediaId): JsonResponse
    {
        try {
            $hotspot = $this->hotspotService->setHotspot(
                $mediaId,
                $request->validated('x'),
                $request->validated('y'),
                $request->validated('name')
            );

            return $this->successResponse($hotspot);
        } catch (\Throwable $e) {
            return $this->errorResponse($e->getMessage(), 400, [], 'HOTSPOT_ERROR');
        }
    }

    /**
     * Set image crop
     */
    public function setCrop(ImageCropRequest $request, int $mediaId): JsonResponse
    {
        try {
            $crop = $this->hotspotService->setCrop(
                $mediaId,
                $request->validated('name'),
                $request->validated('top'),
                $request->validated('left'),
                $request->validated('bottom'),
                $request->validated('right'),
                $request->validated('aspectRatio')
            );

            return $this->successResponse($crop);
        } catch (\Throwable $e) {
            return $this->errorResponse($e->getMessage(), 400, [], 'CROP_ERROR');
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

            return $this->successResponse([
                'url' => $url,
                'srcset' => $this->hotspotService->getSrcSet($mediaId, [320, 640, 1280], $options),
            ]);
        } catch (\Throwable $e) {
            return $this->errorResponse($e->getMessage(), 400, [], 'URL_ERROR');
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

            return $this->successResponse([
                'lqip' => $lqip,
                'blurhash' => $blurhash,
            ]);
        } catch (\Throwable $e) {
            return $this->errorResponse($e->getMessage(), 400, [], 'PLACEHOLDER_ERROR');
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
        return $this->successResponse($this->schemaService->list($options));
    }

    /**
     * Get schema
     */
    public function getSchema(string $name): JsonResponse
    {
        $schema = $this->schemaService->get($name);

        if (!$schema) {
            return $this->notFoundResponse('Schema');
        }

        return $this->successResponse($schema);
    }

    /**
     * Register schema
     */
    public function registerSchema(SchemaRequest $request): JsonResponse
    {
        try {
            $schema = $this->schemaService->register($request->validated());
            return $this->createdResponse($schema, 'Schema registered successfully');
        } catch (\Throwable $e) {
            return $this->errorResponse($e->getMessage(), 400, [], 'SCHEMA_ERROR');
        }
    }

    /**
     * Validate document against schema
     */
    public function validateAgainstSchema(Request $request, string $schemaName): JsonResponse
    {
        $request->validate([
            'document' => 'required|array',
        ]);

        try {
            $result = $this->schemaService->validate($schemaName, $request->input('document'));
            return $this->successResponse($result);
        } catch (\Throwable $e) {
            return $this->errorResponse($e->getMessage(), 400, [], 'VALIDATE_ERROR');
        }
    }

    /**
     * Generate TypeScript for schema
     */
    public function generateTypeScript(string $schemaName): JsonResponse
    {
        try {
            $typescript = $this->schemaService->generateTypeScript($schemaName);
            return $this->successResponse(['typescript' => $typescript]);
        } catch (\Throwable $e) {
            return $this->errorResponse($e->getMessage(), 400, [], 'TYPESCRIPT_ERROR');
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
        return $this->successResponse($this->releaseService->list($options));
    }

    /**
     * Get release
     */
    public function getRelease(string $bundleId): JsonResponse
    {
        $release = $this->releaseService->get($bundleId);

        if (!$release) {
            return $this->notFoundResponse('Release');
        }

        return $this->successResponse($release);
    }

    /**
     * Create release
     */
    public function createRelease(ReleaseRequest $request): JsonResponse
    {
        try {
            $release = $this->releaseService->create($request->validated());
            return $this->createdResponse($release, 'Release created successfully');
        } catch (\Throwable $e) {
            return $this->errorResponse($e->getMessage(), 400, [], 'CREATE_ERROR');
        }
    }

    /**
     * Add document to release
     */
    public function addDocumentToRelease(AddDocumentToReleaseRequest $request, string $bundleId): JsonResponse
    {
        try {
            $this->releaseService->addDocument(
                $bundleId,
                $request->validated('documentId'),
                $request->validated('documentType'),
                $request->validated('action', 'update')
            );

            return $this->successResponse(null, 'Document added to release');
        } catch (\Throwable $e) {
            return $this->errorResponse($e->getMessage(), 400, [], 'ADD_DOCUMENT_ERROR');
        }
    }

    /**
     * Remove document from release
     */
    public function removeDocumentFromRelease(string $bundleId, string $documentId): JsonResponse
    {
        $success = $this->releaseService->removeDocument($bundleId, $documentId);

        if ($success) {
            return $this->successResponse(null, 'Document removed from release');
        }

        return $this->notFoundResponse('Document in release');
    }

    /**
     * Schedule release
     */
    public function scheduleRelease(ScheduleReleaseRequest $request, string $bundleId): JsonResponse
    {
        try {
            $release = $this->releaseService->schedule($bundleId, $request->validated('scheduled_at'));
            return $this->successResponse($release, 'Release scheduled successfully');
        } catch (\Throwable $e) {
            return $this->errorResponse($e->getMessage(), 400, [], 'SCHEDULE_ERROR');
        }
    }

    /**
     * Publish release
     */
    public function publishRelease(string $bundleId): JsonResponse
    {
        try {
            $release = $this->releaseService->publish($bundleId, auth()->id());
            return $this->successResponse($release, 'Release published successfully');
        } catch (\Throwable $e) {
            return $this->errorResponse($e->getMessage(), 400, [], 'PUBLISH_ERROR');
        }
    }

    /**
     * Cancel release
     */
    public function cancelRelease(string $bundleId): JsonResponse
    {
        try {
            $release = $this->releaseService->cancel($bundleId);
            return $this->successResponse($release, 'Release cancelled');
        } catch (\Throwable $e) {
            return $this->errorResponse($e->getMessage(), 400, [], 'CANCEL_ERROR');
        }
    }

    /**
     * Delete release
     */
    public function deleteRelease(string $bundleId): JsonResponse
    {
        $success = $this->releaseService->delete($bundleId);

        if ($success) {
            return $this->noContentResponse();
        }

        return $this->notFoundResponse('Release');
    }
}

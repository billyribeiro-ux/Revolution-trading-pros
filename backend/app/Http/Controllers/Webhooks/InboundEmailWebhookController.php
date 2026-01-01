<?php

declare(strict_types=1);

namespace App\Http\Controllers\Webhooks;

use App\Http\Controllers\Controller;
use App\Services\Email\InboundEmailService;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\RateLimiter;
use Throwable;

/**
 * InboundEmailWebhookController
 *
 * Handles inbound email webhooks from email service providers.
 * Implements security verification, rate limiting, and idempotency.
 *
 * Supported Providers:
 * - Postmark
 * - AWS SES (via SNS)
 * - SendGrid
 *
 * Security Features:
 * - HMAC signature verification
 * - Rate limiting per IP
 * - Idempotency via message ID
 * - Request logging
 *
 * @version 1.0.0
 */
class InboundEmailWebhookController extends Controller
{
    /**
     * Rate limit: max requests per minute per IP.
     */
    private const RATE_LIMIT = 60;

    /**
     * Rate limit decay in seconds.
     */
    private const RATE_LIMIT_DECAY = 60;

    public function __construct(
        private readonly InboundEmailService $inboundEmailService,
    ) {}

    /**
     * Handle Postmark inbound email webhook.
     *
     * @route POST /webhooks/postmark/inbound
     */
    public function handlePostmark(Request $request): JsonResponse
    {
        // 1. Rate limiting
        $rateLimitKey = 'inbound_email:' . $request->ip();
        if (RateLimiter::tooManyAttempts($rateLimitKey, self::RATE_LIMIT)) {
            Log::warning('Inbound email webhook rate limit exceeded', [
                'ip' => $request->ip(),
            ]);

            return response()->json([
                'success' => false,
                'error' => 'Rate limit exceeded',
            ], 429);
        }
        RateLimiter::hit($rateLimitKey, self::RATE_LIMIT_DECAY);

        // 2. Verify signature
        if (!$this->verifyPostmarkSignature($request)) {
            Log::warning('Invalid Postmark webhook signature', [
                'ip' => $request->ip(),
                'user_agent' => $request->userAgent(),
            ]);

            return response()->json([
                'success' => false,
                'error' => 'Invalid signature',
            ], 401);
        }

        // 3. Check idempotency
        $messageId = $request->input('MessageID');
        if ($messageId && $this->isDuplicateMessage($messageId)) {
            Log::info('Duplicate inbound email webhook received', [
                'message_id' => $messageId,
            ]);

            return response()->json([
                'success' => true,
                'message' => 'Already processed',
            ], 200);
        }

        // 4. Log incoming webhook
        $this->logWebhook('postmark', $request);

        // 5. Process the inbound email
        try {
            $message = $this->inboundEmailService->processPostmarkWebhook(
                $request->all()
            );

            // 6. Mark message ID as processed
            if ($messageId) {
                $this->markMessageProcessed($messageId);
            }

            return response()->json([
                'success' => true,
                'data' => [
                    'message_id' => $message->id,
                    'conversation_id' => $message->conversation_id,
                    'status' => $message->status->value,
                ],
            ], 200);
        } catch (Throwable $e) {
            Log::error('Failed to process Postmark inbound email', [
                'error' => $e->getMessage(),
                'trace' => $e->getTraceAsString(),
                'message_id' => $messageId,
                'from' => $request->input('From'),
                'subject' => $request->input('Subject'),
            ]);

            // Return 500 to trigger Postmark retry
            return response()->json([
                'success' => false,
                'error' => 'Processing failed',
            ], 500);
        }
    }

    /**
     * Handle AWS SES inbound email webhook (via SNS).
     *
     * @route POST /webhooks/ses/inbound
     */
    public function handleSes(Request $request): JsonResponse
    {
        // 1. Rate limiting
        $rateLimitKey = 'inbound_email:' . $request->ip();
        if (RateLimiter::tooManyAttempts($rateLimitKey, self::RATE_LIMIT)) {
            return response()->json(['error' => 'Rate limit exceeded'], 429);
        }
        RateLimiter::hit($rateLimitKey, self::RATE_LIMIT_DECAY);

        // 2. Handle SNS subscription confirmation
        $messageType = $request->header('x-amz-sns-message-type');
        if ($messageType === 'SubscriptionConfirmation') {
            return $this->handleSnsSubscriptionConfirmation($request);
        }

        // 3. Verify SNS signature (simplified - implement full verification in production)
        if (!$this->verifySnsSignature($request)) {
            Log::warning('Invalid SNS signature for SES webhook', [
                'ip' => $request->ip(),
            ]);

            return response()->json(['error' => 'Invalid signature'], 401);
        }

        // 4. Parse SNS message
        $payload = json_decode($request->getContent(), true);
        if (!$payload) {
            return response()->json(['error' => 'Invalid JSON payload'], 400);
        }

        // 5. Extract message from SNS wrapper
        $sesMessage = json_decode($payload['Message'] ?? '{}', true);

        // 6. Check message type
        $notificationType = $sesMessage['notificationType'] ?? $sesMessage['eventType'] ?? null;
        if ($notificationType !== 'Received') {
            // Not an inbound email event
            return response()->json(['success' => true, 'message' => 'Ignored'], 200);
        }

        // 7. Log incoming webhook
        $this->logWebhook('ses', $request);

        // 8. Process the inbound email
        try {
            $message = $this->inboundEmailService->processSesWebhook($sesMessage);

            return response()->json([
                'success' => true,
                'data' => [
                    'message_id' => $message->id,
                    'conversation_id' => $message->conversation_id,
                ],
            ], 200);
        } catch (Throwable $e) {
            Log::error('Failed to process SES inbound email', [
                'error' => $e->getMessage(),
                'trace' => $e->getTraceAsString(),
            ]);

            return response()->json(['error' => 'Processing failed'], 500);
        }
    }

    /**
     * Handle SendGrid inbound email webhook.
     *
     * @route POST /webhooks/sendgrid/inbound
     */
    public function handleSendGrid(Request $request): JsonResponse
    {
        // 1. Rate limiting
        $rateLimitKey = 'inbound_email:' . $request->ip();
        if (RateLimiter::tooManyAttempts($rateLimitKey, self::RATE_LIMIT)) {
            return response()->json(['error' => 'Rate limit exceeded'], 429);
        }
        RateLimiter::hit($rateLimitKey, self::RATE_LIMIT_DECAY);

        // 2. SendGrid sends multipart form data
        // Transform to normalized format
        $normalized = [
            'From' => $request->input('from'),
            'To' => $request->input('to'),
            'Subject' => $request->input('subject'),
            'MessageID' => $request->input('Message-ID', $request->input('headers', '')),
            'Date' => $request->input('Date', now()->toIso8601String()),
            'TextBody' => $request->input('text'),
            'HtmlBody' => $request->input('html'),
            'Headers' => $this->parseSendGridHeaders($request->input('headers', '')),
            'Attachments' => [], // SendGrid attachments need special handling
        ];

        // 3. Extract message ID from headers
        if (empty($normalized['MessageID'])) {
            $headers = $this->parseSendGridHeaders($request->input('headers', ''));
            foreach ($headers as $header) {
                if (strtolower($header['Name'] ?? '') === 'message-id') {
                    $normalized['MessageID'] = $header['Value'];
                    break;
                }
            }
        }

        // 4. Log incoming webhook
        $this->logWebhook('sendgrid', $request);

        // 5. Process using Postmark handler (normalized format)
        try {
            $message = $this->inboundEmailService->processPostmarkWebhook($normalized);

            return response()->json([
                'success' => true,
                'data' => [
                    'message_id' => $message->id,
                    'conversation_id' => $message->conversation_id,
                ],
            ], 200);
        } catch (Throwable $e) {
            Log::error('Failed to process SendGrid inbound email', [
                'error' => $e->getMessage(),
                'from' => $request->input('from'),
                'subject' => $request->input('subject'),
            ]);

            return response()->json(['error' => 'Processing failed'], 500);
        }
    }

    /**
     * Health check endpoint for webhook monitoring.
     *
     * @route GET /webhooks/inbound/health
     */
    public function health(): JsonResponse
    {
        return response()->json([
            'status' => 'healthy',
            'timestamp' => now()->toIso8601String(),
            'providers' => ['postmark', 'ses', 'sendgrid'],
        ]);
    }

    /**
     * Verify Postmark webhook signature.
     */
    private function verifyPostmarkSignature(Request $request): bool
    {
        $signature = $request->header('X-Postmark-Signature');

        // If no signature header, check if signature verification is required
        if (!$signature) {
            $requireSignature = DB::table('inbound_email_settings')
                ->where('key', 'require_webhook_signature')
                ->value('value') ?? 'false';

            // Allow unsigned requests in development
            if ($requireSignature !== 'true' && app()->environment('local', 'development')) {
                return true;
            }

            return false;
        }

        // Get signature key from settings
        $signatureKey = DB::table('inbound_email_settings')
            ->where('key', 'signature_key')
            ->value('value');

        if (!$signatureKey) {
            // No key configured - allow in development
            return app()->environment('local', 'development');
        }

        // Calculate expected signature
        $expectedSignature = hash_hmac(
            'sha256',
            $request->getContent(),
            $signatureKey
        );

        // Compare signatures (timing-safe)
        return hash_equals("sha256={$expectedSignature}", $signature);
    }

    /**
     * Verify AWS SNS signature.
     */
    private function verifySnsSignature(Request $request): bool
    {
        // In production, implement full SNS signature verification
        // using the certificate from the SigningCertURL
        // For now, check basic headers
        $topicArn = $request->header('x-amz-sns-topic-arn');

        if (!$topicArn) {
            return false;
        }

        // Verify topic ARN matches expected value
        $expectedTopicArn = config('services.ses.sns_topic_arn');

        if ($expectedTopicArn && $topicArn !== $expectedTopicArn) {
            return false;
        }

        return true;
    }

    /**
     * Handle SNS subscription confirmation.
     */
    private function handleSnsSubscriptionConfirmation(Request $request): JsonResponse
    {
        $payload = json_decode($request->getContent(), true);

        if (!isset($payload['SubscribeURL'])) {
            return response()->json(['error' => 'Missing SubscribeURL'], 400);
        }

        // Auto-confirm subscription by visiting the URL
        try {
            $client = new \GuzzleHttp\Client(['timeout' => 10]);
            $client->get($payload['SubscribeURL']);

            Log::info('SNS subscription confirmed', [
                'topic_arn' => $payload['TopicArn'] ?? 'unknown',
            ]);

            return response()->json(['success' => true, 'message' => 'Subscription confirmed']);
        } catch (Throwable $e) {
            Log::error('Failed to confirm SNS subscription', [
                'error' => $e->getMessage(),
                'subscribe_url' => $payload['SubscribeURL'],
            ]);

            return response()->json(['error' => 'Failed to confirm subscription'], 500);
        }
    }

    /**
     * Check if message has already been processed.
     */
    private function isDuplicateMessage(string $messageId): bool
    {
        // Check cache first
        $cacheKey = "inbound_email:processed:{$messageId}";
        if (cache()->has($cacheKey)) {
            return true;
        }

        // Check database
        return DB::table('email_messages')
            ->where('message_id', $messageId)
            ->exists();
    }

    /**
     * Mark message as processed for idempotency.
     */
    private function markMessageProcessed(string $messageId): void
    {
        $cacheKey = "inbound_email:processed:{$messageId}";
        cache()->put($cacheKey, true, now()->addHours(24));
    }

    /**
     * Log incoming webhook for debugging and audit.
     */
    private function logWebhook(string $provider, Request $request): void
    {
        if (!config('app.debug')) {
            return;
        }

        Log::debug('Inbound email webhook received', [
            'provider' => $provider,
            'ip' => $request->ip(),
            'user_agent' => $request->userAgent(),
            'content_type' => $request->header('Content-Type'),
            'from' => $request->input('From') ?? $request->input('from'),
            'subject' => $request->input('Subject') ?? $request->input('subject'),
            'message_id' => $request->input('MessageID') ?? $request->input('Message-ID'),
        ]);
    }

    /**
     * Parse SendGrid headers string to array format.
     *
     * @return array<array{Name: string, Value: string}>
     */
    private function parseSendGridHeaders(string $headers): array
    {
        $parsed = [];
        $lines = explode("\n", $headers);

        foreach ($lines as $line) {
            $line = trim($line);
            if (empty($line)) {
                continue;
            }

            $colonPos = strpos($line, ':');
            if ($colonPos === false) {
                continue;
            }

            $name = trim(substr($line, 0, $colonPos));
            $value = trim(substr($line, $colonPos + 1));

            $parsed[] = [
                'Name' => $name,
                'Value' => $value,
            ];
        }

        return $parsed;
    }
}

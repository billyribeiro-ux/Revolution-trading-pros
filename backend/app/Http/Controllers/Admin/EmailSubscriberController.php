<?php

declare(strict_types=1);

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Models\NewsletterSubscription;
use App\Services\Email\AuditService;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Validator;
use Illuminate\Support\Str;
use Illuminate\Validation\Rule;

/**
 * EmailSubscriberController
 *
 * Enterprise subscriber/audience management for email marketing.
 *
 * Features:
 * - Full CRUD operations
 * - Bulk import/export
 * - Tagging and segmentation
 * - Engagement scoring
 * - Activity timeline
 * - GDPR compliance
 *
 * @version 1.0.0
 */
class EmailSubscriberController extends Controller
{
    public function __construct(
        private readonly AuditService $auditService
    ) {}

    /**
     * List all subscribers with filtering and pagination
     */
    public function index(Request $request): JsonResponse
    {
        $query = NewsletterSubscription::query()->latest();

        // Status filter
        if ($status = $request->get('status')) {
            $query->where('status', $status);
        }

        // Search by email or name
        if ($search = $request->get('search')) {
            $query->where(function ($q) use ($search) {
                $q->where('email', 'like', "%{$search}%")
                    ->orWhere('name', 'like', "%{$search}%")
                    ->orWhere('first_name', 'like', "%{$search}%")
                    ->orWhere('last_name', 'like', "%{$search}%");
            });
        }

        // Source filter
        if ($source = $request->get('source')) {
            $query->where('source', $source);
        }

        // Engagement filter
        if ($engagement = $request->get('engagement')) {
            $query->where('engagement_level', $engagement);
        }

        // Tag filter
        if ($tag = $request->get('tag')) {
            $query->whereJsonContains('tags', $tag);
        }

        // Date range
        if ($from = $request->get('from_date')) {
            $query->whereDate('created_at', '>=', $from);
        }
        if ($to = $request->get('to_date')) {
            $query->whereDate('created_at', '<=', $to);
        }

        $perPage = min($request->integer('per_page', 25), 100);
        $subscribers = $query->paginate($perPage);

        return response()->json([
            'success' => true,
            'data' => $subscribers->items(),
            'meta' => [
                'current_page' => $subscribers->currentPage(),
                'last_page' => $subscribers->lastPage(),
                'per_page' => $subscribers->perPage(),
                'total' => $subscribers->total(),
            ],
        ]);
    }

    /**
     * Get subscriber statistics
     */
    public function stats(): JsonResponse
    {
        $stats = [
            'total' => NewsletterSubscription::count(),
            'active' => NewsletterSubscription::where('status', 'active')->count(),
            'pending' => NewsletterSubscription::where('status', 'pending')->count(),
            'unsubscribed' => NewsletterSubscription::where('status', 'unsubscribed')->count(),
            'bounced' => NewsletterSubscription::where('status', 'bounced')->count(),
            'complained' => NewsletterSubscription::where('status', 'complained')->count(),
        ];

        // Growth metrics
        $stats['new_today'] = NewsletterSubscription::whereDate('created_at', today())->count();
        $stats['new_this_week'] = NewsletterSubscription::whereBetween('created_at', [
            now()->startOfWeek(),
            now()->endOfWeek(),
        ])->count();
        $stats['new_this_month'] = NewsletterSubscription::whereMonth('created_at', now()->month)
            ->whereYear('created_at', now()->year)
            ->count();

        // Engagement breakdown
        $stats['engagement'] = [
            'highly_engaged' => NewsletterSubscription::where('engagement_level', 'high')->count(),
            'moderately_engaged' => NewsletterSubscription::where('engagement_level', 'medium')->count(),
            'low_engagement' => NewsletterSubscription::where('engagement_level', 'low')->count(),
            'inactive' => NewsletterSubscription::where('engagement_level', 'inactive')->count(),
        ];

        // Source breakdown
        $stats['sources'] = NewsletterSubscription::groupBy('source')
            ->selectRaw('source, COUNT(*) as count')
            ->pluck('count', 'source');

        // Unsubscribe rate
        $totalEver = NewsletterSubscription::withTrashed()->count();
        $stats['unsubscribe_rate'] = $totalEver > 0
            ? round(($stats['unsubscribed'] / $totalEver) * 100, 2)
            : 0;

        return response()->json([
            'success' => true,
            'data' => $stats,
        ]);
    }

    /**
     * Get single subscriber details
     */
    public function show(string|int $id): JsonResponse
    {
        $subscriber = NewsletterSubscription::with(['segments'])
            ->findOrFail((int) $id);

        // Get activity history
        $activity = DB::table('email_logs')
            ->where('subscriber_id', $id)
            ->orderByDesc('created_at')
            ->limit(50)
            ->get(['event_type', 'campaign_id', 'created_at', 'metadata']);

        return response()->json([
            'success' => true,
            'data' => [
                'subscriber' => $subscriber,
                'activity' => $activity,
            ],
        ]);
    }

    /**
     * Create new subscriber (manual add)
     */
    public function store(Request $request): JsonResponse
    {
        $validator = Validator::make($request->all(), [
            'email' => 'required|email|unique:newsletter_subscriptions,email',
            'name' => 'nullable|string|max:255',
            'first_name' => 'nullable|string|max:100',
            'last_name' => 'nullable|string|max:100',
            'status' => ['nullable', Rule::in(['active', 'pending', 'unsubscribed'])],
            'source' => 'nullable|string|max:50',
            'tags' => 'nullable|array',
            'custom_fields' => 'nullable|array',
            'skip_verification' => 'boolean',
        ]);

        if ($validator->fails()) {
            return response()->json([
                'success' => false,
                'errors' => $validator->errors(),
            ], 422);
        }

        $this->auditService->startTiming();

        $subscriber = NewsletterSubscription::create([
            'email' => $request->get('email'),
            'name' => $request->get('name'),
            'first_name' => $request->get('first_name'),
            'last_name' => $request->get('last_name'),
            'status' => $request->boolean('skip_verification') ? 'active' : ($request->get('status', 'pending')),
            'source' => $request->get('source', 'admin'),
            'tags' => $request->get('tags', []),
            'custom_fields' => $request->get('custom_fields', []),
            'verification_token' => Str::random(64),
            'unsubscribe_token' => Str::random(64),
            'consent_given_at' => now(),
            'consent_ip' => $request->ip(),
            'subscribed_at' => now(),
            'verified_at' => $request->boolean('skip_verification') ? now() : null,
        ]);

        $this->auditService->log(
            'create',
            'subscriber',
            $subscriber->id,
            $subscriber->email,
            null,
            $subscriber->toArray()
        );

        return response()->json([
            'success' => true,
            'data' => $subscriber,
            'message' => 'Subscriber created successfully',
        ], 201);
    }

    /**
     * Update subscriber
     */
    public function update(Request $request, string|int $id): JsonResponse
    {
        $subscriber = NewsletterSubscription::findOrFail((int) $id);

        $validator = Validator::make($request->all(), [
            'email' => ['sometimes', 'email', Rule::unique('newsletter_subscriptions')->ignore($subscriber->id)],
            'name' => 'nullable|string|max:255',
            'first_name' => 'nullable|string|max:100',
            'last_name' => 'nullable|string|max:100',
            'status' => ['nullable', Rule::in(['active', 'pending', 'unsubscribed', 'bounced', 'complained'])],
            'tags' => 'nullable|array',
            'custom_fields' => 'nullable|array',
            'preferences' => 'nullable|array',
        ]);

        if ($validator->fails()) {
            return response()->json([
                'success' => false,
                'errors' => $validator->errors(),
            ], 422);
        }

        $this->auditService->startTiming();
        $oldValues = $subscriber->toArray();

        $subscriber->update($request->only([
            'email', 'name', 'first_name', 'last_name', 'status',
            'tags', 'custom_fields', 'preferences',
        ]));

        $this->auditService->log(
            'update',
            'subscriber',
            $subscriber->id,
            $subscriber->email,
            $oldValues,
            $subscriber->fresh()->toArray()
        );

        return response()->json([
            'success' => true,
            'data' => $subscriber->fresh(),
            'message' => 'Subscriber updated successfully',
        ]);
    }

    /**
     * Delete subscriber (GDPR right to be forgotten)
     */
    public function destroy(string|int $id): JsonResponse
    {
        $subscriber = NewsletterSubscription::findOrFail((int) $id);

        $this->auditService->startTiming();
        $oldData = $subscriber->toArray();

        // Anonymize data for GDPR compliance
        $subscriber->update([
            'email' => 'deleted_' . $subscriber->id . '@anonymized.local',
            'name' => null,
            'first_name' => null,
            'last_name' => null,
            'custom_fields' => [],
            'status' => 'deleted',
        ]);

        $subscriber->delete();

        $this->auditService->log(
            'delete',
            'subscriber',
            (int) $id,
            $oldData['email'],
            $oldData,
            null,
            ['gdpr_deletion' => true]
        );

        return response()->json([
            'success' => true,
            'message' => 'Subscriber deleted and data anonymized (GDPR compliance)',
        ]);
    }

    /**
     * Add tags to subscriber
     */
    public function addTags(Request $request, string|int $id): JsonResponse
    {
        $subscriber = NewsletterSubscription::findOrFail((int) $id);

        $validator = Validator::make($request->all(), [
            'tags' => 'required|array',
            'tags.*' => 'string|max:50',
        ]);

        if ($validator->fails()) {
            return response()->json([
                'success' => false,
                'errors' => $validator->errors(),
            ], 422);
        }

        $existingTags = $subscriber->tags ?? [];
        $newTags = array_unique(array_merge($existingTags, $request->get('tags')));

        $subscriber->update(['tags' => $newTags]);

        return response()->json([
            'success' => true,
            'data' => $subscriber->fresh(),
            'message' => 'Tags added successfully',
        ]);
    }

    /**
     * Remove tags from subscriber
     */
    public function removeTags(Request $request, string|int $id): JsonResponse
    {
        $subscriber = NewsletterSubscription::findOrFail((int) $id);

        $validator = Validator::make($request->all(), [
            'tags' => 'required|array',
            'tags.*' => 'string',
        ]);

        if ($validator->fails()) {
            return response()->json([
                'success' => false,
                'errors' => $validator->errors(),
            ], 422);
        }

        $existingTags = $subscriber->tags ?? [];
        $tagsToRemove = $request->get('tags');
        $newTags = array_values(array_diff($existingTags, $tagsToRemove));

        $subscriber->update(['tags' => $newTags]);

        return response()->json([
            'success' => true,
            'data' => $subscriber->fresh(),
            'message' => 'Tags removed successfully',
        ]);
    }

    /**
     * Bulk import subscribers
     */
    public function import(Request $request): JsonResponse
    {
        $validator = Validator::make($request->all(), [
            'subscribers' => 'required|array|max:1000',
            'subscribers.*.email' => 'required|email',
            'subscribers.*.name' => 'nullable|string|max:255',
            'subscribers.*.tags' => 'nullable|array',
            'skip_duplicates' => 'boolean',
            'update_existing' => 'boolean',
            'source' => 'nullable|string|max:50',
        ]);

        if ($validator->fails()) {
            return response()->json([
                'success' => false,
                'errors' => $validator->errors(),
            ], 422);
        }

        $imported = 0;
        $updated = 0;
        $skipped = 0;
        $errors = [];

        $source = $request->get('source', 'bulk_import');
        $skipDuplicates = $request->boolean('skip_duplicates', true);
        $updateExisting = $request->boolean('update_existing', false);

        DB::beginTransaction();

        try {
            foreach ($request->get('subscribers') as $index => $data) {
                $existing = NewsletterSubscription::where('email', $data['email'])->first();

                if ($existing) {
                    if ($updateExisting) {
                        $existing->update([
                            'name' => $data['name'] ?? $existing->name,
                            'tags' => array_unique(array_merge(
                                $existing->tags ?? [],
                                $data['tags'] ?? []
                            )),
                        ]);
                        $updated++;
                    } elseif ($skipDuplicates) {
                        $skipped++;
                    } else {
                        $errors[] = "Row {$index}: Email {$data['email']} already exists";
                    }
                    continue;
                }

                NewsletterSubscription::create([
                    'email' => $data['email'],
                    'name' => $data['name'] ?? null,
                    'first_name' => $data['first_name'] ?? null,
                    'last_name' => $data['last_name'] ?? null,
                    'status' => 'active',
                    'source' => $source,
                    'tags' => $data['tags'] ?? [],
                    'verification_token' => Str::random(64),
                    'unsubscribe_token' => Str::random(64),
                    'consent_given_at' => now(),
                    'consent_ip' => $request->ip(),
                    'subscribed_at' => now(),
                    'verified_at' => now(),
                ]);
                $imported++;
            }

            DB::commit();

            $this->auditService->log(
                'bulk_import',
                'subscriber',
                null,
                "Imported {$imported} subscribers",
                null,
                null,
                ['imported' => $imported, 'updated' => $updated, 'skipped' => $skipped]
            );

            return response()->json([
                'success' => true,
                'data' => [
                    'imported' => $imported,
                    'updated' => $updated,
                    'skipped' => $skipped,
                    'errors' => $errors,
                ],
                'message' => "Successfully processed " . count($request->get('subscribers')) . " subscribers",
            ]);

        } catch (\Exception $e) {
            DB::rollBack();
            return response()->json([
                'success' => false,
                'message' => 'Import failed: ' . $e->getMessage(),
            ], 500);
        }
    }

    /**
     * Export subscribers
     */
    public function export(Request $request): JsonResponse
    {
        $query = NewsletterSubscription::query();

        // Apply filters
        if ($status = $request->get('status')) {
            $query->where('status', $status);
        }
        if ($tag = $request->get('tag')) {
            $query->whereJsonContains('tags', $tag);
        }
        if ($from = $request->get('from_date')) {
            $query->whereDate('created_at', '>=', $from);
        }
        if ($to = $request->get('to_date')) {
            $query->whereDate('created_at', '<=', $to);
        }

        $subscribers = $query->limit(10000)->get([
            'email', 'name', 'first_name', 'last_name', 'status',
            'source', 'tags', 'created_at', 'verified_at', 'engagement_level',
        ]);

        $this->auditService->log(
            'export',
            'subscriber',
            null,
            "Exported {$subscribers->count()} subscribers",
            null,
            null,
            ['count' => $subscribers->count()]
        );

        return response()->json([
            'success' => true,
            'data' => $subscribers,
            'meta' => [
                'total' => $subscribers->count(),
                'exported_at' => now()->toIso8601String(),
            ],
        ]);
    }

    /**
     * Get all unique tags
     */
    public function tags(): JsonResponse
    {
        $tags = NewsletterSubscription::whereNotNull('tags')
            ->pluck('tags')
            ->flatten()
            ->unique()
            ->values();

        return response()->json([
            'success' => true,
            'data' => $tags,
        ]);
    }

    /**
     * Resend verification email
     */
    public function resendVerification(string|int $id): JsonResponse
    {
        $subscriber = NewsletterSubscription::findOrFail((int) $id);

        if ($subscriber->status !== 'pending') {
            return response()->json([
                'success' => false,
                'message' => 'Subscriber is not pending verification',
            ], 400);
        }

        // Generate new token
        $subscriber->update([
            'verification_token' => Str::random(64),
        ]);

        // Send verification email
        $subscriber->notify(new \App\Notifications\NewsletterVerificationNotification());

        return response()->json([
            'success' => true,
            'message' => 'Verification email sent',
        ]);
    }

    /**
     * Manually verify subscriber
     */
    public function verify(string|int $id): JsonResponse
    {
        $subscriber = NewsletterSubscription::findOrFail((int) $id);

        if ($subscriber->status === 'active') {
            return response()->json([
                'success' => false,
                'message' => 'Subscriber is already verified',
            ], 400);
        }

        $subscriber->update([
            'status' => 'active',
            'verified_at' => now(),
            'verification_token' => null,
        ]);

        $this->auditService->log(
            'verify',
            'subscriber',
            $subscriber->id,
            $subscriber->email,
            ['status' => 'pending'],
            ['status' => 'active'],
            ['manual_verification' => true]
        );

        return response()->json([
            'success' => true,
            'data' => $subscriber->fresh(),
            'message' => 'Subscriber verified successfully',
        ]);
    }
}

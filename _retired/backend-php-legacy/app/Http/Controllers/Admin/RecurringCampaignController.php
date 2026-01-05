<?php

declare(strict_types=1);

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Models\RecurringCampaign;
use App\Models\RecurringMail;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Str;
use Illuminate\Validation\Rule;

/**
 * Recurring Campaign Controller (FluentCRM Pro Scheduled Newsletters)
 *
 * Handles CRUD operations for recurring campaigns.
 * December 2025 Laravel 12 syntax.
 */
class RecurringCampaignController extends Controller
{
    private const ALLOWED_SORT_COLUMNS = [
        'created_at', 'updated_at', 'title', 'status', 'total_campaigns_sent', 'next_scheduled_at'
    ];

    public function index(Request $request): JsonResponse
    {
        $query = RecurringCampaign::query();

        if ($request->filled('status')) {
            $query->where('status', $request->status);
        }

        if ($request->filled('search')) {
            $search = $request->search;
            $query->where('title', 'like', "%{$search}%");
        }

        $sortBy = in_array($request->get('sort_by'), self::ALLOWED_SORT_COLUMNS, true)
            ? $request->get('sort_by')
            : 'created_at';
        $sortOrder = in_array(strtolower($request->get('sort_order', 'desc')), ['asc', 'desc'], true)
            ? $request->get('sort_order', 'desc')
            : 'desc';

        $campaigns = $query->with('creator')
            ->withCount('emails')
            ->orderBy($sortBy, $sortOrder)
            ->paginate($request->integer('per_page', 25));

        return response()->json($campaigns);
    }

    public function store(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'title' => 'required|string|max:255',
            'email_subject' => 'nullable|string|max:255',
            'email_pre_header' => 'nullable|string|max:255',
            'email_body' => 'nullable|string',
            'design_template' => 'nullable|string|max:50',
            'settings' => 'nullable|array',
            'scheduling_settings' => 'nullable|array',
            'subscriber_settings' => 'nullable|array',
            'template_config' => 'nullable|array',
            'labels' => 'nullable|array',
        ]);

        $campaign = RecurringCampaign::create([
            'id' => Str::uuid(),
            'created_by' => auth()->id(),
            ...$validated,
        ]);

        // Calculate next scheduled time
        $campaign->scheduleNextSend();

        return response()->json($campaign->load('creator'), 201);
    }

    public function show(string $id): JsonResponse
    {
        $campaign = RecurringCampaign::with(['creator', 'emails' => fn ($q) => $q->latest('scheduled_at')->limit(10)])
            ->withCount('emails')
            ->findOrFail($id);

        $stats = $campaign->getStats();

        return response()->json([
            'campaign' => $campaign,
            'stats' => $stats,
        ]);
    }

    public function update(Request $request, string $id): JsonResponse
    {
        $campaign = RecurringCampaign::findOrFail($id);

        $validated = $request->validate([
            'title' => 'sometimes|string|max:255',
            'email_subject' => 'nullable|string|max:255',
            'email_pre_header' => 'nullable|string|max:255',
            'email_body' => 'nullable|string',
            'design_template' => 'nullable|string|max:50',
            'settings' => 'nullable|array',
            'scheduling_settings' => 'nullable|array',
            'subscriber_settings' => 'nullable|array',
            'template_config' => 'nullable|array',
            'labels' => 'nullable|array',
        ]);

        $campaign->update($validated);

        // Recalculate next scheduled time if scheduling changed
        if ($request->has('scheduling_settings')) {
            $campaign->scheduleNextSend();
        }

        return response()->json($campaign->fresh('creator'));
    }

    public function destroy(string $id): JsonResponse
    {
        $campaign = RecurringCampaign::findOrFail($id);
        $campaign->delete();

        return response()->json(['message' => 'Recurring campaign deleted']);
    }

    public function changeStatus(Request $request, string $id): JsonResponse
    {
        $campaign = RecurringCampaign::findOrFail($id);

        $validated = $request->validate([
            'status' => ['required', Rule::in(['draft', 'active', 'paused'])],
        ]);

        $campaign->update(['status' => $validated['status']]);

        if ($validated['status'] === 'active') {
            $campaign->scheduleNextSend();
        }

        return response()->json($campaign->fresh());
    }

    public function duplicate(string $id): JsonResponse
    {
        $original = RecurringCampaign::findOrFail($id);

        $clone = $original->replicate();
        $clone->id = Str::uuid();
        $clone->title = "{$original->title} (Copy)";
        $clone->slug = Str::slug($clone->title);
        $clone->status = 'draft';
        $clone->total_campaigns_sent = 0;
        $clone->total_emails_sent = 0;
        $clone->total_revenue = 0;
        $clone->last_sent_at = null;
        $clone->next_scheduled_at = null;
        $clone->created_by = auth()->id();
        $clone->save();

        return response()->json($clone, 201);
    }

    public function updateSettings(Request $request, string $id): JsonResponse
    {
        $campaign = RecurringCampaign::findOrFail($id);

        $validated = $request->validate([
            'scheduling_settings' => 'nullable|array',
            'subscriber_settings' => 'nullable|array',
            'settings' => 'nullable|array',
        ]);

        $campaign->update($validated);
        $campaign->scheduleNextSend();

        return response()->json($campaign->fresh());
    }

    public function updateLabels(Request $request, string $id): JsonResponse
    {
        $campaign = RecurringCampaign::findOrFail($id);

        $validated = $request->validate([
            'labels' => 'required|array',
        ]);

        $campaign->update(['labels' => $validated['labels']]);

        return response()->json($campaign->fresh());
    }

    // Campaign emails (sent instances)
    public function getEmails(Request $request, string $id): JsonResponse
    {
        $campaign = RecurringCampaign::findOrFail($id);

        $emails = $campaign->emails()
            ->orderByDesc('scheduled_at')
            ->paginate($request->integer('per_page', 25));

        return response()->json($emails);
    }

    public function getEmail(string $id, string $emailId): JsonResponse
    {
        $email = RecurringMail::where('recurring_campaign_id', $id)
            ->findOrFail($emailId);

        return response()->json([
            'email' => $email,
            'stats' => $email->getStats(),
        ]);
    }

    public function updateCampaignEmail(Request $request, string $id): JsonResponse
    {
        $campaign = RecurringCampaign::findOrFail($id);

        $validated = $request->validate([
            'email_subject' => 'required|string|max:255',
            'email_pre_header' => 'nullable|string|max:255',
            'email_body' => 'required|string',
        ]);

        $campaign->update($validated);

        return response()->json($campaign->fresh());
    }

    public function deleteBulk(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'ids' => 'required|array',
            'ids.*' => 'uuid',
        ]);

        $deleted = RecurringCampaign::whereIn('id', $validated['ids'])->delete();

        return response()->json([
            'message' => "Deleted {$deleted} campaigns",
            'deleted_count' => $deleted,
        ]);
    }

    public function handleBulkAction(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'ids' => 'required|array',
            'ids.*' => 'uuid',
            'action' => ['required', Rule::in(['activate', 'pause', 'delete'])],
        ]);

        $count = 0;

        switch ($validated['action']) {
            case 'activate':
                $count = RecurringCampaign::whereIn('id', $validated['ids'])
                    ->update(['status' => 'active']);
                break;
            case 'pause':
                $count = RecurringCampaign::whereIn('id', $validated['ids'])
                    ->update(['status' => 'paused']);
                break;
            case 'delete':
                $count = RecurringCampaign::whereIn('id', $validated['ids'])->delete();
                break;
        }

        return response()->json([
            'message' => "Processed {$count} campaigns",
            'affected_count' => $count,
        ]);
    }
}

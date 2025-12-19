<?php

declare(strict_types=1);

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Models\EmailSequence;
use App\Models\SequenceMail;
use App\Models\SequenceTracker;
use App\Models\Contact;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Str;
use Illuminate\Validation\Rule;

/**
 * Email Sequence Controller (FluentCRM Pro Drip Campaigns)
 *
 * Handles CRUD operations for email sequences and their emails.
 * December 2025 Laravel 12 syntax.
 */
class SequenceController extends Controller
{
    private const ALLOWED_SORT_COLUMNS = [
        'created_at', 'updated_at', 'title', 'status', 'subscribers_count', 'emails_count'
    ];

    public function index(Request $request): JsonResponse
    {
        $query = EmailSequence::query();

        // Filters
        if ($request->filled('status')) {
            $query->where('status', $request->status);
        }

        if ($request->filled('search')) {
            $search = $request->search;
            $query->where(function ($q) use ($search) {
                $q->where('title', 'like', "%{$search}%")
                  ->orWhere('description', 'like', "%{$search}%");
            });
        }

        // Sorting with validation
        $sortBy = in_array($request->get('sort_by'), self::ALLOWED_SORT_COLUMNS, true)
            ? $request->get('sort_by')
            : 'created_at';
        $sortOrder = in_array(strtolower($request->get('sort_order', 'desc')), ['asc', 'desc'], true)
            ? $request->get('sort_order', 'desc')
            : 'desc';

        $query->orderBy($sortBy, $sortOrder);

        $sequences = $query->with(['creator'])
            ->withCount(['emails', 'trackers'])
            ->paginate($request->integer('per_page', 25));

        return response()->json($sequences);
    }

    public function store(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'title' => 'required|string|max:255',
            'description' => 'nullable|string|max:1000',
            'design_template' => 'nullable|string|max:50',
            'settings' => 'nullable|array',
            'subscriber_settings' => 'nullable|array',
        ]);

        $sequence = EmailSequence::create([
            'id' => Str::uuid(),
            'created_by' => auth()->id(),
            ...$validated,
        ]);

        return response()->json($sequence->load('creator'), 201);
    }

    public function show(string $id): JsonResponse
    {
        $sequence = EmailSequence::with([
            'emails' => fn ($q) => $q->orderBy('position'),
            'creator',
        ])
        ->withCount(['emails', 'trackers'])
        ->findOrFail($id);

        $stats = $sequence->getStats();

        return response()->json([
            'sequence' => $sequence,
            'stats' => $stats,
        ]);
    }

    public function update(Request $request, string $id): JsonResponse
    {
        $sequence = EmailSequence::findOrFail($id);

        $validated = $request->validate([
            'title' => 'sometimes|string|max:255',
            'description' => 'nullable|string|max:1000',
            'status' => ['sometimes', Rule::in(['draft', 'active', 'paused', 'completed'])],
            'design_template' => 'nullable|string|max:50',
            'settings' => 'nullable|array',
            'subscriber_settings' => 'nullable|array',
        ]);

        $sequence->update($validated);

        return response()->json($sequence->fresh(['creator', 'emails']));
    }

    public function destroy(string $id): JsonResponse
    {
        $sequence = EmailSequence::findOrFail($id);
        $sequence->delete();

        return response()->json(['message' => 'Sequence deleted']);
    }

    public function duplicate(string $id): JsonResponse
    {
        $original = EmailSequence::with('emails')->findOrFail($id);

        $clone = $original->replicate();
        $clone->id = Str::uuid();
        $clone->title = "{$original->title} (Copy)";
        $clone->slug = Str::slug($clone->title);
        $clone->status = 'draft';
        $clone->subscribers_count = 0;
        $clone->total_sent = 0;
        $clone->total_opened = 0;
        $clone->total_clicked = 0;
        $clone->total_revenue = 0;
        $clone->created_by = auth()->id();
        $clone->save();

        // Clone emails
        foreach ($original->emails as $email) {
            $emailClone = $email->replicate();
            $emailClone->id = Str::uuid();
            $emailClone->sequence_id = $clone->id;
            $emailClone->status = 'draft';
            $emailClone->sent_count = 0;
            $emailClone->open_count = 0;
            $emailClone->click_count = 0;
            $emailClone->revenue = 0;
            $emailClone->save();
        }

        return response()->json($clone->load('emails'), 201);
    }

    // Email endpoints
    public function storeEmail(Request $request, string $sequenceId): JsonResponse
    {
        $sequence = EmailSequence::findOrFail($sequenceId);

        $validated = $request->validate([
            'title' => 'required|string|max:255',
            'email_subject' => 'required|string|max:255',
            'email_pre_header' => 'nullable|string|max:255',
            'email_body' => 'required|string',
            'delay_value' => 'required|integer|min:0',
            'delay_unit' => ['required', Rule::in(['minutes', 'hours', 'days', 'weeks'])],
            'settings' => 'nullable|array',
        ]);

        $position = $sequence->emails()->max('position') ?? -1;

        $email = $sequence->emails()->create([
            'id' => Str::uuid(),
            'position' => $position + 1,
            'created_by' => auth()->id(),
            ...$validated,
        ]);

        return response()->json($email, 201);
    }

    public function showEmail(string $sequenceId, string $emailId): JsonResponse
    {
        $email = SequenceMail::where('sequence_id', $sequenceId)
            ->findOrFail($emailId);

        return response()->json($email);
    }

    public function updateEmail(Request $request, string $sequenceId, string $emailId): JsonResponse
    {
        $email = SequenceMail::where('sequence_id', $sequenceId)
            ->findOrFail($emailId);

        $validated = $request->validate([
            'title' => 'sometimes|string|max:255',
            'email_subject' => 'sometimes|string|max:255',
            'email_pre_header' => 'nullable|string|max:255',
            'email_body' => 'sometimes|string',
            'delay_value' => 'sometimes|integer|min:0',
            'delay_unit' => ['sometimes', Rule::in(['minutes', 'hours', 'days', 'weeks'])],
            'status' => ['sometimes', Rule::in(['draft', 'active', 'paused'])],
            'position' => 'sometimes|integer|min:0',
            'settings' => 'nullable|array',
        ]);

        $email->update($validated);

        return response()->json($email->fresh());
    }

    public function destroyEmail(string $sequenceId, string $emailId): JsonResponse
    {
        $email = SequenceMail::where('sequence_id', $sequenceId)
            ->findOrFail($emailId);
        $email->delete();

        return response()->json(['message' => 'Email deleted']);
    }

    public function duplicateEmail(Request $request, string $sequenceId): JsonResponse
    {
        $emailId = $request->input('email_id');
        $email = SequenceMail::where('sequence_id', $sequenceId)
            ->findOrFail($emailId);

        $clone = $email->replicate();
        $clone->id = Str::uuid();
        $clone->title = "{$email->title} (Copy)";
        $clone->status = 'draft';
        $clone->position = SequenceMail::where('sequence_id', $sequenceId)->max('position') + 1;
        $clone->sent_count = 0;
        $clone->open_count = 0;
        $clone->click_count = 0;
        $clone->revenue = 0;
        $clone->save();

        return response()->json($clone, 201);
    }

    // Subscriber endpoints
    public function getSubscribers(Request $request, string $id): JsonResponse
    {
        $sequence = EmailSequence::findOrFail($id);

        $query = $sequence->trackers()->with(['contact', 'lastMail', 'nextMail']);

        if ($request->filled('status')) {
            $query->where('status', $request->status);
        }

        $subscribers = $query->paginate($request->integer('per_page', 25));

        return response()->json($subscribers);
    }

    public function subscribe(Request $request, string $id): JsonResponse
    {
        $sequence = EmailSequence::findOrFail($id);

        $validated = $request->validate([
            'contact_ids' => 'required|array',
            'contact_ids.*' => 'uuid|exists:contacts,id',
        ]);

        $results = [];
        foreach ($validated['contact_ids'] as $contactId) {
            $contact = Contact::find($contactId);
            $tracker = $sequence->subscribe($contact);
            $results[] = [
                'contact_id' => $contactId,
                'success' => $tracker !== null,
                'tracker_id' => $tracker?->id,
            ];
        }

        return response()->json([
            'message' => 'Contacts processed',
            'results' => $results,
        ]);
    }

    public function unsubscribe(Request $request, string $id): JsonResponse
    {
        $sequence = EmailSequence::findOrFail($id);

        $validated = $request->validate([
            'contact_ids' => 'required|array',
            'contact_ids.*' => 'uuid|exists:contacts,id',
        ]);

        foreach ($validated['contact_ids'] as $contactId) {
            $contact = Contact::find($contactId);
            $sequence->unsubscribe($contact, 'manual');
        }

        return response()->json(['message' => 'Contacts unsubscribed']);
    }

    public function reapply(string $id): JsonResponse
    {
        $sequence = EmailSequence::findOrFail($id);

        // Reactivate completed subscribers to receive next email
        $updated = SequenceTracker::where('sequence_id', $id)
            ->where('status', 'completed')
            ->update([
                'status' => 'active',
                'next_execution_at' => now(),
            ]);

        return response()->json([
            'message' => "Reapplied sequence to {$updated} subscribers",
            'updated_count' => $updated,
        ]);
    }
}

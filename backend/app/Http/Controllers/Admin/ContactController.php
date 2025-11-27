<?php

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Models\Contact;
use App\Services\LeadScoringService;
use App\Services\HealthScoringService;
use App\Services\ContactTimelineService;
use Illuminate\Http\Request;
use Illuminate\Support\Str;

class ContactController extends Controller
{
    /**
     * Allowed columns for sorting (SQL injection prevention)
     */
    private const ALLOWED_SORT_COLUMNS = [
        'created_at', 'updated_at', 'first_name', 'last_name', 'email',
        'status', 'lifecycle_stage', 'lead_score', 'id'
    ];

    public function __construct(
        private LeadScoringService $leadScoringService,
        private HealthScoringService $healthScoringService,
        private ContactTimelineService $timelineService
    ) {}

    public function index(Request $request)
    {
        $query = Contact::query();

        // Filters
        if ($request->has('status')) {
            $query->where('status', $request->status);
        }

        if ($request->has('lifecycle_stage')) {
            $query->where('lifecycle_stage', $request->lifecycle_stage);
        }

        if ($request->has('owner_id')) {
            $query->where('owner_id', $request->owner_id);
        }

        if ($request->has('search')) {
            $search = $request->search;
            $query->where(function($q) use ($search) {
                $q->where('first_name', 'like', "%{$search}%")
                  ->orWhere('last_name', 'like', "%{$search}%")
                  ->orWhere('email', 'like', "%{$search}%")
                  ->orWhere('job_title', 'like', "%{$search}%");
            });
        }

        // Sorting with whitelist validation (SQL injection prevention)
        $sortBy = $request->get('sort_by', 'created_at');
        $sortOrder = $request->get('sort_order', 'desc');

        if (!in_array($sortBy, self::ALLOWED_SORT_COLUMNS, true)) {
            $sortBy = 'created_at';
        }
        if (!in_array(strtolower($sortOrder), ['asc', 'desc'], true)) {
            $sortOrder = 'desc';
        }

        $query->orderBy($sortBy, $sortOrder);

        return response()->json(
            $query->with(['owner'])->paginate($request->get('per_page', 25))
        );
    }

    public function store(Request $request)
    {
        $validated = $request->validate([
            'email' => 'required|email|unique:contacts',
            'first_name' => 'required|string|max:100',
            'last_name' => 'required|string|max:100',
            'phone' => 'nullable|string|max:50',
            'job_title' => 'nullable|string|max:100',
            'source' => 'required|in:website,form,import,api,manual,referral,event',
            'owner_id' => 'nullable|uuid|exists:users,id',
            'status' => 'nullable|in:lead,prospect,customer,churned,unqualified',
        ]);

        $contact = Contact::create([
            'id' => Str::uuid(),
            ...$validated,
        ]);

        // Calculate initial lead score
        $leadScore = $this->leadScoringService->calculateLeadScore($contact);
        $contact->update(['lead_score' => $leadScore]);

        return response()->json($contact, 201);
    }

    public function show(string $id)
    {
        $contact = Contact::with([
            'owner',
            'deals.pipeline',
            'deals.stage',
            'notes.createdBy',
        ])->findOrFail($id);

        return response()->json($contact);
    }

    public function update(Request $request, string $id)
    {
        $contact = Contact::findOrFail($id);

        $validated = $request->validate([
            'first_name' => 'sometimes|string|max:100',
            'last_name' => 'sometimes|string|max:100',
            'email' => 'sometimes|email|unique:contacts,email,' . $id,
            'phone' => 'nullable|string|max:50',
            'job_title' => 'nullable|string|max:100',
            'status' => 'sometimes|in:lead,prospect,customer,churned,unqualified',
            'lifecycle_stage' => 'sometimes|in:subscriber,lead,mql,sql,opportunity,customer,evangelist',
            'owner_id' => 'nullable|uuid|exists:users,id',
        ]);

        $contact->update($validated);

        // Recalculate scores
        $this->leadScoringService->recalculateScore($contact);
        if ($contact->status === 'customer') {
            $this->healthScoringService->recalculateHealthScore($contact);
        }

        return response()->json($contact);
    }

    public function destroy(string $id)
    {
        $contact = Contact::findOrFail($id);
        $contact->delete();

        return response()->json(['message' => 'Contact deleted']);
    }

    public function timeline(string $id)
    {
        $contact = Contact::findOrFail($id);
        $timeline = $this->timelineService->getTimeline($contact);

        return response()->json($timeline);
    }

    public function recalculateScore(string $id)
    {
        $contact = Contact::findOrFail($id);
        $this->leadScoringService->recalculateScore($contact);

        if ($contact->status === 'customer') {
            $this->healthScoringService->recalculateHealthScore($contact);
        }

        return response()->json($contact->fresh());
    }
}

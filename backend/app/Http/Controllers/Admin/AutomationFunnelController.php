<?php

declare(strict_types=1);

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Models\AutomationFunnel;
use App\Models\FunnelAction;
use App\Models\FunnelSubscriber;
use App\Models\Contact;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Str;
use Illuminate\Validation\Rule;

/**
 * Automation Funnel Controller (FluentCRM Pro Automations)
 *
 * Handles CRUD operations for automation funnels and their actions.
 * December 2025 Laravel 12 syntax.
 */
class AutomationFunnelController extends Controller
{
    private const ALLOWED_SORT_COLUMNS = [
        'created_at', 'updated_at', 'title', 'status', 'subscribers_count', 'completed_count'
    ];

    public function index(Request $request): JsonResponse
    {
        $query = AutomationFunnel::query();

        if ($request->filled('status')) {
            $query->where('status', $request->status);
        }

        if ($request->filled('trigger_type')) {
            $query->where('trigger_type', $request->trigger_type);
        }

        if ($request->filled('search')) {
            $search = $request->search;
            $query->where(function ($q) use ($search) {
                $q->where('title', 'like', "%{$search}%")
                  ->orWhere('description', 'like', "%{$search}%");
            });
        }

        $sortBy = in_array($request->get('sort_by'), self::ALLOWED_SORT_COLUMNS, true)
            ? $request->get('sort_by')
            : 'created_at';
        $sortOrder = in_array(strtolower($request->get('sort_order', 'desc')), ['asc', 'desc'], true)
            ? $request->get('sort_order', 'desc')
            : 'desc';

        $funnels = $query->with('creator')
            ->withCount(['actions', 'subscribers'])
            ->orderBy($sortBy, $sortOrder)
            ->paginate($request->integer('per_page', 25));

        return response()->json($funnels);
    }

    public function store(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'title' => 'required|string|max:255',
            'description' => 'nullable|string|max:1000',
            'trigger_type' => ['required', Rule::in(array_keys(AutomationFunnel::TRIGGER_TYPES))],
            'trigger_settings' => 'nullable|array',
            'conditions' => 'nullable|array',
        ]);

        $funnel = AutomationFunnel::create([
            'id' => Str::uuid(),
            'created_by' => auth()->id(),
            ...$validated,
        ]);

        return response()->json($funnel->load('creator'), 201);
    }

    public function show(string $id): JsonResponse
    {
        $funnel = AutomationFunnel::with([
            'actions' => fn ($q) => $q->orderBy('position'),
            'creator',
        ])
        ->withCount(['actions', 'subscribers'])
        ->findOrFail($id);

        $stats = $funnel->getStats();

        return response()->json([
            'funnel' => $funnel,
            'stats' => $stats,
            'trigger_types' => AutomationFunnel::TRIGGER_TYPES,
            'action_types' => FunnelAction::ACTION_TYPES,
        ]);
    }

    public function update(Request $request, string $id): JsonResponse
    {
        $funnel = AutomationFunnel::findOrFail($id);

        $validated = $request->validate([
            'title' => 'sometimes|string|max:255',
            'description' => 'nullable|string|max:1000',
            'status' => ['sometimes', Rule::in(['draft', 'active', 'paused'])],
            'trigger_type' => ['sometimes', Rule::in(array_keys(AutomationFunnel::TRIGGER_TYPES))],
            'trigger_settings' => 'nullable|array',
            'conditions' => 'nullable|array',
        ]);

        $funnel->update($validated);

        return response()->json($funnel->fresh(['creator', 'actions']));
    }

    public function destroy(string $id): JsonResponse
    {
        $funnel = AutomationFunnel::findOrFail($id);
        $funnel->delete();

        return response()->json(['message' => 'Automation funnel deleted']);
    }

    public function duplicate(string $id): JsonResponse
    {
        $original = AutomationFunnel::with('actions')->findOrFail($id);

        $clone = $original->replicate();
        $clone->id = Str::uuid();
        $clone->title = "{$original->title} (Copy)";
        $clone->slug = Str::slug($clone->title);
        $clone->status = 'draft';
        $clone->subscribers_count = 0;
        $clone->completed_count = 0;
        $clone->total_revenue = 0;
        $clone->created_by = auth()->id();
        $clone->save();

        // Clone actions
        $actionMap = [];
        foreach ($original->actions as $action) {
            $actionClone = $action->replicate();
            $actionClone->id = Str::uuid();
            $actionClone->funnel_id = $clone->id;
            $actionClone->execution_count = 0;
            $actionClone->save();
            $actionMap[$action->id] = $actionClone->id;
        }

        // Update parent references
        foreach ($clone->actions as $action) {
            if ($action->parent_id && isset($actionMap[$action->parent_id])) {
                $action->update(['parent_id' => $actionMap[$action->parent_id]]);
            }
        }

        return response()->json($clone->load('actions'), 201);
    }

    public function activate(string $id): JsonResponse
    {
        $funnel = AutomationFunnel::findOrFail($id);
        $funnel->activate();

        return response()->json($funnel->fresh());
    }

    public function pause(string $id): JsonResponse
    {
        $funnel = AutomationFunnel::findOrFail($id);
        $funnel->pause();

        return response()->json($funnel->fresh());
    }

    // Action endpoints
    public function storeAction(Request $request, string $funnelId): JsonResponse
    {
        $funnel = AutomationFunnel::findOrFail($funnelId);

        $validated = $request->validate([
            'action_type' => ['required', Rule::in(array_keys(FunnelAction::ACTION_TYPES))],
            'title' => 'nullable|string|max:255',
            'parent_id' => 'nullable|uuid|exists:funnel_actions,id',
            'settings' => 'nullable|array',
            'condition_type' => ['nullable', Rule::in(['yes', 'no', 'none'])],
            'delay_seconds' => 'nullable|integer|min:0',
        ]);

        $position = $funnel->actions()->max('position') ?? -1;

        $action = $funnel->actions()->create([
            'id' => Str::uuid(),
            'position' => $position + 1,
            ...$validated,
        ]);

        return response()->json($action, 201);
    }

    public function updateAction(Request $request, string $funnelId, string $actionId): JsonResponse
    {
        $action = FunnelAction::where('funnel_id', $funnelId)->findOrFail($actionId);

        $validated = $request->validate([
            'action_type' => ['sometimes', Rule::in(array_keys(FunnelAction::ACTION_TYPES))],
            'title' => 'nullable|string|max:255',
            'parent_id' => 'nullable|uuid|exists:funnel_actions,id',
            'settings' => 'nullable|array',
            'condition_type' => ['nullable', Rule::in(['yes', 'no', 'none'])],
            'delay_seconds' => 'nullable|integer|min:0',
            'position' => 'nullable|integer|min:0',
        ]);

        $action->update($validated);

        return response()->json($action->fresh());
    }

    public function destroyAction(string $funnelId, string $actionId): JsonResponse
    {
        $action = FunnelAction::where('funnel_id', $funnelId)->findOrFail($actionId);
        $action->delete();

        return response()->json(['message' => 'Action deleted']);
    }

    public function reorderActions(Request $request, string $funnelId): JsonResponse
    {
        $validated = $request->validate([
            'actions' => 'required|array',
            'actions.*.id' => 'required|uuid|exists:funnel_actions,id',
            'actions.*.position' => 'required|integer|min:0',
        ]);

        foreach ($validated['actions'] as $item) {
            FunnelAction::where('id', $item['id'])
                ->where('funnel_id', $funnelId)
                ->update(['position' => $item['position']]);
        }

        return response()->json(['message' => 'Actions reordered']);
    }

    // Subscriber endpoints
    public function getSubscribers(Request $request, string $id): JsonResponse
    {
        $funnel = AutomationFunnel::findOrFail($id);

        $query = $funnel->subscribers()->with(['contact', 'currentAction']);

        if ($request->filled('status')) {
            $query->where('status', $request->status);
        }

        $subscribers = $query->orderByDesc('entered_at')
            ->paginate($request->integer('per_page', 25));

        return response()->json($subscribers);
    }

    public function enrollContact(Request $request, string $id): JsonResponse
    {
        $funnel = AutomationFunnel::findOrFail($id);

        $validated = $request->validate([
            'contact_id' => 'required|uuid|exists:contacts,id',
        ]);

        $contact = Contact::findOrFail($validated['contact_id']);
        $subscriber = $funnel->enroll($contact);

        if (!$subscriber) {
            return response()->json(['message' => 'Contact cannot be enrolled'], 422);
        }

        return response()->json($subscriber->load('contact'), 201);
    }

    public function removeSubscriber(Request $request, string $id, string $subscriberId): JsonResponse
    {
        $subscriber = FunnelSubscriber::where('funnel_id', $id)->findOrFail($subscriberId);
        $subscriber->cancel($request->input('reason', 'manual'));

        return response()->json(['message' => 'Subscriber removed from funnel']);
    }

    public function retrySubscriber(string $id, string $subscriberId): JsonResponse
    {
        $subscriber = FunnelSubscriber::where('funnel_id', $id)->findOrFail($subscriberId);
        $subscriber->retry();

        return response()->json($subscriber->fresh());
    }

    // Trigger types reference
    public function getTriggerTypes(): JsonResponse
    {
        return response()->json(AutomationFunnel::TRIGGER_TYPES);
    }

    public function getActionTypes(): JsonResponse
    {
        return response()->json(FunnelAction::ACTION_TYPES);
    }
}

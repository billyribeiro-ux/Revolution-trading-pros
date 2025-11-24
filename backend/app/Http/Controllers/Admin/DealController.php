<?php

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Models\Deal;
use App\Models\DealStageHistory;
use App\Services\DealForecastingService;
use Illuminate\Http\Request;
use Illuminate\Support\Str;

class DealController extends Controller
{
    public function __construct(
        private DealForecastingService $forecastingService
    ) {}

    public function index(Request $request)
    {
        $query = Deal::query();

        if ($request->has('pipeline_id')) {
            $query->where('pipeline_id', $request->pipeline_id);
        }

        if ($request->has('stage_id')) {
            $query->where('stage_id', $request->stage_id);
        }

        if ($request->has('status')) {
            $query->where('status', $request->status);
        }

        if ($request->has('owner_id')) {
            $query->where('owner_id', $request->owner_id);
        }

        return response()->json(
            $query->with(['contact', 'pipeline', 'stage', 'owner'])->paginate(50)
        );
    }

    public function store(Request $request)
    {
        $validated = $request->validate([
            'name' => 'required|string|max:255',
            'contact_id' => 'required|uuid|exists:contacts,id',
            'pipeline_id' => 'required|uuid|exists:pipelines,id',
            'stage_id' => 'required|uuid|exists:stages,id',
            'amount' => 'required|numeric|min:0',
            'expected_close_date' => 'required|date',
            'owner_id' => 'required|uuid|exists:users,id',
        ]);

        $deal = Deal::create([
            'id' => Str::uuid(),
            ...$validated,
            'probability' => 0,
        ]);

        // Create stage history
        DealStageHistory::create([
            'id' => Str::uuid(),
            'deal_id' => $deal->id,
            'to_stage_id' => $deal->stage_id,
        ]);

        return response()->json($deal->load(['contact', 'pipeline', 'stage']), 201);
    }

    public function show(string $id)
    {
        $deal = Deal::with([
            'contact',
            'pipeline',
            'stage',
            'owner',
            'activities',
            'notes.createdBy',
            'stageHistory.toStage'
        ])->findOrFail($id);

        return response()->json($deal);
    }

    public function update(Request $request, string $id)
    {
        $deal = Deal::findOrFail($id);

        $validated = $request->validate([
            'name' => 'sometimes|string|max:255',
            'amount' => 'sometimes|numeric|min:0',
            'expected_close_date' => 'sometimes|date',
            'probability' => 'sometimes|integer|min:0|max:100',
        ]);

        $deal->update($validated);

        return response()->json($deal);
    }

    public function updateStage(Request $request, string $id)
    {
        $deal = Deal::findOrFail($id);

        $validated = $request->validate([
            'stage_id' => 'required|uuid|exists:stages,id',
            'reason' => 'nullable|string',
        ]);

        $previousStageId = $deal->stage_id;
        $timeInPreviousStage = $deal->stage_entered_at->diffInSeconds(now());

        $deal->update([
            'stage_id' => $validated['stage_id'],
            'stage_entered_at' => now(),
            'stage_changes_count' => $deal->stage_changes_count + 1,
        ]);

        // Create stage history
        DealStageHistory::create([
            'id' => Str::uuid(),
            'deal_id' => $deal->id,
            'from_stage_id' => $previousStageId,
            'to_stage_id' => $validated['stage_id'],
            'time_in_previous_stage' => $timeInPreviousStage,
            'changed_by_id' => auth()->id(),
            'reason' => $validated['reason'] ?? null,
        ]);

        return response()->json($deal->fresh(['stage']));
    }

    public function win(Request $request, string $id)
    {
        $deal = Deal::findOrFail($id);

        $validated = $request->validate([
            'won_details' => 'nullable|string',
            'close_date' => 'nullable|date',
        ]);

        $deal->update([
            'status' => 'won',
            'closed_at' => now(),
            'close_date' => $validated['close_date'] ?? now(),
            'won_details' => $validated['won_details'] ?? null,
        ]);

        return response()->json($deal);
    }

    public function lose(Request $request, string $id)
    {
        $deal = Deal::findOrFail($id);

        $validated = $request->validate([
            'lost_reason' => 'required|string',
        ]);

        $deal->update([
            'status' => 'lost',
            'closed_at' => now(),
            'lost_reason' => $validated['lost_reason'],
        ]);

        return response()->json($deal);
    }

    public function forecast(Request $request)
    {
        $period = $request->get('period', 'this_month');
        $forecast = $this->forecastingService->getForecast($period);

        return response()->json($forecast);
    }
}

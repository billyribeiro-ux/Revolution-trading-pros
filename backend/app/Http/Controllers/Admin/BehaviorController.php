<?php

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Models\BehaviorSession;
use App\Models\FrictionPoint;
use App\Models\IntentSignal;
use App\Services\BehaviorProcessorService;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\DB;

class BehaviorController extends Controller
{
    public function __construct(
        private BehaviorProcessorService $processorService
    ) {}

    public function ingestEvents(Request $request)
    {
        $validated = $request->validate([
            'session_id' => 'required|string',
            'visitor_id' => 'required|string',
            'user_id' => 'nullable|string',
            'events' => 'required|array',
            'events.*.event_type' => 'required|string',
            'events.*.timestamp' => 'required|integer',
            'events.*.page_url' => 'required|string',
        ]);

        try {
            $session = $this->processorService->processEventBatch($validated);
            
            return response()->json([
                'success' => true,
                'session_id' => $session->id,
            ]);
        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'error' => $e->getMessage(),
            ], 500);
        }
    }

    public function getSession(string $sessionId)
    {
        $session = BehaviorSession::with(['events', 'frictionPoints', 'intentSignals'])
            ->findOrFail($sessionId);

        return response()->json($session);
    }

    public function getDashboard(Request $request)
    {
        $period = $request->get('period', '7d');
        $startDate = $this->getStartDate($period);

        $overview = [
            'total_sessions' => BehaviorSession::where('started_at', '>=', $startDate)->count(),
            'avg_engagement_score' => BehaviorSession::where('started_at', '>=', $startDate)->avg('engagement_score'),
            'avg_friction_score' => BehaviorSession::where('started_at', '>=', $startDate)->avg('friction_score'),
            'avg_intent_score' => BehaviorSession::where('started_at', '>=', $startDate)->avg('intent_score'),
            'high_churn_risk_count' => BehaviorSession::where('started_at', '>=', $startDate)
                ->where('churn_risk_score', '>=', 75)
                ->count(),
        ];

        $frictionHeatmap = FrictionPoint::select('page_url', DB::raw('COUNT(*) as friction_count'))
            ->where('created_at', '>=', $startDate)
            ->groupBy('page_url')
            ->orderByDesc('friction_count')
            ->limit(10)
            ->get()
            ->map(function ($item) {
                return [
                    'page_url' => $item->page_url,
                    'friction_count' => $item->friction_count,
                    'avg_severity' => FrictionPoint::where('page_url', $item->page_url)->avg('severity'),
                    'top_friction_type' => FrictionPoint::where('page_url', $item->page_url)
                        ->select('friction_type', DB::raw('COUNT(*) as count'))
                        ->groupBy('friction_type')
                        ->orderByDesc('count')
                        ->first()
                        ->friction_type ?? 'unknown',
                ];
            });

        $sessionTimeline = BehaviorSession::select(
                DB::raw('DATE(started_at) as date'),
                DB::raw('COUNT(*) as sessions'),
                DB::raw('AVG(engagement_score) as avg_engagement')
            )
            ->where('started_at', '>=', $startDate)
            ->groupBy('date')
            ->orderBy('date')
            ->get()
            ->map(fn($item) => [
                'timestamp' => strtotime($item->date) * 1000,
                'sessions' => $item->sessions,
                'avg_engagement' => round($item->avg_engagement, 2),
            ]);

        return response()->json([
            'overview' => $overview,
            'friction_heatmap' => $frictionHeatmap,
            'session_timeline' => $sessionTimeline,
        ]);
    }

    public function getFrictionPoints(Request $request)
    {
        $query = FrictionPoint::query();

        if ($request->has('page_url')) {
            $query->where('page_url', 'like', '%' . $request->get('page_url') . '%');
        }

        if ($request->has('friction_type')) {
            $query->where('friction_type', $request->get('friction_type'));
        }

        if ($request->has('severity')) {
            $query->where('severity', $request->get('severity'));
        }

        if ($request->has('resolved')) {
            $query->where('resolved', $request->boolean('resolved'));
        }

        return response()->json(
            $query->orderByDesc('created_at')->paginate(50)
        );
    }

    public function getIntentSignals(Request $request)
    {
        $query = IntentSignal::query();

        if ($request->has('user_id')) {
            $query->where('user_id', $request->get('user_id'));
        }

        if ($request->has('signal_type')) {
            $query->where('signal_type', $request->get('signal_type'));
        }

        if ($request->has('converted')) {
            $query->where('converted', $request->boolean('converted'));
        }

        return response()->json(
            $query->orderByDesc('timestamp')->paginate(50)
        );
    }

    public function resolveFrictionPoint(Request $request, string $id)
    {
        $validated = $request->validate([
            'notes' => 'nullable|string',
        ]);

        $frictionPoint = FrictionPoint::findOrFail($id);
        $frictionPoint->update([
            'resolved' => true,
            'resolved_at' => now(),
            'resolution_notes' => $validated['notes'] ?? null,
        ]);

        return response()->json([
            'success' => true,
            'friction_point' => $frictionPoint,
        ]);
    }

    private function getStartDate(string $period): \DateTime
    {
        return match($period) {
            '24h' => now()->subDay(),
            '7d' => now()->subDays(7),
            '30d' => now()->subDays(30),
            '90d' => now()->subDays(90),
            default => now()->subDays(7),
        };
    }
}

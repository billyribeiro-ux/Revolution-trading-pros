<?php

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Log;

class TimerAnalyticsController extends Controller
{
    public function store(Request $request): JsonResponse
    {
        $data = $request->validate([
            'timer_id' => ['required', 'string', 'max:191'],
            'type' => ['required', 'string', 'max:50'],
            'timestamp' => ['required', 'date'],
            'remaining_ms' => ['required', 'integer'],
            'payload' => ['nullable', 'array'],
        ]);

        Log::info('timer_event', $data + [
            'ip' => $request->ip(),
            'user_id' => optional($request->user())->id,
        ]);

        return response()->json([
            'status' => 'ok',
        ]);
    }
}

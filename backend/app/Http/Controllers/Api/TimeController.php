<?php

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use Illuminate\Http\JsonResponse;

class TimeController extends Controller
{
    public function now(): JsonResponse
    {
        $now = now();

        return response()->json([
            'server_time' => $now->toIso8601String(),
            'timezone' => config('app.timezone'),
        ]);
    }
}

<?php

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Models\Media;
use App\Models\Product;
use App\Services\MembershipService;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Storage;

class IndicatorController extends Controller
{
    public function __construct(
        protected MembershipService $membershipService
    ) {}

    public function index()
    {
        $indicators = Product::where('type', 'indicator')
            ->where('is_active', true)
            ->select('id', 'name', 'slug', 'description', 'price', 'thumbnail', 'meta_description')
            ->get();

        return response()->json($indicators);
    }

    public function show(string $slug)
    {
        $indicator = Product::where('slug', $slug)
            ->where('type', 'indicator')
            ->where('is_active', true)
            ->firstOrFail();

        return response()->json($indicator);
    }

    public function download(Request $request, string $slug, int $downloadId)
    {
        $user = $request->user();

        if (! $this->membershipService->userOwnsProduct($user, $slug)) {
            return response()->json(['message' => 'You do not own this indicator'], 403);
        }

        $media = Media::findOrFail($downloadId);

        if (! Storage::exists($media->path)) {
            return response()->json(['message' => 'File not found'], 404);
        }

        return Storage::download($media->path, $media->filename);
    }
}

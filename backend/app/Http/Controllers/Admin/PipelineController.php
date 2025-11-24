<?php

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Models\Pipeline;
use App\Models\Stage;
use Illuminate\Http\Request;
use Illuminate\Support\Str;

class PipelineController extends Controller
{
    public function index()
    {
        return response()->json(
            Pipeline::with('stages')->orderBy('position')->get()
        );
    }

    public function store(Request $request)
    {
        $validated = $request->validate([
            'name' => 'required|string|max:255',
            'description' => 'nullable|string',
            'color' => 'nullable|string|max:7',
            'is_default' => 'boolean',
        ]);

        $pipeline = Pipeline::create([
            'id' => Str::uuid(),
            ...$validated,
            'position' => Pipeline::max('position') + 1,
        ]);

        return response()->json($pipeline, 201);
    }

    public function show(string $id)
    {
        return response()->json(
            Pipeline::with('stages', 'deals')->findOrFail($id)
        );
    }

    public function update(Request $request, string $id)
    {
        $pipeline = Pipeline::findOrFail($id);

        $validated = $request->validate([
            'name' => 'sometimes|string|max:255',
            'description' => 'nullable|string',
            'color' => 'nullable|string|max:7',
            'is_active' => 'boolean',
        ]);

        $pipeline->update($validated);

        return response()->json($pipeline);
    }

    public function destroy(string $id)
    {
        $pipeline = Pipeline::findOrFail($id);
        
        if ($pipeline->deals_count > 0) {
            return response()->json([
                'message' => 'Cannot delete pipeline with existing deals'
            ], 422);
        }

        $pipeline->delete();

        return response()->json(['message' => 'Pipeline deleted']);
    }

    public function addStage(Request $request, string $id)
    {
        $pipeline = Pipeline::findOrFail($id);

        $validated = $request->validate([
            'name' => 'required|string|max:255',
            'probability' => 'required|integer|min:0|max:100',
            'color' => 'nullable|string|max:7',
        ]);

        $stage = Stage::create([
            'id' => Str::uuid(),
            'pipeline_id' => $pipeline->id,
            ...$validated,
            'position' => $pipeline->stages()->max('position') + 1,
        ]);

        return response()->json($stage, 201);
    }
}

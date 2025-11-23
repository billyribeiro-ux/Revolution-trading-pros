<?php

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Models\Redirect;
use Illuminate\Http\Request;

class RedirectController extends Controller
{
    /**
     * List all redirects.
     */
    public function index(Request $request)
    {
        $query = Redirect::query();

        if ($request->has('search')) {
            $search = $request->search;
            $query->where(function ($q) use ($search) {
                $q->where('from_path', 'like', "%{$search}%")
                  ->orWhere('to_path', 'like', "%{$search}%");
            });
        }

        if ($request->has('is_active')) {
            $query->where('is_active', $request->boolean('is_active'));
        }

        $redirects = $query->orderBy('created_at', 'desc')->paginate(50);

        return response()->json($redirects);
    }

    /**
     * Create a new redirect.
     */
    public function store(Request $request)
    {
        $request->validate([
            'from_path' => 'required|string|unique:redirects,from_path',
            'to_path' => 'required|string',
            'status_code' => 'nullable|integer|in:301,302,307,308',
            'type' => 'nullable|in:manual,automatic',
        ]);

        $redirect = Redirect::create([
            'from_path' => $request->from_path,
            'to_path' => $request->to_path,
            'status_code' => $request->status_code ?? 301,
            'type' => $request->type ?? 'manual',
            'is_active' => true,
        ]);

        return response()->json(['redirect' => $redirect], 201);
    }

    /**
     * Update a redirect.
     */
    public function update(Request $request, int $id)
    {
        $redirect = Redirect::findOrFail($id);

        $request->validate([
            'from_path' => 'sometimes|string|unique:redirects,from_path,' . $id,
            'to_path' => 'sometimes|string',
            'status_code' => 'sometimes|integer|in:301,302,307,308',
            'is_active' => 'sometimes|boolean',
        ]);

        $redirect->update($request->only([
            'from_path',
            'to_path',
            'status_code',
            'is_active',
        ]));

        return response()->json(['redirect' => $redirect]);
    }

    /**
     * Delete a redirect.
     */
    public function destroy(int $id)
    {
        $redirect = Redirect::findOrFail($id);
        $redirect->delete();

        return response()->json(['message' => 'Redirect deleted successfully']);
    }

    /**
     * Detect redirect chains.
     */
    public function detectChains()
    {
        // Mock implementation - in production, trace redirect chains
        return response()->json([
            'chains' => [],
        ]);
    }

    /**
     * Import redirects from CSV.
     */
    public function import(Request $request)
    {
        $request->validate([
            'file' => 'required|file|mimes:csv,txt',
        ]);

        // Mock implementation
        return response()->json([
            'imported' => 0,
            'failed' => 0,
        ]);
    }

    /**
     * Export redirects to CSV.
     */
    public function export()
    {
        $redirects = Redirect::all();

        $csv = "From Path,To Path,Status Code,Type,Active,Hits\n";
        foreach ($redirects as $redirect) {
            $csv .= sprintf(
                "%s,%s,%d,%s,%s,%d\n",
                $redirect->from_path,
                $redirect->to_path,
                $redirect->status_code,
                $redirect->type,
                $redirect->is_active ? 'Yes' : 'No',
                $redirect->hit_count
            );
        }

        return response($csv, 200, [
            'Content-Type' => 'text/csv',
            'Content-Disposition' => 'attachment; filename="redirects.csv"',
        ]);
    }
}

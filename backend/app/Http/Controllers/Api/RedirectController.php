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
     * Detect redirect chains by tracing redirect paths.
     * A chain occurs when redirect A -> B and B -> C exist.
     */
    public function detectChains()
    {
        $redirects = Redirect::where('is_active', true)->get();
        $redirectMap = [];

        // Build a map of from_path -> to_path
        foreach ($redirects as $redirect) {
            $redirectMap[$redirect->from_path] = [
                'to' => $redirect->to_path,
                'id' => $redirect->id
            ];
        }

        $chains = [];

        // Detect chains by following redirects
        foreach ($redirects as $redirect) {
            $chain = [
                ['id' => $redirect->id, 'from' => $redirect->from_path, 'to' => $redirect->to_path]
            ];
            $visited = [$redirect->from_path];
            $current = $redirect->to_path;

            // Follow the chain (max 10 hops to prevent infinite loops)
            $hops = 0;
            while (isset($redirectMap[$current]) && $hops < 10) {
                if (in_array($current, $visited)) {
                    // Circular redirect detected
                    $chain[] = ['circular' => true, 'path' => $current];
                    break;
                }

                $visited[] = $current;
                $nextRedirect = $redirectMap[$current];
                $chain[] = ['id' => $nextRedirect['id'], 'from' => $current, 'to' => $nextRedirect['to']];
                $current = $nextRedirect['to'];
                $hops++;
            }

            // Only report chains with more than 1 redirect
            if (count($chain) > 1) {
                $chains[] = [
                    'start' => $redirect->from_path,
                    'end' => $current,
                    'length' => count($chain),
                    'redirects' => $chain,
                    'has_circular' => isset($chain[count($chain) - 1]['circular'])
                ];
            }
        }

        // Remove duplicate chains (same start path)
        $uniqueChains = [];
        $seenStarts = [];
        foreach ($chains as $chain) {
            if (!in_array($chain['start'], $seenStarts)) {
                $uniqueChains[] = $chain;
                $seenStarts[] = $chain['start'];
            }
        }

        return response()->json([
            'chains' => $uniqueChains,
            'total_redirects' => count($redirects),
            'chains_found' => count($uniqueChains),
        ]);
    }

    /**
     * Import redirects from CSV file.
     * Expected format: from_path,to_path,status_code (optional)
     */
    public function import(Request $request)
    {
        $request->validate([
            'file' => 'required|file|mimes:csv,txt|max:5120', // Max 5MB
        ]);

        $file = $request->file('file');
        $handle = fopen($file->getRealPath(), 'r');

        if (!$handle) {
            return response()->json(['error' => 'Unable to read file'], 400);
        }

        $imported = 0;
        $failed = 0;
        $errors = [];
        $lineNumber = 0;

        // Skip header row if present
        $firstLine = fgetcsv($handle);
        $lineNumber++;

        // Check if first line is header
        $isHeader = str_contains(strtolower($firstLine[0] ?? ''), 'from') ||
                    str_contains(strtolower($firstLine[0] ?? ''), 'path');

        if (!$isHeader && $firstLine) {
            // First line is data, process it
            rewind($handle);
            $lineNumber = 0;
        }

        while (($data = fgetcsv($handle)) !== false) {
            $lineNumber++;

            if (count($data) < 2) {
                $failed++;
                $errors[] = "Line {$lineNumber}: Invalid format - need at least from_path and to_path";
                continue;
            }

            $fromPath = trim($data[0]);
            $toPath = trim($data[1]);
            $statusCode = isset($data[2]) ? (int) trim($data[2]) : 301;

            // Validate paths
            if (empty($fromPath) || empty($toPath)) {
                $failed++;
                $errors[] = "Line {$lineNumber}: Empty path value";
                continue;
            }

            // Validate status code
            if (!in_array($statusCode, [301, 302, 307, 308])) {
                $statusCode = 301;
            }

            // Check for existing redirect
            $existing = Redirect::where('from_path', $fromPath)->first();
            if ($existing) {
                $failed++;
                $errors[] = "Line {$lineNumber}: Redirect from '{$fromPath}' already exists";
                continue;
            }

            try {
                Redirect::create([
                    'from_path' => $fromPath,
                    'to_path' => $toPath,
                    'status_code' => $statusCode,
                    'type' => 'import',
                    'is_active' => true,
                ]);
                $imported++;
            } catch (\Exception $e) {
                $failed++;
                $errors[] = "Line {$lineNumber}: " . $e->getMessage();
            }
        }

        fclose($handle);

        return response()->json([
            'imported' => $imported,
            'failed' => $failed,
            'errors' => array_slice($errors, 0, 20), // Return first 20 errors only
            'total_errors' => count($errors),
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
